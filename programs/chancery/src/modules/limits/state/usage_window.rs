use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::{seeds, window_kind},
    error::ChanceryError,
    modules::core::instructions::create_pda_account,
};

// ─── Discriminator ────────────────────────────────────────────────────────────
pub const USAGE_WINDOW_DISCRIMINATOR: [u8; 8] =
    [0x75, 0x73, 0x67, 0x77, 0x6e, 0x64, 0x77, 0x00];

// ─── Account size ─────────────────────────────────────────────────────────────
//   [u8;8]   discriminator          =  8 @ 0
//   u16      version                =  2 @ 8
//   u8       bump                   =  1 @ 10
//   u8       window_kind            =  1 @ 11
//   [u8;4]   _pad0                  =  4 @ 12
//   [u8;32]  scope_hash             = 32 @ 16
//   i64      window_start           =  8 @ 48  (48%8=0 Y)
//   [u64;2]  gross_in               = 16 @ 56  (u128 via two words)
//   [u64;2]  gross_output_amount    = 16 @ 72
//   [u64;2]  net_flow               = 16 @ 88  (two's-complement i128)
//   u32      action_count           =  4 @ 104
//   [u8;4]   _pad1                  =  4 @ 108
//   [u8;32]  rent_refund_recipient  = 32 @ 112
//   [u8;32]  _reserved              = 32 @ 144
//                                    ─────
//                                    176 bytes
pub const USAGE_WINDOW_SIZE: usize = 176;

// ─── net_flow helpers ─────────────────────────────────────────────────────────

/// Encode an i128 into two u64 words (little-endian two's complement).
#[inline]
pub fn i128_to_words(v: i128) -> [u64; 2] {
    let bits = v as u128;
    [bits as u64, (bits >> 64) as u64]
}

/// Decode two u64 words back to i128.
#[inline]
pub fn words_to_i128(w: [u64; 2]) -> i128 {
    ((w[0] as u128) | ((w[1] as u128) << 64)) as i128
}

/// Combine two u64 words into a u128 (for gross_in / gross_output_amount).
#[inline]
pub fn words_to_u128(w: [u64; 2]) -> u128 {
    (w[0] as u128) | ((w[1] as u128) << 64)
}

/// Encode u128 into two u64 words.
#[inline]
pub fn u128_to_words(v: u128) -> [u64; 2] {
    [v as u64, (v >> 64) as u64]
}

// ─── State ────────────────────────────────────────────────────────────────────

/// Stable rolling accumulator for a single (scope, window_kind) pair.
///
/// Seed: [b"usage-window", scope_hash, window_kind (1 byte)]
///
/// One PDA per (scope, kind) - the account address never changes at a period
/// boundary. `window_start_unix_timestamp` is mutable state, not PDA identity:
/// every consuming instruction derives the canonical current start from Clock
/// and rolls the account forward in place (`roll_forward`) before checking any
/// cap. The closed period's final totals are preserved in canonical
/// `UsageWindowRolled` evidence at roll time; program-owned state is not a
/// historical archive.
///
/// Accounts are created at binding time (limit-policy registration, widening
/// consume, permission grant, remote-domain registration), never in the
/// settlement hot path, and store the payer as `rent_refund_recipient` for
/// later reclamation.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct UsageWindow {
    pub discriminator:               [u8; 8],
    pub version:                     u16,
    pub bump:                        u8,

    /// One of `window_kind::*` constants.
    pub window_kind:                 u8,
    pub _pad0:                       [u8; 4],

    /// Scope hash - sha256(scope_kind || scope_key) to keep seed size fixed.
    pub scope_hash:                  [u8; 32],

    /// Unix timestamp of the current period start (aligned to
    /// hour/day/week/month boundary). Mutable: advanced in place by
    /// `roll_forward`; never part of the PDA seeds.
    pub window_start_unix_timestamp: i64,

    /// Cumulative gross inflow for the current period (two u64 words, u128
    /// semantics).
    pub gross_in:                    [u64; 2],

    /// Cumulative gross outflow for the current period.
    pub gross_output_amount:         [u64; 2],

    /// Net flow = gross_in - gross_output_amount (two u64 words, i128
    /// two's-complement).
    pub net_flow:                    [u64; 2],

    /// Number of individual actions (mints or redeems) in the current period.
    pub action_count:                u32,
    pub _pad1:                       [u8; 4],

    /// Deterministic close/refund recipient recorded at creation (the binding
    /// ceremony's payer). Terminal cleanup cannot redirect rent.
    pub rent_refund_recipient:       Pubkey,

    /// Append-only layout headroom. Must remain zero until consumed by a
    /// layout change, and must be replenished in that same change.
    pub _reserved:                   [u8; 32],
}

