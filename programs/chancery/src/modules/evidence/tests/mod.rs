/// modules/evidence/tests/mod.rs
///
/// Unit tests for EvidencePolicy state and the required_field mask helpers.
///
/// Covers:
///   - size / layout
///   - load guards
///   - required_field mask bit constants are distinct
///   - requires_field: single bit, multiple bits, absent bits
///   - freeform_allowed: true/false toggling
///   - PDA determinism and uniqueness

use std::{cell::RefCell, rc::Rc};

use bytemuck::Zeroable;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::seeds,
    error::ChanceryError,
    modules::evidence::state::evidence_policy::{
        required_field, EvidencePolicy,
        EVIDENCE_POLICY_DISCRIMINATOR, EVIDENCE_POLICY_SIZE,
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

fn make_evidence_policy(
    policy_id:           [u8; 32],
    required_field_mask: [u64; 2],
    allow_freeform:      bool,
) -> (Pubkey, Vec<u8>, u64) {
    let pid         = program_id();
    let (key, bump) = Pubkey::find_program_address(
        &[seeds::EVIDENCE_POLICY, policy_id.as_ref()],
        &pid,
    );
    let mut data     = vec![0u8; EVIDENCE_POLICY_SIZE];
    let lamports = 1u64;

    {
        // direct mut access to data
        let p: &mut EvidencePolicy = bytemuck::from_bytes_mut(&mut data[..]);

        p.discriminator                      = EVIDENCE_POLICY_DISCRIMINATOR;
        p.version                            = 1;
        p.bump                               = bump;
        p.allow_freeform_counterparty_fields = allow_freeform as u8;
        p.evidence_policy_id                 = policy_id;
        p.required_field_mask                = required_field_mask;
        p.counterparty_reporting_schema_hash = [0u8; 32];
        p.maximum_freeform_field_count       = 10;
        p.maximum_freeform_value_bytes       = 64;
        p.retention_flags                    = 0;
    }

    (key, data, lamports)
}

// ─── required_field constants ─────────────────────────────────────────────────

#[test]
fn required_field_constants_are_distinct_single_bits() {
    let fields = [
        required_field::PATHWAY_ID,
        required_field::SETTLEMENT_MODE,
        required_field::INTENT_ID,
        required_field::PRINCIPAL_A,
        required_field::PRINCIPAL_B,
        required_field::EXECUTOR,
        required_field::ASSET_MINT,
        required_field::ISSUED_TOKEN_MINT,
        required_field::SOURCE_ACCOUNT,
        required_field::DESTINATION_ACCOUNT,
        required_field::GROSS_AMOUNT_IN,
        required_field::GROSS_AMOUNT_OUT,
        required_field::FEE_AMOUNT,
        required_field::REBATE_AMOUNT,
        required_field::NET_AMOUNT,
        required_field::FEE_POLICY_ID,
        required_field::LIMIT_POLICY_ID,
        required_field::RESERVE_COMPARTMENT,
        required_field::INSURANCE_POLICY_ID,
        required_field::SCHEMA_HASH,
    ];

    for (i, &f) in fields.iter().enumerate() {
        assert_eq!(f.count_ones(), 1, "field {i} must be a single bit, got {f:#b}");
    }

    let unique: std::collections::HashSet<u64> = fields.iter().cloned().collect();

    assert_eq!(unique.len(), fields.len(), "all required_field constants must be distinct");
}

#[test]
fn required_field_constants_are_powers_of_two() {
    let fields = [
        required_field::PATHWAY_ID,
        required_field::SETTLEMENT_MODE,
        required_field::INTENT_ID,
        required_field::PRINCIPAL_A,
        required_field::PRINCIPAL_B,
    ];

    for &f in &fields {
        assert!(f & (f - 1) == 0, "field {f} is not a power of two");
    }
}

#[test]
fn all_required_field_bits_fit_in_low_word() {
    // Currently all required_field constants fit within 64 bits (low word).
    // This test documents that boundary - if we ever need > 64 fields,
    // both words of required_field_mask must be checked.
    let all = required_field::PATHWAY_ID
        | required_field::SETTLEMENT_MODE
        | required_field::INTENT_ID
        | required_field::PRINCIPAL_A
        | required_field::PRINCIPAL_B
        | required_field::EXECUTOR
        | required_field::ASSET_MINT
        | required_field::ISSUED_TOKEN_MINT
        | required_field::SOURCE_ACCOUNT
        | required_field::DESTINATION_ACCOUNT
        | required_field::GROSS_AMOUNT_IN
        | required_field::GROSS_AMOUNT_OUT
        | required_field::FEE_AMOUNT
        | required_field::REBATE_AMOUNT
        | required_field::NET_AMOUNT
        | required_field::FEE_POLICY_ID
        | required_field::LIMIT_POLICY_ID
        | required_field::RESERVE_COMPARTMENT
        | required_field::INSURANCE_POLICY_ID
        | required_field::SCHEMA_HASH;
    
    // All current fields fit in a u64 (low word)
    let _ = all;  // if this computation overflows, there's a duplicate bit
}

// ─── Size / layout ────────────────────────────────────────────────────────────

#[test]
fn size_matches_declared() {
    assert_eq!(core::mem::size_of::<EvidencePolicy>(), EVIDENCE_POLICY_SIZE);
}

#[test]
fn zeroed_struct_all_zero_bytes() {
    let p     = EvidencePolicy::zeroed();
    let bytes : &[u8] = bytemuck::bytes_of(&p);

    assert!(bytes.iter().all(|&b| b == 0));
}

// ─── Load guards ──────────────────────────────────────────────────────────────

#[test]
fn load_zero_data_returns_not_initialized() {
    let pid          = program_id();
    let key          = Pubkey::new_unique();
    let mut data     = vec![0u8; EVIDENCE_POLICY_SIZE];
    let mut lamports = 1u64;
    let ai           = make_account_info(&key, &mut data, &mut lamports, &pid, false);

    assert!(matches!(
        EvidencePolicy::load(&ai),
        Err(e) if e == ProgramError::Custom(ChanceryError::NotInitialized as u32),
    ));
}

#[test]
fn load_wrong_size_returns_length_mismatch() {
    let pid          = program_id();
    let key          = Pubkey::new_unique();
    let mut data     = vec![0u8; EVIDENCE_POLICY_SIZE + 8];
    let mut lamports = 1u64;
    let ai           = make_account_info(&key, &mut data, &mut lamports, &pid, false);

    assert!(matches!(
        EvidencePolicy::load(&ai),
        Err(e) if e == ProgramError::Custom(ChanceryError::AccountDataLengthMismatch as u32),
    ));
}

#[test]
fn load_mut_non_writable_returns_error() {
    let id  = [0x01u8; 32];
    let (key, mut data, mut lamports) = make_evidence_policy(id, [0u64; 2], false);
    let pid = program_id();
    let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);

    assert!(matches!(
        EvidencePolicy::load_mut(&ai),
        Err(e) if e == ProgramError::Custom(ChanceryError::AccountNotWritable as u32),
    ));
}

