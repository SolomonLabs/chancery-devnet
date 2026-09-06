/// mint_direct - DirectPrincipalSettlement, mint path.
///
/// Flow (spec 04 §4.9):
///   1. validate global + asset pause
///   2. validate pathway policy (kind=DIRECT, active, asset match)
///   3. validate principal permission (CAN_MINT_DIRECT)
///   4. validate asset mode (deposits permitted)
///   5. validate amounts
///   6. check limit policy
///   7. compute gross output from deposit_rate_e9
///   8. apply fee policy -> net_output_amount
///   9. CPI: transfer asset_amount from principal -> reserve_asset_token_account
///  10. CPI: mint_to net_output_amount -> principal issued-token destination
///  11. update UsageWindow(s)
///  12. emit SettlementEvidence (signed CPI to events_cpi::EMIT)
///
/// Settlement is dispatch-gated: wire account 0 is the module activation
/// state; indices below are handler-local after that prefix. Clients built
/// from the generated IDL also append the chancery program itself as the
/// final `event_program` account for the evidence self-CPI.
///
/// Accounts (fixed):
///   0  chancery_config                    writable  PDA
///   1  event_authority                    readable  PDA [b"event-authority"]
///   2  pause_state                        readable  PDA [b"pause-state"]
///   3  asset_config                       readable  PDA [b"asset-config", asset_mint]
///   4  pathway_policy                     readable  PDA [b"pathway-policy", pathway_id]
///   5  permission_record                  readable  PDA [b"permission", principal, scope, scope_key]
///   6  source_asset_token_account         writable  principal's collateral token account
///   7  reserve_asset_token_account        writable  program-owned reserve token account
///   8  destination_issued_token_account   writable  principal's issued-token account
///   9  asset_mint                         readable
///  10  issued_token_mint                  writable
///  11  mint_authority_pda                 readable  PDA [b"mint-authority"]
///  12  asset_token_program                readable
///  13  issued_token_program               readable
///  14  principal                          signer
///  15  asset_pause_state                  readable  PDA [b"asset-pause", asset_mint]
///  16  issued_token_control               readable  PDA [b"issued-token-control"]
///
/// Optional (present only when pathway_policy references them, checked by key):
///  17  fee_policy                         readable  (zero address = skip)
///  18  fee_recipient_token_account        writable  required iff fee_policy charges a non-zero fee
///  19  limit_policy                       readable
///  20  hourly_usage_window                writable
///  21  daily_usage_window                 writable
///  22  weekly_usage_window                writable
///  23  monthly_usage_window               writable
///  24  evidence_policy                    readable
///
/// Dimension-limit optionals (simultaneous multi-scope limits; present only
/// when pathway_policy references the dimension, checked by key):
///  25  asset_limit_policy                 readable
///  26  asset_daily_usage_window           writable
///  27  counterparty_limit_policy          readable
///  28  counterparty_daily_usage_window    writable

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    account_security::assert_external_signer,
    constants::{pathway_kind, programs, role, scope, settlement_mode, status_flag, window_kind},
    error::ChanceryError,
    modules::{
        control::state::{
            asset_pause_state::AssetPauseState,
            pause_state::{pause_bit, PauseState},
        },
        core::state::{asset_config::AssetConfig, chancery_config::ChanceryConfig},
        evidence::emit::{emit_usage_windows_rolled, emit_settlement_mint, SettlementMint},
        fees::state::fee_policy::FeePolicy,
        limits::{
            dimension::{
                prepare_dimension_window_for_recording, asset_dimension_binding,
                counterparty_dimension_binding,
                load_dimension_policy,
            },
            state::{
                limit_policy::LimitPolicy,
                usage_window::{prepare_enforced_window_for_recording, ClosedPeriod},
            },
        },
        pathway::state::pathway_policy::PathwayPolicy,
        permissions::{
            auth::assert_subject_holds_role,
        },
        settlement::instructions::{
            assert_canonical_issued_token_mint, assert_distinct_settlement_account_indexes,
            assert_issued_token_extension_gates, assert_pathway_evidence_policy,
            assert_spl_token_account_binding, cpi_mint_to_with_decimals, cpi_transfer_asset_in, read_mint_decimals,
        },
    },
};

