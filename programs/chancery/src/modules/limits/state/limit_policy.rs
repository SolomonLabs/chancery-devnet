use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;
use solana_sha256_hasher::hashv;

use crate::{
    constants::{scope, seeds},
    error::ChanceryError,
};

// ─── Discriminator ────────────────────────────────────────────────────────────
pub const LIMIT_POLICY_DISCRIMINATOR: [u8; 8] =
    [0x6c, 0x6d, 0x74, 0x70, 0x6f, 0x6c, 0x79, 0x00];

// ─── Account size ─────────────────────────────────────────────────────────────
//   [u8;8]   discriminator             =  8 @ 0
//   u16      version                   =  2 @ 8
//   u8       bump                      =  1 @ 10
//   u8       scope_kind                =  1 @ 11
//   [u8;4]   _pad0                     =  4 @ 12  -> [u8;32] at 16
//   [u8;32]  limit_policy_id           = 32 @ 16
//   [u8;32]  scope_key                 = 32 @ 48
//   u64      per_transaction_maximum   =  8 @ 80  (80%8=0 Y)
//   u64      per_hour_maximum          =  8 @ 88
//   u64      per_day_maximum           =  8 @ 96
//   u64      per_seven_day_maximum     =  8 @ 104
//   u64      per_thirty_day_maximum    =  8 @ 112
//   u32      maximum_actions_per_hour  =  4 @ 120
//   u32      maximum_actions_per_day   =  4 @ 124
//   [u64;2]  _reserved_breach_flags    = 16 @ 128  (128%8=0 Y)
//   u64      status_flags              =  8 @ 144
//   [u8;32]  _reserved                 = 32 @ 152
//                                       ─────
//                                       184 bytes
pub const LIMIT_POLICY_SIZE: usize = 184;

// ─── State ────────────────────────────────────────────────────────────────────

/// Defines fixed-window volume and action-count limits for a scope.
/// Seed: [b"limit-policy", limit_policy_id]
///
/// A zero value for any cap means that cap is disabled.
/// `scope_key` is the asset mint, pathway id hash, counterparty key, etc.
/// depending on `scope_kind`.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LimitPolicy {
    pub discriminator:            [u8; 8],
    pub version:                  u16,
    pub bump:                     u8,

    /// One of `scope::*` constants.
    pub scope_kind:               u8,
    pub _pad0:                    [u8; 4],

    pub limit_policy_id:          [u8; 32],
    pub scope_key:                Pubkey,

    /// Hard cap on a single transaction. 0 = disabled.
    pub per_transaction_maximum:  u64,

    /// Fixed UTC-hour gross flow cap. 0 = disabled.
    pub per_hour_maximum:         u64,

    /// Fixed UTC-day gross flow cap. 0 = disabled.
    pub per_day_maximum:          u64,

    /// Fixed 7-day epoch gross flow cap. 0 = disabled.
    pub per_seven_day_maximum:    u64,

    /// Fixed 30-day epoch gross flow cap. 0 = disabled.
    pub per_thirty_day_maximum:   u64,

    /// Maximum number of actions within a single hour window. 0 = disabled.
    pub maximum_actions_per_hour: u32,

    /// Maximum number of actions within a single day window. 0 = disabled.
    pub maximum_actions_per_day:  u32,

    /// Reserved for a future keeper-finalized breach design. Breach behavior
    /// is revert-only in the MVP: no breach flags are stored or enforced
    /// (formerly `hard_fail_flags` / `breach_action_flags`, which were inert).
    /// Must be zero.
    pub _reserved_breach_flags:   [u64; 2],

    /// Policy-level status flags.
    pub status_flags:             u64,

    pub _reserved:                [u8; 32],
}

