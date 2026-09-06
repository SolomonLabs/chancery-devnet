/// modules/fees/tests/mod.rs
///
/// Unit tests for FeePolicy state and its fee-computation methods.
/// Covers every branch of the §4.4 fee application order:
///   1. raw fee (flat | basis points | zero)
///   2. cap
///   3. minimum
///   4. rebate (flat | basis points)
///   5. rebate cap
///   6. net fee floor
///   7. net output zero guard
///
/// All tests are pure - no CPI, no runtime.

use bytemuck::Zeroable;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::fee_recipient,
    error::ChanceryError,
    modules::fees::state::fee_policy::{
        fee_flag, FeePolicy, FEE_POLICY_DISCRIMINATOR, FEE_POLICY_SIZE,
    },
};

// ─── Helper: build a FeePolicy value directly (not an account) ───────────────

fn build_fee_policy(
    flat_issued_token: u64,
    bps:               u32,
    cap:               u64,
    minimum_fee:       u64,
    rebate_flat:       u64,
    rebate_bps:        u32,
    rebate_cap:        u64,
    floor_zero:        bool,
) -> FeePolicy {
    let mut fp = FeePolicy::zeroed();

    fp.discriminator                  = FEE_POLICY_DISCRIMINATOR;
    fp.version                        = 1;
    fp.flat_fee_in_issued_token       = flat_issued_token;
    fp.percent_fee_bps                = bps;
    fp.fee_cap_amount                 = cap;
    fp.minimum_fee_amount             = minimum_fee;
    fp.rebate_flat_amount             = rebate_flat;
    fp.rebate_bps                     = rebate_bps;
    fp.rebate_cap_amount              = rebate_cap;
    fp.net_fee_floor_zero             = floor_zero as u8;
    fp.effective_from_unix_timestamp  = 0;
    fp.effective_until_unix_timestamp = 0;
    // Helper configures an issued-token-denominated active policy. Rebate
    // fields require the principal-side rebate semantic bit to match them.
    fp.fee_policy_flags = fee_flag::ACTIVE | fee_flag::FEE_IN_ISSUED_TOKEN;
    if rebate_flat != 0 || rebate_bps != 0 {
        fp.fee_policy_flags |= fee_flag::REBATE_TO_PRINCIPAL;
    }

    fp
}

// ─── Size and layout ──────────────────────────────────────────────────────────

#[test]
fn size_matches_declared() {
    assert_eq!(core::mem::size_of::<FeePolicy>(), FEE_POLICY_SIZE);
}

#[test]
fn zeroed_struct_has_all_zero_bytes() {
    let fp    = FeePolicy::zeroed();
    let bytes : &[u8] = bytemuck::bytes_of(&fp);

    assert!(bytes.iter().all(|&b| b == 0));
}

// ─── No-fee baseline ──────────────────────────────────────────────────────────

#[test]
fn no_fee_returns_gross_amount_unchanged() {
    let fp = build_fee_policy(0, 0, 0, 0, 0, 0, 0, true);
    let (fee, rebate, net) = fp.compute_issued_token_fee(1_000_000).unwrap();

    assert_eq!(fee,    0,         "fee must be 0 when no fee configured");
    assert_eq!(rebate, 0,         "rebate must be 0 when no fee configured");
    assert_eq!(net,    1_000_000, "net must equal gross when no fee");
}

#[test]
fn no_fee_small_amount_returns_unchanged() {
    let fp = build_fee_policy(0, 0, 0, 0, 0, 0, 0, true);
    let (fee, rebate, net) = fp.compute_issued_token_fee(1).unwrap();

    assert_eq!(fee,    0);
    assert_eq!(rebate, 0);
    assert_eq!(net,    1);
}

// ─── Flat fee ─────────────────────────────────────────────────────────────────

#[test]
fn flat_fee_deducted_from_gross() {
    let fp = build_fee_policy(1_000, 0, 0, 0, 0, 0, 0, true);
    let (fee, rebate, net) = fp.compute_issued_token_fee(100_000).unwrap();

    assert_eq!(fee,    1_000);
    assert_eq!(rebate, 0);
    assert_eq!(net,    99_000);
}

#[test]
fn flat_fee_equal_to_gross_returns_net_output_zero_error() {
    let fp = build_fee_policy(100_000, 0, 0, 0, 0, 0, 0, true);

    assert!(matches!(
        fp.compute_issued_token_fee(100_000),
        Err(e) if e == ProgramError::Custom(ChanceryError::NetOutputZero as u32),
    ));
}

