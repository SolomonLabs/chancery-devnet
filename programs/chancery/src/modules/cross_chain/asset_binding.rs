//! Cross-chain asset binding.
//!
//! Validates that the `remote_asset` field on inbound and outbound cross-
//! chain messages matches the policy configured for the remote domain.
//!
//! Per
//!   - `remote_asset` is `[u8; 32]` (left-padded EVM, like remote_chancery_contract)
//!   - In MINT mode: nonzero remote_asset is REJECTED (mint authority is local)
//!   - In RELEASE mode: nonzero remote_asset is REQUIRED and must match policy
//!   - Helper covers all 5 message kinds (existing 0x00..0x02 + new 0x03, 0x04)

use crate::{
    constants::{cross_chain_message_kind, cross_chain_message_kind_extensions, remote_domain_mode},
    error::ChanceryError,
    modules::cross_chain::state::remote_domain_policy::RemoteDomainPolicy,
};

/// Direction-of-flow indicator. Used to compute symmetric mode requirements.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageDirection {
    Inbound,
    Outbound,
}

#[inline]
fn is_zero_32(bytes: &[u8; 32]) -> bool {
    let mut acc: u8 = 0;
    for i in 0..32 {
        acc |= bytes[i];
    }
    acc == 0
}

/// Per-direction binding check.
///
/// MINT mode: outbound REDEEM_TO_REMOTE_MINT and inbound MINT_FROM_REMOTE_BURN.
///   The remote chain mints/burns its representation; chancery's remote_asset
///   field MUST be all-zero. The local chancery mint is the authority.
///
/// RELEASE mode: outbound REDEEM_TO_REMOTE_RELEASE and inbound RELEASE_FROM_REMOTE_LOCK.
///   The remote chain locks/releases its native asset; chancery's
///   remote_asset field MUST be the configured RemoteDomainPolicy.remote_asset.
///
/// LOCK mode (outbound): OUTBOUND_LOCK_FOR_REMOTE_RELEASE.
///   Same constraint as RELEASE - chancery refers to a specific remote asset.
pub fn assert_inbound_remote_asset_binding(
    message_kind:            u8,
    remote_asset_in_message: &[u8; 32],
    remote_domain_policy:    &RemoteDomainPolicy,
) -> Result<(), ChanceryError> {
    use cross_chain_message_kind as ck;
    use cross_chain_message_kind_extensions as cke;
    
    let policy_mode:                 u8        = remote_domain_policy.mode;
    let policy_remote_asset:         &[u8; 32] = &remote_domain_policy.remote_asset;
    let policy_remote_asset_is_zero: bool      = is_zero_32(policy_remote_asset);

    match message_kind {
        ck::INBOUND_MINT_FROM_REMOTE_BURN => {
            // Mint mode required; remote_asset must be zero.
            if policy_mode != remote_domain_mode::MINT {
                return Err(ChanceryError::RemoteDomainModeMismatch);
            }
            if !is_zero_32(remote_asset_in_message) {
                return Err(ChanceryError::RemoteAssetForbiddenInMintMode);
            }
            if !policy_remote_asset_is_zero {
                return Err(ChanceryError::RemoteAssetBindingMismatch);
            }
            Ok(())
        }
        cke::INBOUND_RELEASE_FROM_REMOTE_LOCK => {
            // Release mode required; remote_asset must match policy.
            if policy_mode != remote_domain_mode::RELEASE {
                return Err(ChanceryError::RemoteDomainModeMismatch);
            }
            if is_zero_32(remote_asset_in_message) {
                return Err(ChanceryError::RemoteAssetRequiredInReleaseMode);
            }
            if remote_asset_in_message != policy_remote_asset {
                return Err(ChanceryError::RemoteAssetBindingMismatch);
            }
            Ok(())
        }
        _ => Err(ChanceryError::InvalidMessageKind),
    }
}

