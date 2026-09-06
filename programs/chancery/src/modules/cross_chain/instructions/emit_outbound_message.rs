/// emit_outbound_message
///
/// Spec §9.7. Atomic outbound cross-chain redemption from Solana.
/// The user (principal) holds issued tokens that they want to redeem on a
/// remote chain. This handler:
///
///   1. Validates the pathway is `CROSS_CHAIN_REDEEM` and bound to the
///      issued-token mint being burned.
///   2. Validates the destination `RemoteDomainPolicy` is active and the
///      requested amount is within `per_message_maximum`.
///   3. Validates the policy's referenced `CrossChainSignerSet` is active
///      and within its validity window - enforced source-side because
///      attestors must be functional at the time the message is committed.
///   4. Bumps `RemoteNonce.next_outbound_nonce` to obtain `source_nonce` for
///      this message. The corridor's `RemoteNonce` is created at
///      `register_remote_domain_policy` (binding time), so this handler never
///      allocates: every emission gets a unique monotonic nonce, hence a
///      unique message hash, and stamps `last_emitted_message_hash` as the
///      outbound reconciliation cursor. The canonical record of emission is
///      the `CrossChainOutboundEmitted` event; no per-message account exists.
///   5. Computes `message_hash` over the canonical preimage (spec 10 §10.5).
///   6. Burns `args.amount` issued tokens from the principal's account via
///      SPL Token CPI.
///   7. Updates pathway-scoped usage windows (if a limit policy is bound).
///   8. Emits `CrossChainOutboundEmitted` carrying the full canonical
///      preimage so attestors can recompute the hash byte-for-byte.
///
/// Atomicity is provided by Solana transaction semantics: any failure after
/// the nonce bump rolls back the bump (no nonce gaps); any failure before
/// the burn means the user keeps their tokens.
///
/// Handler-local accounts after `dispatch_gated` removes the public
/// `module_activation_state` prefix. Public transaction/IDL indexes are one
/// greater than the indexes documented below.
///
/// Accounts (fixed):
///   0  chancery_config                    writable  (event_sequence_nonce bump)
///   1  event_authority                    readable  PDA [b"event-authority"]
///   2  pause_state                        readable  PDA [b"pause-state"]
///   3  asset_pause_state                  readable  PDA [b"asset-pause", pathway.asset_mint]
///   4  pathway_policy                     readable  PDA [b"pathway-policy", pathway_id]
///   5  remote_domain_policy               readable  PDA [b"remote-domain-policy", chain_kind, domain_id_be]
///   6  cross_chain_signer_set             readable  PDA [b"cross-chain-signer-set", policy.signer_set_id]
///   7  remote_nonce                       writable  PDA [b"remote-nonce", chain_kind, domain_id_be, default]
///   8  source_issued_token_account        writable  burn-from
///   9  issued_token_mint                  writable  burn-target
///  10  issued_token_program               readable
///  11  sender (principal)                 signer    owner of source_issued_token_account
///  12  issued_token_control               readable  PDA [b"issued-token-control"]
///  13  sender_permission_record           readable  PATHWAY-scoped CAN_EMIT_OUTBOUND_MESSAGE
///  14  asset_config                       readable  PDA [b"asset-config", pathway.asset_mint]
///
/// Accounts (optional, in slot-order):
///  15  fee_policy                         readable  (zero pubkey = skip; required iff pathway.has_fee_policy)
///  16  fee_recipient_token_account        writable  required iff fee_policy charges a non-zero fee
///  17  mint_authority_pda                 readable  PDA [b"mint-authority"]; required iff fee mint runs
///  18  limit_policy                       readable  PDA [b"limit-policy", limit_policy_id]
///  19  hourly_usage_window                writable  pathway-scoped
///  20  daily_usage_window                 writable  pathway-scoped
///  21  remote_domain_daily_usage_window   writable  remote-domain-scoped corridor cap
///  22  weekly_usage_window                writable  pathway-scoped; required iff per_seven_day_maximum != 0
///  23  monthly_usage_window               writable  pathway-scoped; required iff per_thirty_day_maximum != 0

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    constants::{
        chain_kind, cross_chain_message_kind, pathway_kind, remote_domain_pause_bit,
        window_kind, SOLANA_SELF_DOMAIN_ID,
    },
    error::ChanceryError,
    modules::{
        control::state::{
            asset_pause_state::AssetPauseState,
            pause_state::{pause_bit, PauseState},
        },
        core::state::{asset_config::AssetConfig, chancery_config::ChanceryConfig},
        cross_chain::{
            asset_binding::assert_outbound_remote_asset_binding,
            instructions::assert_distinct_cross_chain_account_indexes,
            message_hash::{compute_message_hash, MessageHashPreimage, MESSAGE_HASH_CANON_VERSION},
            state::{
                cross_chain_signer_set::CrossChainSignerSet,
                remote_domain_policy::RemoteDomainPolicy,
                remote_nonce::RemoteNonce,
            },
            usage::remote_domain_usage_window_scope_hash,
        },
        evidence::emit::{emit_usage_windows_rolled,
            emit_cross_chain_outbound_emitted, CrossChainOutboundEmitted,
        },
        fees::state::fee_policy::FeePolicy,
        limits::state::{
            limit_policy::LimitPolicy,
            usage_window::{prepare_enforced_window_for_recording, ClosedPeriod},
        },
        pathway::state::pathway_policy::PathwayPolicy,
        permissions::auth::assert_can_emit_outbound_message,
        settlement::instructions::{
            assert_canonical_issued_token_mint, assert_issued_token_extension_gates,
            assert_spl_token_account_binding, cpi_burn, cpi_mint_to,
        },
    },
};