// ─── Helpers ──────────────────────────────────────────────────────────────────
impl LimitPolicy {
    /// Canonical semantic payload for config-change hashing. Account headers,
    /// program-managed status, breach headroom, and reserved bytes are omitted.
    pub(crate) fn config_change_payload(&self) -> Vec<u8> {
        let mut payload = Vec::with_capacity(113);
        payload.extend_from_slice(&self.limit_policy_id);
        payload.push(self.scope_kind);
        payload.extend_from_slice(self.scope_key.as_ref());
        payload.extend_from_slice(&self.per_transaction_maximum.to_le_bytes());
        payload.extend_from_slice(&self.per_hour_maximum.to_le_bytes());
        payload.extend_from_slice(&self.per_day_maximum.to_le_bytes());
        payload.extend_from_slice(&self.per_seven_day_maximum.to_le_bytes());
        payload.extend_from_slice(&self.per_thirty_day_maximum.to_le_bytes());
        payload.extend_from_slice(&self.maximum_actions_per_hour.to_le_bytes());
        payload.extend_from_slice(&self.maximum_actions_per_day.to_le_bytes());
        payload
    }

    pub fn pda(limit_policy_id: &[u8; 32], program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[seeds::LIMIT_POLICY, limit_policy_id.as_ref()],
            program_id,
        )
    }

    pub fn verify_pda(
        account:         &AccountInfo,
        limit_policy_id: &[u8; 32],
        program_id:      &Pubkey,
    ) -> Result<u8, ProgramError> {
        let (expected, bump) = Self::pda(limit_policy_id, program_id);

        if account.key != &expected {
            return Err(ChanceryError::InvalidPda.into());
        }

        Ok(bump)
    }

    /// sha256(scope_kind || scope_key) - matches the UsageWindow seed convention.
    pub fn scope_hash(&self) -> [u8; 32] {
        hashv(&[&[self.scope_kind], self.scope_key.as_ref()]).to_bytes()
    }

    pub fn load<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = crate::state_loader::load_state::<Self>(
            account,
            LIMIT_POLICY_SIZE,
            LIMIT_POLICY_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    pub(crate) fn load_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        limit_policy_id: &[u8; 32],
        expected_bump: u8,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;

        if state.limit_policy_id != *limit_policy_id {
            return Err(ChanceryError::InvalidPda.into());
        }

        crate::state_loader::assert_stored_bump_matches(state.bump, expected_bump)?;

        Ok(state)
    }

    /// Load initialized state and prove that its identity fields resolve to
    /// this account's canonical PDA and stored bump.
    pub fn load_verified<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;
        let expected_bump = Self::verify_pda(account, &state.limit_policy_id, &crate::id())?;

        if state.bump != expected_bump {
            return Err(ChanceryError::StoredBumpMismatch.into());
        }

        Ok(state)
    }

    pub fn load_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = crate::state_loader::load_state_mut::<Self>(
            account,
            LIMIT_POLICY_SIZE,
            LIMIT_POLICY_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    pub(crate) fn load_mut_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        limit_policy_id: &[u8; 32],
        expected_bump: u8,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;

        if state.limit_policy_id != *limit_policy_id {
            return Err(ChanceryError::InvalidPda.into());
        }

        crate::state_loader::assert_stored_bump_matches(state.bump, expected_bump)?;

        Ok(state)
    }

    /// Mutable counterpart to `load_verified`.
    pub fn load_verified_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;
        let expected_bump = Self::verify_pda(account, &state.limit_policy_id, &crate::id())?;

        if state.bump != expected_bump {
            return Err(ChanceryError::StoredBumpMismatch.into());
        }

        Ok(state)
    }

    pub fn load_uninitialized_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        crate::state_loader::load_uninitialized_state_mut::<Self>(
            account,
            LIMIT_POLICY_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    /// Reject operator-supplied caps that cannot be enforced coherently.
    /// Called from register/update handlers before persisting.
    pub fn assert_parameter_sanity(&self) -> Result<(), ProgramError> {
        if !matches!(
            self.scope_kind,
            scope::ASSET
                | scope::PATHWAY
                | scope::DESTINATION
                | scope::COUNTERPARTY
                | scope::EXECUTOR
        ) {
            return Err(ChanceryError::PermissionUnknownScope.into());
        }

        if self.limit_policy_id == [0u8; 32] {
            return Err(ChanceryError::LimitPolicyInvalidParameters.into());
        }

        let is_template_scope =
            matches!(self.scope_kind, scope::COUNTERPARTY | scope::EXECUTOR);

        if is_template_scope {
            if self.scope_key != Pubkey::default() {
                return Err(ChanceryError::LimitPolicyInvalidParameters.into());
            }
        } else if self.scope_key == Pubkey::default() {
            return Err(ChanceryError::LimitPolicyInvalidParameters.into());
        }

        // Period caps must be non-decreasing when enabled (0 = no cap).
        let mut prev_enabled: Option<u64> = None;
        for cap in [
            self.per_transaction_maximum,
            self.per_hour_maximum,
            self.per_day_maximum,
            self.per_seven_day_maximum,
            self.per_thirty_day_maximum,
        ] {
            if cap == 0 {
                continue;
            }
            if let Some(prev) = prev_enabled {
                if prev > cap {
                    return Err(ChanceryError::LimitPolicyInvalidParameters.into());
                }
            }
            prev_enabled = Some(cap);
        }

        if self.maximum_actions_per_hour != 0
            && self.maximum_actions_per_day != 0
            && self.maximum_actions_per_hour > self.maximum_actions_per_day
        {
            return Err(ChanceryError::LimitPolicyInvalidParameters.into());
        }

        // Breach behavior is revert-only (issue RB-05): the former breach flag
        // fields were removed rather than shipped as inert policy state. The
        // reserved slots and program-managed status must stay zero here.
        if self._reserved_breach_flags != [0u64; 2] || self.status_flags != 0 {
            return Err(ChanceryError::LimitPolicyInvalidParameters.into());
        }

        // Only pathway-scoped policies have runtime support for hourly,
        // weekly, monthly, and action-count windows. All dimension and reserve
        // destination policies are enforced with per-transaction plus fixed
        // UTC-day caps only.
        if self.scope_kind != scope::PATHWAY {
            self.assert_dimension_scope_caps()?;
        }

        Ok(())
    }

    /// Assert this policy is usable as a settlement *dimension* policy
    /// (asset / counterparty / executor references on `PathwayPolicy`).
    ///
    /// Dimension enforcement is per-transaction + fixed UTC-day only, so any
    /// other configured cap would be stored-but-unenforced - exactly the
    /// semantic-truth failure this module is not allowed to reintroduce.
    /// Fail closed on hourly / 7-day / 30-day / action-count caps.
    pub fn assert_dimension_scope_caps(&self) -> Result<(), ProgramError> {
        if self.per_hour_maximum != 0
            || self.per_seven_day_maximum != 0
            || self.per_thirty_day_maximum != 0
            || self.maximum_actions_per_hour != 0
            || self.maximum_actions_per_day != 0
        {
            return Err(ChanceryError::LimitPolicyInvalidParameters.into());
        }

        Ok(())
    }

    /// Assert a dimension policy supplies the mandatory fixed UTC-day cap.
    /// Used for delegated counterparty containment after scope binding has
    /// already identified the policy as COUNTERPARTY-scoped.
    pub fn assert_required_daily_dimension_cap(&self) -> Result<(), ProgramError> {
        self.assert_dimension_scope_caps()?;

        if self.per_day_maximum == 0 {
            return Err(ChanceryError::LimitPolicyInvalidParameters.into());
        }

        Ok(())
    }

    /// Assert this policy is suitable for reserve withdrawal containment.
    /// Reserve withdrawals intentionally use a simple, auditable control:
    /// mandatory per-transaction and fixed UTC-day caps, scoped to the exact
    /// ReserveDestination PDA. Other windows/action caps are rejected so the
    /// execution path cannot silently store unenforced policy fields.
    pub fn assert_reserve_withdrawal_caps(
        &self,
        reserve_destination: &Pubkey,
    ) -> Result<(), ProgramError> {
        if self.scope_kind != scope::DESTINATION
            || &self.scope_key != reserve_destination
            || self.per_transaction_maximum == 0
            || self.per_day_maximum == 0
        {
            return Err(ChanceryError::ReserveWithdrawalLimitPolicyRequired.into());
        }

        self.assert_dimension_scope_caps()
    }

    // ── Cap checks (called with current window totals from UsageWindow) ────────

    pub fn assert_per_tx(&self, amount: u64) -> Result<(), ProgramError> {
        if self.per_transaction_maximum != 0 && amount > self.per_transaction_maximum {
            return Err(ChanceryError::PerTxLimitBreached.into());
        }

        Ok(())
    }

    /// Cap check against a fixed-window total.
    ///
    /// `window_gross` is u128 because `UsageWindow` accumulates inflow/outflow
    /// across many actions and a per-action u64 amount times u32 action_count
    /// can exceed u64::MAX. Truncating the window total to u64 (the prior
    /// behaviour) silently wrapped a saturated window down to a small value,
    /// making the cap check pass when it should not.
    pub fn assert_window_volume(
        &self,
        window_gross: u128,
        additional  : u64,
        error       : ChanceryError,
    ) -> Result<(), ProgramError> {
        let cap = match error {
            ChanceryError::HourlyLimitBreached  => self.per_hour_maximum,
            ChanceryError::DailyLimitBreached   => self.per_day_maximum,
            ChanceryError::WeeklyLimitBreached  => self.per_seven_day_maximum,
            ChanceryError::MonthlyLimitBreached => self.per_thirty_day_maximum,
            _ => return Err(ChanceryError::LimitPolicyInvalidParameters.into()),
        };

        if cap == 0 {
            return Ok(());
        }

        let new_total: u128 = window_gross
            .checked_add(additional as u128)
            .ok_or(ChanceryError::ArithmeticOverflow)?;

        if new_total > cap as u128 {
            return Err(error.into());
        }

        Ok(())
    }

    pub fn assert_action_count(
        &self,
        current_count: u32,
        is_hourly    : bool,
    ) -> Result<(), ProgramError> {
        let cap = if is_hourly {
            self.maximum_actions_per_hour
        } else {
            self.maximum_actions_per_day
        };

        if cap != 0 && current_count >= cap {
            return Err(ChanceryError::ActionCountLimitBreached.into());
        }

        Ok(())
    }
}

