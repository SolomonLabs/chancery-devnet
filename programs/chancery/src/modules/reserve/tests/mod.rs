/// modules/reserve/tests/mod.rs
///
/// Unit tests for ReserveDestination state and guard methods.
///
/// Covers:
///   - size / layout
///   - load guards (uninit, wrong size, non-writable, double-init)
///   - explicit ENABLED / DISABLED / DEPRECATED lifecycle
///   - assert_asset: matching and mismatched mint
///   - assert_destination_account: matching and mismatched token account
///   - destination_flag constants are distinct single bits
///   - PDA derivation: deterministic, unique per (asset_mint, destination_account)
///   - field round-trip

use std::{cell::RefCell, rc::Rc};

use bytemuck::Zeroable;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::{reserve_destination_status, seeds},
    error::ChanceryError,
    modules::reserve::state::reserve_destination::{
        destination_flag, ReserveDestination,
        RESERVE_DESTINATION_DISCRIMINATOR, RESERVE_DESTINATION_SIZE,
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

fn make_destination(
    asset_mint:          Pubkey,
    destination_account: Pubkey,
    destination_owner:   Pubkey,
    flags:               u64,
) -> (Pubkey, Vec<u8>, u64) {
    let pid = program_id();
    let (key, bump) = Pubkey::find_program_address(
        &[
            seeds::RESERVE_DESTINATION,
            asset_mint.as_ref(),
            destination_account.as_ref(),
        ],
        &pid,
    );
    let mut data    = vec![0u8; RESERVE_DESTINATION_SIZE];
    let lamports    = 1u64;

    {
        // direct mut access to data
        let dest: &mut ReserveDestination = bytemuck::from_bytes_mut(&mut data[..]);

        dest.discriminator             = RESERVE_DESTINATION_DISCRIMINATOR;
        dest.version                   = 1;
        dest.bump                      = bump;
        dest.status                    = reserve_destination_status::ENABLED;
        dest.asset_mint                = asset_mint;
        dest.destination_token_account = destination_account;
        dest.destination_owner         = destination_owner;
        dest.destination_flags         = flags;
        dest.approved_by               = Pubkey::new_unique();
    }

    (key, data, lamports)
}

// ─── destination purpose flags ───────────────────────────────────────────────

#[test]
fn destination_purpose_flags_are_distinct_single_bits() {
    let flags = [
        destination_flag::TREASURY,
        destination_flag::DOWNSTREAM_CUSTODY,
        destination_flag::OPERATIONS,
        destination_flag::RECOVERY,
    ];

    for (i, &a) in flags.iter().enumerate() {
        assert_eq!(a.count_ones(), 1, "flag {i} must be a single bit");

        for (j, &b) in flags.iter().enumerate() {
            if i != j {
                assert_eq!(a & b, 0, "flags {i} and {j} must not overlap");
            }
        }
    }
}

#[test]
fn purpose_validation_requires_exactly_one_known_flag() {
    assert!(ReserveDestination::validate_purpose_flags(destination_flag::TREASURY).is_ok());
    assert!(ReserveDestination::validate_purpose_flags(0).is_err());
    assert!(ReserveDestination::validate_purpose_flags(
        destination_flag::TREASURY | destination_flag::RECOVERY,
    ).is_err());
    assert!(ReserveDestination::validate_purpose_flags(destination_flag::DISABLED).is_err());
}

// ─── Size / layout ────────────────────────────────────────────────────────────

#[test]
fn size_matches_declared() {
    assert_eq!(core::mem::size_of::<ReserveDestination>(), 216);
    assert_eq!(RESERVE_DESTINATION_SIZE, 216);
    assert_eq!(core::mem::offset_of!(ReserveDestination, withdrawal_limit_policy_id), 152);
    assert_eq!(core::mem::offset_of!(ReserveDestination, _reserved), 184);
}

#[test]
fn zeroed_struct_all_zero_bytes() {
    let d            = ReserveDestination::zeroed();
    let bytes: &[u8] = bytemuck::bytes_of(&d);

    assert!(bytes.iter().all(|&b| b == 0));
    assert!(d._reserved.iter().all(|&b| b == 0));
}

// ─── Load guards ──────────────────────────────────────────────────────────────

#[test]
fn load_zero_data_returns_not_initialized() {
    let pid          = program_id();
    let key          = Pubkey::new_unique();
    let mut data     = vec![0u8; RESERVE_DESTINATION_SIZE];
    let mut lamports = 1u64;
    let ai           = make_account_info(&key, &mut data, &mut lamports, &pid, false);

    assert!(matches!(
        ReserveDestination::load(&ai),
        Err(e) if e == ProgramError::Custom(ChanceryError::NotInitialized as u32),
    ));
}

#[test]
fn load_wrong_size_returns_length_mismatch() {
    let pid          = program_id();
    let key          = Pubkey::new_unique();
    let mut data     = vec![0u8; RESERVE_DESTINATION_SIZE - 1];
    let mut lamports = 1u64;
    let ai           = make_account_info(&key, &mut data, &mut lamports, &pid, false);

    assert!(matches!(
        ReserveDestination::load(&ai),
        Err(e) if e == ProgramError::Custom(ChanceryError::AccountDataLengthMismatch as u32),
    ));
}

#[test]
fn load_mut_non_writable_returns_error() {
    let mint = Pubkey::new_unique();
    let dest = Pubkey::new_unique();
    let (key, mut data, mut lamports) = make_destination(mint, dest, Pubkey::new_unique(), 0);
    let pid = program_id();
    let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);

    assert!(matches!(
        ReserveDestination::load_mut(&ai),
        Err(e) if e == ProgramError::Custom(ChanceryError::AccountNotWritable as u32),
    ));
}

