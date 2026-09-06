/// modules/core/tests/mod.rs
///
/// Unit tests for core state structs and their helper methods.
/// All tests are pure (no CPI, no runtime). Run with:
///   cargo test --features no-entrypoint modules::core::tests

use std::{cell::RefCell, rc::Rc};

use bytemuck::Zeroable;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::{asset_mode, authority_role, seeds, status_flag, RATE_PRECISION_E9},
    error::ChanceryError,
    modules::core::state::{
        asset_config::{AssetConfig, ASSET_CONFIG_DISCRIMINATOR, ASSET_CONFIG_SIZE},
        authority_transfer::{
            AuthorityTransfer, AUTHORITY_TRANSFER_DISCRIMINATOR, AUTHORITY_TRANSFER_SIZE,
        },
        chancery_config::{ChanceryConfig, CHANCERY_CONFIG_DISCRIMINATOR, CHANCERY_CONFIG_SIZE},
    },
};

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn make_account(size: usize) -> (Pubkey, Vec<u8>) {
    let key  = Pubkey::new_unique();
    let data = vec![0u8; size];

    (key, data)
}

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

fn program_id() -> Pubkey {
    crate::id()
}

// ─── ChanceryConfig ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod chancery_config_tests {
    use super::*;

    #[test]
    fn size_matches_declared() {
        assert_eq!(
            core::mem::size_of::<ChanceryConfig>(),
            CHANCERY_CONFIG_SIZE,
            "ChanceryConfig: size_of does not match CHANCERY_CONFIG_SIZE",
        );
    }

    #[test]
    fn zeroed_load_returns_not_initialized() {
        let pid             = program_id();
        let (key, mut data) = make_account(CHANCERY_CONFIG_SIZE);
        let mut lamports    = 1u64;
        let ai              = make_account_info(&key, &mut data, &mut lamports, &pid, false);

        assert!(matches!(
            ChanceryConfig::load(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::NotInitialized as u32),
        ));
    }

    #[test]
    fn wrong_data_length_returns_error() {
        let pid             = program_id();
        let (key, mut data) = make_account(CHANCERY_CONFIG_SIZE - 1);
        let mut lamports    = 1u64;
        let ai              = make_account_info(&key, &mut data, &mut lamports, &pid, false);

        assert!(matches!(
            ChanceryConfig::load(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::AccountDataLengthMismatch as u32),
        ));
    }

    #[test]
    fn load_uninitialized_blocks_double_init() {
        let pid             = program_id();
        let (key, mut data) = make_account(CHANCERY_CONFIG_SIZE);
        let mut lamports    = 1u64;

        // First init: write discriminator
        {
            let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, true);
            let mut cfg = ChanceryConfig::load_uninitialized_mut(&ai).unwrap();

            cfg.discriminator = CHANCERY_CONFIG_DISCRIMINATOR;
            cfg.status_flags  = status_flag::INITIALIZED;
        }

        // Second init attempt: AlreadyInitialized
        {
            let ai = make_account_info(&key, &mut data, &mut lamports, &pid, true);

            assert!(matches!(
                ChanceryConfig::load_uninitialized_mut(&ai),
                Err(e) if e == ProgramError::Custom(ChanceryError::AlreadyInitialized as u32),
            ));
        }
    }

    #[test]
    fn load_mut_on_non_writable_returns_error() {
        let pid             = program_id();
        let (key, mut data) = make_account(CHANCERY_CONFIG_SIZE);
        let mut lamports    = 1u64;

        // Write discriminator so load_mut proceeds to writable check
        {
            let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, true);
            let mut cfg = ChanceryConfig::load_uninitialized_mut(&ai).unwrap();

            cfg.discriminator = CHANCERY_CONFIG_DISCRIMINATOR;
            cfg.status_flags  = status_flag::INITIALIZED;
        }

        let ai = make_account_info(&key, &mut data, &mut lamports, &pid, false);  // NOT writable

        assert!(matches!(
            ChanceryConfig::load_mut(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::AccountNotWritable as u32),
        ));
    }

    #[test]
    fn load_and_read_fields_correctly() {
        let pid             = program_id();
        let gov             = Pubkey::new_unique();
        let ops             = Pubkey::new_unique();
        let (key, mut data) = make_account(CHANCERY_CONFIG_SIZE);
        let mut lamports    = 1u64;

        {
            let ai                  = make_account_info(&key, &mut data, &mut lamports, &pid, true);
            let mut chancery_config_mut = ChanceryConfig::load_uninitialized_mut(&ai).unwrap();

            chancery_config_mut.discriminator         = CHANCERY_CONFIG_DISCRIMINATOR;
            chancery_config_mut.version               = 1;
            chancery_config_mut.status_flags          = status_flag::INITIALIZED;
            chancery_config_mut.governance_authority  = gov;
            chancery_config_mut.operations_authority  = ops;
            chancery_config_mut.domain_separator      = [0xFFu8; 32];
            chancery_config_mut.event_sequence_nonce  = 42;
        }

        let ai              = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let chancery_config = ChanceryConfig::load(&ai).unwrap();

        assert_eq!(chancery_config.version,              1);
        assert_eq!(chancery_config.governance_authority, gov);
        assert_eq!(chancery_config.operations_authority, ops);
        assert_eq!(chancery_config.domain_separator,     [0xFFu8; 32]);
        assert_eq!(chancery_config.event_sequence_nonce, 42);
    }

    #[test]
    fn next_sequence_nonce_increments_monotonically() {
        let mut cfg = ChanceryConfig::zeroed();

        cfg.event_sequence_nonce = 0;

        assert_eq!(cfg.next_sequence_nonce().unwrap(), 1);
        assert_eq!(cfg.next_sequence_nonce().unwrap(), 2);
        assert_eq!(cfg.next_sequence_nonce().unwrap(), 3);
        assert_eq!(cfg.event_sequence_nonce,           3);
    }

    #[test]
    fn next_sequence_nonce_at_maxmimum_returns_overflow_error() {
        let mut cfg = ChanceryConfig::zeroed();

        cfg.event_sequence_nonce = u64::MAX;

        assert!(matches!(
            cfg.next_sequence_nonce(),
            Err(e) if e == ProgramError::Custom(ChanceryError::ArithmeticOverflow as u32),
        ));
    }

    #[test]
    fn verified_load_rejects_wrong_stored_bump() {
        let pid = program_id();
        let (key, expected_bump) = ChanceryConfig::pda(&pid);
        let mut data = vec![0u8; CHANCERY_CONFIG_SIZE];
        let mut lamports = 1u64;

        {
            let config: &mut ChanceryConfig = bytemuck::from_bytes_mut(&mut data);
            config.discriminator = CHANCERY_CONFIG_DISCRIMINATOR;
            config.version = 1;
            config.bump = expected_bump.wrapping_add(1);
            config.status_flags = status_flag::INITIALIZED;
        }

        let account = make_account_info(&key, &mut data, &mut lamports, &pid, false);

        assert!(matches!(
            ChanceryConfig::load_verified(&account),
            Err(ProgramError::Custom(code))
                if code == ChanceryError::StoredBumpMismatch as u32,
        ));
    }

    #[test]
    fn pda_derivation_is_stable() {
        let pid       = program_id();
        let (key1, _) = ChanceryConfig::pda(&pid);
        let (key2, _) = ChanceryConfig::pda(&pid);

        assert_eq!(key1, key2, "PDA derivation must be deterministic");
    }
}

