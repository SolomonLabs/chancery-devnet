//! Shared permission-mutation authorization and state-write helpers.

use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::{
    account_security::{assert_external_identity, assert_external_signer},
    constants::{role, scope},
    error::ChanceryError,
    modules::{
        control::change_risk::ConfigChangeRiskClass,
        core::state::chancery_config::ChanceryConfig,
        permissions::{
            auth::load_canonical_permission_record,
            change_detection::{
                permission_change_mask, PermissionChangeClassification, PermissionValue,
            },
            dangerous_roles::{
                assert_permission_grant_constraints, assert_permission_scope_contained,
                scope_specificity_rank,
            },
            state::permission_record::{
                role_bits_as_u128, PermissionRecord, PERMISSION_RECORD_DISCRIMINATOR,
            },
        },
    },
};

pub fn assert_permission_value_invariants(
    proposed: &PermissionValue,
    current_exists: bool,
    risk: ConfigChangeRiskClass,
    now_unix_timestamp: i64,
) -> ProgramResult {
    if scope_specificity_rank(proposed.scope_kind) == 0xFF {
        return Err(ChanceryError::PermissionUnknownScope.into());
    }

    assert_external_identity(&proposed.subject, &crate::id())?;
    let proposed_roles = role_bits_as_u128(proposed.role_bits);
    if proposed_roles & !role::ACTIVE_ROLE_MASK != 0 {
        return Err(ChanceryError::PermissionUnknownRoleBits.into());
    }

    if !current_exists && proposed_roles == 0 {
        return Err(ChanceryError::PermissionEmptyGrant.into());
    }

    assert_permission_grant_constraints(
        proposed_roles,
        proposed.scope_kind,
        &proposed.scope_key,
        proposed.expiry_unix_timestamp,
        scope::GLOBAL,
        &Pubkey::default(),
        true,
    )?;

    if risk.requires_timelock()
        && proposed.expiry_unix_timestamp != 0
        && proposed.expiry_unix_timestamp <= now_unix_timestamp
    {
        return Err(ChanceryError::PermissionExpired.into());
    }

    Ok(())
}

pub fn required_permission_mutation_roles(
    classification: PermissionChangeClassification,
) -> u128 {
    // Creation is always a grant operation. EXPIRY_ADDED and PAUSED describe
    // containment on the newly-created value; they must not make a non-admin
    // grantor also require CAN_REVOKE_PERMISSION before the record exists.
    if classification.change_mask & permission_change_mask::CREATED != 0 {
        return role::CAN_GRANT_PERMISSION;
    }

    let widening_mask = permission_change_mask::CREATED
        | permission_change_mask::ROLES_ADDED
        | permission_change_mask::UNPAUSED
        | permission_change_mask::EXPIRY_EXTENDED
        | permission_change_mask::EXPIRY_REMOVED;
    let restrictive_mask = permission_change_mask::ROLES_REMOVED
        | permission_change_mask::PAUSED
        | permission_change_mask::EXPIRY_ADDED
        | permission_change_mask::EXPIRY_SHORTENED;
    let mut required_roles = 0u128;

    if classification.change_mask == 0
        || classification.change_mask & widening_mask != 0
    {
        required_roles |= role::CAN_GRANT_PERMISSION;
    }
    if classification.change_mask & restrictive_mask != 0 {
        required_roles |= role::CAN_REVOKE_PERMISSION;
    }

    required_roles
}

/// Return true when the mutation creates or reactivates capability and should
/// therefore establish new grant provenance. Pure containment edits preserve
/// the original `issued_at_unix_timestamp` and `granted_by` values.
pub fn permission_change_rewrites_grant_provenance(
    classification: PermissionChangeClassification,
) -> bool {
    let provenance_rewrite_mask = permission_change_mask::CREATED
        | permission_change_mask::ROLES_ADDED
        | permission_change_mask::UNPAUSED
        | permission_change_mask::EXPIRY_EXTENDED
        | permission_change_mask::EXPIRY_REMOVED;

    classification.change_mask & provenance_rewrite_mask != 0
}

