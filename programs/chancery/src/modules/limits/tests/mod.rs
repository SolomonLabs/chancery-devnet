/// modules/limits/tests/mod.rs
///
/// Unit tests for LimitPolicy guard methods and UsageWindow accumulation.
/// Covers:
///   - per-tx cap (zero = disabled, exact, over)
///   - fixed-window volume caps (hourly, daily, weekly, monthly)
///   - action count caps
///   - UsageWindow record_inflow / record_outflow accumulation
///   - net_flow sign tracking
///   - window boundary helpers (hourly, daily, weekly, monthly floor)
///   - word-level u128/i128 encoding helpers
///
/// All tests are pure - no CPI, no runtime.

use bytemuck::Zeroable;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::{scope, window_kind},
    error::ChanceryError,
    modules::limits::state::{
        limit_policy::{LimitPolicy, LIMIT_POLICY_DISCRIMINATOR, LIMIT_POLICY_SIZE},
        usage_window::{
            canonical_window_start, daily_window_start, hourly_window_start,
            i128_to_words, monthly_window_start, u128_to_words, weekly_window_start,
            words_to_i128, words_to_u128, UsageWindow, USAGE_WINDOW_DISCRIMINATOR,
            USAGE_WINDOW_SIZE,
        },
    },
};

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn build_limit_policy(
    per_tx:           u64,
    per_hour:         u64,
    per_day:          u64,
    per_7d:           u64,
    per_30d:          u64,
    max_act_per_hour: u32,
    max_act_per_day:  u32,
) -> LimitPolicy {
    let mut p = LimitPolicy::zeroed();

    p.discriminator            = LIMIT_POLICY_DISCRIMINATOR;
    p.version                  = 1;
    p.limit_policy_id          = [1u8; 32];
    p.scope_kind               = scope::PATHWAY;
    p.scope_key                = Pubkey::new_from_array([2u8; 32]);
    p.per_transaction_maximum  = per_tx;
    p.per_hour_maximum         = per_hour;
    p.per_day_maximum          = per_day;
    p.per_seven_day_maximum    = per_7d;
    p.per_thirty_day_maximum   = per_30d;
    p.maximum_actions_per_hour = max_act_per_hour;
    p.maximum_actions_per_day  = max_act_per_day;

    p
}

fn build_usage_window(gross_in: u128, gross_output_amount: u128, action_count: u32) -> UsageWindow {
    let mut w = UsageWindow::zeroed();

    w.discriminator       = USAGE_WINDOW_DISCRIMINATOR;
    w.version             = 1;
    w.gross_in            = u128_to_words(gross_in);
    w.gross_output_amount = u128_to_words(gross_output_amount);
    w.net_flow            = i128_to_words(gross_in as i128 - gross_output_amount as i128);
    w.action_count        = action_count;

    w
}

// ─── Size assertions ──────────────────────────────────────────────────────────

#[test]
fn limit_policy_size_matches_declared() {
    assert_eq!(core::mem::size_of::<LimitPolicy>(), LIMIT_POLICY_SIZE);
}

#[test]
fn usage_window_size_matches_declared() {
    assert_eq!(core::mem::size_of::<UsageWindow>(), USAGE_WINDOW_SIZE);
}

// ─── Word encoding helpers ────────────────────────────────────────────────────

#[test]
fn u128_words_round_trip_zero() {
    assert_eq!(words_to_u128(u128_to_words(0)), 0);
}

#[test]
fn u128_words_round_trip_max() {
    assert_eq!(words_to_u128(u128_to_words(u128::MAX)), u128::MAX);
}

#[test]
fn u128_words_round_trip_mid_values() {
    for v in [1u128, 1000, u64::MAX as u128, (u64::MAX as u128) + 1, 0xDEADBEEF_CAFEBABE] {
        assert_eq!(words_to_u128(u128_to_words(v)), v, "failed for {v}");
    }
}

#[test]
fn u128_to_words_splits_at_64_bit_boundary() {
    let v = (1u128 << 64) | 42u128;
    let w = u128_to_words(v);

    assert_eq!(w[0], 42u64, "low word must be the low 64 bits");
    assert_eq!(w[1], 1u64,  "high word must be the high 64 bits");
}

#[test]
fn i128_words_round_trip_positive() {
    assert_eq!(words_to_i128(i128_to_words(1_000_000i128)), 1_000_000i128);
}

#[test]
fn i128_words_round_trip_zero() {
    assert_eq!(words_to_i128(i128_to_words(0)), 0);
}

