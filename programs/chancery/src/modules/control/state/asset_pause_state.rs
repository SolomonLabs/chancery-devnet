use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::seeds,
    error::ChanceryError,
};

// ─── Discriminator ────────────────────────────────────────────────────────────
pub const ASSET_PAUSE_STATE_DISCRIMINATOR: [u8; 8] =
    [0x61, 0x73, 0x70, 0x61, 0x75, 0x73, 0x65, 0x00];

// ─── Account size ─────────────────────────────────────────────────────────────
//   [u8;8]   discriminator        =  8 @ 0
//   u16      version              =  2 @ 8
//   u8       bump                 =  1 @ 10
//   [u8;5]   _pad0                =  5 @ 11  -> Pubkey at 16
//   [u8;32]  asset_mint           = 32 @ 16
//   u64      asset_pause_bits     =  8 @ 48  (48%8=0 Y)
//   u32      reason_code          =  4 @ 56
//   [u8;4]   _pad1                =  4 @ 60  -> Pubkey at 64
//   [u8;32]  activated_by         = 32 @ 64
//   u64      activated_at_slot    =  8 @ 96  (96%8=0 Y)
//   u64      expires_at_slot      =  8 @ 104
//   [u8;32]  _reserved            = 32 @ 112
//                                  ─────
//                                  144 bytes
pub const ASSET_PAUSE_STATE_SIZE: usize = 144;

// ─── State ────────────────────────────────────────────────────────────────────

/// Per-asset pause state.
/// Seed: [b"asset-pause", asset_mint]
///
/// Allows pausing a single asset's mint/redeem without a global pause.
/// Uses the same `pause_bit::*` constants as `PauseState` for consistency.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct AssetPauseState {
    pub discriminator:     [u8; 8],
    pub version:           u16,
    pub bump:              u8,
    pub _pad0:             [u8; 5],
    pub asset_mint:        Pubkey,

    /// Bitfield using `pause_bit::*` constants, scoped to this asset.
    pub asset_pause_bits:  u64,
    pub reason_code:       u32,
    pub _pad1:             [u8; 4],
    pub activated_by:      Pubkey,
    pub activated_at_slot: u64,

    /// Slot at which this per-asset pause auto-expires. 0 = no auto-expiry.
    pub expires_at_slot:   u64,
    pub _reserved:         [u8; 32],
}

// ─── Helpers ──────────────────────────────────────────────────────────────────
impl AssetPauseState {
    pub fn pda(asset_mint: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[seeds::ASSET_PAUSE, asset_mint.as_ref()],
            program_id,
        )
    }

    pub fn verify_pda(
        account   : &AccountInfo,
        asset_mint: &Pubkey,
        program_id: &Pubkey,
    ) -> Result<u8, ProgramError> {
        let (expected, bump) = Self::pda(asset_mint, program_id);

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
            ASSET_PAUSE_STATE_SIZE,
            ASSET_PAUSE_STATE_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    /// Load initialized state after the caller has already verified the PDA
    /// for `asset_mint`, reusing that derivation's bump instead of deriving it
    /// again from the stored identity field.
    pub(crate) fn load_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        asset_mint: &Pubkey,
        expected_bump: u8,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;

        if state.asset_mint != *asset_mint {
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
        let expected_bump = Self::verify_pda(account, &state.asset_mint, &crate::id())?;

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
            ASSET_PAUSE_STATE_SIZE,
            ASSET_PAUSE_STATE_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    pub(crate) fn load_mut_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        asset_mint: &Pubkey,
        expected_bump: u8,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;

        if state.asset_mint != *asset_mint {
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
        let expected_bump = Self::verify_pda(account, &state.asset_mint, &crate::id())?;

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
            ASSET_PAUSE_STATE_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    pub fn assert_not_paused_for(
        &self,
        operation_bit: u64,
        current_slot : u64,
    ) -> Result<(), ProgramError> {
        if self.expires_at_slot != 0 && current_slot >= self.expires_at_slot {
            return Ok(());
        }

        if self.asset_pause_bits & operation_bit != 0 {
            return Err(ChanceryError::AssetPaused.into());
        }

        Ok(())
    }
}

// ─── Size assertion ───────────────────────────────────────────────────────────
const _: () = assert!(
    core::mem::size_of::<AssetPauseState>() == ASSET_PAUSE_STATE_SIZE,
    "AssetPauseState size mismatch - update ASSET_PAUSE_STATE_SIZE",
);