pub fn assert_outbound_remote_asset_binding(
    message_kind:            u8,
    remote_asset_in_message: &[u8; 32],
    remote_domain_policy:    &RemoteDomainPolicy,
) -> Result<(), ChanceryError> {
    use cross_chain_message_kind as ck;
    use cross_chain_message_kind_extensions as cke;

    let policy_mode:                  u8        = remote_domain_policy.mode;
    let policy_remote_asset:          &[u8; 32] = &remote_domain_policy.remote_asset;
    let policy_remote_asset_is_zero:  bool      = is_zero_32(policy_remote_asset);

    match message_kind {
        ck::OUTBOUND_REDEEM_TO_REMOTE_MINT => {
            if policy_mode != remote_domain_mode::MINT {
                return Err(ChanceryError::RemoteDomainModeMismatch);
            }
            if !is_zero_32(remote_asset_in_message) {
                return Err(ChanceryError::RemoteAssetForbiddenInMintMode);
            }
            if !policy_remote_asset_is_zero {
                return Err(ChanceryError::RemoteAssetBindingMismatch);
            }
            Ok(())
        }
        ck::OUTBOUND_REDEEM_TO_REMOTE_RELEASE => {
            if policy_mode != remote_domain_mode::RELEASE {
                return Err(ChanceryError::RemoteDomainModeMismatch);
            }
            if is_zero_32(remote_asset_in_message) {
                return Err(ChanceryError::RemoteAssetRequiredInReleaseMode);
            }
            if remote_asset_in_message != policy_remote_asset {
                return Err(ChanceryError::RemoteAssetBindingMismatch);
            }
            Ok(())
        }
        cke::OUTBOUND_LOCK_FOR_REMOTE_RELEASE => {
            if policy_mode != remote_domain_mode::RELEASE {
                return Err(ChanceryError::RemoteDomainModeMismatch);
            }
            if is_zero_32(remote_asset_in_message) {
                return Err(ChanceryError::RemoteAssetRequiredInReleaseMode);
            }
            if remote_asset_in_message != policy_remote_asset {
                return Err(ChanceryError::RemoteAssetBindingMismatch);
            }
            Ok(())
        }
        _ => Err(ChanceryError::InvalidMessageKind),
    }
}

/// Convenience wrapper: dispatches by direction.
pub fn assert_remote_asset_binding(
    direction:               MessageDirection,
    message_kind:            u8,
    remote_asset_in_message: &[u8; 32],
    remote_domain_policy:    &RemoteDomainPolicy,
) -> Result<(), ChanceryError> {
    match direction {
        MessageDirection::Inbound => {
            assert_inbound_remote_asset_binding(message_kind, remote_asset_in_message, remote_domain_policy)
        }
        MessageDirection::Outbound => {
            assert_outbound_remote_asset_binding(message_kind, remote_asset_in_message, remote_domain_policy)
        }
    }
}

#[cfg(test)]
mod tests {
    //! Exhaustive coverage of the per-message asset-binding gates that
    //! `emit_outbound_message` and `consume_inbound_message` rely on
    //! (PR #20, issue-20). The helper is a pure function over
    //! `RemoteDomainPolicy`; we construct synthetic policies with the
    //! `Zeroable` derive so each case isolates `(mode, remote_asset,
    //! message_kind, asset_in_message)` from on-chain state.
    use super::*;
    use bytemuck::Zeroable;

    const ZERO_ASSET: [u8; 32] = [0u8; 32];
    const ASSET_A:    [u8; 32] = [0xAAu8; 32];
    const ASSET_B:    [u8; 32] = [0xBBu8; 32];

    fn policy(mode: u8, remote_asset: [u8; 32]) -> RemoteDomainPolicy {
        let mut p = RemoteDomainPolicy::zeroed();
        p.mode         = mode;
        p.remote_asset = remote_asset;
        p
    }

    // ── Outbound: MINT mode + OUTBOUND_REDEEM_TO_REMOTE_MINT ────────────────
    #[test]
    fn outbound_mint_redeem_to_mint_zero_asset_passes() {
        let p = policy(remote_domain_mode::MINT, ZERO_ASSET);
        let r = assert_outbound_remote_asset_binding(
            cross_chain_message_kind::OUTBOUND_REDEEM_TO_REMOTE_MINT,
            &ZERO_ASSET,
            &p,
        );
        assert!(r.is_ok());
    }

    #[test]
    fn outbound_mint_redeem_to_mint_nonzero_asset_rejected() {
        let p = policy(remote_domain_mode::MINT, ZERO_ASSET);
        let r = assert_outbound_remote_asset_binding(
            cross_chain_message_kind::OUTBOUND_REDEEM_TO_REMOTE_MINT,
            &ASSET_A,
            &p,
        );
        assert_eq!(r, Err(ChanceryError::RemoteAssetForbiddenInMintMode));
    }