// ─── Account indices ──────────────────────────────────────────────────────────
const CHANCERY_CONFIG:                  usize = 0;
const EVENT_AUTHORITY:                  usize = 1;
const PAUSE_STATE:                      usize = 2;
const ASSET_CONFIG:                     usize = 3;
const PATHWAY_POLICY:                   usize = 4;
const PERMISSION_RECORD:                usize = 5;
const SOURCE_ASSET_TOKEN_ACCOUNT:       usize = 6;
const RESERVE_ASSET_TOKEN_ACCOUNT:      usize = 7;
const DESTINATION_ISSUED_TOKEN_ACCOUNT: usize = 8;
const ASSET_MINT:                       usize = 9;
const ISSUED_TOKEN_MINT:                usize = 10;
const MINT_AUTHORITY_PDA:               usize = 11;
const ASSET_TOKEN_PROGRAM:              usize = 12;
const ISSUED_TOKEN_PROGRAM:             usize = 13;
const PRINCIPAL:                        usize = 14;
const ASSET_PAUSE_STATE:                usize = 15;
const ISSUED_TOKEN_CONTROL:             usize = 16;
const REQUIRED_ACCOUNT_COUNT:           usize = 17;

// Optional indices
const FEE_POLICY:                       usize = 17;
const FEE_RECIPIENT_TOKEN_ACCOUNT:      usize = 18;
const LIMIT_POLICY:                     usize = 19;
const HOURLY_USAGE_WINDOW:              usize = 20;
const DAILY_USAGE_WINDOW:               usize = 21;
const WEEKLY_USAGE_WINDOW:              usize = 22;
const MONTHLY_USAGE_WINDOW:             usize = 23;
const EVIDENCE_POLICY:                  usize = 24;
const ASSET_LIMIT_POLICY:               usize = 25;
const ASSET_DAILY_USAGE_WINDOW:         usize = 26;
const COUNTERPARTY_LIMIT_POLICY:        usize = 27;
const COUNTERPARTY_DAILY_USAGE_WINDOW:  usize = 28;

const DISTINCT_ACCOUNT_INDEXES:      &[usize] = &[
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    PAUSE_STATE,
    ASSET_CONFIG,
    PATHWAY_POLICY,
    PERMISSION_RECORD,
    SOURCE_ASSET_TOKEN_ACCOUNT,
    RESERVE_ASSET_TOKEN_ACCOUNT,
    DESTINATION_ISSUED_TOKEN_ACCOUNT,
    ASSET_MINT,
    ISSUED_TOKEN_MINT,
    MINT_AUTHORITY_PDA,
    PRINCIPAL,
    ASSET_PAUSE_STATE,
    ISSUED_TOKEN_CONTROL,
    FEE_POLICY,
    FEE_RECIPIENT_TOKEN_ACCOUNT,
    LIMIT_POLICY,
    HOURLY_USAGE_WINDOW,
    DAILY_USAGE_WINDOW,
    WEEKLY_USAGE_WINDOW,
    MONTHLY_USAGE_WINDOW,
    EVIDENCE_POLICY,
    ASSET_LIMIT_POLICY,
    ASSET_DAILY_USAGE_WINDOW,
    COUNTERPARTY_LIMIT_POLICY,
    COUNTERPARTY_DAILY_USAGE_WINDOW,
];

#[derive(BorshDeserialize)]
pub struct MintDirectArgs {
    pub pathway_id:   [u8; 32],

    /// Gross asset amount to deposit.
    pub asset_amount: u64,