#[test]
fn i128_words_round_trip_negative() {
    assert_eq!(words_to_i128(i128_to_words(-1)), -1i128);
    assert_eq!(words_to_i128(i128_to_words(-999_999)), -999_999i128);
    assert_eq!(words_to_i128(i128_to_words(i128::MIN)), i128::MIN);
}

#[test]
fn i128_words_round_trip_max() {
    assert_eq!(words_to_i128(i128_to_words(i128::MAX)), i128::MAX);
}

// ─── LimitPolicy::assert_per_tx ───────────────────────────────────────────────

#[test]
fn per_tx_zero_cap_always_passes() {
    let p = build_limit_policy(0, 0, 0, 0, 0, 0, 0);

    assert!(p.assert_per_tx(u64::MAX).is_ok(), "zero cap must be disabled");
}

#[test]
fn per_tx_passes_at_exact_max() {
    let p = build_limit_policy(1_000_000, 0, 0, 0, 0, 0, 0);

    assert!(p.assert_per_tx(1_000_000).is_ok());
}

#[test]
fn per_tx_passes_below_max() {
    let p = build_limit_policy(1_000_000, 0, 0, 0, 0, 0, 0);

    assert!(p.assert_per_tx(999_999).is_ok());
}

#[test]
fn per_tx_fails_one_above_max() {
    let p = build_limit_policy(1_000_000, 0, 0, 0, 0, 0, 0);

    assert!(matches!(
        p.assert_per_tx(1_000_001),
        Err(e) if e == ProgramError::Custom(ChanceryError::PerTxLimitBreached as u32),
    ));
}

#[test]
fn per_tx_fails_at_u64_maxmimum_with_small_cap() {
    let p = build_limit_policy(100, 0, 0, 0, 0, 0, 0);

    assert!(p.assert_per_tx(u64::MAX).is_err());
}

// ─── LimitPolicy::assert_window_volume ───────────────────────────────────────

#[test]
fn hourly_window_zero_cap_always_passes() {
    let p = build_limit_policy(0, 0, 0, 0, 0, 0, 0);

    assert!(p.assert_window_volume(u64::MAX as u128, u64::MAX, ChanceryError::HourlyLimitBreached).is_ok());
}

#[test]
fn daily_window_zero_cap_always_passes() {
    let p = build_limit_policy(0, 0, 0, 0, 0, 0, 0);

    assert!(p.assert_window_volume(u64::MAX as u128, u64::MAX, ChanceryError::DailyLimitBreached).is_ok());
}

#[test]
fn hourly_window_passes_at_exact_limit() {
    // existing = 900_000, additional = 100_000 -> total = 1_000_000 = cap
    let p = build_limit_policy(0, 1_000_000, 0, 0, 0, 0, 0);

    assert!(p.assert_window_volume(900_000, 100_000, ChanceryError::HourlyLimitBreached).is_ok());
}

#[test]
fn hourly_window_fails_one_above_limit() {
    let p = build_limit_policy(0, 1_000_000, 0, 0, 0, 0, 0);

    assert!(matches!(
        p.assert_window_volume(900_000, 100_001, ChanceryError::HourlyLimitBreached),
        Err(e) if e == ProgramError::Custom(ChanceryError::HourlyLimitBreached as u32),
    ));
}

#[test]
fn daily_window_passes_at_zero_existing() {
    let p = build_limit_policy(0, 0, 1_000_000, 0, 0, 0, 0);

    assert!(p.assert_window_volume(0, 1_000_000, ChanceryError::DailyLimitBreached).is_ok());
}

#[test]
fn daily_window_fails_when_existing_already_at_cap() {
    let p = build_limit_policy(0, 0, 1_000_000, 0, 0, 0, 0);

    assert!(matches!(
        p.assert_window_volume(1_000_000, 1, ChanceryError::DailyLimitBreached),
        Err(e) if e == ProgramError::Custom(ChanceryError::DailyLimitBreached as u32),
    ));
}

#[test]
fn weekly_window_volume_check_uses_weekly_error() {
    let p = build_limit_policy(0, 0, 0, 500, 0, 0, 0);

    assert!(matches!(
        p.assert_window_volume(500, 1, ChanceryError::WeeklyLimitBreached),
        Err(e) if e == ProgramError::Custom(ChanceryError::WeeklyLimitBreached as u32),
    ));
}

#[test]
fn monthly_window_volume_check_uses_monthly_error() {
    let p = build_limit_policy(0, 0, 0, 0, 200, 0, 0);

    assert!(matches!(
        p.assert_window_volume(200, 1, ChanceryError::MonthlyLimitBreached),
        Err(e) if e == ProgramError::Custom(ChanceryError::MonthlyLimitBreached as u32),
    ));
}

