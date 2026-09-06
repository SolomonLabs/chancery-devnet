//! Shared current-state transition classification helpers.
//!
//! Modules use these primitives in their own `change_detection.rs` to
//! translate "current state vs proposed state" into a `ConfigChangeRiskClass`.
//!
//! Mask criticality lets callers tune severity per mask. e.g.
//! `pathway.allowed_program_mask` is Critical; `asset.approved_extension_mask`
//! is Standard.

use crate::modules::control::change_risk::ConfigChangeRiskClass;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaskCriticality {
    /// Default. e.g. asset_config.approved_extension_mask
    Standard,
    /// e.g. pathway.allowed_program_mask, allowed_instruction_mask
    Critical,
}

#[derive(Clone, Copy, Debug)]
pub enum MaskSemantics {
    /// Bits represent things permitted. Adding bits widens; removing narrows.
    AllowList(MaskCriticality),
    /// Bits represent things forbidden. Adding bits narrows; removing widens.
    ForbiddenList,
    /// Bits represent things required. Adding bits narrows; removing widens.
    RequiredList,
}

#[inline]
fn mask_diff(old: [u64; 2], new: [u64; 2]) -> ([u64; 2], [u64; 2]) {
    let added   = [new[0] & !old[0], new[1] & !old[1]];
    let removed = [old[0] & !new[0], old[1] & !new[1]];

    (added, removed)
}

#[inline]
fn mask_nonzero(m: [u64; 2]) -> bool {
    m[0] != 0 || m[1] != 0
}

/// Classify a u128-as-[u64;2] mask change.
pub fn classify_mask_change(
    old:       [u64; 2],
    new:       [u64; 2],
    semantics: MaskSemantics,
) -> ConfigChangeRiskClass {
    let (added, removed) = mask_diff(old, new);
    match semantics {
        MaskSemantics::AllowList(crit) => {
            if mask_nonzero(added) {
                match crit {
                    MaskCriticality::Standard => ConfigChangeRiskClass::Widening,
                    MaskCriticality::Critical => ConfigChangeRiskClass::HighImpact,
                }
            } else if mask_nonzero(removed) {
                ConfigChangeRiskClass::RestrictiveImmediate
            } else {
                ConfigChangeRiskClass::RoutineOps
            }
        }
        MaskSemantics::ForbiddenList => {
            if mask_nonzero(removed) {
                ConfigChangeRiskClass::Widening
            } else if mask_nonzero(added) {
                ConfigChangeRiskClass::RestrictiveImmediate
            } else {
                ConfigChangeRiskClass::RoutineOps
            }
        }
        MaskSemantics::RequiredList => {
            if mask_nonzero(removed) {
                ConfigChangeRiskClass::Widening
            } else if mask_nonzero(added) {
                ConfigChangeRiskClass::RestrictiveImmediate
            } else {
                ConfigChangeRiskClass::RoutineOps
            }
        }
    }
}

/// Classify a numeric-cap change. None = no change; zero means unbounded.
/// Finite -> unbounded and finite increases widen authority. Unbounded ->
/// finite and finite decreases are restrictive containment.
pub fn classify_optional_u64_cap_change(
    old_value: u64,
    new_value: Option<u64>,
) -> ConfigChangeRiskClass {
    match new_value {
        None => ConfigChangeRiskClass::RoutineOps,
        Some(v) if v == old_value => ConfigChangeRiskClass::RoutineOps,
        Some(0) => ConfigChangeRiskClass::Widening,
        Some(_) if old_value == 0 => ConfigChangeRiskClass::RestrictiveImmediate,
        Some(v) if v > old_value => ConfigChangeRiskClass::Widening,
        Some(_) => ConfigChangeRiskClass::RestrictiveImmediate,
    }
}

/// Same shape as u64 but for u32 caps.
pub fn classify_optional_u32_cap_change(
    old_value: u32,
    new_value: Option<u32>,
) -> ConfigChangeRiskClass {
    classify_optional_u64_cap_change(old_value as u64, new_value.map(|v| v as u64))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowlist_added_bit_critical_is_high_impact() {
        let r = classify_mask_change(
            [0, 0],
            [1, 0],
            MaskSemantics::AllowList(MaskCriticality::Critical),
        );

        assert_eq!(r, ConfigChangeRiskClass::HighImpact);
    }

    #[test]
    fn allowlist_added_bit_standard_is_widening() {
        let r = classify_mask_change(
            [0, 0],
            [1, 0],
            MaskSemantics::AllowList(MaskCriticality::Standard),
        );

        assert_eq!(r, ConfigChangeRiskClass::Widening);
    }

    #[test]
    fn allowlist_removed_bit_is_restrictive() {
        let r = classify_mask_change(
            [1, 0],
            [0, 0],
            MaskSemantics::AllowList(MaskCriticality::Critical),
        );

        assert_eq!(r, ConfigChangeRiskClass::RestrictiveImmediate);
    }

    #[test]
    fn forbidden_list_removed_bit_is_widening() {
        let r = classify_mask_change(
            [1, 0],
            [0, 0],
            MaskSemantics::ForbiddenList,
        );

        assert_eq!(r, ConfigChangeRiskClass::Widening);
    }

    #[test]
    fn forbidden_list_added_bit_is_restrictive() {
        let r = classify_mask_change(
            [0, 0],
            [1, 0],
            MaskSemantics::ForbiddenList,
        );

        assert_eq!(r, ConfigChangeRiskClass::RestrictiveImmediate);
    }

    #[test]
    fn required_list_removed_bit_is_widening() {
        let r = classify_mask_change(
            [1, 0],
            [0, 0],
            MaskSemantics::RequiredList,
        );

        assert_eq!(r, ConfigChangeRiskClass::Widening);
    }

    #[test]
    fn cap_increase_is_widening() {
        let r = classify_optional_u64_cap_change(100, Some(200));

        assert_eq!(r, ConfigChangeRiskClass::Widening);
    }

    #[test]
    fn cap_decrease_is_restrictive() {
        let r = classify_optional_u64_cap_change(200, Some(100));

        assert_eq!(r, ConfigChangeRiskClass::RestrictiveImmediate);
    }

    #[test]
    fn cap_no_change_is_routine() {
        let r = classify_optional_u64_cap_change(100, Some(100));

        assert_eq!(r, ConfigChangeRiskClass::RoutineOps);
    }

    #[test]
    fn cap_none_is_routine() {
        let r = classify_optional_u64_cap_change(100, None);

        assert_eq!(r, ConfigChangeRiskClass::RoutineOps);
    }

    #[test]
    fn unbounded_cap_to_finite_is_restrictive() {
        let r = classify_optional_u64_cap_change(0, Some(100));

        assert_eq!(r, ConfigChangeRiskClass::RestrictiveImmediate);
    }

    #[test]
    fn finite_cap_to_unbounded_is_widening() {
        let r = classify_optional_u64_cap_change(100, Some(0));

        assert_eq!(r, ConfigChangeRiskClass::Widening);
    }

    #[test]
    fn unbounded_cap_unchanged_is_routine() {
        let r = classify_optional_u64_cap_change(0, Some(0));

        assert_eq!(r, ConfigChangeRiskClass::RoutineOps);
    }
}