    /// Slippage floor on issued-token output. 0 = disabled.
    pub minimum_issued_token_amount: u64,
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let program_id = crate::id();

    assert_distinct_settlement_account_indexes(accounts, DISTINCT_ACCOUNT_INDEXES)?;

    let chancery_config_account_info          = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info          = &accounts[EVENT_AUTHORITY];
    let pause_state_account_info              = &accounts[PAUSE_STATE];
    let asset_config_account_info             = &accounts[ASSET_CONFIG];
    let pathway_policy_account_info           = &accounts[PATHWAY_POLICY];
    let permission_record_account_info        = &accounts[PERMISSION_RECORD];
    let source_asset_token_account_info       = &accounts[SOURCE_ASSET_TOKEN_ACCOUNT];
    let reserve_asset_token_account_info      = &accounts[RESERVE_ASSET_TOKEN_ACCOUNT];
    let destination_issued_token_account_info = &accounts[DESTINATION_ISSUED_TOKEN_ACCOUNT];
    let asset_mint_account_info               = &accounts[ASSET_MINT];
    let issued_token_mint_account_info        = &accounts[ISSUED_TOKEN_MINT];
    let mint_authority_pda_account_info       = &accounts[MINT_AUTHORITY_PDA];
    let asset_token_program_account_info      = &accounts[ASSET_TOKEN_PROGRAM];
    let issued_token_program_account_info     = &accounts[ISSUED_TOKEN_PROGRAM];
    let principal_account_info                = &accounts[PRINCIPAL];
    let asset_pause_state_account_info        = &accounts[ASSET_PAUSE_STATE];
    let issued_token_control_account_info     = &accounts[ISSUED_TOKEN_CONTROL];

    // ── Signer / writable guards ──────────────────────────────────────────────
    assert_external_signer(principal_account_info, &program_id)?;

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !source_asset_token_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !reserve_asset_token_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !destination_issued_token_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !issued_token_mint_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    // ── Args ──────────────────────────────────────────────────────────────────
    let args = MintDirectArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let clock = Clock::get()?;
    let mut rolled_windows: Vec<ClosedPeriod> = Vec::new();

    // ── 1. Global pause ───────────────────────────────────────────────────────
    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;


    let pause_state_bump = PauseState::verify_pda(pause_state_account_info, &program_id)?;
    let pause_state = PauseState::load_for_verified_pda(pause_state_account_info, pause_state_bump)?;
    pause_state.assert_not_paused_for(pause_bit::MINT, clock.slot)?;

    // ── 1b. Per-asset pause. PDA is verified so a foreign/empty account cannot
    //    be substituted to bypass; an uninitialized PDA means never paused. ────
    let asset_pause_state_bump = AssetPauseState::verify_pda(asset_pause_state_account_info, asset_mint_account_info.key, &program_id)?;

    if !asset_pause_state_account_info.data_is_empty() {
        let asset_pause_state = AssetPauseState::load_for_verified_pda(
            asset_pause_state_account_info,
            asset_mint_account_info.key,
            asset_pause_state_bump,
        )?;
        asset_pause_state.assert_not_paused_for(pause_bit::MINT, clock.slot)?;
    }

    // ── 2. Pathway policy ─────────────────────────────────────────────────────
    let pathway = PathwayPolicy::load_verified(pathway_policy_account_info)?;

    pathway.assert_active()?;
    pathway.assert_kind(pathway_kind::DIRECT)?;

    if &pathway.asset_mint != asset_mint_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    assert_canonical_issued_token_mint(&chancery_config, &pathway, issued_token_mint_account_info)?;

    // load_verified already proves the account is the canonical PDA for the
    // stored pathway_id, so bind the instruction argument without deriving it
    // a second time.
    if pathway.pathway_id != args.pathway_id {
        return Err(ChanceryError::InvalidPda.into());
    }