pub fn assert_permission_mutation_authority<'a>(
    chancery_config: &ChanceryConfig,
    granting_authority_account_info: &AccountInfo<'a>,
    grantor_permission_account_info: Option<&'a AccountInfo<'a>>,
    proposed: &PermissionValue,
    classification: PermissionChangeClassification,
    now_unix_timestamp: i64,
    program_id: &Pubkey,
) -> ProgramResult {
    assert_external_signer(granting_authority_account_info, program_id)?;

    let is_admin = granting_authority_account_info.key == &chancery_config.governance_authority
        || granting_authority_account_info.key == &chancery_config.operations_authority;
    let proposed_roles = role_bits_as_u128(proposed.role_bits);

    if is_admin {
        return Ok(());
    }

    let grantor_permission_account_info = grantor_permission_account_info
        .ok_or(ChanceryError::InsufficientRole)?;
    let grantor = load_canonical_permission_record(
        grantor_permission_account_info,
        program_id,
    )?;

    if &grantor.subject != granting_authority_account_info.key {
        return Err(ChanceryError::InsufficientRole.into());
    }

    let required_roles = required_permission_mutation_roles(classification);
    grantor.assert_valid(
        required_roles,
        grantor.scope_kind,
        &grantor.scope_key,
        now_unix_timestamp,
    )?;

    if required_roles & role::CAN_GRANT_PERMISSION != 0 {
        assert_permission_grant_constraints(
            proposed_roles,
            proposed.scope_kind,
            &proposed.scope_key,
            proposed.expiry_unix_timestamp,
            grantor.scope_kind,
            &grantor.scope_key,
            false,
        )?;

        let grantor_roles = role_bits_as_u128(grantor.role_bits);
        if proposed_roles & !grantor_roles != 0 {
            return Err(ChanceryError::InsufficientRole.into());
        }
    } else {
        // Purely restrictive mutations still must target the revoker's exact
        // resource scope. The grant checker is intentionally not used here
        // because it also applies role-redistribution and dangerous-grant
        // invariants that should not block containment.
        assert_permission_scope_contained(
            proposed.scope_kind,
            &proposed.scope_key,
            grantor.scope_kind,
            &grantor.scope_key,
            false,
        )?;
    }

    if required_roles & role::CAN_GRANT_PERMISSION != 0
        && grantor.expiry_unix_timestamp > 0
        && (proposed.expiry_unix_timestamp <= 0
            || proposed.expiry_unix_timestamp > grantor.expiry_unix_timestamp)
    {
        return Err(ChanceryError::PermissionExpiryExtensionExceedsGrantor.into());
    }

    Ok(())
}

pub fn write_permission_record(
    record: &mut PermissionRecord,
    value: PermissionValue,
    bump: u8,
    issued_at_unix_timestamp: i64,
    granted_by: Pubkey,
    rewrite_grant_provenance: bool,
) {
    // Operational pause is owned by set_executor_pause and
    // set_counterparty_pause. Permission upserts must preserve its live value.
    let operational_permission_flags = record.permission_flags;

    record.discriminator            = PERMISSION_RECORD_DISCRIMINATOR;
    record.version                  = 1;
    record.bump                     = bump;
    record.scope_kind               = value.scope_kind;
    record._pad0                    = [0u8; 4];
    record.subject                  = value.subject;
    record.scope_key                = value.scope_key;
    record.role_bits                = value.role_bits;
    record.permission_flags         = operational_permission_flags;
    record.expiry_unix_timestamp    = value.expiry_unix_timestamp;

    if rewrite_grant_provenance {
        record.issued_at_unix_timestamp = issued_at_unix_timestamp;
        record.granted_by               = granted_by;
    }

    record.role_schema_version      = role::PERMISSION_ROLE_SCHEMA_VERSION;
    record._pad1                    = [0u8; 6];
    record.permission_generation    = value.permission_generation;
    record._reserved                = [0u8; 24];
}

#[cfg(test)]
mod tests {
    use bytemuck::Zeroable;

    use super::*;
    use crate::modules::control::change_risk::ConfigChangeRiskClass;