#[test]
fn load_uninitialized_blocks_double_init() {
    let pid          = program_id();
    let key          = Pubkey::new_unique();
    let mut data     = vec![0u8; RESERVE_DESTINATION_SIZE];
    let mut lamports = 1u64;

    {
        let ai = make_account_info(&key, &mut data, &mut lamports, &pid, true);
        let mut d  = ReserveDestination::load_uninitialized_mut(&ai).unwrap();

        d.discriminator = RESERVE_DESTINATION_DISCRIMINATOR;
    }
    {
        let ai = make_account_info(&key, &mut data, &mut lamports, &pid, true);

        assert!(matches!(
            ReserveDestination::load_uninitialized_mut(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::AlreadyInitialized as u32),
        ));
    }
}

#[test]
fn load_reads_fields_correctly() {
    let mint  = Pubkey::new_unique();
    let dest  = Pubkey::new_unique();
    let owner = Pubkey::new_unique();
    let (key, mut data, mut lamports) = make_destination(
        mint, dest, owner, destination_flag::TREASURY,
    );
    let pid   = program_id();
    let ai    = make_account_info(&key, &mut data, &mut lamports, &pid, false);
    let d     = ReserveDestination::load(&ai).unwrap();

    assert_eq!(d.asset_mint,                mint);
    assert_eq!(d.destination_token_account, dest);
    assert_eq!(d.destination_owner,         owner);
    assert_eq!(d.destination_flags & destination_flag::TREASURY, destination_flag::TREASURY);
}

// ─── assert_enabled ───────────────────────────────────────────────────────────

#[test]
fn assert_enabled_passes_for_enabled_status() {
    let mut d = ReserveDestination::zeroed();
    d.status = reserve_destination_status::ENABLED;

    assert!(d.assert_enabled().is_ok());
}

#[test]
fn assert_enabled_reports_disabled_status() {
    let mut d = ReserveDestination::zeroed();
    d.status = reserve_destination_status::DISABLED;

    assert!(matches!(
        d.assert_enabled(),
        Err(e) if e == ProgramError::Custom(ChanceryError::ReserveDestinationDisabled as u32),
    ));
}

#[test]
fn assert_enabled_reports_deprecated_status() {
    let mut d = ReserveDestination::zeroed();
    d.status = reserve_destination_status::DEPRECATED;

    assert!(matches!(
        d.assert_enabled(),
        Err(e) if e == ProgramError::Custom(ChanceryError::ReserveDestinationDeprecated as u32),
    ));
}

#[test]
fn assert_enabled_rejects_none_or_unknown_status() {
    let mut d = ReserveDestination::zeroed();
    d.status = reserve_destination_status::NONE;
    assert!(d.assert_enabled().is_err());

    d.status = u8::MAX;
    assert!(d.assert_enabled().is_err());
}

// ─── assert_asset ─────────────────────────────────────────────────────────────

#[test]
fn assert_asset_passes_when_mint_matches() {
    let mint  = Pubkey::new_unique();
    let mut d = ReserveDestination::zeroed();

    d.asset_mint = mint;

    assert!(d.assert_asset(&mint).is_ok());
}