#[test]
fn unexpected_window_error_selector_fails_closed() {
    let p = build_limit_policy(0, 1_000_000, 1_000_000, 1_000_000, 1_000_000, 0, 0);

    assert!(matches!(
        p.assert_window_volume(0, 1, ChanceryError::PerTxLimitBreached),
        Err(e) if e == ProgramError::Custom(ChanceryError::LimitPolicyInvalidParameters as u32),
    ));
}

#[test]
fn window_volume_overflow_on_addition_returns_overflow_error() {
    let p = build_limit_policy(0, u64::MAX, 0, 0, 0, 0, 0);

    // window_gross is u128 since the audit fix - the only way to overflow the
    // checked_add now is to start at u128::MAX. u64::MAX + 1 fits in u128.
    assert!(matches!(
        p.assert_window_volume(u128::MAX, 1, ChanceryError::HourlyLimitBreached),
        Err(e) if e == ProgramError::Custom(ChanceryError::ArithmeticOverflow as u32),
    ));
}

// ─── Issue #13: u128 widening of assert_window_volume ────────────────────────
// Pre-PR the helper took `window_gross: u64`. Callers cast u128 → u64 with a
// silent truncation: a saturated 128-bit window would wrap to a small value
// and bypass the cap check. These tests pin the widening so a regression that
// re-narrows the parameter is caught.

#[test]
fn window_gross_exceeding_u64_max_does_not_silently_pass_under_small_cap() {
    // Cap = 100. window_gross > u64::MAX must be rejected (it would have
    // passed under the pre-PR `as u64` truncation because the high bits
    // would wrap away).
    let p = build_limit_policy(0, 100, 0, 0, 0, 0, 0);
    let saturated_gross: u128 = (u64::MAX as u128) + 1;
    assert!(matches!(
        p.assert_window_volume(saturated_gross, 1, ChanceryError::HourlyLimitBreached),
        Err(e) if e == ProgramError::Custom(ChanceryError::HourlyLimitBreached as u32),
    ));
}

#[test]
fn window_gross_with_high_word_set_does_not_truncate_for_daily_cap() {
    // Distinguish the daily branch from the hourly one.
    let p = build_limit_policy(0, 0, 1_000, 0, 0, 0, 0);
    let saturated: u128 = 1u128 << 100;  // way above u64::MAX
    assert!(matches!(
        p.assert_window_volume(saturated, 1, ChanceryError::DailyLimitBreached),
        Err(e) if e == ProgramError::Custom(ChanceryError::DailyLimitBreached as u32),
    ));
}

#[test]
fn window_gross_just_below_u128_cap_with_zero_cap_still_passes() {
    // cap = 0 is the universal short-circuit and must remain so under u128.
    let p = build_limit_policy(0, 0, 0, 0, 0, 0, 0);
    assert!(p.assert_window_volume(u128::MAX - 1, 1, ChanceryError::HourlyLimitBreached).is_ok());
}

// ─── LimitPolicy::assert_action_count ────────────────────────────────────────

#[test]
fn action_count_zero_cap_always_passes() {
    let p = build_limit_policy(0, 0, 0, 0, 0, 0, 0);  // both caps = 0

    assert!(p.assert_action_count(u32::MAX, true).is_ok());  // hourly
    assert!(p.assert_action_count(u32::MAX, false).is_ok());  // daily
}

#[test]
fn hourly_action_count_passes_below_max() {
    let p = build_limit_policy(0, 0, 0, 0, 0, 10, 0);

    assert!(p.assert_action_count(9, true).is_ok());
}

#[test]
fn hourly_action_count_fails_at_max() {
    let p = build_limit_policy(0, 0, 0, 0, 0, 10, 0);

    // count >= cap -> blocked
    assert!(matches!(
        p.assert_action_count(10, true),
        Err(e) if e == ProgramError::Custom(ChanceryError::ActionCountLimitBreached as u32),
    ));
}

#[test]
fn daily_action_count_fails_at_max() {
    let p = build_limit_policy(0, 0, 0, 0, 0, 0, 5);

    assert!(matches!(
        p.assert_action_count(5, false),
        Err(e) if e == ProgramError::Custom(ChanceryError::ActionCountLimitBreached as u32),
    ));
}

#[test]
fn daily_action_count_passes_below_max() {
    let p = build_limit_policy(0, 0, 0, 0, 0, 0, 5);

    assert!(p.assert_action_count(4, false).is_ok());
}

// ─── UsageWindow::record_inflow ───────────────────────────────────────────────

#[test]
fn record_inflow_increments_gross_in() {
    let mut w = UsageWindow::zeroed();

    w.discriminator = USAGE_WINDOW_DISCRIMINATOR;

    w.record_inflow(500_000).unwrap();

    assert_eq!(w.gross_in_u128(), 500_000);
}

