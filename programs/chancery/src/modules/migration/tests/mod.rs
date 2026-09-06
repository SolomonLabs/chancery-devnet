/// modules/migration/tests/mod.rs
///
/// Unit tests for LegacyMigrationConfig state and guard methods.
///
/// Covers:
///   - size / layout
///   - load guards (uninit, wrong size, non-writable, double-init)
///   - assert_enabled: ENABLED flag set/absent
///   - assert_legacy_mint: matching and mismatched mint
///   - accumulate_migrated: basic addition, checked overflow
///   - migration_flag::ENABLED is a single bit
///   - immutable supply-snapshot enforcement across both migration denominations
///   - PDA determinism (singleton)

use std::{cell::RefCell, rc::Rc};

use bytemuck::Zeroable;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::seeds,
    error::ChanceryError,
    modules::migration::state::legacy_migration_config::{
        convert_migration_amount, migration_flag, LegacyMigrationConfig,
        LEGACY_MIGRATION_CONFIG_DISCRIMINATOR, LEGACY_MIGRATION_CONFIG_SIZE,
    },
};

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn program_id() -> Pubkey { crate::id() }

#[allow(deprecated)]
fn make_account_info<'a>(
    key:      &'a Pubkey,
    data:     &'a mut [u8],
    lamports: &'a mut u64,
    owner:    &'a Pubkey,
    writable: bool,
) -> solana_account_info::AccountInfo<'a> {
    solana_account_info::AccountInfo {
        key,
        lamports:    Rc::new(RefCell::new(lamports)),
        data:        Rc::new(RefCell::new(data)),
        owner,
        _unused:     0,
        is_signer:   false,
        is_writable: writable,
        executable:  false,
    }
}

fn make_migration_config(
    legacy_mint:     Pubkey,
    migration_flags: u64,
    migrated_total:  u64,
) -> (Pubkey, Vec<u8>, u64) {
    let pid          = program_id();
    let (key, bump)  = Pubkey::find_program_address(&[seeds::LEGACY_MIGRATION], &pid);
    let mut data     = vec![0u8; LEGACY_MIGRATION_CONFIG_SIZE];
    let lamports     = 1u64;

    {
        // direct mut access to data
        let c: &mut LegacyMigrationConfig = bytemuck::from_bytes_mut(&mut data[..]);

        c.discriminator              = LEGACY_MIGRATION_CONFIG_DISCRIMINATOR;
        c.version                    = 1;
        c.bump                       = bump;
        c.migration_flags            = migration_flags;
        c.legacy_program             = Pubkey::new_unique();
        c.legacy_mint                = legacy_mint;
        c.migration_enabled_at_slot  = 100;
        c.migrated_total             = migrated_total;
        c.legacy_supply_snapshot     = u64::MAX;
        c.migrated_legacy_total      = 0;
    }
    (key, data, lamports)
}

// ─── migration_flag constants ─────────────────────────────────────────────────

#[test]
fn migration_flag_enabled_is_bit_zero_and_single_bit() {
    assert_eq!(migration_flag::ENABLED, 1u64 << 0);
    assert_eq!(migration_flag::ENABLED.count_ones(), 1);
}

// ─── Size / layout ────────────────────────────────────────────────────────────

#[test]
fn size_matches_declared() {
    assert_eq!(
        core::mem::size_of::<LegacyMigrationConfig>(),
        LEGACY_MIGRATION_CONFIG_SIZE,
    );
}

#[test]
fn zeroed_struct_all_zero_bytes() {
    let c            = LegacyMigrationConfig::zeroed();
    let bytes: &[u8] = bytemuck::bytes_of(&c);

    assert!(bytes.iter().all(|&b| b == 0));
}

// ─── Load guards ──────────────────────────────────────────────────────────────

#[test]
fn load_zero_data_returns_not_initialized() {
    let pid          = program_id();
    let (key, _)     = LegacyMigrationConfig::pda(&pid);
    let mut data     = vec![0u8; LEGACY_MIGRATION_CONFIG_SIZE];
    let mut lamports = 1u64;

    let ai = make_account_info(&key, &mut data, &mut lamports, &pid, false);

    assert!(matches!(
        LegacyMigrationConfig::load(&ai),
        Err(e) if e == ProgramError::Custom(ChanceryError::NotInitialized as u32),
    ));
}