#[test]
fn flat_fee_larger_than_gross_returns_net_output_zero_error() {
    // flat_fee > gross -> net would go negative -> error
    let fp = build_fee_policy(200_000, 0, 0, 0, 0, 0, 0, true);

    assert!(matches!(
        fp.compute_issued_token_fee(100_000),
        Err(e) if e == ProgramError::Custom(ChanceryError::NetOutputZero as u32),
    ));
}

#[test]
fn flat_fee_in_asset_flag_selects_asset_field() {
    // fee_policy_flags = FEE_IN_ASSET; flat_fee_in_asset set, issued token not set
    let mut fp = build_fee_policy(0, 0, 0, 0, 0, 0, 0, true);

    fp.flat_fee_in_asset = 500;
    fp.fee_policy_flags  = fee_flag::FEE_IN_ASSET;

    let (fee, _, net) = fp.compute_asset_fee(10_000).unwrap();

    assert_eq!(fee, 500, "FEE_IN_ASSET flag must select flat_fee_in_asset field");
    assert_eq!(net, 9_500);
}

// ─── Percentage fee ───────────────────────────────────────────────────────────

#[test]
fn bps_10000_charges_full_gross() {
    // 10_000 basis points = 100% of gross -> net = 0 -> error
    let fp = build_fee_policy(0, 10_000, 0, 0, 0, 0, 0, true);

    assert!(matches!(
        fp.compute_issued_token_fee(1_000_000),
        Err(e) if e == ProgramError::Custom(ChanceryError::NetOutputZero as u32),
    ));
}

#[test]
fn bps_20_charges_correct_amount() {
    // 20 basis points of 1_000_000 = 2_000
    let fp            = build_fee_policy(0, 20, 0, 0, 0, 0, 0, true);
    let (fee, _, net) = fp.compute_issued_token_fee(1_000_000).unwrap();

    assert_eq!(fee, 2_000);
    assert_eq!(net, 998_000);
}

#[test]
fn bps_50_of_200_is_1() {
    // 50 basis points of 200 = 1.0 -> 1
    let fp            = build_fee_policy(0, 50, 0, 0, 0, 0, 0, true);
    let (fee, _, net) = fp.compute_issued_token_fee(200).unwrap();

    assert_eq!(fee, 1);
    assert_eq!(net, 199);
}

#[test]
fn bps_truncates_toward_zero() {
    // 1 basis points of 99 = 0.0099 -> truncated to 0
    let fp            = build_fee_policy(0, 1, 0, 0, 0, 0, 0, true);
    let (fee, _, net) = fp.compute_issued_token_fee(99).unwrap();

    assert_eq!(fee, 0, "bps should truncate toward zero, not round up");
    assert_eq!(net, 99);
}

#[test]
fn bps_9999_leaves_minimum_output() {
    // 9_999 basis points of 10_000 = 9_999 -> net = 1
    let fp            = build_fee_policy(0, 9_999, 0, 0, 0, 0, 0, true);
    let (fee, _, net) = fp.compute_issued_token_fee(10_000).unwrap();

    assert_eq!(fee, 9_999);
    assert_eq!(net, 1);
}

// ─── Fee cap ──────────────────────────────────────────────────────────────────

#[test]
fn fee_cap_limits_bps_fee() {
    // 1_000 basis points of 1_000_000 = 100_000 but capped at 500
    let fp            = build_fee_policy(0, 1_000, 500, 0, 0, 0, 0, true);
    let (fee, _, net) = fp.compute_issued_token_fee(1_000_000).unwrap();

    assert_eq!(fee, 500, "cap must override bps calculation");
    assert_eq!(net, 999_500);
}

#[test]
fn fee_cap_limits_flat_fee() {
    // flat = 10_000, cap = 100
    let fp          = build_fee_policy(10_000, 0, 100, 0, 0, 0, 0, true);
    let (fee, _, _) = fp.compute_issued_token_fee(1_000_000).unwrap();

    assert_eq!(fee, 100);
}

#[test]
fn fee_cap_not_applied_when_fee_below_cap() {
    // 20 basis points of 1_000_000 = 2_000; cap = 5_000 -> cap not triggered
    let fp          = build_fee_policy(0, 20, 5_000, 0, 0, 0, 0, true);
    let (fee, _, _) = fp.compute_issued_token_fee(1_000_000).unwrap();

    assert_eq!(fee, 2_000, "cap must not reduce fee below raw amount");
}

