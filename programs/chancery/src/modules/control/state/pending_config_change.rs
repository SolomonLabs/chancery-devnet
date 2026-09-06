//! Pending config-change state.
//!
//! Generic two-step + timelock framework for changes classified as Widening,
//! HighImpact, Dangerous, or Irreversible.
//!
//! Per
//!   - Separate `consumed_at_slot` field (296 bytes total, NOT stolen from _reserved)
//!   - `proposer_nonce: u64` to break slot-collision in change_id derivation
//!   - `expires_at_slot` is explicit (no zero-as-no-expiry)
//!
//! The PDA stores hashes only, not arbitrary payloads. Target handlers
//! recompute the hash from current state + their args and compare.

use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::error::ChanceryError;

pub const PENDING_CONFIG_CHANGE_DISCRIMINATOR: [u8; 8] =
    [0x70, 0x65, 0x6e, 0x64, 0x63, 0x66, 0x67, 0x01];  // "pendcfg\x01"

pub use crate::constants::config_change_status;

//   [u8;8]   discriminator                   =  8 @ 0
//   u16      version                         =  2 @ 8
//   u8       bump                            =  1 @ 10
//   u8       status                          =  1 @ 11
//   u8       risk_class                      =  1 @ 12
//   [u8;3]   _pad0                           =  3 @ 13
//   [u8;32]  change_id                       = 32 @ 16
//   u16      change_kind                     =  2 @ 48
//   [u8;6]   _pad1                           =  6 @ 50  -> align u64 at 56
//   [u8;32]  target_account                  = 32 @ 56
//   [u8;32]  old_value_hash                  = 32 @ 88
//   [u8;32]  new_value_hash                  = 32 @ 120
//   [u8;32]  proposed_by                     = 32 @ 152
//   u64      proposer_nonce                  =  8 @ 184
//   i64      executable_after_unix_timestamp =  8 @ 192
//   i64      expires_at_unix_timestamp       =  8 @ 200
//   u64      proposed_at_slot                =  8 @ 208
//   u64      accepted_at_slot                =  8 @ 216
//   u64      cancelled_at_slot               =  8 @ 224
//   u64      consumed_at_slot                =  8 @ 232
//   u64      last_event_sequence_nonce       =  8 @ 240
//   [u8;32]  rent_refund_recipient           = 32 @ 248
//   [u8;16]  _reserved                       = 16 @ 280
//                                            ─────
//                                            296 bytes
pub const PENDING_CONFIG_CHANGE_SIZE: usize = 296;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PendingConfigChange {
    pub discriminator:                   [u8; 8],
    pub version:                         u16,
    pub bump:                            u8,
    pub status:                          u8,
    pub risk_class:                      u8,
    pub _pad0:                           [u8; 3],
    pub change_id:                       [u8; 32],
    pub change_kind:                     u16,
    pub _pad1:                           [u8; 6],
    pub target_account:                  Pubkey,
    pub old_value_hash:                  [u8; 32],
    pub new_value_hash:                  [u8; 32],
    pub proposed_by:                     Pubkey,
    pub proposer_nonce:                  u64,
    pub executable_after_unix_timestamp: i64,
    pub expires_at_unix_timestamp:       i64,
    pub proposed_at_slot:                u64,
    pub accepted_at_slot:                u64,
    pub cancelled_at_slot:               u64,
    pub consumed_at_slot:                u64,
    pub last_event_sequence_nonce:       u64,

    /// Rent refund target recorded at propose time (the propose payer).
    /// Terminal close paths (consume, cancel, expiry sweep) refund here and
    /// only here - cleanup can never redirect rent.
    pub rent_refund_recipient:           Pubkey,

    /// Append-only layout headroom. Must remain zero until consumed by a
    /// layout change, and must be replenished in that same change.
    pub _reserved:                       [u8; 16],
}

impl PendingConfigChange {
    pub fn pda(change_id: &[u8; 32], program_id: &Pubkey) -> (Pubkey, u8) {
        // Seed: PENDING_CONFIG_CHANGE_SEED ("pending-config-change")
        Pubkey::find_program_address(
            &[crate::constants::seeds::PENDING_CONFIG_CHANGE, change_id],
            program_id,
        )
    }

