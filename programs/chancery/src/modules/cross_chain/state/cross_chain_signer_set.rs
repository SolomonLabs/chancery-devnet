use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::{cross_chain_signer_set_flag, seeds, status_flag},
    error::ChanceryError,
};

// ─── Discriminator ────────────────────────────────────────────────────────────
pub const CROSS_CHAIN_SIGNER_SET_DISCRIMINATOR: [u8; 8] =
    [0x63, 0x63, 0x73, 0x69, 0x67, 0x73, 0x65, 0x74];  // "ccsigset"

// ─── Account size ─────────────────────────────────────────────────────────────
//   [u8;8]   discriminator               =  8 @ 0
//   u16      version                     =  2 @ 8
//   u8       bump                        =  1 @ 10
//   u8       threshold                   =  1 @ 11
//   u8       signer_count                =  1 @ 12
//   [u8;3]   _pad0                       =  3 @ 13  -> i64 at 16
//   i64      valid_after_unix_timestamp  =  8 @ 16  (16%8=0 Y)
//   i64      expires_at_unix_timestamp   =  8 @ 24
//   u64      status_flags                =  8 @ 32
//   [u8;32]  signer_set_id               = 32 @ 40
//   [u8;32]  signer_root                 = 32 @ 72
//   [u8;32]  created_by                  = 32 @ 104
//   [u8;32]  _reserved                   = 32 @ 136
//                                         ─────
//                                         168 bytes
pub const CROSS_CHAIN_SIGNER_SET_SIZE: usize = 168;

// ─── State ────────────────────────────────────────────────────────────────────

/// Threshold attestation quorum for a remote domain. Spec §2.9 / §9.7.
/// Seed: [b"cross-chain-signer-set", signer_set_id]
///
/// `signer_root` is the root of the canonical Keccak-256 Merkle tree:
/// sort and deduplicate raw 20-byte secp256k1 signer addresses, hash each
/// address as one leaf, pad by duplicating the final leaf to a power of two,
/// then hash lexicographically sorted child pairs. The full signer list is
/// held off-chain; verification passes each address and its proof as
/// instruction args. This keeps the account fixed-size regardless of count.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CrossChainSignerSet {
    pub discriminator:              [u8; 8],
    pub version:                    u16,
    pub bump:                       u8,

    /// Minimum number of valid signatures required. `0 < threshold <= signer_count`.
    pub threshold:                  u8,

    /// Total number of signers committed to in `signer_root`.
    pub signer_count:               u8,
    pub _pad0:                      [u8; 3],

    /// Set inactive before this timestamp.
    pub valid_after_unix_timestamp: i64,

    /// Set expires at this timestamp; verification rejects past expiry.
    /// Zero means no auto-expiry.
    pub expires_at_unix_timestamp:  i64,

    /// `status_flag::INITIALIZED` (bit 0) plus
    /// `cross_chain_signer_set_flag::REVOKED` (bit 16).
    pub status_flags:               u64,

    /// Opaque set identifier; referenced by `RemoteDomainPolicy.signer_set_id`.
    pub signer_set_id:              [u8; 32],

    /// Merkle root over the canonical signer list. Layout pinned in the
    /// EVM daughter contract spec (spec 11).
    pub signer_root:                [u8; 32],

    /// Authority that registered the set.
    pub created_by:                 Pubkey,
    pub _reserved :                 [u8; 32],
}

