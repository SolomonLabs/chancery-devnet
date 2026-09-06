/// modules/permissions/tests/mod.rs
///
/// Unit tests for PermissionRecord state, role bit helpers, and guard methods.
/// All tests are pure - no CPI, no runtime.

use std::{cell::RefCell, rc::Rc};

use bytemuck::Zeroable;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::{role, scope, seeds},
    error::ChanceryError,
    modules::permissions::state::permission_record::{
        role_bits_as_u128, u128_to_role_bits, PermissionRecord,
        PERMISSION_RECORD_DISCRIMINATOR, PERMISSION_RECORD_SIZE,
        PERMISSION_FLAG_PAUSED,
    },
};

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn program_id() -> Pubkey {
    crate::id()
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

/// Build an initialised PermissionRecord with the given role mask and expiry.
fn make_record(
    subject:    Pubkey,
    scope_kind: u8,
    scope_key:  Pubkey,
    role_mask:  u128,
    expiry:     i64,
) -> (Pubkey, Vec<u8>, u64) {
    let pid         = program_id();
    let (key, bump) = Pubkey::find_program_address(
        &[seeds::PERMISSION, subject.as_ref(), &[scope_kind], scope_key.as_ref()],
        &pid,
    );
    let mut data    = vec![0u8; PERMISSION_RECORD_SIZE];
    let lamports    = 1u64;

    {
        // direct mut access to data
        let r: &mut PermissionRecord = bytemuck::from_bytes_mut(&mut data[..]);

        r.discriminator            = PERMISSION_RECORD_DISCRIMINATOR;
        r.version                  = 1;
        r.bump                     = bump;
        r.scope_kind               = scope_kind;
        r.subject                  = subject;
        r.scope_key                = scope_key;
        r.role_bits                = u128_to_role_bits(role_mask);
        r.permission_flags         = 0;
        r.issued_at_unix_timestamp = 0;
        r.expiry_unix_timestamp    = expiry;
        r.granted_by               = Pubkey::new_unique();
        r.role_schema_version      = role::PERMISSION_ROLE_SCHEMA_VERSION;
    }

    (key, data, lamports)
}

// ─── Size and layout ──────────────────────────────────────────────────────────

#[test]
fn size_matches_declared() {
    assert_eq!(
        core::mem::size_of::<PermissionRecord>(),
        PERMISSION_RECORD_SIZE,
    );
}

#[test]
fn zeroed_struct_has_all_zero_bytes() {
    let r            = PermissionRecord::zeroed();
    let bytes: &[u8] = bytemuck::bytes_of(&r);

    assert!(bytes.iter().all(|&b| b == 0), "zeroed() must produce all-zero bytes");
}

// ─── Load / init guards ───────────────────────────────────────────────────────

#[test]
fn load_on_zero_data_returns_not_initialized() {
    let pid          = program_id();
    let key          = Pubkey::new_unique();
    let mut data     = vec![0u8; PERMISSION_RECORD_SIZE];
    let mut lamports = 1u64;
    let ai           = make_account_info(&key, &mut data, &mut lamports, &pid, false);

    assert!(matches!(
        PermissionRecord::load(&ai),
        Err(e) if e == ProgramError::Custom(ChanceryError::NotInitialized as u32),
    ));
}

#[test]
fn load_on_wrong_size_returns_length_mismatch() {
    let pid          = program_id();
    let key          = Pubkey::new_unique();
    let mut data     = vec![0u8; PERMISSION_RECORD_SIZE - 1];
    let mut lamports = 1u64;
    let ai           = make_account_info(&key, &mut data, &mut lamports, &pid, false);

    assert!(matches!(
        PermissionRecord::load(&ai),
        Err(e) if e == ProgramError::Custom(ChanceryError::AccountDataLengthMismatch as u32),
    ));
}

#[test]
fn load_mut_on_non_writable_returns_error() {
    let subject = Pubkey::new_unique();
    let (key, mut data, mut lamports) = make_record(
        subject, scope::GLOBAL, Pubkey::default(), role::CAN_MINT_DIRECT, 0,
    );
    let pid     = program_id();
    let ai      = make_account_info(&key, &mut data, &mut lamports, &pid, false);  // NOT writable

    assert!(matches!(
        PermissionRecord::load_mut(&ai),
        Err(e) if e == ProgramError::Custom(ChanceryError::AccountNotWritable as u32),
    ));
}

#[test]
fn load_uninitialized_mut_blocks_double_init() {
    let pid          = program_id();
    let key          = Pubkey::new_unique();
    let mut data     = vec![0u8; PERMISSION_RECORD_SIZE];
    let mut lamports = 1u64;

    // First init
    {
        let ai = make_account_info(&key, &mut data, &mut lamports, &pid, true);
        let mut r  = PermissionRecord::load_uninitialized_mut(&ai).unwrap();

        r.discriminator = PERMISSION_RECORD_DISCRIMINATOR;
    }

    // Second attempt
    {
        let ai = make_account_info(&key, &mut data, &mut lamports, &pid, true);

        assert!(matches!(
            PermissionRecord::load_uninitialized_mut(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::AlreadyInitialized as u32),
        ));
    }
}

#[test]
fn load_reads_fields_correctly() {
    let subject   = Pubkey::new_unique();
    let scope_key = Pubkey::new_unique();
    let role_mask = role::CAN_MINT_DIRECT | role::CAN_REDEEM_DIRECT;
    let expiry    = 99_999i64;
    let (key, mut data, mut lamports) = make_record(
        subject, scope::ASSET, scope_key, role_mask, expiry,
    );
    let pid       = program_id();
    let ai        = make_account_info(&key, &mut data, &mut lamports, &pid, false);
    let r         = PermissionRecord::load(&ai).unwrap();

    assert_eq!(r.subject,                      subject);
    assert_eq!(r.scope_kind,                   scope::ASSET);
    assert_eq!(r.scope_key,                    scope_key);
    assert_eq!(r.expiry_unix_timestamp,        expiry);
    assert_eq!(role_bits_as_u128(r.role_bits), role_mask);
}

// ─── Role bit helpers ─────────────────────────────────────────────────────────

#[test]
fn role_bits_round_trip_single_low_bit() {
    let mask = role::CAN_MINT_DIRECT;  // bit 0

    assert_eq!(role_bits_as_u128(u128_to_role_bits(mask)), mask);
}

#[test]
fn role_bits_round_trip_high_bit() {
    // CAN_SET_INSURANCE_POLICY = 1 << 25 - sits in low word
    let mask = role::CAN_SET_INSURANCE_POLICY;

    assert_eq!(role_bits_as_u128(u128_to_role_bits(mask)), mask);
}

#[test]
fn role_bits_round_trip_all_mvp_roles() {
    let mask = role::CAN_MINT_DIRECT
        | role::CAN_REDEEM_DIRECT
        | role::CAN_MINT_DELEGATED
        | role::CAN_REDEEM_DELEGATED
        | role::CAN_EXECUTE_SETTLEMENT
        | role::CAN_USE_TRILATERAL_PATHWAY
        | role::CAN_WITHDRAW_RESERVE
        | role::CAN_GRANT_PERMISSION
        | role::CAN_REVOKE_PERMISSION
        | role::CAN_SET_GLOBAL_PAUSE
        | role::CAN_PROPOSE_AUTHORITY_TRANSFER
        | role::CAN_ACCEPT_AUTHORITY_TRANSFER
        | role::CAN_EXECUTE_LEGACY_MIGRATION
        | role::CAN_SET_PATHWAY_POLICY
        | role::CAN_SET_FEE_POLICY;
    let words  = u128_to_role_bits(mask);
    let result = role_bits_as_u128(words);

    assert_eq!(result, mask, "Full MVP role mask failed round-trip");
}

#[test]
fn role_bits_high_word_carries_bits_above_64() {
    // Simulate a bit in position 64+ by setting high word manually
    let words = [0u64, 1u64];  // bit 64 set
    let value = role_bits_as_u128(words);

    assert_eq!(value, 1u128 << 64);
}

#[test]
fn u128_to_role_bits_splits_at_64_boundary() {
    let mask  = (1u128 << 63) | (1u128 << 64);
    let words = u128_to_role_bits(mask);

    assert_eq!(words[0], 1u64 << 63, "low word should carry bit 63");
    assert_eq!(words[1], 1u64,       "high word should carry bit 64 as bit 0");
}

// ─── has_role / assert_role ───────────────────────────────────────────────────

#[test]
fn has_role_exact_single_bit_match() {
    let mut r = PermissionRecord::zeroed();

    r.role_bits = u128_to_role_bits(role::CAN_MINT_DIRECT);

    assert!(r.has_role(role::CAN_MINT_DIRECT));
}

#[test]
fn has_role_does_not_match_adjacent_bit() {
    let mut r = PermissionRecord::zeroed();

    r.role_bits = u128_to_role_bits(role::CAN_MINT_DIRECT);

    assert!(!r.has_role(role::CAN_REDEEM_DIRECT));
}

#[test]
fn has_role_requires_all_bits_in_mask() {
    let mask = role::CAN_MINT_DIRECT | role::CAN_REDEEM_DIRECT;
    let mut r    = PermissionRecord::zeroed();

    // Only one of two required bits set
    r.role_bits = u128_to_role_bits(role::CAN_MINT_DIRECT);

    assert!(!r.has_role(mask), "partial match must not pass has_role");
}

#[test]
fn has_role_passes_superset() {
    // Record holds more roles than required - still passes
    let held     = role::CAN_MINT_DIRECT | role::CAN_REDEEM_DIRECT | role::CAN_WITHDRAW_RESERVE;
    let required = role::CAN_MINT_DIRECT | role::CAN_REDEEM_DIRECT;
    let mut r    = PermissionRecord::zeroed();

    r.role_bits = u128_to_role_bits(held);

    assert!(r.has_role(required));
}

#[test]
fn assert_role_passes_when_held() {
    let mut r = PermissionRecord::zeroed();

    r.role_bits = u128_to_role_bits(role::CAN_EXECUTE_SETTLEMENT);
    r.role_schema_version = role::PERMISSION_ROLE_SCHEMA_VERSION;

    assert!(r.assert_role(role::CAN_EXECUTE_SETTLEMENT).is_ok());
}

#[test]
fn assert_role_returns_insufficient_role_when_missing() {
    let mut r = PermissionRecord::zeroed();

    r.role_bits = u128_to_role_bits(role::CAN_MINT_DIRECT);
    r.role_schema_version = role::PERMISSION_ROLE_SCHEMA_VERSION;

    assert!(matches!(
        r.assert_role(role::CAN_EXECUTE_FORCED_BURN),
        Err(e) if e == ProgramError::Custom(ChanceryError::InsufficientRole as u32),
    ));
}

#[test]
fn assert_role_zero_mask_always_passes() {
    // Requesting no roles is always satisfied
    let mut r = PermissionRecord::zeroed();
    r.role_schema_version = role::PERMISSION_ROLE_SCHEMA_VERSION;

    assert!(r.assert_role(0u128).is_ok());
}

// ─── Expiry ───────────────────────────────────────────────────────────────────

#[test]
fn assert_not_expired_before_expiry_passes() {
    let mut r = PermissionRecord::zeroed();

    r.expiry_unix_timestamp = 1_000_000;

    assert!(r.assert_not_expired(999_999).is_ok());
}

#[test]
fn assert_not_expired_at_exact_expiry_timestamp_fails() {
    let mut r = PermissionRecord::zeroed();

    r.expiry_unix_timestamp = 1_000_000;

    // at exactly expiry_unix_timestamp -> expired
    assert!(matches!(
        r.assert_not_expired(1_000_000),
        Err(e) if e == ProgramError::Custom(ChanceryError::PermissionExpired as u32),
    ));
}

#[test]
fn assert_not_expired_past_expiry_fails() {
    let mut r = PermissionRecord::zeroed();

    r.expiry_unix_timestamp = 1_000_000;

    assert!(r.assert_not_expired(1_000_001).is_err());
}

#[test]
fn assert_not_expired_zero_expiry_never_expires() {
    let mut r = PermissionRecord::zeroed();

    r.expiry_unix_timestamp = 0;  // 0 = no expiry

    assert!(r.assert_not_expired(i64::MAX).is_ok());
}

#[test]
fn assert_not_expired_negative_timestamp_never_expires_if_zero() {
    // Negative timestamps before unix epoch; expiry=0 still means never
    let mut r = PermissionRecord::zeroed();

    r.expiry_unix_timestamp = 0;

    assert!(r.assert_not_expired(-1).is_ok());
}

// ─── Scope ────────────────────────────────────────────────────────────────────

#[test]
fn assert_scope_matches_exact() {
    let scope_key = Pubkey::new_unique();
    let mut r     = PermissionRecord::zeroed();

    r.scope_kind = scope::ASSET;
    r.scope_key  = scope_key;

    assert!(r.assert_scope(scope::ASSET, &scope_key).is_ok());
}

#[test]
fn assert_scope_wrong_kind_fails() {
    let scope_key = Pubkey::new_unique();
    let mut r     = PermissionRecord::zeroed();

    r.scope_kind = scope::ASSET;
    r.scope_key  = scope_key;

    assert!(matches!(
        r.assert_scope(scope::GLOBAL, &scope_key),
        Err(e) if e == ProgramError::Custom(ChanceryError::PermissionScopeMismatch as u32),
    ));
}

#[test]
fn assert_scope_wrong_key_fails() {
    let scope_key = Pubkey::new_unique();
    let other_key = Pubkey::new_unique();
    let mut r     = PermissionRecord::zeroed();

    r.scope_kind = scope::ASSET;
    r.scope_key  = scope_key;

    assert!(matches!(
        r.assert_scope(scope::ASSET, &other_key),
        Err(e) if e == ProgramError::Custom(ChanceryError::PermissionScopeMismatch as u32),
    ));
}

#[test]
fn assert_scope_global_uses_default_key() {
    let mut r = PermissionRecord::zeroed();

    r.scope_kind = scope::GLOBAL;
    r.scope_key  = Pubkey::default();

    assert!(r.assert_scope(scope::GLOBAL, &Pubkey::default()).is_ok());
}

// ─── PERMISSION_FLAG_PAUSED ─────────────────────────────────────────────────────────

#[test]
fn perm_flag_paused_is_bit_zero_of_permission_flags() {
    assert_eq!(PERMISSION_FLAG_PAUSED, 1u64,
        "PERMISSION_FLAG_PAUSED must be bit 0 - settlement handlers rely on this");
}

#[test]
fn permission_flags_paused_bit_is_distinct_from_role_bits() {
    // permission_flags and role_bits are separate fields - setting PERMISSION_FLAG_PAUSED
    // must not affect role_bits
    let mut r = PermissionRecord::zeroed();

    r.role_bits        = u128_to_role_bits(role::CAN_MINT_DIRECT);
    r.permission_flags = PERMISSION_FLAG_PAUSED;

    assert!(r.has_role(role::CAN_MINT_DIRECT), "role_bits must be unaffected");
    assert_eq!(r.permission_flags & PERMISSION_FLAG_PAUSED, PERMISSION_FLAG_PAUSED);
}

#[test]
fn paused_flag_can_be_set_and_cleared() {
    let mut r = PermissionRecord::zeroed();

    r.permission_flags = 0;

    assert_eq!(r.permission_flags & PERMISSION_FLAG_PAUSED, 0);

    r.permission_flags |= PERMISSION_FLAG_PAUSED;

    assert_eq!(r.permission_flags & PERMISSION_FLAG_PAUSED, PERMISSION_FLAG_PAUSED);

    r.permission_flags &= !PERMISSION_FLAG_PAUSED;

    assert_eq!(r.permission_flags & PERMISSION_FLAG_PAUSED, 0);
}

// ─── assert_valid (combined gate) ────────────────────────────────────────────

#[test]
fn assert_valid_passes_when_all_conditions_met() {
    let scope_key = Pubkey::new_unique();
    let mut r     = PermissionRecord::zeroed();

    r.discriminator  = PERMISSION_RECORD_DISCRIMINATOR;
    r.scope_kind     = scope::ASSET;
    r.scope_key      = scope_key;
    r.role_bits      = u128_to_role_bits(role::CAN_MINT_DIRECT);
    r.expiry_unix_timestamp = 0;  // never expires
    r.role_schema_version = role::PERMISSION_ROLE_SCHEMA_VERSION;

    assert!(r.assert_valid(
        role::CAN_MINT_DIRECT,
        scope::ASSET,
        &scope_key,
        1_000,
    ).is_ok());
}

#[test]
fn assert_valid_fails_on_wrong_role() {
    let scope_key = Pubkey::new_unique();
    let mut r     = PermissionRecord::zeroed();

    r.scope_kind            = scope::ASSET;
    r.scope_key             = scope_key;
    r.role_bits             = u128_to_role_bits(role::CAN_MINT_DIRECT);
    r.expiry_unix_timestamp = 0;
    r.role_schema_version = role::PERMISSION_ROLE_SCHEMA_VERSION;

    assert!(r.assert_valid(
        role::CAN_REDEEM_DIRECT,  // wrong role
        scope::ASSET,
        &scope_key,
        1_000,
    ).is_err());
}

#[test]
fn assert_valid_fails_on_expired() {
    let mut r = PermissionRecord::zeroed();

    r.scope_kind            = scope::GLOBAL;
    r.scope_key             = Pubkey::default();
    r.role_bits             = u128_to_role_bits(role::CAN_MINT_DIRECT);
    r.expiry_unix_timestamp = 500;  // expires at 500
    r.role_schema_version = role::PERMISSION_ROLE_SCHEMA_VERSION;

    assert!(r.assert_valid(
        role::CAN_MINT_DIRECT,
        scope::GLOBAL,
        &Pubkey::default(),
        500,  // now == expiry -> expired
    ).is_err());
}

#[test]
fn assert_valid_fails_when_permission_is_paused() {
    let scope_key = Pubkey::new_unique();
    let mut r = PermissionRecord::zeroed();

    r.scope_kind = scope::PATHWAY;
    r.scope_key = scope_key;
    r.role_bits = u128_to_role_bits(role::CAN_EXECUTE_SETTLEMENT);
    r.permission_flags = PERMISSION_FLAG_PAUSED;
    r.role_schema_version = role::PERMISSION_ROLE_SCHEMA_VERSION;

    assert!(matches!(
        r.assert_valid(
            role::CAN_EXECUTE_SETTLEMENT,
            scope::PATHWAY,
            &scope_key,
            1_000,
        ),
        Err(e) if e == ProgramError::Custom(ChanceryError::PermissionPaused as u32),
    ));
}

#[test]
fn assert_valid_fails_on_wrong_scope_kind() {
    let scope_key = Pubkey::new_unique();
    let mut r     = PermissionRecord::zeroed();

    r.scope_kind            = scope::ASSET;
    r.scope_key             = scope_key;
    r.role_bits             = u128_to_role_bits(role::CAN_MINT_DIRECT);
    r.expiry_unix_timestamp = 0;
    r.role_schema_version = role::PERMISSION_ROLE_SCHEMA_VERSION;

    assert!(r.assert_valid(
        role::CAN_MINT_DIRECT,
        scope::GLOBAL,  // wrong scope
        &scope_key,
        0,
    ).is_err());
}

// ─── PDA derivation ───────────────────────────────────────────────────────────

#[test]
fn pda_derivation_includes_all_four_seeds() {
    let pid       = program_id();
    let subject   = Pubkey::new_unique();
    let scope_key = Pubkey::new_unique();

    let (key_asset, _) = PermissionRecord::pda(&subject, scope::ASSET,   &scope_key, &pid);
    let (key_global,_) = PermissionRecord::pda(&subject, scope::GLOBAL,  &scope_key, &pid);
    let (key_path, _)  = PermissionRecord::pda(&subject, scope::PATHWAY, &scope_key, &pid);

    // Different scope_kind -> different PDAs
    assert_ne!(key_asset, key_global);
    assert_ne!(key_asset, key_path);
    assert_ne!(key_global, key_path);
}

#[test]
fn pda_derivation_different_subject_gives_different_key() {
    let pid     = program_id();
    let s1      = Pubkey::new_unique();
    let s2      = Pubkey::new_unique();
    let sk      = Pubkey::default();
    let (k1, _) = PermissionRecord::pda(&s1, scope::GLOBAL, &sk, &pid);
    let (k2, _) = PermissionRecord::pda(&s2, scope::GLOBAL, &sk, &pid);

    assert_ne!(k1, k2);
}

#[test]
fn pda_derivation_is_deterministic() {
    let pid       = program_id();
    let subject   = Pubkey::new_unique();
    let scope_key = Pubkey::new_unique();
    let (k1, b1)  = PermissionRecord::pda(&subject, scope::ASSET, &scope_key, &pid);
    let (k2, b2)  = PermissionRecord::pda(&subject, scope::ASSET, &scope_key, &pid);

    assert_eq!(k1, k2);
    assert_eq!(b1, b2);
}

// ─── Anti-escalation invariant ────────────────────────────────────────────────

#[test]
fn role_superset_check_catches_escalation() {
    // Grantor holds CAN_MINT_DIRECT only.
    // Attempt to grant CAN_MINT_DIRECT | CAN_REDEEM_DIRECT -> escalation.
    let grantor_roles: u128 = role::CAN_MINT_DIRECT;
    let granted_roles: u128 = role::CAN_MINT_DIRECT | role::CAN_REDEEM_DIRECT;

    // The upsert_permission handler checks: granted & !grantor != 0 -> deny.
    let escalation_detected = granted_roles & !grantor_roles != 0;

    assert!(escalation_detected, "Escalation must be detected when granted > held");
}

#[test]
fn role_subset_check_passes_for_valid_grant() {
    let grantor_roles: u128 = role::CAN_MINT_DIRECT | role::CAN_REDEEM_DIRECT;
    let granted_roles: u128 = role::CAN_MINT_DIRECT;  // subset

    let escalation_detected = granted_roles & !grantor_roles != 0;

    assert!(!escalation_detected, "No escalation when granting subset of own roles");
}

#[test]
fn exact_match_grant_is_not_escalation() {
    let grantor_roles: u128 = role::CAN_MINT_DIRECT;
    let granted_roles: u128 = role::CAN_MINT_DIRECT;

    let escalation_detected = granted_roles & !grantor_roles != 0;

    assert!(!escalation_detected, "Granting exact held roles is not escalation");
}

// ─── Canonical auth loaders (permissions/auth.rs) ─────────────────────────────
//
// Direct unit pins for the shared permission-gate chain: owner check +
// canonical-PDA proof + verified load (discriminator/version/length) +
// stored-bump proof + identity/role/scope/expiry/paused semantics. Every
// role-consuming handler routes through these loaders, so this module pins
// them against refactor drift the way state_loader's own tests pin the
// generic loader.
mod canonical_auth_loaders {
    use std::{cell::RefCell, rc::Rc};

    use solana_program_error::ProgramError;
    use solana_pubkey::Pubkey;

    use crate::{
        constants::{role, scope},
        error::ChanceryError,
        modules::permissions::auth::{
            assert_can_freeze_token_account, assert_subject_holds_role,
            load_canonical_permission_record, load_optional_canonical_permission_record,
        },
    };

    use super::{make_record, program_id};

    // ── Fixed clock for Clock-consuming gates ─────────────────────────────────
    //
    // `assert_can_freeze/thaw_token_account` reach `assert_not_expired_now`,
    // which calls `Clock::get()` unconditionally; off-runtime the default
    // syscall stubs return `UNSUPPORTED_SYSVAR` and the gate errors before its
    // scope/key semantics are reached. Install a process-global stub serving a
    // fixed, valid clock. This is strictly additive: no unit test in this
    // crate depends on the clock being unavailable, and every time-sensitive
    // assertion in these modules passes `now` explicitly.
    struct FixedClockStubs;

    impl solana_sysvar::program_stubs::SyscallStubs for FixedClockStubs {
        fn sol_get_clock_sysvar(&self, var_addr: *mut u8) -> u64 {
            let clock = solana_clock::Clock {
                slot:                  1,
                epoch_start_timestamp: 0,
                epoch:                 0,
                leader_schedule_epoch: 0,
                unix_timestamp:        1_000,
            };
            unsafe { core::ptr::write(var_addr as *mut solana_clock::Clock, clock) };
            solana_program_entrypoint::SUCCESS
        }
    }

    fn install_fixed_clock() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            solana_sysvar::program_stubs::set_syscall_stubs(Box::new(FixedClockStubs));
        });
    }

    #[allow(deprecated)]
    fn account_info<'a>(
        key:       &'a Pubkey,
        data:      &'a mut [u8],
        lamports:  &'a mut u64,
        owner:     &'a Pubkey,
        writable:  bool,
        is_signer: bool,
    ) -> solana_account_info::AccountInfo<'a> {
        solana_account_info::AccountInfo {
            key,
            lamports:    Rc::new(RefCell::new(lamports)),
            data:        Rc::new(RefCell::new(data)),
            owner,
            _unused:     0,
            is_signer,
            is_writable: writable,
            executable:  false,
        }
    }

    #[test]
    fn load_canonical_accepts_canonical_record() {
        let pid     = program_id();
        let subject = Pubkey::new_unique();
        let scoped  = Pubkey::new_unique();
        let (key, mut data, mut lam) =
            make_record(subject, scope::PATHWAY, scoped, role::CAN_EXECUTE_SETTLEMENT, 0);
        let ai = account_info(&key, &mut data, &mut lam, &pid, false, false);

        let r = load_canonical_permission_record(&ai, &pid).unwrap();
        assert_eq!(r.subject, subject);
        assert_eq!(r.scope_kind, scope::PATHWAY);
        assert_eq!(r.scope_key, scoped);
    }

    #[test]
    fn load_canonical_rejects_wrong_owner() {
        let pid         = program_id();
        let wrong_owner = Pubkey::new_unique();
        let subject     = Pubkey::new_unique();
        let (key, mut data, mut lam) =
            make_record(subject, scope::GLOBAL, Pubkey::default(), role::CAN_MINT_DIRECT, 0);
        let ai = account_info(&key, &mut data, &mut lam, &wrong_owner, false, false);

        assert!(matches!(
            load_canonical_permission_record(&ai, &pid),
            Err(e) if e == ProgramError::Custom(ChanceryError::AccountOwnerMismatch as u32),
        ));
    }

    #[test]
    fn load_canonical_rejects_non_canonical_key() {
        let pid     = program_id();
        let subject = Pubkey::new_unique();
        let (_, mut data, mut lam) =
            make_record(subject, scope::GLOBAL, Pubkey::default(), role::CAN_MINT_DIRECT, 0);
        // Same initialized bytes presented at a non-canonical address.
        let wrong_key = Pubkey::new_unique();
        let ai = account_info(&wrong_key, &mut data, &mut lam, &pid, false, false);

        assert!(load_canonical_permission_record(&ai, &pid).is_err());
    }

    #[test]
    fn load_canonical_rejects_stored_bump_mismatch() {
        let pid     = program_id();
        let subject = Pubkey::new_unique();
        let (key, mut data, mut lam) =
            make_record(subject, scope::GLOBAL, Pubkey::default(), role::CAN_MINT_DIRECT, 0);
        // Corrupt the stored bump: identity fields still resolve to this PDA,
        // but the stored-bump re-proof must fail closed.
        {
            let r: &mut super::PermissionRecord = bytemuck::from_bytes_mut(&mut data[..]);
            r.bump = r.bump.wrapping_add(1);
        }
        let ai = account_info(&key, &mut data, &mut lam, &pid, false, false);

        assert!(matches!(
            load_canonical_permission_record(&ai, &pid),
            Err(e) if e == ProgramError::Custom(ChanceryError::StoredBumpMismatch as u32),
        ));
    }

    #[test]
    fn load_optional_returns_none_for_empty_and_zeroed() {
        let pid = program_id();

        let key_a        = Pubkey::new_unique();
        let mut empty    = [0u8; 0];
        let mut lam_a    = 1u64;
        let empty_ai     = account_info(&key_a, &mut empty, &mut lam_a, &pid, false, false);
        assert!(load_optional_canonical_permission_record(&empty_ai, &pid).unwrap().is_none());

        let key_b        = Pubkey::new_unique();
        let mut zeroed   = vec![0u8; super::PERMISSION_RECORD_SIZE];
        let mut lam_b    = 1u64;
        let zeroed_ai    = account_info(&key_b, &mut zeroed, &mut lam_b, &pid, false, false);
        assert!(load_optional_canonical_permission_record(&zeroed_ai, &pid).unwrap().is_none());
    }

    #[test]
    fn load_optional_rejects_wrong_length() {
        let pid      = program_id();
        let key      = Pubkey::new_unique();
        let mut data = vec![0u8; super::PERMISSION_RECORD_SIZE - 1];
        let mut lam  = 1u64;
        let ai       = account_info(&key, &mut data, &mut lam, &pid, false, false);

        assert!(matches!(
            load_optional_canonical_permission_record(&ai, &pid),
            Err(e) if e == ProgramError::Custom(ChanceryError::AccountDataLengthMismatch as u32),
        ));
    }

    #[test]
    fn subject_holds_role_happy_path() {
        let pid     = program_id();
        let subject = Pubkey::new_unique();
        let scoped  = Pubkey::new_unique();
        let (key, mut data, mut lam) =
            make_record(subject, scope::PATHWAY, scoped, role::CAN_EXECUTE_SETTLEMENT, 0);
        let ai = account_info(&key, &mut data, &mut lam, &pid, false, false);

        assert!(assert_subject_holds_role(
            &subject, &ai, role::CAN_EXECUTE_SETTLEMENT, scope::PATHWAY, &scoped, 1_000, &pid,
        )
        .is_ok());
    }

    #[test]
    fn subject_holds_role_rejects_wrong_subject() {
        let pid     = program_id();
        let subject = Pubkey::new_unique();
        let scoped  = Pubkey::new_unique();
        let (key, mut data, mut lam) =
            make_record(subject, scope::PATHWAY, scoped, role::CAN_EXECUTE_SETTLEMENT, 0);
        let ai = account_info(&key, &mut data, &mut lam, &pid, false, false);

        let impostor = Pubkey::new_unique();
        assert!(matches!(
            assert_subject_holds_role(
                &impostor, &ai, role::CAN_EXECUTE_SETTLEMENT, scope::PATHWAY, &scoped, 1_000, &pid,
            ),
            Err(e) if e == ProgramError::Custom(ChanceryError::PermissionScopeMismatch as u32),
        ));
    }

    #[test]
    fn subject_holds_role_rejects_missing_role() {
        let pid     = program_id();
        let subject = Pubkey::new_unique();
        let scoped  = Pubkey::new_unique();
        let (key, mut data, mut lam) =
            make_record(subject, scope::PATHWAY, scoped, role::CAN_MINT_DIRECT, 0);
        let ai = account_info(&key, &mut data, &mut lam, &pid, false, false);

        assert!(matches!(
            assert_subject_holds_role(
                &subject, &ai, role::CAN_EXECUTE_SETTLEMENT, scope::PATHWAY, &scoped, 1_000, &pid,
            ),
            Err(e) if e == ProgramError::Custom(ChanceryError::InsufficientRole as u32),
        ));
    }

    #[test]
    fn subject_holds_role_rejects_wrong_scope() {
        let pid     = program_id();
        let subject = Pubkey::new_unique();
        let scoped  = Pubkey::new_unique();
        let (key, mut data, mut lam) =
            make_record(subject, scope::PATHWAY, scoped, role::CAN_EXECUTE_SETTLEMENT, 0);
        let ai = account_info(&key, &mut data, &mut lam, &pid, false, false);

        let other_pathway = Pubkey::new_unique();
        assert!(matches!(
            assert_subject_holds_role(
                &subject, &ai, role::CAN_EXECUTE_SETTLEMENT, scope::PATHWAY, &other_pathway, 1_000, &pid,
            ),
            Err(e) if e == ProgramError::Custom(ChanceryError::PermissionScopeMismatch as u32),
        ));
    }

    #[test]
    fn subject_holds_role_rejects_expired_grant() {
        let pid     = program_id();
        let subject = Pubkey::new_unique();
        let scoped  = Pubkey::new_unique();
        let (key, mut data, mut lam) =
            make_record(subject, scope::PATHWAY, scoped, role::CAN_EXECUTE_SETTLEMENT, 500);
        let ai = account_info(&key, &mut data, &mut lam, &pid, false, false);

        assert!(matches!(
            assert_subject_holds_role(
                &subject, &ai, role::CAN_EXECUTE_SETTLEMENT, scope::PATHWAY, &scoped, 1_000, &pid,
            ),
            Err(e) if e == ProgramError::Custom(ChanceryError::PermissionExpired as u32),
        ));
    }

    #[test]
    fn subject_holds_role_rejects_paused_grant() {
        let pid     = program_id();
        let subject = Pubkey::new_unique();
        let scoped  = Pubkey::new_unique();
        let (key, mut data, mut lam) =
            make_record(subject, scope::PATHWAY, scoped, role::CAN_EXECUTE_SETTLEMENT, 0);
        {
            let r: &mut super::PermissionRecord = bytemuck::from_bytes_mut(&mut data[..]);
            r.permission_flags = super::PERMISSION_FLAG_PAUSED;
        }
        let ai = account_info(&key, &mut data, &mut lam, &pid, false, false);

        assert!(matches!(
            assert_subject_holds_role(
                &subject, &ai, role::CAN_EXECUTE_SETTLEMENT, scope::PATHWAY, &scoped, 1_000, &pid,
            ),
            Err(e) if e == ProgramError::Custom(ChanceryError::PermissionPaused as u32),
        ));
    }

    #[test]
    fn freeze_gate_requires_token_account_scope_and_exact_key() {
        install_fixed_clock();

        let pid           = program_id();
        let subject       = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();

        // Wide (pathway) scope: dangerous role must be narrow-scope only.
        let (wide_key, mut wide_data, mut wide_lam) =
            make_record(subject, scope::PATHWAY, token_account, role::CAN_FREEZE_TOKEN_ACCOUNT, 0);
        {
            let mut signer_lam  = 1u64;
            let mut signer_data = [0u8; 0];
            let signer = account_info(&subject, &mut signer_data, &mut signer_lam, &pid, false, true);
            let record = account_info(&wide_key, &mut wide_data, &mut wide_lam, &pid, false, false);

            assert!(matches!(
                assert_can_freeze_token_account(&signer, &record, &token_account, &pid),
                Err(e) if e == ProgramError::Custom(
                    ChanceryError::DangerousPermissionRequiresNarrowScope as u32,
                ),
            ));
        }

        // Narrow scope, wrong token key: the grant must not transfer.
        let (narrow_key, mut narrow_data, mut narrow_lam) =
            make_record(subject, scope::TOKEN_ACCOUNT, token_account, role::CAN_FREEZE_TOKEN_ACCOUNT, 0);
        {
            let mut signer_lam  = 1u64;
            let mut signer_data = [0u8; 0];
            let signer = account_info(&subject, &mut signer_data, &mut signer_lam, &pid, false, true);
            let record = account_info(&narrow_key, &mut narrow_data, &mut narrow_lam, &pid, false, false);

            let other_token_account = Pubkey::new_unique();
            assert!(matches!(
                assert_can_freeze_token_account(&signer, &record, &other_token_account, &pid),
                Err(e) if e == ProgramError::Custom(ChanceryError::AccountKeyMismatch as u32),
            ));

            // Exact match passes.
            assert!(assert_can_freeze_token_account(&signer, &record, &token_account, &pid).is_ok());
        }
    }

    #[test]
    fn freeze_gate_requires_signer() {
        install_fixed_clock();

        let pid           = program_id();
        let subject       = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let (key, mut data, mut lam) =
            make_record(subject, scope::TOKEN_ACCOUNT, token_account, role::CAN_FREEZE_TOKEN_ACCOUNT, 0);

        let mut signer_lam  = 1u64;
        let mut signer_data = [0u8; 0];
        let non_signer = account_info(&subject, &mut signer_data, &mut signer_lam, &pid, false, false);
        let record     = account_info(&key, &mut data, &mut lam, &pid, false, false);

        assert!(matches!(
            assert_can_freeze_token_account(&non_signer, &record, &token_account, &pid),
            Err(e) if e == ProgramError::Custom(ChanceryError::AccountNotSigner as u32),
        ));
    }
}