#[test]
fn zero_cap_means_no_cap() {
    // cap = 0 -> disabled -> full basis points fee applies
    let fp          = build_fee_policy(0, 1_000, 0, 0, 0, 0, 0, true);
    let (fee, _, _) = fp.compute_issued_token_fee(1_000_000).unwrap();

    assert_eq!(fee, 100_000, "zero cap must mean no cap");
}

// ─── Minimum fee ──────────────────────────────────────────────────────────────

#[test]
fn minimum_fee_enforced_when_bps_fee_is_below_minimum() {
    // 1 basis points of 100 = 0 (truncates) but minimum_fee = 10 -> fee = 10
    let fp          = build_fee_policy(0, 1, 0, 10, 0, 0, 0, true);
    let (fee, _, _) = fp.compute_issued_token_fee(100).unwrap();

    assert_eq!(fee, 10, "minimum_fee must be enforced when bps fee truncates to below min");
}

#[test]
fn minimum_fee_not_applied_when_fee_exceeds_minimum() {
    // 100 basis points of 1_000 = 10; min = 5 -> fee stays 10
    let fp          = build_fee_policy(0, 100, 0, 5, 0, 0, 0, true);
    let (fee, _, _) = fp.compute_issued_token_fee(1_000).unwrap();

    assert_eq!(fee, 10, "minimum_fee must not increase fee already above minimum");
}

#[test]
fn zero_minimum_means_no_minimum() {
    // 1 basis points of 50 = 0; minimum_fee = 0 -> fee stays 0
    let fp          = build_fee_policy(0, 1, 0, 0, 0, 0, 0, true);
    let (fee, _, _) = fp.compute_issued_token_fee(50).unwrap();

    assert_eq!(fee, 0, "zero minimum_fee must mean no minimum");
}

#[test]
fn cap_applies_before_minimum_when_both_set() {
    // raw basis points = 1_000, cap = 200, min = 100
    // after cap: fee = 200 (> min 100) -> min not applied
    let fp          = build_fee_policy(0, 1_000, 200, 100, 0, 0, 0, true);
    let (fee, _, _) = fp.compute_issued_token_fee(1_000_000).unwrap();

    assert_eq!(fee, 200);
}

// ─── Flat rebate ──────────────────────────────────────────────────────────────

#[test]
fn flat_rebate_reduces_net_fee() {
    // flat fee = 1_000, flat rebate = 400 -> net_fee = 600
    let fp                 = build_fee_policy(1_000, 0, 0, 0, 400, 0, 0, false);
    let (fee, rebate, net) = fp.compute_issued_token_fee(100_000).unwrap();

    assert_eq!(fee,    1_000);
    assert_eq!(rebate, 400);
    assert_eq!(net,    99_400);  // gross - (fee - rebate)
}

#[test]
fn flat_rebate_equal_to_fee_yields_zero_net_fee() {
    // fee = 500, rebate = 500 -> net_fee = 0 -> net output = gross
    let fp                 = build_fee_policy(500, 0, 0, 0, 500, 0, 0, false);
    let (fee, rebate, net) = fp.compute_issued_token_fee(100_000).unwrap();

    assert_eq!(fee,    500);
    assert_eq!(rebate, 500);
    assert_eq!(net,    100_000, "zero net fee must return full gross output");
}

// ─── BPS rebate ───────────────────────────────────────────────────────────────

#[test]
fn bps_rebate_of_fee_amount() {
    // 100 basis points fee of 1_000_000 = 10_000; 5_000 basis points rebate of 10_000 = 5_000
    let fp                 = build_fee_policy(0, 100, 0, 0, 0, 5_000, 0, false);
    let (fee, rebate, net) = fp.compute_issued_token_fee(1_000_000).unwrap();

    assert_eq!(fee,    10_000);
    assert_eq!(rebate, 5_000);
    assert_eq!(net,    995_000);
}

#[test]
fn bps_rebate_truncates_toward_zero() {
    // fee = 9, rebate_basis points = 1 -> 1/10_000 of 9 = 0.0009 -> 0
    let fp               = build_fee_policy(9, 0, 0, 0, 0, 1, 0, false);
    let (fee, rebate, _) = fp.compute_issued_token_fee(1_000_000).unwrap();

    assert_eq!(fee,    9);
    assert_eq!(rebate, 0);
}