// ─── AssetConfig ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod asset_config_tests {
    use super::*;

    fn make_initialized_asset_config(
        asset_mint:     Pubkey,
        mode:           u8,
        deposit_e9:     u64,
        redeem_e9:      u64,
        minimum_dep:    u64,
        minimum_red:    u64,
        maximum_single: u64,
    ) -> (Pubkey, Vec<u8>, u64) {
        let pid             = program_id();
        let mut data        = vec![0u8; ASSET_CONFIG_SIZE];
        let lamports        = 1u64;
        let (pda_key, bump) = Pubkey::find_program_address(
            &[seeds::ASSET_CONFIG, asset_mint.as_ref()],
            &pid,
        );

        {
            let asset_config: &mut AssetConfig = bytemuck::from_bytes_mut(&mut data[..]);

            asset_config.discriminator                     = ASSET_CONFIG_DISCRIMINATOR;
            asset_config.version                           = 1;
            asset_config.bump                              = bump;
            asset_config.mode                              = mode;
            asset_config.decimals                          = 6;
            asset_config.asset_mint                        = asset_mint;
            asset_config.asset_token_program               = Pubkey::new_unique();
            asset_config.deposit_rate_e9                   = deposit_e9;
            asset_config.redeem_rate_e9                    = redeem_e9;
            asset_config.minimum_deposit_amount            = minimum_dep;
            asset_config.minimum_redeem_amount             = minimum_red;
            asset_config.maximum_single_settlement_amount  = maximum_single;
            asset_config.status_flags                      = status_flag::INITIALIZED;
            asset_config._reserved                         = [0u8; 32];
        }
        (pda_key, data, lamports)
    }

    #[test]
    fn size_matches_declared() {
        assert_eq!(core::mem::size_of::<AssetConfig>(), 280);
        assert_eq!(ASSET_CONFIG_SIZE, 280);
        assert_eq!(core::mem::offset_of!(AssetConfig, max_extension_observation_age_slots), 240);
        assert_eq!(core::mem::offset_of!(AssetConfig, _reserved), 248);
    }

    #[test]
    fn initialized_asset_reserved_tail_is_zero() {
        let mint = Pubkey::new_unique();
        let pid = program_id();
        let (key, mut data, mut lamports) = make_initialized_asset_config(
            mint, asset_mode::ACTIVE, RATE_PRECISION_E9, RATE_PRECISION_E9, 1, 1, 10,
        );
        let ai = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let cfg = AssetConfig::load(&ai).unwrap();
        assert!(cfg._reserved.iter().all(|&b| b == 0));
    }

    #[test]
    fn zeroed_load_returns_not_initialized() {
        let pid          = program_id();
        let mint         = Pubkey::new_unique();
        let (key, _)     = Pubkey::find_program_address(
            &[seeds::ASSET_CONFIG, mint.as_ref()], &pid,
        );
        let mut data     = vec![0u8; ASSET_CONFIG_SIZE];
        let mut lamports = 1u64;
        let ai           = make_account_info(&key, &mut data, &mut lamports, &pid, false);

        assert!(matches!(
            AssetConfig::load(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::NotInitialized as u32),
        ));
    }

    #[test]
    fn wrong_size_returns_length_mismatch() {
        let pid             = program_id();
        let (key, mut data) = make_account(ASSET_CONFIG_SIZE - 4);
        let mut lamports    = 1u64;
        let ai              = make_account_info(&key, &mut data, &mut lamports, &pid, false);

        assert!(matches!(
            AssetConfig::load(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::AccountDataLengthMismatch as u32),
        ));
    }

    // ── Mode guards ───────────────────────────────────────────────────────────

    #[test]
    fn active_mode_permissionits_deposits_and_redeems() {
        let mint = Pubkey::new_unique();
        let (key, mut data, mut lamports) = make_initialized_asset_config(
            mint, asset_mode::ACTIVE, 1_000_000_000, 1_000_000_000,
            0, 0, u64::MAX,
        );
        let pid = program_id();
        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let cfg = AssetConfig::load(&ai).unwrap();

        assert!(cfg.assert_deposits_permitted().is_ok());
        assert!(cfg.assert_redeems_permitted().is_ok());
        assert!(cfg.is_active());
    }

    #[test]
    fn wind_down_mode_blocks_deposits_but_allows_redeems() {
        let mint = Pubkey::new_unique();
        let (key, mut data, mut lamports) = make_initialized_asset_config(
            mint, asset_mode::WIND_DOWN, 1_000_000_000, 1_000_000_000,
            0, 0, u64::MAX,
        );
        let pid = program_id();
        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let cfg = AssetConfig::load(&ai).unwrap();

        assert!(matches!(
            cfg.assert_deposits_permitted(),
            Err(e) if e == ProgramError::Custom(ChanceryError::AssetModeForbids as u32),
        ));
        assert!(cfg.assert_redeems_permitted().is_ok());
        assert!(cfg.is_wind_down());
    }

    #[test]
    fn frozen_mode_blocks_both_deposits_and_redeems() {
        let mint = Pubkey::new_unique();
        let (key, mut data, mut lamports) = make_initialized_asset_config(
            mint, asset_mode::FROZEN, 1_000_000_000, 1_000_000_000,
            0, 0, u64::MAX,
        );
        let pid = program_id();
        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let cfg = AssetConfig::load(&ai).unwrap();

        assert!(cfg.assert_deposits_permitted().is_err());
        assert!(cfg.assert_redeems_permitted().is_err());
        assert!(cfg.is_frozen());
    }

    // ── Amount guards ─────────────────────────────────────────────────────────

    #[test]
    fn deposit_amount_below_minimum_rejected() {
        let mint = Pubkey::new_unique();
        let (key, mut data, mut lamports) = make_initialized_asset_config(
            mint, asset_mode::ACTIVE, 1_000_000_000, 1_000_000_000,
            1_000, 1_000, 1_000_000_000,
        );
        let pid = program_id();
        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let cfg = AssetConfig::load(&ai).unwrap();

        assert!(matches!(
            cfg.assert_deposit_amount(999),
            Err(e) if e == ProgramError::Custom(ChanceryError::AmountBelowMinimum as u32),
        ));
    }

    #[test]
    fn deposit_amount_at_minimum_accepted() {
        let mint = Pubkey::new_unique();
        let (key, mut data, mut lamports) = make_initialized_asset_config(
            mint, asset_mode::ACTIVE, 1_000_000_000, 1_000_000_000,
            1_000, 1_000, 1_000_000_000,
        );
        let pid = program_id();
        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let cfg = AssetConfig::load(&ai).unwrap();

        assert!(cfg.assert_deposit_amount(1_000).is_ok());
    }

    #[test]
    fn deposit_amount_above_maxmimum_rejected() {
        let mint = Pubkey::new_unique();
        let (key, mut data, mut lamports) = make_initialized_asset_config(
            mint, asset_mode::ACTIVE, 1_000_000_000, 1_000_000_000,
            0, 0, 500_000,
        );
        let pid = program_id();
        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let cfg = AssetConfig::load(&ai).unwrap();

        assert!(matches!(
            cfg.assert_deposit_amount(500_001),
            Err(e) if e == ProgramError::Custom(ChanceryError::AmountExceedsMaximum as u32),
        ));
    }

    // ── Extension mask ────────────────────────────────────────────────────────

    #[test]
    fn unapproved_extension_fails() {
        let mint = Pubkey::new_unique();
        let (key, mut data, mut lamports) = make_initialized_asset_config(
            mint, asset_mode::ACTIVE, 1_000_000_000, 1_000_000_000,
            0, 0, u64::MAX,
        );

        // Set observed bit that is not in approved mask
        {
            // direct mut access to data
            let cfg: &mut AssetConfig = bytemuck::from_bytes_mut(&mut data[..]);

            cfg.approved_extension_mask = [0u64; 2];
            cfg.observed_extension_mask = [1u64, 0u64];  // bit 0 observed, not approved
        }

        let pid = program_id();
        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let cfg = AssetConfig::load(&ai).unwrap();

        assert!(matches!(
            cfg.assert_extensions_approved(),
            Err(e) if e == ProgramError::Custom(ChanceryError::UnsupportedTokenExtension as u32),
        ));
    }

    #[test]
    fn forbidden_extension_fails() {
        let mint = Pubkey::new_unique();
        let (key, mut data, mut lamports) = make_initialized_asset_config(
            mint, asset_mode::ACTIVE, 1_000_000_000, 1_000_000_000,
            0, 0, u64::MAX,
        );

        {
            let cfg: &mut AssetConfig = bytemuck::from_bytes_mut(&mut data[..]);

            cfg.approved_extension_mask  = [0b11u64, 0u64];  // bits 0,1 approved
            cfg.observed_extension_mask  = [0b01u64, 0u64];  // bit 0 observed (approved)
            cfg.forbidden_extension_mask = [0b01u64, 0u64];  // bit 0 forbidden
        }

        let pid = program_id();
        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let cfg = AssetConfig::load(&ai).unwrap();

        assert!(matches!(
            cfg.assert_extensions_approved(),
            Err(e) if e == ProgramError::Custom(ChanceryError::ForbiddenExtension as u32),
        ));
    }

    #[test]
    fn collateral_default_forbids_transfer_hook_even_when_approved() {
        let mint = Pubkey::new_unique();
        let (key, mut data, mut lamports) = make_initialized_asset_config(
            mint, asset_mode::ACTIVE, 1_000_000_000, 1_000_000_000,
            0, 0, u64::MAX,
        );

        {
            let cfg: &mut AssetConfig = bytemuck::from_bytes_mut(&mut data[..]);
            cfg.approved_extension_mask = [crate::constants::extension_bit::TRANSFER_HOOK, 0];
            cfg.observed_extension_mask = [crate::constants::extension_bit::TRANSFER_HOOK, 0];
            cfg.forbidden_extension_mask = [0; 2];
        }

        let pid = program_id();
        let ai = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let cfg = AssetConfig::load(&ai).unwrap();

        assert!(matches!(
            cfg.assert_extensions_approved(),
            Err(e) if e == ProgramError::Custom(ChanceryError::ForbiddenExtension as u32),
        ));
    }

    #[test]
    fn collateral_allows_dormant_transfer_hook_when_approved() {
        let mint = Pubkey::new_unique();
        let (key, mut data, mut lamports) = make_initialized_asset_config(
            mint, asset_mode::ACTIVE, 1_000_000_000, 1_000_000_000,
            0, 0, u64::MAX,
        );

        {
            let cfg: &mut AssetConfig = bytemuck::from_bytes_mut(&mut data[..]);
            cfg.approved_extension_mask = [crate::constants::extension_bit::DORMANT_TRANSFER_HOOK, 0];
            cfg.observed_extension_mask = [crate::constants::extension_bit::DORMANT_TRANSFER_HOOK, 0];
            cfg.forbidden_extension_mask = [0; 2];
        }

        let pid = program_id();
        let ai = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let cfg = AssetConfig::load(&ai).unwrap();
        assert!(cfg.assert_extensions_approved().is_ok());
    }

    #[test]
    fn forbidden_mask_zero_passes_with_approved() {
        let mint = Pubkey::new_unique();
        let (key, mut data, mut lamports) = make_initialized_asset_config(
            mint, asset_mode::ACTIVE, 1_000_000_000, 1_000_000_000,
            0, 0, u64::MAX,
        );

        {
            let cfg: &mut AssetConfig = bytemuck::from_bytes_mut(&mut data[..]);

            cfg.approved_extension_mask  = [0b110u64, 0u64];
            cfg.observed_extension_mask  = [0b010u64, 0u64];
            cfg.forbidden_extension_mask = [0u64; 2];  // no asset-specific additions
        }

        let pid = program_id();
        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let cfg = AssetConfig::load(&ai).unwrap();

        assert!(cfg.assert_extensions_approved().is_ok());
    }

    #[test]
    fn forbidden_extension_high_word() {
        let mint = Pubkey::new_unique();
        let (key, mut data, mut lamports) = make_initialized_asset_config(
            mint, asset_mode::ACTIVE, 1_000_000_000, 1_000_000_000,
            0, 0, u64::MAX,
        );

        {
            let cfg: &mut AssetConfig = bytemuck::from_bytes_mut(&mut data[..]);

            cfg.approved_extension_mask  = [0u64, 0b10u64];  // high-word bit 1 approved
            cfg.observed_extension_mask  = [0u64, 0b10u64];  // high-word bit 1 observed
            cfg.forbidden_extension_mask = [0u64, 0b10u64];  // high-word bit 1 forbidden
        }

        let pid = program_id();
        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let cfg = AssetConfig::load(&ai).unwrap();

        assert!(matches!(
            cfg.assert_extensions_approved(),
            Err(e) if e == ProgramError::Custom(ChanceryError::ForbiddenExtension as u32),
        ));
    }

    #[test]
    fn forbidden_extensions_absent_rejects_asset_forbidden_bit() {
        let mint = Pubkey::new_unique();
        let (key, mut data, mut lamports) = make_initialized_asset_config(
            mint, asset_mode::ACTIVE, 1_000_000_000, 1_000_000_000,
            0, 0, u64::MAX,
        );

        {
            let cfg: &mut AssetConfig = bytemuck::from_bytes_mut(&mut data[..]);
            cfg.forbidden_extension_mask = [0b100u64, 0u64];
        }

        let pid = program_id();
        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let cfg = AssetConfig::load(&ai).unwrap();

        assert!(matches!(
            cfg.assert_forbidden_extensions_absent([0b100u64, 0u64]),
            Err(e) if e == ProgramError::Custom(ChanceryError::ForbiddenExtension as u32),
        ));
    }

    #[test]
    fn forbidden_extensions_absent_passes_when_disjoint() {
        let mint = Pubkey::new_unique();
        let (key, mut data, mut lamports) = make_initialized_asset_config(
            mint, asset_mode::ACTIVE, 1_000_000_000, 1_000_000_000,
            0, 0, u64::MAX,
        );

        {
            let cfg: &mut AssetConfig = bytemuck::from_bytes_mut(&mut data[..]);
            cfg.forbidden_extension_mask = [0b100u64, 0u64];
        }

        let pid = program_id();
        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let cfg = AssetConfig::load(&ai).unwrap();

        assert!(cfg.assert_forbidden_extensions_absent([0b011u64, 0u64]).is_ok());
    }

    #[test]
    fn approved_extension_passes() {
        let mint = Pubkey::new_unique();
        let (key, mut data, mut lamports) = make_initialized_asset_config(
            mint, asset_mode::ACTIVE, 1_000_000_000, 1_000_000_000,
            0, 0, u64::MAX,
        );

        {
            // direct mut access to data
            let cfg: &mut AssetConfig = bytemuck::from_bytes_mut(&mut data[..]);

            cfg.approved_extension_mask = [0b110u64, 0u64];  // allowed bits 1 and 2 approved
            cfg.observed_extension_mask = [0b010u64, 0u64];  // only allowed bit 1 observed
        }
        let pid = program_id();
        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let cfg = AssetConfig::load(&ai).unwrap();

        assert!(cfg.assert_extensions_approved().is_ok());
    }

    #[test]
    fn extensions_fresh_passes_within_age_window() {
        let mint = Pubkey::new_unique();
        let (key, mut data, mut lamports) = make_initialized_asset_config(
            mint, asset_mode::ACTIVE, 1_000_000_000, 1_000_000_000,
            0, 0, u64::MAX,
        );
        {
            let cfg: &mut AssetConfig = bytemuck::from_bytes_mut(&mut data[..]);
            cfg.extension_observed_at_slot          = 100;
            cfg.max_extension_observation_age_slots = 50;
        }
        let pid = program_id();
        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let cfg = AssetConfig::load(&ai).unwrap();

        assert!(cfg.assert_extensions_fresh(150).is_ok());
    }

    #[test]
    fn extensions_fresh_fails_when_stale() {
        let mint = Pubkey::new_unique();
        let (key, mut data, mut lamports) = make_initialized_asset_config(
            mint, asset_mode::ACTIVE, 1_000_000_000, 1_000_000_000,
            0, 0, u64::MAX,
        );
        {
            let cfg: &mut AssetConfig = bytemuck::from_bytes_mut(&mut data[..]);
            cfg.extension_observed_at_slot          = 100;
            cfg.max_extension_observation_age_slots = 50;
        }
        let pid = program_id();
        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let cfg = AssetConfig::load(&ai).unwrap();

        assert!(matches!(
            cfg.assert_extensions_fresh(151),
            Err(e) if e == ProgramError::Custom(ChanceryError::ExtensionObservationStale as u32),
        ));
    }

    #[test]
    fn extensions_fresh_disabled_when_max_age_zero() {
        let mint = Pubkey::new_unique();
        let (key, mut data, mut lamports) = make_initialized_asset_config(
            mint, asset_mode::ACTIVE, 1_000_000_000, 1_000_000_000,
            0, 0, u64::MAX,
        );
        {
            let cfg: &mut AssetConfig = bytemuck::from_bytes_mut(&mut data[..]);
            cfg.extension_observed_at_slot          = 0;
            cfg.max_extension_observation_age_slots = 0;
        }
        let pid = program_id();
        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let cfg = AssetConfig::load(&ai).unwrap();

        assert!(cfg.assert_extensions_fresh(u64::MAX).is_ok());
    }

    // ── Rate math ─────────────────────────────────────────────────────────────

    #[test]
    fn mint_output_1_to_1() {
        let mut cfg = AssetConfig::zeroed();

        cfg.deposit_rate_e9 = RATE_PRECISION_E9;

        assert_eq!(cfg.compute_mint_output(1_000_000).unwrap(), 1_000_000);
    }

    #[test]
    fn mint_output_2_to_1() {
        let mut cfg = AssetConfig::zeroed();

        cfg.deposit_rate_e9 = RATE_PRECISION_E9 * 2;

        assert_eq!(cfg.compute_mint_output(1_000_000).unwrap(), 2_000_000);
    }

    #[test]
    fn mint_output_half_rate() {
        // 0.5:1 - 1M asset -> 500K issued tokens
        let mut cfg = AssetConfig::zeroed();

        cfg.deposit_rate_e9 = RATE_PRECISION_E9 / 2;

        assert_eq!(cfg.compute_mint_output(1_000_000).unwrap(), 500_000);
    }

    #[test]
    fn mint_output_zero_rate_returns_zero() {
        let mut cfg = AssetConfig::zeroed();

        cfg.deposit_rate_e9 = 0;

        assert_eq!(cfg.compute_mint_output(1_000_000).unwrap(), 0);
    }

    #[test]
    fn mint_output_large_amount_no_overflow() {
        let mut cfg = AssetConfig::zeroed();

        cfg.deposit_rate_e9 = RATE_PRECISION_E9;  // 1:1

        // u32::MAX amount at 1:1 should not overflow
        assert_eq!(
            cfg.compute_mint_output(u32::MAX as u64).unwrap(),
            u32::MAX as u64,
        );
    }

    #[test]
    fn mint_output_overflow_returns_error() {
        let mut cfg = AssetConfig::zeroed();

        cfg.deposit_rate_e9 = u64::MAX;

        assert!(matches!(
            cfg.compute_mint_output(u64::MAX),
            Err(e) if e == ProgramError::Custom(ChanceryError::ArithmeticOverflow as u32),
        ));
    }

    #[test]
    fn redeem_output_1_to_1() {
        let mut cfg = AssetConfig::zeroed();

        cfg.redeem_rate_e9 = RATE_PRECISION_E9;

        assert_eq!(cfg.compute_redeem_output(500_000).unwrap(), 500_000);
    }

    #[test]
    fn redeem_output_fractional_truncates() {
        // rate = 1.5:1, 100 tokens -> 150.0 -> 150 (truncate)
        let mut cfg = AssetConfig::zeroed();

        cfg.redeem_rate_e9 = RATE_PRECISION_E9 + RATE_PRECISION_E9 / 2;

        assert_eq!(cfg.compute_redeem_output(100).unwrap(), 150);
    }

    #[test]
    fn deposit_and_redeem_are_inverse_at_1_to_1() {
        let mut cfg  = AssetConfig::zeroed();

        cfg.deposit_rate_e9 = RATE_PRECISION_E9;
        cfg.redeem_rate_e9  = RATE_PRECISION_E9;

        let amount   = 1_234_567u64;
        let minted   = cfg.compute_mint_output(amount).unwrap();
        let redeemed = cfg.compute_redeem_output(minted).unwrap();

        assert_eq!(amount, redeemed);
    }
}