    // ── Outbound: MINT mode + OUTBOUND_REDEEM_TO_REMOTE_RELEASE (corridor confusion) ──
    #[test]
    fn outbound_mint_policy_redeem_to_release_rejected_as_mode_mismatch() {
        let p = policy(remote_domain_mode::MINT, ZERO_ASSET);
        let r = assert_outbound_remote_asset_binding(
            cross_chain_message_kind::OUTBOUND_REDEEM_TO_REMOTE_RELEASE,
            &ASSET_A,
            &p,
        );
        assert_eq!(r, Err(ChanceryError::RemoteDomainModeMismatch));
    }

    // ── Outbound: RELEASE mode + OUTBOUND_REDEEM_TO_REMOTE_RELEASE ──────────
    #[test]
    fn outbound_release_redeem_to_release_matching_asset_passes() {
        let p = policy(remote_domain_mode::RELEASE, ASSET_A);
        let r = assert_outbound_remote_asset_binding(
            cross_chain_message_kind::OUTBOUND_REDEEM_TO_REMOTE_RELEASE,
            &ASSET_A,
            &p,
        );
        assert!(r.is_ok());
    }

    #[test]
    fn outbound_release_redeem_to_release_zero_asset_rejected() {
        let p = policy(remote_domain_mode::RELEASE, ASSET_A);
        let r = assert_outbound_remote_asset_binding(
            cross_chain_message_kind::OUTBOUND_REDEEM_TO_REMOTE_RELEASE,
            &ZERO_ASSET,
            &p,
        );
        assert_eq!(r, Err(ChanceryError::RemoteAssetRequiredInReleaseMode));
    }

    #[test]
    fn outbound_release_redeem_to_release_mismatched_asset_rejected() {
        let p = policy(remote_domain_mode::RELEASE, ASSET_A);
        let r = assert_outbound_remote_asset_binding(
            cross_chain_message_kind::OUTBOUND_REDEEM_TO_REMOTE_RELEASE,
            &ASSET_B,
            &p,
        );
        assert_eq!(r, Err(ChanceryError::RemoteAssetBindingMismatch));
    }

    // ── Outbound: RELEASE mode + OUTBOUND_REDEEM_TO_REMOTE_MINT (mode mismatch) ──
    #[test]
    fn outbound_release_policy_redeem_to_mint_rejected_as_mode_mismatch() {
        let p = policy(remote_domain_mode::RELEASE, ASSET_A);
        let r = assert_outbound_remote_asset_binding(
            cross_chain_message_kind::OUTBOUND_REDEEM_TO_REMOTE_MINT,
            &ZERO_ASSET,
            &p,
        );
        assert_eq!(r, Err(ChanceryError::RemoteDomainModeMismatch));
    }

    // ── Outbound: LOCK_FOR_REMOTE_RELEASE (extension, RELEASE-only) ────────
    #[test]
    fn outbound_lock_for_release_release_policy_matching_asset_passes() {
        let p = policy(remote_domain_mode::RELEASE, ASSET_A);
        let r = assert_outbound_remote_asset_binding(
            cross_chain_message_kind_extensions::OUTBOUND_LOCK_FOR_REMOTE_RELEASE,
            &ASSET_A,
            &p,
        );
        assert!(r.is_ok());
    }

    #[test]
    fn outbound_lock_for_release_mint_policy_rejected_as_mode_mismatch() {
        let p = policy(remote_domain_mode::MINT, ZERO_ASSET);
        let r = assert_outbound_remote_asset_binding(
            cross_chain_message_kind_extensions::OUTBOUND_LOCK_FOR_REMOTE_RELEASE,
            &ASSET_A,
            &p,
        );
        assert_eq!(r, Err(ChanceryError::RemoteDomainModeMismatch));
    }

    // ── Outbound: inbound message_kind on outbound path -> InvalidMessageKind ──
    #[test]
    fn outbound_inbound_kind_rejected_as_invalid_message_kind() {
        let p = policy(remote_domain_mode::MINT, ZERO_ASSET);
        let r = assert_outbound_remote_asset_binding(
            cross_chain_message_kind::INBOUND_MINT_FROM_REMOTE_BURN,
            &ZERO_ASSET,
            &p,
        );
        assert_eq!(r, Err(ChanceryError::InvalidMessageKind));
    }

    // ── Inbound: MINT mode + INBOUND_MINT_FROM_REMOTE_BURN ─────────────────
    #[test]
    fn inbound_mint_from_burn_zero_asset_passes() {
        let p = policy(remote_domain_mode::MINT, ZERO_ASSET);
        let r = assert_inbound_remote_asset_binding(
            cross_chain_message_kind::INBOUND_MINT_FROM_REMOTE_BURN,
            &ZERO_ASSET,
            &p,
        );
        assert!(r.is_ok());
    }