// ─── Rebate cap ───────────────────────────────────────────────────────────────

#[test]
fn rebate_cap_limits_bps_rebate() {
    // 10_000 basis points rebate (full fee back) of fee=10_000 = 10_000 but capped at 50
    let fp               = build_fee_policy(0, 100, 0, 0, 0, 10_000, 50, false);
    let (fee, rebate, _) = fp.compute_issued_token_fee(1_000_000).unwrap();

    assert_eq!(fee,    10_000);
    assert_eq!(rebate, 50, "rebate cap must limit bps rebate");
}

#[test]
fn rebate_cap_limits_flat_rebate() {
    // flat_rebate = 1_000, rebate_cap = 100
    let fp             = build_fee_policy(2_000, 0, 0, 0, 1_000, 0, 100, false);
    let (_, rebate, _) = fp.compute_issued_token_fee(100_000).unwrap();

    assert_eq!(rebate, 100, "rebate cap must limit flat rebate");
}

#[test]
fn zero_rebate_cap_means_no_cap() {
    // rebate_cap = 0 -> disabled; full rebate applies
    let fp               = build_fee_policy(0, 100, 0, 0, 0, 10_000, 0, false);
    let (fee, rebate, _) = fp.compute_issued_token_fee(1_000_000).unwrap();

    assert_eq!(fee, rebate, "full rebate should match fee when cap=0 and rebate_bps=10_000");
}

// ─── Net fee floor ────────────────────────────────────────────────────────────

#[test]
fn floor_zero_prevents_negative_net_fee() {
    // flat_fee = 100, flat_rebate = 200 -> nominal rebate > fee
    // floor_zero = true -> effective rebate = fee and net_fee = 0
    let fp = build_fee_policy(100, 0, 0, 0, 200, 0, 0, true);
    let computation = fp
        .compute_issued_token_fee_with_input(100_000, 100_000)
        .unwrap();

    assert!(
        computation.nominal_rebate > computation.assessed_fee,
        "configured rebate must exceed the assessed fee for this test",
    );
    assert_eq!(computation.effective_rebate, computation.assessed_fee);
    assert_eq!(computation.net_fee, 0);
    assert_eq!(
        computation.net_output,
        100_000,
        "floor_zero must prevent a negative net fee",
    );
}

#[test]
fn no_floor_allows_rebate_to_exceed_fee_and_reduce_net_fee() {
    // flat_fee = 100, flat_rebate = 200, floor_zero = false
    // net_fee = 100 - 200 = -100 -> checked_sub fails -> ArithmeticUnderflow
    let fp     = build_fee_policy(100, 0, 0, 0, 200, 0, 0, false);
    let result = fp.compute_issued_token_fee(100_000);

    // Without floor_zero this should fail with underflow
    assert!(result.is_err(), "without floor_zero, rebate > fee should error");
}

#[test]
fn floor_zero_with_equal_fee_and_rebate_returns_gross() {
    let fp          = build_fee_policy(1_000, 0, 0, 0, 1_000, 0, 0, true);
    let (_, _, net) = fp.compute_issued_token_fee(50_000).unwrap();

    assert_eq!(net, 50_000);
}

// ─── NetOutputZero guard ──────────────────────────────────────────────────────

#[test]
fn net_output_of_one_passes() {
    // fee = gross - 1; net = 1
    let gross: u64  = 10_000;
    let fp          = build_fee_policy(gross - 1, 0, 0, 0, 0, 0, 0, true);
    let (_, _, net) = fp.compute_issued_token_fee(gross).unwrap();

    assert_eq!(net, 1);
}

#[test]
fn net_output_zero_from_flat_fee_returns_error() {
    let fp = build_fee_policy(100_000, 0, 0, 0, 0, 0, 0, true);

    assert!(matches!(
        fp.compute_issued_token_fee(100_000),
        Err(e) if e == ProgramError::Custom(ChanceryError::NetOutputZero as u32),
    ));
}

#[test]
fn net_output_zero_from_bps_fee_returns_error() {
    let fp = build_fee_policy(0, 10_000, 0, 0, 0, 0, 0, true);  // 100% fee

    assert!(matches!(
        fp.compute_issued_token_fee(100_000),
        Err(e) if e == ProgramError::Custom(ChanceryError::NetOutputZero as u32),
    ));
}

