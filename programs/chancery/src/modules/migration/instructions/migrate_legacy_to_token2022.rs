/// migrate_legacy_to_token2022
///
/// One-way migration: burn legacy SPL tokens from the holder's source account,
/// then mint the value-equivalent amount of the new Token-2022 issued token to
/// the holder's destination account.
///
/// Spec §3.10 rules:
///   - Migration must be enabled.
///   - The legacy_mint on the config must match the provided mint.
///   - No reserve movement - this is purely a token swap.
///   - No reverse path.
///   - 1:1 in *value*. Legacy and issued may have different decimals: the burn
///     amount is converted via `convert_migration_amount`. When the issued token
///     has fewer decimals, the full amount is burned and the mint is floored;
///     the sub-precision remainder is forfeited so no legacy dust is left behind.
///   - The mint leg is subject to the same IssuedTokenControl deployment gates
///     as the settlement mint paths: `READY_FOR_SETTLEMENT` must be set (by
///     `verify_issued_token_deployment`) and the extension observation must be
///     fresh. Closes audit finding P5-F1 (migration could previously mint onto
///     an unverified Token-2022 deployment).
///
/// Accounts:
///   0  chancery_config                        writable  PDA (sequence_nonce for evidence)
///   1  event_authority                        readable  PDA [b"event-authority"]
///   2  pause_state                            readable  PDA [b"pause-state"]
///   3  legacy_migration_config                writable  PDA [b"legacy-migration"] (migrated_total)
///   4  source_legacy_token_account            writable  holder's legacy SPL token account
///   5  destination_issued_token_account       writable  holder's Token-2022 account
///   6  legacy_mint                            writable  SPL mint to burn from
///   7  issued_token_mint                      writable  Token-2022 mint to mint into
///   8  mint_authority_pda                     readable  PDA [b"mint-authority"] signs mint_to
///   9  legacy_token_program                   readable  spl-token program
///  10  issued_token_program                   readable  token-2022 program
///  11  holder                                 signer    burns their own legacy tokens
///  12  permission_record                      readable  PDA [b"permission", holder, MIGRATION, legacy_migration_config]
///  13  issued_token_control                   readable  PDA [b"issued-token-control"] deployment gates

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_sysvar::Sysvar;

use crate::{
    account_security::{assert_distinct_present_accounts, assert_external_signer},
    constants::{role, scope, token_program},
    error::ChanceryError,
    modules::{
        control::state::pause_state::{pause_bit, PauseState},
        core::state::chancery_config::ChanceryConfig,
        evidence::emit::{emit_legacy_migration, LegacyMigration},
        migration::state::legacy_migration_config::{
            convert_migration_amount,
            LegacyMigrationConfig,
        },
        permissions::auth::assert_signer_holds_role,
        issuance::tlv::tlv_parser::{
            read_spl_base_mint_decimals, read_spl_base_mint_supply,
        },
        settlement::instructions::{
            assert_issued_token_deployment_ready, assert_spl_token_account_binding,
            cpi_burn, cpi_mint_to,
        },
    },
};

const CHANCERY_CONFIG:                  usize = 0;
const EVENT_AUTHORITY:                  usize = 1;
const PAUSE_STATE:                      usize = 2;
const MIGRATION_CONFIG:                 usize = 3;
const SOURCE_LEGACY_TOKEN_ACCOUNT:      usize = 4;
const DESTINATION_ISSUED_TOKEN_ACCOUNT: usize = 5;
const LEGACY_MINT:                      usize = 6;
const ISSUED_TOKEN_MINT:                usize = 7;
const MINT_AUTHORITY_PDA:               usize = 8;
const LEGACY_TOKEN_PROGRAM:             usize = 9;
const ISSUED_TOKEN_PROGRAM:             usize = 10;
const HOLDER:                           usize = 11;
const PERMISSION_RECORD:                usize = 12;
const ISSUED_TOKEN_CONTROL:             usize = 13;
const REQUIRED_ACCOUNT_COUNT:           usize = 14;

