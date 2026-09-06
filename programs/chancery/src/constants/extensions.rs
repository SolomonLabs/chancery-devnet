//! Token-2022 extension bit registry.
//!
//! Bit positions are Chancery-internal and do NOT correspond to Token-2022
//! ExtensionType enum discriminants. Canonical TLV tag constants and the
//! tag-to-bit map live in `modules::issuance::tlv::tlv_parser` (`extension_tlv_tag`,
//! `classify_mint_tlv_tag`).
//!
//! Append-only. Retired bit positions must NEVER be reused.

use solana_program_error::ProgramError;

use crate::error::ChanceryError;

/// Shared freshness gate for `AssetConfig` and `IssuedTokenControl`.
/// Zero `max_extension_observation_age_slots` disables only maximum-age
/// expiry; observations must still not be later than `current_slot`.
pub fn assert_extension_observation_fresh(
    extension_observed_at_slot:          u64,
    max_extension_observation_age_slots: u64,
    current_slot:                      u64,
) -> Result<(), ProgramError> {
    if extension_observed_at_slot > current_slot {
        return Err(ChanceryError::ExtensionObservationFuture.into());
    }

    if max_extension_observation_age_slots == 0 {
        return Ok(());
    }

    if current_slot - extension_observed_at_slot
        > max_extension_observation_age_slots
    {
        return Err(ChanceryError::ExtensionObservationStale.into());
    }

    Ok(())
}

#[cfg(test)]
mod freshness_tests {
    use super::*;

    #[test]
    fn equal_and_boundary_aged_observations_are_fresh() {
        assert!(assert_extension_observation_fresh(100, 10, 100).is_ok());
        assert!(assert_extension_observation_fresh(90, 10, 100).is_ok());
    }

    #[test]
    fn observations_older_than_the_maximum_are_stale() {
        assert_eq!(
            assert_extension_observation_fresh(89, 10, 100),
            Err(ChanceryError::ExtensionObservationStale.into()),
        );
    }

    #[test]
    fn future_observations_are_rejected_with_or_without_age_gating() {
        for maximum_age in [0, 10] {
            assert_eq!(
                assert_extension_observation_fresh(101, maximum_age, 100),
                Err(ChanceryError::ExtensionObservationFuture.into()),
            );
        }
    }
}

// ─── Token-2022 mint-level extension bitmask positions ──────────────────────
pub mod extension_bit {
    pub const TRANSFER_HOOK:                    u64 = 1 << 0;
    pub const CONFIDENTIAL_TRANSFER :           u64 = 1 << 1;
    pub const CONFIDENTIAL_MINT_BURN:           u64 = 1 << 2;
    pub const PERMANENT_DELEGATE:               u64 = 1 << 3;
    pub const PAUSABLE:                         u64 = 1 << 4;
    pub const METADATA_POINTER:                 u64 = 1 << 5;
    pub const TOKEN_METADATA:                   u64 = 1 << 6;
    pub const MEMO_TRANSFER:                    u64 = 1 << 7;
    /// Chancery-internal forward compat. Not a current Token-2022 extension.
    pub const PERMISSIONED_BURN:                u64 = 1 << 8;
    pub const MINT_CLOSE_AUTHORITY:             u64 = 1 << 9;
    pub const DEFAULT_ACCOUNT_STATE:            u64 = 1 << 10;
    // Bit 11 retired (was FREEZE_AUTHORITY - base mint field, not extension).
    // Do not reuse.
    pub const TRANSFER_FEE_CONFIG:              u64 = 1 << 12;
    pub const INTEREST_BEARING_CONFIG:          u64 = 1 << 13;
    pub const NON_TRANSFERABLE:                 u64 = 1 << 14;
    pub const SCALED_UI_AMOUNT:                 u64 = 1 << 15;
    pub const GROUP_POINTER:                    u64 = 1 << 16;
    pub const TOKEN_GROUP:                      u64 = 1 << 17;
    pub const GROUP_MEMBER_POINTER:             u64 = 1 << 18;
    pub const TOKEN_GROUP_MEMBER:               u64 = 1 << 19;
    pub const CONFIDENTIAL_TRANSFER_FEE_CONFIG: u64 = 1 << 20;
    /// TransferHook extension present with its program id set to None/zero.
    /// This reserves future hook activation without executing a hook today.
    pub const DORMANT_TRANSFER_HOOK:          u64 = 1 << 21;
}

