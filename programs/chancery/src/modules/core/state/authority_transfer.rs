use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::seeds,
    error::ChanceryError,
};

// ─── Discriminator ────────────────────────────────────────────────────────────
pub const AUTHORITY_TRANSFER_DISCRIMINATOR: [u8; 8] =
    [0x61, 0x75, 0x74, 0x68, 0x78, 0x66, 0x72, 0x00];

// ─── Account size ─────────────────────────────────────────────────────────────
// Layout (repr(C); all offsets in bytes):
//   [u8;8]   discriminator           =  8 @   0
//   u16       version                =  2 @   8
//   u8        bump                   =  1 @  10
//   u8        role_kind              =  1 @  11
//   [u8;4]   _pad0                   =  4 @  12
//   [u8;32]  old_authority           = 32 @  16
//   [u8;32]  proposed_authority      = 32 @  48
//   u64       proposed_at_slot       =  8 @  80
//   u64       executable_after_slot  =  8 @  88
//   [u8;32]  proposing_governance    = 32 @  96
//   u64       expires_at_slot        =  8 @ 128
//   [u8;16]  _reserved               = 16 @ 136
//                                      ─────
//                                      152 bytes
pub const AUTHORITY_TRANSFER_SIZE: usize =
    8     // discriminator
    + 2   // version
    + 1   // bump
    + 1   // role_kind
    + 4   // _pad0
    + 32  // old_authority
    + 32  // proposed_authority
    + 8   // proposed_at_slot
    + 8   // executable_after_slot
    + 32  // proposing_governance
    + 8   // expires_at_slot
    + 16; // _reserved

pub const AUTHORITY_TRANSFER_RESERVED_SIZE: usize = 16;

/// Total: 8+8+32+32+8+8+32+8+16 = 152 bytes

// ─── State ────────────────────────────────────────────────────────────────────

/// Pending authority transfer for a single role.
/// Seed: [b"authority-transfer", role_kind]
///
/// One PDA per role kind allows each role to have an independent in-flight
/// transfer without contention.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct AuthorityTransfer {
    pub discriminator:         [u8; 8],

    pub version:               u16,

    pub bump:                  u8,

    /// Role being transferred - one of `authority_role::*` constants.
    pub role_kind:             u8,

    pub _pad0:                 [u8; 4],

    /// The authority key being rotated.
    pub old_authority:         Pubkey,

    /// The proposed new authority key.
    pub proposed_authority:    Pubkey,

    /// Slot at which the proposal was created.
    pub proposed_at_slot:      u64,

    /// Earliest slot at which `accept_authority_transfer` may execute.
    /// Set by governance; zero means no timelock.
    pub executable_after_slot: u64,

    /// Governance authority that created this proposal. Acceptance requires
    /// this to still be the live governance authority, so every outstanding
    /// proposal is automatically void the moment governance rotates.
    pub proposing_governance:  Pubkey,

    /// Last slot (inclusive) at which the proposal may be accepted.
    /// `executable_after_slot + AUTHORITY_TRANSFER_ACCEPTANCE_WINDOW_SLOTS`;
    /// a proposal that is not accepted inside its window is void and must be
    /// re-proposed, so no stale rotation can linger indefinitely.
    pub expires_at_slot:       u64,

    /// Append-only layout headroom. Must remain zero until consumed by a
    /// versioned layout change, and must be replenished in that same change.
    pub _reserved: [u8; 16],
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

impl AuthorityTransfer {
    pub fn pda(role_kind: u8, program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[seeds::AUTHORITY_TRANSFER, &[role_kind]],
            program_id,
        )
    }

    pub fn verify_pda(
        account:    &AccountInfo,
        role_kind:  u8,
        program_id: &Pubkey,
    ) -> Result<u8, ProgramError> {
        let (expected, bump) = Self::pda(role_kind, program_id);

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
            AUTHORITY_TRANSFER_SIZE,
            AUTHORITY_TRANSFER_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    pub(crate) fn load_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        role_kind: u8,
        expected_bump: u8,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;

        if state.role_kind != role_kind {
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
        let expected_bump = Self::verify_pda(account, state.role_kind, &crate::id())?;

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
            AUTHORITY_TRANSFER_SIZE,
            AUTHORITY_TRANSFER_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    pub(crate) fn load_mut_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        role_kind: u8,
        expected_bump: u8,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;

        if state.role_kind != role_kind {
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
        let expected_bump = Self::verify_pda(account, state.role_kind, &crate::id())?;

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
            AUTHORITY_TRANSFER_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    /// Assert the timelock has elapsed relative to the given current slot.
    pub fn assert_timelock_elapsed(
        &self,
        current_slot: u64,
    ) -> Result<(), ProgramError> {
        if self.executable_after_slot != 0
            && current_slot < self.executable_after_slot
        {
            return Err(ChanceryError::AuthorityTransferTimelockActive.into());
        }

        Ok(())
    }

    /// Assert the account holds a live pending proposal (discriminator set).
    pub fn assert_pending(&self) -> Result<(), ProgramError> {
        if self.discriminator != AUTHORITY_TRANSFER_DISCRIMINATOR {
            return Err(ChanceryError::AuthorityTransferNotPending.into());
        }

        Ok(())
    }
}

// ─── Size assertion ───────────────────────────────────────────────────────────
const _: () = assert!(
    core::mem::size_of::<AuthorityTransfer>() == AUTHORITY_TRANSFER_SIZE,
    "AuthorityTransfer size mismatch - update AUTHORITY_TRANSFER_SIZE",
);