#[test]
fn record_inflow_twice_accountumulates() {
    let mut w = UsageWindow::zeroed();

    w.discriminator = USAGE_WINDOW_DISCRIMINATOR;

    w.record_inflow(300_000).unwrap();
    w.record_inflow(200_000).unwrap();

    assert_eq!(w.gross_in_u128(), 500_000);
}

#[test]
fn record_inflow_increments_action_count() {
    let mut w = UsageWindow::zeroed();

    w.discriminator = USAGE_WINDOW_DISCRIMINATOR;

    w.record_inflow(1).unwrap();
    w.record_inflow(1).unwrap();

    assert_eq!(w.action_count, 2);
}

#[test]
fn record_inflow_updates_net_flow() {
    let mut w = UsageWindow::zeroed();

    w.discriminator = USAGE_WINDOW_DISCRIMINATOR;

    w.record_inflow(1_000_000).unwrap();

    assert_eq!(w.net_flow_i128(), 1_000_000i128);
}

#[test]
fn record_inflow_preserves_existing_outflow() {
    let mut w = build_usage_window(0, 400_000, 1);

    w.record_inflow(600_000).unwrap();

    assert_eq!(w.gross_in_u128(),  600_000);
    assert_eq!(w.gross_out_u128(), 400_000);
    assert_eq!(w.net_flow_i128(),  200_000i128);
}

// ─── UsageWindow::record_outflow ──────────────────────────────────────────────

#[test]
fn record_outflow_increments_gross_out() {
    let mut w = UsageWindow::zeroed();

    w.discriminator = USAGE_WINDOW_DISCRIMINATOR;

    w.record_outflow(250_000).unwrap();

    assert_eq!(w.gross_out_u128(), 250_000);
}

#[test]
fn record_outflow_twice_accountumulates() {
    let mut w = UsageWindow::zeroed();

    w.discriminator = USAGE_WINDOW_DISCRIMINATOR;

    w.record_outflow(100_000).unwrap();
    w.record_outflow(150_000).unwrap();

    assert_eq!(w.gross_out_u128(), 250_000);
}

#[test]
fn record_outflow_increments_action_count() {
    let mut w = UsageWindow::zeroed();

    w.discriminator = USAGE_WINDOW_DISCRIMINATOR;

    w.record_outflow(1).unwrap();

    assert_eq!(w.action_count, 1);
}

#[test]
fn record_outflow_produces_negative_net_flow() {
    let mut w = UsageWindow::zeroed();

    w.discriminator = USAGE_WINDOW_DISCRIMINATOR;

    w.record_outflow(500_000).unwrap();

    assert_eq!(w.net_flow_i128(), -500_000i128);
}

#[test]
fn mixed_inflow_and_outflow_net_flow_correct() {
    let mut w = UsageWindow::zeroed();

    w.discriminator = USAGE_WINDOW_DISCRIMINATOR;

    w.record_inflow(1_000_000).unwrap();
    w.record_outflow(600_000).unwrap();

    assert_eq!(w.gross_in_u128(),  1_000_000);
    assert_eq!(w.gross_out_u128(), 600_000);
    assert_eq!(w.net_flow_i128(),  400_000i128);
    assert_eq!(w.action_count,     2);
}

#[test]
fn net_flow_zero_when_in_equals_out() {
    let mut w = UsageWindow::zeroed();

    w.discriminator = USAGE_WINDOW_DISCRIMINATOR;

    w.record_inflow(500_000).unwrap();
    w.record_outflow(500_000).unwrap();

    assert_eq!(w.net_flow_i128(), 0i128);
}

#[test]
fn record_inflow_large_values_no_overflow() {
    let mut w = UsageWindow::zeroed();

    w.discriminator = USAGE_WINDOW_DISCRIMINATOR;

    // u64::MAX worth of inflows -> requires u128 to accumulate
    w.record_inflow(u64::MAX).unwrap();
    w.record_inflow(u64::MAX).unwrap();

    let expected = (u64::MAX as u128) * 2;

    assert_eq!(w.gross_in_u128(), expected);
}

// ─── Window accessor helpers ──────────────────────────────────────────────────

#[test]
fn gross_in_u128_matches_set_value() {
    let w = build_usage_window(12_345_678, 0, 0);

    assert_eq!(w.gross_in_u128(), 12_345_678);
}

#[test]
fn gross_out_u128_matches_set_value() {
    let w = build_usage_window(0, 98_765_432, 0);

    assert_eq!(w.gross_out_u128(), 98_765_432);
}