    fn classification(change_mask: u64) -> PermissionChangeClassification {
        PermissionChangeClassification {
            risk: ConfigChangeRiskClass::RoutineOps,
            change_mask,
            added_role_bits: 0,
            removed_role_bits: 0,
        }
    }

    #[test]
    fn creation_requires_grant_role() {
        assert_eq!(
            required_permission_mutation_roles(classification(
                permission_change_mask::CREATED | permission_change_mask::ROLES_ADDED,
            )),
            role::CAN_GRANT_PERMISSION,
        );
    }

    #[test]
    fn creation_with_containment_fields_does_not_require_revoke_role() {
        assert_eq!(
            required_permission_mutation_roles(classification(
                permission_change_mask::CREATED
                    | permission_change_mask::ROLES_ADDED
                    | permission_change_mask::EXPIRY_ADDED
                    | permission_change_mask::PAUSED,
            )),
            role::CAN_GRANT_PERMISSION,
        );
    }

    #[test]
    fn removal_requires_revoke_role() {
        assert_eq!(
            required_permission_mutation_roles(classification(
                permission_change_mask::ROLES_REMOVED,
            )),
            role::CAN_REVOKE_PERMISSION,
        );
    }

    #[test]
    fn mixed_replacement_requires_grant_and_revoke_roles() {
        assert_eq!(
            required_permission_mutation_roles(classification(
                permission_change_mask::ROLES_ADDED
                    | permission_change_mask::ROLES_REMOVED,
            )),
            role::CAN_GRANT_PERMISSION | role::CAN_REVOKE_PERMISSION,
        );
    }

    #[test]
    fn unpause_requires_grant_while_pause_requires_revoke() {
        assert_eq!(
            required_permission_mutation_roles(classification(
                permission_change_mask::UNPAUSED,
            )),
            role::CAN_GRANT_PERMISSION,
        );
        assert_eq!(
            required_permission_mutation_roles(classification(
                permission_change_mask::PAUSED,
            )),
            role::CAN_REVOKE_PERMISSION,
        );
    }

    #[test]
    fn no_op_requires_grant_role() {
        assert_eq!(
            required_permission_mutation_roles(classification(0)),
            role::CAN_GRANT_PERMISSION,
        );
    }

    fn value(roles: u128, generation: u64, expiry: i64) -> PermissionValue {
        PermissionValue {
            subject: Pubkey::new_from_array([1u8; 32]),
            scope_kind: scope::ASSET,
            scope_key: Pubkey::new_from_array([2u8; 32]),
            role_bits: crate::modules::permissions::state::permission_record::u128_to_role_bits(
                roles,
            ),
            permission_generation: generation,
            expiry_unix_timestamp: expiry,
        }
    }

    #[test]
    fn permission_upsert_preserves_operational_pause_and_generation() {
        let mut record = PermissionRecord::zeroed();
        record.permission_flags =
            crate::modules::permissions::state::permission_record::PERMISSION_FLAG_PAUSED;

        write_permission_record(
            &mut record,
            value(role::CAN_MINT_DIRECT, 7, 0),
            3,
            1_000,
            Pubkey::new_unique(),
            true,
        );

        assert_eq!(
            record.permission_flags,
            crate::modules::permissions::state::permission_record::PERMISSION_FLAG_PAUSED,
        );
        assert_eq!(record.permission_generation, 7);
    }

    #[test]
    fn pure_restrictions_preserve_grant_provenance() {
        for change_mask in [
            permission_change_mask::ROLES_REMOVED,
            permission_change_mask::PAUSED,
            permission_change_mask::EXPIRY_ADDED,
            permission_change_mask::EXPIRY_SHORTENED,
        ] {
            assert!(!permission_change_rewrites_grant_provenance(classification(
                change_mask,
            )));
        }
    }

    #[test]
    fn capability_widening_rewrites_grant_provenance() {
        for change_mask in [
            permission_change_mask::CREATED,
            permission_change_mask::ROLES_ADDED,
            permission_change_mask::UNPAUSED,
            permission_change_mask::EXPIRY_EXTENDED,
            permission_change_mask::EXPIRY_REMOVED,
        ] {
            assert!(permission_change_rewrites_grant_provenance(classification(
                change_mask,
            )));
        }
    }