// ─── Helpers ──────────────────────────────────────────────────────────────────
impl CrossChainSignerSet {
    pub fn pda(signer_set_id: &[u8; 32], program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[seeds::CROSS_CHAIN_SIGNER_SET, signer_set_id.as_ref()],
            program_id,
        )
    }

    pub fn verify_pda(
        account:       &AccountInfo,
        signer_set_id: &[u8; 32],
        program_id:    &Pubkey,
    ) -> Result<u8, ProgramError> {
        let (expected, bump) = Self::pda(signer_set_id, program_id);

        if account.key != &expected {
            return Err(ChanceryError::InvalidPda.into());
        }

        Ok(bump)
    }

    pub fn load<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = crate::state_loader::load_state::<Self>(
            account,
            CROSS_CHAIN_SIGNER_SET_SIZE,
            CROSS_CHAIN_SIGNER_SET_DISCRIMINATOR,
            ChanceryError::SignerSetNotFound,
        )?;

        Ok(state)
    }

    pub(crate) fn load_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        signer_set_id: &[u8; 32],
        expected_bump: u8,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;

        if state.signer_set_id != *signer_set_id {
            return Err(ChanceryError::InvalidPda.into());
        }

        crate::state_loader::assert_stored_bump_matches(state.bump, expected_bump)?;

        Ok(state)
    }

    /// Load initialized state and prove that its identity fields resolve to
    /// this account's canonical PDA and stored bump.
    pub fn load_verified<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;
        let expected_bump = Self::verify_pda(account, &state.signer_set_id, &crate::id())?;

        if state.bump != expected_bump {
            return Err(ChanceryError::StoredBumpMismatch.into());
        }

        Ok(state)
    }

    pub fn load_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = crate::state_loader::load_state_mut::<Self>(
            account,
            CROSS_CHAIN_SIGNER_SET_SIZE,
            CROSS_CHAIN_SIGNER_SET_DISCRIMINATOR,
            ChanceryError::SignerSetNotFound,
        )?;

        Ok(state)
    }

    pub(crate) fn load_mut_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        signer_set_id: &[u8; 32],
        expected_bump: u8,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;

        if state.signer_set_id != *signer_set_id {
            return Err(ChanceryError::InvalidPda.into());
        }

        crate::state_loader::assert_stored_bump_matches(state.bump, expected_bump)?;

        Ok(state)
    }

    /// Mutable counterpart to `load_verified`.
    pub fn load_verified_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;
        let expected_bump = Self::verify_pda(account, &state.signer_set_id, &crate::id())?;

        if state.bump != expected_bump {
            return Err(ChanceryError::StoredBumpMismatch.into());
        }

        Ok(state)
    }

    pub fn load_uninitialized_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        crate::state_loader::load_uninitialized_state_mut::<Self>(
            account,
            CROSS_CHAIN_SIGNER_SET_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    // ── Guards ────────────────────────────────────────────────────────────────

    #[inline]
    pub fn is_initialized(&self) -> bool {
        self.status_flags & status_flag::INITIALIZED != 0
    }

    #[inline]
    pub fn is_revoked(&self) -> bool {
        self.status_flags & cross_chain_signer_set_flag::REVOKED != 0
    }

    pub fn assert_active(&self) -> Result<(), ProgramError> {
        if self.threshold == 0
            || self.threshold > self.signer_count
            || self.threshold > crate::modules::cross_chain::attestation::MAX_ATTESTATION_THRESHOLD
            || self.signer_count
                > crate::modules::cross_chain::attestation::MAX_CROSS_CHAIN_SIGNER_COUNT
            || self.signer_set_id == [0u8; 32]
            || self.signer_root == [0u8; 32]
        {
            return Err(ChanceryError::AttestationResourceLimitExceeded.into());
        }

        if !self.is_initialized() {
            return Err(ChanceryError::SignerSetNotFound.into());
        }

        if self.is_revoked() {
            return Err(ChanceryError::SignerSetInactive.into());
        }

        Ok(())
    }

    /// `valid_after <= now < expires_at` (both inclusive on the lower bound;
    /// expiry is exclusive). Treat zero `expires_at_unix_timestamp` as no auto-expiry.
    pub fn assert_temporally_valid(&self, now: i64) -> Result<(), ProgramError> {
        if self.valid_after_unix_timestamp != 0 && now < self.valid_after_unix_timestamp {
            return Err(ChanceryError::SignerSetInactive.into());
        }

        if self.expires_at_unix_timestamp != 0 && now >= self.expires_at_unix_timestamp {
            return Err(ChanceryError::SignerSetExpired.into());
        }

        Ok(())
    }

    /// A signer set with an automatic expiry may be registered and reviewed,
    /// but it cannot back an opened corridor. Otherwise a head message can
    /// cross the set-expiry boundary and become neither consumable nor safely
    /// retireable under the protocol's current-trust-root rule.
    pub fn assert_non_expiring_for_live_corridor(&self) -> Result<(), ProgramError> {
        if self.expires_at_unix_timestamp != 0 {
            return Err(ChanceryError::LiveCorridorSignerSetMustNotExpire.into());
        }

        Ok(())
    }

    pub fn assert_id_matches(&self, expected: &[u8; 32]) -> Result<(), ProgramError> {
        if &self.signer_set_id != expected {
            return Err(ChanceryError::SignerSetIdMismatch.into());
        }

        Ok(())
    }

    pub fn assert_threshold_met(&self, valid_count: u8) -> Result<(), ProgramError> {
        if valid_count < self.threshold {
            return Err(ChanceryError::AttestationThresholdNotMet.into());
        }

        Ok(())
    }
}

// ─── Size assertion ───────────────────────────────────────────────────────────
const _: () = assert!(
    core::mem::size_of::<CrossChainSignerSet>() == CROSS_CHAIN_SIGNER_SET_SIZE,
    "CrossChainSignerSet size mismatch - update CROSS_CHAIN_SIGNER_SET_SIZE",
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_corridor_rejects_auto_expiring_signer_set() {
        let mut signer_set = CrossChainSignerSet::zeroed();
        signer_set.expires_at_unix_timestamp = 1;

        assert_eq!(
            signer_set.assert_non_expiring_for_live_corridor(),
            Err(ChanceryError::LiveCorridorSignerSetMustNotExpire.into()),
        );

        signer_set.expires_at_unix_timestamp = 0;
        assert!(signer_set.assert_non_expiring_for_live_corridor().is_ok());
    }
}