// ─── AuthorityTransfer ────────────────────────────────────────────────────────

#[cfg(test)]
mod authority_transfer_tests {
    use super::*;

    fn make_initialized_transfer(
        role_kind:             u8,
        old_authority:         Pubkey,
        proposed_authority:    Pubkey,
        executable_after_slot: u64,
    ) -> (Pubkey, Vec<u8>, u64) {
        let pid          = program_id();
        let (key, bump)  = Pubkey::find_program_address(
            &[seeds::AUTHORITY_TRANSFER, &[role_kind]], &pid,
        );
        let mut data     = vec![0u8; AUTHORITY_TRANSFER_SIZE];
        let lamports     = 1u64;

        {
            let t: &mut AuthorityTransfer = bytemuck::from_bytes_mut(&mut data[..]);

            t.discriminator         = AUTHORITY_TRANSFER_DISCRIMINATOR;
            t.version               = 1;
            t.bump                  = bump;
            t.role_kind             = role_kind;
            t.old_authority         = old_authority;
            t.proposed_authority    = proposed_authority;
            t.proposed_at_slot      = 100;
            t.executable_after_slot = executable_after_slot;
            t._reserved             = [0u8; 16];
        }

        (key, data, lamports)
    }

    #[test]
    fn size_matches_declared() {
        assert_eq!(core::mem::size_of::<AuthorityTransfer>(), 152);
        assert_eq!(AUTHORITY_TRANSFER_SIZE, 152);
        assert_eq!(core::mem::offset_of!(AuthorityTransfer, executable_after_slot), 88);
        assert_eq!(core::mem::offset_of!(AuthorityTransfer, proposing_governance), 96);
        assert_eq!(core::mem::offset_of!(AuthorityTransfer, expires_at_slot), 128);
        assert_eq!(core::mem::offset_of!(AuthorityTransfer, _reserved), 136);
    }

