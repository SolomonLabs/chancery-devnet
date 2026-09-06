//! Permission-record semantic value and transition classification.
//!
//! Classification is based on capability delta, not the caller's title:
//!   - creating a record or adding ordinary role bits is Widening;
//!   - every capability-widening transition whose resulting role set contains
//!     a dangerous bit is Dangerous;
//!   - removing role bits or shortening expiry is restrictive;
//!   - extending expiry is Widening;
//!   - removing a finite expiry is HighImpact.
//!
//! Dynamic provenance fields (`issued_at_unix_timestamp`, `granted_by`) are
//! deliberately excluded from the value hash. They do not participate in
//! permission consumption and would make an accepted proposal impossible to
//! reproduce at execution time.
//! Operational pause state is excluded for the same reason: its dedicated
//! control endpoints must remain usable without invalidating governance work.
//! The deterministic `PermissionRecord` generation remains in the semantic
//! payload so stale approvals cannot become valid again after the permission
//! value cycles back to earlier role and expiry bytes.

use solana_pubkey::Pubkey;

use crate::{
    error::ChanceryError,
    modules::{
        control::change_risk::{max_risk, ConfigChangeRiskClass},
        permissions::{
            dangerous_roles::dangerous_role_mask,
            state::permission_record::{role_bits_as_u128, PermissionRecord},
        },
    },
};

pub mod permission_change_mask {
    pub const CREATED:          u64 = 1 << 0;
    pub const ROLES_ADDED:      u64 = 1 << 1;
    pub const ROLES_REMOVED:    u64 = 1 << 2;
    pub const PAUSED:           u64 = 1 << 3;
    pub const UNPAUSED:         u64 = 1 << 4;
    pub const EXPIRY_ADDED:     u64 = 1 << 5;
    pub const EXPIRY_SHORTENED: u64 = 1 << 6;
    pub const EXPIRY_EXTENDED:  u64 = 1 << 7;
    pub const EXPIRY_REMOVED:   u64 = 1 << 8;
}

pub const PERMISSION_VALUE_PAYLOAD_SIZE: usize = 97;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PermissionValue {
    pub subject:               Pubkey,
    pub scope_kind:            u8,
    pub scope_key:             Pubkey,
    pub role_bits:             [u64; 2],
    pub permission_generation: u64,
    pub expiry_unix_timestamp: i64,
}

impl PermissionValue {
    pub fn from_record(record: &PermissionRecord) -> Self {
        Self {
            subject:               record.subject,
            scope_kind:            record.scope_kind,
            scope_key:             record.scope_key,
            role_bits:             record.role_bits,
            permission_generation: record.effective_permission_generation(),
            expiry_unix_timestamp: record.expiry_unix_timestamp,
        }
    }

    pub fn empty(subject: Pubkey, scope_kind: u8, scope_key: Pubkey) -> Self {
        Self {
            subject,
            scope_kind,
            scope_key,
            role_bits: [0u64; 2],
            permission_generation: 0,
            expiry_unix_timestamp: 0,
        }
    }

    /// Canonical fixed-width payload used inside `compute_config_change_hash`.
    /// Integer fields are little-endian to match the program's Borsh/state
    /// representation and the TypeScript ceremony helper.
    pub fn to_payload(self) -> [u8; PERMISSION_VALUE_PAYLOAD_SIZE] {
        let mut payload = [0u8; PERMISSION_VALUE_PAYLOAD_SIZE];

        payload[0..32].copy_from_slice(self.subject.as_ref());
        payload[32] = self.scope_kind;
        payload[33..65].copy_from_slice(self.scope_key.as_ref());
        payload[65..73].copy_from_slice(&self.role_bits[0].to_le_bytes());
        payload[73..81].copy_from_slice(&self.role_bits[1].to_le_bytes());
        payload[81..89].copy_from_slice(&self.permission_generation.to_le_bytes());
        payload[89..97].copy_from_slice(&self.expiry_unix_timestamp.to_le_bytes());

        payload
    }

}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PermissionChangeClassification {
    pub risk:               ConfigChangeRiskClass,
    pub change_mask:        u64,
    pub added_role_bits:    u128,
    pub removed_role_bits:  u128,
}