#[test]
fn net_flow_i128_reflects_set_value() {
    let w = build_usage_window(1_000_000, 400_000, 2);

    assert_eq!(w.net_flow_i128(), 600_000i128);
}

// ─── Window boundary helpers ──────────────────────────────────────────────────

#[test]
fn hourly_window_start_floors_to_hour() {
    // 3661s = 1h 1m 1s -> floor to 3600
    assert_eq!(hourly_window_start(3661), 3600);
}

#[test]
fn hourly_window_start_at_exact_hour_boundary() {
    assert_eq!(hourly_window_start(7200), 7200);  // exactly 2 hours
}

#[test]
fn hourly_window_start_at_zero() {
    assert_eq!(hourly_window_start(0), 0);
}

#[test]
fn hourly_window_start_one_second_before_boundary() {
    assert_eq!(hourly_window_start(3599), 0);
}

#[test]
fn daily_window_start_floors_to_day() {
    // 86_400 * 1 + 3_600 = day 1 + 1h -> floor to 86_400
    assert_eq!(daily_window_start(90_000), 86_400);
}

#[test]
fn daily_window_start_at_exact_day_boundary() {
    assert_eq!(daily_window_start(86_400 * 5), 86_400 * 5);
}

#[test]
fn daily_window_start_at_zero() {
    assert_eq!(daily_window_start(0), 0);
}

#[test]
fn fixed_daily_window_rotates_at_utc_boundary() {
    assert_eq!(daily_window_start(86_399), 0);
    assert_eq!(daily_window_start(86_400), 86_400);
}

#[test]
fn window_floor_is_correct_before_unix_epoch() {
    assert_eq!(hourly_window_start(-1), -3_600);
    assert_eq!(daily_window_start(-1), -86_400);
}

#[test]
fn weekly_window_start_floors_to_7day_epoch() {
    let week = 7 * 86_400i64;

    // 8 days -> second weekly epoch
    assert_eq!(weekly_window_start(week + 1), week);
}

#[test]
fn monthly_window_start_floors_to_30day_epoch() {
    let month = 30 * 86_400i64;

    assert_eq!(monthly_window_start(month + 86_400), month);
}

#[test]
fn hourly_and_daily_boundaries_are_aligned() {
    // At any hour boundary, daily window start is on a day boundary
    let ts = 86_400i64 * 3 + 3_600 * 5;  // day 3 + 5 hours
    let h  = hourly_window_start(ts);
    // The hourly boundary must be >= the daily boundary
    let d  = daily_window_start(ts);

    assert!(h >= d, "hourly start must be >= daily start for same timestamp");
}

#[test]
fn window_boundaries_are_idempotent() {
    // Applying the floor function to an already-floored value is a no-op
    let ts = 7200i64;  // exactly 2 hours, already on a boundary

    assert_eq!(hourly_window_start(hourly_window_start(ts)), hourly_window_start(ts));
}

// ─── Pre-populated window integrity ───────────────────────────────────────────

#[test]
fn build_usage_window_helper_correct() {
    let w = build_usage_window(1_000_000, 750_000, 3);

    assert_eq!(w.gross_in_u128(),  1_000_000);
    assert_eq!(w.gross_out_u128(), 750_000);
    assert_eq!(w.net_flow_i128(),  250_000i128);
    assert_eq!(w.action_count,     3);
}

#[test]
fn build_usage_window_zero_values() {
    let w = build_usage_window(0, 0, 0);

    assert_eq!(w.gross_in_u128(),  0);
    assert_eq!(w.gross_out_u128(), 0);
    assert_eq!(w.net_flow_i128(),  0);
    assert_eq!(w.action_count,     0);
}

// ─── assert_parameter_sanity ──────────────────────────────────────────────────

#[test]
fn parameter_sanity_accepts_monotonic_caps() {
    let p = build_limit_policy(100, 1_000, 10_000, 50_000, 100_000, 10, 100);

    assert!(p.assert_parameter_sanity().is_ok());
}

#[test]
fn parameter_sanity_accepts_zero_caps_as_disabled() {
    let p = build_limit_policy(1_000_000, 0, 0, 0, 0, 0, 0);

    assert!(p.assert_parameter_sanity().is_ok());
}

#[test]
fn parameter_sanity_rejects_tx_exceeding_hourly() {
    let p = build_limit_policy(1_000_000, 100, 0, 0, 0, 0, 0);

    assert!(matches!(
        p.assert_parameter_sanity(),
        Err(e) if e == ProgramError::Custom(ChanceryError::LimitPolicyInvalidParameters as u32),
    ));
}