    #[test]
    fn inbound_mint_from_burn_nonzero_asset_rejected() {
        let p = policy(remote_domain_mode::MINT, ZERO_ASSET);
        let r = assert_inbound_remote_asset_binding(
            cross_chain_message_kind::INBOUND_MINT_FROM_REMOTE_BURN,
            &ASSET_A,
            &p,
        );
        assert_eq!(r, Err(ChanceryError::RemoteAssetForbiddenInMintMode));
    }

    #[test]
    fn inbound_mint_from_burn_release_policy_rejected_as_mode_mismatch() {
        let p = policy(remote_domain_mode::RELEASE, ASSET_A);
        let r = assert_inbound_remote_asset_binding(
            cross_chain_message_kind::INBOUND_MINT_FROM_REMOTE_BURN,
            &ZERO_ASSET,
            &p,
        );
        assert_eq!(r, Err(ChanceryError::RemoteDomainModeMismatch));
    }

    // ── Inbound: RELEASE mode + INBOUND_RELEASE_FROM_REMOTE_LOCK ───────────
    #[test]
    fn inbound_release_from_lock_matching_asset_passes() {
        let p = policy(remote_domain_mode::RELEASE, ASSET_A);
        let r = assert_inbound_remote_asset_binding(
            cross_chain_message_kind_extensions::INBOUND_RELEASE_FROM_REMOTE_LOCK,
            &ASSET_A,
            &p,
        );
        assert!(r.is_ok());
    }

    #[test]
    fn inbound_release_from_lock_zero_asset_rejected() {
        let p = policy(remote_domain_mode::RELEASE, ASSET_A);
        let r = assert_inbound_remote_asset_binding(
            cross_chain_message_kind_extensions::INBOUND_RELEASE_FROM_REMOTE_LOCK,
            &ZERO_ASSET,
            &p,
        );
        assert_eq!(r, Err(ChanceryError::RemoteAssetRequiredInReleaseMode));
    }

    #[test]
    fn inbound_release_from_lock_mismatched_asset_rejected() {
        let p = policy(remote_domain_mode::RELEASE, ASSET_A);
        let r = assert_inbound_remote_asset_binding(
            cross_chain_message_kind_extensions::INBOUND_RELEASE_FROM_REMOTE_LOCK,
            &ASSET_B,
            &p,
        );
        assert_eq!(r, Err(ChanceryError::RemoteAssetBindingMismatch));
    }

    #[test]
    fn inbound_release_from_lock_mint_policy_rejected_as_mode_mismatch() {
        let p = policy(remote_domain_mode::MINT, ZERO_ASSET);
        let r = assert_inbound_remote_asset_binding(
            cross_chain_message_kind_extensions::INBOUND_RELEASE_FROM_REMOTE_LOCK,
            &ASSET_A,
            &p,
        );
        assert_eq!(r, Err(ChanceryError::RemoteDomainModeMismatch));
    }

    // ── Inbound: outbound message_kind on inbound path -> InvalidMessageKind ──
    #[test]
    fn inbound_outbound_kind_rejected_as_invalid_message_kind() {
        let p = policy(remote_domain_mode::MINT, ZERO_ASSET);
        let r = assert_inbound_remote_asset_binding(
            cross_chain_message_kind::OUTBOUND_REDEEM_TO_REMOTE_MINT,
            &ZERO_ASSET,
            &p,
        );
        assert_eq!(r, Err(ChanceryError::InvalidMessageKind));
    }

    // ── Convenience dispatcher round-trips ────────────────────────────────
    #[test]
    fn dispatcher_inbound_routes_to_inbound_check() {
        let p = policy(remote_domain_mode::MINT, ZERO_ASSET);
        assert!(assert_remote_asset_binding(
            MessageDirection::Inbound,
            cross_chain_message_kind::INBOUND_MINT_FROM_REMOTE_BURN,
            &ZERO_ASSET,
            &p,
        ).is_ok());
    }

    #[test]
    fn dispatcher_outbound_routes_to_outbound_check() {
        let p = policy(remote_domain_mode::RELEASE, ASSET_A);
        assert!(assert_remote_asset_binding(
            MessageDirection::Outbound,
            cross_chain_message_kind_extensions::OUTBOUND_LOCK_FOR_REMOTE_RELEASE,
            &ASSET_A,
            &p,
        ).is_ok());
    }
}