    #[test]
    fn zeroed_load_returns_not_initialized() {
        let pid             = program_id();
        let (key, mut data) = make_account(AUTHORITY_TRANSFER_SIZE);
        let mut lamports    = 1u64;
        let ai              = make_account_info(&key, &mut data, &mut lamports, &pid, false);

        assert!(matches!(
            AuthorityTransfer::load(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::NotInitialized as u32),
        ));
    }

    #[test]
    fn load_reads_fields_correctly() {
        let old      = Pubkey::new_unique();
        let proposed = Pubkey::new_unique();
        let pid      = program_id();
        let (key, mut data, mut lamports) = make_initialized_transfer(
            authority_role::OPS, old, proposed, 500,
        );
        let ai       = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let t        = AuthorityTransfer::load(&ai).unwrap();

        assert_eq!(t.role_kind,             authority_role::OPS);
        assert_eq!(t.old_authority,         old);
        assert_eq!(t.proposed_authority,    proposed);
        assert_eq!(t.executable_after_slot, 500);
        assert!(t._reserved.iter().all(|&b| b == 0));
    }

    #[test]
    fn timelock_not_elapsed_returns_error() {
        let pid = program_id();
        let (key, mut data, mut lamports) = make_initialized_transfer(
            authority_role::GOVERNANCE,
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            1000,  // executable after slot 1000
        );

        let ai = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let t  = AuthorityTransfer::load(&ai).unwrap();

        // Current slot 999 < 1000
        assert!(matches!(
            t.assert_timelock_elapsed(999),
            Err(e) if e == ProgramError::Custom(ChanceryError::AuthorityTransferTimelockActive as u32),
        ));
    }