// ─── Helpers ──────────────────────────────────────────────────────────────────
impl UsageWindow {
    /// Construct the stable PDA seeds for a usage window.
    pub fn pda(
        scope_hash:  &[u8; 32],
        window_kind: u8,
        program_id:  &Pubkey,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                seeds::USAGE_WINDOW,
                scope_hash.as_ref(),
                &[window_kind],
            ],
            program_id,
        )
    }

    pub fn verify_pda(
        account:     &AccountInfo,
        scope_hash:  &[u8; 32],
        window_kind: u8,
        program_id:  &Pubkey,
    ) -> Result<u8, ProgramError> {
        let (expected, bump) = Self::pda(scope_hash, window_kind, program_id);

        if account.key != &expected {
            return Err(ChanceryError::InvalidPda.into());
        }

        Ok(bump)
    }

    pub fn load<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = crate::state_loader::load_state::<Self>(
            account,
            USAGE_WINDOW_SIZE,
            USAGE_WINDOW_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    fn load_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        scope_hash: &[u8; 32],
        window_kind: u8,
        expected_bump: u8,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;

        if state.scope_hash != *scope_hash || state.window_kind != window_kind {
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
        let expected_bump = Self::verify_pda(account, &state.scope_hash, state.window_kind, &crate::id())?;

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
            USAGE_WINDOW_SIZE,
            USAGE_WINDOW_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    fn load_mut_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        scope_hash: &[u8; 32],
        window_kind: u8,
        expected_bump: u8,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;

        if state.scope_hash != *scope_hash || state.window_kind != window_kind {
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
        let expected_bump = Self::verify_pda(account, &state.scope_hash, state.window_kind, &crate::id())?;

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
            USAGE_WINDOW_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    // ── Rollover ──────────────────────────────────────────────────────────────

    /// Advance a stale account to the clock-derived current period, in place.
    ///
    /// Returns `Ok(true)` when the account rolled (accumulators reset to zero
    /// for the new period), `Ok(false)` when it was already current, and fails
    /// closed with `UsageWindowClockRegression` when the stored start is later
    /// than the canonical current start. The caller must never supply the new
    /// start as an instruction argument; it must come entirely from Clock and
    /// the program's boundary helpers.
    ///
    /// The transition is not discretionary: a caller can advance a stale
    /// account or leave a current one unchanged, and can never reset the
    /// active period, select an arbitrary period, move the account backwards,
    /// or alter its scope or kind.
    pub fn roll_forward(
        &mut self,
        current_window_start_unix_timestamp: i64,
    ) -> Result<bool, ProgramError> {
        if self.window_start_unix_timestamp > current_window_start_unix_timestamp {
            return Err(ChanceryError::UsageWindowClockRegression.into());
        }

        if self.window_start_unix_timestamp == current_window_start_unix_timestamp {
            return Ok(false);
        }

        self.window_start_unix_timestamp = current_window_start_unix_timestamp;
        self.gross_in            = [0, 0];
        self.gross_output_amount = [0, 0];
        self.net_flow            = [0, 0];
        self.action_count        = 0;

        Ok(true)
    }

    // ── Accumulation ──────────────────────────────────────────────────────────

    /// Record a mint (inflow) event. All fallible calculations complete before
    /// any field is changed, so an arithmetic failure cannot leave a partially
    /// updated accumulator.
    pub fn record_inflow(&mut self, amount: u64) -> Result<(), ProgramError> {
        let next_gross_in = words_to_u128(self.gross_in)
            .checked_add(amount as u128)
            .ok_or(ChanceryError::ArithmeticOverflow)?;
        let next_action_count = self
            .action_count
            .checked_add(1)
            .ok_or(ChanceryError::ArithmeticOverflow)?;
        let next_net_flow = checked_net_flow_words(
            next_gross_in,
            words_to_u128(self.gross_output_amount),
        )?;

        self.gross_in = u128_to_words(next_gross_in);
        self.net_flow = next_net_flow;
        self.action_count = next_action_count;

        Ok(())
    }

    /// Record a redeem (outflow) event. All fallible calculations complete
    /// before mutation for the same all-or-nothing guarantee as `record_inflow`.
    pub fn record_outflow(&mut self, amount: u64) -> Result<(), ProgramError> {
        let next_gross_output_amount = words_to_u128(self.gross_output_amount)
            .checked_add(amount as u128)
            .ok_or(ChanceryError::ArithmeticOverflow)?;
        let next_action_count = self
            .action_count
            .checked_add(1)
            .ok_or(ChanceryError::ArithmeticOverflow)?;
        let next_net_flow = checked_net_flow_words(
            words_to_u128(self.gross_in),
            next_gross_output_amount,
        )?;

        self.gross_output_amount = u128_to_words(next_gross_output_amount);
        self.net_flow = next_net_flow;
        self.action_count = next_action_count;

        Ok(())
    }

    /// Current gross_in as u128.
    #[inline]
    pub fn gross_in_u128(&self) -> u128  { words_to_u128(self.gross_in) }

    /// Current gross_output_amount as u128.
    #[inline]
    pub fn gross_out_u128(&self) -> u128 { words_to_u128(self.gross_output_amount) }

    /// Current net_flow as i128.
    #[inline]
    pub fn net_flow_i128(&self) -> i128  { words_to_i128(self.net_flow) }
}

/// A usage window that the loaded policy actually caps is mandatory: it must be
/// supplied, non-default, and writable so its running total can accrue and later
/// transactions in the same window observe it. Fail closed - an absent window
/// would skip the volume/action-count check entirely, and a read-only window
/// would pass the check but never record inflow, so the fixed-window cap would never
/// bind. Both are silent bypasses of an enforced limit; reject them here, before
/// any CPI, rather than discovering the problem at record time after settlement.
pub fn require_enforced_window(
    accounts: &[AccountInfo],
    index:    usize,
) -> Result<(), ProgramError> {
    if accounts.len() <= index || accounts[index].key == &Pubkey::default() {
        return Err(ChanceryError::MissingAccount.into());
    }
    if !accounts[index].is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }
    Ok(())
}


fn checked_net_flow_words(gross_in: u128, gross_output_amount: u128) -> Result<[u64; 2], ProgramError> {
    if gross_in >= gross_output_amount {
        let magnitude = gross_in - gross_output_amount;
        let net = i128::try_from(magnitude).map_err(|_| ChanceryError::ArithmeticOverflow)?;
        return Ok(i128_to_words(net));
    }

    let magnitude = gross_output_amount - gross_in;
    let minimum_magnitude = (i128::MAX as u128) + 1;
    if magnitude > minimum_magnitude {
        return Err(ChanceryError::ArithmeticUnderflow.into());
    }
    if magnitude == minimum_magnitude {
        return Ok(i128_to_words(i128::MIN));
    }

    let positive = i128::try_from(magnitude).map_err(|_| ChanceryError::ArithmeticUnderflow)?;
    let net = positive.checked_neg().ok_or(ChanceryError::ArithmeticUnderflow)?;
    Ok(i128_to_words(net))
}

// ─── Enforcement preparation ──────────────────────────────────────────────────

/// Snapshot of a closed period recorded when `roll_forward` advances a stable
/// window. Handlers collect these during the check phase and emit them as
/// canonical `UsageWindowRolled` evidence last, so the overwritten period's
/// final totals reach the historical record.
pub struct ClosedPeriod {
    pub window:                               Pubkey,
    pub scope_hash:                           [u8; 32],
    pub window_kind:                          u8,
    pub previous_window_start_unix_timestamp: i64,
    pub new_window_start_unix_timestamp:      i64,
    pub final_gross_in:                       u128,
    pub final_gross_output_amount:            u128,
    pub final_net_flow:                       i128,
    pub final_action_count:                   u32,
}

/// Current-period totals after roll-forward, for cap checks.
pub struct PreparedWindow {
    pub gross_in:     u128,
    pub gross_out:    u128,
    pub action_count: u32,
}

/// Internal proof that the exact account was canonically verified while its
/// current-period totals were prepared. Private identity fields let a later
/// evidence-last reborrow reuse that derivation without exposing a bump-only
/// loader to instruction modules.
#[derive(Clone, Copy)]
pub(crate) struct PreparedWindowProof {
    pub(crate) gross_in:     u128,
    pub(crate) gross_out:    u128,
    pub(crate) action_count: u32,
    account_key:             Pubkey,
    scope_hash:              [u8; 32],
    window_kind:             u8,
    expected_bump:           u8,
}

impl PreparedWindowProof {
    fn load_mut<'a>(
        &self,
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, UsageWindow>, ProgramError> {
        if account.key != &self.account_key {
            return Err(ChanceryError::InvalidPda.into());
        }

        UsageWindow::load_mut_for_verified_pda(
            account,
            &self.scope_hash,
            self.window_kind,
            self.expected_bump,
        )
    }

    pub(crate) fn record_inflow<'a>(
        &self,
        account: &'a AccountInfo<'a>,
        amount: u64,
    ) -> Result<(), ProgramError> {
        self.load_mut(account)?.record_inflow(amount)
    }

    pub(crate) fn record_outflow<'a>(
        &self,
        account: &'a AccountInfo<'a>,
        amount: u64,
    ) -> Result<(), ProgramError> {
        self.load_mut(account)?.record_outflow(amount)
    }
}