#[test]
fn load_wrong_size_returns_length_mismatch() {
    let pid          = program_id();
    let key          = Pubkey::new_unique();
    let mut data     = vec![0u8; LEGACY_MIGRATION_CONFIG_SIZE - 4];
    let mut lamports = 1u64;
    let ai           = make_account_info(&key, &mut data, &mut lamports, &pid, false);

    assert!(matches!(
        LegacyMigrationConfig::load(&ai),
        Err(e) if e == ProgramError::Custom(ChanceryError::AccountDataLengthMismatch as u32),
    ));
}

#[test]
fn load_mut_non_writable_returns_error() {
    let mint = Pubkey::new_unique();
    let (key, mut data, mut lamports) = make_migration_config(mint, migration_flag::ENABLED, 0);
    let pid  = program_id();
    let ai   = make_account_info(&key, &mut data, &mut lamports, &pid, false);

    assert!(matches!(
        LegacyMigrationConfig::load_mut(&ai),
        Err(e) if e == ProgramError::Custom(ChanceryError::AccountNotWritable as u32),
    ));
}

#[test]
fn load_uninitialized_blocks_double_init() {
    let pid          = program_id();
    let (key, _)     = LegacyMigrationConfig::pda(&pid);
    let mut data     = vec![0u8; LEGACY_MIGRATION_CONFIG_SIZE];
    let mut lamports = 1u64;

    {
        let ai = make_account_info(&key, &mut data, &mut lamports, &pid, true);
        let mut c  = LegacyMigrationConfig::load_uninitialized_mut(&ai).unwrap();

        c.discriminator = LEGACY_MIGRATION_CONFIG_DISCRIMINATOR;
    }
    {
        let ai = make_account_info(&key, &mut data, &mut lamports, &pid, true);

        assert!(matches!(
            LegacyMigrationConfig::load_uninitialized_mut(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::AlreadyInitialized as u32),
        ));
    }
}

#[test]
fn load_reads_fields_correctly() {
    let mint  = Pubkey::new_unique();
    let total = 12_345_678u64;
    let (key, mut data, mut lamports) = make_migration_config(
        mint, migration_flag::ENABLED, total,
    );
    let pid   = program_id();
    let ai    = make_account_info(&key, &mut data, &mut lamports, &pid, false);
    let c     = LegacyMigrationConfig::load(&ai).unwrap();

    assert_eq!(c.legacy_mint,      mint);
    assert_eq!(c.migrated_total,   total);
    assert_eq!(c.migration_flags & migration_flag::ENABLED, migration_flag::ENABLED);
}

// ─── assert_enabled ───────────────────────────────────────────────────────────

#[test]
fn assert_enabled_passes_when_enabled_flag_set() {
    let mut config = LegacyMigrationConfig::zeroed();
    config.migration_flags = migration_flag::ENABLED;

    assert!(config.assert_enabled().is_ok());
}

#[test]
fn assert_enabled_fails_when_flag_not_set() {
    let config = LegacyMigrationConfig::zeroed();

    assert!(matches!(
        config.assert_enabled(),
        Err(error) if error == ProgramError::Custom(
            ChanceryError::LegacyMigrationNotEnabled as u32
        ),
    ));
}

#[test]
fn enabled_flag_can_be_cleared() {
    let mut flags = migration_flag::ENABLED;
    flags &= !migration_flag::ENABLED;

    assert_eq!(flags & migration_flag::ENABLED, 0);
}

// ─── assert_legacy_mint ───────────────────────────────────────────────────────

#[test]
fn assert_legacy_mint_passes_when_mint_matches() {
    let mint  = Pubkey::new_unique();
    let mut c = LegacyMigrationConfig::zeroed();

    c.legacy_mint = mint;

    assert!(c.assert_legacy_mint(&mint).is_ok());
}