    #[test]
    fn timelock_elapsed_at_exact_slot_passes() {
        let pid = program_id();
        let (key, mut data, mut lamports) = make_initialized_transfer(
            authority_role::GOVERNANCE,
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            1000,
        );
        let ai = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let t  = AuthorityTransfer::load(&ai).unwrap();

        // At exactly slot 1000 -> elapsed
        assert!(t.assert_timelock_elapsed(1000).is_ok());
    }

    #[test]
    fn zero_timelock_always_elapsed() {
        let pid = program_id();
        let (key, mut data, mut lamports) = make_initialized_transfer(
            authority_role::OPS,
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            0,  // no timelock
        );
        let ai = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let t  = AuthorityTransfer::load(&ai).unwrap();

        assert!(t.assert_timelock_elapsed(0).is_ok());
        assert!(t.assert_timelock_elapsed(u64::MAX).is_ok());
    }

    #[test]
    fn assert_pending_passes_on_initialized() {
        let pid = program_id();
        let (key, mut data, mut lamports) = make_initialized_transfer(
            authority_role::EMERGENCY,
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            0,
        );
        let ai = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let t  = AuthorityTransfer::load(&ai).unwrap();

        assert!(t.assert_pending().is_ok());
    }

    #[test]
    fn assert_pending_fails_on_zeroed() {
        // Zeroed discriminator -> not pending
        let t = AuthorityTransfer::zeroed();

        assert!(matches!(
            t.assert_pending(),
            Err(e) if e == ProgramError::Custom(ChanceryError::AuthorityTransferNotPending as u32),
        ));
    }

    #[test]
    fn all_role_kinds_derive_distinct_pdas() {
        use crate::constants::authority_role::*;

        let pid = program_id();
        let keys: Vec<Pubkey> = [GOVERNANCE, OPS, EMERGENCY, ENFORCEMENT, INSURANCE_ADMIN]
            .iter()
            .map(|&r| AuthorityTransfer::pda(r, &pid).0)
            .collect();
        // All five must be distinct
        let unique: std::collections::HashSet<_> = keys.iter().collect();

        assert_eq!(unique.len(), 5, "Role PDAs must all be distinct");
    }
}