fn raise_risk(
    current: &mut Option<ConfigChangeRiskClass>,
    candidate: ConfigChangeRiskClass,
) {
    *current = Some(match *current {
        Some(existing) => max_risk(existing, candidate),
        None => candidate,
    });
}

pub fn classify_permission_change(
    current: Option<&PermissionRecord>,
    proposed: &PermissionValue,
) -> Result<PermissionChangeClassification, ChanceryError> {
    let proposed_roles = role_bits_as_u128(proposed.role_bits);

    let Some(current) = current else {
        let risk = if proposed_roles & dangerous_role_mask() != 0 {
            ConfigChangeRiskClass::Dangerous
        } else {
            ConfigChangeRiskClass::Widening
        };
        let mut change_mask = permission_change_mask::CREATED;
        if proposed_roles != 0 {
            change_mask |= permission_change_mask::ROLES_ADDED;
        }
        if proposed.expiry_unix_timestamp != 0 {
            change_mask |= permission_change_mask::EXPIRY_ADDED;
        }

        return Ok(PermissionChangeClassification {
            risk,
            change_mask,
            added_role_bits: proposed_roles,
            removed_role_bits: 0,
        });
    };

    if current.subject != proposed.subject
        || current.scope_kind != proposed.scope_kind
        || current.scope_key != proposed.scope_key
    {
        return Err(ChanceryError::PermissionScopeMismatch);
    }

    let current_roles = role_bits_as_u128(current.role_bits);
    let added_role_bits = proposed_roles & !current_roles;
    let removed_role_bits = current_roles & !proposed_roles;
    let proposed_has_dangerous_roles = proposed_roles & dangerous_role_mask() != 0;
    let mut risk: Option<ConfigChangeRiskClass> = None;
    let mut change_mask = 0u64;

    if added_role_bits != 0 {
        change_mask |= permission_change_mask::ROLES_ADDED;
        raise_risk(
            &mut risk,
            if proposed_has_dangerous_roles {
                ConfigChangeRiskClass::Dangerous
            } else {
                ConfigChangeRiskClass::Widening
            },
        );
    }

    if removed_role_bits != 0 {
        change_mask |= permission_change_mask::ROLES_REMOVED;
        raise_risk(&mut risk, ConfigChangeRiskClass::RestrictiveImmediate);
    }

    let old_expiry = current.expiry_unix_timestamp;
    let new_expiry = proposed.expiry_unix_timestamp;
    let old_has_expiry = old_expiry != 0;
    let new_has_expiry = new_expiry != 0;

    match (old_has_expiry, new_has_expiry) {
        (false, true) => {
            change_mask |= permission_change_mask::EXPIRY_ADDED;
            raise_risk(&mut risk, ConfigChangeRiskClass::RestrictiveImmediate);
        }
        (true, false) => {
            change_mask |= permission_change_mask::EXPIRY_REMOVED;
            raise_risk(
                &mut risk,
                if proposed_has_dangerous_roles {
                    ConfigChangeRiskClass::Dangerous
                } else {
                    ConfigChangeRiskClass::HighImpact
                },
            );
        }
        (true, true) if new_expiry > old_expiry => {
            change_mask |= permission_change_mask::EXPIRY_EXTENDED;
            raise_risk(
                &mut risk,
                if proposed_has_dangerous_roles {
                    ConfigChangeRiskClass::Dangerous
                } else {
                    ConfigChangeRiskClass::Widening
                },
            );
        }
        (true, true) if new_expiry < old_expiry => {
            change_mask |= permission_change_mask::EXPIRY_SHORTENED;
            raise_risk(&mut risk, ConfigChangeRiskClass::RestrictiveImmediate);
        }
        _ => {}
    }

    Ok(PermissionChangeClassification {
        risk: risk.unwrap_or(ConfigChangeRiskClass::RoutineOps),
        change_mask,
        added_role_bits,
        removed_role_bits,
    })
}

#[cfg(test)]
mod tests {
    use bytemuck::Zeroable;

    use super::*;
    use crate::{
        constants::{role, scope},
        modules::permissions::state::permission_record::u128_to_role_bits,
    };