#[test]
fn parameter_sanity_rejects_hourly_exceeding_daily() {
    let p = build_limit_policy(0, 1_000, 100, 0, 0, 0, 0);

    assert!(matches!(
        p.assert_parameter_sanity(),
        Err(e) if e == ProgramError::Custom(ChanceryError::LimitPolicyInvalidParameters as u32),
    ));
}

#[test]
fn parameter_sanity_rejects_inverted_action_caps() {
    let p = build_limit_policy(0, 0, 0, 0, 0, 100, 10);

    assert!(matches!(
        p.assert_parameter_sanity(),
        Err(e) if e == ProgramError::Custom(ChanceryError::LimitPolicyInvalidParameters as u32),
    ));
}

#[test]
fn parameter_sanity_accepts_equal_action_caps() {
    let p = build_limit_policy(0, 0, 0, 0, 0, 50, 50);

    assert!(p.assert_parameter_sanity().is_ok());
}

#[test]
fn parameter_sanity_rejects_unknown_scope_kind() {
    let mut p = build_limit_policy(0, 0, 0, 0, 0, 0, 0);

    p.scope_kind = 0xFF;

    assert!(matches!(
        p.assert_parameter_sanity(),
        Err(e) if e == ProgramError::Custom(ChanceryError::PermissionUnknownScope as u32),
    ));
}

#[test]
fn canonical_window_start_floors_per_kind() {
    assert_eq!(canonical_window_start(window_kind::HOURLY,  3_661).unwrap(), 3_600);
    assert_eq!(canonical_window_start(window_kind::DAILY,   90_000).unwrap(), 86_400);
    assert_eq!(
        canonical_window_start(window_kind::WEEKLY, 7 * 86_400 + 1).unwrap(),
        7 * 86_400,
    );
    assert_eq!(
        canonical_window_start(window_kind::MONTHLY, 30 * 86_400 + 86_400).unwrap(),
        30 * 86_400,
    );
}

#[test]
fn canonical_window_start_rejects_unknown_kind() {
    assert!(matches!(
        canonical_window_start(0xFF, 0),
        Err(e) if e == ProgramError::Custom(ChanceryError::UsageWindowKindInvalid as u32),
    ));
}

// ─── roll_forward ─────────────────────────────────────────────────────────────

#[test]
fn roll_forward_same_start_is_a_noop() {
    let mut w = build_usage_window(1_000, 250, 7);
    w.window_start_unix_timestamp = 86_400;

    let rolled = w.roll_forward(86_400).unwrap();

    assert!(!rolled);
    assert_eq!(w.gross_in_u128(),  1_000);
    assert_eq!(w.gross_out_u128(), 250);
    assert_eq!(w.net_flow_i128(),  750);
    assert_eq!(w.action_count,     7);
    assert_eq!(w.window_start_unix_timestamp, 86_400);
}

#[test]
fn roll_forward_advances_and_zeroes_accumulators() {
    let mut w = build_usage_window(1_000, 250, 7);
    w.window_start_unix_timestamp = 86_400;

    let rolled = w.roll_forward(2 * 86_400).unwrap();

    assert!(rolled);
    assert_eq!(w.window_start_unix_timestamp, 2 * 86_400);
    assert_eq!(w.gross_in_u128(),  0);
    assert_eq!(w.gross_out_u128(), 0);
    assert_eq!(w.net_flow_i128(),  0);
    assert_eq!(w.action_count,     0);
}

#[test]
fn roll_forward_rejects_clock_regression() {
    let mut w = build_usage_window(1_000, 250, 7);
    w.window_start_unix_timestamp = 2 * 86_400;

    assert!(matches!(
        w.roll_forward(86_400),
        Err(e) if e == ProgramError::Custom(ChanceryError::UsageWindowClockRegression as u32),
    ));

    // Fail-closed regression must leave the account untouched.
    assert_eq!(w.window_start_unix_timestamp, 2 * 86_400);
    assert_eq!(w.gross_in_u128(), 1_000);
}

#[test]
fn roll_forward_then_record_accrues_in_new_period() {
    let mut w = build_usage_window(500, 0, 2);
    w.window_start_unix_timestamp = 0;

    assert!(w.roll_forward(86_400).unwrap());

    w.record_inflow(75).unwrap();

    assert_eq!(w.gross_in_u128(), 75);
    assert_eq!(w.action_count,    1);
}

// ─── require_enforced_window guard (limit-window bypass fix) ──────────────────
//
// When a policy cap references a usage window, the window is mandatory: present,
// non-default, and writable. This is the primitive that closes both the omission
// bypass (window absent) and the read-only bypass (window present but not
// writable, so the update would silently skip and the fixed-window cap never accrue).

