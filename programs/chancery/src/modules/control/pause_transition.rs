//! Shared pause set-path transition validation.
//!
//! Set operations are monotonic: they may add pause bits and may preserve or
//! strengthen expiry, but must never shorten an active pause. Emergency
//! authority may only write activation metadata for a fresh activation; when
//! a pause is already active it can add bits but cannot replace metadata.

use solana_program_error::ProgramError;

use crate::error::ChanceryError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PauseSetTransition {
    pub new_bits:            u64,
    pub new_expires_at_slot: u64,
    pub write_metadata:      bool,
}

/// Outcome of a clear request.
///
/// Audit #37 (PR #26) settled that `expires_at_slot` MAY be written on the
/// clear path: clear requires governance/ops, who can lift any bit anyway, so
/// adjusting the surviving episode's expiry while clearing one bit is a
/// legitimate capability. That capability is preserved here.
///
/// What is rejected is the *ambiguous hidden all-clear*: a clear that supplies
/// an already-elapsed expiry while bits remain set. `assert_not_paused_for`
/// treats the ENTIRE bitfield as lifted once the shared expiry passes, so that
/// form would report surviving bits while making all of them ineffective.
/// Governance/ops retain the accepted audit-#37 capability to change expiry on
/// the clear path, including an expiry-only update with `requested_bits == 0`.
///
///   - Surviving bits, expiry 0       -> preserve the episode's expiry.
///   - Surviving bits, expiry > now   -> apply it to the surviving episode.
///   - Surviving bits, expiry <= now  -> reject the hidden all-clear.
///   - No surviving bits              -> canonicalise the episode to zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PauseClearTransition {
    pub new_bits:            u64,
    pub new_expires_at_slot: u64,
    /// True only on final clear, where the episode record is canonicalised.
    pub canonicalise:        bool,
}

/// Validate and compute a clear while preserving the audit-accepted authority
/// to adjust expiry on the clear path.
pub fn compute_pause_clear_transition(
    current_bits:              u64,
    current_expires_at_slot:   u64,
    requested_bits:            u64,
    requested_expires_at_slot: u64,
    current_slot:              u64,
) -> Result<PauseClearTransition, ProgramError> {
    let new_bits = current_bits & !requested_bits;

    if new_bits == 0 {
        // Episode over. Nothing survives for an expiry to govern, so the state
        // is canonical regardless of the caller's now-irrelevant expiry input.
        return Ok(PauseClearTransition {
            new_bits:            0,
            new_expires_at_slot: 0,
            canonicalise:        true,
        });
    }

    let new_expires_at_slot = if requested_expires_at_slot == 0 {
        current_expires_at_slot
    } else if requested_expires_at_slot > current_slot {
        requested_expires_at_slot
    } else {
        return Err(ChanceryError::PauseStateInvalidParameters.into());
    };

    Ok(PauseClearTransition {
        new_bits,
        new_expires_at_slot,
        canonicalise: false,
    })
}

pub fn compute_pause_set_transition(
    current_bits:             u64,
    current_expires_at_slot:  u64,
    requested_bits:           u64,
    proposed_expires_at_slot: u64,
    current_slot:             u64,
    is_emergency_authority:   bool,
) -> Result<PauseSetTransition, ProgramError> {
    if requested_bits == 0 {
        return Err(ChanceryError::PauseStateInvalidParameters.into());
    }

    let current_is_active = current_bits != 0
        && (current_expires_at_slot == 0 || current_slot < current_expires_at_slot);

    validate_pause_set_expiry(
        proposed_expires_at_slot,
        current_expires_at_slot,
        current_slot,
        current_is_active,
    )?;

    Ok(PauseSetTransition {
        new_bits: if current_is_active {
            current_bits | requested_bits
        } else {
            requested_bits
        },
        new_expires_at_slot: proposed_expires_at_slot,
        // Emergency additions preserve the opener/reason metadata of the
        // active episode, but expiry is still updated monotonically above.
        write_metadata: !(is_emergency_authority && current_is_active),
    })
}

