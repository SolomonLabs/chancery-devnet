use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::{
        assert_extension_observation_fresh,
        issued_token_default_forbidden,
        issued_token_deployment_flag,
        seeds,
        status_flag,
    },
    error::ChanceryError,
};

// ─── Discriminator ────────────────────────────────────────────────────────────
pub const ISSUED_TOKEN_CONTROL_DISCRIMINATOR: [u8; 8] =
    [0x69, 0x74, 0x63, 0x74, 0x72, 0x6c, 0x30, 0x01];  // "itctrl01"

// ─── Account size ─────────────────────────────────────────────────────────────
//   [u8;8]   discriminator                        =   8 @   0
//   u16      version                              =   2 @   8
//   u8       bump                                 =   1 @  10
//   [u8;5]   _pad0                                =   5 @  11  -> u64 at 16
//   [u8;32]  issued_token_mint                    =  32 @  16
//   [u8;32]  issued_token_program                 =  32 @  48
//   [u64;2]  reserved_mint_extension_mask         =  16 @  80   (80%8=0 Y)
//   [u64;2]  active_mint_extension_mask           =  16 @  96
//   [u64;2]  reserved_account_extension_mask      =  16 @ 112
//   [u64;2]  active_account_extension_mask        =  16 @ 128
//   u64      control_flags                        =   8 @ 144
//   [u8;32]  mint_authority_pda                   =  32 @ 152
//   [u8;32]  freeze_authority_pda                 =  32 @ 184
//   [u8;32]  close_mint_authority_pda             =  32 @ 216
//   [u8;32]  transfer_hook_authority_pda          =  32 @ 248
//   [u8;32]  permanent_delegate_authority_pda     =  32 @ 280
//   [u8;32]  metadata_pointer_authority_pda       =  32 @ 312
//   [u8;32]  metadata_update_authority_pda        =  32 @ 344
//   [u8;32]  pause_authority_pda                  =  32 @ 376
//   [u8;32]  confidential_transfer_authority_pda  =  32 @ 408
//   [u8;32]  default_account_state_authority_pda  =  32 @ 440
//   [u8;32]  hook_program_id                      =  32 @ 472
//   [u8;32]  permanent_delegate                   =  32 @ 504
//   [u8;32]  metadata_address                     =  32 @ 536
//   u64      last_configured_at_slot              =   8 @ 568
//   [u8;32]  configured_by                        =  32 @ 576
//   u64      extension_observed_at_slot           =   8 @ 608
//   u64      max_extension_observation_age_slots  =   8 @ 616
//   [u8;32]  _reserved                            =  32 @ 624
//                                                  ─────
//                                                   656 bytes
pub const ISSUED_TOKEN_CONTROL_SIZE: usize = 656;

// ─── State ────────────────────────────────────────────────────────────────────

/// Isolates Token-2022 extension capability from the root protocol config.
/// Preserves future control surfaces without bloating ChanceryConfig.
///
/// Seed: [b"issued-token-control"]
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct IssuedTokenControl {
    pub discriminator:                       [u8; 8],
    pub version:                             u16,
    pub bump:                                u8,
    pub _pad0:                               [u8; 5],

    /// The issued Token-2022 mint this control record governs.
    pub issued_token_mint:                   Pubkey,

    /// The token program (Token-2022) owning the issued mint.
    pub issued_token_program:                Pubkey,

    /// Bitmask of mint-level extensions reserved at deployment.
    /// Split into [u64; 2] (low, high) to avoid u128 alignment on BPF.
    pub reserved_mint_extension_mask:        [u64; 2],

    /// Bitmask of mint-level extensions observed at the most recent
    /// `verify_issued_token_deployment` pass. Pair with
    /// `extension_observed_at_slot` for staleness checks.
    pub active_mint_extension_mask:          [u64; 2],

    /// Bitmask of account-level extensions reserved at deployment.
    pub reserved_account_extension_mask:     [u64; 2],

    /// Bitmask of account-level extensions currently active.
    pub active_account_extension_mask:       [u64; 2],

    /// General control flags for extension management.
    pub control_flags:                       u64,

    // ── Authority PDAs ────────────────────────────────────────────────────
    pub mint_authority_pda:                  Pubkey,
    pub freeze_authority_pda:                Pubkey,
    pub close_mint_authority_pda:            Pubkey,
    pub transfer_hook_authority_pda:         Pubkey,
    pub permanent_delegate_authority_pda:    Pubkey,
    pub metadata_pointer_authority_pda:      Pubkey,
    pub metadata_update_authority_pda:       Pubkey,
    pub pause_authority_pda:                 Pubkey,
    pub confidential_transfer_authority_pda: Pubkey,
    pub default_account_state_authority_pda: Pubkey,

    // ── Current extension state ───────────────────────────────────────────
    /// The program ID currently set for the transfer hook (zero if inactive).
    pub hook_program_id:                     Pubkey,

    /// The current permanent delegate address (zero if inactive).
    pub permanent_delegate:                  Pubkey,

    /// The current metadata address (zero if inactive).
    pub metadata_address:                    Pubkey,


    /// Slot when this control record was last configured.
    pub last_configured_at_slot:             u64,

    /// The authority that last configured this record.
    pub configured_by:                       Pubkey,

    /// Slot at which `active_mint_extension_mask` was last observed via
    /// `verify_issued_token_deployment`.
    pub extension_observed_at_slot:          u64,

    /// Max age in slots before the observation is stale at settlement. Zero is
    /// the deliberate no-age-ceiling sentinel: future observations remain
    /// invalid and extension-mask policy remains enforced. This value is set at
    /// initialization in the current layout; future deployments may select a
    /// finite SLA, while changing an existing record requires a versioned path.
    pub max_extension_observation_age_slots: u64,

    pub _reserved:                           [u8; 32],
}