#[derive(BorshDeserialize)]
pub struct MigrateLegacyArgs {
    pub amount: u64,
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info          = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info          = &accounts[EVENT_AUTHORITY];
    let pause_state_account_info              = &accounts[PAUSE_STATE];
    let migration_config_account_info         = &accounts[MIGRATION_CONFIG];
    let source_legacy_token_account_info      = &accounts[SOURCE_LEGACY_TOKEN_ACCOUNT];
    let destination_issued_token_account_info = &accounts[DESTINATION_ISSUED_TOKEN_ACCOUNT];
    let legacy_mint_account_info              = &accounts[LEGACY_MINT];
    let issued_token_mint_account_info        = &accounts[ISSUED_TOKEN_MINT];
    let mint_authority_pda_account_info       = &accounts[MINT_AUTHORITY_PDA];
    let legacy_token_program_account_info     = &accounts[LEGACY_TOKEN_PROGRAM];
    let issued_token_program_account_info     = &accounts[ISSUED_TOKEN_PROGRAM];
    let holder_account_info                   = &accounts[HOLDER];
    let permission_record_account_info        = &accounts[PERMISSION_RECORD];
    let issued_token_control_account_info     = &accounts[ISSUED_TOKEN_CONTROL];

    // ── Guards ────────────────────────────────────────────────────────────────
    assert_external_signer(holder_account_info, &crate::id())?;

    assert_distinct_present_accounts(&[
        chancery_config_account_info,
        event_authority_account_info,
        pause_state_account_info,
        migration_config_account_info,
        source_legacy_token_account_info,
        destination_issued_token_account_info,
        legacy_mint_account_info,
        issued_token_mint_account_info,
        mint_authority_pda_account_info,
        legacy_token_program_account_info,
        issued_token_program_account_info,
        holder_account_info,
        permission_record_account_info,
        issued_token_control_account_info,
    ])?;

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !migration_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !source_legacy_token_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !destination_issued_token_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !legacy_mint_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !issued_token_mint_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let args = MigrateLegacyArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    if args.amount == 0 {
        return Err(ChanceryError::AmountBelowMinimum.into());
    }

    // ── Load + validate state ─────────────────────────────────────────────────
    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    // PauseState is the single source of truth for migration pause.

    let clock = Clock::get()?;

    let pause_state_bump = PauseState::verify_pda(pause_state_account_info, &crate::id())?;
    let pause_state = PauseState::load_for_verified_pda(pause_state_account_info, pause_state_bump)?;
    pause_state.assert_not_paused_for(pause_bit::MIGRATION, clock.slot)?;

    let migration_config_bump =
        LegacyMigrationConfig::verify_pda(migration_config_account_info, &crate::id())?;
    let migration_cfg = LegacyMigrationConfig::load_for_verified_pda(
        migration_config_account_info,
        migration_config_bump,
    )?;

    migration_cfg.assert_enabled()?;
    migration_cfg.assert_legacy_mint(legacy_mint_account_info.key)?;

    if migration_cfg.legacy_program != token_program::SPL_TOKEN
        || legacy_token_program_account_info.key != &migration_cfg.legacy_program
        || legacy_mint_account_info.owner != legacy_token_program_account_info.key
    {
        return Err(ChanceryError::TokenProgramMismatch.into());
    }

    let program_id = crate::id();
    let permission_record = assert_signer_holds_role(
        holder_account_info,
        permission_record_account_info,
        role::CAN_EXECUTE_LEGACY_MIGRATION,
        scope::MIGRATION,
        migration_config_account_info.key,
        &program_id,
    )?;
    drop(permission_record);