pub fn validate_pause_set_expiry(
    proposed_expires_at_slot: u64,
    current_expires_at_slot:  u64,
    current_slot:             u64,
    current_is_active:        bool,
) -> Result<(), ProgramError> {
    if proposed_expires_at_slot != 0 && proposed_expires_at_slot <= current_slot {
        return Err(ChanceryError::PauseStateInvalidParameters.into());
    }

    if !current_is_active {
        return Ok(());
    }

    if current_expires_at_slot == 0 {
        if proposed_expires_at_slot != 0 {
            return Err(ChanceryError::PauseStateInvalidParameters.into());
        }

        return Ok(());
    }

    if proposed_expires_at_slot != 0
        && proposed_expires_at_slot < current_expires_at_slot
    {
        return Err(ChanceryError::PauseStateInvalidParameters.into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zero_bit_set() {
        assert!(compute_pause_set_transition(0, 0, 0, 0, 10, false).is_err());
    }

    #[test]
    fn rejects_backdated_fresh_expiry() {
        assert!(compute_pause_set_transition(0, 0, 1, 10, 10, false).is_err());
    }

    #[test]
    fn admin_cannot_introduce_expiry_on_active_no_expiry_pause() {
        assert!(compute_pause_set_transition(1, 0, 2, 100, 10, false).is_err());
    }

    #[test]
    fn admin_can_remove_expiry_or_extend_it() {
        assert!(compute_pause_set_transition(1, 100, 2, 0, 10, false).is_ok());
        assert!(compute_pause_set_transition(1, 100, 2, 100, 10, false).is_ok());
        assert!(compute_pause_set_transition(1, 100, 2, 101, 10, false).is_ok());
    }

    #[test]
    fn admin_cannot_shorten_active_expiry() {
        assert!(compute_pause_set_transition(1, 100, 2, 99, 10, false).is_err());
    }

    #[test]
    fn emergency_preserves_metadata_and_extends_active_pause() {
        let transition = compute_pause_set_transition(1, 100, 2, 250, 10, true).unwrap();

        assert_eq!(transition.new_bits, 3);
        assert_eq!(transition.new_expires_at_slot, 250);
        assert!(!transition.write_metadata);
    }

    #[test]
    fn emergency_can_make_active_finite_pause_indefinite() {
        let transition = compute_pause_set_transition(1, 100, 2, 0, 10, true).unwrap();

        assert_eq!(transition.new_bits, 3);
        assert_eq!(transition.new_expires_at_slot, 0);
        assert!(!transition.write_metadata);
    }

    #[test]
    fn emergency_cannot_inherit_or_shorten_stale_active_expiry() {
        assert!(compute_pause_set_transition(1, 100, 2, 99, 10, true).is_err());
        assert!(compute_pause_set_transition(1, 100, 2, 10, 10, true).is_err());
    }

    #[test]
    fn emergency_starts_fresh_episode_after_expiry() {
        let transition = compute_pause_set_transition(1, 10, 2, 100, 10, true).unwrap();

        assert_eq!(transition.new_bits, 2);
        assert_eq!(transition.new_expires_at_slot, 100);
        assert!(transition.write_metadata);
    }

    #[test]
    fn governance_starts_fresh_episode_after_expiry() {
        let transition = compute_pause_set_transition(1, 10, 4, 100, 10, false).unwrap();

        assert_eq!(transition.new_bits, 4);
        assert_eq!(transition.new_expires_at_slot, 100);
        assert!(transition.write_metadata);
    }

    #[test]
    fn active_episode_still_accumulates_requested_bits() {
        let transition = compute_pause_set_transition(1, 100, 2, 100, 10, false).unwrap();

        assert_eq!(transition.new_bits, 3);
        assert_eq!(transition.new_expires_at_slot, 100);
        assert!(transition.write_metadata);
    }

    // ── Clear path (P-B2, reconciled with audit #37) ──────────────────────────

    #[test]
    fn zero_bit_clear_can_adjust_expiry_without_changing_bits() {
        let t = compute_pause_clear_transition(0b11, 100, 0, 250, 50).unwrap();

        assert_eq!(t.new_bits, 0b11);
        assert_eq!(t.new_expires_at_slot, 250);
        assert!(!t.canonicalise);
    }

    #[test]
    fn zero_bit_clear_rejects_an_elapsed_expiry() {
        assert!(compute_pause_clear_transition(0b11, 100, 0, 50, 50).is_err());
    }

    #[test]
    fn partial_clear_with_absent_expiry_preserves_the_episode_expiry() {
        // MINT|REDEEM active with expiry 100; clear MINT only, supplying no
        // expiry. REDEEM keeps the original window.
        let t = compute_pause_clear_transition(0b11, 100, 0b01, 0, 50).unwrap();

        assert_eq!(t.new_bits, 0b10);
        assert_eq!(t.new_expires_at_slot, 100);
        assert!(!t.canonicalise);
    }

    #[test]
    fn partial_clear_may_adjust_the_surviving_expiry() {
        // Audit #37 settled that governance/ops MAY set expiry on the clear
        // path. A future expiry is a deliberate adjustment and is accepted.
        let t = compute_pause_clear_transition(0b11, 100, 0b01, 250, 50).unwrap();

        assert_eq!(t.new_bits, 0b10);
        assert_eq!(t.new_expires_at_slot, 250);
        assert!(!t.canonicalise);
    }

    #[test]
    fn partial_clear_rejects_an_elapsed_expiry() {
        // The accident: clearing MINT with a stale expiry would leave REDEEM
        // set but inert. Governance can clear REDEEM explicitly instead.
        assert!(compute_pause_clear_transition(0b11, 100, 0b01, 50, 50).is_err());
        assert!(compute_pause_clear_transition(0b11, 100, 0b01, 1, 50).is_err());
    }

    #[test]
    fn final_clear_canonicalises_the_episode() {
        let t = compute_pause_clear_transition(0b01, 100, 0b01, 0, 50).unwrap();

        assert_eq!(t.new_bits, 0);
        assert_eq!(t.new_expires_at_slot, 0);
        assert!(t.canonicalise);
    }

    #[test]
    fn final_clear_ignores_a_supplied_expiry_and_canonicalises() {
        let t = compute_pause_clear_transition(0b01, 100, 0b01, 250, 50).unwrap();

        assert_eq!(t.new_bits, 0);
        assert_eq!(t.new_expires_at_slot, 0);
        assert!(t.canonicalise);
    }

    #[test]
    fn clearing_unset_bits_is_a_partial_clear_not_a_final_one() {
        // Clearing RESERVE when only MINT is set leaves MINT set: the episode
        // continues and its expiry must survive.
        let t = compute_pause_clear_transition(0b01, 100, 0b100, 0, 50).unwrap();

        assert_eq!(t.new_bits, 0b01);
        assert_eq!(t.new_expires_at_slot, 100);
        assert!(!t.canonicalise);
    }

    #[test]
    fn clear_superset_canonicalises() {
        let t = compute_pause_clear_transition(0b11, 100, u64::MAX, 0, 50).unwrap();

        assert_eq!(t.new_bits, 0);
        assert_eq!(t.new_expires_at_slot, 0);
        assert!(t.canonicalise);
    }
}
