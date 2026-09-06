//! Module-status transition classifier.
//!
//! Classifies a `(module_id, old_status, new_status)` triple into a
//! `ConfigChangeRiskClass`. Per
//!   - Deprecation is one-way → Irreversible (NOT RestrictiveImmediate as
//!     the specification §11.4 originally stated)
//!   - DISABLED → ACTIVE depends on module class (operational vs dangerous)

use crate::{
    constants::{module, module_status},
    modules::control::change_risk::ConfigChangeRiskClass,
};

/// Module class for risk-classification purposes. Operational MVP modules
/// (LIMITS, FEES, etc.) are normal; later/dangerous modules require Dangerous
/// on activation.
///
/// `MIGRATION` is intentionally not in this set. Its module activation is the
/// reviewed, reversible opening step; the legacy migration config is a separate
/// one-shot admin action, and operators are expected to disable the module again
/// after the migration window closes.
fn module_class_is_dangerous(module_id: u8) -> bool {
    matches!(module_id,
          module::CROSS_CHAIN
        | module::ENFORCEMENT
        | module::INSURANCE
        | module::COMPARTMENTS
        | module::PROVENANCE
    )
}

pub fn classify_module_status_transition(
    module_id: u8,
    old:       u8,
    new:       u8,
) -> ConfigChangeRiskClass {
    use module_status as st;
    use ConfigChangeRiskClass::*;

    if old == new {
        return RoutineOps;
    }

    // Deprecation is one-way and irreversible.
    if new == st::DEPRECATED {
        return Irreversible;
    }

    // DEPRECATED → anything: effectively forbidden, but report as Irreversible
    // so the caller surfaces it. The actual handler additionally rejects.
    if old == st::DEPRECATED {
        return Irreversible;
    }

    // Restrictive transitions from ACTIVE close some or all execution surface.
    if old == st::ACTIVE && (new == st::DISABLED
        || new == st::EMERGENCY_DISABLED
        || new == st::ADMIN_ONLY)
    {
        return RestrictiveImmediate;
    }

    // Any transition out of EMERGENCY_DISABLED that reopens executable surface
    // reverses emergency containment. ACTIVE reopens every instruction and
    // ADMIN_ONLY reopens the administrative surface, so both require review.
    if old == st::EMERGENCY_DISABLED && matches!(new, st::ACTIVE | st::ADMIN_ONLY) {
        if module_class_is_dangerous(module_id) {
            return Dangerous;
        }
        return HighImpact;
    }

    // Moving from an executable or ordinarily-disabled posture into the
    // emergency posture is always immediately restrictive. In particular,
    // ADMIN_ONLY still permits admin instructions and must remain haltable by
    // the emergency authority without a governance timelock.
    if new == st::EMERGENCY_DISABLED
        && matches!(old, st::ACTIVE | st::ADMIN_ONLY | st::DISABLED)
    {
        return RestrictiveImmediate;
    }

    // DISABLED/ADMIN_ONLY → ACTIVE: depends on module class.
    if matches!(old, st::DISABLED | st::ADMIN_ONLY) && new == st::ACTIVE {
        return if module_class_is_dangerous(module_id) { Dangerous } else { Widening };
    }

    // DISABLED → ADMIN_ONLY: opening admin surface only.
    if old == st::DISABLED && new == st::ADMIN_ONLY {
        return if module_class_is_dangerous(module_id) { HighImpact } else { Widening };
    }

    // ADMIN_ONLY → DISABLED: closing admin.
    if old == st::ADMIN_ONLY && new == st::DISABLED {
        return RestrictiveImmediate;
    }

    // Preserve emergency-origin risk across composed transitions. For a
    // dangerous module, clearing the emergency label must itself consume a
    // Dangerous ceremony; otherwise EMERGENCY_DISABLED -> DISABLED ->
    // ADMIN_ONLY composes two HighImpact edges into a faster reopening path.
    if old == st::EMERGENCY_DISABLED && new == st::DISABLED {
        return if module_class_is_dangerous(module_id) {
            Dangerous
        } else {
            HighImpact
        };
    }

    // Catch-all for unhandled (status_a, status_b) pairs.
    HighImpact
}