#[test]
fn assert_legacy_mint_fails_when_mint_mismatches() {
    let mint  = Pubkey::new_unique();
    let other = Pubkey::new_unique();
    let mut c = LegacyMigrationConfig::zeroed();

    c.legacy_mint = mint;

    assert!(matches!(
        c.assert_legacy_mint(&other),
        Err(e) if e == ProgramError::Custom(ChanceryError::LegacyMintMismatch as u32),
    ));
}

#[test]
fn assert_legacy_mint_fails_for_default_pubkey_when_set_to_real_mint() {
    let mint  = Pubkey::new_unique();
    let mut c = LegacyMigrationConfig::zeroed();

    c.legacy_mint = mint;

    assert!(c.assert_legacy_mint(&Pubkey::default()).is_err());
}

// ─── supply cap and accounting ────────────────────────────────────────────────

#[test]
fn supply_snapshot_is_an_inclusive_legacy_burn_cap() {
    let mut config = LegacyMigrationConfig::zeroed();
    config.legacy_supply_snapshot = 1_000;
    config.migrated_legacy_total = 900;
    config.migrated_total = 450;

    assert!(config.assert_migration_supply_available(100, 100).is_ok());
    assert!(matches!(
        config.assert_migration_supply_available(100, 101),
        Err(error) if error == ProgramError::Custom(
            ChanceryError::MigrationSupplyExceeded as u32
        ),
    ));

    config.accumulate_migrated(100, 50).unwrap();
    assert_eq!(config.migrated_legacy_total, 1_000);
    assert_eq!(config.migrated_total, 500);
}

#[test]
fn post_snapshot_issuance_breaks_the_conservation_envelope() {
    let mut config = LegacyMigrationConfig::zeroed();
    config.legacy_supply_snapshot = 1_000;
    config.migrated_legacy_total = 100;

    assert!(config.assert_migration_supply_available(900, 1).is_ok());
    assert!(matches!(
        config.assert_migration_supply_available(901, 1),
        Err(error) if error == ProgramError::Custom(
            ChanceryError::MigrationSupplyExceeded as u32
        ),
    ));
}

#[test]
fn accumulation_checks_both_additions_before_mutation() {
    let mut config = LegacyMigrationConfig::zeroed();
    config.legacy_supply_snapshot = u64::MAX;
    config.migrated_legacy_total = 10;
    config.migrated_total = u64::MAX;

    assert!(matches!(
        config.accumulate_migrated(1, 1),
        Err(error) if error == ProgramError::Custom(ChanceryError::ArithmeticOverflow as u32),
    ));
    assert_eq!(config.migrated_legacy_total, 10);
    assert_eq!(config.migrated_total, u64::MAX);
}

#[test]
fn zero_amount_accounting_is_a_noop() {
    let mut config = LegacyMigrationConfig::zeroed();
    config.legacy_supply_snapshot = 42;
    config.migrated_legacy_total = 7;
    config.migrated_total = 9;

    config.accumulate_migrated(0, 0).unwrap();
    assert_eq!(config.migrated_legacy_total, 7);
    assert_eq!(config.migrated_total, 9);
}

// ─── PDA (singleton) ──────────────────────────────────────────────────────────

#[test]
fn pda_is_deterministic() {
    let pid      = program_id();
    let (k1, b1) = LegacyMigrationConfig::pda(&pid);
    let (k2, b2) = LegacyMigrationConfig::pda(&pid);

    assert_eq!(k1, k2);
    assert_eq!(b1, b2);
}

#[test]
fn pda_key_and_bump_are_stable_across_calls() {
    // Migration config is a singleton - same PDA regardless of which
    // asset or pathway is being migrated.
    let pid           = program_id();
    let (key, bump)   = LegacyMigrationConfig::pda(&pid);

    assert_ne!(key, Pubkey::default());

    // Re-derive to confirm stability
    let (key2, bump2) = LegacyMigrationConfig::pda(&pid);

    assert_eq!(key,  key2);
    assert_eq!(bump, bump2);
}

