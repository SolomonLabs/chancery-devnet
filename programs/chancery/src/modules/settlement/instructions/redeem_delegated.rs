/// redeem_delegated - DelegatedNonCustodialExecution, redeem path.
/// Structurally mirrors mint_delegated with burn + transfer-out flow.
///
/// Settlement is dispatch-gated: wire account 0 is the module activation
/// state; indices below are handler-local after that prefix. Clients built
/// from the generated IDL also append the chancery program itself as the
/// final `event_program` account for the evidence self-CPI.
///
/// Accounts (fixed):
///   0  chancery_config                        writable
///   1  event_authority                        readable  PDA [b"event-authority"]
///   2  pause_state                            readable
///   3  asset_config                           readable
///   4  pathway_policy                         readable
///   5  settlement_intent                      writable
///   6  principal_permission_record            readable
///   7  executor_permission_record             readable
///   8  source_issued_token_account            writable  principal burns from here
///   9  reserve_asset_token_account            writable
///  10  destination_asset_token_account        writable  principal receives asset
///  11  asset_mint                             readable
///  12  issued_token_mint                      writable
///  13  reserve_authority_pda                  readable
///  14  asset_token_program                    readable
///  15  issued_token_program                   readable
///  16  executor                               signer
///  17  principal                              readable
///  18  asset_pause_state                      readable  PDA [b"asset-pause", asset_mint]
///  19  issued_token_control                   readable  PDA [b"issued-token-control"]

/// Optional (20-28): fee_policy, fee_recipient_token_account, limit_policy,
/// hourly_usage_window, daily_usage_window, weekly_usage_window,
/// monthly_usage_window, evidence_policy, settlement_policy.
/// Dimension-limit optionals (29-34): asset_limit_policy,
/// asset_daily_usage_window, counterparty_limit_policy,
/// counterparty_daily_usage_window, executor_limit_policy,
/// executor_daily_usage_window.

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    account_security::{assert_external_identity, assert_external_signer},
    constants::{pathway_kind, programs, role, scope, settlement_mode, status_flag, window_kind},
    error::ChanceryError,
    modules::{
        control::state::{
            asset_pause_state::AssetPauseState,
            pause_state::{pause_bit, PauseState},
        },
        core::{
            instructions::close_pda_to_recipient_account,
            state::{asset_config::AssetConfig, chancery_config::ChanceryConfig},
        },
        evidence::emit::{emit_usage_windows_rolled, emit_settlement_redeem, SettlementRedeem},
        fees::state::fee_policy::FeePolicy,
        limits::{
            dimension::{
                prepare_dimension_window_for_recording, asset_dimension_binding,
                counterparty_dimension_binding, executor_dimension_binding,
                load_dimension_policy,
            },
            state::{
                limit_policy::LimitPolicy,
                usage_window::{prepare_enforced_window_for_recording, ClosedPeriod},
            },
        },
        pathway::state::pathway_policy::PathwayPolicy,
        permissions::{
            auth::{assert_can_execute_settlement, assert_subject_holds_role},
        },
        settlement::{
            instructions::{assert_disjoint_settlement_account_index_groups, assert_distinct_settlement_account_indexes, assert_settlement_policy_for_terms, assert_pathway_evidence_policy, assert_canonical_issued_token_mint, assert_issued_token_extension_gates, assert_spl_token_account_binding, cpi_burn, cpi_transfer_asset_out_with_decimals, read_mint_decimals},
            state::settlement_intent::SettlementIntent,
        },
    },
};

const CHANCERY_CONFIG:                 usize = 0;
const EVENT_AUTHORITY:                 usize = 1;
const PAUSE_STATE:                     usize = 2;
const ASSET_CONFIG:                    usize = 3;
const PATHWAY_POLICY:                  usize = 4;
const INTENT:                          usize = 5;
const PRINCIPAL_PERMISSION_RECORD:     usize = 6;
const EXECUTOR_PERMISSION_RECORD:      usize = 7;
const SOURCE_ISSUED_TOKEN_ACCOUNT:     usize = 8;
const RESERVE_ASSET_TOKEN_ACCOUNT:     usize = 9;
const DESTINATION_ASSET_TOKEN_ACCOUNT: usize = 10;
const ASSET_MINT:                      usize = 11;
const ISSUED_TOKEN_MINT:               usize = 12;
const RESERVE_AUTHORITY_PDA:           usize = 13;
const ASSET_TOKEN_PROGRAM:             usize = 14;
const ISSUED_TOKEN_PROGRAM:            usize = 15;
const EXECUTOR:                        usize = 16;
const PRINCIPAL:                       usize = 17;
const ASSET_PAUSE_STATE:               usize = 18;
const ISSUED_TOKEN_CONTROL:            usize = 19;
const REQUIRED_ACCOUNT_COUNT:          usize = 20;