/// Default per-module status at chancery deployment.
/// Operational MVP modules → ACTIVE. Later/dangerous → DISABLED.
pub fn default_status_for(module_id: u8) -> u8 {
    if matches!(module_id,
          module::CORE
        | module::EVENTS_CPI
        | module::PERMISSIONS
        | module::PATHWAY
        | module::SETTLEMENT
        | module::LIMITS
        | module::EVIDENCE
        | module::FEES
        | module::RESERVE
        | module::CONTROL
        | module::ISSUED_TOKEN_CONTROL
    ) {
        module_status::ACTIVE
    } else {
        module_status::DISABLED
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use module_status as st;

    #[test]
    fn active_to_disabled_is_restrictive() {
        let r = classify_module_status_transition(module::CROSS_CHAIN, st::ACTIVE, st::DISABLED);

        assert_eq!(r, ConfigChangeRiskClass::RestrictiveImmediate);
    }

    #[test]
    fn active_to_emergency_disabled_is_restrictive() {
        let r = classify_module_status_transition(module::PERMISSIONS, st::ACTIVE, st::EMERGENCY_DISABLED);

        assert_eq!(r, ConfigChangeRiskClass::RestrictiveImmediate);
    }

    #[test]
    fn disabled_to_active_dangerous_module_is_dangerous() {
        let r = classify_module_status_transition(module::CROSS_CHAIN, st::DISABLED, st::ACTIVE);

        assert_eq!(r, ConfigChangeRiskClass::Dangerous);
    }

    #[test]
    fn disabled_to_active_operational_module_is_widening() {
        let r = classify_module_status_transition(module::PERMISSIONS, st::DISABLED, st::ACTIVE);

        assert_eq!(r, ConfigChangeRiskClass::Widening);
    }

    #[test]
    fn migration_activation_is_widening_by_design() {
        let r = classify_module_status_transition(module::MIGRATION, st::DISABLED, st::ACTIVE);

        assert_eq!(r, ConfigChangeRiskClass::Widening);
    }

    #[test]
    fn dangerous_module_emergency_reactivation_remains_dangerous() {
        assert_eq!(
            classify_module_status_transition(
                module::CROSS_CHAIN,
                st::EMERGENCY_DISABLED,
                st::ACTIVE,
            ),
            ConfigChangeRiskClass::Dangerous,
        );
        assert_eq!(
            classify_module_status_transition(
                module::CROSS_CHAIN,
                st::EMERGENCY_DISABLED,
                st::ADMIN_ONLY,
            ),
            ConfigChangeRiskClass::Dangerous,
        );
    }

    #[test]
    fn operational_module_emergency_reactivation_is_high_impact() {
        assert_eq!(
            classify_module_status_transition(
                module::PERMISSIONS,
                st::EMERGENCY_DISABLED,
                st::ACTIVE,
            ),
            ConfigChangeRiskClass::HighImpact,
        );
        assert_eq!(
            classify_module_status_transition(
                module::PERMISSIONS,
                st::EMERGENCY_DISABLED,
                st::ADMIN_ONLY,
            ),
            ConfigChangeRiskClass::HighImpact,
        );
    }

    #[test]
    fn admin_only_to_emergency_disabled_is_immediately_restrictive() {
        let r = classify_module_status_transition(
            module::CROSS_CHAIN,
            st::ADMIN_ONLY,
            st::EMERGENCY_DISABLED,
        );

        assert_eq!(r, ConfigChangeRiskClass::RestrictiveImmediate);
    }

    #[test]
    fn disabled_to_emergency_disabled_is_immediately_restrictive() {
        let r = classify_module_status_transition(
            module::CROSS_CHAIN,
            st::DISABLED,
            st::EMERGENCY_DISABLED,
        );

        assert_eq!(r, ConfigChangeRiskClass::RestrictiveImmediate);
    }

    #[test]
    fn deprecation_is_irreversible() {
        let r = classify_module_status_transition(module::COMPARTMENTS, st::ACTIVE, st::DEPRECATED);

        assert_eq!(r, ConfigChangeRiskClass::Irreversible);
    }

    #[test]
    fn deprecated_to_active_is_irreversible_class_for_handler_rejection() {
        let r = classify_module_status_transition(module::COMPARTMENTS, st::DEPRECATED, st::ACTIVE);

        assert_eq!(r, ConfigChangeRiskClass::Irreversible);
    }

    #[test]
    fn no_op_transition_is_routine() {
        let r = classify_module_status_transition(module::CROSS_CHAIN, st::DISABLED, st::DISABLED);

        assert_eq!(r, ConfigChangeRiskClass::RoutineOps);
    }

    #[test]
    fn default_status_for_core_modules_is_active() {
        assert_eq!(default_status_for(module::CORE), module_status::ACTIVE);
        assert_eq!(default_status_for(module::EVIDENCE), module_status::ACTIVE);
        assert_eq!(default_status_for(module::SETTLEMENT), module_status::ACTIVE);
    }

    #[test]
    fn default_status_for_dangerous_modules_is_disabled() {
        assert_eq!(default_status_for(module::CROSS_CHAIN), module_status::DISABLED);
        assert_eq!(default_status_for(module::ENFORCEMENT), module_status::DISABLED);
        assert_eq!(default_status_for(module::INSURANCE), module_status::DISABLED);
        // The legacy-migration one-shot is irreversible if misconfigured;
        // requiring a timelocked DISABLED -> ACTIVE governance ceremony first
        // makes the reversible step come before the irreversible one.
        assert_eq!(default_status_for(module::MIGRATION), module_status::DISABLED);
    }
    #[test]
    fn emergency_disabled_to_disabled_is_high_impact() {
        let r = classify_module_status_transition(
            module::PERMISSIONS,
            st::EMERGENCY_DISABLED,
            st::DISABLED,
        );

        assert_eq!(r, ConfigChangeRiskClass::HighImpact);
    }

    #[test]
    fn dangerous_module_emergency_clear_is_dangerous() {
        let r = classify_module_status_transition(
            module::CROSS_CHAIN,
            st::EMERGENCY_DISABLED,
            st::DISABLED,
        );

        assert_eq!(r, ConfigChangeRiskClass::Dangerous);
    }

    #[test]
    fn dangerous_module_split_reopen_cannot_beat_dangerous_minimum() {
        use crate::modules::control::pending_change::minimum_timelock_seconds_for_risk;

        let dangerous_minimum =
            minimum_timelock_seconds_for_risk(ConfigChangeRiskClass::Dangerous);
        let clear_emergency = minimum_timelock_seconds_for_risk(
            classify_module_status_transition(
                module::CROSS_CHAIN,
                st::EMERGENCY_DISABLED,
                st::DISABLED,
            ),
        );
        let open_admin = minimum_timelock_seconds_for_risk(
            classify_module_status_transition(
                module::CROSS_CHAIN,
                st::DISABLED,
                st::ADMIN_ONLY,
            ),
        );

        // If proposals are created sequentially, delays add. If a future-state
        // proposal is pre-aged, the strongest lower bound is the slower edge.
        assert!(clear_emergency + open_admin >= dangerous_minimum);
        assert!(core::cmp::max(clear_emergency, open_admin) >= dangerous_minimum);
    }
}
