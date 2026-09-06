//! Evidence-policy transition classification.
//!
//! The policy is field-sensitive: removing requirements or increasing the
//! amount of counterparty-supplied data widens the accepted evidence surface;
//! adding requirements or reducing those allowances is restrictive.

use crate::modules::{
    control::{
        change_detection::{classify_mask_change, MaskSemantics},
        change_risk::{max_risk_many, ConfigChangeRiskClass},
    },
    evidence::state::evidence_policy::EvidencePolicy,
};

fn classify_schema_requirement_change(
    current:  [u8; 32],
    proposed: [u8; 32],
) -> ConfigChangeRiskClass {
    if current == proposed {
        return ConfigChangeRiskClass::RoutineOps;
    }

    let current_is_unset = current == [0u8; 32];
    let proposed_is_unset = proposed == [0u8; 32];

    if current_is_unset && !proposed_is_unset {
        ConfigChangeRiskClass::RestrictiveImmediate
    } else {
        // Removing a schema requirement or replacing one accepted schema with
        // another changes what evidence can satisfy the policy.
        ConfigChangeRiskClass::Widening
    }
}

fn classify_allowance_change(
    current:  bool,
    proposed: bool,
) -> ConfigChangeRiskClass {
    match (current, proposed) {
        (false, true) => ConfigChangeRiskClass::Widening,
        (true, false) => ConfigChangeRiskClass::RestrictiveImmediate,
        _ => ConfigChangeRiskClass::RoutineOps,
    }
}

fn classify_maximum_change(
    current:  u16,
    proposed: u16,
) -> ConfigChangeRiskClass {
    if proposed > current {
        ConfigChangeRiskClass::Widening
    } else if proposed < current {
        ConfigChangeRiskClass::RestrictiveImmediate
    } else {
        ConfigChangeRiskClass::RoutineOps
    }
}

pub fn classify_evidence_policy_update(
    current:  &EvidencePolicy,
    proposed: &EvidencePolicy,
) -> ConfigChangeRiskClass {
    let risks = [
        classify_mask_change(
            current.required_field_mask,
            proposed.required_field_mask,
            MaskSemantics::RequiredList,
        ),
        classify_schema_requirement_change(
            current.counterparty_reporting_schema_hash,
            proposed.counterparty_reporting_schema_hash,
        ),
        classify_allowance_change(
            current.allow_freeform_counterparty_fields != 0,
            proposed.allow_freeform_counterparty_fields != 0,
        ),
        classify_maximum_change(
            current.maximum_freeform_field_count,
            proposed.maximum_freeform_field_count,
        ),
        classify_maximum_change(
            current.maximum_freeform_value_bytes,
            proposed.maximum_freeform_value_bytes,
        ),
        classify_mask_change(
            [current.retention_flags, 0],
            [proposed.retention_flags, 0],
            MaskSemantics::RequiredList,
        ),
    ];

    max_risk_many(&risks)
}

#[cfg(test)]
mod tests {
    use bytemuck::Zeroable;

    use super::*;

    fn policy() -> EvidencePolicy {
        EvidencePolicy::zeroed()
    }

    #[test]
    fn removing_required_fields_is_widening() {
        let mut current = policy();
        current.required_field_mask = [0b11, 0];
        let mut proposed = current;
        proposed.required_field_mask = [0b01, 0];

        assert_eq!(
            classify_evidence_policy_update(&current, &proposed),
            ConfigChangeRiskClass::Widening,
        );
    }

    #[test]
    fn adding_required_fields_is_restrictive() {
        let current = policy();
        let mut proposed = current;
        proposed.required_field_mask = [0b01, 0];

        assert_eq!(
            classify_evidence_policy_update(&current, &proposed),
            ConfigChangeRiskClass::RestrictiveImmediate,
        );
    }

    #[test]
    fn enabling_or_expanding_freeform_reporting_is_widening() {
        let current = policy();
        let mut proposed = current;
        proposed.allow_freeform_counterparty_fields = 1;

        assert_eq!(
            classify_evidence_policy_update(&current, &proposed),
            ConfigChangeRiskClass::Widening,
        );

        let mut current_with_limit = policy();
        current_with_limit.maximum_freeform_field_count = 2;
        let mut expanded = current_with_limit;
        expanded.maximum_freeform_field_count = 3;

        assert_eq!(
            classify_evidence_policy_update(&current_with_limit, &expanded),
            ConfigChangeRiskClass::Widening,
        );
    }

    #[test]
    fn unchanged_policy_is_routine() {
        let current = policy();

        assert_eq!(
            classify_evidence_policy_update(&current, &current),
            ConfigChangeRiskClass::RoutineOps,
        );
    }
}
