//! Reserve destination status classifier.
//!
//! Status transition rules:
//!   - `destination_flags` is purpose-only; lifecycle lives in the status byte.
//!   - direct execution is limited to restrictive/routine transitions.
//!   - widening and irreversible transitions consume an accepted pending change.
//!   - `approved_by` records the direct authority or pending proposer.

use crate::{
    constants::reserve_destination_status,
    modules::control::change_risk::ConfigChangeRiskClass,
};

pub fn classify_reserve_destination_status_transition(
    old: u8,
    new: u8,
) -> ConfigChangeRiskClass {
    use reserve_destination_status as rs;
    use ConfigChangeRiskClass::*;

    if old == new {
        return RoutineOps;
    }

    if new == rs::DEPRECATED || old == rs::DEPRECATED {
        return Irreversible;
    }

    match (old, new) {
        (rs::ENABLED, rs::DISABLED) => RestrictiveImmediate,
        (rs::DISABLED, rs::ENABLED) => Widening,
        (rs::NONE, rs::ENABLED)     => Widening,  // first-time activation
        (rs::NONE, rs::DISABLED)    => RoutineOps,  // create-as-disabled
        _                           => HighImpact,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enabled_to_disabled_is_restrictive() {
        let r = classify_reserve_destination_status_transition(
            reserve_destination_status::ENABLED,
            reserve_destination_status::DISABLED,
        );

        assert_eq!(r, ConfigChangeRiskClass::RestrictiveImmediate);
    }

    #[test]
    fn disabled_to_enabled_is_widening() {
        let r = classify_reserve_destination_status_transition(
            reserve_destination_status::DISABLED,
            reserve_destination_status::ENABLED,
        );

        assert_eq!(r, ConfigChangeRiskClass::Widening);
    }

    #[test]
    fn deprecation_is_irreversible() {
        let r = classify_reserve_destination_status_transition(
            reserve_destination_status::ENABLED,
            reserve_destination_status::DEPRECATED,
        );

        assert_eq!(r, ConfigChangeRiskClass::Irreversible);
    }


    #[test]
    fn no_op_is_routine() {
        let r = classify_reserve_destination_status_transition(
            reserve_destination_status::ENABLED,
            reserve_destination_status::ENABLED,
        );

        assert_eq!(r, ConfigChangeRiskClass::RoutineOps);
    }

    #[test]
    fn none_to_enabled_is_widening() {
        let r = classify_reserve_destination_status_transition(
            reserve_destination_status::NONE,
            reserve_destination_status::ENABLED,
        );

        assert_eq!(r, ConfigChangeRiskClass::Widening);
    }

    #[test]
    fn any_transition_from_deprecated_is_irreversible() {
        let r = classify_reserve_destination_status_transition(
            reserve_destination_status::DEPRECATED,
            reserve_destination_status::DISABLED,
        );

        assert_eq!(r, ConfigChangeRiskClass::Irreversible);
    }
}
