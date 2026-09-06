//! Risk classification for config-changing instructions.
//!
//! Every mutable handler classifies its proposed change as one of six risk
//! classes. Direct handlers permit only RestrictiveImmediate and RoutineOps.
//! Widening, HighImpact, Dangerous, and Irreversible changes require the
//! PendingConfigChange flow.
//!
//! Per Widening+ requires governance authority.

use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;

use crate::{
    error::ChanceryError,
    modules::core::state::chancery_config::ChanceryConfig,
};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigChangeRiskClass {
    RestrictiveImmediate = 0,
    RoutineOps           = 1,
    Widening             = 2,
    HighImpact           = 3,
    Dangerous            = 4,
    Irreversible         = 5,
}

impl ConfigChangeRiskClass {
    /// True when the change must go through propose/accept/consume.
    #[inline]
    pub fn requires_timelock(self) -> bool {
        !matches!(self, Self::RestrictiveImmediate | Self::RoutineOps)
    }

    /// True when only governance authority may sign the change.
    /// Per Widening and above require governance.
    #[inline]
    pub fn requires_governance(self) -> bool {
        matches!(self,
            Self::Widening | Self::HighImpact | Self::Dangerous | Self::Irreversible)
    }

    #[inline]
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn try_from_u8(v: u8) -> Result<Self, ChanceryError> {
        match v {
            0 => Ok(Self::RestrictiveImmediate),
            1 => Ok(Self::RoutineOps),
            2 => Ok(Self::Widening),
            3 => Ok(Self::HighImpact),
            4 => Ok(Self::Dangerous),
            5 => Ok(Self::Irreversible),
            _ => Err(ChanceryError::ConfigChangeInvalidRiskClass),
        }
    }
}

#[inline]
fn risk_priority(risk: ConfigChangeRiskClass) -> u8 {
    match risk {
        ConfigChangeRiskClass::RoutineOps           => 0,
        ConfigChangeRiskClass::RestrictiveImmediate => 1,
        ConfigChangeRiskClass::Widening             => 2,
        ConfigChangeRiskClass::HighImpact           => 3,
        ConfigChangeRiskClass::Dangerous            => 4,
        ConfigChangeRiskClass::Irreversible         => 5,
    }
}

#[inline]
pub fn max_risk(a: ConfigChangeRiskClass, b: ConfigChangeRiskClass) -> ConfigChangeRiskClass {
    if risk_priority(a) >= risk_priority(b) { a } else { b }
}

#[inline]
pub fn max_risk_many(values: &[ConfigChangeRiskClass]) -> ConfigChangeRiskClass {
    let mut acc = ConfigChangeRiskClass::RoutineOps;

    for &v in values {
        acc = max_risk(acc, v);
    }

    acc
}

/// Direct-update gate. Allows RestrictiveImmediate / RoutineOps; rejects
/// Widening+ with `ConfigChangeRequiresTimelock`. Widening+ direct rejection
/// is BEFORE governance check so callers get the most specific error.
///
/// Caller must already have verified `authority_account_info.is_signer`.
pub fn assert_direct_config_change_allowed(
    risk:                   ConfigChangeRiskClass,
    chancery_config:        &ChanceryConfig,
    authority_account_info: &AccountInfo,
) -> ProgramResult {
    if !authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if risk.requires_timelock() {
        return Err(ChanceryError::ConfigChangeRequiresTimelock.into());
    }

    if risk.requires_governance() {
        // Defensive - requires_timelock should already cover Widening+.
        if authority_account_info.key != &chancery_config.governance_authority {
            return Err(ChanceryError::ConfigChangeWideningRequiresGovernance.into());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restrictive_immediate_does_not_require_timelock() {
        assert!(!ConfigChangeRiskClass::RestrictiveImmediate.requires_timelock());
        assert!(!ConfigChangeRiskClass::RestrictiveImmediate.requires_governance());
    }

    #[test]
    fn routine_ops_does_not_require_timelock() {
        assert!(!ConfigChangeRiskClass::RoutineOps.requires_timelock());
        assert!(!ConfigChangeRiskClass::RoutineOps.requires_governance());
    }

    #[test]
    fn widening_requires_timelock_and_governance() {
        let r = ConfigChangeRiskClass::Widening;

        assert!(r.requires_timelock());
        assert!(r.requires_governance());
    }

    #[test]
    fn high_impact_requires_timelock_and_governance() {
        let r = ConfigChangeRiskClass::HighImpact;

        assert!(r.requires_timelock());
        assert!(r.requires_governance());
    }

    #[test]
    fn dangerous_requires_timelock_and_governance() {
        let r = ConfigChangeRiskClass::Dangerous;

        assert!(r.requires_timelock());
        assert!(r.requires_governance());
    }

    #[test]
    fn irreversible_requires_timelock_and_governance() {
        let r = ConfigChangeRiskClass::Irreversible;
        
        assert!(r.requires_timelock());
        assert!(r.requires_governance());
    }

    #[test]
    fn max_risk_picks_higher() {
        assert_eq!(
            max_risk(
                ConfigChangeRiskClass::RestrictiveImmediate,
                ConfigChangeRiskClass::RoutineOps,
            ),
            ConfigChangeRiskClass::RestrictiveImmediate,
        );
        assert_eq!(
            max_risk(ConfigChangeRiskClass::Widening, ConfigChangeRiskClass::RoutineOps),
            ConfigChangeRiskClass::Widening
        );
        assert_eq!(
            max_risk(ConfigChangeRiskClass::Dangerous, ConfigChangeRiskClass::Widening),
            ConfigChangeRiskClass::Dangerous
        );
        assert_eq!(
            max_risk(ConfigChangeRiskClass::Irreversible, ConfigChangeRiskClass::Dangerous),
            ConfigChangeRiskClass::Irreversible
        );
    }

    #[test]
    fn max_risk_many_finds_max() {
        let restrictive = [
            ConfigChangeRiskClass::RoutineOps,
            ConfigChangeRiskClass::RestrictiveImmediate,
            ConfigChangeRiskClass::RoutineOps,
        ];
        assert_eq!(
            max_risk_many(&restrictive),
            ConfigChangeRiskClass::RestrictiveImmediate,
        );

        let high_impact = [
            ConfigChangeRiskClass::RestrictiveImmediate,
            ConfigChangeRiskClass::Widening,
            ConfigChangeRiskClass::RoutineOps,
            ConfigChangeRiskClass::HighImpact,
        ];
        assert_eq!(
            max_risk_many(&high_impact),
            ConfigChangeRiskClass::HighImpact,
        );
    }

    #[test]
    fn try_from_u8_rejects_invalid() {
        assert!(ConfigChangeRiskClass::try_from_u8(6).is_err());
        assert!(ConfigChangeRiskClass::try_from_u8(0xFF).is_err());
    }

    #[test]
    fn try_from_u8_round_trip() {
        for v in 0..=5u8 {
            let r = ConfigChangeRiskClass::try_from_u8(v).unwrap();
            assert_eq!(r.as_u8(), v);
        }
    }
}
