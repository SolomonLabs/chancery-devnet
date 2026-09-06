//! Permission role classification and scope-specificity ranking.
//!
//! Dangerous roles are protocol-control or value-movement capabilities. Any
//! capability-widening transition whose resulting role set contains a
//! dangerous bit is classified as `Dangerous` by the permission change
//! classifier and therefore requires governance plus an accepted pending
//! change. The scope/expiry constraints in this module remain state invariants
//! for every path.

use crate::{
    constants::{role, scope, scope_extensions},
    error::ChanceryError,
};

/// Bits considered dangerous for permission-state invariants. Grants that
/// contain any of these bits require a non-global scope and a finite expiry.
/// The permission transition classifier also assigns the `Dangerous`
/// governance risk class to every widening or reactivation whose resulting
/// role set retains one of these bits.
pub fn dangerous_role_mask() -> u128 {
    role::DANGEROUS_PERMISSION_ROLE_MASK
}

/// Roles a non-admin grantor may redistribute.
pub fn non_admin_grantable_role_mask() -> u128 {
    role::NON_ADMIN_GRANTABLE_ROLE_MASK
}

/// Role bits reserved for code paths that are not implemented in this binary.
pub fn inactive_reserved_role_mask() -> u128 {
    role::INACTIVE_RESERVED_ROLE_MASK
}

/// Authority-adjacent bits - power to redistribute permissions, control
/// pauses, propose authority transfers, drain the reserve. A non-admin
/// grantor must not redistribute any of these regardless of scope; even
/// ops/governance must use narrow scope + expiry (inherited from the
/// dangerous-role checks above).
pub fn authority_adjacent_mask() -> u128 {
    role::AUTHORITY_ADJACENT_ROLE_MASK
}

/// Scope specificity rank from broadest (low) to narrowest (high).
///
/// The rank is a taxonomy and unknown-scope guard, not a substitute for
/// resource containment. Distinct scope kinds intentionally share ranks.
/// Non-admin grants are therefore additionally bound to the grantor's exact
/// `(scope_kind, scope_key)` unless the grantor is GLOBAL.
pub fn scope_specificity_rank(scope_kind: u8) -> u8 {
    match scope_kind {
        scope::GLOBAL                            => 0,
        scope_extensions::MODULE                 => 1,
        scope::ASSET                             => 2,
        scope::ENFORCEMENT                       => 3,
        scope::MIGRATION                         => 3,
        scope_extensions::CROSS_CHAIN_SIGNER_SET => 3,
        scope::REMOTE_DOMAIN                     => 3,
        scope_extensions::ISSUED_TOKEN_CONTROL   => 3,
        scope::PATHWAY                           => 4,
        scope::DESTINATION                       => 5,
        scope::EXECUTOR                          => 5,
        scope::COUNTERPARTY                      => 5,
        scope_extensions::TOKEN_ACCOUNT          => 6,
        _ => 0xFF,
    }
}

/// Enforce resource containment for a non-admin permission mutation.
///
/// A GLOBAL grantor may act within any known narrower scope. Every non-GLOBAL
/// grantor is confined to the exact `(scope_kind, scope_key)` it holds. Scope
/// ranks remain useful for rejecting unknown kinds and attempted broadening,
/// but they cannot distinguish unrelated kinds or keys at the same rank.
pub fn assert_permission_scope_contained(
    target_scope_kind:  u8,
    target_scope_key:   &solana_pubkey::Pubkey,
    grantor_scope_kind: u8,
    grantor_scope_key:  &solana_pubkey::Pubkey,
    is_admin_grantor:   bool,
) -> Result<(), ChanceryError> {
    if is_admin_grantor {
        return Ok(());
    }

    let target_rank = scope_specificity_rank(target_scope_kind);
    let grantor_rank = scope_specificity_rank(grantor_scope_kind);

    if target_rank == 0xFF || grantor_rank == 0xFF {
        return Err(ChanceryError::PermissionUnknownScope);
    }

    if target_rank < grantor_rank {
        return Err(ChanceryError::PermissionScopeBroadeningFailed);
    }

    if grantor_scope_kind != scope::GLOBAL
        && (target_scope_kind != grantor_scope_kind || target_scope_key != grantor_scope_key)
    {
        return Err(ChanceryError::PermissionScopeBroadeningFailed);
    }

    Ok(())
}