/// Prepare a stable enforced window for a cap check: require it present +
/// writable, verify the stable PDA for (scope_hash, window_kind), load it
/// mutably through the verified loader, and roll it forward to the canonical
/// clock-derived current period. When the roll closes a period, its final
/// totals are appended to `rolled` for evidence emission.
///
/// Every mutation here rolls back atomically if any later check, token CPI, or
/// evidence CPI in the transaction fails. Solana's writable-account locking
/// serializes competing first operations at a period boundary.
pub fn prepare_enforced_window<'a>(
    accounts:       &'a [AccountInfo<'a>],
    index:          usize,
    scope_hash:     &[u8; 32],
    window_kind:    u8,
    unix_timestamp: i64,
    program_id:     &Pubkey,
    rolled:         &mut Vec<ClosedPeriod>,
) -> Result<PreparedWindow, ProgramError> {
    let prepared = prepare_enforced_window_for_recording(
        accounts,
        index,
        scope_hash,
        window_kind,
        unix_timestamp,
        program_id,
        rolled,
    )?;

    Ok(PreparedWindow {
        gross_in:     prepared.gross_in,
        gross_out:    prepared.gross_out,
        action_count: prepared.action_count,
    })
}

pub(crate) fn prepare_enforced_window_for_recording<'a>(
    accounts:       &'a [AccountInfo<'a>],
    index:          usize,
    scope_hash:     &[u8; 32],
    window_kind:    u8,
    unix_timestamp: i64,
    program_id:     &Pubkey,
    rolled:         &mut Vec<ClosedPeriod>,
) -> Result<PreparedWindowProof, ProgramError> {
    require_enforced_window(accounts, index)?;

    let expected_bump =
        UsageWindow::verify_pda(&accounts[index], scope_hash, window_kind, program_id)?;
    let mut window = UsageWindow::load_mut_for_verified_pda(
        &accounts[index],
        scope_hash,
        window_kind,
        expected_bump,
    )?;

    let current_start      = canonical_window_start(window_kind, unix_timestamp)?;
    let previous_start     = window.window_start_unix_timestamp;
    let final_gross_in     = window.gross_in_u128();
    let final_gross_output = window.gross_out_u128();
    let final_net_flow     = window.net_flow_i128();
    let final_action_count = window.action_count;

    if window.roll_forward(current_start)? {
        rolled.push(ClosedPeriod {
            window:                               *accounts[index].key,
            scope_hash:                           *scope_hash,
            window_kind,
            previous_window_start_unix_timestamp: previous_start,
            new_window_start_unix_timestamp:      current_start,
            final_gross_in,
            final_gross_output_amount:            final_gross_output,
            final_net_flow,
            final_action_count,
        });
    }

    Ok(PreparedWindowProof {
        gross_in:     window.gross_in_u128(),
        gross_out:    window.gross_out_u128(),
        action_count: window.action_count,
        account_key:  *accounts[index].key,
        scope_hash:   *scope_hash,
        window_kind,
        expected_bump,
    })
}