// ─── Interaction: enabled + mint check + accumulate ───────────────────────────

#[test]
fn full_migration_check_sequence_passes() {
    let legacy_mint = Pubkey::new_unique();
    let mut c       = LegacyMigrationConfig::zeroed();

    c.discriminator   = LEGACY_MIGRATION_CONFIG_DISCRIMINATOR;
    c.migration_flags = migration_flag::ENABLED;
    c.legacy_mint     = legacy_mint;
    c.migrated_total          = 0;
    c.legacy_supply_snapshot  = 500_000;
    c.migrated_legacy_total   = 0;

    // Simulate what migrate_legacy_to_token2022 does.
    c.assert_enabled().unwrap();
    c.assert_legacy_mint(&legacy_mint).unwrap();
    c.assert_migration_supply_available(500_000, 500_000).unwrap();
    c.accumulate_migrated(500_000, 500_000).unwrap();

    assert_eq!(c.migrated_legacy_total, 500_000);
    assert_eq!(c.migrated_total, 500_000);
    assert_eq!(c.migration_flags & migration_flag::ENABLED, migration_flag::ENABLED);
}

// ─── convert_migration_amount: decimal-aware migration conversion ──────────────

#[test]
fn convert_equal_decimals_is_exact_1to1() {
    assert_eq!(convert_migration_amount(2_500, 6, 6).unwrap(), (2_500, 2_500));
    assert_eq!(convert_migration_amount(1, 9, 9).unwrap(), (1, 1));
}

#[test]
fn convert_reducing_decimals_clean_amount_has_no_dust() {
    // 9->6 decimals: factor 1000. 5_000 legacy raw -> mint 5, burn all 5_000.
    assert_eq!(convert_migration_amount(5_000, 9, 6).unwrap(), (5_000, 5));
    // 1.33 tokens at 9 dec = 1_330_000_000 -> 1_330_000 issued raw, no dust.
    assert_eq!(
        convert_migration_amount(1_330_000_000, 9, 6).unwrap(),
        (1_330_000_000, 1_330_000),
    );
}

#[test]
fn convert_reducing_decimals_burns_full_amount_floors_mint_option_c() {
    // 2_500 legacy raw, factor 1000: mint 2, burn the FULL 2_500 (500 remainder forfeited).
    assert_eq!(convert_migration_amount(2_500, 9, 6).unwrap(), (2_500, 2));
    // ...999 in positions 7-9 is forfeited: mint 2_500_000, burn the full 2_500_000_999.
    assert_eq!(
        convert_migration_amount(2_500_000_999, 9, 6).unwrap(),
        (2_500_000_999, 2_500_000),
    );
}

#[test]
fn convert_reducing_below_one_issued_unit_is_rejected() {
    // amount < factor (1000) -> minted 0 -> nothing migratable.
    assert_eq!(
        convert_migration_amount(500, 9, 6).unwrap_err(),
        ChanceryError::AmountBelowMinimum.into(),
    );
    assert_eq!(
        convert_migration_amount(999, 9, 6).unwrap_err(),
        ChanceryError::AmountBelowMinimum.into(),
    );
}

#[test]
fn convert_increasing_decimals_is_exact_multiply_no_dust() {
    // 6->9 decimals: factor 1000. 5 legacy raw -> mint 5_000, burn 5.
    assert_eq!(convert_migration_amount(5, 6, 9).unwrap(), (5, 5_000));
}

#[test]
fn convert_increasing_decimals_overflow_is_checked() {
    // u64::MAX * 10^18 overflows u64.
    assert_eq!(
        convert_migration_amount(u64::MAX, 0, 18).unwrap_err(),
        ChanceryError::ArithmeticOverflow.into(),
    );
}

#[test]
fn migration_disabled_blocks_before_mint_check() {
    let legacy_mint = Pubkey::new_unique();
    let mut c       = LegacyMigrationConfig::zeroed();

    c.migration_flags = 0;  // not enabled
    c.legacy_mint     = legacy_mint;

    assert!(c.assert_enabled().is_err(),
        "disabled migration must fail before mint check");
}
