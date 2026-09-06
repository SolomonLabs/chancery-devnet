/// modules/issuance/tests/mod.rs
///
/// Unit tests for IssuedTokenControl state struct and helpers.
/// Pure tests (no CPI, no runtime). Run with:
///   cargo test --features no-entrypoint modules::issuance::tests

use std::{cell::RefCell, rc::Rc};

use bytemuck::Zeroable;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::{extension_bit, issued_token_deployment_flag, seeds, status_flag},
    error::ChanceryError,
    modules::issuance::state::issued_token_control::{
        IssuedTokenControl, ISSUED_TOKEN_CONTROL_DISCRIMINATOR, ISSUED_TOKEN_CONTROL_SIZE,
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

// ─── Size ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod issued_token_control_tests {
    use super::*;

    #[test]
    fn size_matches_declared() {
        assert_eq!(
            core::mem::size_of::<IssuedTokenControl>(),
            ISSUED_TOKEN_CONTROL_SIZE,
            "IssuedTokenControl: size_of does not match ISSUED_TOKEN_CONTROL_SIZE",
        );
    }

    #[test]
    fn zeroed_struct_is_all_zero_bytes() {
        let ctl   = IssuedTokenControl::zeroed();
        let bytes = bytemuck::bytes_of(&ctl);

        assert!(bytes.iter().all(|&b| b == 0), "Zeroed struct must be all-zero");
    }

    // ── PDA ───────────────────────────────────────────────────────────────────

    #[test]
    fn pda_derivation_is_deterministic() {
        let pid           = program_id();
        let (key1, bump1) = IssuedTokenControl::pda(&pid);
        let (key2, bump2) = IssuedTokenControl::pda(&pid);

        assert_eq!(key1, key2, "PDA derivation must be deterministic");
        assert_eq!(bump1, bump2);
    }

    #[test]
    fn verify_pda_rejects_wrong_key() {
        let pid           = program_id();
        let wrong_key     = Pubkey::new_unique();
        let mut data      = vec![0u8; ISSUED_TOKEN_CONTROL_SIZE];
        let mut lamports  = 1u64;
        let ai            = make_account_info(&wrong_key, &mut data, &mut lamports, &pid, false);

        assert!(matches!(
            IssuedTokenControl::verify_pda(&ai, &pid),
            Err(e) if e == ProgramError::Custom(ChanceryError::InvalidPda as u32),
        ));
    }

    #[test]
    fn verify_pda_accepts_correct_key() {
        let pid                  = program_id();
        let (correct_key, _bump) = IssuedTokenControl::pda(&pid);
        let mut data             = vec![0u8; ISSUED_TOKEN_CONTROL_SIZE];
        let mut lamports         = 1u64;
        let ai                   = make_account_info(&correct_key, &mut data, &mut lamports, &pid, false);

        assert!(IssuedTokenControl::verify_pda(&ai, &pid).is_ok());
    }

    // ── Discriminator / Load ──────────────────────────────────────────────────

    #[test]
    fn zeroed_load_returns_not_initialized() {
        let pid             = program_id();
        let (key, mut data) = make_account(ISSUED_TOKEN_CONTROL_SIZE);
        let mut lamports    = 1u64;
        let ai              = make_account_info(&key, &mut data, &mut lamports, &pid, false);

        assert!(matches!(
            IssuedTokenControl::load(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::NotInitialized as u32),
        ));
    }

    #[test]
    fn wrong_data_length_returns_error() {
        let pid             = program_id();
        let (key, mut data) = make_account(ISSUED_TOKEN_CONTROL_SIZE - 1);
        let mut lamports    = 1u64;
        let ai              = make_account_info(&key, &mut data, &mut lamports, &pid, false);

        assert!(matches!(
            IssuedTokenControl::load(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::AccountDataLengthMismatch as u32),
        ));
    }

    #[test]
    fn load_uninitialized_blocks_double_init() {
        let pid             = program_id();
        let (key, mut data) = make_account(ISSUED_TOKEN_CONTROL_SIZE);
        let mut lamports    = 1u64;

        {
            let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, true);
            let mut ctl = IssuedTokenControl::load_uninitialized_mut(&ai).unwrap();

            ctl.discriminator = ISSUED_TOKEN_CONTROL_DISCRIMINATOR;
            ctl.control_flags = status_flag::INITIALIZED;
        }

        {
            let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, true);

            assert!(matches!(
                IssuedTokenControl::load_uninitialized_mut(&ai),
                Err(e) if e == ProgramError::Custom(ChanceryError::AlreadyInitialized as u32),
            ));
        }
    }

    #[test]
    fn load_mut_on_non_writable_returns_error() {
        let pid             = program_id();
        let (key, mut data) = make_account(ISSUED_TOKEN_CONTROL_SIZE);
        let mut lamports    = 1u64;

        {
            let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, true);
            let mut ctl = IssuedTokenControl::load_uninitialized_mut(&ai).unwrap();

            ctl.discriminator = ISSUED_TOKEN_CONTROL_DISCRIMINATOR;
            ctl.control_flags = status_flag::INITIALIZED;
        }

        let ai = make_account_info(&key, &mut data, &mut lamports, &pid, false);

        assert!(matches!(
            IssuedTokenControl::load_mut(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::AccountNotWritable as u32),
        ));
    }

    #[test]
    fn load_reads_fields_correctly() {
        let pid             = program_id();
        let mint            = Pubkey::new_unique();
        let token_program   = Pubkey::new_unique();
        let (key, mut data) = make_account(ISSUED_TOKEN_CONTROL_SIZE);
        let mut lamports    = 1u64;

        {
            let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, true);
            let mut ctl = IssuedTokenControl::load_uninitialized_mut(&ai).unwrap();

            ctl.discriminator                = ISSUED_TOKEN_CONTROL_DISCRIMINATOR;
            ctl.version                      = 1;
            ctl.bump                         = 254;
            ctl.control_flags                = status_flag::INITIALIZED;
            ctl.issued_token_mint            = mint;
            ctl.issued_token_program         = token_program;
            ctl.reserved_mint_extension_mask = [0xFF, 0];
        }

        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let ctl = IssuedTokenControl::load(&ai).unwrap();

        assert_eq!(ctl.version, 1);
        assert_eq!(ctl.bump, 254);
        assert_eq!(ctl.issued_token_mint, mint);
        assert_eq!(ctl.issued_token_program, token_program);
        assert_eq!(ctl.reserved_mint_extension_mask, [0xFF, 0]);
    }

    // ── Extension mask helpers ────────────────────────────────────────────────

    #[test]
    fn is_mint_extension_reserved_low_bits() {
        let mut ctl = IssuedTokenControl::zeroed();

        ctl.reserved_mint_extension_mask = [
            extension_bit::TRANSFER_HOOK
                | extension_bit::CONFIDENTIAL_TRANSFER
                | extension_bit::PERMANENT_DELEGATE
                | extension_bit::PAUSABLE,
            0,
        ];

        assert!(ctl.is_mint_extension_reserved(0));   // TRANSFER_HOOK
        assert!(ctl.is_mint_extension_reserved(1));   // CONFIDENTIAL_TRANSFER
        assert!(!ctl.is_mint_extension_reserved(2));  // CONFIDENTIAL_MINT_BURN not set
        assert!(ctl.is_mint_extension_reserved(3));   // PERMANENT_DELEGATE
        assert!(ctl.is_mint_extension_reserved(4));   // PAUSABLE
        assert!(!ctl.is_mint_extension_reserved(5));  // METADATA_POINTER not set
    }

    #[test]
    fn is_mint_extension_reserved_high_bits() {
        let mut ctl = IssuedTokenControl::zeroed();

        ctl.reserved_mint_extension_mask = [0, 1u64 << 3];

        assert!(ctl.is_mint_extension_reserved(64 + 3));
        assert!(!ctl.is_mint_extension_reserved(64 + 4));
    }

    #[test]
    fn is_mint_extension_active_tracks_independently() {
        let mut ctl = IssuedTokenControl::zeroed();

        ctl.reserved_mint_extension_mask = [extension_bit::TRANSFER_HOOK | extension_bit::PAUSABLE, 0];
        ctl.active_mint_extension_mask   = [extension_bit::TRANSFER_HOOK, 0];

        assert!(ctl.is_mint_extension_reserved(0));   // reserved
        assert!(ctl.is_mint_extension_active(0));    // also active
        assert!(ctl.is_mint_extension_reserved(4));  // reserved
        assert!(!ctl.is_mint_extension_active(4));   // not yet active
    }

    // ── Authority PDA derivations ─────────────────────────────────────────────

    #[test]
    fn all_authority_pda_seeds_derive_distinct_keys() {
        let pid = program_id();
        let authority_seeds: &[&[u8]] = &[
            seeds::MINT_AUTHORITY,
            seeds::FREEZE_AUTHORITY,
            seeds::CLOSE_MINT_AUTHORITY,
            seeds::TRANSFER_HOOK_AUTHORITY,
            seeds::PERMANENT_DELEGATE_AUTHORITY,
            seeds::METADATA_POINTER_AUTHORITY,
            seeds::METADATA_UPDATE_AUTHORITY,
            seeds::PAUSE_AUTHORITY,
            seeds::CONFIDENTIAL_TRANSFER_AUTHORITY,
            seeds::DEFAULT_ACCOUNT_STATE_AUTHORITY,
        ];

        let keys: Vec<Pubkey> = authority_seeds
            .iter()
            .map(|seed| Pubkey::find_program_address(&[seed], &pid).0)
            .collect();

        let unique: std::collections::HashSet<_> = keys.iter().collect();

        assert_eq!(unique.len(), authority_seeds.len(), "All authority PDAs must be distinct");
    }

    #[test]
    fn confidential_transfer_and_transfer_hook_combination_is_allowed() {
        let mask     = [extension_bit::CONFIDENTIAL_TRANSFER | extension_bit::TRANSFER_HOOK, 0];
        let mut ctl  = IssuedTokenControl::zeroed();

        ctl.reserved_mint_extension_mask = mask;

        assert!(ctl.is_mint_extension_reserved(0));  // TRANSFER_HOOK
        assert!(ctl.is_mint_extension_reserved(1));  // CONFIDENTIAL_TRANSFER
    }

    #[test]
    fn default_forbidden_transfer_fee_config_does_not_overlap_default_reserved() {
        use crate::constants::{extension_bit as ex, issued_token_default_forbidden as forb};

        // Recommended default reserved set per spec §9.4
        let default_reserved_lo =
            ex::MINT_CLOSE_AUTHORITY
            | ex::CONFIDENTIAL_TRANSFER
            | ex::DEFAULT_ACCOUNT_STATE
            | ex::PERMANENT_DELEGATE
            | ex::TRANSFER_HOOK
            | ex::METADATA_POINTER
            | ex::TOKEN_METADATA
            | ex::CONFIDENTIAL_MINT_BURN
            | ex::PAUSABLE
            | ex::DORMANT_TRANSFER_HOOK;

        assert_eq!(default_reserved_lo & forb::MINT_FORBIDDEN_LO, 0);
    }

    #[test]
    fn active_must_be_subset_of_reserved_mint() {
        let mut ctl  = IssuedTokenControl::zeroed();

        ctl.reserved_mint_extension_mask = [extension_bit::TRANSFER_HOOK, 0];
        ctl.active_mint_extension_mask   = [extension_bit::TRANSFER_HOOK, 0];

        let active   = ctl.active_mint_extension_mask;
        let reserved = ctl.reserved_mint_extension_mask;

        assert_eq!(active[0] & !reserved[0], 0);
        assert_eq!(active[1] & !reserved[1], 0);
    }

    #[test]
    fn observed_mint_mask_rejects_unreserved_extension() {
        let mut ctl = IssuedTokenControl::zeroed();
        ctl.reserved_mint_extension_mask = [extension_bit::TRANSFER_HOOK, 0];

        assert!(matches!(
            ctl.assert_observed_mint_mask_valid([extension_bit::PERMANENT_DELEGATE, 0]),
            Err(e) if e == ProgramError::Custom(ChanceryError::ExtensionNotReserved as u32),
        ));
    }

    #[test]
    fn observed_mint_mask_rejects_default_forbidden_extension() {
        let mut ctl = IssuedTokenControl::zeroed();
        ctl.reserved_mint_extension_mask = [extension_bit::TRANSFER_FEE_CONFIG, 0];

        assert!(matches!(
            ctl.assert_observed_mint_mask_valid([extension_bit::TRANSFER_FEE_CONFIG, 0]),
            Err(e) if e == ProgramError::Custom(
                ChanceryError::IssuedTokenForbiddenDefaultExtensionActive as u32
            ),
        ));
    }

    #[test]
    fn asset_forbidden_mask_uses_forbidden_extension_error() {
        use crate::modules::core::state::asset_config::AssetConfig;

        let mut cfg = AssetConfig::zeroed();
        cfg.forbidden_extension_mask = [extension_bit::PERMANENT_DELEGATE, 0];

        assert!(matches!(
            cfg.assert_forbidden_extensions_absent([extension_bit::PERMANENT_DELEGATE, 0]),
            Err(e) if e == ProgramError::Custom(ChanceryError::ForbiddenExtension as u32),
        ));
    }

    #[test]
    fn default_forbidden_takes_precedence_over_asset_forbidden() {
        use crate::modules::core::state::asset_config::AssetConfig;

        let mut ctl = IssuedTokenControl::zeroed();
        ctl.reserved_mint_extension_mask = [extension_bit::TRANSFER_FEE_CONFIG, 0];

        let mut cfg = AssetConfig::zeroed();
        cfg.forbidden_extension_mask = [extension_bit::TRANSFER_FEE_CONFIG, 0];

        let observed = [extension_bit::TRANSFER_FEE_CONFIG, 0];

        // Audit #64: protocol-default hits keep IssuedTokenForbiddenDefaultExtensionActive
        // even when the operator also listed the same bit in asset_config.
        assert!(matches!(
            ctl.assert_observed_mint_mask_valid(observed),
            Err(e) if e == ProgramError::Custom(
                ChanceryError::IssuedTokenForbiddenDefaultExtensionActive as u32
            ),
        ));
        assert!(cfg.assert_forbidden_extensions_absent(observed).is_err());
    }

    #[test]
    fn assert_ready_for_settlement_rejects_when_flag_clear() {
        let mut ctl = IssuedTokenControl::zeroed();

        ctl.control_flags = status_flag::INITIALIZED;

        assert!(matches!(
            ctl.assert_ready_for_settlement(),
            Err(e) if e == ProgramError::Custom(ChanceryError::IssuedTokenDeploymentNotVerified as u32),
        ));
    }

    #[test]
    fn assert_ready_for_settlement_accepts_ready_flag() {
        let mut ctl = IssuedTokenControl::zeroed();

        ctl.control_flags =
            status_flag::INITIALIZED | issued_token_deployment_flag::READY_FOR_SETTLEMENT;

        assert!(ctl.assert_ready_for_settlement().is_ok());
    }

    #[test]
    fn tlv_non_transferable_observation_rejected_as_default_forbidden() {
        use crate::modules::issuance::tlv::tlv_parser::{self, extension_tlv_tag};

        let mut data = vec![0u8; tlv_parser::ACCOUNT_TYPE_OFFSET];
        data.push(tlv_parser::ACCOUNT_TYPE_MINT);
        data.extend_from_slice(&[
            (extension_tlv_tag::NON_TRANSFERABLE & 0xFF) as u8,
            (extension_tlv_tag::NON_TRANSFERABLE >> 8) as u8,
            1,
            0,
            0,
        ]);

        let observed = tlv_parser::parse_mint_extension_mask_full(&data).unwrap();
        let mut ctl = IssuedTokenControl::zeroed();
        ctl.reserved_mint_extension_mask = [
            extension_bit::TRANSFER_HOOK | extension_bit::NON_TRANSFERABLE,
            0,
        ];

        assert!(matches!(
            ctl.assert_observed_mint_mask_valid(observed),
            Err(e) if e == ProgramError::Custom(
                ChanceryError::IssuedTokenForbiddenDefaultExtensionActive as u32
            ),
        ));
    }
}