// ─── Size assertion ───────────────────────────────────────────────────────────
const _: () = assert!(
    core::mem::size_of::<LimitPolicy>() == LIMIT_POLICY_SIZE,
    "LimitPolicy size mismatch - update LIMIT_POLICY_SIZE",
);

#[cfg(test)]
mod tests {
    //! Unit coverage for `assert_action_count` - the helper newly invoked by
    //! every settlement + cross-chain handler under PR #15 / issue-16. Existing
    //! 07c_limits_actions.test.ts exercises the real on-chain path for
    //! mint_direct + redeem_direct; this unit test pins the four boundary
    //! cases for each cap so a regression in either branch (hourly/daily)
    //! gets caught without spinning up a full SVM.
    use super::*;
    use bytemuck::Zeroable;

    fn policy(hourly: u32, daily: u32) -> LimitPolicy {
        let mut p = LimitPolicy::zeroed();
        p.maximum_actions_per_hour = hourly;
        p.maximum_actions_per_day  = daily;
        p
    }

    // ── Hourly branch ───────────────────────────────────────────────────────
    #[test]
    fn hourly_below_cap_passes() {
        let p = policy(3, 0);
        assert!(p.assert_action_count(2, true).is_ok());
    }

    #[test]
    fn hourly_at_cap_rejected() {
        let p = policy(3, 0);
        let err = p.assert_action_count(3, true).unwrap_err();
        // Boundary case: assert_action_count rejects when count >= cap (NOT >).
        assert!(matches!(err, e if e == ChanceryError::ActionCountLimitBreached.into()));
    }