// ─── Binding-time creation ────────────────────────────────────────────────────

/// Create a stable usage window at binding time if it does not exist yet.
///
/// Creation is idempotent-by-existence: windows are keyed by `scope_hash`, not
/// policy id, so distinct policies with the same scope share one accumulator -
/// an already-initialized canonical window is verified and left untouched.
/// Never called from the settlement hot path.
///
/// Returns `Ok(true)` when the account was created and initialized here.
pub fn create_usage_window_if_missing<'a>(
    window_account_info:         &'a AccountInfo<'a>,
    payer_account_info:          &'a AccountInfo<'a>,
    system_program_account_info: &'a AccountInfo<'a>,
    scope_hash:                  &[u8; 32],
    window_kind:                 u8,
    unix_timestamp:              i64,
    rent_refund_recipient:       &Pubkey,
    program_id:                  &Pubkey,
) -> Result<bool, ProgramError> {
    let (expected_key, bump) = UsageWindow::pda(scope_hash, window_kind, program_id);

    if window_account_info.key != &expected_key {
        return Err(ChanceryError::InvalidPda.into());
    }

    if !window_account_info.data_is_empty() {
        // Shared-scope sibling policy already bound this window: prove it is
        // the canonical initialized account, then leave its accumulators and
        // period untouched - re-binding must never reset a live cap.
        UsageWindow::load_for_verified_pda(
            window_account_info,
            scope_hash,
            window_kind,
            bump,
        )?;

        return Ok(false);
    }

    create_pda_account(
        payer_account_info,
        window_account_info,
        system_program_account_info,
        program_id,
        &[
            seeds::USAGE_WINDOW,
            scope_hash.as_ref(),
            &[window_kind],
            &[bump],
        ],
        USAGE_WINDOW_SIZE,
    )?;

    let mut window = UsageWindow::load_uninitialized_mut(window_account_info)?;

    window.discriminator               = USAGE_WINDOW_DISCRIMINATOR;
    window.version                     = crate::state_loader::SUPPORTED_STATE_VERSION;
    window.bump                        = bump;
    window.window_kind                 = window_kind;
    window._pad0                       = [0u8; 4];
    window.scope_hash                  = *scope_hash;
    window.window_start_unix_timestamp = canonical_window_start(window_kind, unix_timestamp)?;
    window.gross_in                    = [0, 0];
    window.gross_output_amount         = [0, 0];
    window.net_flow                    = [0, 0];
    window.action_count                = 0;
    window._pad1                       = [0u8; 4];
    window.rent_refund_recipient       = *rent_refund_recipient;
    window._reserved                   = [0u8; 32];

    Ok(true)
}