const FEE_POLICY:                      usize = 20;
const FEE_RECIPIENT_TOKEN_ACCOUNT:     usize = 21;
const LIMIT_POLICY:                    usize = 22;
const HOURLY_USAGE_WINDOW:             usize = 23;
const DAILY_USAGE_WINDOW:              usize = 24;
const WEEKLY_USAGE_WINDOW:             usize = 25;
const MONTHLY_USAGE_WINDOW:            usize = 26;
const EVIDENCE_POLICY:                 usize = 27;
const SETTLEMENT_POLICY:               usize = 28;
const ASSET_LIMIT_POLICY:              usize = 29;
const ASSET_DAILY_USAGE_WINDOW:        usize = 30;
const COUNTERPARTY_LIMIT_POLICY:       usize = 31;
const COUNTERPARTY_DAILY_USAGE_WINDOW: usize = 32;
const EXECUTOR_LIMIT_POLICY:           usize = 33;
const EXECUTOR_DAILY_USAGE_WINDOW:     usize = 34;

// Refund conveyance: the account whose key equals the record's stored
// rent_refund_recipient. Required whenever this instruction closes the
// record - the refund is conveyed, never redirected.
const RENT_REFUND_RECIPIENT:           usize = 35;

const DISTINCT_ACCOUNT_INDEXES:     &[usize] = &[
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    PAUSE_STATE,
    ASSET_CONFIG,
    PATHWAY_POLICY,
    INTENT,
    SOURCE_ISSUED_TOKEN_ACCOUNT,
    RESERVE_ASSET_TOKEN_ACCOUNT,
    DESTINATION_ASSET_TOKEN_ACCOUNT,
    ASSET_MINT,
    ISSUED_TOKEN_MINT,
    RESERVE_AUTHORITY_PDA,
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
    SETTLEMENT_POLICY,
    ASSET_LIMIT_POLICY,
    ASSET_DAILY_USAGE_WINDOW,
    COUNTERPARTY_LIMIT_POLICY,
    COUNTERPARTY_DAILY_USAGE_WINDOW,
    EXECUTOR_LIMIT_POLICY,
    EXECUTOR_DAILY_USAGE_WINDOW,
];

const PROTECTED_ACCOUNT_INDEXES: &[usize] = &[
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    PAUSE_STATE,
    ASSET_CONFIG,
    PATHWAY_POLICY,
    INTENT,
    SOURCE_ISSUED_TOKEN_ACCOUNT,
    RESERVE_ASSET_TOKEN_ACCOUNT,
    DESTINATION_ASSET_TOKEN_ACCOUNT,
    ASSET_MINT,
    ISSUED_TOKEN_MINT,
    RESERVE_AUTHORITY_PDA,
    ASSET_TOKEN_PROGRAM,
    ISSUED_TOKEN_PROGRAM,
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
    SETTLEMENT_POLICY,
    ASSET_LIMIT_POLICY,
    ASSET_DAILY_USAGE_WINDOW,
    COUNTERPARTY_LIMIT_POLICY,
    COUNTERPARTY_DAILY_USAGE_WINDOW,
    EXECUTOR_LIMIT_POLICY,
    EXECUTOR_DAILY_USAGE_WINDOW,
];
const IDENTITY_ACCOUNT_INDEXES: &[usize] = &[EXECUTOR, PRINCIPAL];
const PERMISSION_ACCOUNT_INDEXES: &[usize] = &[
    PRINCIPAL_PERMISSION_RECORD,
    EXECUTOR_PERMISSION_RECORD,
];

#[derive(BorshDeserialize)]
pub struct RedeemDelegatedArgs {
    pub intent_id : [u8; 32],
    pub pathway_id: [u8; 32],
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let program_id = crate::id();