#[test]
fn assert_asset_fails_when_mint_mismatches() {
    let mint  = Pubkey::new_unique();
    let other = Pubkey::new_unique();
    let mut d = ReserveDestination::zeroed();

    d.asset_mint = mint;

    assert!(matches!(
        d.assert_asset(&other),
        Err(e) if e == ProgramError::Custom(ChanceryError::ReserveDestinationAssetMismatch as u32),
    ));
}

#[test]
fn assert_asset_fails_for_default_pubkey_when_set_to_real_mint() {
    let mint  = Pubkey::new_unique();
    let mut d = ReserveDestination::zeroed();

    d.asset_mint = mint;

    assert!(d.assert_asset(&Pubkey::default()).is_err());
}

// ─── assert_destination_account ───────────────────────────────────────────────

#[test]
fn assert_destination_account_passes_when_account_matches() {
    let account = Pubkey::new_unique();
    let mut d   = ReserveDestination::zeroed();

    d.destination_token_account = account;

    assert!(d.assert_destination_account(&account).is_ok());
}

#[test]
fn assert_destination_account_fails_when_account_mismatches() {
    let account = Pubkey::new_unique();
    let other   = Pubkey::new_unique();
    let mut d   = ReserveDestination::zeroed();

    d.destination_token_account = account;

    assert!(matches!(
        d.assert_destination_account(&other),
        Err(e) if e == ProgramError::Custom(ChanceryError::ReserveDestinationNotApproved as u32),
    ));
}

#[test]
fn combined_guard_chain_all_pass() {
    let mint    = Pubkey::new_unique();
    let account = Pubkey::new_unique();
    let mut d   = ReserveDestination::zeroed();

    d.asset_mint                = mint;
    d.destination_token_account = account;
    d.status                    = reserve_destination_status::ENABLED;
    d.destination_flags         = destination_flag::TREASURY;

    assert!(d.assert_enabled().is_ok());
    assert!(d.assert_asset(&mint).is_ok());
    assert!(d.assert_destination_account(&account).is_ok());
}

#[test]
fn combined_guard_chain_fails_at_enabled_check_first() {
    let mint    = Pubkey::new_unique();
    let account = Pubkey::new_unique();
    let mut d   = ReserveDestination::zeroed();

    d.asset_mint                = mint;
    d.destination_token_account = account;
    d.status                    = reserve_destination_status::DISABLED;
    d.destination_flags         = destination_flag::TREASURY;

    assert!(d.assert_enabled().is_err(),
        "enabled check should fail even though asset and account match");
}

// ─── PDA derivation ───────────────────────────────────────────────────────────

#[test]
fn pda_is_deterministic() {
    let pid      = program_id();
    let mint     = Pubkey::new_unique();
    let dest     = Pubkey::new_unique();
    let (k1, b1) = ReserveDestination::pda(&mint, &dest, &pid);
    let (k2, b2) = ReserveDestination::pda(&mint, &dest, &pid);

    assert_eq!(k1, k2);
    assert_eq!(b1, b2);
}

#[test]
fn different_asset_mints_produce_different_pdas() {
    let pid     = program_id();
    let dest    = Pubkey::new_unique();
    let m1      = Pubkey::new_unique();
    let m2      = Pubkey::new_unique();
    let (k1, _) = ReserveDestination::pda(&m1, &dest, &pid);
    let (k2, _) = ReserveDestination::pda(&m2, &dest, &pid);

    assert_ne!(k1, k2);
}

#[test]
fn different_destination_accounts_produce_different_pdas() {
    let pid     = program_id();
    let mint    = Pubkey::new_unique();
    let d1      = Pubkey::new_unique();
    let d2      = Pubkey::new_unique();
    let (k1, _) = ReserveDestination::pda(&mint, &d1, &pid);
    let (k2, _) = ReserveDestination::pda(&mint, &d2, &pid);

    assert_ne!(k1, k2);
}

#[test]
fn pda_encodes_both_mint_and_destination_independently() {
    // Swapping mint and dest should produce a different PDA
    let pid     = program_id();
    let a       = Pubkey::new_unique();
    let b       = Pubkey::new_unique();
    let (k1, _) = ReserveDestination::pda(&a, &b, &pid);
    let (k2, _) = ReserveDestination::pda(&b, &a, &pid);

    assert_ne!(k1, k2, "PDA must not be symmetric with respect to mint and dest");
}