// ─── Token-2022 account-level extension bitmask positions ───────────────────
pub mod account_extension_bit {
    pub const IMMUTABLE_OWNER:                  u64 = 1 << 0;
    pub const CONFIDENTIAL_TRANSFER_ACCOUNT:    u64 = 1 << 1;
    pub const MEMO_TRANSFER:                    u64 = 1 << 2;
    pub const CPI_GUARD:                        u64 = 1 << 3;
    pub const TRANSFER_HOOK_ACCOUNT:            u64 = 1 << 4;
    pub const PAUSABLE_ACCOUNT:                 u64 = 1 << 5;
    pub const CONFIDENTIAL_TRANSFER_FEE_AMOUNT: u64 = 1 << 6;
    pub const TRANSFER_FEE_AMOUNT:              u64 = 1 << 7;
    pub const NON_TRANSFERABLE_ACCOUNT:         u64 = 1 << 8;
}

// ─── Default forbidden masks for collateral ─────────────────────────────────
//
// An ACTIVE TransferHook is not executable by the current fixed-account
// TransferChecked settlement CPIs because ExtraAccountMetaList resolution is
// not implemented. A dormant hook (program id None/zero) is represented by
// DORMANT_TRANSFER_HOOK and is allowed; enabling it later changes the observed
// mask to TRANSFER_HOOK. An explicit collateral refresh persists that unsafe
// observation, and fund-moving consumers fail closed until the hook is disabled
// and the observation is refreshed again, or Chancery is upgraded to support
// hook-account resolution.
//
// Authority-bearing collateral extensions are intentionally not part of this
// protocol floor. Canonical collateral assets may legitimately use features
// such as PermanentDelegate, and Chancery cannot require those authorities to
// be its own PDAs. Their presence remains explicit governance policy: the live
// observation must be included in the asset's approved extension mask. This is
// distinct from the issued-token mint, whose extension authorities are wholly
// controlled and therefore bound to Chancery PDAs.
pub mod collateral_default_forbidden {
    use super::extension_bit;

    pub const MINT_FORBIDDEN_LO: u64 = extension_bit::TRANSFER_HOOK;
    pub const MINT_FORBIDDEN_HI: u64 = 0;
}

#[cfg(test)]
mod collateral_default_forbidden_tests {
    use super::{collateral_default_forbidden, extension_bit};

    #[test]
    fn active_transfer_hook_remains_protocol_forbidden() {
        assert_ne!(
            collateral_default_forbidden::MINT_FORBIDDEN_LO & extension_bit::TRANSFER_HOOK,
            0,
        );
    }

    #[test]
    fn permanent_delegate_is_governance_approvable_collateral_policy() {
        assert_eq!(
            collateral_default_forbidden::MINT_FORBIDDEN_LO & extension_bit::PERMANENT_DELEGATE,
            0,
        );
    }
}

// ─── Default forbidden masks for issued token (chancery policy, the specification) ─────
//
// Operators may not register reserved masks containing any of these bits.
// `initialize_issued_token_control` and `verify_issued_token_deployment`
// reject overlap with these masks.
pub mod issued_token_default_forbidden {
    use super::{account_extension_bit, extension_bit};

    pub const MINT_FORBIDDEN_LO: u64 =
          extension_bit::TRANSFER_FEE_CONFIG
        | extension_bit::INTEREST_BEARING_CONFIG
        | extension_bit::NON_TRANSFERABLE
        | extension_bit::SCALED_UI_AMOUNT;

    pub const MINT_FORBIDDEN_HI: u64 = 0;

    pub const ACCOUNT_FORBIDDEN_LO: u64 =
          account_extension_bit::TRANSFER_FEE_AMOUNT
        | account_extension_bit::NON_TRANSFERABLE_ACCOUNT;

    pub const ACCOUNT_FORBIDDEN_HI: u64 = 0;
}

// ─── Issued-token deployment flags ─────────────────────────────────
//
// `IssuedTokenControl.control_flags` carries:
//   - bit 0:    status_flag::INITIALIZED (existing)
//   - bits 4-7: individual deployment-verification gates
//   - bit 63:   READY_FOR_SETTLEMENT (settlement gates on this)
pub mod issued_token_deployment_flag {
    pub const MINT_VERIFIED:             u64 = 1 << 4;
    pub const AUTHORITIES_VERIFIED:      u64 = 1 << 5;
    pub const EXTENSIONS_VERIFIED:       u64 = 1 << 6;
    pub const ACCOUNT_STRATEGY_VERIFIED: u64 = 1 << 7;
    pub const READY_FOR_SETTLEMENT:      u64 = 1 << 63;

    pub const ALL_PRE_REQUIRED: u64 =
          MINT_VERIFIED
        | AUTHORITIES_VERIFIED
        | EXTENSIONS_VERIFIED
        | ACCOUNT_STRATEGY_VERIFIED;
}
