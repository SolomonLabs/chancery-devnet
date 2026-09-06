/// withdraw_reserve
///
/// Transfers collateral from a program-controlled reserve token account
/// to an approved ReserveDestination.
///
/// Accounts:
///   0  chancery_config                  writable  PDA  (sequence_nonce for evidence)
///   1  event_authority                  readable  PDA [b"event-authority"]
///   2  pause_state                      readable  PDA [b"pause-state"]
///   3  asset_config                     readable  PDA [b"asset-config", asset_mint]
///   4  reserve_destination              readable  PDA [b"reserve-destination", asset_mint, destination]
///   5  reserve_asset_token_account      writable  source token account (program-owned PDA)
///   6  destination_asset_token_account  writable  the approved destination
///   7  asset_mint                       readable
///   8  reserve_authority_pda            readable  PDA [b"reserve-authority"] signs the transfer
///   9  asset_token_program              readable  SPL or Token-2022
///  10  authority                        signer    must hold CAN_WITHDRAW_RESERVE or be operations/governance
///  11  asset_pause_state                readable  PDA [b"asset-pause", asset_mint]
///  12  withdrawal_limit_policy          readable  PDA [b"limit-policy", destination.withdrawal_limit_policy_id]
///  13  withdrawal_daily_usage_window    writable  PDA for the current fixed UTC day
///  14  permission_record                optional  PDA [b"permission", authority, scope, scope_key]

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    constants::{programs, role, scope, window_kind},
    error::ChanceryError,
    modules::{
        control::state::{
            asset_pause_state::AssetPauseState,
            pause_state::{pause_bit, PauseState},
        },
        core::state::{asset_config::AssetConfig, chancery_config::ChanceryConfig},
        evidence::emit::{emit_usage_windows_rolled, emit_reserve_withdrawal, ReserveWithdrawal},
        limits::state::{
            limit_policy::LimitPolicy,
            usage_window::{prepare_enforced_window_for_recording, ClosedPeriod},
        },
        permissions::{
            auth::assert_signer_holds_role,
            state::permission_record::PermissionRecord,
        },
        reserve::state::reserve_destination::ReserveDestination,
        settlement::instructions::{cpi_transfer_asset_out},
    },
};

const CHANCERY_CONFIG:                 usize = 0;
const EVENT_AUTHORITY:                 usize = 1;
const PAUSE_STATE:                     usize = 2;
const ASSET_CONFIG:                    usize = 3;
const RESERVE_DESTINATION:             usize = 4;
const RESERVE_ASSET_TOKEN_ACCOUNT:     usize = 5;
const DESTINATION_ASSET_TOKEN_ACCOUNT: usize = 6;
const ASSET_MINT:                      usize = 7;
const RESERVE_AUTHORITY_PDA:           usize = 8;
const ASSET_TOKEN_PROGRAM:             usize = 9;
const AUTHORITY:                       usize = 10;
const ASSET_PAUSE_STATE:               usize = 11;
const WITHDRAWAL_LIMIT_POLICY:          usize = 12;
const WITHDRAWAL_DAILY_USAGE_WINDOW:    usize = 13;
const PERMISSION_RECORD:               usize = 14;
const REQUIRED_ACCOUNT_COUNT:          usize = 14;

#[derive(BorshDeserialize)]
pub struct WithdrawReserveArgs {
    pub amount:                     u64,
    /// Minimum net token credit the destination must receive after any
    /// Token-2022 transfer fee. Checked against the measured balance delta.
    pub minimum_destination_amount: u64,
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let program_id = crate::id();

    let chancery_config_account_info         = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info         = &accounts[EVENT_AUTHORITY];
    let pause_state_account_info             = &accounts[PAUSE_STATE];
    let asset_config_account_info            = &accounts[ASSET_CONFIG];
    let reserve_destination_account_info     = &accounts[RESERVE_DESTINATION];
    let reserve_asset_token_account_info     = &accounts[RESERVE_ASSET_TOKEN_ACCOUNT];
    let destination_asset_token_account_info = &accounts[DESTINATION_ASSET_TOKEN_ACCOUNT];
    let asset_mint_account_info              = &accounts[ASSET_MINT];
    let reserve_authority_account_info       = &accounts[RESERVE_AUTHORITY_PDA];
    let asset_token_program_account_info     = &accounts[ASSET_TOKEN_PROGRAM];
    let authority_account_info               = &accounts[AUTHORITY];
    let asset_pause_state_account_info       = &accounts[ASSET_PAUSE_STATE];

    if !authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !reserve_asset_token_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !destination_asset_token_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    let clock = Clock::get()?;
    let mut rolled_windows: Vec<ClosedPeriod> = Vec::new();

