use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::seeds,
    error::ChanceryError,
};

// ─── Discriminator ────────────────────────────────────────────────────────────
pub const PAUSE_STATE_DISCRIMINATOR: [u8; 8] =
    [0x70, 0x61, 0x75, 0x73, 0x65, 0x73, 0x74, 0x00];

// ─── Global pause bits ────────────────────────────────────────────────────────
/// Bits for `global_pause_bits`. Each bit gates a class of operations.
pub mod pause_bit {
    pub const MINT:           u64 = 1 << 0;
    pub const REDEEM:         u64 = 1 << 1;
    pub const RESERVE:        u64 = 1 << 2;
    pub const MIGRATION:      u64 = 1 << 3;
    pub const ALL_SETTLEMENT: u64 = MINT | REDEEM;
    pub const ALL:            u64 = u64::MAX;
}

// ─── Account size ─────────────────────────────────────────────────────────────
//   [u8;8]   discriminator        =  8 @ 0
//   u16      version              =  2 @ 8
//   u8       bump                 =  1 @ 10
//   [u8;5]   _pad0                =  5 @ 11  -> u64 at 16
//   u64      global_pause_bits    =  8 @ 16
//   u32      reason_code          =  4 @ 24
//   [u8;4]   _pad1                =  4 @ 28  -> Pubkey at 32
//   [u8;32]  activated_by         = 32 @ 32
//   u64      activated_at_slot    =  8 @ 64  (64%8=0 Y)
//   u64      expires_at_slot      =  8 @ 72
//   [u8;32]  _reserved            = 32 @ 80
//                                  ─────
//                                  112 bytes
pub const PAUSE_STATE_SIZE: usize = 112;

// ─── State ────────────────────────────────────────────────────────────────────

/// Singleton global pause state.
/// Seed: [b"pause-state"]
///
/// The emergency authority writes here directly without a timelock.
/// Unpause requires governance or ops authority co-signature.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PauseState {
    pub discriminator:     [u8; 8],
    pub version:           u16,
    pub bump:              u8,
    pub _pad0:             [u8; 5],

    /// Bitfield using `pause_bit::*` constants.
    pub global_pause_bits: u64,

    /// Operator-defined reason code for audit trail.
    pub reason_code:       u32,
    pub _pad1:             [u8; 4],

    /// The authority that activated the pause.
    pub activated_by:      Pubkey,

    /// Slot at which the pause was activated.
    pub activated_at_slot: u64,

    /// Slot at which the pause auto-expires. 0 = no auto-expiry.
    pub expires_at_slot:   u64,
    pub _reserved:         [u8; 32],
}

// ─── Helpers ──────────────────────────────────────────────────────────────────
impl PauseState {
    pub fn pda(program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[seeds::PAUSE_STATE], program_id)
    }

    pub fn verify_pda(
        account   : &AccountInfo,
        program_id: &Pubkey,
    ) -> Result<u8, ProgramError> {
        let (expected, bump) = Self::pda(program_id);

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
            PAUSE_STATE_SIZE,
            PAUSE_STATE_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    pub(crate) fn load_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        expected_bump: u8,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;

        crate::state_loader::assert_stored_bump_matches(state.bump, expected_bump)?;

        Ok(state)
    }

    /// Load initialized state and prove that its identity fields resolve to
    /// this account's canonical PDA and stored bump.
    pub fn load_verified<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;
        let expected_bump = Self::verify_pda(account, &crate::id())?;

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
            PAUSE_STATE_SIZE,
            PAUSE_STATE_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    pub(crate) fn load_mut_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        expected_bump: u8,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;

        crate::state_loader::assert_stored_bump_matches(state.bump, expected_bump)?;

        Ok(state)
    }

    /// Mutable counterpart to `load_verified`.
    pub fn load_verified_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;
        let expected_bump = Self::verify_pda(account, &crate::id())?;

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
            PAUSE_STATE_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    // ── Guards ────────────────────────────────────────────────────────────────

    /// Assert a specific class of operation is not paused.
    pub fn assert_not_paused_for(
        &self,
        operation_bit: u64,
        current_slot : u64,
    ) -> Result<(), ProgramError> {
        // Auto-expiry: if past expires_at_slot, pause is implicitly lifted.
        if self.expires_at_slot != 0 && current_slot >= self.expires_at_slot {
            return Ok(());
        }

        if self.global_pause_bits & operation_bit != 0 {
            return Err(ChanceryError::GloballyPaused.into());
        }

        Ok(())
    }

    #[inline]
    pub fn is_paused_for(&self, operation_bit: u64, current_slot: u64) -> bool {
        if self.expires_at_slot != 0 && current_slot >= self.expires_at_slot {
            return false;
        }

        self.global_pause_bits & operation_bit != 0
    }

    /// Raw persisted bit visibility for audit and state reconciliation. This
    /// does not imply the pause is currently effective when auto-expiry has
    /// elapsed.
    #[inline]
    pub fn has_pause_bit(&self, operation_bit: u64) -> bool {
        self.global_pause_bits & operation_bit != 0
    }
}

// ─── Size assertion ───────────────────────────────────────────────────────────
const _: () = assert!(
    core::mem::size_of::<PauseState>() == PAUSE_STATE_SIZE,
    "PauseState size mismatch - update PAUSE_STATE_SIZE",
);