// ─── Helpers ──────────────────────────────────────────────────────────────────
impl IssuedTokenControl {
    pub fn pda(program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[seeds::ISSUED_TOKEN_CONTROL],
            program_id,
        )
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
            ISSUED_TOKEN_CONTROL_SIZE,
            ISSUED_TOKEN_CONTROL_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        if state.control_flags & status_flag::INITIALIZED == 0 {
            return Err(ChanceryError::NotInitialized.into());
        }

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
            ISSUED_TOKEN_CONTROL_SIZE,
            ISSUED_TOKEN_CONTROL_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        if state.control_flags & status_flag::INITIALIZED == 0 {
            return Err(ChanceryError::NotInitialized.into());
        }

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
            ISSUED_TOKEN_CONTROL_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    /// Returns true if a given mint extension bit is reserved.
    #[inline]
    pub fn is_mint_extension_reserved(&self, bit: u32) -> bool {
        if bit >= 128 {
            return false;
        }

        let word  = if bit < 64 { self.reserved_mint_extension_mask[0] } else { self.reserved_mint_extension_mask[1] };
        let shift = if bit < 64 { bit } else { bit - 64 };
        word & (1u64 << shift) != 0
    }

    /// Returns true if a given mint extension bit is active.
    #[inline]
    pub fn is_mint_extension_active(&self, bit: u32) -> bool {
        if bit >= 128 {
            return false;
        }

        let word  = if bit < 64 { self.active_mint_extension_mask[0] } else { self.active_mint_extension_mask[1] };
        let shift = if bit < 64 { bit } else { bit - 64 };
        word & (1u64 << shift) != 0
    }

    /// Reject when `control_flags` lacks `READY_FOR_SETTLEMENT`. The bit is set
    /// by `verify_issued_token_deployment` once all pre-required deployment flags
    /// are present.
    pub fn assert_ready_for_settlement(&self) -> Result<(), ProgramError> {
        if self.control_flags & issued_token_deployment_flag::READY_FOR_SETTLEMENT == 0 {
            return Err(ChanceryError::IssuedTokenDeploymentNotVerified.into());
        }

        Ok(())
    }

    /// Reject future observations and observations older than the configured
    /// ceiling. `max_extension_observation_age_slots == 0` disables only the
    /// maximum-age rejection; chronology remains mandatory.
    pub fn assert_extensions_fresh(&self, current_slot: u64) -> Result<(), ProgramError> {
        assert_extension_observation_fresh(
            self.extension_observed_at_slot,
            self.max_extension_observation_age_slots,
            current_slot,
        )
    }

    /// TLV-derived mint mask must be ⊆ `reserved_mint_extension_mask` and
    /// disjoint from the protocol default forbidden set.
    pub fn assert_observed_mint_mask_valid(
        &self,
        observed: [u64; 2],
    ) -> Result<(), ProgramError> {
        if (observed[0] & !self.reserved_mint_extension_mask[0] != 0)
            || (observed[1] & !self.reserved_mint_extension_mask[1] != 0)
        {
            return Err(ChanceryError::ExtensionNotReserved.into());
        }

        if (observed[0] & issued_token_default_forbidden::MINT_FORBIDDEN_LO != 0)
            || (observed[1] & issued_token_default_forbidden::MINT_FORBIDDEN_HI != 0)
        {
            return Err(ChanceryError::IssuedTokenForbiddenDefaultExtensionActive.into());
        }

        Ok(())
    }
}

// ─── Size assertion ───────────────────────────────────────────────────────────
const _: () = assert!(
    core::mem::size_of::<IssuedTokenControl>() == ISSUED_TOKEN_CONTROL_SIZE,
    "IssuedTokenControl size mismatch - update ISSUED_TOKEN_CONTROL_SIZE",
);