    fn record(roles: u128, generation: u64, expiry: i64) -> PermissionRecord {
        let mut record = PermissionRecord::zeroed();
        record.subject = Pubkey::new_from_array([1u8; 32]);
        record.scope_kind = scope::ASSET;
        record.scope_key = Pubkey::new_from_array([2u8; 32]);
        record.role_bits = u128_to_role_bits(roles);
        record.role_schema_version = role::PERMISSION_ROLE_SCHEMA_VERSION;
        record.permission_generation = generation;
        record.expiry_unix_timestamp = expiry;
        record
    }

    fn proposed(roles: u128, generation: u64, expiry: i64) -> PermissionValue {
        PermissionValue {
            subject: Pubkey::new_from_array([1u8; 32]),
            scope_kind: scope::ASSET,
            scope_key: Pubkey::new_from_array([2u8; 32]),
            role_bits: u128_to_role_bits(roles),
            permission_generation: generation,
            expiry_unix_timestamp: expiry,
        }
    }

    #[test]
    fn revoked_tombstone_is_generation_distinct_from_never_created_record() {
        let subject = Pubkey::new_from_array([1u8; 32]);
        let scope_key = Pubkey::new_from_array([2u8; 32]);
        let empty = PermissionValue::empty(subject, scope::ASSET, scope_key);
        let tombstone = PermissionValue {
            permission_generation: 1,
            ..empty
        };

        assert_ne!(empty.to_payload(), tombstone.to_payload());
    }

    #[test]
    fn legacy_paused_tombstone_gets_a_nonzero_effective_generation() {
        let mut tombstone = record(0, 0, 0);
        tombstone.permission_flags =
            crate::modules::permissions::state::permission_record::PERMISSION_FLAG_PAUSED;

        let value = PermissionValue::from_record(&tombstone);

        assert_eq!(value.permission_generation, 1);
    }

    #[test]
    fn new_ordinary_grant_is_widening() {
        let classification =
            classify_permission_change(None, &proposed(role::CAN_MINT_DIRECT, 0, 0)).unwrap();

        assert_eq!(classification.risk, ConfigChangeRiskClass::Widening);
        assert_ne!(classification.change_mask & permission_change_mask::CREATED, 0);
        assert_eq!(
            classification.change_mask & permission_change_mask::EXPIRY_ADDED,
            0,
        );
    }

    #[test]
    fn new_grant_with_expiry_records_expiry_added() {
        let classification = classify_permission_change(
            None,
            &proposed(role::CAN_MINT_DIRECT, 0, i64::MAX),
        )
        .unwrap();

        assert_ne!(
            classification.change_mask & permission_change_mask::EXPIRY_ADDED,
            0,
        );
    }

    #[test]
    fn new_dangerous_grant_is_dangerous() {
        let classification = classify_permission_change(
            None,
            &proposed(role::CAN_WITHDRAW_RESERVE, 0, i64::MAX),
        )
        .unwrap();

        assert_eq!(classification.risk, ConfigChangeRiskClass::Dangerous);
    }

    #[test]
    fn removing_roles_is_restrictive_not_routine() {
        let current = record(role::CAN_MINT_DIRECT | role::CAN_REDEEM_DIRECT, 0, 0);
        let classification = classify_permission_change(
            Some(&current),
            &proposed(role::CAN_MINT_DIRECT, 0, 0),
        )
        .unwrap();

        assert_eq!(classification.risk, ConfigChangeRiskClass::RestrictiveImmediate);
        assert_eq!(classification.removed_role_bits, role::CAN_REDEEM_DIRECT);
    }

    #[test]
    fn adding_dangerous_role_dominates_removal() {
        let current = record(role::CAN_MINT_DIRECT, 0, i64::MAX);
        let classification = classify_permission_change(
            Some(&current),
            &proposed(role::CAN_WITHDRAW_RESERVE, 0, i64::MAX),
        )
        .unwrap();

        assert_eq!(classification.risk, ConfigChangeRiskClass::Dangerous);
    }