    assert_distinct_settlement_account_indexes(accounts, DISTINCT_ACCOUNT_INDEXES)?;
    assert_disjoint_settlement_account_index_groups(
        accounts,
        PROTECTED_ACCOUNT_INDEXES,
        IDENTITY_ACCOUNT_INDEXES,
    )?;
    assert_disjoint_settlement_account_index_groups(
        accounts,
        PROTECTED_ACCOUNT_INDEXES,
        PERMISSION_ACCOUNT_INDEXES,
    )?;
    assert_disjoint_settlement_account_index_groups(
        accounts,
        IDENTITY_ACCOUNT_INDEXES,
        PERMISSION_ACCOUNT_INDEXES,
    )?;

    let chancery_config_account_info             = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info             = &accounts[EVENT_AUTHORITY];
    let pause_state_account_info                 = &accounts[PAUSE_STATE];
    let asset_config_account_info                = &accounts[ASSET_CONFIG];
    let pathway_policy_account_info              = &accounts[PATHWAY_POLICY];
    let intent_account_info                      = &accounts[INTENT];
    let principal_permission_record_account_info = &accounts[PRINCIPAL_PERMISSION_RECORD];
    let executor_permission_record_account_info  = &accounts[EXECUTOR_PERMISSION_RECORD];
    let source_issued_token_account_info         = &accounts[SOURCE_ISSUED_TOKEN_ACCOUNT];
    let reserve_asset_token_account_info         = &accounts[RESERVE_ASSET_TOKEN_ACCOUNT];
    let destination_asset_token_account_info     = &accounts[DESTINATION_ASSET_TOKEN_ACCOUNT];
    let asset_mint_account_info                  = &accounts[ASSET_MINT];
    let issued_token_mint_account_info           = &accounts[ISSUED_TOKEN_MINT];
    let reserve_authority_pda_account_info       = &accounts[RESERVE_AUTHORITY_PDA];
    let asset_token_program_account_info         = &accounts[ASSET_TOKEN_PROGRAM];
    let issued_token_program_account_info        = &accounts[ISSUED_TOKEN_PROGRAM];
    let executor_account_info                    = &accounts[EXECUTOR];
    let principal_account_info                   = &accounts[PRINCIPAL];
    let asset_pause_state_account_info           = &accounts[ASSET_PAUSE_STATE];
    let issued_token_control_account_info        = &accounts[ISSUED_TOKEN_CONTROL];

    assert_external_signer(executor_account_info, &program_id)?;
    assert_external_identity(principal_account_info.key, &program_id)?;
    if executor_account_info.key == principal_account_info.key {
        return Err(ChanceryError::AccountAliasNotAllowed.into());
    }

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !intent_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !source_issued_token_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !reserve_asset_token_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !destination_asset_token_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !issued_token_mint_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let args = RedeemDelegatedArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let clock           = Clock::get()?;
    let mut rolled_windows: Vec<ClosedPeriod> = Vec::new();

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    let pause_state_bump = PauseState::verify_pda(pause_state_account_info, &program_id)?;
    let pause_state = PauseState::load_for_verified_pda(pause_state_account_info, pause_state_bump)?;
    pause_state.assert_not_paused_for(pause_bit::REDEEM, clock.slot)?;

    // ── Per-asset pause. PDA verified so a foreign/empty account cannot be
    //    substituted; an uninitialized PDA means never paused. ─────────────────
    let asset_pause_state_bump = AssetPauseState::verify_pda(asset_pause_state_account_info, asset_mint_account_info.key, &program_id)?;

    if !asset_pause_state_account_info.data_is_empty() {
        let asset_pause_state = AssetPauseState::load_for_verified_pda(
            asset_pause_state_account_info,
            asset_mint_account_info.key,
            asset_pause_state_bump,
        )?;
        asset_pause_state.assert_not_paused_for(pause_bit::REDEEM, clock.slot)?;
    }

    let pathway = PathwayPolicy::load_verified(pathway_policy_account_info)?;

    pathway.assert_active()?;
    pathway.assert_kind(pathway_kind::DELEGATED)?;
    pathway.assert_executor(executor_account_info.key)?;

    assert_canonical_issued_token_mint(&chancery_config, &pathway, issued_token_mint_account_info)?;