    #[test]
    fn hourly_above_cap_rejected() {
        let p = policy(3, 0);
        let err = p.assert_action_count(4, true).unwrap_err();
        assert!(matches!(err, e if e == ChanceryError::ActionCountLimitBreached.into()));
    }

    #[test]
    fn hourly_zero_cap_means_no_limit() {
        let p = policy(0, 0);
        // cap = 0 disables the check even at huge counts.
        assert!(p.assert_action_count(u32::MAX, true).is_ok());
    }

    // ── Daily branch ────────────────────────────────────────────────────────
    #[test]
    fn daily_below_cap_passes() {
        let p = policy(0, 100);
        assert!(p.assert_action_count(99, false).is_ok());
    }

    #[test]
    fn daily_at_cap_rejected() {
        let p = policy(0, 100);
        let err = p.assert_action_count(100, false).unwrap_err();
        assert!(matches!(err, e if e == ChanceryError::ActionCountLimitBreached.into()));
    }

    #[test]
    fn daily_zero_cap_means_no_limit() {
        let p = policy(0, 0);
        assert!(p.assert_action_count(u32::MAX, false).is_ok());
    }

    #[test]
    fn unexpected_window_error_selector_fails_closed() {
        let mut p = policy(0, 0);
        p.per_day_maximum = 100;

        assert_eq!(
            p.assert_window_volume(
                0,
                1,
                ChanceryError::AmountExceedsMaximum,
            ),
            Err(ChanceryError::LimitPolicyInvalidParameters.into()),
        );
    }