/// Validate a permission grant against dangerous-role and scope-rank
/// constraints.
///
/// `is_admin_grantor`: true when the grantor is governance or ops (in which
/// case scope-rank constraint is bypassed; admin can grant any scope).
pub fn assert_permission_grant_constraints(
    granted_role_bits:   u128,
    granted_scope_kind:  u8,
    granted_scope_key:   &solana_pubkey::Pubkey,
    granted_expiry_unix: i64,
    grantor_scope_kind:  u8,
    grantor_scope_key:   &solana_pubkey::Pubkey,
    is_admin_grantor:    bool,
) -> Result<(), ChanceryError> {
    if (granted_role_bits & inactive_reserved_role_mask()) != 0 {
        return Err(ChanceryError::ModuleNotEnabled);
    }

    let dangerous         : u128 = dangerous_role_mask();
    let touches_dangerous : bool = (granted_role_bits & dangerous) != 0;

    // Dangerous bit constraints (apply to all grantors, including admins).
    if touches_dangerous {
        // Narrow-scope constraint: GLOBAL is forbidden for any dangerous grant.
        if granted_scope_kind == scope::GLOBAL {
            return Err(ChanceryError::DangerousPermissionRequiresNarrowScope);
        }
        // Expiry constraint: dangerous grants must have a future expiry.
        if granted_expiry_unix <= 0 {
            return Err(ChanceryError::DangerousPermissionRequiresExpiry);
        }
    }

    // Authority-adjacent bits cannot be redistributed by non-admin grantors
    // regardless of scope. Prevents fan-out trees of CAN_GRANT_PERMISSION
    // and stops a narrow grantor from re-issuing protocol-level powers.
    if !is_admin_grantor && (granted_role_bits & authority_adjacent_mask()) != 0 {
        return Err(ChanceryError::RestrictedRoleNotGrantableByNonAdmin);
    }

    if !is_admin_grantor && (granted_role_bits & !non_admin_grantable_role_mask()) != 0 {
        return Err(ChanceryError::RestrictedRoleNotGrantableByNonAdmin);
    }

    assert_permission_scope_contained(
        granted_scope_kind,
        granted_scope_key,
        grantor_scope_kind,
        grantor_scope_key,
        is_admin_grantor,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_is_broadest() {
        assert_eq!(scope_specificity_rank(scope::GLOBAL), 0);
    }

    #[test]
    fn token_account_is_narrowest() {
        assert_eq!(scope_specificity_rank(scope_extensions::TOKEN_ACCOUNT), 6);
    }

    #[test]
    fn unknown_scope_returns_max() {
        assert_eq!(scope_specificity_rank(0xFE), 0xFF);
    }

    fn key(byte: u8) -> solana_pubkey::Pubkey {
        solana_pubkey::Pubkey::new_from_array([byte; 32])
    }

    #[test]
    fn admin_can_grant_any_scope() {
        let r = assert_permission_grant_constraints(
            0,              // no dangerous bits
            scope::GLOBAL,  // broadest
            &key(1),
            0,              // no expiry
            scope::PATHWAY, // grantor at narrow scope
            &key(2),
            true,           // admin
        );

        assert!(r.is_ok());
    }

    #[test]
    fn nonadmin_cannot_broaden_scope() {
        let r = assert_permission_grant_constraints(
            0,
            scope::GLOBAL,  // broadening
            &key(1),
            0,
            scope::PATHWAY,
            &key(1),
            false,
        );

        assert_eq!(r, Err(ChanceryError::PermissionScopeBroadeningFailed));
    }

    #[test]
    fn nonadmin_cannot_cross_scope_kind_even_if_rank_is_narrower() {
        let r = assert_permission_grant_constraints(
            0,
            scope::DESTINATION, // rank 5
            &key(1),
            0,
            scope::PATHWAY,     // rank 4 grantor
            &key(1),
            false,
        );

        assert_eq!(r, Err(ChanceryError::PermissionScopeBroadeningFailed));
    }

    #[test]
    fn nonadmin_cannot_cross_scope_key() {
        let r = assert_permission_grant_constraints(
            0,
            scope::PATHWAY,
            &key(2),
            0,
            scope::PATHWAY,
            &key(1),
            false,
        );

        assert_eq!(r, Err(ChanceryError::PermissionScopeBroadeningFailed));
    }

    #[test]
    fn nonadmin_can_grant_the_exact_same_scope() {
        let scope_key = key(1);
        let r = assert_permission_grant_constraints(
            0,
            scope::PATHWAY,
            &scope_key,
            0,
            scope::PATHWAY,
            &scope_key,
            false,
        );

        assert!(r.is_ok());
    }

    #[test]
    fn global_nonadmin_can_grant_a_known_narrower_scope() {
        let r = assert_permission_grant_constraints(
            0,
            scope::PATHWAY,
            &key(2),
            0,
            scope::GLOBAL,
            &solana_pubkey::Pubkey::default(),
            false,
        );

        assert!(r.is_ok());
    }

    #[test]
    fn pure_restriction_scope_check_rejects_another_resource_key() {
        assert_eq!(
            assert_permission_scope_contained(
                scope::PATHWAY,
                &key(2),
                scope::PATHWAY,
                &key(1),
                false,
            ),
            Err(ChanceryError::PermissionScopeBroadeningFailed),
        );
    }

    #[test]
    fn nonadmin_cannot_grant_role_outside_allowlist() {
        let scope_key = key(1);
        let r = assert_permission_grant_constraints(
            role::CAN_FREEZE_TOKEN_ACCOUNT,
            scope_extensions::TOKEN_ACCOUNT,
            &scope_key,
            (i64::MAX) - 1,
            scope_extensions::TOKEN_ACCOUNT,
            &scope_key,
            false,
        );

        assert_eq!(r, Err(ChanceryError::RestrictedRoleNotGrantableByNonAdmin));
    }

    #[test]
    fn dangerous_grant_at_global_is_rejected_even_for_admin() {
        let r = assert_permission_grant_constraints(
            role::CAN_FREEZE_TOKEN_ACCOUNT,
            scope::GLOBAL,
            &key(1),
            (i64::MAX) - 1,
            scope::GLOBAL,
            &solana_pubkey::Pubkey::default(),
            true,  // admin, but still rejected
        );

        assert_eq!(r, Err(ChanceryError::DangerousPermissionRequiresNarrowScope));
    }

    #[test]
    fn dangerous_grant_without_expiry_is_rejected() {
        let r = assert_permission_grant_constraints(
            role::CAN_FREEZE_TOKEN_ACCOUNT,
            scope_extensions::TOKEN_ACCOUNT,
            &key(1),
            0,  // no expiry
            scope::GLOBAL,
            &solana_pubkey::Pubkey::default(),
            true,
        );

        assert_eq!(r, Err(ChanceryError::DangerousPermissionRequiresExpiry));
    }

    #[test]
    fn dangerous_grant_with_narrow_scope_and_expiry_admin_passes() {
        let r = assert_permission_grant_constraints(
            role::CAN_FREEZE_TOKEN_ACCOUNT,
            scope_extensions::TOKEN_ACCOUNT,
            &key(1),
            (i64::MAX) - 1,
            scope::GLOBAL,
            &solana_pubkey::Pubkey::default(),
            true,
        );

        assert!(r.is_ok());
    }

    #[test]
    fn remote_domain_pause_relaxation_requires_finite_expiry() {
        let r = assert_permission_grant_constraints(
            role::CAN_RELAX_REMOTE_DOMAIN_PAUSE,
            scope::REMOTE_DOMAIN,
            &key(1),
            0,
            scope::GLOBAL,
            &solana_pubkey::Pubkey::default(),
            true,
        );

        assert_eq!(r, Err(ChanceryError::DangerousPermissionRequiresExpiry));
    }

    #[test]
    fn nonadmin_cannot_redistribute_cross_chain_roles() {
        let scope_key = key(1);
        let result = assert_permission_grant_constraints(
            role::CAN_EMIT_OUTBOUND_MESSAGE,
            scope::PATHWAY,
            &scope_key,
            i64::MAX - 1,
            scope::PATHWAY,
            &scope_key,
            false,
        );

        assert_eq!(
            result,
            Err(ChanceryError::RestrictedRoleNotGrantableByNonAdmin),
        );
    }

    #[test]
    fn nonadmin_cannot_pregrant_stubbed_token_control_roles() {
        let scope_key = key(1);
        let result = assert_permission_grant_constraints(
            role::CAN_UPDATE_TOKEN_METADATA,
            scope_extensions::ISSUED_TOKEN_CONTROL,
            &scope_key,
            i64::MAX - 1,
            scope_extensions::ISSUED_TOKEN_CONTROL,
            &scope_key,
            false,
        );

        assert_eq!(result, Err(ChanceryError::ModuleNotEnabled));
    }

    #[test]
    fn admin_cannot_pregrant_inactive_reserved_role() {
        let result = assert_permission_grant_constraints(
            role::CAN_SET_TRANSFER_HOOK_PROGRAM,
            scope_extensions::ISSUED_TOKEN_CONTROL,
            &key(1),
            i64::MAX - 1,
            scope::GLOBAL,
            &solana_pubkey::Pubkey::default(),
            true,
        );

        assert_eq!(result, Err(ChanceryError::ModuleNotEnabled));
    }

    #[test]
    fn admin_cannot_pregrant_unconsumed_administrative_permission_roles() {
        let roles = [
            role::CAN_CONFIGURE_ASSET,
            role::CAN_SET_GLOBAL_PAUSE,
            role::CAN_SET_ASSET_PAUSE,
            role::CAN_PROPOSE_AUTHORITY_TRANSFER,
            role::CAN_ACCEPT_AUTHORITY_TRANSFER,
            role::CAN_SET_PATHWAY_POLICY,
            role::CAN_SET_LIMIT_POLICY,
            role::CAN_SET_EVIDENCE_POLICY,
            role::CAN_SET_FEE_POLICY,
            role::CAN_SET_MODULE_STATUS,
        ];

        for role_bit in roles {
            let result = assert_permission_grant_constraints(
                role_bit,
                scope::ASSET,
                &key(1),
                i64::MAX - 1,
                scope::GLOBAL,
                &solana_pubkey::Pubkey::default(),
                true,
            );

            assert_eq!(result, Err(ChanceryError::ModuleNotEnabled));
        }
    }
}