    pub fn verify_pda(
        account   : &AccountInfo,
        change_id : &[u8; 32],
        program_id: &Pubkey,
    ) -> Result<u8, ProgramError> {
        let (expected, bump) = Self::pda(change_id, program_id);

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
            PENDING_CONFIG_CHANGE_SIZE,
            PENDING_CONFIG_CHANGE_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    /// Load initialized state and prove that its identity fields resolve to
    /// this account's canonical PDA and stored bump.
    pub fn load_verified<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;
        let expected_bump = Self::verify_pda(account, &state.change_id, &crate::id())?;

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
            PENDING_CONFIG_CHANGE_SIZE,
            PENDING_CONFIG_CHANGE_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    /// Mutable counterpart to `load_verified`.
    pub fn load_verified_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;
        let expected_bump = Self::verify_pda(account, &state.change_id, &crate::id())?;

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
            PENDING_CONFIG_CHANGE_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    pub fn assert_timelock_elapsed(&self, current_unix_timestamp: i64) -> Result<(), ProgramError> {
        if self.executable_after_unix_timestamp == 0
            || current_unix_timestamp < self.executable_after_unix_timestamp
        {
            return Err(ChanceryError::ConfigChangeTimelockNotElapsed.into());
        }

        Ok(())
    }

    pub fn assert_not_expired(&self, current_unix_timestamp: i64) -> Result<(), ProgramError> {
        if current_unix_timestamp >= self.expires_at_unix_timestamp {
            return Err(ChanceryError::ConfigChangeExpired.into());
        }

        Ok(())
    }

    /// Bind a proposal to the governance epoch that created it. Authority
    /// rotation invalidates every proposal made by the previous governance
    /// key, even when its timelock has elapsed or it was already accepted.
    pub fn assert_proposed_by_current_governance(
        &self,
        current_governance_authority: &Pubkey,
    ) -> Result<(), ProgramError> {
        if &self.proposed_by != current_governance_authority {
            return Err(ChanceryError::ConfigChangeProposerSuperseded.into());
        }

        Ok(())
    }
}

const _: () = assert!(
    core::mem::size_of::<PendingConfigChange>() == PENDING_CONFIG_CHANGE_SIZE,
    "PendingConfigChange size mismatch - update PENDING_CONFIG_CHANGE_SIZE",
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn struct_size_is_exactly_296() {
        assert_eq!(core::mem::size_of::<PendingConfigChange>(), 296);
    }

    #[test]
    fn discriminator_is_pendcfg() {
        let d = PENDING_CONFIG_CHANGE_DISCRIMINATOR;

        assert_eq!(&d[..7], b"pendcfg");
        assert_eq!(d[7], 0x01);  // version
    }

    use bytemuck::Zeroable;

    fn timed_pending(executable_after: i64, expires_at: i64) -> PendingConfigChange {
        let mut p = PendingConfigChange::zeroed();
        p.executable_after_unix_timestamp = executable_after;
        p.expires_at_unix_timestamp       = expires_at;
        p
    }

    #[test]
    fn timelock_passes_when_current_equals_executable_after() {
        // The check is `current < executable_after`, so equality is elapsed.
        let p = timed_pending(1_000, 10_000);
        assert!(p.assert_timelock_elapsed(1_000).is_ok());
    }

    #[test]
    fn timelock_fails_one_second_before_executable_after() {
        let p = timed_pending(1_000, 10_000);
        assert!(p.assert_timelock_elapsed(999).is_err());
    }

    #[test]
    fn timelock_zero_executable_after_fails_closed() {
        let p = timed_pending(0, 10_000);
        assert!(p.assert_timelock_elapsed(0).is_err());
        assert!(p.assert_timelock_elapsed(10_000).is_err());
    }

    #[test]
    fn expiry_rejects_at_exact_expires_at() {
        // `current >= expires_at` is expired.
        let p = timed_pending(0, 10_000);
        assert!(p.assert_not_expired(10_000).is_err());
    }

    #[test]
    fn expiry_passes_one_second_before_expires_at() {
        let p = timed_pending(0, 10_000);
        assert!(p.assert_not_expired(9_999).is_ok());
    }

    #[test]
    fn proposal_is_bound_to_current_governance_epoch() {
        let current_governance = Pubkey::new_from_array([0x11; 32]);
        let previous_governance = Pubkey::new_from_array([0x22; 32]);
        let mut pending = PendingConfigChange::zeroed();
        pending.proposed_by = current_governance;

        assert!(pending
            .assert_proposed_by_current_governance(&current_governance)
            .is_ok());
        assert_eq!(
            pending.assert_proposed_by_current_governance(&previous_governance),
            Err(ChanceryError::ConfigChangeProposerSuperseded.into()),
        );
    }
}