    let pause_state_bump = PauseState::verify_pda(pause_state_account_info, &program_id)?;
    let pause_state = PauseState::load_for_verified_pda(pause_state_account_info, pause_state_bump)?;
    pause_state.assert_not_paused_for(pause_bit::RESERVE, clock.slot)?;

    let args = WithdrawReserveArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    if args.amount == 0 || args.minimum_destination_amount == 0 {
        return Err(ChanceryError::AmountBelowMinimum.into());
    }

    // ── Authority check ───────────────────────────────────────────────────────
    let is_admin = authority_account_info.key == &chancery_config.governance_authority
        || authority_account_info.key == &chancery_config.operations_authority;

    if !is_admin {
        // Non-admin must hold a CAN_WITHDRAW_RESERVE permission scoped to THIS
        // ReserveDestination. `assert_signer_holds_role` verifies the permission
        // PDA derivation (binding subject + scope_kind + scope_key), the role, the
        // expiry, and the scope - so a grant issued for a different destination
        // cannot be replayed against this one (issue-67). The earlier hand-rolled
        // check validated subject/role/expiry but ignored scope entirely.
        // Resolve the exact canonical permission PDA from the optional tail.
        // Do not select by owner: the tail can contain other Chancery-owned
        // accounts, and optional account encoding may move the record relative
        // to the trailing event-program account.
        let (expected_permission_record, _) = PermissionRecord::pda(
            authority_account_info.key,
            scope::DESTINATION,
            reserve_destination_account_info.key,
            &program_id,
        );
        let permission_record_account_info = if accounts.len() > PERMISSION_RECORD {
            let primary = &accounts[PERMISSION_RECORD];
            if primary.key == &expected_permission_record {
                Some(primary)
            } else {
                accounts
                    .iter()
                    .skip(PERMISSION_RECORD + 1)
                    .find(|account| account.key == &expected_permission_record)
            }
        } else {
            None
        }
        .ok_or(ChanceryError::InsufficientRole)?;

        assert_signer_holds_role(
            authority_account_info,
            permission_record_account_info,
            role::CAN_WITHDRAW_RESERVE,
            scope::DESTINATION,
            reserve_destination_account_info.key,
            &program_id,
        )?;
    }

    // ── Load + validate asset config ──────────────────────────────────────────
    // `AssetConfig.mode` is a settlement lifecycle policy. Reserve movement is
    // a separate administrative surface governed by the global/asset RESERVE
    // pauses, an approved destination, an exact destination-scoped permission,
    // and mandatory per-transaction/daily limits. A FROZEN settlement asset is
    // therefore not automatically immobilized in reserve; incident policy must
    // apply the RESERVE pause or disable the destination when outflow must stop.
    let asset_config = AssetConfig::load_verified(asset_config_account_info)?;

    if &asset_config.asset_mint != asset_mint_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if &asset_config.asset_token_program != asset_token_program_account_info.key {
        return Err(ChanceryError::TokenProgramMismatch.into());
    }

    // Reserve outflow is collateral movement, so it consumes the same
    // asset-level extension-observation gates as settlement. The freshness
    // check honors the configured policy; an age ceiling of zero disables only
    // stale-age rejection, while the mask check still rejects every observed
    // extension that is unapproved or forbidden. No pathway-level
    // extension policy applies to this non-settlement transfer.
    asset_config.assert_extensions_fresh(clock.slot)?;
    asset_config.assert_extensions_approved()?;

    // ── Per-asset pause. PDA verified so a foreign/empty account cannot be
    //    substituted; an uninitialized PDA means never paused. ─────────────────
    let asset_pause_state_bump = AssetPauseState::verify_pda(asset_pause_state_account_info, asset_mint_account_info.key, &program_id)?;

    if !asset_pause_state_account_info.data_is_empty() {
        let asset_pause_state = AssetPauseState::load_for_verified_pda(
            asset_pause_state_account_info,
            asset_mint_account_info.key,
            asset_pause_state_bump,
        )?;
        asset_pause_state.assert_not_paused_for(pause_bit::RESERVE, clock.slot)?;
    }

    // ── Validate destination ──────────────────────────────────────────────────
    let dest = ReserveDestination::load_verified(reserve_destination_account_info)?;

    dest.assert_enabled()?;
    dest.assert_asset(asset_mint_account_info.key)?;
    dest.assert_destination_account(destination_asset_token_account_info.key)?;
    if destination_asset_token_account_info.owner != asset_token_program_account_info.key {
        return Err(ChanceryError::TokenProgramMismatch.into());
    }