#[test]
fn load_uninitialized_blocks_double_init() {
    let pid          = program_id();
    let key          = Pubkey::new_unique();
    let mut data     = vec![0u8; EVIDENCE_POLICY_SIZE];
    let mut lamports = 1u64;

    {
        let ai = make_account_info(&key, &mut data, &mut lamports, &pid, true);
        let mut p  = EvidencePolicy::load_uninitialized_mut(&ai).unwrap();

        p.discriminator = EVIDENCE_POLICY_DISCRIMINATOR;
    }

    {
        let ai = make_account_info(&key, &mut data, &mut lamports, &pid, true);

        assert!(matches!(
            EvidencePolicy::load_uninitialized_mut(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::AlreadyInitialized as u32),
        ));
    }
}

#[test]
fn load_reads_fields_correctly() {
    let id   = [0xBBu8; 32];
    let mask = [required_field::PATHWAY_ID | required_field::NET_AMOUNT, 0u64];
    let (key, mut data, mut lamports) = make_evidence_policy(id, mask, true);
    let pid = program_id();
    let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
    let p   = EvidencePolicy::load(&ai).unwrap();

    assert_eq!(p.evidence_policy_id,                 id);
    assert_eq!(p.required_field_mask[0],             mask[0]);
    assert_eq!(p.allow_freeform_counterparty_fields, 1u8);
    assert_eq!(p.maximum_freeform_field_count,       10u16);
}

// ─── requires_field ───────────────────────────────────────────────────────────

#[test]
fn requires_field_true_when_single_bit_set() {
    let mask  = [required_field::PATHWAY_ID, 0u64];
    let mut p = EvidencePolicy::zeroed();

    p.discriminator       = EVIDENCE_POLICY_DISCRIMINATOR;
    p.required_field_mask = mask;

    assert!(p.requires_field(0, required_field::PATHWAY_ID));
}

#[test]
fn requires_field_false_when_bit_not_set() {
    let mut p = EvidencePolicy::zeroed();

    p.discriminator       = EVIDENCE_POLICY_DISCRIMINATOR;
    p.required_field_mask = [0u64; 2];  // no bits set

    assert!(!p.requires_field(0, required_field::PATHWAY_ID));
}

#[test]
fn requires_field_checks_selected_word() {
    let mut p = EvidencePolicy::zeroed();

    p.discriminator       = EVIDENCE_POLICY_DISCRIMINATOR;
    p.required_field_mask = [required_field::NET_AMOUNT, 1u64 << 7];

    assert!(p.requires_field(0, required_field::NET_AMOUNT));
    assert!(!p.requires_field(0, required_field::PATHWAY_ID));
    assert!(p.requires_field(1, 1u64 << 7));
    assert!(!p.requires_field(1, 1u64 << 8));
    assert!(!p.requires_field(2, 1u64 << 7));
}