// ─── Account indices ──────────────────────────────────────────────────────────
const CHANCERY_CONFIG:                  usize = 0;
const EVENT_AUTHORITY:                  usize = 1;
const PAUSE_STATE:                      usize = 2;
const ASSET_PAUSE_STATE:                usize = 3;
const PATHWAY_POLICY:                   usize = 4;
const REMOTE_DOMAIN_POLICY:             usize = 5;
const SIGNER_SET:                       usize = 6;
const REMOTE_NONCE_ACCOUNT_INDEX:       usize = 7;
const SOURCE_ISSUED_TOKEN_ACCOUNT:      usize = 8;
const ISSUED_TOKEN_MINT:                usize = 9;
const ISSUED_TOKEN_PROGRAM:             usize = 10;
const SENDER:                           usize = 11;
const ISSUED_TOKEN_CONTROL:             usize = 12;
const SENDER_PERMISSION_RECORD:         usize = 13;
const ASSET_CONFIG:                     usize = 14;
const REQUIRED_ACCOUNT_COUNT:           usize = 15;

const FEE_POLICY:                       usize = 15;
const FEE_RECIPIENT_TOKEN_ACCOUNT:      usize = 16;
const MINT_AUTHORITY_PDA:               usize = 17;
const LIMIT_POLICY:                     usize = 18;
const HOURLY_USAGE_WINDOW:              usize = 19;
const DAILY_USAGE_WINDOW:               usize = 20;
const REMOTE_DOMAIN_DAILY_USAGE_WINDOW: usize = 21;
const WEEKLY_USAGE_WINDOW:              usize = 22;
const MONTHLY_USAGE_WINDOW:             usize = 23;

const DISTINCT_ACCOUNT_INDEXES:      &[usize] = &[
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    PAUSE_STATE,
    ASSET_PAUSE_STATE,
    PATHWAY_POLICY,
    REMOTE_DOMAIN_POLICY,
    SIGNER_SET,
    REMOTE_NONCE_ACCOUNT_INDEX,
    SOURCE_ISSUED_TOKEN_ACCOUNT,
    ISSUED_TOKEN_MINT,
    ISSUED_TOKEN_PROGRAM,
    SENDER,
    ISSUED_TOKEN_CONTROL,
    SENDER_PERMISSION_RECORD,
    ASSET_CONFIG,
    FEE_POLICY,
    FEE_RECIPIENT_TOKEN_ACCOUNT,
    MINT_AUTHORITY_PDA,
    LIMIT_POLICY,
    HOURLY_USAGE_WINDOW,
    DAILY_USAGE_WINDOW,
    REMOTE_DOMAIN_DAILY_USAGE_WINDOW,
    WEEKLY_USAGE_WINDOW,
    MONTHLY_USAGE_WINDOW,
];