// ─── Interaction: cap + min together ─────────────────────────────────────────

#[test]
fn minimum_above_nonzero_cap_is_rejected() {
    let fp = build_fee_policy(0, 5_000, 10, 50, 0, 0, 0, true);

    assert!(matches!(
        fp.compute_issued_token_fee(10_000),
        Err(e) if e == ProgramError::Custom(ChanceryError::FeePolicyInvalidParameters as u32),
    ));
}

// ─── assert_effective ─────────────────────────────────────────────────────────

#[test]
fn assert_effective_passes_when_timestamps_zero() {
    let fp = build_fee_policy(0, 0, 0, 0, 0, 0, 0, true);

    assert!(fp.assert_effective(i64::MAX).is_ok());
    assert!(fp.assert_effective(0).is_ok());
    assert!(fp.assert_effective(-1_000).is_ok());
}

#[test]
fn assert_effective_fails_before_effective_from() {
    let mut fp = build_fee_policy(0, 100, 0, 0, 0, 0, 0, true);

    fp.effective_from_unix_timestamp = 1_000;

    assert!(matches!(
        fp.assert_effective(999),
        Err(e) if e == ProgramError::Custom(ChanceryError::FeePolicyNotFound as u32),
    ));
}

#[test]
fn assert_effective_passes_at_effective_from_timestamp() {
    let mut fp = build_fee_policy(0, 100, 0, 0, 0, 0, 0, true);

    fp.effective_from_unix_timestamp = 1_000;

    assert!(fp.assert_effective(1_000).is_ok());
}

#[test]
fn assert_effective_fails_at_effective_until_timestamp() {
    let mut fp = build_fee_policy(0, 100, 0, 0, 0, 0, 0, true);

    fp.effective_until_unix_timestamp = 2_000;

    assert!(matches!(
        fp.assert_effective(2_000),
        Err(e) if e == ProgramError::Custom(ChanceryError::FeePolicyExpired as u32),
    ));
}

#[test]
fn assert_effective_passes_within_window() {
    let mut fp = build_fee_policy(0, 100, 0, 0, 0, 0, 0, true);

    fp.effective_from_unix_timestamp  = 1_000;
    fp.effective_until_unix_timestamp = 2_000;

    assert!(fp.assert_effective(1_500).is_ok());
}

#[test]
fn assert_effective_zero_until_means_no_expiry() {
    let mut fp = build_fee_policy(0, 100, 0, 0, 0, 0, 0, true);

    fp.effective_from_unix_timestamp  = 0;
    fp.effective_until_unix_timestamp = 0;

    assert!(fp.assert_effective(i64::MAX).is_ok());
}

// ─── assert_parameter_sanity / ACTIVE ────────────────────────────────────────

#[test]
fn assert_effective_fails_when_active_bit_clear() {
    let mut fp = build_fee_policy(0, 100, 0, 0, 0, 0, 0, true);

    fp.fee_policy_flags &= !fee_flag::ACTIVE;

    assert!(matches!(
        fp.assert_effective(0),
        Err(e) if e == ProgramError::Custom(ChanceryError::FeePolicyNotFound as u32),
    ));
}

#[test]
fn parameter_sanity_rejects_percent_bps_above_max() {
    let fp = build_fee_policy(0, 10_001, 0, 0, 0, 0, 0, true);

    assert!(matches!(
        fp.assert_parameter_sanity(),
        Err(e) if e == ProgramError::Custom(ChanceryError::FeePolicyInvalidParameters as u32),
    ));
}

#[test]
fn parameter_sanity_rejects_asset_flat_without_flag() {
    let mut fp = build_fee_policy(0, 100, 0, 0, 0, 0, 0, true);

    fp.flat_fee_in_asset = 500;

    assert!(matches!(
        fp.assert_parameter_sanity(),
        Err(e) if e == ProgramError::Custom(ChanceryError::FeePolicyInvalidParameters as u32),
    ));
}

// ─── has_fee / fee_in_asset helpers ──────────────────────────────────────────

#[test]
fn has_fee_false_when_all_zero() {
    let fp = build_fee_policy(0, 0, 0, 0, 0, 0, 0, true);

    assert!(!fp.has_fee());
}