    // ── Branch isolation: hourly cap exhausted, daily branch independent ────
    #[test]
    fn hourly_cap_exhausted_does_not_affect_daily_branch() {
        let p = policy(3, 100);
        // hourly check is at-cap → rejected
        assert!(p.assert_action_count(3, true).is_err());
        // same count under daily branch (well below daily cap) → ok
        assert!(p.assert_action_count(3, false).is_ok());
    }

    #[test]
    fn nonzero_reserved_breach_slots_or_status_are_rejected() {
        let mut p = LimitPolicy::zeroed();
        p.limit_policy_id = [1u8; 32];
        p.scope_kind = scope::PATHWAY;
        p.scope_key = Pubkey::new_from_array([2u8; 32]);
        p._reserved_breach_flags = [1, 0];
        assert!(p.assert_parameter_sanity().is_err());

        p._reserved_breach_flags = [0, 0];
        p.status_flags = 1;
        assert!(p.assert_parameter_sanity().is_err());
    }

    // ── Dimension-scope cap coherence (spec §5.8, issue RB-04) ──────────────
    #[test]
    fn dimension_caps_per_tx_and_daily_only_pass() {
        let mut p = LimitPolicy::zeroed();
        p.per_transaction_maximum = 1_000;
        p.per_day_maximum         = 10_000;
        assert!(p.assert_dimension_scope_caps().is_ok());
    }

    #[test]
    fn dimension_caps_all_zero_pass() {
        let p = LimitPolicy::zeroed();
        assert!(p.assert_dimension_scope_caps().is_ok());
    }

    #[test]
    fn required_daily_dimension_cap_rejects_zero_daily_maximum() {
        let mut p = LimitPolicy::zeroed();
        p.per_transaction_maximum = 1_000;

        assert!(p.assert_required_daily_dimension_cap().is_err());
    }

    #[test]
    fn required_daily_dimension_cap_accepts_nonzero_daily_maximum() {
        let mut p = LimitPolicy::zeroed();
        p.per_transaction_maximum = 1_000;
        p.per_day_maximum = 10_000;

        assert!(p.assert_required_daily_dimension_cap().is_ok());
    }

    #[test]
    fn dimension_caps_hourly_rejected() {
        let mut p = LimitPolicy::zeroed();
        p.per_hour_maximum = 1;
        assert!(p.assert_dimension_scope_caps().is_err());
    }