// ─── Args ─────────────────────────────────────────────────────────────────────
#[derive(BorshDeserialize)]
pub struct EmitOutboundMessageArgs {
    /// Local pathway PDA selector. Must resolve to a `CROSS_CHAIN_REDEEM` pathway.
    pub pathway_id:                [u8; 32],

    /// Destination remote chain-family discriminant. `chain_kind::SOLANA` is
    /// valid for another SVM deployment; corridor identity is not inferred from
    /// this discriminant alone.
    pub remote_chain_kind:         u8,

    /// Destination remote domain id, BE-encoded into the policy/nonce PDA seeds.
    pub remote_domain_id:          u64,

    /// Amount of issued tokens to burn (Solana-side u64; widened to u128 in canon).
    pub amount:                    u64,

    /// Minimum post-fee amount committed for the remote recipient. 0 = disabled.
    pub minimum_remote_amount:     u64,

    /// 32-byte recipient on the remote chain (left-padded for 20-byte EVM addrs).
    pub recipient:                 [u8; 32],

    /// 32-byte remote-asset identifier; informational on outbound, validated
    /// by the daughter contract per its local config.
    pub destination_asset:         [u8; 32],

    /// One of `cross_chain_message_kind::OUTBOUND_*`.
    pub message_kind:              u8,

    /// Unix timestamp after which the daughter must reject this message.
    /// 0 = no expiry. Recommended: clock + remote_domain_policy.message_expiry_seconds.
    pub expires_at_unix_timestamp: i64,
}