    if &pathway.asset_mint != asset_mint_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if pathway.pathway_id != args.pathway_id {
        return Err(ChanceryError::InvalidPda.into());
    }

    // Pin the intent account to the canonical PDA so the executor cannot
    // substitute different committed settlement terms.
    // Evidence policy binds fail-closed: a referenced policy account is
    // mandatory (indexed here so the IDL derives it from source, issue RB-02).
    if pathway.has_evidence_policy() {
        if accounts.len() <= EVIDENCE_POLICY || accounts[EVIDENCE_POLICY].key == &Pubkey::default() {
            return Err(ChanceryError::MissingAccount.into());
        }

        assert_pathway_evidence_policy(&accounts[EVIDENCE_POLICY], &pathway)?;
    }

    let intent_bump =
        SettlementIntent::verify_pda(intent_account_info, &args.intent_id, &program_id)?;

    let intent = SettlementIntent::load_for_verified_pda(
        intent_account_info,
        &args.intent_id,
        intent_bump,
    )?;
    let intent_rent_refund_recipient = intent.rent_refund_recipient;

    intent.assert_pending()?;
    intent.assert_temporally_valid(clock.unix_timestamp)?;
    intent.assert_settlement_mode(settlement_mode::DELEGATED_NON_CUSTODIAL)?;
    intent.assert_redeem_terms()?;

    if intent.pathway_id != args.pathway_id {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if &intent.asset_mint != asset_mint_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if &intent.issued_token_mint != issued_token_mint_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if &intent.executor != executor_account_info.key {
        return Err(ChanceryError::ExecutorNotPermitted.into());
    }

    if &intent.principal_a != principal_account_info.key {
        return Err(ChanceryError::PermissionScopeMismatch.into());
    }

    let issued_token_amount        = intent.issued_token_amount;
    let minimum_asset_amount       = intent.minimum_asset_amount;
    // Intent authorization binds the output owner and mint, not a concrete
    // token-account address. The pathway executor is permissioned to select any
    // compatible principal-A-owned destination at execution.
    let asset_destination_owner    = intent.principal_a;
    let intent_id                  = intent.intent_id;
    let settlement_pol_id          = intent.policy_id;

    // Settlement policy binds fail-closed: a nonzero policy id makes the
    // policy account mandatory (indexed here so the IDL derives it from
    // source, issue RB-03).
    if settlement_pol_id != [0u8; 32] {
        if accounts.len() <= SETTLEMENT_POLICY || accounts[SETTLEMENT_POLICY].key == &Pubkey::default() {
            return Err(ChanceryError::MissingAccount.into());
        }

        assert_settlement_policy_for_terms(
            &accounts[SETTLEMENT_POLICY],
            &settlement_pol_id,
            settlement_mode::DELEGATED_NON_CUSTODIAL,
            asset_mint_account_info.key,
            principal_account_info.key,
            principal_account_info.key,
            executor_account_info.key,
            intent.issued_token_amount,
            clock.unix_timestamp,
        )?;
    }

    drop(intent);

    let _p_rec = assert_subject_holds_role(
        principal_account_info.key,
        principal_permission_record_account_info,
        role::CAN_REDEEM_DELEGATED,
        scope::PATHWAY,
        pathway_policy_account_info.key,
        clock.unix_timestamp,
        &program_id,
    )?;

    let _e_rec = assert_can_execute_settlement(
        executor_account_info,
        executor_permission_record_account_info,
        pathway_policy_account_info.key,
        &program_id,
    )?;

    let asset_config = AssetConfig::load_verified(asset_config_account_info)?;

    asset_config.assert_extensions_fresh(clock.slot)?;
    asset_config.assert_redeems_permitted()?;
    asset_config.assert_extensions_approved()?;
    pathway.assert_collateral_extensions_allowed(asset_config.observed_extension_mask)?;

    assert_issued_token_extension_gates(
        issued_token_control_account_info,
        issued_token_mint_account_info,
        &pathway,
        clock.slot,
        &program_id,
    )?;

    asset_config.assert_redeem_amount(issued_token_amount)?;

    if &asset_config.asset_mint != asset_mint_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if &asset_config.asset_token_program != asset_token_program_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    // Audit #7: pin reserve to the per-asset ATA so the reserve-authority
    // PDA signature can only release tokens from the legitimate reserve.
    // SPL Token's mint check then constrains dest.mint == asset_mint.
    let (reserve_authority, reserve_authority_bump) =
        chancery_config.resolved_reserve_authority(&program_id)?;

    {
        if reserve_asset_token_account_info.owner != asset_token_program_account_info.key {
            return Err(ChanceryError::TokenProgramMismatch.into());
        }

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

    if &chancery_config.issued_token_program != issued_token_program_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if pathway.has_limit_policy()
        && (accounts.len() <= LIMIT_POLICY
            || accounts[LIMIT_POLICY].key == &Pubkey::default())
    {
        return Err(ChanceryError::MissingAccount.into());
    }

    let has_limit = accounts.len() > LIMIT_POLICY
        && pathway.has_limit_policy()
        && accounts[LIMIT_POLICY].key != &Pubkey::default();

    // Shared pathway limits use collateral raw units in both directions.
    // Convert the issued-token burn amount before applying any pathway cap.
    let gross_asset_out = asset_config.compute_redeem_output(issued_token_amount)?;

    if gross_asset_out == 0 {
        return Err(ChanceryError::NetOutputZero.into());
    }

    // Derived from the loaded policy, never from which accounts the caller
    // passed, so an enforced window cannot be skipped by omission or read-only.
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

        limit_policy.assert_per_tx(gross_asset_out)?;

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

            limit_policy.assert_window_volume(hourly_usage_window.gross_out, gross_asset_out, ChanceryError::HourlyLimitBreached)?;
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

            limit_policy.assert_window_volume(daily_usage_window.gross_out, gross_asset_out, ChanceryError::DailyLimitBreached)?;
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
            limit_policy.assert_window_volume(weekly_usage_window.gross_out, gross_asset_out, ChanceryError::WeeklyLimitBreached)?;

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
            limit_policy.assert_window_volume(monthly_usage_window.gross_out, gross_asset_out, ChanceryError::MonthlyLimitBreached)?;

            monthly_prepared = Some(monthly_usage_window);
        }
    }

