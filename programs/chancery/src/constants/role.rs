//! Role bits - u128 bitfield used in `Permission.role_bits` and grant checks.
//!
//! Append-only. Retired bit positions must never be reused.

pub mod role {
    /// Persisted permission records bind their role bits to this exact schema.
    /// A future binary that changes the meaning or availability of role bits
    /// must perform an explicit state transition rather than silently giving
    /// new meaning to previously persisted data.
    pub const PERMISSION_ROLE_SCHEMA_VERSION: u16 = 1;

    pub const CAN_MINT_DIRECT:                     u128 = 1 << 0;
    pub const CAN_REDEEM_DIRECT:                   u128 = 1 << 1;
    pub const CAN_MINT_DELEGATED:                  u128 = 1 << 2;
    pub const CAN_REDEEM_DELEGATED:                u128 = 1 << 3;
    pub const CAN_EXECUTE_SETTLEMENT:              u128 = 1 << 4;
    pub const CAN_USE_TRILATERAL_PATHWAY:          u128 = 1 << 5;
    pub const CAN_WITHDRAW_RESERVE:                u128 = 1 << 6;
    pub const CAN_CONFIGURE_ASSET:                 u128 = 1 << 7;
    pub const CAN_GRANT_PERMISSION:                u128 = 1 << 8;
    pub const CAN_REVOKE_PERMISSION:               u128 = 1 << 9;
    pub const CAN_SET_GLOBAL_PAUSE:                u128 = 1 << 10;
    pub const CAN_SET_ASSET_PAUSE:                 u128 = 1 << 11;
    pub const CAN_PROPOSE_AUTHORITY_TRANSFER:      u128 = 1 << 12;
    pub const CAN_ACCEPT_AUTHORITY_TRANSFER:       u128 = 1 << 13;
    pub const CAN_EXECUTE_LEGACY_MIGRATION:        u128 = 1 << 14;
    pub const CAN_OPEN_PROVENANCE_CASE:            u128 = 1 << 15;
    pub const CAN_APPROVE_PROVENANCE_CASE:         u128 = 1 << 16;
    pub const CAN_PROMOTE_COMPARTMENT_BALANCE:     u128 = 1 << 17;
    pub const CAN_FREEZE_TOKEN_ACCOUNT:            u128 = 1 << 18;
    pub const CAN_THAW_TOKEN_ACCOUNT:              u128 = 1 << 19;
    pub const CAN_EXECUTE_FORCED_BURN:             u128 = 1 << 20;
    pub const CAN_SET_PATHWAY_POLICY:              u128 = 1 << 21;
    pub const CAN_SET_LIMIT_POLICY:                u128 = 1 << 22;
    pub const CAN_SET_EVIDENCE_POLICY:             u128 = 1 << 23;
    pub const CAN_SET_FEE_POLICY:                  u128 = 1 << 24;
    pub const CAN_SET_INSURANCE_POLICY:            u128 = 1 << 25;

    // ── Token-2022 extension management ──
    pub const CAN_SET_TRANSFER_HOOK_PROGRAM:       u128 = 1 << 26;
    pub const CAN_SET_PERMANENT_DELEGATE:          u128 = 1 << 27;
    pub const CAN_INITIALIZE_TOKEN_METADATA:       u128 = 1 << 28;
    pub const CAN_UPDATE_TOKEN_METADATA:           u128 = 1 << 29;
    pub const CAN_SET_DEFAULT_ACCOUNT_STATE:       u128 = 1 << 30;
    pub const CAN_SET_TOKEN_PAUSE:                 u128 = 1 << 31;
    pub const CAN_CONFIGURE_CONFIDENTIAL_TRANSFER: u128 = 1 << 32;
    pub const CAN_ACTIVATE_ISSUED_TOKEN_MODULE:    u128 = 1 << 33;
    pub const CAN_DEACTIVATE_ISSUED_TOKEN_MODULE:  u128 = 1 << 34;