// ─── Handler ──────────────────────────────────────────────────────────────────
pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    assert_distinct_cross_chain_account_indexes(accounts, DISTINCT_ACCOUNT_INDEXES)?;

    let chancery_config_account_info      = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];
    let pause_state_account_info          = &accounts[PAUSE_STATE];
    let asset_pause_state_account_info    = &accounts[ASSET_PAUSE_STATE];
    let pathway_policy_account_info       = &accounts[PATHWAY_POLICY];
    let remote_policy_account_info        = &accounts[REMOTE_DOMAIN_POLICY];
    let signer_set_account_info           = &accounts[SIGNER_SET];
    let remote_nonce_account_info         = &accounts[REMOTE_NONCE_ACCOUNT_INDEX];
    let source_issued_token_account_info  = &accounts[SOURCE_ISSUED_TOKEN_ACCOUNT];
    let issued_token_mint_account_info    = &accounts[ISSUED_TOKEN_MINT];
    let issued_token_program_account_info = &accounts[ISSUED_TOKEN_PROGRAM];
    let sender_account_info               = &accounts[SENDER];
    let issued_token_control_account_info = &accounts[ISSUED_TOKEN_CONTROL];
    let sender_permission_account_info    = &accounts[SENDER_PERMISSION_RECORD];
    let asset_config_account_info         = &accounts[ASSET_CONFIG];

    if !sender_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !remote_nonce_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !source_issued_token_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !issued_token_mint_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }


    let args = EmitOutboundMessageArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    if args.amount == 0 {
        return Err(ChanceryError::AmountMustBePositive.into());
    }

    let clock = Clock::get()?;
    let mut rolled_windows: Vec<ClosedPeriod> = Vec::new();

    // ── Chancery + global pause check ─────────────────────────────────────────
    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;


    if &chancery_config.issued_token_program != issued_token_program_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }
    // Copy out the only chancery_config field we still need after the mutable
    // re-borrow at emit time. Avoids a borrow-checker conflict between
    // `chancery_config` (immut) and `chancery_config_mut` (mut) on the same account.
    let chancery_domain_separator: [u8; 32] = chancery_config.domain_separator;

    let pause_state_bump = PauseState::verify_pda(pause_state_account_info, &crate::id())?;
    let pause_state = PauseState::load_for_verified_pda(pause_state_account_info, pause_state_bump)?;
    pause_state.assert_not_paused_for(pause_bit::REDEEM, clock.slot)?;

    // ── message_kind allowlist (outbound only) ────────────────────────────────
    match args.message_kind {
        cross_chain_message_kind::OUTBOUND_REDEEM_TO_REMOTE_MINT
        | cross_chain_message_kind::OUTBOUND_REDEEM_TO_REMOTE_RELEASE => {}
        _ => return Err(ChanceryError::MessageKindNotSupported.into()),
    }

    // ── Expiry sanity (must be future-dated if non-zero) ──────────────────────
    if args.expires_at_unix_timestamp != 0 && args.expires_at_unix_timestamp <= clock.unix_timestamp {
        return Err(ChanceryError::MessageExpired.into());
    }

    let program_id = crate::id();

    // ── Pathway validation ────────────────────────────────────────────────────
    let pathway_policy_bump = PathwayPolicy::verify_pda(pathway_policy_account_info, &args.pathway_id, &program_id)?;

    let pathway = PathwayPolicy::load_for_verified_pda(
        pathway_policy_account_info,
        &args.pathway_id,
        pathway_policy_bump,
    )?;

    pathway.assert_active()?;
    pathway.assert_kind(pathway_kind::CROSS_CHAIN_REDEEM)?;
    
    if pathway.has_evidence_policy() {
        return Err(ChanceryError::PathwayDependencyMissing.into());
    }

    let asset_pause_state_bump = AssetPauseState::verify_pda(
        asset_pause_state_account_info,
        &pathway.asset_mint,
        &program_id,
    )?;
    if !asset_pause_state_account_info.data_is_empty() {
        let asset_pause_state = AssetPauseState::load_for_verified_pda(
            asset_pause_state_account_info,
            &pathway.asset_mint,
            asset_pause_state_bump,
        )?;
        asset_pause_state.assert_not_paused_for(pause_bit::REDEEM, clock.slot)?;
    }

    // Cross-chain redemption is a user settlement surface for the collateral
    // asset committed by the pathway. Bind its canonical AssetConfig and honor
    // the same lifecycle, amount, and extension gates as local redemptions
    // before advancing the outbound nonce or burning issued tokens.
    let asset_config_bump = AssetConfig::verify_pda(
        asset_config_account_info,
        &pathway.asset_mint,
        &program_id,
    )?;
    let asset_config = AssetConfig::load_for_verified_pda(
        asset_config_account_info,
        &pathway.asset_mint,
        asset_config_bump,
    )?;
    asset_config.assert_extensions_fresh(clock.slot)?;
    asset_config.assert_redeems_permitted()?;
    asset_config.assert_extensions_approved()?;
    pathway.assert_collateral_extensions_allowed(asset_config.observed_extension_mask)?;
    asset_config.assert_redeem_amount(args.amount)?;
    drop(asset_config);

    let outbound_permission = assert_can_emit_outbound_message(
        sender_account_info,
        sender_permission_account_info,
        pathway_policy_account_info.key,
        &program_id,
    )?;
    drop(outbound_permission);

    assert_canonical_issued_token_mint(&chancery_config, &pathway, issued_token_mint_account_info)?;

    // Outbound emission is a principal action, not a delegated burn surface.
    // Token-2022 BurnChecked accepts either the account owner or an approved
    // delegate, so bind the burn source to the canonical sender before the CPI.
    // Without this check a pathway-authorized delegate could burn a holder's
    // balance and direct the remote proceeds to its own recipient.
    assert_spl_token_account_binding(
        source_issued_token_account_info,
        issued_token_program_account_info,
        &pathway.issued_token_mint,
        sender_account_info.key,
    )?;

    assert_issued_token_extension_gates(
        issued_token_control_account_info,
        issued_token_mint_account_info,
        &pathway,
        clock.slot,
        &program_id,
    )?;

    // ── Remote-domain policy validation ───────────────────────────────────────
    let remote_policy_bump = RemoteDomainPolicy::verify_pda(
        remote_policy_account_info,
        args.remote_chain_kind,
        args.remote_domain_id,
        &program_id,
    )?;

    let remote_policy = RemoteDomainPolicy::load_for_verified_pda(
        remote_policy_account_info,
        args.remote_chain_kind,
        args.remote_domain_id,
        remote_policy_bump,
    )?;

    // The remote daughter fixes `motherAsset` at deployment. Bind the selected
    // local pathway to that same immutable corridor identity before advancing
    // its strict nonce lane or burning the sender's issued tokens.
    remote_policy.assert_local_asset_mint_matches(&pathway.asset_mint)?;

    remote_policy.assert_not_paused_for(
        remote_domain_pause_bit::OUTBOUND_MESSAGES
            | remote_domain_pause_bit::REMOTE_REDEEM
            | remote_domain_pause_bit::DAUGHTER_CONTRACT,
    )?;
    remote_policy.assert_expiry_within_window(
        args.expires_at_unix_timestamp,
        clock.unix_timestamp,
    )?;
    remote_policy.assert_per_message_within_cap(args.amount)?;

    // Bind message_kind to the policy's mode and the message's claimed
    // remote asset. Without this an OUTBOUND_REDEEM_TO_REMOTE_RELEASE can be
    // routed through a MINT-mode corridor (or vice versa), and the
    // destination_asset field is effectively informational.
    assert_outbound_remote_asset_binding(
        args.message_kind,
        &args.destination_asset,
        &remote_policy,
    )?;

    // ── Signer-set validation: must exist, be active, be temporally valid,
    //    and match the id committed on the policy. ──────────────────────────────
    let signer_set_bump = CrossChainSignerSet::verify_pda(signer_set_account_info, &remote_policy.signer_set_id, &program_id)?;

    let signer_set = CrossChainSignerSet::load_for_verified_pda(
        signer_set_account_info,
        &remote_policy.signer_set_id,
        signer_set_bump,
    )?;

    signer_set.assert_active()?;
    signer_set.assert_temporally_valid(clock.unix_timestamp)?;
    remote_policy.assert_signer_set_id_matches(&signer_set.signer_set_id)?;

    // ── RemoteNonce: corridor counter PDA per (chain_kind, domain_id) ─────────
    // Outbound counters are partitioned per-domain with scope_key = default.
    // Created at `register_remote_domain_policy`, never here.
    let nonce_scope_key = [0u8; 32];
    let remote_nonce_bump = RemoteNonce::verify_pda(
        remote_nonce_account_info,
        args.remote_chain_kind,
        args.remote_domain_id,
        &nonce_scope_key,
        &program_id,
    )?;

    // Bound at `register_remote_domain_policy` (binding time). A corridor
    // whose nonce account is missing was never registered - fail closed
    // before any state change rather than allocating in the hot path.
    if remote_nonce_account_info.data_is_empty() {
        return Err(ChanceryError::NotInitialized.into());
    }

    let source_nonce = {
        let mut n = RemoteNonce::load_mut_for_verified_pda(
            remote_nonce_account_info,
            args.remote_domain_id,
            &nonce_scope_key,
            remote_nonce_bump,
        )?;

        n.take_outbound()?
    };

    // ── Pathway-scoped limit policy enforcement (optional) ────────────────────
    let has_limit = pathway.has_limit_policy()
        && accounts.len() > LIMIT_POLICY
        && accounts[LIMIT_POLICY].key != &Pubkey::default();

    if pathway.has_limit_policy() && !has_limit {
        return Err(ChanceryError::MissingAccount.into());
    }

    // Derived from the loaded policy, never from which accounts the caller
    // passed, so an enforced window cannot be skipped by omission or read-only.
    let mut hourly_prepared  = None;
    let mut daily_prepared   = None;
    let mut weekly_prepared  = None;
    let mut monthly_prepared = None;

    if has_limit {
        let limit_policy_bump = LimitPolicy::verify_pda(
            &accounts[LIMIT_POLICY],
            &pathway.limit_policy_id,
            &program_id,
        )?;

        let limit_policy = LimitPolicy::load_for_verified_pda(
            &accounts[LIMIT_POLICY],
            &pathway.limit_policy_id,
            limit_policy_bump,
        )?;

        let scope_hash       = limit_policy.scope_hash();

        limit_policy.assert_per_tx(args.amount)?;

        let hourly_enforced  = limit_policy.per_hour_maximum != 0
            || limit_policy.maximum_actions_per_hour != 0;
        let daily_enforced   = limit_policy.per_day_maximum != 0
            || limit_policy.maximum_actions_per_day != 0;
        // Weekly/monthly are volume-only caps (no action-count component).
        let weekly_enforced  = limit_policy.per_seven_day_maximum != 0;
        let monthly_enforced = limit_policy.per_thirty_day_maximum != 0;

        if hourly_enforced {
            let hourly_usage_window = prepare_enforced_window_for_recording(
                accounts,
                HOURLY_USAGE_WINDOW,
                &scope_hash,
                window_kind::HOURLY,
                clock.unix_timestamp,
                &program_id,
                &mut rolled_windows,
            )?;

            limit_policy.assert_window_volume(
                hourly_usage_window.gross_out,
                args.amount,
                ChanceryError::HourlyLimitBreached,
            )?;
            limit_policy.assert_action_count(hourly_usage_window.action_count, true)?;

            hourly_prepared = Some(hourly_usage_window);
        }

        if daily_enforced {
            let daily_usage_window = prepare_enforced_window_for_recording(
                accounts,
                DAILY_USAGE_WINDOW,
                &scope_hash,
                window_kind::DAILY,
                clock.unix_timestamp,
                &program_id,
                &mut rolled_windows,
            )?;

            limit_policy.assert_window_volume(
                daily_usage_window.gross_out,
                args.amount,
                ChanceryError::DailyLimitBreached,
            )?;
            limit_policy.assert_action_count(daily_usage_window.action_count, false)?;

            daily_prepared = Some(daily_usage_window);
        }

        if weekly_enforced {
            let weekly_usage_window = prepare_enforced_window_for_recording(
                accounts,
                WEEKLY_USAGE_WINDOW,
                &scope_hash,
                window_kind::WEEKLY,
                clock.unix_timestamp,
                &program_id,
                &mut rolled_windows,
            )?;

            limit_policy.assert_window_volume(
                weekly_usage_window.gross_out,
                args.amount,
                ChanceryError::WeeklyLimitBreached,
            )?;

            weekly_prepared = Some(weekly_usage_window);
        }

        if monthly_enforced {
            let monthly_usage_window = prepare_enforced_window_for_recording(
                accounts,
                MONTHLY_USAGE_WINDOW,
                &scope_hash,
                window_kind::MONTHLY,
                clock.unix_timestamp,
                &program_id,
                &mut rolled_windows,
            )?;

            limit_policy.assert_window_volume(
                monthly_usage_window.gross_out,
                args.amount,
                ChanceryError::MonthlyLimitBreached,
            )?;

            monthly_prepared = Some(monthly_usage_window);
        }
    }

    // ── Remote-domain per-day corridor cap. Mirrors `assert_per_day_within_cap`
    //    on the policy. The window account is optional: when omitted, no
    //    daily cap is enforced and `per_day_maximum == 0` semantics apply. When
    //    present, its scope_hash MUST match the canonical convention. ─────────
    // Enforcement derived from the remote-domain policy, not from which accounts
    // were passed. When the corridor caps daily flow the window is mandatory
    // (present + writable), so it cannot be skipped by omission or read-only.
    let remote_enforced = remote_policy.per_day_maximum != 0;

    let mut remote_prepared = None;

    if remote_enforced {
        let expected_scope_hash =
            remote_domain_usage_window_scope_hash(remote_policy_account_info.key);

        let rdw = prepare_enforced_window_for_recording(
            accounts,
            REMOTE_DOMAIN_DAILY_USAGE_WINDOW,
            &expected_scope_hash,
            window_kind::DAILY,
            clock.unix_timestamp,
            &program_id,
            &mut rolled_windows,
        )?;

        remote_policy.assert_per_day_within_cap(
            rdw.gross_out,
            args.amount,
        )?;

        remote_prepared = Some(rdw);
    }

    // ── Fee policy. Mirror of mint_direct's pattern. The user
    //    burns `args.amount` (gross); the canonical message commits `net`
    //    (the post-fee amount that crosses the corridor). The effective fee
    //    `gross - net` is either retained as a local supply reduction or
    //    re-minted to the policy-approved fee recipient. ─────────────────────
    if pathway.has_fee_policy()
        && (accounts.len() <= FEE_POLICY
            || accounts[FEE_POLICY].key == &Pubkey::default())
    {
        return Err(ChanceryError::MissingAccount.into());
    }

    let (
        fee_output_amount,
        rebate_amount,
        net_output_amount,
        fee_policy_id,
        route_fee_to_recipient,
    ): (u64, u64, u64, [u8; 32], bool) =
        if accounts.len() > FEE_POLICY
            && pathway.has_fee_policy()
            && accounts[FEE_POLICY].key != &Pubkey::default()
        {
            let fee_policy_bump = FeePolicy::verify_pda(
                &accounts[FEE_POLICY],
                &pathway.fee_policy_id,
                &program_id,
            )?;

            let fp = FeePolicy::load_for_verified_pda(
                &accounts[FEE_POLICY],
                &pathway.fee_policy_id,
                fee_policy_bump,
            )?;

            fp.assert_effective(clock.unix_timestamp)?;

            let computation = fp.compute_issued_token_fee_with_input(args.amount, args.amount)?;

            let route_fee_to_recipient = computation.net_fee > 0 && fp.routes_fee_to_recipient();

            if route_fee_to_recipient {
                if accounts.len() <= FEE_RECIPIENT_TOKEN_ACCOUNT
                    || accounts[FEE_RECIPIENT_TOKEN_ACCOUNT].key == &Pubkey::default()
                {
                    return Err(ChanceryError::MissingAccount.into());
                }
                if accounts.len() <= MINT_AUTHORITY_PDA
                    || accounts[MINT_AUTHORITY_PDA].key == &Pubkey::default()
                {
                    return Err(ChanceryError::MissingAccount.into());
                }
                if accounts[MINT_AUTHORITY_PDA].key != &chancery_config.mint_authority_pda {
                    return Err(ChanceryError::AccountKeyMismatch.into());
                }
                fp.assert_recipient_token_account(
                    &accounts[FEE_RECIPIENT_TOKEN_ACCOUNT],
                    issued_token_program_account_info,
                    issued_token_mint_account_info.key,
                )?;
            }

            (
                computation.assessed_fee,
                computation.effective_rebate,
                computation.net_output,
                pathway.fee_policy_id,
                route_fee_to_recipient,
            )
        } else {
            (0u64, 0u64, args.amount, [0u8; 32], false)
        };

    // The sender signs this floor with the gross burn amount. A fee-policy
    // update cannot silently reduce the remote principal below it.
    crate::modules::settlement::assert_output_meets_minimum(
        net_output_amount,
        args.minimum_remote_amount,
        ChanceryError::AmountBelowMinimum,
    )?;

    // ── Compute canonical message_hash. Pubkey → [u8;32] copies are
    //    held in stack bindings to keep the references alive across the call. ─
    let source_chancery_bytes = chancery_config_account_info.key.to_bytes();
    let source_asset_bytes    = pathway.asset_mint.to_bytes();
    let source_issued_bytes   = pathway.issued_token_mint.to_bytes();
    let sender_bytes          = sender_account_info.key.to_bytes();

    let preimage = MessageHashPreimage {
        message_kind:                  args.message_kind,
        source_chain_kind:             chain_kind::SOLANA,
        destination_chain_kind:        remote_policy.remote_chain_kind,
        source_domain_id:              SOLANA_SELF_DOMAIN_ID,
        destination_domain_id:         remote_policy.remote_domain_id,
        source_nonce,
        amount:                        net_output_amount as u128,
        expires_at_unix_timestamp:     args.expires_at_unix_timestamp,
        source_chancery_contract:      &source_chancery_bytes,
        destination_chancery_contract: &remote_policy.remote_chancery_contract,
        source_domain_separator:       &chancery_domain_separator,
        destination_domain_separator:  &remote_policy.remote_domain_separator,
        source_asset:                  &source_asset_bytes,
        destination_asset:             &args.destination_asset,
        source_issued_token:           &source_issued_bytes,
        destination_issued_token:      &remote_policy.remote_issued_token,
        sender:                        &sender_bytes,
        recipient:                     &args.recipient,
        signer_set_id:                 &remote_policy.signer_set_id,
    };
    let message_hash = compute_message_hash(&preimage);

    // ── Outbound reconciliation cursor. `take_outbound` already retired this
    //    nonce, so the hash is unique per emission; the event is the canonical
    //    record. The cursor lets operators reconcile the corridor head without
    //    replaying history. ─────────────────────────────────────────────────────
    {
        let mut n = RemoteNonce::load_mut_for_verified_pda(
            remote_nonce_account_info,
            args.remote_domain_id,
            &nonce_scope_key,
            remote_nonce_bump,
        )?;

        n.last_emitted_message_hash = message_hash;
    }

    // ── BURN: atomic with this entire ix. Failure here reverts the nonce bump. ─
    cpi_burn(
        issued_token_program_account_info,
        source_issued_token_account_info,
        issued_token_mint_account_info,
        sender_account_info,
        args.amount,
    )?;

    // ── Mint the net fee (fee - rebate) back to an approved recipient.
    //    Retention modes intentionally route nothing: the fee remains a local
    //    supply reduction while the remote corridor credits only `net`.
    let net_fee_output_amount = args.amount
        .checked_sub(net_output_amount)
        .ok_or(ChanceryError::ArithmeticUnderflow)?;

    if route_fee_to_recipient && net_fee_output_amount > 0 {
        let mint_bump = chancery_config.resolved_mint_authority_bump(&program_id);

        cpi_mint_to(
            issued_token_program_account_info,
            issued_token_mint_account_info,
            &accounts[FEE_RECIPIENT_TOKEN_ACCOUNT],
            &accounts[MINT_AUTHORITY_PDA],
            mint_bump,
            net_fee_output_amount,
        )?;
    }

    // ── Update pathway-scoped usage windows. Burn = outflow of issued token.
    //    Enforced windows were required present + writable and PDA-verified
    //    above, so record unconditionally - no `is_writable` escape hatch. ─────
    if let Some(prepared_window) = hourly_prepared {
        prepared_window.record_outflow(&accounts[HOURLY_USAGE_WINDOW], args.amount)?;
    }

    if let Some(prepared_window) = daily_prepared {
        prepared_window.record_outflow(&accounts[DAILY_USAGE_WINDOW], args.amount)?;
    }

    if let Some(prepared_window) = weekly_prepared {
        prepared_window.record_outflow(&accounts[WEEKLY_USAGE_WINDOW], args.amount)?;
    }

    if let Some(prepared_window) = monthly_prepared {
        prepared_window.record_outflow(&accounts[MONTHLY_USAGE_WINDOW], args.amount)?;
    }

    // ── Update remote-domain usage window. Corridor-level outflow;
    //    required present + writable above when the corridor caps daily flow. ──
    if let Some(prepared_window) = remote_prepared {
        prepared_window.record_outflow(&accounts[REMOTE_DOMAIN_DAILY_USAGE_WINDOW], args.amount)?;
    }

    // ── Bump seq + emit (invariant #9: evidence last) ─────────────────────────
    drop(chancery_config);

    let mut chancery_config_mut  = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump = chancery_config_mut.event_authority_bump;

    emit_usage_windows_rolled(
        event_authority_account_info,
        event_authority_bump,
        &mut chancery_config_mut,
        chancery_config_account_info.key,
        &clock,
        &rolled_windows,
    )?;
    let seq                  = chancery_config_mut.next_sequence_nonce()?;

    emit_cross_chain_outbound_emitted(
        event_authority_account_info,
        event_authority_bump,
        CrossChainOutboundEmitted {
            sequence_nonce:                seq,
            chancery:                      *chancery_config_account_info.key,
            slot:                          clock.slot,
            unix_timestamp:                clock.unix_timestamp,
            message_hash:                  message_hash,
            canon_version:                 MESSAGE_HASH_CANON_VERSION,
            message_kind:                  args.message_kind,
            source_chain_kind:             chain_kind::SOLANA,
            destination_chain_kind:        remote_policy.remote_chain_kind,
            source_domain_id:              SOLANA_SELF_DOMAIN_ID,
            destination_domain_id:         remote_policy.remote_domain_id,
            source_nonce,
            amount:                        net_output_amount as u128,
            expires_at_unix_timestamp:     args.expires_at_unix_timestamp,
            source_chancery_contract:      source_chancery_bytes,
            destination_chancery_contract: remote_policy.remote_chancery_contract,
            source_domain_separator:       chancery_domain_separator,
            destination_domain_separator:  remote_policy.remote_domain_separator,
            source_asset:                  source_asset_bytes,
            destination_asset:             args.destination_asset,
            source_issued_token:           source_issued_bytes,
            destination_issued_token:      remote_policy.remote_issued_token,
            sender:                        sender_bytes,
            recipient:                     args.recipient,
            signer_set_id:                 remote_policy.signer_set_id,
            pathway_id:                    args.pathway_id,
            burned_by:                     *sender_account_info.key,
            gross_amount:                  args.amount as u128,
            fee_output_amount:             fee_output_amount as u128,
            rebate_amount:                 rebate_amount as u128,
            fee_policy_id:                 fee_policy_id,
        },
    )?;

    Ok(())
}