    // Pin the delegated burn source to principal A. Token-2022 then proves the
    // live authority relationship by requiring source.delegate == executor and
    // a sufficient delegated_amount before it permits the burn.
    assert_spl_token_account_binding(
        source_issued_token_account_info,
        issued_token_program_account_info,
        issued_token_mint_account_info.key,
        principal_account_info.key,
    )?;

    // Burn first, then compute + transfer.
    cpi_burn(
        issued_token_program_account_info,
        source_issued_token_account_info,
        issued_token_mint_account_info,
        executor_account_info,
        issued_token_amount,
    )?;

    // ── Dimension limits (spec §5.8) ──────────────────────────────────────────
    // Asset-denominated per-transaction + fixed UTC-day caps derived from
    // pathway state - the handler, not the caller, selects which dimensions
    // are enforced and which policy / window PDAs are expected. Template
    // dimensions (counterparty / executor) share one policy but accrue in
    // per-party daily windows.
    let (asset_scope_kind, asset_scope_key) = asset_dimension_binding(asset_mint_account_info.key);
    let asset_daily_prepared = if pathway.has_asset_redeem_limit_policy() {
        if accounts.len() <= ASSET_LIMIT_POLICY || accounts[ASSET_LIMIT_POLICY].key == &Pubkey::default() {
            return Err(ChanceryError::MissingAccount.into());
        }

        let policy = load_dimension_policy(
            &accounts[ASSET_LIMIT_POLICY],
            &pathway.asset_redeem_limit_policy_id,
            asset_scope_kind,
            &asset_scope_key,
            &program_id,
        )?;

        policy.assert_per_tx(gross_asset_out)?;

        if policy.per_day_maximum != 0 {
            Some(prepare_dimension_window_for_recording(
                accounts,
                ASSET_DAILY_USAGE_WINDOW,
                &policy,
                asset_scope_kind,
                &asset_scope_key,
                gross_asset_out,
                false,
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

    if !pathway.has_counterparty_limit_policy() {
        return Err(ChanceryError::PathwayDependencyMissing.into());
    }

    if accounts.len() <= COUNTERPARTY_LIMIT_POLICY
        || accounts[COUNTERPARTY_LIMIT_POLICY].key == &Pubkey::default()
    {
        return Err(ChanceryError::MissingAccount.into());
    }

    let (cp_scope_kind, cp_scope_key) = counterparty_dimension_binding();
    let counterparty_policy = load_dimension_policy(
        &accounts[COUNTERPARTY_LIMIT_POLICY],
        &pathway.counterparty_limit_policy_id,
        cp_scope_kind,
        &cp_scope_key,
        &program_id,
    )?;

    counterparty_policy.assert_required_daily_dimension_cap()?;
    counterparty_policy.assert_per_tx(gross_asset_out)?;

    let counterparty_daily_prepared = prepare_dimension_window_for_recording(
        accounts,
        COUNTERPARTY_DAILY_USAGE_WINDOW,
        &counterparty_policy,
        cp_scope_kind,
        principal_account_info.key,
        gross_asset_out,
        false,
        clock.unix_timestamp,
        &program_id,
        &mut rolled_windows,
    )?;

    let (exec_scope_kind, exec_scope_key) = executor_dimension_binding();
    let executor_daily_prepared = if pathway.has_executor_limit_policy() {
        if accounts.len() <= EXECUTOR_LIMIT_POLICY || accounts[EXECUTOR_LIMIT_POLICY].key == &Pubkey::default() {
            return Err(ChanceryError::MissingAccount.into());
        }

        let policy = load_dimension_policy(
            &accounts[EXECUTOR_LIMIT_POLICY],
            &pathway.executor_limit_policy_id,
            exec_scope_kind,
            &exec_scope_key,
            &program_id,
        )?;

        policy.assert_per_tx(gross_asset_out)?;

        if policy.per_day_maximum != 0 {
            Some(prepare_dimension_window_for_recording(
                accounts,
                EXECUTOR_DAILY_USAGE_WINDOW,
                &policy,
                exec_scope_kind,
                executor_account_info.key,
                gross_asset_out,
                false,
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


    if pathway.has_fee_policy()
        && (accounts.len() <= FEE_POLICY
            || accounts[FEE_POLICY].key == &Pubkey::default())
    {
        return Err(ChanceryError::MissingAccount.into());
    }

    let (
        fee_output_amount,
        rebate_amount,
        net_asset_out,
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

            let computation = fp.compute_asset_fee_with_input(issued_token_amount, gross_asset_out)?;

            let route_fee_to_recipient = computation.net_fee > 0 && fp.routes_fee_to_recipient();

            if route_fee_to_recipient {
                if accounts.len() <= FEE_RECIPIENT_TOKEN_ACCOUNT
                    || accounts[FEE_RECIPIENT_TOKEN_ACCOUNT].key == &Pubkey::default()
                {
                    return Err(ChanceryError::MissingAccount.into());
                }
                fp.assert_recipient_token_account(
                    &accounts[FEE_RECIPIENT_TOKEN_ACCOUNT],
                    asset_token_program_account_info,
                    asset_mint_account_info.key,
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
            (0u64, 0u64, gross_asset_out, [0u8; 32], false, Pubkey::default())
        };


    assert_spl_token_account_binding(
        destination_asset_token_account_info,
        asset_token_program_account_info,
        &pathway.asset_mint,
        &asset_destination_owner,
    )?;

    let asset_decimals = read_mint_decimals(
        asset_mint_account_info,
        asset_token_program_account_info,
    )?;

    let actual_net_asset_out = cpi_transfer_asset_out_with_decimals(
        asset_token_program_account_info,
        reserve_asset_token_account_info,
        asset_mint_account_info,
        destination_asset_token_account_info,
        reserve_authority_pda_account_info,
        reserve_authority_bump,
        net_asset_out,
        asset_decimals,
    )?;

    // Enforce the exact asset floor committed by the verified intent against
    // the recipient's measured post-transfer output.
    crate::modules::settlement::assert_output_meets_minimum(
        actual_net_asset_out,
        minimum_asset_amount,
        ChanceryError::IntentAmountBelowMinimum,
    )?;

    // net_asset_out already credits the rebate back to the principal, so the
    // recipient's share is gross − net (= fee − rebate). Routing the pre-rebate
    // fee_output_amount would release the rebate from the reserve a second time.
    let net_fee_output_amount = gross_asset_out
        .checked_sub(net_asset_out)
        .ok_or(ChanceryError::ArithmeticUnderflow)?;

    let mut actual_fee_recipient_amount = 0u64;
    if route_fee_to_recipient && net_fee_output_amount > 0 {
        actual_fee_recipient_amount = cpi_transfer_asset_out_with_decimals(
            asset_token_program_account_info,
            reserve_asset_token_account_info,
            asset_mint_account_info,
            &accounts[FEE_RECIPIENT_TOKEN_ACCOUNT],
            reserve_authority_pda_account_info,
            reserve_authority_bump,
            net_fee_output_amount,
            asset_decimals,
        )?;
    }

    // Terminal: close the executed intent, refunding rent to the recipient
    // stored at creation (which must appear as a writable account in this
    // transaction). The settlement event is the canonical record. This live
    // lifecycle cannot execute again after close. A later lifecycle at the same
    // intent_id is optional, requires a fresh available account, and carries no
    // prior authorization or replay state (spec 14 §14.3.1).
    if accounts.len() <= RENT_REFUND_RECIPIENT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let rent_refund_recipient_account_info = &accounts[RENT_REFUND_RECIPIENT];

    close_pda_to_recipient_account(intent_account_info, rent_refund_recipient_account_info, &intent_rent_refund_recipient)?;

    // Enforced windows were required present + writable and PDA-verified above,
    // so record unconditionally - no `is_writable` escape hatch. Shared
    // pathway windows use the same collateral denomination as mint inflows.
    if let Some(prepared_window) = hourly_prepared {
        prepared_window.record_outflow(&accounts[HOURLY_USAGE_WINDOW], gross_asset_out)?;
    }

    if let Some(prepared_window) = daily_prepared {
        prepared_window.record_outflow(&accounts[DAILY_USAGE_WINDOW], gross_asset_out)?;
    }

    if let Some(prepared_window) = weekly_prepared {
        prepared_window.record_outflow(&accounts[WEEKLY_USAGE_WINDOW], gross_asset_out)?;
    }

    if let Some(prepared_window) = monthly_prepared {
        prepared_window.record_outflow(&accounts[MONTHLY_USAGE_WINDOW], gross_asset_out)?;
    }

    if let Some(prepared_window) = asset_daily_prepared {
        prepared_window.record_outflow(&accounts[ASSET_DAILY_USAGE_WINDOW], gross_asset_out)?;
    }

    counterparty_daily_prepared.record_outflow(
        &accounts[COUNTERPARTY_DAILY_USAGE_WINDOW],
        gross_asset_out,
    )?;

    if let Some(prepared_window) = executor_daily_prepared {
        prepared_window.record_outflow(&accounts[EXECUTOR_DAILY_USAGE_WINDOW], gross_asset_out)?;
    }

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

    emit_settlement_redeem(event_authority_account_info, event_authority_bump, SettlementRedeem {
        sequence_nonce:         seq,
        chancery:               *chancery_config_account_info.key,
        slot:                   clock.slot,
        unix_timestamp:         clock.unix_timestamp,
        pathway_id:             args.pathway_id,
        settlement_mode:        settlement_mode::DELEGATED_NON_CUSTODIAL,
        intent_id:              intent_id,
        settlement_policy_id:   settlement_pol_id,
        limit_policy_id:        pathway.limit_policy_id,
        evidence_policy_id:     pathway.evidence_policy_id,
        fee_policy_id:          fee_policy_id,
        insurance_policy_id:    pathway.insurance_policy_id,
        principal_a:            *principal_account_info.key,
        principal_b:            *principal_account_info.key,
        executor:               *executor_account_info.key,
        asset_mint:             *asset_mint_account_info.key,
        issued_token_mint:      *issued_token_mint_account_info.key,
        asset_token_program:    *asset_token_program_account_info.key,
        issued_token_program:   *issued_token_program_account_info.key,
        source_account:         *source_issued_token_account_info.key,
        destination_account:    *destination_asset_token_account_info.key,
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
        gross_amount_in:        issued_token_amount,
        gross_amount_out:       gross_asset_out,
        fee_output_amount,
        rebate_amount,
        net_output_amount:      actual_net_asset_out,
        status_flags:           status_flag::INITIALIZED,
        reason_code:            0,
        actual_fee_recipient_amount,
    })?;

    Ok(())
}