    {
        let destination_data = destination_asset_token_account_info.try_borrow_data()?;
        if destination_data.len() < 64 {
            return Err(ChanceryError::AccountDataLengthMismatch.into());
        }
        if &destination_data[0..32] != asset_mint_account_info.key.as_ref() {
            return Err(ChanceryError::ReserveDestinationTokenMintMismatch.into());
        }
        if &destination_data[32..64] != dest.destination_owner.as_ref() {
            return Err(ChanceryError::ReserveDestinationOwnerMismatch.into());
        }
    }

    // ── Mandatory execution-side withdrawal containment ──────────────────────
    let limit_policy_bump = LimitPolicy::verify_pda(
        &accounts[WITHDRAWAL_LIMIT_POLICY],
        &dest.withdrawal_limit_policy_id,
        &program_id,
    )?;
    let withdrawal_limit_policy =
        LimitPolicy::load_for_verified_pda(
            &accounts[WITHDRAWAL_LIMIT_POLICY],
            &dest.withdrawal_limit_policy_id,
            limit_policy_bump,
        )?;
    withdrawal_limit_policy
        .assert_reserve_withdrawal_caps(reserve_destination_account_info.key)?;
    withdrawal_limit_policy.assert_per_tx(args.amount)?;

    let scope_hash = withdrawal_limit_policy.scope_hash();
    let withdrawal_daily_usage = prepare_enforced_window_for_recording(
        accounts,
        WITHDRAWAL_DAILY_USAGE_WINDOW,
        &scope_hash,
        window_kind::DAILY,
        clock.unix_timestamp,
        &program_id,
        &mut rolled_windows,
    )?;
    withdrawal_limit_policy.assert_window_volume(
        withdrawal_daily_usage.gross_out,
        args.amount,
        ChanceryError::DailyLimitBreached,
    )?;

    // All three accounts are reloaded mutably or no longer needed after CPI.
    // Release their RefCell guards before invoking the token program so the
    // usage window can be updated through the canonical mutable loader.
    drop(withdrawal_limit_policy);
    drop(dest);

    // ── Validate reserve authority PDA ───────────────────────────────────────
    let (expected_authority, reserve_authority_bump) =
        chancery_config.resolved_reserve_authority(&program_id)?;

    if reserve_authority_account_info.key != &expected_authority {
        return Err(ChanceryError::InvalidPda.into());
    }

    // Pin the reserve to the per-asset ATA so the dedicated reserve-authority
    // PDA signature can only release tokens from the legitimate reserve.
    // SPL Token's mint check then constrains dest.mint == asset_mint.
    {
        if reserve_asset_token_account_info.owner != asset_token_program_account_info.key {
            return Err(ChanceryError::TokenProgramMismatch.into());
        }

        let (expected_ata, _) = Pubkey::find_program_address(
            &[
                expected_authority.as_ref(),
                asset_token_program_account_info.key.as_ref(),
                asset_mint_account_info.key.as_ref(),
            ],
            &programs::ASSOCIATED_TOKEN_PROGRAM_ID,
        );

        if reserve_asset_token_account_info.key != &expected_ata {
            return Err(ChanceryError::InvalidPda.into());
        }
    }

    // ── CPI token transfer ────────────────────────────────────────────────────
    let actual_destination_amount = cpi_transfer_asset_out(
        asset_token_program_account_info,
        reserve_asset_token_account_info,
        asset_mint_account_info,
        destination_asset_token_account_info,
        reserve_authority_account_info,
        reserve_authority_bump,
        args.amount,
    )?;

    crate::modules::settlement::assert_output_meets_minimum(
        actual_destination_amount,
        args.minimum_destination_amount,
        ChanceryError::AmountBelowMinimum,
    )?;

    withdrawal_daily_usage.record_outflow(
        &accounts[WITHDRAWAL_DAILY_USAGE_WINDOW],
        args.amount,
    )?;

    // ── Emit evidence ─────────────────────────────────────────────────────────
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
    let seq                  = chancery_config_mut.next_sequence_nonce()?;

    emit_reserve_withdrawal(
        event_authority_account_info,
        event_authority_bump,
        ReserveWithdrawal {
            sequence_nonce:            seq,
            chancery:                  *chancery_config_account_info.key,
            slot:                      clock.slot,
            unix_timestamp:            clock.unix_timestamp,
            asset_mint:                *asset_mint_account_info.key,
            destination_token_account: *destination_asset_token_account_info.key,
            amount:                    args.amount,
            initiated_by:              *authority_account_info.key,
            actual_destination_amount,
        },
    )?;

    Ok(())
}