    #[test]
    fn creation_rejects_empty_role_set() {
        let result = assert_permission_value_invariants(
            &value(0, 0, 0),
            false,
            ConfigChangeRiskClass::Widening,
            1_000,
        );

        assert_eq!(
            result,
            Err(ChanceryError::PermissionEmptyGrant.into()),
        );
    }

    #[test]
    fn timelocked_creation_rejects_expired_value() {
        let result = assert_permission_value_invariants(
            &value(role::CAN_MINT_DIRECT, 0, 999),
            false,
            ConfigChangeRiskClass::Widening,
            1_000,
        );

        assert_eq!(
            result,
            Err(ChanceryError::PermissionExpired.into()),
        );
    }

    #[test]
    fn direct_expiry_shortening_may_expire_immediately() {
        let result = assert_permission_value_invariants(
            &value(role::CAN_MINT_DIRECT, 0, -1),
            true,
            ConfigChangeRiskClass::RestrictiveImmediate,
            1_000,
        );

        assert!(result.is_ok());
    }
    #[test]
    fn future_retired_and_inactive_role_bits_fail_closed() {
        let cases = [
            role::RETIRED_ROLE_MASK,
            role::CAN_SET_INSURANCE_POLICY,
            1u128 << 90,
            role::CAN_MINT_DIRECT | (1u128 << 100),
        ];
        let mut index = 0usize;
        while index < cases.len() {
            assert_eq!(
                assert_permission_value_invariants(
                    &value(cases[index], 0, 0),
                    false,
                    ConfigChangeRiskClass::Widening,
                    1_000,
                ),
                Err(ChanceryError::PermissionUnknownRoleBits.into()),
            );
            index += 1;
        }
    }

    #[test]
    fn future_upgrade_cannot_activate_a_pregranted_bit() {
        let future_bit = 1u128 << 80;
        assert_eq!(future_bit & role::GRANTABLE_ROLE_MASK, 0);
        assert_eq!(
            assert_permission_value_invariants(
                &value(role::CAN_MINT_DIRECT | future_bit, 0, 0),
                false,
                ConfigChangeRiskClass::Widening,
                1_000,
            ),
            Err(ChanceryError::PermissionUnknownRoleBits.into()),
        );
    }

    #[test]
    fn every_protocol_signer_pda_is_rejected_as_permission_subject() {
        let program_id = crate::id();
        let seeds = [
            crate::constants::seeds::MINT_AUTHORITY,
            crate::constants::seeds::FREEZE_AUTHORITY,
            crate::constants::seeds::RESERVE_AUTHORITY,
            crate::constants::seeds::CLOSE_MINT_AUTHORITY,
            crate::constants::seeds::TRANSFER_HOOK_AUTHORITY,
            crate::constants::seeds::PERMANENT_DELEGATE_AUTHORITY,
            crate::constants::seeds::METADATA_POINTER_AUTHORITY,
            crate::constants::seeds::METADATA_UPDATE_AUTHORITY,
            crate::constants::seeds::PAUSE_AUTHORITY,
            crate::constants::seeds::CONFIDENTIAL_TRANSFER_AUTHORITY,
            crate::constants::seeds::DEFAULT_ACCOUNT_STATE_AUTHORITY,
            crate::constants::seeds::EVENT_AUTHORITY,
        ];
        let mut index = 0usize;
        while index < seeds.len() {
            let (subject, _) = Pubkey::find_program_address(&[seeds[index]], &program_id);
            let mut proposed = value(role::CAN_MINT_DIRECT, 0, 0);
            proposed.subject = subject;
            assert_eq!(
                assert_permission_value_invariants(
                    &proposed,
                    false,
                    ConfigChangeRiskClass::Widening,
                    1_000,
                ),
                Err(ChanceryError::ProtocolSignerIdentityForbidden.into()),
            );
            index += 1;
        }
    }

}
