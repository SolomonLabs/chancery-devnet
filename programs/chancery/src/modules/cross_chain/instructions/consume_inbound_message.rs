/// consume_inbound_message
///
/// Spec §9.8. Atomic inbound cross-chain mint on Solana. A daughter
/// contract on a remote chain has burned (or locked) the equivalent
/// supply, and a quorum of attestors from the remote-domain-policy's
/// committed signer set has signed the canonical message hash. This
/// handler:
///
///   1. Gates the caller on `CAN_CONSUME_INBOUND_MESSAGE` (option-(b)
///      role-bit gating; the cryptographic attestation is still the
///      authoritative cross-chain signal - the role bit is spam control,
///      not a security boundary).
///   2. Validates pathway is `CROSS_CHAIN_MINT`, bound to the local
///      issued-token mint, and not blocked by the pathway asset's pause state.
///   3. Validates `RemoteDomainPolicy` is active and within per-message
///      cap, and the signer set referenced by it is active + within its
///      validity window.
///   4. Asserts strict in-order delivery: `args.source_nonce` must equal
///      `RemoteNonce.next_inbound_nonce`. Out-of-order = `RemoteNonceMismatch`.
///   5. Verifies the recipient token account is owned by `args.recipient`
///      and is a token account of `pathway.issued_token_mint`. Prevents
///      a relayer from redirecting funds to their own account.
///   6. Recomputes `message_hash` from the canonical preimage assembled
///      from policy + pathway + config + args. Asserts equal to the
///      attested hash; any drift in any committed field fails here
///      before any cryptographic work.
///   7. Verifies the attestation quorum via secp256k1_recover + keccak
///      address derivation + sorted-pair Merkle proof against
///      `signer_set.signer_root` (see `cross_chain::attestation`).
///   8. Bumps `RemoteNonce.next_inbound_nonce` and records the message hash
///      as the inbound reconciliation cursor. Strict nonce equality (step 4)
///      plus this atomic bump is the exactly-once primitive: a replay of an
///      already-consumed message carries a stale `source_nonce` and fails
///      `RemoteNonceMismatch` before any token movement. No per-message
///      account exists; the canonical record of consumption is the
///      `CrossChainInboundConsumed` event.
///   9. Mints `args.amount` issued tokens to the recipient token account
///      via SPL Token CPI signed as `mint_authority_pda`.
///  10. Emits `CrossChainInboundConsumed` carrying the full canonical
///      preimage for symmetric off-chain audit with the source-side
///      `CrossChainOutboundEmitted`.
///
/// Atomicity: any failure rolls back the nonce bump and the mint together.
/// The nonce bump is the one persistent state change that matters for
/// idempotency - retiring `source_nonce` makes a second submission of the
/// same message fail `RemoteNonceMismatch` cleanly, with no double-mint.
///
/// Accounts (fixed):
///   0  chancery_config                    writable
///   1  event_authority                    readable  PDA [b"event-authority"]
///   2  pause_state                        readable  PDA [b"pause-state"]
///   3  asset_pause_state                  readable  PDA [b"asset-pause", pathway.asset_mint]
///   4  pathway_policy                     readable  PDA [b"pathway-policy", pathway_id]
///   5  remote_domain_policy               readable  PDA [b"remote-domain-policy", chain_kind, domain_id_be]
///   6  cross_chain_signer_set             readable  PDA [b"cross-chain-signer-set", signer_set_id]
///   7  remote_nonce                       writable  PDA [b"remote-nonce", chain_kind, domain_id_be, default]
///   8  recipient_token_account            writable
///   9  issued_token_mint                  writable
///  10  mint_authority_pda                 readable  PDA [b"mint-authority"]
///  11  issued_token_program               readable
///  12  relayer                            signer
///  13  relayer_permission_record          readable  PDA [b"permission", relayer, GLOBAL, default]
///  14  issued_token_control               readable  PDA [b"issued-token-control"]
///  15  asset_config                       readable  PDA [b"asset-config", pathway.asset_mint]
///
/// Accounts (optional, in slot-order):
///  16  limit_policy                       readable
///  17  hourly_usage_window                writable  pathway-scoped
///  18  daily_usage_window                 writable  pathway-scoped
///  19  remote_domain_daily_usage_window   writable  remote-domain-scoped corridor cap
///  20  weekly_usage_window                writable  pathway-scoped; required iff per_seven_day_maximum != 0
///  21  monthly_usage_window               writable  pathway-scoped; required iff per_thirty_day_maximum != 0

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    constants::{
        chain_kind, cross_chain_message_kind, programs, remote_domain_pause_bit,
        pathway_kind, role, scope, window_kind, SOLANA_SELF_DOMAIN_ID,
    },
    error::ChanceryError,
    modules::{
        control::state::{
            asset_pause_state::AssetPauseState,
            pause_state::{pause_bit, PauseState},
        },
        core::state::{asset_config::AssetConfig, chancery_config::ChanceryConfig},
        cross_chain::{
            asset_binding::assert_inbound_remote_asset_binding,
            attestation::{
                verify_attestation_quorum, AttestationSignature,
                MAX_ATTESTED_INSTRUCTION_DATA_LEN,
            },
            auth::assert_signer_holds_role,
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
            emit_cross_chain_inbound_consumed, CrossChainInboundConsumed,
        },
        limits::state::{
            limit_policy::LimitPolicy,
            usage_window::{prepare_enforced_window_for_recording, ClosedPeriod},
        },
        pathway::state::pathway_policy::PathwayPolicy,
        settlement::instructions::{assert_canonical_issued_token_mint, assert_issued_token_extension_gates, cpi_mint_to},
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
const RECIPIENT_ISSUED_TOKEN_ACCOUNT:   usize = 8;
const ISSUED_TOKEN_MINT:                usize = 9;
const MINT_AUTHORITY_PDA:               usize = 10;
const ISSUED_TOKEN_PROGRAM:             usize = 11;
const RELAYER:                          usize = 12;
const RELAYER_PERMISSION_RECORD:        usize = 13;
const ISSUED_TOKEN_CONTROL:             usize = 14;
const ASSET_CONFIG:                     usize = 15;
const REQUIRED_ACCOUNT_COUNT:           usize = 16;

const LIMIT_POLICY:                     usize = 16;
const HOURLY_USAGE_WINDOW:              usize = 17;
const DAILY_USAGE_WINDOW:               usize = 18;
const REMOTE_DOMAIN_DAILY_USAGE_WINDOW: usize = 19;
const WEEKLY_USAGE_WINDOW:              usize = 20;
const MONTHLY_USAGE_WINDOW:             usize = 21;

const DISTINCT_ACCOUNT_INDEXES:      &[usize] = &[
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    PAUSE_STATE,
    ASSET_PAUSE_STATE,
    PATHWAY_POLICY,
    REMOTE_DOMAIN_POLICY,
    SIGNER_SET,
    REMOTE_NONCE_ACCOUNT_INDEX,
    RECIPIENT_ISSUED_TOKEN_ACCOUNT,
    ISSUED_TOKEN_MINT,
    MINT_AUTHORITY_PDA,
    ISSUED_TOKEN_PROGRAM,
    RELAYER,
    RELAYER_PERMISSION_RECORD,
    ISSUED_TOKEN_CONTROL,
    ASSET_CONFIG,
    LIMIT_POLICY,
    HOURLY_USAGE_WINDOW,
    DAILY_USAGE_WINDOW,
    REMOTE_DOMAIN_DAILY_USAGE_WINDOW,
    WEEKLY_USAGE_WINDOW,
    MONTHLY_USAGE_WINDOW,
];

// ─── Args ─────────────────────────────────────────────────────────────────────
#[derive(BorshDeserialize)]
pub struct ConsumeInboundMessageArgs {
    /// Local pathway selector. Must resolve to a `CROSS_CHAIN_MINT` pathway.
    pub pathway_id:                [u8; 32],

    /// Source remote chain-family discriminant. `chain_kind::SOLANA` is valid
    /// for another SVM deployment; corridor identity is not inferred from this
    /// discriminant alone.
    pub remote_chain_kind:         u8,

    /// Source remote domain id, BE-encoded into policy/nonce PDA seeds.
    pub remote_domain_id:          u64,

    // ── Canon scalars ──
    /// Must be `cross_chain_message_kind::INBOUND_MINT_FROM_REMOTE_BURN`.
    pub message_kind:              u8,

    /// Must equal `RemoteNonce.next_inbound_nonce` (strict in-order).
    pub source_nonce:              u64,

    /// Amount to mint on Solana (u64; widened to u128 for the canon hash).
    pub amount:                    u64,

    /// Unix expiry. 0 = no expiry. Otherwise must be > current unix_timestamp.
    pub expires_at_unix_timestamp: i64,

    // ── Canon 32-byte fields not loadable from on-chain state ──
    /// Source-chain asset (informational, not validated source-side).
    pub source_asset:              [u8; 32],

    /// Sender on the remote chain (informational; left-padded for EVM).
    pub sender:                    [u8; 32],

    /// Recipient - must equal the `owner` field of `recipient_token_account`.
    /// For Solana this is a 32-byte Pubkey.
    pub recipient:                 [u8; 32],

    // ── Provided hash (must match recomputed canonical hash) ──
    pub provided_message_hash:     [u8; 32],

    // ── Attestation bundle ──
    pub signatures:                Vec<AttestationSignature>,
}

// ─── Handler ──────────────────────────────────────────────────────────────────
pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    if args_data.len() > MAX_ATTESTED_INSTRUCTION_DATA_LEN {
        return Err(ChanceryError::AttestationResourceLimitExceeded.into());
    }

    assert_distinct_cross_chain_account_indexes(accounts, DISTINCT_ACCOUNT_INDEXES)?;

    let chancery_config_account_info           = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info           = &accounts[EVENT_AUTHORITY];
    let pause_state_account_info               = &accounts[PAUSE_STATE];
    let asset_pause_state_account_info         = &accounts[ASSET_PAUSE_STATE];
    let pathway_policy_account_info            = &accounts[PATHWAY_POLICY];
    let remote_policy_account_info             = &accounts[REMOTE_DOMAIN_POLICY];
    let signer_set_account_info                = &accounts[SIGNER_SET];
    let remote_nonce_account_info              = &accounts[REMOTE_NONCE_ACCOUNT_INDEX];
    let recipient_issued_token_account_info    = &accounts[RECIPIENT_ISSUED_TOKEN_ACCOUNT];
    let issued_token_mint_account_info         = &accounts[ISSUED_TOKEN_MINT];
    let mint_authority_pda_account_info        = &accounts[MINT_AUTHORITY_PDA];
    let issued_token_program_account_info      = &accounts[ISSUED_TOKEN_PROGRAM];
    let relayer_account_info                   = &accounts[RELAYER];
    let relayer_permission_record_account_info = &accounts[RELAYER_PERMISSION_RECORD];
    let issued_token_control_account_info      = &accounts[ISSUED_TOKEN_CONTROL];
    let asset_config_account_info              = &accounts[ASSET_CONFIG];

    if !relayer_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !remote_nonce_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !recipient_issued_token_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !issued_token_mint_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let args = ConsumeInboundMessageArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let clock      = Clock::get()?;
    let mut rolled_windows: Vec<ClosedPeriod> = Vec::new();
    let program_id = crate::id();

    // ── Chancery + global pause check; copy out fields needed after load_mut ─
    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    if &chancery_config.issued_token_program != issued_token_program_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }
    if &chancery_config.mint_authority_pda != mint_authority_pda_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    let chancery_domain_separator = chancery_config.domain_separator;

    let pause_state_bump = PauseState::verify_pda(pause_state_account_info, &program_id)?;
    let pause_state = PauseState::load_for_verified_pda(pause_state_account_info, pause_state_bump)?;
    pause_state.assert_not_paused_for(pause_bit::MINT, clock.slot)?;

    // ── message_kind allowlist (inbound only) ─────────────────────────────────
    if args.message_kind != cross_chain_message_kind::INBOUND_MINT_FROM_REMOTE_BURN {
        return Err(ChanceryError::MessageKindNotSupported.into());
    }

    // ── Expiry: zero = no expiry; otherwise must be strictly future ───────────
    if args.expires_at_unix_timestamp != 0 && clock.unix_timestamp >= args.expires_at_unix_timestamp {
        return Err(ChanceryError::MessageExpired.into());
    }

    // ── Authority: relayer holds CAN_CONSUME_INBOUND_MESSAGE at GLOBAL ────────
    assert_signer_holds_role(
        relayer_account_info,
        relayer_permission_record_account_info,
        role::CAN_CONSUME_INBOUND_MESSAGE,
        scope::GLOBAL,
        &Pubkey::default(),
        &program_id,
    )?;

    // ── Pathway validation ────────────────────────────────────────────────────
    let pathway_policy_bump = PathwayPolicy::verify_pda(pathway_policy_account_info, &args.pathway_id, &program_id)?;

    let pathway = PathwayPolicy::load_for_verified_pda(
        pathway_policy_account_info,
        &args.pathway_id,
        pathway_policy_bump,
    )?;

    pathway.assert_active()?;
    pathway.assert_kind(pathway_kind::CROSS_CHAIN_MINT)?;
    
    if pathway.has_evidence_policy() {
        return Err(ChanceryError::PathwayDependencyMissing.into());
    }

    // Cross-chain minting is another mint surface for the pathway collateral
    // asset. It must honor the same per-asset containment switch as direct,
    // delegated, and trilateral settlement.
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
        asset_pause_state.assert_not_paused_for(pause_bit::MINT, clock.slot)?;
    }

    // Cross-chain issuance must consume the same collateral lifecycle and
    // extension-containment policy as every local mint surface. The canonical
    // AssetConfig PDA is derived from the pathway's committed collateral mint.
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
    asset_config.assert_deposits_permitted()?;
    asset_config.assert_extensions_approved()?;
    pathway.assert_collateral_extensions_allowed(asset_config.observed_extension_mask)?;
    drop(asset_config);

    assert_canonical_issued_token_mint(&chancery_config, &pathway, issued_token_mint_account_info)?;

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

    remote_policy.assert_not_paused_for(
        remote_domain_pause_bit::INBOUND_MESSAGES
            | remote_domain_pause_bit::REMOTE_MINT
            | remote_domain_pause_bit::ATTESTATION_ACCEPTANCE
            | remote_domain_pause_bit::DAUGHTER_CONTRACT,
    )?;

    // Symmetric with outbound emission. A zero-value inbound message moves no
    // value but would otherwise retire a strict nonce and consume corridor
    // capacity, so reject it before caps, nonce mutation, or token CPI.
    if args.amount == 0 {
        return Err(ChanceryError::AmountMustBePositive.into());
    }

    remote_policy.assert_per_message_within_cap(args.amount)?;
    remote_policy.assert_expiry_within_window(args.expires_at_unix_timestamp, clock.unix_timestamp)?;

    // Bind message_kind to the policy's mode and the message's claimed
    // remote source asset. Without this an INBOUND_MINT_FROM_REMOTE_BURN
    // can be consumed through a RELEASE-mode policy (or vice versa) and the
    // source_asset field is effectively informational.
    assert_inbound_remote_asset_binding(
        args.message_kind,
        &args.source_asset,
        &remote_policy,
    )?;

    // ── Signer set: PDA, active, temporally valid, id matches policy ──────────
    let signer_set_bump = CrossChainSignerSet::verify_pda(signer_set_account_info, &remote_policy.signer_set_id, &program_id)?;

    let signer_set = CrossChainSignerSet::load_for_verified_pda(
        signer_set_account_info,
        &remote_policy.signer_set_id,
        signer_set_bump,
    )?;

    signer_set.assert_active()?;
    signer_set.assert_temporally_valid(clock.unix_timestamp)?;
    remote_policy.assert_signer_set_id_matches(&signer_set.signer_set_id)?;

    // ── RemoteNonce: PDA + strict in-order ────────────────────────────────────
    let nonce_scope_key = [0u8; 32];
    let remote_nonce_bump = RemoteNonce::verify_pda(
        remote_nonce_account_info,
        args.remote_chain_kind,
        args.remote_domain_id,
        &nonce_scope_key,
        &program_id,
    )?;

    {
        let n = RemoteNonce::load_for_verified_pda(
            remote_nonce_account_info,
            args.remote_domain_id,
            &nonce_scope_key,
            remote_nonce_bump,
        )?;
        n.assert_inbound_nonce(args.source_nonce)?;
    }

    // ── Recipient token account: validate owner + mint binding and pin the
    //    credited account to the canonical ATA. The attested recipient is the
    //    owner identity; the relayer must not select another recipient-owned
    //    account with pre-existing delegate, freeze, or custody semantics. ────
    let recipient_owner = Pubkey::new_from_array(args.recipient);

    crate::modules::settlement::instructions::assert_spl_token_account_binding(
        recipient_issued_token_account_info,
        issued_token_program_account_info,
        &pathway.issued_token_mint,
        &recipient_owner,
    )?;

    let (expected_recipient_token_account, _) = Pubkey::find_program_address(
        &[
            recipient_owner.as_ref(),
            issued_token_program_account_info.key.as_ref(),
            pathway.issued_token_mint.as_ref(),
        ],
        &programs::ASSOCIATED_TOKEN_PROGRAM_ID,
    );

    if recipient_issued_token_account_info.key != &expected_recipient_token_account {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    // ── Recompute canonical message hash from policy + pathway + args. ───────
    //    Pubkey → [u8; 32] copies are stack-bound for reference-lifetime.
    let destination_chancery_bytes = chancery_config_account_info.key.to_bytes();
    let destination_asset_bytes    = pathway.asset_mint.to_bytes();
    let destination_issued_bytes   = pathway.issued_token_mint.to_bytes();

    let preimage = MessageHashPreimage {
        message_kind:                  args.message_kind,
        source_chain_kind:             remote_policy.remote_chain_kind,
        destination_chain_kind:        chain_kind::SOLANA,
        source_domain_id:              remote_policy.remote_domain_id,
        destination_domain_id:         SOLANA_SELF_DOMAIN_ID,
        source_nonce:                  args.source_nonce,
        amount:                        args.amount as u128,
        expires_at_unix_timestamp:     args.expires_at_unix_timestamp,
        source_chancery_contract:      &remote_policy.remote_chancery_contract,
        destination_chancery_contract: &destination_chancery_bytes,
        source_domain_separator:       &remote_policy.remote_domain_separator,
        destination_domain_separator:  &chancery_domain_separator,
        source_asset:                  &args.source_asset,
        destination_asset:             &destination_asset_bytes,
        source_issued_token:           &remote_policy.remote_issued_token,
        destination_issued_token:      &destination_issued_bytes,
        sender:                        &args.sender,
        recipient:                     &args.recipient,
        signer_set_id:                 &remote_policy.signer_set_id,
    };

    let message_hash = compute_message_hash(&preimage);

    if message_hash != args.provided_message_hash {
        return Err(ChanceryError::MessageHashMismatch.into());
    }

    // ── Verify attestation quorum (the cryptographic authority) ──────────────
    verify_attestation_quorum(
        &message_hash,
        &signer_set.signer_root,
        signer_set.threshold,
        signer_set.signer_count,
        remote_policy.minimum_attestation_threshold,
        &args.signatures,
    )?;

    // ── Pathway-scoped limit-policy enforcement (optional) ────────────────────
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
                hourly_usage_window.gross_in,
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
                daily_usage_window.gross_in,
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
                weekly_usage_window.gross_in,
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
                monthly_usage_window.gross_in,
                args.amount,
                ChanceryError::MonthlyLimitBreached,
            )?;

            monthly_prepared = Some(monthly_usage_window);
        }
    }

    // ── Remote-domain per-day corridor cap. Symmetric with emit_outbound:
    //    inflow on this side accrues against the same `per_day_maximum` cap as
    //    outflow does. Optional account; required iff `per_day_maximum != 0`. ────
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
            rdw.gross_in,
            args.amount,
        )?;

        remote_prepared = Some(rdw);
    }

    // ── Exactly-once boundary: bump RemoteNonce.next_inbound_nonce ────────────
    // Strict equality was asserted before any cryptographic work; the atomic
    // bump here retires this source_nonce forever. A replay of the same
    // message re-presents the retired nonce and fails `RemoteNonceMismatch`.
    // The hash cursor is operational reconciliation only.
    {
        let mut n = RemoteNonce::load_mut_for_verified_pda(
            remote_nonce_account_info,
            args.remote_domain_id,
            &nonce_scope_key,
            remote_nonce_bump,
        )?;

        n.bump_inbound()?;

        n.last_consumed_message_hash = message_hash;
    }

    // ── Mint CPI: signed as mint_authority PDA ────────────────────────────────
    let mint_authority_b = chancery_config.resolved_mint_authority_bump(&program_id);

    cpi_mint_to(
        issued_token_program_account_info,
        issued_token_mint_account_info,
        recipient_issued_token_account_info,
        mint_authority_pda_account_info,
        mint_authority_b,
        args.amount,
    )?;

    // ── Update pathway-scoped usage windows. Mint = inflow on the
    //    destination side. Enforced windows were required present + writable and
    //    PDA-verified above, so record unconditionally - no `is_writable` escape.
    if let Some(prepared_window) = hourly_prepared {
        prepared_window.record_inflow(&accounts[HOURLY_USAGE_WINDOW], args.amount)?;
    }

    if let Some(prepared_window) = daily_prepared {
        prepared_window.record_inflow(&accounts[DAILY_USAGE_WINDOW], args.amount)?;
    }

    if let Some(prepared_window) = weekly_prepared {
        prepared_window.record_inflow(&accounts[WEEKLY_USAGE_WINDOW], args.amount)?;
    }

    if let Some(prepared_window) = monthly_prepared {
        prepared_window.record_inflow(&accounts[MONTHLY_USAGE_WINDOW], args.amount)?;
    }

    // ── Update remote-domain usage window. Corridor-level inflow;
    //    required present + writable above when the corridor caps daily flow. ───
    if let Some(prepared_window) = remote_prepared {
        prepared_window.record_inflow(&accounts[REMOTE_DOMAIN_DAILY_USAGE_WINDOW], args.amount)?;
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

    emit_cross_chain_inbound_consumed(
        event_authority_account_info,
        event_authority_bump,
        CrossChainInboundConsumed {
            sequence_nonce:                seq,
            chancery:                      *chancery_config_account_info.key,
            slot:                          clock.slot,
            unix_timestamp:                clock.unix_timestamp,
            message_hash:                  message_hash,
            canon_version:                 MESSAGE_HASH_CANON_VERSION,
            message_kind:                  args.message_kind,
            source_chain_kind:             remote_policy.remote_chain_kind,
            destination_chain_kind:        chain_kind::SOLANA,
            source_domain_id:              remote_policy.remote_domain_id,
            destination_domain_id:         SOLANA_SELF_DOMAIN_ID,
            source_nonce:                  args.source_nonce,
            amount:                        args.amount as u128,
            expires_at_unix_timestamp:     args.expires_at_unix_timestamp,
            source_chancery_contract:      remote_policy.remote_chancery_contract,
            destination_chancery_contract: destination_chancery_bytes,
            source_domain_separator:       remote_policy.remote_domain_separator,
            destination_domain_separator:  chancery_domain_separator,
            source_asset:                  args.source_asset,
            destination_asset:             destination_asset_bytes,
            source_issued_token:           remote_policy.remote_issued_token,
            destination_issued_token:      destination_issued_bytes,
            sender:                        args.sender,
            recipient:                     args.recipient,
            signer_set_id:                 remote_policy.signer_set_id,
            pathway_id:                    args.pathway_id,
            recipient_token_account:       *recipient_issued_token_account_info.key,
            consumed_by:                   *relayer_account_info.key,
            attestation_count:             args.signatures.len() as u8,
        },
    )?;

    Ok(())
}