    #[test]
    fn dimension_caps_seven_day_rejected() {
        let mut p = LimitPolicy::zeroed();
        p.per_seven_day_maximum = 1;
        assert!(p.assert_dimension_scope_caps().is_err());
    }

    #[test]
    fn dimension_caps_thirty_day_rejected() {
        let mut p = LimitPolicy::zeroed();
        p.per_thirty_day_maximum = 1;
        assert!(p.assert_dimension_scope_caps().is_err());
    }

    #[test]
    fn dimension_caps_action_counts_rejected() {
        let mut p = LimitPolicy::zeroed();
        p.maximum_actions_per_hour = 1;
        assert!(p.assert_dimension_scope_caps().is_err());

        let mut q = LimitPolicy::zeroed();
        q.maximum_actions_per_day = 1;
        assert!(q.assert_dimension_scope_caps().is_err());
    }

    #[test]
    fn reserve_withdrawal_caps_require_exact_destination_scope_and_both_caps() {
        let destination = Pubkey::new_unique();
        let mut p = LimitPolicy::zeroed();
        p.scope_kind = scope::DESTINATION;
        p.scope_key = destination;
        p.per_transaction_maximum = 1_000;
        p.per_day_maximum = 10_000;

        assert!(p.assert_reserve_withdrawal_caps(&destination).is_ok());
        assert!(p
            .assert_reserve_withdrawal_caps(&Pubkey::new_unique())
            .is_err());

        p.scope_kind = scope::GLOBAL;
        assert!(p.assert_reserve_withdrawal_caps(&destination).is_err());
        p.scope_kind = scope::DESTINATION;

        p.per_transaction_maximum = 0;
        assert!(p.assert_reserve_withdrawal_caps(&destination).is_err());
        p.per_transaction_maximum = 1_000;
        p.per_day_maximum = 0;
        assert!(p.assert_reserve_withdrawal_caps(&destination).is_err());
    }

    #[test]
    fn reserve_withdrawal_caps_reject_unenforced_window_or_action_fields() {
        let destination = Pubkey::new_unique();
        let mut p = LimitPolicy::zeroed();
        p.scope_kind = scope::DESTINATION;
        p.scope_key = destination;
        p.per_transaction_maximum = 1_000;
        p.per_day_maximum = 10_000;

        p.per_hour_maximum = 1;
        assert!(p.assert_reserve_withdrawal_caps(&destination).is_err());
        p.per_hour_maximum = 0;
        p.maximum_actions_per_day = 1;
        assert!(p.assert_reserve_withdrawal_caps(&destination).is_err());
    }

    #[test]
    fn counterparty_and_executor_templates_are_registerable() {
        for scope_kind in [scope::COUNTERPARTY, scope::EXECUTOR] {
            let mut p = LimitPolicy::zeroed();
            p.limit_policy_id = [scope_kind; 32];
            p.scope_kind = scope_kind;
            p.scope_key = Pubkey::default();
            p.per_day_maximum = 1_000;

            assert!(p.assert_parameter_sanity().is_ok());
        }
    }

    #[test]
    fn template_scopes_reject_concrete_scope_keys() {
        for scope_kind in [scope::COUNTERPARTY, scope::EXECUTOR] {
            let mut p = LimitPolicy::zeroed();
            p.limit_policy_id = [scope_kind; 32];
            p.scope_kind = scope_kind;
            p.scope_key = Pubkey::new_unique();

            assert!(p.assert_parameter_sanity().is_err());
        }
    }

    #[test]
    fn keyed_scopes_reject_default_scope_keys() {
        for scope_kind in [scope::ASSET, scope::PATHWAY, scope::DESTINATION] {
            let mut p = LimitPolicy::zeroed();
            p.limit_policy_id = [scope_kind; 32];
            p.scope_kind = scope_kind;
            p.scope_key = Pubkey::default();

            assert!(p.assert_parameter_sanity().is_err());
        }
    }
}