#[test]
fn has_fee_true_when_flat_issued_token_set() {
    let fp = build_fee_policy(1, 0, 0, 0, 0, 0, 0, true);

    assert!(fp.has_fee());
}

#[test]
fn has_fee_true_when_bps_set() {
    let fp = build_fee_policy(0, 1, 0, 0, 0, 0, 0, true);

    assert!(fp.has_fee());
}

#[test]
fn has_fee_true_when_flat_asset_set() {
    let mut fp = build_fee_policy(0, 0, 0, 0, 0, 0, 0, true);

    fp.flat_fee_in_asset = 1;
    fp.fee_policy_flags  = fee_flag::FEE_IN_ASSET;

    assert!(fp.has_fee());
}

#[test]
fn fee_in_asset_flag_reflects_policy_flags_bit() {
    let mut fp = build_fee_policy(0, 0, 0, 0, 0, 0, 0, true);

    assert!(!fp.fee_in_asset());

    fp.fee_policy_flags = fee_flag::FEE_IN_ASSET;

    assert!(fp.fee_in_asset());
}

#[test]
fn none_and_reserve_retention_never_route() {
    let mut fp = build_fee_policy(0, 100, 0, 0, 0, 0, 0, true);

    fp.fee_recipient_policy = fee_recipient::NONE;
    assert!(fp.retains_fee());
    assert!(!fp.routes_fee_to_recipient());

    fp.fee_recipient_policy = fee_recipient::RESERVE_RETENTION;
    assert!(fp.retains_fee());
    assert!(!fp.routes_fee_to_recipient());
}

#[test]
fn external_recipient_policies_route() {
    let mut fp = build_fee_policy(0, 100, 0, 0, 0, 0, 0, true);

    fp.fee_recipient_policy = fee_recipient::PROTOCOL_TREASURY;
    assert!(!fp.retains_fee());
    assert!(fp.routes_fee_to_recipient());
}

#[test]
fn parameter_sanity_accepts_retention_with_zero_recipient_key() {
    for recipient_policy in [fee_recipient::NONE, fee_recipient::RESERVE_RETENTION] {
        let mut fp = build_fee_policy(0, 100, 0, 0, 0, 0, 0, true);
        fp.fee_recipient_policy = recipient_policy;
        fp.fee_recipient_key = Pubkey::default();

        assert!(fp.assert_parameter_sanity().is_ok());
    }
}

#[test]
fn parameter_sanity_accepts_routed_policy_with_recipient_key() {
    let mut fp = build_fee_policy(0, 100, 0, 0, 0, 0, 0, true);
    fp.fee_recipient_policy = fee_recipient::PROTOCOL_TREASURY;
    fp.fee_recipient_key = Pubkey::new_unique();

    assert!(fp.assert_parameter_sanity().is_ok());
}

#[test]
fn parameter_sanity_rejects_recipient_key_on_retention_policy() {
    let mut fp = build_fee_policy(0, 100, 0, 0, 0, 0, 0, true);
    fp.fee_recipient_policy = fee_recipient::RESERVE_RETENTION;
    fp.fee_recipient_key = Pubkey::new_unique();

    assert!(matches!(
        fp.assert_parameter_sanity(),
        Err(e) if e == ProgramError::Custom(ChanceryError::FeePolicyInvalidParameters as u32),
    ));
}

#[test]
fn parameter_sanity_rejects_missing_key_on_routed_policy() {
    let mut fp = build_fee_policy(0, 100, 0, 0, 0, 0, 0, true);
    fp.fee_recipient_policy = fee_recipient::PROTOCOL_TREASURY;
    fp.fee_recipient_key = Pubkey::default();

    assert!(matches!(
        fp.assert_parameter_sanity(),
        Err(e) if e == ProgramError::Custom(ChanceryError::FeePolicyInvalidParameters as u32),
    ));
}

// ─── Large amount stress ──────────────────────────────────────────────────────

#[test]
fn large_gross_with_bps_fee_no_overflow() {
    // 1 basis points of near-u64-max / 2 - must not overflow u128 intermediate
    let fp            = build_fee_policy(0, 1, 0, 0, 0, 0, 0, true);
    let gross         = u32::MAX as u64 * 1_000_000;
    let result        = fp.compute_issued_token_fee(gross);

    assert!(result.is_ok(), "large amounts must not overflow with bps fee");

    let (fee, _, net) = result.unwrap();

    assert!(fee < gross);
    assert_eq!(fee + net, gross);
}