// ─── Window boundary helpers ──────────────────────────────────────────────────

/// Canonical clock-derived period start for a window kind. Fails closed on an
/// unknown kind. This is the only source of a window's new period start -
/// never an instruction argument.
pub fn canonical_window_start(
    window_kind:    u8,
    unix_timestamp: i64,
) -> Result<i64, ProgramError> {
    Ok(match window_kind {
        window_kind::HOURLY  => hourly_window_start(unix_timestamp),
        window_kind::DAILY   => daily_window_start(unix_timestamp),
        window_kind::WEEKLY  => weekly_window_start(unix_timestamp),
        window_kind::MONTHLY => monthly_window_start(unix_timestamp),
        _ => return Err(ChanceryError::UsageWindowKindInvalid.into()),
    })
}

/// Compute the hourly window start for a unix timestamp (floor to hour).
pub fn hourly_window_start(unix_timestamp: i64) -> i64 {
    unix_timestamp.div_euclid(3_600) * 3_600
}

/// Compute the daily window start (floor to UTC day).
pub fn daily_window_start(unix_timestamp: i64) -> i64 {
    unix_timestamp.div_euclid(86_400) * 86_400
}

/// Compute the 7-day window start (floor to 7-day epoch from unix 0).
pub fn weekly_window_start(unix_timestamp: i64) -> i64 {
    let week_secs = 7 * 86_400;

    unix_timestamp.div_euclid(week_secs) * week_secs
}