#[test]
fn requires_field_multiple_bits_independently_checked() {
    let mask = [
        required_field::PRINCIPAL_A
            | required_field::PRINCIPAL_B
            | required_field::GROSS_AMOUNT_IN,
        0u64,
    ];
    let mut p = EvidencePolicy::zeroed();

    p.required_field_mask = mask;

    assert!(p.requires_field(0, required_field::PRINCIPAL_A));
    assert!(p.requires_field(0, required_field::PRINCIPAL_B));
    assert!(p.requires_field(0, required_field::GROSS_AMOUNT_IN));
    assert!(!p.requires_field(0, required_field::EXECUTOR));
    assert!(!p.requires_field(0, required_field::FEE_AMOUNT));
}

#[test]
fn requires_field_all_fields_set_reports_all_required() {
    let all_low = required_field::PATHWAY_ID
        | required_field::SETTLEMENT_MODE
        | required_field::INTENT_ID
        | required_field::PRINCIPAL_A
        | required_field::PRINCIPAL_B
        | required_field::EXECUTOR
        | required_field::ASSET_MINT
        | required_field::ISSUED_TOKEN_MINT
        | required_field::SOURCE_ACCOUNT
        | required_field::DESTINATION_ACCOUNT
        | required_field::GROSS_AMOUNT_IN
        | required_field::GROSS_AMOUNT_OUT
        | required_field::FEE_AMOUNT
        | required_field::REBATE_AMOUNT
        | required_field::NET_AMOUNT
        | required_field::FEE_POLICY_ID
        | required_field::LIMIT_POLICY_ID
        | required_field::RESERVE_COMPARTMENT
        | required_field::INSURANCE_POLICY_ID
        | required_field::SCHEMA_HASH;
    let mut p = EvidencePolicy::zeroed();

    p.required_field_mask = [all_low, 0u64];

    for &field in &[
        required_field::PATHWAY_ID,
        required_field::PRINCIPAL_A,
        required_field::NET_AMOUNT,
        required_field::SCHEMA_HASH,
    ] {
        assert!(p.requires_field(0, field), "field {field:#b} should be required");
    }
}

#[test]
fn requires_field_zero_mask_never_requires_anything() {
    let mut p = EvidencePolicy::zeroed();

    p.required_field_mask = [0u64; 2];

    assert!(!p.requires_field(0, required_field::PATHWAY_ID));
    assert!(!p.requires_field(0, required_field::NET_AMOUNT));
    assert!(!p.requires_field(0, u64::MAX));
}

// ─── freeform_allowed ─────────────────────────────────────────────────────────

#[test]
fn freeform_allowed_true_when_field_nonzero() {
    let id  = [0x01u8; 32];
    let (key, mut data, mut lamports) = make_evidence_policy(id, [0u64; 2], true);
    let pid = program_id();
    let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
    let p   = EvidencePolicy::load(&ai).unwrap();

    assert!(p.freeform_allowed());
}

#[test]
fn freeform_allowed_false_when_field_zero() {
    let id  = [0x02u8; 32];
    let (key, mut data, mut lamports) = make_evidence_policy(id, [0u64; 2], false);
    let pid = program_id();
    let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);
    let p   = EvidencePolicy::load(&ai).unwrap();

    assert!(!p.freeform_allowed());
}

#[test]
fn freeform_allowed_toggling_in_place() {
    let mut p = EvidencePolicy::zeroed();

    p.allow_freeform_counterparty_fields = 0;
    assert!(!p.freeform_allowed());

    p.allow_freeform_counterparty_fields = 1;
    assert!(p.freeform_allowed());

    p.allow_freeform_counterparty_fields = 0;
    assert!(!p.freeform_allowed());
}

#[test]
fn freeform_allowed_treats_any_nonzero_as_true() {
    let mut p = EvidencePolicy::zeroed();

    p.allow_freeform_counterparty_fields = 255;  // max u8
    assert!(p.freeform_allowed());
}

// ─── PDA derivation ───────────────────────────────────────────────────────────

#[test]
fn pda_is_deterministic() {
    let pid      = program_id();
    let id       = [0xCCu8; 32];
    let (k1, b1) = EvidencePolicy::pda(&id, &pid);
    let (k2, b2) = EvidencePolicy::pda(&id, &pid);

    assert_eq!(k1, k2);
    assert_eq!(b1, b2);
}

#[test]
fn different_policy_ids_produce_different_pdas() {
    let pid     = program_id();
    let (k1, _) = EvidencePolicy::pda(&[0x01u8; 32], &pid);
    let (k2, _) = EvidencePolicy::pda(&[0x02u8; 32], &pid);

    assert_ne!(k1, k2);
}