    // Evidence policy binds fail-closed: a referenced policy account is
    // mandatory (indexed here so the IDL derives it from source, issue RB-02).
    if pathway.has_evidence_policy() {
        if accounts.len() <= EVIDENCE_POLICY || accounts[EVIDENCE_POLICY].key == &Pubkey::default() {
            return Err(ChanceryError::MissingAccount.into());
        }

        assert_pathway_evidence_policy(&accounts[EVIDENCE_POLICY], &pathway)?;
    }

    // ── 3. Permission ─────────────────────────────────────────────────────────
    // Subject is the principal; scope is the pathway PDA address.
    let _permission_record = assert_subject_holds_role(
        principal_account_info.key,
        permission_record_account_info,
        role::CAN_MINT_DIRECT,
        scope::PATHWAY,
        pathway_policy_account_info.key,
        clock.unix_timestamp,
        &program_id,
    )?;

    // ── 4. Asset config ───────────────────────────────────────────────────────
    let asset_config = AssetConfig::load_verified(asset_config_account_info)?;

    asset_config.assert_extensions_fresh(clock.slot)?;
    asset_config.assert_deposits_permitted()?;
    asset_config.assert_extensions_approved()?;
    pathway.assert_collateral_extensions_allowed(asset_config.observed_extension_mask)?;

    assert_issued_token_extension_gates(
        issued_token_control_account_info,
        issued_token_mint_account_info,
        &pathway,
        clock.slot,
        &program_id,
    )?;

    if &asset_config.asset_mint != asset_mint_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if &asset_config.asset_token_program != asset_token_program_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if &chancery_config.issued_token_program != issued_token_program_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if &chancery_config.mint_authority_pda != mint_authority_pda_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    assert_spl_token_account_binding(
        source_asset_token_account_info,
        asset_token_program_account_info,
        &pathway.asset_mint,
        principal_account_info.key,
    )?;
    assert_spl_token_account_binding(
        destination_issued_token_account_info,
        issued_token_program_account_info,
        &pathway.issued_token_mint,
        principal_account_info.key,
    )?;

    // Audit #7: pin reserve to the per-asset ATA. ATA seeds encode
    // asset_mint, so SPL Token's source.mint == dest.mint check then
    // transitively constrains source.mint == asset_mint.
    {
        if reserve_asset_token_account_info.owner != asset_token_program_account_info.key {
            return Err(ChanceryError::TokenProgramMismatch.into());
        }

        let (reserve_authority, _) =
            chancery_config.resolved_reserve_authority(&program_id)?;

        let (expected_ata, _) = Pubkey::find_program_address(
            &[
                reserve_authority.as_ref(),
                asset_token_program_account_info.key.as_ref(),
                asset_mint_account_info.key.as_ref(),
            ],
            &programs::ASSOCIATED_TOKEN_PROGRAM_ID,
        );

        if reserve_asset_token_account_info.key != &expected_ata {
            return Err(ChanceryError::InvalidPda.into());
        }
    }

    // ── 5. Amount validation ──────────────────────────────────────────────────
    asset_config.assert_deposit_amount(args.asset_amount)?;

    // ── 6. Limit check (optional) ─────────────────────────────────────────────
    if pathway.has_limit_policy()
        && (accounts.len() <= LIMIT_POLICY
            || accounts[LIMIT_POLICY].key == &Pubkey::default())
    {
        return Err(ChanceryError::MissingAccount.into());
    }

    let has_limit = accounts.len() > LIMIT_POLICY
        && pathway.has_limit_policy()
        && accounts[LIMIT_POLICY].key != &Pubkey::default();

    // Whether the loaded policy actually caps each window. Derived from the
    // policy - never from which accounts the caller chose to pass - so an
    // enforced window cannot be silently skipped by omission or a read-only
    // meta. Hoisted so the update step (§11) records against exactly the
    // windows that were required and checked here.
    let mut hourly_prepared  = None;
    let mut daily_prepared   = None;
    let mut weekly_prepared  = None;
    let mut monthly_prepared = None;