    #[test]
    fn operational_pause_is_excluded_from_permission_identity_and_risk() {
        let unpaused = record(role::CAN_MINT_DIRECT, 7, 0);
        let mut paused = unpaused;
        paused.permission_flags =
            crate::modules::permissions::state::permission_record::PERMISSION_FLAG_PAUSED;
        let proposed = proposed(
            role::CAN_MINT_DIRECT | role::CAN_REDEEM_DIRECT,
            7,
            0,
        );

        assert_eq!(
            PermissionValue::from_record(&unpaused).to_payload(),
            PermissionValue::from_record(&paused).to_payload(),
        );
        assert_eq!(
            classify_permission_change(Some(&unpaused), &proposed).unwrap(),
            classify_permission_change(Some(&paused), &proposed).unwrap(),
        );
    }

    #[test]
    fn expiry_shortening_is_restrictive() {
        let current = record(role::CAN_MINT_DIRECT, 0, 2_000);
        let classification = classify_permission_change(
            Some(&current),
            &proposed(role::CAN_MINT_DIRECT, 0, 1_000),
        )
        .unwrap();

        assert_eq!(classification.risk, ConfigChangeRiskClass::RestrictiveImmediate);
    }

    #[test]
    fn expiry_extension_is_widening() {
        let current = record(role::CAN_MINT_DIRECT, 0, 1_000);
        let classification = classify_permission_change(
            Some(&current),
            &proposed(role::CAN_MINT_DIRECT, 0, 2_000),
        )
        .unwrap();

        assert_eq!(classification.risk, ConfigChangeRiskClass::Widening);
    }

    #[test]
    fn extending_existing_dangerous_roles_is_dangerous() {
        let current = record(role::CAN_WITHDRAW_RESERVE, 0, 1_000);
        let classification = classify_permission_change(
            Some(&current),
            &proposed(role::CAN_WITHDRAW_RESERVE, 0, 2_000),
        )
        .unwrap();

        assert_eq!(classification.risk, ConfigChangeRiskClass::Dangerous);
    }

    #[test]
    fn adding_an_ordinary_role_to_dangerous_capability_is_dangerous() {
        let current = record(role::CAN_WITHDRAW_RESERVE, 0, 2_000);
        let classification = classify_permission_change(
            Some(&current),
            &proposed(
                role::CAN_WITHDRAW_RESERVE | role::CAN_MINT_DIRECT,
                0,
                2_000,
            ),
        )
        .unwrap();

        assert_eq!(classification.risk, ConfigChangeRiskClass::Dangerous);
    }

    #[test]
    fn expiry_removal_is_high_impact() {
        let current = record(role::CAN_MINT_DIRECT, 0, 1_000);
        let classification = classify_permission_change(
            Some(&current),
            &proposed(role::CAN_MINT_DIRECT, 0, 0),
        )
        .unwrap();

        assert_eq!(classification.risk, ConfigChangeRiskClass::HighImpact);
    }

    #[test]
    fn no_semantic_change_is_routine() {
        let current = record(role::CAN_MINT_DIRECT, 0, 1_000);
        let classification = classify_permission_change(
            Some(&current),
            &proposed(role::CAN_MINT_DIRECT, 0, 1_000),
        )
        .unwrap();

        assert_eq!(classification.risk, ConfigChangeRiskClass::RoutineOps);
        assert_eq!(classification.change_mask, 0);
    }

    #[test]
    fn payload_is_stable_and_fixed_width() {
        let value = proposed(role::CAN_MINT_DIRECT, 42, 1_000);
        let payload = value.to_payload();

        assert_eq!(payload.len(), PERMISSION_VALUE_PAYLOAD_SIZE);
        assert_eq!(&payload[0..32], value.subject.as_ref());
        assert_eq!(payload[32], value.scope_kind);
        assert_eq!(&payload[33..65], value.scope_key.as_ref());
        assert_eq!(&payload[81..89], &value.permission_generation.to_le_bytes());
    }

    #[test]
    fn setting_past_expiry_is_restrictive_not_expiry_removal() {
        let current = record(role::CAN_MINT_DIRECT, 0, i64::MAX);
        let classification = classify_permission_change(
            Some(&current),
            &proposed(role::CAN_MINT_DIRECT, 0, -1),
        )
        .unwrap();

        assert_eq!(classification.risk, ConfigChangeRiskClass::RestrictiveImmediate);
        assert_ne!(
            classification.change_mask & permission_change_mask::EXPIRY_SHORTENED,
            0,
        );
        assert_eq!(
            classification.change_mask & permission_change_mask::EXPIRY_REMOVED,
            0,
        );
    }
}