    // ── Cross-chain ──
    pub const CAN_REGISTER_REMOTE_DOMAIN:          u128 = 1 << 35;
    pub const CAN_ROTATE_SIGNER_SET:               u128 = 1 << 36;
    pub const CAN_RESTRICT_REMOTE_DOMAIN_PAUSE:    u128 = 1 << 37;
    pub const CAN_RELAX_REMOTE_DOMAIN_PAUSE:       u128 = 1 << 38;
    pub const CAN_CONSUME_INBOUND_MESSAGE:         u128 = 1 << 39;
    pub const CAN_EMIT_OUTBOUND_MESSAGE:           u128 = 1 << 40;

    // ── ──
    pub const CAN_REGISTER_CROSS_CHAIN_SIGNER_SET: u128 = 1 << 41;
    pub const CAN_SET_MODULE_STATUS:               u128 = 1 << 42;
    pub const CAN_UPDATE_ISSUED_TOKEN_CONTROL:     u128 = 1 << 43;
    pub const CAN_UPDATE_REMOTE_DOMAIN_POLICY:     u128 = 1 << 44;

    // Bit 45 retired pre-deployment: CAN_REGISTER_USAGE_WINDOW died with the
    // period-keyed window design. Stable windows are created at binding time
    // (policy registration / widening consume / permission grant) and roll in
    // place, so no keeper capability exists. Retired positions are permanently
    // ungrantable and must never be reused.
    pub const RETIRED_ROLE_MASK: u128 = 1u128 << 45;

    /// Every role position defined by this binary, including permanently
    /// retired positions. Bits outside this mask were never defined.
    pub const DEFINED_ROLE_MASK: u128 = (1u128 << 46) - 1;

    /// Backward-compatible name for the full defined role-position set.
    pub const KNOWN_ROLE_MASK: u128 = DEFINED_ROLE_MASK;

    // Reuse existing bit positions; both names
    // appear in dangerous_roles.rs and downstream tests/specs.
    pub const CAN_FORCE_BURN:                      u128 = CAN_EXECUTE_FORCED_BURN;
    pub const CAN_ROTATE_CROSS_CHAIN_SIGNER_SET:   u128 = CAN_ROTATE_SIGNER_SET;

    /// Permission bits that a non-admin grantor may redistribute after the
    /// grant itself completes the required governance ceremony.
    pub const NON_ADMIN_GRANTABLE_ROLE_MASK: u128 =
        CAN_MINT_DIRECT
        | CAN_REDEEM_DIRECT
        | CAN_MINT_DELEGATED
        | CAN_REDEEM_DELEGATED
        | CAN_EXECUTE_SETTLEMENT
        | CAN_USE_TRILATERAL_PATHWAY;

    /// Reserved role bits whose corresponding runtime actions are not present
    /// in this binary. Permission records may not pre-grant these bits, even to
    /// admins: a future program upgrade must deliberately remove the bit from
    /// this mask before the capability can be granted.
    pub const INACTIVE_RESERVED_ROLE_MASK: u128 =
        CAN_OPEN_PROVENANCE_CASE
        | CAN_APPROVE_PROVENANCE_CASE
        | CAN_PROMOTE_COMPARTMENT_BALANCE
        | CAN_EXECUTE_FORCED_BURN
        | CAN_SET_INSURANCE_POLICY
        | CAN_SET_TRANSFER_HOOK_PROGRAM
        | CAN_SET_PERMANENT_DELEGATE
        | CAN_INITIALIZE_TOKEN_METADATA
        | CAN_UPDATE_TOKEN_METADATA
        | CAN_SET_DEFAULT_ACCOUNT_STATE
        | CAN_SET_TOKEN_PAUSE
        | CAN_CONFIGURE_CONFIDENTIAL_TRANSFER
        | CAN_ACTIVATE_ISSUED_TOKEN_MODULE
        | CAN_DEACTIVATE_ISSUED_TOKEN_MODULE
        | CAN_UPDATE_ISSUED_TOKEN_CONTROL
        // These administrative instructions exist, but authenticate only
        // against the singleton ChanceryConfig authorities in this binary.
        // Their delegated PermissionRecord capabilities remain reserved until
        // handlers explicitly consume and scope them in a reviewed upgrade.
        | CAN_CONFIGURE_ASSET
        | CAN_SET_GLOBAL_PAUSE
        | CAN_SET_ASSET_PAUSE
        | CAN_PROPOSE_AUTHORITY_TRANSFER
        | CAN_ACCEPT_AUTHORITY_TRANSFER
        | CAN_SET_PATHWAY_POLICY
        | CAN_SET_LIMIT_POLICY
        | CAN_SET_EVIDENCE_POLICY
        | CAN_SET_FEE_POLICY
        | CAN_SET_MODULE_STATUS;