#[allow(deprecated)]
fn make_window_account_info<'a>(
    key:      &'a solana_pubkey::Pubkey,
    lamports: &'a mut u64,
    data:     &'a mut [u8],
    owner:    &'a solana_pubkey::Pubkey,
    writable: bool,
) -> solana_account_info::AccountInfo<'a> {
    solana_account_info::AccountInfo {
        key,
        lamports:    std::rc::Rc::new(std::cell::RefCell::new(lamports)),
        data:        std::rc::Rc::new(std::cell::RefCell::new(data)),
        owner,
        _unused:     0,
        is_signer:   false,
        is_writable: writable,
        executable:  false,
    }
}

#[test]
fn require_enforced_window_accepts_present_writable() {
    use crate::modules::limits::state::usage_window::require_enforced_window;

    let key      = solana_pubkey::Pubkey::new_unique();
    let owner    = crate::id();
    let mut lam  = 1u64;
    let mut data = [0u8; 8];
    let account  = make_window_account_info(&key, &mut lam, &mut data, &owner, true);

    assert!(require_enforced_window(&[account], 0).is_ok());
}

#[test]
fn require_enforced_window_rejects_absent() {
    use crate::modules::limits::state::usage_window::require_enforced_window;

    // Empty slice: index 0 is out of range → the enforced window was omitted.
    assert!(matches!(
        require_enforced_window(&[], 0),
        Err(e) if e == ProgramError::Custom(ChanceryError::MissingAccount as u32),
    ));
}

#[test]
fn require_enforced_window_rejects_default_key() {
    use crate::modules::limits::state::usage_window::require_enforced_window;

    let key      = solana_pubkey::Pubkey::default();
    let owner    = crate::id();
    let mut lam  = 1u64;
    let mut data = [0u8; 8];
    let account  = make_window_account_info(&key, &mut lam, &mut data, &owner, true);

    assert!(matches!(
        require_enforced_window(&[account], 0),
        Err(e) if e == ProgramError::Custom(ChanceryError::MissingAccount as u32),
    ));
}

#[test]
fn require_enforced_window_rejects_read_only() {
    use crate::modules::limits::state::usage_window::require_enforced_window;

    // Present, non-default, but NOT writable - the read-only bypass. Must fail
    // closed rather than pass the check and later skip the usage update.
    let key      = solana_pubkey::Pubkey::new_unique();
    let owner    = crate::id();
    let mut lam  = 1u64;
    let mut data = [0u8; 8];
    let account  = make_window_account_info(&key, &mut lam, &mut data, &owner, false);

    assert!(matches!(
        require_enforced_window(&[account], 0),
        Err(e) if e == ProgramError::Custom(ChanceryError::AccountNotWritable as u32),
    ));
}


#[test]
fn deterministic_usage_window_corpus_preserves_accumulation_invariants() {
    fn next(seed: &mut u64) -> u64 {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        *seed
    }

    let mut seed = 0xd1b5_4a32_d192_ed03u64;
    let mut window = build_usage_window(0, 0, 0);
    let mut expected_in = 0u128;
    let mut expected_out = 0u128;

    for action_index in 0..10_000u32 {
        let amount = 1 + next(&mut seed) % 1_000_000;
        if next(&mut seed) & 1 == 0 {
            window.record_inflow(amount).unwrap();
            expected_in += amount as u128;
        } else {
            window.record_outflow(amount).unwrap();
            expected_out += amount as u128;
        }
        assert_eq!(window.gross_in_u128(), expected_in, "action {action_index}");
        assert_eq!(window.gross_out_u128(), expected_out, "action {action_index}");
        assert_eq!(window.net_flow_i128(), expected_in as i128 - expected_out as i128, "action {action_index}");
        assert_eq!(window.action_count, action_index + 1, "action {action_index}");
    }
}

// ─── create_usage_window_if_missing - binding-time no-reset (spec 14 §14.9) ───
//
// Windows are keyed by (scope, kind) only, so distinct policies with the same
// scope share one accumulator. Re-binding an already-initialized canonical
// window must verify it and leave the live accumulator, period start, and
// stored refund recipient untouched - a second binder can never reset a live
// cap or redirect the close-time rent refund.