    if has_limit {
        let limit_policy_bump = LimitPolicy::verify_pda(&accounts[LIMIT_POLICY], &pathway.limit_policy_id, &program_id)?;

        let limit_policy = LimitPolicy::load_for_verified_pda(
            &accounts[LIMIT_POLICY],
            &pathway.limit_policy_id,
            limit_policy_bump,
        )?;

        let scope_hash       = limit_policy.scope_hash();

        limit_policy.assert_per_tx(args.asset_amount)?;

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
                args.asset_amount,
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
                args.asset_amount,
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
                args.asset_amount,
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
                args.asset_amount,
                ChanceryError::MonthlyLimitBreached,
            )?;

            monthly_prepared = Some(monthly_usage_window);
        }
    }

    // ── Dimension limits (spec §5.8) ──────────────────────────────────────────
    // Asset-denominated per-transaction + fixed UTC-day caps derived from
    // pathway state - the handler, not the caller, selects which dimensions
    // are enforced and which policy / window PDAs are expected. Template
    // dimensions (counterparty / executor) share one policy but accrue in
    // per-party daily windows.
    let (asset_scope_kind, asset_scope_key) = asset_dimension_binding(asset_mint_account_info.key);
    let asset_daily_prepared = if pathway.has_asset_mint_limit_policy() {
        if accounts.len() <= ASSET_LIMIT_POLICY || accounts[ASSET_LIMIT_POLICY].key == &Pubkey::default() {
            return Err(ChanceryError::MissingAccount.into());
        }

        let policy = load_dimension_policy(
            &accounts[ASSET_LIMIT_POLICY],
            &pathway.asset_mint_limit_policy_id,
            asset_scope_kind,
            &asset_scope_key,
            &program_id,
        )?;

        policy.assert_per_tx(args.asset_amount)?;

        if policy.per_day_maximum != 0 {
            Some(prepare_dimension_window_for_recording(
                accounts,
                ASSET_DAILY_USAGE_WINDOW,
                &policy,
                asset_scope_kind,
                &asset_scope_key,
                args.asset_amount,
                true,
                clock.unix_timestamp,
                &program_id,
                &mut rolled_windows,
            )?)
        } else {
            None
        }
    } else {
        None
    };

    let (cp_scope_kind, cp_scope_key) = counterparty_dimension_binding();
    let counterparty_daily_prepared = if pathway.has_counterparty_limit_policy() {
        if accounts.len() <= COUNTERPARTY_LIMIT_POLICY || accounts[COUNTERPARTY_LIMIT_POLICY].key == &Pubkey::default() {
            return Err(ChanceryError::MissingAccount.into());
        }

        let policy = load_dimension_policy(
            &accounts[COUNTERPARTY_LIMIT_POLICY],
            &pathway.counterparty_limit_policy_id,
            cp_scope_kind,
            &cp_scope_key,
            &program_id,
        )?;

        policy.assert_per_tx(args.asset_amount)?;

        if policy.per_day_maximum != 0 {
            Some(prepare_dimension_window_for_recording(
                accounts,
                COUNTERPARTY_DAILY_USAGE_WINDOW,
                &policy,
                cp_scope_kind,
                principal_account_info.key,
                args.asset_amount,
                true,
                clock.unix_timestamp,
                &program_id,
                &mut rolled_windows,
            )?)
        } else {
            None
        }
    } else {
        None
    };

    // ── 7. CPI: transfer collateral into the reserve, measuring actual receipt ─
    // Issuance is based on what the reserve ACTUALLY receives, not the caller-
    // supplied amount. A Token-2022 collateral with an active TransferFeeConfig
    // (or a transfer hook that skims) credits the reserve with less than
    // `asset_amount`; minting against the requested amount would
    // under-collateralize the issued token (issue-73). The transfer runs before
    // the rate so output is computed on the measured delta. Atomicity keeps this
    // safe: any revert below rolls the transfer back too.
    let received = cpi_transfer_asset_in(
        asset_token_program_account_info,
        source_asset_token_account_info,
        asset_mint_account_info,
        reserve_asset_token_account_info,
        principal_account_info,
        args.asset_amount,
    )?;

    // ── 8. Rate computation (on the amount actually credited to the reserve) ──
    let gross_output_amount = asset_config.compute_mint_output(received)?;

    if gross_output_amount == 0 {
        return Err(ChanceryError::NetOutputZero.into());
    }

    // ── 9. Fee application (optional) ─────────────────────────────────────────
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
        fee_recipient_owner,
    ) =
        if accounts.len() > FEE_POLICY
            && pathway.has_fee_policy()
            && accounts[FEE_POLICY].key != &Pubkey::default()
        {
            let fee_policy_bump = FeePolicy::verify_pda(&accounts[FEE_POLICY], &pathway.fee_policy_id, &program_id)?;

            let fp = FeePolicy::load_for_verified_pda(
                &accounts[FEE_POLICY],
                &pathway.fee_policy_id,
                fee_policy_bump,
            )?;

            fp.assert_effective(clock.unix_timestamp)?;

            let computation = fp.compute_issued_token_fee_with_input(received, gross_output_amount)?;

            // Validation and routing share this exact predicate. Retention
            // policies never inspect or transfer to the optional recipient slot.
            let route_fee_to_recipient = computation.net_fee > 0 && fp.routes_fee_to_recipient();

            if route_fee_to_recipient {
                if accounts.len() <= FEE_RECIPIENT_TOKEN_ACCOUNT
                    || accounts[FEE_RECIPIENT_TOKEN_ACCOUNT].key == &Pubkey::default()
                {
                    return Err(ChanceryError::MissingAccount.into());
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
                fp.fee_recipient_key,
            )
        } else {
            (0u64, 0u64, gross_output_amount, [0u8; 32], false, Pubkey::default())
        };

    // Slippage floor - also guards against an unexpectedly high collateral
    // transfer fee: if the fee ate more than the minter tolerates, `net` falls
    // below the floor and the whole tx reverts (returning the collateral).
    crate::modules::settlement::assert_output_meets_minimum(
        net_output_amount,
        args.minimum_issued_token_amount,
        ChanceryError::AmountBelowMinimum,
    )?;

    // ── 10. CPI: mint net_output_amount to principal ─────────────────────────────────
    let bump = chancery_config.resolved_mint_authority_bump(&program_id);
    let issued_token_decimals = read_mint_decimals(
        issued_token_mint_account_info,
        issued_token_program_account_info,
    )?;

    cpi_mint_to_with_decimals(
        issued_token_program_account_info,
        issued_token_mint_account_info,
        destination_issued_token_account_info,
        mint_authority_pda_account_info,
        bump,
        net_output_amount,
        issued_token_decimals,
    )?;

    // ── 10b. CPI: route the net fee to the fee recipient (issued-token fee). ──
    // net_output_amount already credits the rebate back to the principal, so the
    // recipient's share is gross − net (= fee − rebate). Routing the pre-rebate
    // fee_output_amount would mint the rebate a second time, exceeding
    // gross_output_amount.
    let net_fee_output_amount = gross_output_amount
        .checked_sub(net_output_amount)
        .ok_or(ChanceryError::ArithmeticUnderflow)?;

    if route_fee_to_recipient && net_fee_output_amount > 0 {
        cpi_mint_to_with_decimals(
            issued_token_program_account_info,
            issued_token_mint_account_info,
            &accounts[FEE_RECIPIENT_TOKEN_ACCOUNT],
            mint_authority_pda_account_info,
            bump,
            net_fee_output_amount,
            issued_token_decimals,
        )?;
    }

    // ── 11. Update usage windows ──────────────────────────────────────────────
    // Enforced windows were required present + writable and PDA-verified in §6,
    // so record unconditionally. No `is_writable` escape hatch: a read-only
    // window fails closed at `load_mut` rather than silently skipping the record
    // and letting the fixed-window cap never accrue.
    //
    // Windows deliberately accrue the requested `args.asset_amount`, not the
    // measured `received`: for transfer-fee collateral this over-accrues the
    // cap (received <= requested), which is the fail-safe direction, and it
    // matches the amount the limit checks in §6 were evaluated against.
    if let Some(prepared_window) = hourly_prepared {
        prepared_window.record_inflow(&accounts[HOURLY_USAGE_WINDOW], args.asset_amount)?;
    }

    if let Some(prepared_window) = daily_prepared {
        prepared_window.record_inflow(&accounts[DAILY_USAGE_WINDOW], args.asset_amount)?;
    }

    if let Some(prepared_window) = weekly_prepared {
        prepared_window.record_inflow(&accounts[WEEKLY_USAGE_WINDOW], args.asset_amount)?;
    }

    if let Some(prepared_window) = monthly_prepared {
        prepared_window.record_inflow(&accounts[MONTHLY_USAGE_WINDOW], args.asset_amount)?;
    }

    if let Some(prepared_window) = asset_daily_prepared {
        prepared_window.record_inflow(&accounts[ASSET_DAILY_USAGE_WINDOW], args.asset_amount)?;
    }

    if let Some(prepared_window) = counterparty_daily_prepared {
        prepared_window.record_inflow(&accounts[COUNTERPARTY_DAILY_USAGE_WINDOW], args.asset_amount)?;
    }

    // ── 12. Evidence ──────────────────────────────────────────────────────────
    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;

    emit_usage_windows_rolled(
        event_authority_account_info,
        event_authority_bump,
        &mut chancery_config_mut,
        chancery_config_account_info.key,
        &clock,
        &rolled_windows,
    )?;
    let seq                     = chancery_config_mut.next_sequence_nonce()?;

    emit_settlement_mint(event_authority_account_info, event_authority_bump, SettlementMint {
        sequence_nonce:         seq,
        chancery:               *chancery_config_account_info.key,
        slot:                   clock.slot,
        unix_timestamp:         clock.unix_timestamp,
        pathway_id:             args.pathway_id,
        settlement_mode:        settlement_mode::DIRECT_PRINCIPAL,
        intent_id:              [0u8; 32],
        settlement_policy_id:   [0u8; 32],
        limit_policy_id:        pathway.limit_policy_id,
        evidence_policy_id:     pathway.evidence_policy_id,
        fee_policy_id:          fee_policy_id,
        insurance_policy_id:    pathway.insurance_policy_id,
        principal_a:            *principal_account_info.key,
        principal_b:            *principal_account_info.key,
        executor:               Pubkey::default(),
        asset_mint:             *asset_mint_account_info.key,
        issued_token_mint:      *issued_token_mint_account_info.key,
        asset_token_program:    *asset_token_program_account_info.key,
        issued_token_program:   *issued_token_program_account_info.key,
        source_account:         *source_asset_token_account_info.key,
        destination_account:    *destination_issued_token_account_info.key,
        fee_recipient_owner: if route_fee_to_recipient {
            fee_recipient_owner
        } else {
            Pubkey::default()
        },
        fee_recipient_token_account: if route_fee_to_recipient {
            *accounts[FEE_RECIPIENT_TOKEN_ACCOUNT].key
        } else {
            Pubkey::default()
        },
        reserve_compartment_id: [0u8; 32],
        gross_amount_in:        args.asset_amount,
        gross_amount_out:       gross_output_amount,
        fee_output_amount,
        rebate_amount,
        net_output_amount,
        status_flags:           status_flag::INITIALIZED,
        reason_code:            0,
        actual_asset_amount_in: received,
    })?;

    Ok(())
}