    /// Role bits that can be persisted and exercised by this binary. Retired
    /// positions and declared-but-inactive module roles fail closed.
    pub const ACTIVE_ROLE_MASK: u128 =
        DEFINED_ROLE_MASK & !RETIRED_ROLE_MASK & !INACTIVE_RESERVED_ROLE_MASK;

    /// Backward-compatible alias used by mutation code and external checks.
    pub const GRANTABLE_ROLE_MASK: u128 = ACTIVE_ROLE_MASK;

    /// Permission-redistribution and authority-adjacent capabilities that a
    /// non-admin grantor may never redistribute.
    pub const AUTHORITY_ADJACENT_ROLE_MASK: u128 =
        CAN_GRANT_PERMISSION
        | CAN_REVOKE_PERMISSION
        | CAN_SET_GLOBAL_PAUSE
        | CAN_SET_ASSET_PAUSE
        | CAN_PROPOSE_AUTHORITY_TRANSFER
        | CAN_ACCEPT_AUTHORITY_TRANSFER
        | CAN_EXECUTE_LEGACY_MIGRATION
        | CAN_WITHDRAW_RESERVE;

    /// Roles that require a non-global scope, finite expiry, and the
    /// `Dangerous` pending-change tier when newly added.
    pub const DANGEROUS_PERMISSION_ROLE_MASK: u128 =
        CAN_UPDATE_ISSUED_TOKEN_CONTROL
        | CAN_SET_TRANSFER_HOOK_PROGRAM
        | CAN_SET_PERMANENT_DELEGATE
        | CAN_CONFIGURE_CONFIDENTIAL_TRANSFER
        | CAN_FREEZE_TOKEN_ACCOUNT
        | CAN_THAW_TOKEN_ACCOUNT
        | CAN_FORCE_BURN
        | CAN_ROTATE_CROSS_CHAIN_SIGNER_SET
        | CAN_REGISTER_CROSS_CHAIN_SIGNER_SET
        | CAN_RELAX_REMOTE_DOMAIN_PAUSE
        | CAN_SET_MODULE_STATUS
        | CAN_UPDATE_REMOTE_DOMAIN_POLICY
        | AUTHORITY_ADJACENT_ROLE_MASK;
}

#[cfg(test)]
mod tests {
    use super::role;

    const UNCONSUMED_ADMIN_PERMISSION_ROLES: u128 =
        role::CAN_CONFIGURE_ASSET
        | role::CAN_SET_GLOBAL_PAUSE
        | role::CAN_SET_ASSET_PAUSE
        | role::CAN_PROPOSE_AUTHORITY_TRANSFER
        | role::CAN_ACCEPT_AUTHORITY_TRANSFER
        | role::CAN_SET_PATHWAY_POLICY
        | role::CAN_SET_LIMIT_POLICY
        | role::CAN_SET_EVIDENCE_POLICY
        | role::CAN_SET_FEE_POLICY
        | role::CAN_SET_MODULE_STATUS;

    #[test]
    fn unconsumed_administrative_permission_roles_are_inactive_and_not_redistributable() {
        assert_eq!(
            role::INACTIVE_RESERVED_ROLE_MASK & UNCONSUMED_ADMIN_PERMISSION_ROLES,
            UNCONSUMED_ADMIN_PERMISSION_ROLES,
        );
        assert_eq!(
            role::ACTIVE_ROLE_MASK & UNCONSUMED_ADMIN_PERMISSION_ROLES,
            0,
        );
        assert_eq!(
            role::NON_ADMIN_GRANTABLE_ROLE_MASK & UNCONSUMED_ADMIN_PERMISSION_ROLES,
            0,
        );
    }
}