fn build_initialized_canonical_window(
    scope_hash: [u8; 32],
    kind:       u8,
    gross_in:   u128,
    start:      i64,
    recipient:  solana_pubkey::Pubkey,
) -> (solana_pubkey::Pubkey, Vec<u8>) {
    let (key, bump) = UsageWindow::pda(&scope_hash, kind, &crate::id());
    let mut data    = vec![0u8; USAGE_WINDOW_SIZE];

    {
        let w: &mut UsageWindow = bytemuck::from_bytes_mut(&mut data[..]);

        w.discriminator               = USAGE_WINDOW_DISCRIMINATOR;
        w.version                     = 1;
        w.bump                        = bump;
        w.window_kind                 = kind;
        w.scope_hash                  = scope_hash;
        w.window_start_unix_timestamp = start;
        w.gross_in                    = u128_to_words(gross_in);
        w.action_count                = 9;
        w.rent_refund_recipient       = recipient;
    }

    (key, data)
}

#[test]
fn create_if_missing_leaves_live_accumulator_untouched() {
    use crate::modules::limits::state::usage_window::create_usage_window_if_missing;

    let scope     = [7u8; 32];
    let kind      = window_kind::DAILY;
    let start     = daily_window_start(1_000_000);
    let recipient = solana_pubkey::Pubkey::new_unique();

    let (key, mut data) = build_initialized_canonical_window(scope, kind, 123_456, start, recipient);

    let owner            = crate::id();
    let mut lam          = 2_000_000u64;
    let window           = make_window_account_info(&key, &mut lam, &mut data, &owner, true);

    let payer_key        = solana_pubkey::Pubkey::new_unique();
    let system_key       = solana_pubkey::Pubkey::new_unique();
    let system_owner     = solana_pubkey::Pubkey::default();
    let mut payer_lam    = 1u64;
    let mut system_lam   = 1u64;
    let mut payer_data   = [0u8; 0];
    let mut system_data  = [0u8; 0];
    let payer            = make_window_account_info(&payer_key, &mut payer_lam, &mut payer_data, &system_owner, true);
    let system           = make_window_account_info(&system_key, &mut system_lam, &mut system_data, &system_owner, false);

    // Second binder arrives much later (clock well past the seeded period) and
    // with a different payer/refund recipient of its own.
    let second_binder_recipient = solana_pubkey::Pubkey::new_unique();
    let created = create_usage_window_if_missing(
        &window,
        &payer,
        &system,
        &scope,
        kind,
        start + 10 * 86_400,
        &second_binder_recipient,
        &crate::id(),
    )
    .unwrap();

    assert!(!created, "an already-initialized canonical window must not be re-created");

    let data_ref = window.data.borrow();
    let w: &UsageWindow = bytemuck::from_bytes(&data_ref[..]);
    assert_eq!(w.gross_in_u128(), 123_456, "live accumulator must be untouched");
    assert_eq!(w.window_start_unix_timestamp, start, "live period must not be rolled or reset by binding");
    assert_eq!(w.action_count, 9, "action count must be untouched");
    assert_eq!(
        w.rent_refund_recipient, recipient,
        "a second binder must never redirect the stored refund recipient",
    );
}

#[test]
fn create_if_missing_rejects_non_canonical_key_for_scope() {
    use crate::modules::limits::state::usage_window::create_usage_window_if_missing;

    let seeded_scope = [7u8; 32];
    let other_scope  = [8u8; 32];
    let kind         = window_kind::DAILY;
    let recipient    = solana_pubkey::Pubkey::new_unique();

    let (key, mut data) = build_initialized_canonical_window(
        seeded_scope, kind, 1, daily_window_start(1_000_000), recipient,
    );

    let owner           = crate::id();
    let mut lam         = 2_000_000u64;
    let window          = make_window_account_info(&key, &mut lam, &mut data, &owner, true);

    let payer_key       = solana_pubkey::Pubkey::new_unique();
    let system_key      = solana_pubkey::Pubkey::new_unique();
    let system_owner    = solana_pubkey::Pubkey::default();
    let mut payer_lam   = 1u64;
    let mut system_lam  = 1u64;
    let mut payer_data  = [0u8; 0];
    let mut system_data = [0u8; 0];
    let payer           = make_window_account_info(&payer_key, &mut payer_lam, &mut payer_data, &system_owner, true);
    let system          = make_window_account_info(&system_key, &mut system_lam, &mut system_data, &system_owner, false);

    // The account key is canonical for seeded_scope, not other_scope: binding
    // under a different scope must fail before touching any state.
    assert!(matches!(
        create_usage_window_if_missing(
            &window, &payer, &system, &other_scope, kind, 1_000_000, &recipient, &crate::id(),
        ),
        Err(e) if e == ProgramError::Custom(ChanceryError::InvalidPda as u32),
    ));

    let data_ref = window.data.borrow();
    let w: &UsageWindow = bytemuck::from_bytes(&data_ref[..]);
    assert_eq!(w.gross_in_u128(), 1, "rejected binding must leave state untouched");
}