    // Cross-check that the issued_token_mint matches chancery_config.
    if issued_token_mint_account_info.key != &chancery_config.issued_token_mint {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if legacy_token_program_account_info.key != &chancery_config.legacy_token_program {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if issued_token_program_account_info.key != &chancery_config.issued_token_program {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    assert_spl_token_account_binding(
        source_legacy_token_account_info,
        legacy_token_program_account_info,
        legacy_mint_account_info.key,
        holder_account_info.key,
    )?;
    assert_spl_token_account_binding(
        destination_issued_token_account_info,
        issued_token_program_account_info,
        issued_token_mint_account_info.key,
        holder_account_info.key,
    )?;

    // ── P5-F1 gate ─────────────────────────────────────────────────────────────
    // Migration mints issued supply, so it must satisfy the same
    // IssuedTokenControl deployment-verification + extension-freshness gates
    // as the settlement mint paths (READY_FOR_SETTLEMENT is set only by
    // `verify_issued_token_deployment`). Fails closed before any CPI.
    assert_issued_token_deployment_ready(
        issued_token_control_account_info,
        issued_token_mint_account_info,
        clock.slot,
        &program_id,
    )?;

    let legacy_mint_data = legacy_mint_account_info.try_borrow_data()?;
    let legacy_decimals = read_spl_base_mint_decimals(&legacy_mint_data)?;
    let current_legacy_supply = read_spl_base_mint_supply(&legacy_mint_data)?;
    drop(legacy_mint_data);
    let issued_decimals = read_spl_base_mint_decimals(&issued_token_mint_account_info.try_borrow_data()?)?;

    // Value-preserving conversion across (possibly different) decimals. When the
    // issued token has fewer decimals than the legacy token, the full amount is
    // burned and the mint is floored; the sub-precision remainder is forfeited so
    // no legacy dust is left behind. See `convert_migration_amount`.
    let (burn_amount, mint_amount) =
        convert_migration_amount(args.amount, legacy_decimals, issued_decimals)?;

    // Enforce the activation-snapshot conservation envelope before any CPI.
    // Normal migration preserves current supply + cumulative migrated units;
    // post-snapshot issuance that exceeds the snapshot makes this fail closed.
    // No mint-authority state is required or inferred.
    migration_cfg.assert_migration_supply_available(current_legacy_supply, burn_amount)?;

    // ── CPI 1: burn legacy tokens ─────────────────────────────────────────────
    // Holder signs - this is their own account, no PDA needed.
    cpi_burn(
        legacy_token_program_account_info,
        source_legacy_token_account_info,
        legacy_mint_account_info,
        holder_account_info,
        burn_amount,
    )?;

    // ── CPI 2: mint the value-equivalent issued tokens ────────────────────────
    // 1:1 in *value*; raw amounts differ when decimals differ (spec 04 §4.10).
    let bump = chancery_config.resolved_mint_authority_bump(&crate::id());

    cpi_mint_to(
        issued_token_program_account_info,
        issued_token_mint_account_info,
        destination_issued_token_account_info,
        mint_authority_pda_account_info,
        bump,
        mint_amount,
    )?;

    // ── Update migration total (tracks issued units minted) ───────────────────
    // Load mut separately - borrow rules prevent holding immutable ref above.
    drop(migration_cfg);

    LegacyMigrationConfig::load_mut_for_verified_pda(
        migration_config_account_info,
        migration_config_bump,
    )?
        .accumulate_migrated(burn_amount, mint_amount)?;

    // ── Evidence ──────────────────────────────────────────────────────────────
    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let seq                     = chancery_config_mut.next_sequence_nonce()?;

    emit_legacy_migration(
        event_authority_account_info,
        event_authority_bump,
        LegacyMigration {
            sequence_nonce:       seq,
            chancery:             *chancery_config_account_info.key,
            slot:                 clock.slot,
            unix_timestamp:       clock.unix_timestamp,
            legacy_mint:          *legacy_mint_account_info.key,
            issued_token_mint:    *issued_token_mint_account_info.key,
            burned_legacy_amount: burn_amount,
            minted_issued_amount: mint_amount,
            migrated_by:          *holder_account_info.key,
        },
    )?;

    Ok(())
}
