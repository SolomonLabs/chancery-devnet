/// enable_legacy_migration
///
/// Creates and initialises the LegacyMigrationConfigPda, enabling
/// one-way SPL Token -> Token-2022 migration.
///
/// This instruction is one-shot: once the discriminator is set the
/// migration config exists permanently. The `ENABLED` flag can be
/// toggled via a future governance instruction but the PDA itself
/// cannot be re-initialised.
///
/// Accounts:
///   0  chancery_config           writable  PDA - sequence_nonce for evidence
///   1  event_authority           readable  PDA [b"event-authority"]
///   2  legacy_migration_config   writable  PDA [b"legacy-migration"]
///   3  payer                     signer    funds rent
///   4  governance_authority      signer    must be chancery_config.governance_authority
///   5  system_program
///   6  legacy_mint_account       readable  initialized SPL mint; supply is snapshotted
///
/// Authority: governance only - migration enablement is a governance-level
/// decision, not ops.
///
/// Documented risk-policy exception (finding 102163, reviewed and declined):
/// this instruction is labelled HighImpact in evidence but deliberately
/// executes directly under a governance signature, without consuming a
/// pending change. Migration enablement is part of the bootstrap ceremony
/// (deploy -> initialize_chancery -> initialize_issued_token_control ->
/// enable_legacy_migration); requiring a timelock here would insert an
/// observation window into a ceremony that precedes any external exposure.
/// The action is one-shot against a fresh, unallocated PDA whose mint and
/// program are pinned to the values recorded at initialize_chancery, and the
/// supply snapshot is bound at enablement in state and evidence. Deployments
/// that do not enable migration at ceremony leave this slot open to a later
/// direct governance enablement and should account for that in key policy.

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    account_security::{assert_distinct_present_accounts, assert_external_signer},
    constants::{seeds, token_program},
    error::ChanceryError,
    modules::{
        control::change_risk::ConfigChangeRiskClass,
        core::{
            instructions::create_pda_account,
            state::chancery_config::ChanceryConfig,
        },
        evidence::emit::{
            emit_legacy_migration_enabled,
            LegacyMigrationEnabled,
        },
        migration::state::legacy_migration_config::{
            LegacyMigrationConfig,
            LEGACY_MIGRATION_CONFIG_DISCRIMINATOR,
            LEGACY_MIGRATION_CONFIG_SIZE,
            migration_flag,
        },
        issuance::tlv::tlv_parser::read_spl_base_mint_supply,
    },
};

const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const MIGRATION_CONFIG:       usize = 2;
const PAYER:                  usize = 3;
const GOVERNANCE_AUTHORITY:   usize = 4;
const SYSTEM_PROGRAM:         usize = 5;
const LEGACY_MINT_ACCOUNT:    usize = 6;
const REQUIRED_ACCOUNT_COUNT: usize = 7;

#[derive(BorshDeserialize)]
pub struct EnableLegacyMigrationArgs {
    /// The legacy SPL program that owns `legacy_mint`.
    pub legacy_program: Pubkey,

    /// The legacy SPL token mint to be migrated.
    pub legacy_mint:    Pubkey,
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info      = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];
    let migration_config_account_info     = &accounts[MIGRATION_CONFIG];
    let payer_account_info                = &accounts[PAYER];
    let governance_authority_account_info = &accounts[GOVERNANCE_AUTHORITY];
    let system_program_account_info       = &accounts[SYSTEM_PROGRAM];
    let legacy_mint_account_info          = &accounts[LEGACY_MINT_ACCOUNT];

    assert_external_signer(payer_account_info, &crate::id())?;
    assert_external_signer(governance_authority_account_info, &crate::id())?;

    if !migration_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    assert_distinct_present_accounts(&[
        chancery_config_account_info,
        event_authority_account_info,
        migration_config_account_info,
        system_program_account_info,
        legacy_mint_account_info,
    ])?;

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    // Governance-only gate.
    if governance_authority_account_info.key != &chancery_config.governance_authority {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    // Verify chancery_config records a legacy_token_mint that matches args.
    // Both must reference the same legacy ecosystem - the chancery was
    // configured for this migration at initialize_chancery time.
    let args = EnableLegacyMigrationArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    if chancery_config.legacy_token_mint != args.legacy_mint {
        return Err(ChanceryError::LegacyMintMismatch.into());
    }

    if args.legacy_program != token_program::SPL_TOKEN
        || args.legacy_program != chancery_config.legacy_token_program
        || legacy_mint_account_info.key != &args.legacy_mint
        || legacy_mint_account_info.owner != &args.legacy_program
    {
        return Err(ChanceryError::TokenProgramMismatch.into());
    }

    // Snapshot the initialized legacy mint's current raw supply as an immutable
    // aggregate migration cap. This is a bounded-accounting control only; it
    // neither requires nor implies mint-authority revocation.
    let legacy_supply_snapshot =
        read_spl_base_mint_supply(&legacy_mint_account_info.try_borrow_data()?)?;

    // ── Derive + verify PDA ───────────────────────────────────────────────────
    let program_id = crate::id();
    let (expected_key, bump) =
        Pubkey::find_program_address(&[seeds::LEGACY_MIGRATION], &program_id);

    if migration_config_account_info.key != &expected_key {
        return Err(ChanceryError::InvalidPda.into());
    }

    // This is a one-shot state creation. An already allocated zeroed shell is
    // not fresh and must not be accepted as an initialization target.
    if !migration_config_account_info.data_is_empty() {
        return Err(ChanceryError::AlreadyInitialized.into());
    }

    create_pda_account(
        payer_account_info,
        migration_config_account_info,
        system_program_account_info,
        &program_id,
        &[seeds::LEGACY_MIGRATION, &[bump]],
        LEGACY_MIGRATION_CONFIG_SIZE,
    )?;

    // ── Write state ───────────────────────────────────────────────────────────
    let mut cfg   = LegacyMigrationConfig::load_uninitialized_mut(migration_config_account_info)?;
    let clock = Clock::get()?;

    cfg.discriminator              = LEGACY_MIGRATION_CONFIG_DISCRIMINATOR;
    cfg.version                    = 1;
    cfg.bump                       = bump;
    cfg._pad0                      = [0u8; 5];
    cfg.migration_flags            = migration_flag::ENABLED;
    cfg.legacy_program             = args.legacy_program;
    cfg.legacy_mint                = args.legacy_mint;
    cfg.migration_enabled_at_slot  = clock.slot;
    cfg.migrated_total             = 0;
    cfg.legacy_supply_snapshot     = legacy_supply_snapshot;
    cfg.migrated_legacy_total      = 0;
    cfg._reserved                  = [0u8; 32];
    drop(cfg);

    // ── Evidence ──────────────────────────────────────────────────────────────
    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let seq                     = chancery_config_mut.next_sequence_nonce()?;

    emit_legacy_migration_enabled(
        event_authority_account_info,
        event_authority_bump,
        LegacyMigrationEnabled {
            sequence_nonce:         seq,
            chancery:               *chancery_config_account_info.key,
            slot:                   clock.slot,
            unix_timestamp:         clock.unix_timestamp,
            risk_class:             ConfigChangeRiskClass::HighImpact.as_u8(),
            migration_config:       *migration_config_account_info.key,
            legacy_program:         args.legacy_program,
            legacy_mint:            args.legacy_mint,
            legacy_supply_snapshot,
            enabled_by:             *governance_authority_account_info.key,
        },
    )?;

    Ok(())
}
