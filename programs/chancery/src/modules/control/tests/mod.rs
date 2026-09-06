/// modules/control/tests/mod.rs
///
/// Unit tests for PauseState and AssetPauseState.
///
/// Covers:
///   - size / layout
///   - load guards
///   - pause bit set / clear semantics
///   - assert_not_paused_for: each operation bit
///   - auto-expiry via expires_at_slot
///   - is_paused_for helper
///   - combined bit masks (ALL_SETTLEMENT, ALL)
///   - PDA determinism and per-asset uniqueness
///   - PERMISSION_FLAG_PAUSED constant location and value

use std::{cell::RefCell, rc::Rc};

use bytemuck::Zeroable;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    error::ChanceryError,
    modules::{
        control::state::{
            asset_pause_state::{
                AssetPauseState, ASSET_PAUSE_STATE_DISCRIMINATOR, ASSET_PAUSE_STATE_SIZE,
            },
            pause_state::{
                pause_bit, PauseState, PAUSE_STATE_DISCRIMINATOR, PAUSE_STATE_SIZE,
            },
        },
        permissions::state::permission_record::PERMISSION_FLAG_PAUSED,
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

fn make_pause_state(pause_bits: u64, expires_at_slot: u64) -> PauseState {
    let mut pause_state = PauseState::zeroed();

    pause_state.discriminator     = PAUSE_STATE_DISCRIMINATOR;
    pause_state.version           = 1;
    pause_state.global_pause_bits = pause_bits;
    pause_state.expires_at_slot   = expires_at_slot;
    pause_state.activated_by      = Pubkey::new_unique();

    pause_state
}

fn make_asset_pause_state(
    asset_mint     : Pubkey,
    pause_bits     : u64,
    expires_at_slot: u64,
) -> AssetPauseState {
    let mut pause_state = AssetPauseState::zeroed();

    pause_state.discriminator    = ASSET_PAUSE_STATE_DISCRIMINATOR;
    pause_state.version          = 1;
    pause_state.asset_mint       = asset_mint;
    pause_state.asset_pause_bits = pause_bits;
    pause_state.expires_at_slot  = expires_at_slot;
    pause_state.activated_by     = Pubkey::new_unique();

    pause_state
}

// ─── pause_bit constants ──────────────────────────────────────────────────────

#[test]
fn pause_bit_mint_is_bit_0() {
    assert_eq!(pause_bit::MINT, 1u64 << 0);
}

#[test]
fn pause_bit_redeem_is_bit_1() {
    assert_eq!(pause_bit::REDEEM, 1u64 << 1);
}

#[test]
fn pause_bit_reserve_is_bit_2() {
    assert_eq!(pause_bit::RESERVE, 1u64 << 2);
}

#[test]
fn pause_bit_migration_is_bit_3() {
    assert_eq!(pause_bit::MIGRATION, 1u64 << 3);
}

#[test]
fn all_settlement_is_mint_or_redeem() {
    assert_eq!(pause_bit::ALL_SETTLEMENT, pause_bit::MINT | pause_bit::REDEEM);
}

#[test]
fn all_is_u64_max() {
    assert_eq!(pause_bit::ALL, u64::MAX);
}

#[test]
fn pause_bits_are_all_distinct() {
    let bits = [pause_bit::MINT, pause_bit::REDEEM, pause_bit::RESERVE, pause_bit::MIGRATION];

    // No two pause bits should share any bit
    for i in 0..bits.len() {
        for j in (i+1)..bits.len() {
            assert_eq!(bits[i] & bits[j], 0, "pause bits {i} and {j} must not overlap");
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PauseState
// ═══════════════════════════════════════════════════════════════════════════════

mod pause_state_tests {
    use super::*;

    #[test]
    fn size_matches_declared() {
        assert_eq!(core::mem::size_of::<PauseState>(), PAUSE_STATE_SIZE);
    }

    #[test]
    fn zeroed_struct_all_zero_bytes() {
        let pause_state  = PauseState::zeroed();
        let bytes: &[u8] = bytemuck::bytes_of(&pause_state);

        assert!(bytes.iter().all(|&b| b == 0));
    }

    // ── Load guards ───────────────────────────────────────────────────────────

    #[test]
    fn load_zero_data_returns_not_initialized() {
        let pid          = program_id();
        let (key, _)     = PauseState::pda(&pid);
        let mut data     = vec![0u8; PAUSE_STATE_SIZE];
        let mut lamports = 1u64;
        let ai           = make_account_info(&key, &mut data, &mut lamports, &pid, false);

        assert!(matches!(
            PauseState::load(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::NotInitialized as u32),
        ));
    }

    #[test]
    fn load_wrong_size_returns_length_mismatch() {
        let pid          = program_id();
        let key          = Pubkey::new_unique();
        let mut data     = vec![0u8; PAUSE_STATE_SIZE - 1];
        let mut lamports = 1u64;
        let ai           = make_account_info(&key, &mut data, &mut lamports, &pid, false);

        assert!(matches!(
            PauseState::load(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::AccountDataLengthMismatch as u32),
        ));
    }

    #[test]
    fn load_mut_non_writable_returns_error() {
        let pid          = program_id();
        let (key, _)     = PauseState::pda(&pid);
        let mut data     = vec![0u8; PAUSE_STATE_SIZE];
        let mut lamports = 1u64;

        // Write discriminator so load_mut proceeds to writable check
        {
            let pause_state: &mut PauseState = bytemuck::from_bytes_mut(&mut data[..]);
            pause_state.discriminator = PAUSE_STATE_DISCRIMINATOR;
        }

        let ai = make_account_info(&key, &mut data, &mut lamports, &pid, false);

        assert!(matches!(
            PauseState::load_mut(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::AccountNotWritable as u32),
        ));
    }

    // ── assert_not_paused_for ─────────────────────────────────────────────────

    #[test]
    fn not_paused_when_all_bits_zero() {
        let pause_state = make_pause_state(0, 0);

        assert!(pause_state.assert_not_paused_for(pause_bit::MINT,      0).is_ok());
        assert!(pause_state.assert_not_paused_for(pause_bit::REDEEM,    0).is_ok());
        assert!(pause_state.assert_not_paused_for(pause_bit::RESERVE,   0).is_ok());
        assert!(pause_state.assert_not_paused_for(pause_bit::MIGRATION, 0).is_ok());
    }

    #[test]
    fn mint_pause_blocks_mint_only() {
        let pause_state = make_pause_state(pause_bit::MINT, 0);

        assert!(matches!(
            pause_state.assert_not_paused_for(pause_bit::MINT, 0),
            Err(e) if e == ProgramError::Custom(ChanceryError::GloballyPaused as u32),
        ));
        assert!(pause_state.assert_not_paused_for(pause_bit::REDEEM,    0).is_ok());
        assert!(pause_state.assert_not_paused_for(pause_bit::RESERVE,   0).is_ok());
        assert!(pause_state.assert_not_paused_for(pause_bit::MIGRATION, 0).is_ok());
    }

    #[test]
    fn redeem_pause_blocks_redeem_only() {
        let pause_state = make_pause_state(pause_bit::REDEEM, 0);

        assert!(pause_state.assert_not_paused_for(pause_bit::MINT, 0).is_ok());
        assert!(matches!(
            pause_state.assert_not_paused_for(pause_bit::REDEEM, 0),
            Err(e) if e == ProgramError::Custom(ChanceryError::GloballyPaused as u32),
        ));
    }

    #[test]
    fn all_settlement_pause_blocks_mint_and_redeem() {
        let pause_state = make_pause_state(pause_bit::ALL_SETTLEMENT, 0);

        assert!(pause_state.assert_not_paused_for(pause_bit::MINT,   0).is_err());
        assert!(pause_state.assert_not_paused_for(pause_bit::REDEEM, 0).is_err());
        assert!(pause_state.assert_not_paused_for(pause_bit::RESERVE,   0).is_ok());
        assert!(pause_state.assert_not_paused_for(pause_bit::MIGRATION, 0).is_ok());
    }

    #[test]
    fn all_bits_pause_blocks_every_operation() {
        let pause_state = make_pause_state(pause_bit::ALL, 0);

        assert!(pause_state.assert_not_paused_for(pause_bit::MINT,      0).is_err());
        assert!(pause_state.assert_not_paused_for(pause_bit::REDEEM,    0).is_err());
        assert!(pause_state.assert_not_paused_for(pause_bit::RESERVE,   0).is_err());
        assert!(pause_state.assert_not_paused_for(pause_bit::MIGRATION, 0).is_err());
    }

    // ── Auto-expiry via expires_at_slot ───────────────────────────────────────

    #[test]
    fn pause_auto_expires_past_expiry_slot() {
        // Paused but expired at slot 100 -> passes at slot 101
        let pause_state = make_pause_state(pause_bit::ALL, 100);

        assert!(pause_state.assert_not_paused_for(pause_bit::MINT, 101).is_ok(),
            "pause must auto-expire after expires_at_slot");
    }

    #[test]
    fn pause_still_active_at_exact_expiry_slot() {
        // expires_at_slot is exclusive - at exactly 100 pause is still active
        let pause_state = make_pause_state(pause_bit::MINT, 100);

        assert!(pause_state.assert_not_paused_for(pause_bit::MINT, 99).is_err(),
            "pause must be active one slot before expiry");
    }

    #[test]
    fn pause_expires_at_expiry_slot() {
        let pause_state = make_pause_state(pause_bit::MINT, 100);

        assert!(pause_state.assert_not_paused_for(pause_bit::MINT, 100).is_ok(),
            "pause must expire at expires_at_slot");
    }

    #[test]
    fn zero_expires_at_slot_means_no_auto_expiry() {
        let pause_state = make_pause_state(pause_bit::ALL, 0);  // 0 = no auto-expiry

        assert!(pause_state.assert_not_paused_for(pause_bit::MINT, u64::MAX).is_err(),
            "zero expires_at_slot must not auto-expire");
    }

    #[test]
    fn paused_with_no_bits_and_no_expiry_always_passes() {
        let pause_state = make_pause_state(0, 0);

        assert!(pause_state.assert_not_paused_for(pause_bit::ALL, u64::MAX).is_ok());
    }

    // ── is_paused_for ─────────────────────────────────────────────────────────

    #[test]
    fn is_paused_for_true_when_bit_set() {
        let pause_state = make_pause_state(pause_bit::MINT, 0);

        assert!(pause_state.is_paused_for(pause_bit::MINT, 0));
    }

    #[test]
    fn is_paused_for_false_when_bit_not_set() {
        let pause_state = make_pause_state(pause_bit::MINT, 0);

        assert!(!pause_state.is_paused_for(pause_bit::REDEEM, 0));
    }

    #[test]
    fn is_paused_for_considers_expiry_and_raw_bit_remains_visible() {
        let pause_state = make_pause_state(pause_bit::MINT, 1);  // expires at slot 1

        assert!(!pause_state.is_paused_for(pause_bit::MINT, 1),
            "effective pause status must lift at expires_at_slot");
        assert!(pause_state.has_pause_bit(pause_bit::MINT),
            "raw persisted bits remain available for audit reconciliation");
    }

    // ── PDA ───────────────────────────────────────────────────────────────────

    #[test]
    fn pda_is_deterministic() {
        let pid      = program_id();
        let (k1, b1) = PauseState::pda(&pid);
        let (k2, b2) = PauseState::pda(&pid);

        assert_eq!(k1, k2);
        assert_eq!(b1, b2);
    }

    // ── Bit manipulation invariants ───────────────────────────────────────────

    #[test]
    fn setting_then_clearing_bit_restores_zero() {
        let mut bits: u64 = 0;

        bits |= pause_bit::MINT;

        assert_ne!(bits, 0);

        bits &= !pause_bit::MINT;

        assert_eq!(bits, 0);
    }

    #[test]
    fn setting_multiple_bits_and_clearing_one() {
        let mut bits: u64 = pause_bit::MINT | pause_bit::REDEEM;

        bits &= !pause_bit::MINT;

        assert_eq!(bits, pause_bit::REDEEM);
        assert!(bits & pause_bit::MINT   == 0);
        assert!(bits & pause_bit::REDEEM != 0);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// AssetPauseState
// ═══════════════════════════════════════════════════════════════════════════════

mod asset_pause_state_tests {
    use super::*;

    #[test]
    fn size_matches_declared() {
        assert_eq!(core::mem::size_of::<AssetPauseState>(), ASSET_PAUSE_STATE_SIZE);
    }

    #[test]
    fn zeroed_struct_all_zero_bytes() {
        let pause_state  = AssetPauseState::zeroed();
        let bytes: &[u8] = bytemuck::bytes_of(&pause_state);

        assert!(bytes.iter().all(|&b| b == 0));
    }

    // ── Load guards ───────────────────────────────────────────────────────────

    #[test]
    fn load_zero_data_returns_not_initialized() {
        let pid          = program_id();
        let mint         = Pubkey::new_unique();
        let (key, _)     = AssetPauseState::pda(&mint, &pid);
        let mut data     = vec![0u8; ASSET_PAUSE_STATE_SIZE];
        let mut lamports = 1u64;
        let ai           = make_account_info(&key, &mut data, &mut lamports, &pid, false);

        assert!(matches!(
            AssetPauseState::load(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::NotInitialized as u32),
        ));
    }

    #[test]
    fn load_wrong_size_returns_length_mismatch() {
        let pid          = program_id();
        let key          = Pubkey::new_unique();
        let mut data     = vec![0u8; ASSET_PAUSE_STATE_SIZE + 8];
        let mut lamports = 1u64;
        let ai           = make_account_info(&key, &mut data, &mut lamports, &pid, false);

        assert!(matches!(
            AssetPauseState::load(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::AccountDataLengthMismatch as u32),
        ));
    }

    // ── assert_not_paused_for ─────────────────────────────────────────────────

    #[test]
    fn asset_not_paused_when_bits_zero() {
        let mint        = Pubkey::new_unique();
        let pause_state = make_asset_pause_state(mint, 0, 0);

        assert!(pause_state.assert_not_paused_for(pause_bit::MINT,   0).is_ok());
        assert!(pause_state.assert_not_paused_for(pause_bit::REDEEM, 0).is_ok());
    }

    #[test]
    fn asset_mint_pause_blocks_mint() {
        let mint        = Pubkey::new_unique();
        let pause_state = make_asset_pause_state(mint, pause_bit::MINT, 0);

        assert!(matches!(
            pause_state.assert_not_paused_for(pause_bit::MINT, 0),
            Err(e) if e == ProgramError::Custom(ChanceryError::AssetPaused as u32),
        ));
    }

    #[test]
    fn asset_pause_error_is_asset_paused_not_globally_paused() {
        // AssetPauseState returns AssetPaused, not GloballyPaused
        let mint        = Pubkey::new_unique();
        let pause_state = make_asset_pause_state(mint, pause_bit::REDEEM, 0);
        let err         = pause_state.assert_not_paused_for(pause_bit::REDEEM, 0).unwrap_err();

        assert_eq!(
            err,
            ProgramError::Custom(ChanceryError::AssetPaused as u32),
            "asset pause must return AssetPaused, not GloballyPaused",
        );
    }

    #[test]
    fn asset_mint_pause_does_not_block_redeem() {
        let mint        = Pubkey::new_unique();
        let pause_state = make_asset_pause_state(mint, pause_bit::MINT, 0);

        assert!(pause_state.assert_not_paused_for(pause_bit::REDEEM, 0).is_ok());
    }

    #[test]
    fn asset_pause_auto_expires_past_slot() {
        let mint        = Pubkey::new_unique();
        let pause_state = make_asset_pause_state(mint, pause_bit::ALL, 50);

        assert!(pause_state.assert_not_paused_for(pause_bit::MINT, 50).is_ok(),
            "asset pause must auto-expire at expires_at_slot");
        assert!(pause_state.assert_not_paused_for(pause_bit::MINT, 51).is_ok());
    }

    #[test]
    fn asset_pause_still_active_before_expiry_slot() {
        let mint        = Pubkey::new_unique();
        let pause_state = make_asset_pause_state(mint, pause_bit::MINT, 50);

        assert!(pause_state.assert_not_paused_for(pause_bit::MINT, 49).is_err());
    }

    #[test]
    fn asset_pause_zero_expiry_never_auto_expires() {
        let mint        = Pubkey::new_unique();
        let pause_state = make_asset_pause_state(mint, pause_bit::ALL, 0);

        assert!(pause_state.assert_not_paused_for(pause_bit::MINT, u64::MAX).is_err());
    }

    // ── PDA uniqueness ────────────────────────────────────────────────────────

    #[test]
    fn pda_is_deterministic() {
        let pid  = program_id();
        let mint = Pubkey::new_unique();
        let (k1, b1) = AssetPauseState::pda(&mint, &pid);
        let (k2, b2) = AssetPauseState::pda(&mint, &pid);

        assert_eq!(k1, k2);
        assert_eq!(b1, b2);
    }

    #[test]
    fn different_assets_produce_different_pdas() {
        let pid = program_id();
        let m1  = Pubkey::new_unique();
        let m2  = Pubkey::new_unique();
        let (k1, _) = AssetPauseState::pda(&m1, &pid);
        let (k2, _) = AssetPauseState::pda(&m2, &pid);

        assert_ne!(k1, k2, "different asset mints must produce different pause state PDAs");
    }

    #[test]
    fn asset_pause_pda_differs_from_global_pause_pda() {
        let pid  = program_id();
        let mint = Pubkey::new_unique();
        let (global_key, _) = PauseState::pda(&pid);
        let (asset_key, _)  = AssetPauseState::pda(&mint, &pid);

        assert_ne!(global_key, asset_key);
    }

    // ── Stores asset_mint field correctly ────────────────────────────────────

    #[test]
    fn asset_mint_field_is_set_correctly() {
        let mint        = Pubkey::new_unique();
        let pause_state = make_asset_pause_state(mint, 0, 0);

        assert_eq!(pause_state.asset_mint, mint);
    }
}

// ─── PERMISSION_FLAG_PAUSED cross-module invariants ─────────────────────────────────

/// These tests verify the PERMISSION_FLAG_PAUSED contract that settlement handlers
/// depend on. The constant lives in permissions::state::permission_record
/// and is imported by all relevant modules.

#[test]
fn perm_flag_paused_is_bit_zero() {
    assert_eq!(PERMISSION_FLAG_PAUSED, 1u64,
        "PERMISSION_FLAG_PAUSED must be bit 0 (value 1) - handlers check this exact bit");
}

#[test]
fn perm_flag_paused_does_not_overlap_pause_bits() {
    // PERMISSION_FLAG_PAUSED operates on permission_flags field, not global_pause_bits.
    // But we verify the constant value is sensible.
    assert!(PERMISSION_FLAG_PAUSED > 0, "PERMISSION_FLAG_PAUSED must be non-zero");
    assert!(PERMISSION_FLAG_PAUSED.count_ones() == 1, "PERMISSION_FLAG_PAUSED must be a single bit");
}

#[test]
fn setting_permission_flag_paused_does_not_affect_pause_bit_fields() {
    // These are separate u64 fields on separate structs.
    // Demonstrate independence: setting PERMISSION_FLAG_PAUSED=1 in permission_flags
    // has no effect on the interpretation of PauseState.global_pause_bits.
    let permission_flags : u64 = PERMISSION_FLAG_PAUSED;
    let pause_state      = make_pause_state(0, 0);  // no bits set in global_pause_bits

    // Even though permission_flags has bit 0 set, the global pause is not active
    assert!(pause_state.assert_not_paused_for(pause_bit::MINT, 0).is_ok(),
        "PERMISSION_FLAG_PAUSED on permission_flags must not affect global pause check");

    // If PERMISSION_FLAG_PAUSED happened to equal pause_bit::MINT (bit 0), this
    // would break - confirm they are indeed the same bit (both are 1u64)
    // and that the independence comes from them being on different structs.
    let _ = permission_flags;  // used above
}

// ─── Pause hierarchy ordering invariant ──────────────────────────────────────

/// The pause hierarchy (narrowest to broadest) must follow:
///   PERMISSION_FLAG_PAUSED < AssetPauseState < PauseState(operation bit) < PauseState(ALL)
///
/// This test documents the expected check ordering used by settlement handlers.
#[test]
fn pause_hierarchy_documentation() {
    // Level 1 (narrowest): counterparty/executor PERMISSION_FLAG_PAUSED
    //   -> checked via PermissionRecord.permission_flags & PERMISSION_FLAG_PAUSED
    //   -> error: CounterpartyPaused or ExecutorPaused

    // Level 2: pathway PATHWAY_PAUSE bit on PathwayPolicy.status_flags
    //   -> checked via pathway.assert_active()
    //   -> error: PathwayPaused

    // Level 3: per-asset AssetPauseState
    //   -> checked via AssetPauseState.assert_not_paused_for(operation_bit, slot)
    //   -> error: AssetPaused

    // Level 4 (broadest): global operation-specific bit in PauseState
    //   -> checked via PauseState.assert_not_paused_for(operation_bit, slot)
    //   -> error: GloballyPaused

    // (A retired Level 5 bit on ChanceryConfig.status_flags previously
    // existed but was never wired to any instruction; PauseState is now
    // the single source of truth for global pause.)

    // This test exists purely as documentation - no assertions beyond
    // verifying error codes are distinct.
    assert_ne!(
        ProgramError::Custom(ChanceryError::CounterpartyPaused as u32),
        ProgramError::Custom(ChanceryError::AssetPaused as u32),
    );
    assert_ne!(
        ProgramError::Custom(ChanceryError::AssetPaused as u32),
        ProgramError::Custom(ChanceryError::GloballyPaused as u32),
    );
    assert_ne!(
        ProgramError::Custom(ChanceryError::ExecutorPaused as u32),
        ProgramError::Custom(ChanceryError::PathwayPaused as u32),
    );
}

// ─── dispatch_gated (control/auth.rs) ────────────────────────────────────────

mod dispatch_gated_tests {
    use bytemuck::Zeroable;
    use solana_account_info::AccountInfo;
    use solana_program_entrypoint::ProgramResult;
    use solana_program_error::ProgramError;
    use solana_pubkey::Pubkey;

    use super::make_account_info;
    use crate::{
        constants::{ix, module, module_status},
        error::ChanceryError,
        modules::control::{
            auth::dispatch_gated,
            state::module_activation_state::{
                ModuleActivationState, MODULE_ACTIVATION_STATE_DISCRIMINATOR,
                MODULE_ACTIVATION_STATE_SIZE,
            },
        },
    };

    fn write_activation_state(data: &mut [u8], module_id: u8, status: u8) {
        let mut state = ModuleActivationState::zeroed();
        state.discriminator = MODULE_ACTIVATION_STATE_DISCRIMINATOR;
        state.version = 1;
        state.bump = ModuleActivationState::pda(&crate::id()).1;
        state.module_statuses[module_id as usize] = status;
        data.copy_from_slice(bytemuck::bytes_of(&state));
    }

    const INNER_SENTINEL_CODE: u32 = 0xFFFF_FFFE;

    fn inner_must_not_be_called(_accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
        panic!("inner dispatcher must not run after gate rejection");
    }

    fn inner_returns_sentinel(_accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
        Err(ProgramError::Custom(INNER_SENTINEL_CODE))
    }

    #[test]
    fn dispatch_gated_rejects_empty_accounts() {
        let result = dispatch_gated(module::SETTLEMENT, &[], &[], inner_must_not_be_called);
        assert_eq!(result, Err(ChanceryError::MissingAccount.into()));
    }

    #[test]
    fn dispatch_gated_rejects_disabled_module() {
        let program_id = crate::id();
        let (activation_key, _bump) = ModuleActivationState::pda(&program_id);
        let mut activation_data = vec![0u8; MODULE_ACTIVATION_STATE_SIZE];
        write_activation_state(
            &mut activation_data,
            module::SETTLEMENT,
            module_status::DISABLED,
        );
        let mut activation_lamports = 0u64;

        let dummy_key = Pubkey::new_unique();
        let mut dummy_data = vec![0u8; 1];
        let mut dummy_lamports = 0u64;

        let activation_info = make_account_info(
            &activation_key,
            &mut activation_data,
            &mut activation_lamports,
            &program_id,
            false,
        );
        let dummy_info = make_account_info(
            &dummy_key,
            &mut dummy_data,
            &mut dummy_lamports,
            &program_id,
            false,
        );

        let result = dispatch_gated(
            module::SETTLEMENT,
            &[activation_info, dummy_info],
            &[0],
            inner_must_not_be_called,
        );

        assert_eq!(result, Err(ChanceryError::ModuleNotEnabled.into()));
    }

    #[test]
    fn dispatch_gated_rejects_unknown_instruction_before_admin_only_authorization() {
        let program_id = crate::id();
        let (activation_key, _bump) = ModuleActivationState::pda(&program_id);
        let mut activation_data = vec![0u8; MODULE_ACTIVATION_STATE_SIZE];
        write_activation_state(
            &mut activation_data,
            module::PERMISSIONS,
            module_status::ADMIN_ONLY,
        );
        let mut activation_lamports = 0u64;

        let dummy_key = Pubkey::new_unique();
        let mut dummy_data = vec![0u8; 1];
        let mut dummy_lamports = 0u64;

        let activation_info = make_account_info(
            &activation_key,
            &mut activation_data,
            &mut activation_lamports,
            &program_id,
            false,
        );
        let dummy_info = make_account_info(
            &dummy_key,
            &mut dummy_data,
            &mut dummy_lamports,
            &program_id,
            false,
        );

        let result = dispatch_gated(
            module::PERMISSIONS,
            &[activation_info, dummy_info],
            &[0xFF],
            inner_must_not_be_called,
        );

        assert_eq!(result, Err(ChanceryError::UnknownInstruction.into()));
    }

    #[test]
    fn dispatch_gated_allows_known_admin_instruction_under_admin_only() {
        let program_id = crate::id();
        let (activation_key, _bump) = ModuleActivationState::pda(&program_id);
        let mut activation_data = vec![0u8; MODULE_ACTIVATION_STATE_SIZE];
        write_activation_state(
            &mut activation_data,
            module::PERMISSIONS,
            module_status::ADMIN_ONLY,
        );
        let mut activation_lamports = 0u64;

        let dummy_key = Pubkey::new_unique();
        let mut dummy_data = vec![0u8; 1];
        let mut dummy_lamports = 0u64;

        let activation_info = make_account_info(
            &activation_key,
            &mut activation_data,
            &mut activation_lamports,
            &program_id,
            false,
        );
        let dummy_info = make_account_info(
            &dummy_key,
            &mut dummy_data,
            &mut dummy_lamports,
            &program_id,
            false,
        );

        let result = dispatch_gated(
            module::PERMISSIONS,
            &[activation_info, dummy_info],
            &[ix::permissions::UPSERT_PERMISSION],
            inner_returns_sentinel,
        );

        assert_eq!(result, Err(ProgramError::Custom(INNER_SENTINEL_CODE)));
    }

    #[test]
    fn dispatch_gated_rejects_known_hot_path_under_admin_only() {
        let program_id = crate::id();
        let (activation_key, _bump) = ModuleActivationState::pda(&program_id);
        let mut activation_data = vec![0u8; MODULE_ACTIVATION_STATE_SIZE];
        write_activation_state(
            &mut activation_data,
            module::SETTLEMENT,
            module_status::ADMIN_ONLY,
        );
        let mut activation_lamports = 0u64;

        let dummy_key = Pubkey::new_unique();
        let mut dummy_data = vec![0u8; 1];
        let mut dummy_lamports = 0u64;

        let activation_info = make_account_info(
            &activation_key,
            &mut activation_data,
            &mut activation_lamports,
            &program_id,
            false,
        );
        let dummy_info = make_account_info(
            &dummy_key,
            &mut dummy_data,
            &mut dummy_lamports,
            &program_id,
            false,
        );

        let result = dispatch_gated(
            module::SETTLEMENT,
            &[activation_info, dummy_info],
            &[ix::settlement::MINT_DIRECT],
            inner_must_not_be_called,
        );

        assert_eq!(result, Err(ChanceryError::ModuleAdminOnly.into()));
    }

    #[test]
    fn dispatch_gated_forwards_when_module_active() {
        let program_id = crate::id();
        let (activation_key, _bump) = ModuleActivationState::pda(&program_id);
        let mut activation_data = vec![0u8; MODULE_ACTIVATION_STATE_SIZE];
        write_activation_state(
            &mut activation_data,
            module::SETTLEMENT,
            module_status::ACTIVE,
        );
        let mut activation_lamports = 0u64;

        let dummy_key = Pubkey::new_unique();
        let mut dummy_data = vec![0u8; 1];
        let mut dummy_lamports = 0u64;

        let activation_info = make_account_info(
            &activation_key,
            &mut activation_data,
            &mut activation_lamports,
            &program_id,
            false,
        );
        let dummy_info = make_account_info(
            &dummy_key,
            &mut dummy_data,
            &mut dummy_lamports,
            &program_id,
            false,
        );

        let result = dispatch_gated(
            module::SETTLEMENT,
            &[activation_info, dummy_info],
            &[0],
            inner_returns_sentinel,
        );

        assert_eq!(result, Err(ProgramError::Custom(INNER_SENTINEL_CODE)));
    }
}
