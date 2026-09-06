/// modules/settlement/tests/mod.rs
///
/// Unit tests for SettlementIntent and SettlementPolicy state structs.
///
/// Covers:
///   SettlementIntent:
///     - size / layout
///     - load guards (uninitialised, wrong size, non-writable)
///     - double-init prevention
///     - assert_pending for every status variant
///     - temporal guards (valid_after, expires_at, combined)
///     - amounts_meet_minimums
///     - terminal status vocabulary (close-on-terminal lifecycle)
///     - PDA derivation determinism and uniqueness
///
///   SettlementPolicy:
///     - size / layout
///     - load guards
///     - assert_not_expired
///     - assert_settlement_mode_allowed bitmask logic
///     - assert_asset (zero = wildcard)
///     - assert_notional (min and max)
///     - PDA determinism

use std::{cell::RefCell, rc::Rc};

use bytemuck::Zeroable;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::{intent_status, seeds, settlement_mode},
    error::ChanceryError,
    modules::settlement::state::{
        settlement_intent::{
            SettlementIntent, SETTLEMENT_INTENT_DISCRIMINATOR, SETTLEMENT_INTENT_SIZE,
        },
        settlement_policy::{
            SettlementPolicy, SETTLEMENT_POLICY_DISCRIMINATOR, SETTLEMENT_POLICY_SIZE,
        },
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

/// Build a SettlementIntent with explicit control over every field.
fn make_intent(
    intent_id:           [u8; 32],
    status:              u8,
    mode:                u8,
    principal_a:         Pubkey,
    principal_b:         Pubkey,
    executor:            Pubkey,
    asset_mint:          Pubkey,
    asset_amount:        u64,
    issued_token_amount: u64,
    min_asset:           u64,
    min_issued_token:    u64,
    valid_after:         i64,
    expires_at:          i64,
) -> (Pubkey, Vec<u8>, u64) {
    let pid = program_id();
    let (key, bump) = Pubkey::find_program_address(
        &[seeds::SETTLEMENT_INTENT, intent_id.as_ref()],
        &pid,
    );
    let mut data     = vec![0u8; SETTLEMENT_INTENT_SIZE];
    let lamports = 1u64;

    {
        // direct mut access to data
        let i: &mut SettlementIntent = bytemuck::from_bytes_mut(&mut data[..]);

        i.discriminator               = SETTLEMENT_INTENT_DISCRIMINATOR;
        i.version                     = 1;
        i.bump                        = bump;
        i.status                      = status;
        i.settlement_mode             = mode;
        i.intent_id                   = intent_id;
        i.principal_a                 = principal_a;
        i.principal_b                 = principal_b;
        i.executor                    = executor;
        i.asset_mint                  = asset_mint;
        i.issued_token_mint           = Pubkey::new_unique();
        i.asset_amount                = asset_amount;
        i.issued_token_amount         = issued_token_amount;
        i.minimum_asset_amount        = min_asset;
        i.minimum_issued_token_amount = min_issued_token;
        i.nonce                       = 1;
        i.valid_after_unix_timestamp  = valid_after;
        i.expires_at_unix_timestamp   = expires_at;
        i.policy_id                   = [0u8; 32];
        i.intent_hash                 = [0u8; 32];
    }

    (key, data, lamports)
}

/// Short form for a pending intent with default time window.
fn pending_intent() -> (Pubkey, Vec<u8>, u64) {
    make_intent(
        [0x01u8; 32],
        intent_status::PENDING,
        settlement_mode::DIRECT_PRINCIPAL,
        Pubkey::new_unique(),
        Pubkey::new_unique(),
        Pubkey::default(),
        Pubkey::new_unique(),
        100_000,
        100_000,
        0,
        0,
        0,  // valid_after  = 0 -> always valid
        0,  // expires_at   = 0 -> never expires
    )
}

// ═══════════════════════════════════════════════════════════════════════════════
// SettlementIntent tests
// ═══════════════════════════════════════════════════════════════════════════════

mod settlement_intent_tests {
    use super::*;

    // ── Size / layout ─────────────────────────────────────────────────────────

    #[test]
    fn size_matches_declared() {
        assert_eq!(core::mem::size_of::<SettlementIntent>(), 424);
        assert_eq!(SETTLEMENT_INTENT_SIZE, 424);
        assert_eq!(core::mem::offset_of!(SettlementIntent, pathway_id), 328);
        assert_eq!(core::mem::offset_of!(SettlementIntent, rent_refund_recipient), 360);
        assert_eq!(core::mem::offset_of!(SettlementIntent, _reserved), 392);
    }

    #[test]
    fn zeroed_struct_all_zero_bytes() {
        let i            = SettlementIntent::zeroed();
        let bytes: &[u8] = bytemuck::bytes_of(&i);

        assert!(bytes.iter().all(|&b| b == 0));
        assert_eq!(i.rent_refund_recipient, Pubkey::default());
        assert!(i._reserved.iter().all(|&b| b == 0));
    }

    // ── Load guards ───────────────────────────────────────────────────────────

    #[test]
    fn load_zero_data_returns_not_initialized() {
        let pid          = program_id();
        let key          = Pubkey::new_unique();
        let mut data     = vec![0u8; SETTLEMENT_INTENT_SIZE];
        let mut lamports = 1u64;
        let ai           = make_account_info(&key, &mut data, &mut lamports, &pid, false);

        assert!(matches!(
            SettlementIntent::load(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::NotInitialized as u32),
        ));
    }

    #[test]
    fn load_wrong_size_returns_length_mismatch() {
        let pid          = program_id();
        let key          = Pubkey::new_unique();
        let mut data     = vec![0u8; SETTLEMENT_INTENT_SIZE + 1];
        let mut lamports = 1u64;
        let ai           = make_account_info(&key, &mut data, &mut lamports, &pid, false);

        assert!(matches!(
            SettlementIntent::load(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::AccountDataLengthMismatch as u32),
        ));
    }

    #[test]
    fn load_mut_non_writable_returns_error() {
        let (key, mut data, mut lamports) = pending_intent();
        let pid = program_id();
        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);

        assert!(matches!(
            SettlementIntent::load_mut(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::AccountNotWritable as u32),
        ));
    }

    #[test]
    fn load_uninitialized_blocks_double_init() {
        let pid          = program_id();
        let key          = Pubkey::new_unique();
        let mut data     = vec![0u8; SETTLEMENT_INTENT_SIZE];
        let mut lamports = 1u64;

        {
            let ai = make_account_info(&key, &mut data, &mut lamports, &pid, true);
            let mut i  = SettlementIntent::load_uninitialized_mut(&ai).unwrap();

            i.discriminator = SETTLEMENT_INTENT_DISCRIMINATOR;
        }

        {
            let ai = make_account_info(&key, &mut data, &mut lamports, &pid, true);

            assert!(matches!(
                SettlementIntent::load_uninitialized_mut(&ai),
                Err(e) if e == ProgramError::Custom(ChanceryError::AlreadyInitialized as u32),
            ));
        }
    }

    #[test]
    fn load_reads_fields_correctly() {
        let pa   = Pubkey::new_unique();
        let pb   = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let id   = [0xAAu8; 32];
        let (key, mut data, mut lamports) = make_intent(
            id, intent_status::PENDING, settlement_mode::DELEGATED_NON_CUSTODIAL,
            pa, pb, Pubkey::new_unique(), mint,
            500_000, 500_000, 100_000, 50_000,
            1_000, 9_000,
        );
        let pid  = program_id();
        let ai   = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let i    = SettlementIntent::load(&ai).unwrap();

        assert_eq!(i.intent_id,                   id);
        assert_eq!(i.status,                      intent_status::PENDING);
        assert_eq!(i.settlement_mode,             settlement_mode::DELEGATED_NON_CUSTODIAL);
        assert_eq!(i.principal_a,                 pa);
        assert_eq!(i.principal_b,                 pb);
        assert_eq!(i.asset_mint,                  mint);
        assert_eq!(i.asset_amount,                500_000);
        assert_eq!(i.issued_token_amount,         500_000);
        assert_eq!(i.minimum_asset_amount,        100_000);
        assert_eq!(i.minimum_issued_token_amount, 50_000);
        assert_eq!(i.valid_after_unix_timestamp,  1_000);
        assert_eq!(i.expires_at_unix_timestamp,   9_000);
    }

    // ── assert_pending ────────────────────────────────────────────────────────

    #[test]
    fn assert_pending_passes_on_pending_status() {
        let (key, mut data, mut lamports) = pending_intent();
        let pid = program_id();
        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let i   = SettlementIntent::load(&ai).unwrap();

        assert!(i.assert_pending().is_ok());
    }

    #[test]
    fn assert_pending_fails_on_executed() {
        let (key, mut data, mut lamports) = make_intent(
            [1u8; 32], intent_status::EXECUTED, settlement_mode::DIRECT_PRINCIPAL,
            Pubkey::new_unique(), Pubkey::new_unique(), Pubkey::default(),
            Pubkey::new_unique(), 0, 0, 0, 0, 0, 0,
        );
        let pid = program_id();
        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let i   = SettlementIntent::load(&ai).unwrap();

        assert!(matches!(
            i.assert_pending(),
            Err(e) if e == ProgramError::Custom(ChanceryError::IntentAlreadyExecuted as u32),
        ));
    }

    #[test]
    fn assert_pending_fails_on_expired_status() {
        let (key, mut data, mut lamports) = make_intent(
            [2u8; 32], intent_status::EXPIRED, settlement_mode::DIRECT_PRINCIPAL,
            Pubkey::new_unique(), Pubkey::new_unique(), Pubkey::default(),
            Pubkey::new_unique(), 0, 0, 0, 0, 0, 0,
        );
        let pid = program_id();
        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let i   = SettlementIntent::load(&ai).unwrap();

        assert!(matches!(
            i.assert_pending(),
            Err(e) if e == ProgramError::Custom(ChanceryError::IntentExpired as u32),
        ));
    }

    #[test]
    fn assert_pending_fails_on_cancelled() {
        let (key, mut data, mut lamports) = make_intent(
            [3u8; 32], intent_status::CANCELLED, settlement_mode::DIRECT_PRINCIPAL,
            Pubkey::new_unique(), Pubkey::new_unique(), Pubkey::default(),
            Pubkey::new_unique(), 0, 0, 0, 0, 0, 0,
        );
        let pid = program_id();
        let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
        let i   = SettlementIntent::load(&ai).unwrap();

        // CANCELLED maps to IntentExpired in the handler
        assert!(i.assert_pending().is_err());
    }

    // ── Temporal guards ───────────────────────────────────────────────────────

    #[test]
    fn assert_not_expired_passes_before_expiry() {
        let mut i = SettlementIntent::zeroed();

        i.expires_at_unix_timestamp = 1_000_000;

        assert!(i.assert_not_expired(999_999).is_ok());
    }

    #[test]
    fn assert_not_expired_fails_at_exact_expiry() {
        let mut i = SettlementIntent::zeroed();

        i.expires_at_unix_timestamp = 1_000_000;

        assert!(matches!(
            i.assert_not_expired(1_000_000),
            Err(e) if e == ProgramError::Custom(ChanceryError::IntentExpired as u32),
        ));
    }

    #[test]
    fn assert_not_expired_fails_past_expiry() {
        let mut i = SettlementIntent::zeroed();

        i.expires_at_unix_timestamp = 500;

        assert!(i.assert_not_expired(501).is_err());
    }

    #[test]
    fn assert_not_expired_zero_means_never_expires() {
        let mut i = SettlementIntent::zeroed();

        i.expires_at_unix_timestamp = 0;

        assert!(i.assert_not_expired(i64::MAX).is_ok());
    }

    #[test]
    fn assert_valid_after_passes_at_valid_after_timestamp() {
        let mut i = SettlementIntent::zeroed();

        i.valid_after_unix_timestamp = 1_000;

        assert!(i.assert_valid_after(1_000).is_ok());
    }

    #[test]
    fn assert_valid_after_fails_before_valid_after_timestamp() {
        let mut i = SettlementIntent::zeroed();

        i.valid_after_unix_timestamp = 1_000;

        assert!(matches!(
            i.assert_valid_after(999),
            Err(e) if e == ProgramError::Custom(ChanceryError::IntentNotYetValid as u32),
        ));
    }

    #[test]
    fn assert_valid_after_zero_means_always_valid() {
        let mut i = SettlementIntent::zeroed();

        i.valid_after_unix_timestamp = 0;

        assert!(i.assert_valid_after(0).is_ok());
        assert!(i.assert_valid_after(-1).is_ok());
    }

    #[test]
    fn assert_temporally_valid_passes_within_window() {
        let mut i = SettlementIntent::zeroed();

        i.valid_after_unix_timestamp = 1_000;
        i.expires_at_unix_timestamp  = 2_000;

        assert!(i.assert_temporally_valid(1_500).is_ok());
    }

    #[test]
    fn assert_temporally_valid_fails_before_window_opens() {
        let mut i = SettlementIntent::zeroed();

        i.valid_after_unix_timestamp = 1_000;
        i.expires_at_unix_timestamp  = 2_000;

        assert!(i.assert_temporally_valid(999).is_err());
    }

    #[test]
    fn assert_temporally_valid_fails_after_window_closes() {
        let mut i = SettlementIntent::zeroed();

        i.valid_after_unix_timestamp = 1_000;
        i.expires_at_unix_timestamp  = 2_000;

        assert!(i.assert_temporally_valid(2_000).is_err());
    }

    #[test]
    fn assert_temporally_valid_at_open_boundary_passes() {
        let mut i = SettlementIntent::zeroed();

        i.valid_after_unix_timestamp = 1_000;
        i.expires_at_unix_timestamp  = 2_000;

        assert!(i.assert_temporally_valid(1_000).is_ok());
    }

    #[test]
    fn assert_temporally_valid_zero_timestamps_always_valid() {
        let mut i = SettlementIntent::zeroed();

        i.valid_after_unix_timestamp = 0;
        i.expires_at_unix_timestamp  = 0;

        assert!(i.assert_temporally_valid(i64::MAX).is_ok());
        assert!(i.assert_temporally_valid(0).is_ok());
        assert!(i.assert_temporally_valid(-1).is_ok());
    }

    // ── Committed action-specific minimums ───────────────────────────────────

    #[test]
    fn mint_intent_committed_issued_token_minimum_accepts_exact_output() {
        let mut intent = SettlementIntent::zeroed();

        intent.minimum_issued_token_amount = 50_000;

        assert!(crate::modules::settlement::assert_output_meets_minimum(
            50_000,
            intent.minimum_issued_token_amount,
            ChanceryError::IntentAmountBelowMinimum,
        )
        .is_ok());
    }

    #[test]
    fn mint_intent_committed_issued_token_minimum_rejects_lower_output() {
        let mut intent = SettlementIntent::zeroed();

        intent.minimum_issued_token_amount = 50_000;

        assert!(matches!(
            crate::modules::settlement::assert_output_meets_minimum(
                49_999,
                intent.minimum_issued_token_amount,
                ChanceryError::IntentAmountBelowMinimum,
            ),
            Err(error)
                if error
                    == ProgramError::Custom(
                        ChanceryError::IntentAmountBelowMinimum as u32,
                    ),
        ));
    }

    #[test]
    fn redeem_intent_committed_asset_minimum_rejects_lower_received_amount() {
        let mut intent = SettlementIntent::zeroed();

        intent.minimum_asset_amount = 100_000;

        assert!(matches!(
            crate::modules::settlement::assert_output_meets_minimum(
                99_999,
                intent.minimum_asset_amount,
                ChanceryError::IntentAmountBelowMinimum,
            ),
            Err(error)
                if error
                    == ProgramError::Custom(
                        ChanceryError::IntentAmountBelowMinimum as u32,
                    ),
        ));
    }

    #[test]
    fn zero_committed_minimum_disables_the_floor() {
        let intent = SettlementIntent::zeroed();

        assert!(crate::modules::settlement::assert_output_meets_minimum(
            0,
            intent.minimum_issued_token_amount,
            ChanceryError::IntentAmountBelowMinimum,
        )
        .is_ok());
    }

    // ── Status transitions ────────────────────────────────────────────────────

    // The mark_executed/mark_expired setters are retired: terminal
    // transitions close the account. The status vocabulary itself stays
    // pinned because evidence and clients speak it.
    #[test]
    fn terminal_status_vocabulary_is_pinned() {
        assert_eq!(intent_status::PENDING,   0);
        assert_eq!(intent_status::EXECUTED,  1);
        assert_eq!(intent_status::CANCELLED, 2);
        assert_eq!(intent_status::EXPIRED,   3);
    }

    #[test]
    fn executed_status_still_fails_assert_pending() {
        let mut i = SettlementIntent::zeroed();

        i.discriminator = SETTLEMENT_INTENT_DISCRIMINATOR;
        i.status        = intent_status::EXECUTED;

        assert!(matches!(
            i.assert_pending(),
            Err(e) if e == ProgramError::Custom(ChanceryError::IntentAlreadyExecuted as u32),
        ));
    }

    #[test]
    fn executed_is_terminal_assert_pending_never_passes() {
        let mut i = SettlementIntent::zeroed();

        i.discriminator = SETTLEMENT_INTENT_DISCRIMINATOR;

        for status in [intent_status::EXECUTED, intent_status::EXPIRED, intent_status::CANCELLED] {
            i.status = status;

            assert!(i.assert_pending().is_err(),
                "status={status} must not pass assert_pending");
        }
    }

    // ── PDA derivation ────────────────────────────────────────────────────────

    #[test]
    fn pda_is_deterministic() {
        let pid      = program_id();
        let id       = [0xBBu8; 32];
        let (k1, b1) = SettlementIntent::pda(&id, &pid);
        let (k2, b2) = SettlementIntent::pda(&id, &pid);

        assert_eq!(k1, k2);
        assert_eq!(b1, b2);
    }

    #[test]
    fn different_intent_ids_produce_different_pdas() {
        let pid     = program_id();
        let (k1, _) = SettlementIntent::pda(&[0x01u8; 32], &pid);
        let (k2, _) = SettlementIntent::pda(&[0x02u8; 32], &pid);

        assert_ne!(k1, k2);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SettlementPolicy tests
// ═══════════════════════════════════════════════════════════════════════════════

mod settlement_policy_tests {
    use super::*;

    // ── Size ──────────────────────────────────────────────────────────────────

    #[test]
    fn size_matches_declared() {
        assert_eq!(core::mem::size_of::<SettlementPolicy>(), 288);
        assert_eq!(SETTLEMENT_POLICY_SIZE, 288);
        assert_eq!(core::mem::offset_of!(SettlementPolicy, designated_executor), 160);
        assert_eq!(core::mem::offset_of!(SettlementPolicy, max_notional), 192);
        assert_eq!(core::mem::offset_of!(SettlementPolicy, created_by), 224);
        assert_eq!(core::mem::offset_of!(SettlementPolicy, _reserved), 256);
        assert_eq!(core::mem::size_of_val(&SettlementPolicy::zeroed()._reserved), 32);
    }

    #[test]
    fn zeroed_struct_all_zero_bytes() {
        let p            = SettlementPolicy::zeroed();
        let bytes: &[u8] = bytemuck::bytes_of(&p);

        assert!(bytes.iter().all(|&b| b == 0));
    }

    // ── Load guards ───────────────────────────────────────────────────────────

    #[test]
    fn load_zero_data_returns_not_initialized() {
        let pid          = program_id();
        let key          = Pubkey::new_unique();
        let mut data     = vec![0u8; SETTLEMENT_POLICY_SIZE];
        let mut lamports = 1u64;
        let ai           = make_account_info(&key, &mut data, &mut lamports, &pid, false);

        assert!(matches!(
            SettlementPolicy::load(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::NotInitialized as u32),
        ));
    }

    #[test]
    fn load_wrong_size_returns_length_mismatch() {
        let pid          = program_id();
        let key          = Pubkey::new_unique();
        let mut data     = vec![0u8; SETTLEMENT_POLICY_SIZE - 8];
        let mut lamports = 1u64;
        let ai           = make_account_info(&key, &mut data, &mut lamports, &pid, false);

        assert!(matches!(
            SettlementPolicy::load(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::AccountDataLengthMismatch as u32),
        ));
    }

    #[test]
    fn pre_release_320_byte_layout_is_rejected() {
        let pid          = program_id();
        let key          = Pubkey::new_unique();
        let mut data     = vec![0u8; 320];
        let mut lamports = 1u64;
        let ai           = make_account_info(&key, &mut data, &mut lamports, &pid, false);

        assert!(matches!(
            SettlementPolicy::load(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::AccountDataLengthMismatch as u32),
        ));
    }

    #[test]
    fn load_uninitialized_blocks_double_init() {
        let pid          = program_id();
        let key          = Pubkey::new_unique();
        let mut data     = vec![0u8; SETTLEMENT_POLICY_SIZE];
        let mut lamports = 1u64;

        {
            let ai = make_account_info(&key, &mut data, &mut lamports, &pid, true);
            let mut p  = SettlementPolicy::load_uninitialized_mut(&ai).unwrap();

            p.discriminator = SETTLEMENT_POLICY_DISCRIMINATOR;
        }

        {
            let ai = make_account_info(&key, &mut data, &mut lamports, &pid, true);

            assert!(matches!(
                SettlementPolicy::load_uninitialized_mut(&ai),
                Err(e) if e == ProgramError::Custom(ChanceryError::AlreadyInitialized as u32),
            ));
        }
    }

    // ── assert_not_expired ────────────────────────────────────────────────────

    #[test]
    fn policy_not_expired_when_expires_at_zero() {
        let mut p = SettlementPolicy::zeroed();

        p.expires_at_unix_timestamp = 0;

        assert!(p.assert_not_expired(i64::MAX).is_ok());
    }

    #[test]
    fn policy_not_expired_before_expiry() {
        let mut p = SettlementPolicy::zeroed();

        p.expires_at_unix_timestamp = 5_000;

        assert!(p.assert_not_expired(4_999).is_ok());
    }

    #[test]
    fn policy_expired_at_exact_expiry_timestamp() {
        let mut p = SettlementPolicy::zeroed();

        p.expires_at_unix_timestamp = 5_000;

        assert!(matches!(
            p.assert_not_expired(5_000),
            Err(e) if e == ProgramError::Custom(ChanceryError::SettlementPolicyExpired as u32),
        ));
    }

    #[test]
    fn policy_expired_past_expiry() {
        let mut p = SettlementPolicy::zeroed();

        p.expires_at_unix_timestamp = 1;

        assert!(p.assert_not_expired(2).is_err());
    }

    // ── assert_settlement_mode_allowed ────────────────────────────────────────

    #[test]
    fn intent_backed_modes_are_allowed_when_bits_are_set() {
        let mut p = SettlementPolicy::zeroed();

        p.allowed_settlement_modes =
            (1u32 << settlement_mode::DELEGATED_NON_CUSTODIAL)
                | (1u32 << settlement_mode::TRILATERAL_ATOMIC);

        assert!(p
            .assert_settlement_mode_allowed(settlement_mode::DELEGATED_NON_CUSTODIAL)
            .is_ok());
        assert!(p
            .assert_settlement_mode_allowed(settlement_mode::TRILATERAL_ATOMIC)
            .is_ok());
    }

    #[test]
    fn mode_is_blocked_when_its_bit_is_not_set() {
        let mut p = SettlementPolicy::zeroed();

        p.allowed_settlement_modes =
            1u32 << settlement_mode::DELEGATED_NON_CUSTODIAL;

        assert!(matches!(
            p.assert_settlement_mode_allowed(settlement_mode::TRILATERAL_ATOMIC),
            Err(e) if e == ProgramError::Custom(ChanceryError::SettlementModeNotAllowed as u32),
        ));
    }

    #[test]
    fn direct_mode_bit_is_reserved_even_when_present() {
        let mut p = SettlementPolicy::zeroed();

        p.allowed_settlement_modes = u32::MAX;

        assert!(p
            .assert_settlement_mode_allowed(settlement_mode::DIRECT_PRINCIPAL)
            .is_err());
        assert!(p.assert_parameter_sanity().is_err());
    }

    #[test]
    fn no_modes_allowed_when_zero_bits() {
        let mut p = SettlementPolicy::zeroed();

        p.allowed_settlement_modes = 0;

        assert!(p
            .assert_settlement_mode_allowed(settlement_mode::DELEGATED_NON_CUSTODIAL)
            .is_err());
    }

    // ── assert_asset ──────────────────────────────────────────────────────────

    #[test]
    fn asset_wildcard_zero_accepts_any_mint() {
        let mut p = SettlementPolicy::zeroed();

        p.allowed_asset_mint = Pubkey::default();  // zero = wildcard

        assert!(p.assert_asset(&Pubkey::new_unique()).is_ok());
        assert!(p.assert_asset(&Pubkey::new_unique()).is_ok());
    }

    #[test]
    fn asset_specific_accepts_matching_mint() {
        let mint  = Pubkey::new_unique();
        let mut p = SettlementPolicy::zeroed();

        p.allowed_asset_mint = mint;

        assert!(p.assert_asset(&mint).is_ok());
    }

    #[test]
    fn asset_specific_rejects_non_matching_mint() {
        let allowed = Pubkey::new_unique();
        let other   = Pubkey::new_unique();
        let mut p   = SettlementPolicy::zeroed();

        p.allowed_asset_mint = allowed;

        // AssetNotRegistered is the error when asset doesn't match
        assert!(p.assert_asset(&other).is_err());
    }

    // ── assert_notional ───────────────────────────────────────────────────────

    #[test]
    fn notional_within_range_passes() {
        let mut p = SettlementPolicy::zeroed();

        p.min_notional = 1_000;
        p.max_notional = 1_000_000;

        assert!(p.assert_notional(500_000).is_ok());
    }

    #[test]
    fn notional_at_minimum_passes() {
        let mut p = SettlementPolicy::zeroed();

        p.min_notional = 1_000;
        p.max_notional = 1_000_000;

        assert!(p.assert_notional(1_000).is_ok());
    }

    #[test]
    fn notional_at_maxmimum_passes() {
        let mut p = SettlementPolicy::zeroed();

        p.min_notional = 0;
        p.max_notional = 1_000_000;

        assert!(p.assert_notional(1_000_000).is_ok());
    }

    #[test]
    fn notional_below_minimum_fails() {
        let mut p = SettlementPolicy::zeroed();

        p.min_notional = 1_000;
        p.max_notional = 1_000_000;

        assert!(matches!(
            p.assert_notional(999),
            Err(e) if e == ProgramError::Custom(ChanceryError::AmountBelowMinimum as u32),
        ));
    }

    #[test]
    fn notional_above_maxmimum_fails() {
        let mut p = SettlementPolicy::zeroed();

        p.min_notional = 0;
        p.max_notional = 1_000_000;

        assert!(matches!(
            p.assert_notional(1_000_001),
            Err(e) if e == ProgramError::Custom(ChanceryError::AmountExceedsMaximum as u32),
        ));
    }

    #[test]
    fn zero_maxmimum_notional_means_no_cap() {
        let mut p = SettlementPolicy::zeroed();

        p.min_notional = 0;
        p.max_notional = 0;  // 0 = disabled

        assert!(p.assert_notional(u64::MAX).is_ok());
    }

    #[test]
    fn zero_minimum_notional_passes_for_any_amount() {
        let mut p = SettlementPolicy::zeroed();

        p.min_notional = 0;
        p.max_notional = 0;

        assert!(p.assert_notional(0).is_ok());
        assert!(p.assert_notional(1).is_ok());
    }

    // ── PDA derivation ────────────────────────────────────────────────────────

    #[test]
    fn pda_is_deterministic() {
        let pid      = program_id();
        let id       = [0xCCu8; 32];
        let (k1, b1) = SettlementPolicy::pda(&id, &pid);
        let (k2, b2) = SettlementPolicy::pda(&id, &pid);

        assert_eq!(k1, k2);
        assert_eq!(b1, b2);
    }

    #[test]
    fn different_policy_ids_produce_different_pdas() {
        let pid     = program_id();
        let (k1, _) = SettlementPolicy::pda(&[0x01u8; 32], &pid);
        let (k2, _) = SettlementPolicy::pda(&[0x02u8; 32], &pid);

        assert_ne!(k1, k2);
    }
}

// ─── Cross-struct invariant tests ─────────────────────────────────────────────

/// Settlement mode constants retain stable wire ordinals. Bit 0 is reserved
/// in SettlementPolicy because direct settlement is intentionally policy-free.
#[test]
fn settlement_mode_constants_are_sequential_bits() {
    // bit 0 = DIRECT_PRINCIPAL (reserved in SettlementPolicy)
    assert_eq!(settlement_mode::DIRECT_PRINCIPAL,        0u8);

    // bit 1 = DELEGATED_NON_CUSTODIAL
    assert_eq!(settlement_mode::DELEGATED_NON_CUSTODIAL, 1u8);

    // bit 2 = TRILATERAL_ATOMIC
    assert_eq!(settlement_mode::TRILATERAL_ATOMIC,       2u8);
}

/// Intent status constants are distinct non-overlapping values.
#[test]
fn intent_status_constants_are_distinct() {
    let statuses = [
        intent_status::PENDING,
        intent_status::EXECUTED,
        intent_status::EXPIRED,
        intent_status::CANCELLED,
    ];
    let unique: std::collections::HashSet<u8> = statuses.iter().cloned().collect();

    assert_eq!(unique.len(), 4, "All intent_status constants must be distinct");
}

/// An intent that is valid_after=now and expires_at=now+1 is only usable
/// for exactly one second - assert the boundary behaviour is correct.
#[test]
fn tight_temporal_window_boundary_behaviour() {
    let mut i = SettlementIntent::zeroed();

    i.valid_after_unix_timestamp = 1_000;
    i.expires_at_unix_timestamp  = 1_001;

    assert!(i.assert_temporally_valid(1_000).is_ok(),  "open boundary must pass");
    assert!(i.assert_temporally_valid(1_001).is_err(), "close boundary must fail");
    assert!(i.assert_temporally_valid(999).is_err(),   "before open must fail");
}

// ─── SPL token account binding helper (#39) ───────────────────────────────────

fn make_token_account_data(mint: &Pubkey, owner: &Pubkey) -> Vec<u8> {
    let mut data = vec![0u8; 165];

    data[0..32].copy_from_slice(mint.as_ref());
    data[32..64].copy_from_slice(owner.as_ref());

    data
}

#[test]
fn spl_token_account_binding_accepts_matching_mint_and_owner() {
    use crate::modules::settlement::instructions::assert_spl_token_account_binding;

    let mint   = Pubkey::new_unique();
    let owner  = Pubkey::new_unique();
    let key    = Pubkey::new_unique();
    let token_program = program_id();
    let token_program_owner = Pubkey::new_unique();
    let mut data = make_token_account_data(&mint, &owner);
    let mut lamports = 1u64;
    let mut token_program_data = Vec::new();
    let mut token_program_lamports = 1u64;
    let ai = make_account_info(&key, &mut data, &mut lamports, &token_program, true);
    let token_program_ai = make_account_info(
        &token_program,
        &mut token_program_data,
        &mut token_program_lamports,
        &token_program_owner,
        false,
    );

    assert!(assert_spl_token_account_binding(&ai, &token_program_ai, &mint, &owner).is_ok());
}

#[test]
fn spl_token_account_binding_rejects_wrong_owner() {
    use crate::modules::settlement::instructions::assert_spl_token_account_binding;

    let mint          = Pubkey::new_unique();
    let owner         = Pubkey::new_unique();
    let wrong_owner   = Pubkey::new_unique();
    let key           = Pubkey::new_unique();
    let token_program = program_id();
    let token_program_owner = Pubkey::new_unique();
    let mut data      = make_token_account_data(&mint, &owner);
    let mut lamports  = 1u64;
    let mut token_program_data = Vec::new();
    let mut token_program_lamports = 1u64;
    let ai            = make_account_info(&key, &mut data, &mut lamports, &token_program, true);
    let token_program_ai = make_account_info(
        &token_program,
        &mut token_program_data,
        &mut token_program_lamports,
        &token_program_owner,
        false,
    );

    assert!(matches!(
        assert_spl_token_account_binding(&ai, &token_program_ai, &mint, &wrong_owner),
        Err(e) if e == ProgramError::Custom(ChanceryError::AccountKeyMismatch as u32),
    ));
}

#[test]
fn spl_token_account_binding_rejects_short_account_data() {
    use crate::modules::settlement::instructions::assert_spl_token_account_binding;

    let mint         = Pubkey::new_unique();
    let owner        = Pubkey::new_unique();
    let key          = Pubkey::new_unique();
    let token_program = program_id();
    let token_program_owner = Pubkey::new_unique();
    let mut data     = vec![0u8; 32];
    let mut lamports = 1u64;
    let mut token_program_data = Vec::new();
    let mut token_program_lamports = 1u64;
    let ai           = make_account_info(&key, &mut data, &mut lamports, &token_program, true);
    let token_program_ai = make_account_info(
        &token_program,
        &mut token_program_data,
        &mut token_program_lamports,
        &token_program_owner,
        false,
    );

    assert!(matches!(
        assert_spl_token_account_binding(&ai, &token_program_ai, &mint, &owner),
        Err(e) if e == ProgramError::Custom(ChanceryError::AccountDataLengthMismatch as u32),
    ));
}


#[test]
fn spl_token_account_binding_rejects_wrong_token_program_owner() {
    use crate::modules::settlement::instructions::assert_spl_token_account_binding;

    let mint = Pubkey::new_unique();
    let owner = Pubkey::new_unique();
    let key = Pubkey::new_unique();
    let actual_token_program = Pubkey::new_unique();
    let expected_token_program = Pubkey::new_unique();
    let token_program_owner = Pubkey::new_unique();
    let mut data = make_token_account_data(&mint, &owner);
    let mut lamports = 1u64;
    let mut token_program_data = Vec::new();
    let mut token_program_lamports = 1u64;
    let ai = make_account_info(&key, &mut data, &mut lamports, &actual_token_program, true);
    let token_program_ai = make_account_info(
        &expected_token_program,
        &mut token_program_data,
        &mut token_program_lamports,
        &token_program_owner,
        false,
    );

    assert!(matches!(
        assert_spl_token_account_binding(&ai, &token_program_ai, &mint, &owner),
        Err(e) if e == ProgramError::Custom(ChanceryError::TokenProgramMismatch as u32),
    ));
}

// ─── Canonical issued-token mint binding (#49 / #50) ─────────────────────────

#[test]
fn canonical_issued_token_mint_accepts_config_pathway_and_account_match() {
    use crate::modules::{
        core::state::chancery_config::ChanceryConfig,
        pathway::state::pathway_policy::PathwayPolicy,
        settlement::instructions::assert_canonical_issued_token_mint,
    };

    let canonical = Pubkey::new_unique();
    let mut config = ChanceryConfig::zeroed();
    config.issued_token_mint = canonical;

    let mut pathway = PathwayPolicy::zeroed();
    pathway.issued_token_mint = canonical;

    let mut lamports = 1u64;
    let pid          = program_id();
    let mut data     = vec![0u8; 82];
    let ai           = make_account_info(&canonical, &mut data, &mut lamports, &pid, true);

    assert!(assert_canonical_issued_token_mint(&config, &pathway, &ai).is_ok());
}

#[test]
fn canonical_issued_token_mint_rejects_non_canonical_account() {
    use crate::modules::{
        core::state::chancery_config::ChanceryConfig,
        pathway::state::pathway_policy::PathwayPolicy,
        settlement::instructions::assert_canonical_issued_token_mint,
    };

    let canonical = Pubkey::new_unique();
    let wrong     = Pubkey::new_unique();
    let mut config = ChanceryConfig::zeroed();
    config.issued_token_mint = canonical;

    let mut pathway = PathwayPolicy::zeroed();
    pathway.issued_token_mint = canonical;

    let mut lamports = 1u64;
    let pid          = program_id();
    let mut data     = vec![0u8; 82];
    let ai           = make_account_info(&wrong, &mut data, &mut lamports, &pid, true);

    assert!(matches!(
        assert_canonical_issued_token_mint(&config, &pathway, &ai),
        Err(e) if e == ProgramError::Custom(ChanceryError::AccountKeyMismatch as u32),
    ));
}

#[test]
fn canonical_issued_token_mint_rejects_pathway_not_bound_to_config() {
    use crate::modules::{
        core::state::chancery_config::ChanceryConfig,
        pathway::state::pathway_policy::PathwayPolicy,
        settlement::instructions::assert_canonical_issued_token_mint,
    };

    let canonical     = Pubkey::new_unique();
    let pathway_mint  = Pubkey::new_unique();
    let mut config = ChanceryConfig::zeroed();
    config.issued_token_mint = canonical;

    let mut pathway = PathwayPolicy::zeroed();
    pathway.issued_token_mint = pathway_mint;

    let mut lamports = 1u64;
    let pid          = program_id();
    let mut data     = vec![0u8; 82];
    let ai           = make_account_info(&canonical, &mut data, &mut lamports, &pid, true);

    assert!(matches!(
        assert_canonical_issued_token_mint(&config, &pathway, &ai),
        Err(e) if e == ProgramError::Custom(ChanceryError::AccountKeyMismatch as u32),
    ));
}