/// Compute the 30-day window start (floor to 30-day epoch from unix 0).
pub fn monthly_window_start(unix_timestamp: i64) -> i64 {
    let month_secs = 30 * 86_400;

    unix_timestamp.div_euclid(month_secs) * month_secs
}

#[cfg(test)]
mod tests {
    use super::*;

    fn window() -> UsageWindow {
        UsageWindow {
            discriminator: USAGE_WINDOW_DISCRIMINATOR,
            version: crate::state_loader::SUPPORTED_STATE_VERSION,
            bump: 1,
            window_kind: window_kind::DAILY,
            _pad0: [0u8; 4],
            scope_hash: [0u8; 32],
            window_start_unix_timestamp: 0,
            gross_in: [0u64; 2],
            gross_output_amount: [0u64; 2],
            net_flow: [0u64; 2],
            action_count: 0,
            _pad1: [0u8; 4],
            rent_refund_recipient: Pubkey::default(),
            _reserved: [0u8; 32],
        }
    }

    #[test]
    fn record_inflow_is_atomic_when_action_count_overflows() {
        let mut value = window();
        value.gross_in = u128_to_words(7);
        value.net_flow = i128_to_words(7);
        value.action_count = u32::MAX;
        let before = value;

        assert!(value.record_inflow(1).is_err());
        assert_eq!(value.gross_in, before.gross_in);
        assert_eq!(value.net_flow, before.net_flow);
        assert_eq!(value.action_count, before.action_count);
    }

    #[test]
    fn record_outflow_is_atomic_when_net_flow_is_unrepresentable() {
        let mut value = window();
        value.gross_output_amount = u128_to_words((i128::MAX as u128) + 1);
        value.net_flow = i128_to_words(i128::MIN);
        let before = value;

        assert!(value.record_outflow(1).is_err());
        assert_eq!(value.gross_output_amount, before.gross_output_amount);
        assert_eq!(value.net_flow, before.net_flow);
        assert_eq!(value.action_count, before.action_count);
    }
}

// ─── Size assertion ───────────────────────────────────────────────────────────
const _: () = assert!(
    core::mem::size_of::<UsageWindow>() == USAGE_WINDOW_SIZE,
    "UsageWindow size mismatch - update USAGE_WINDOW_SIZE",
);
