use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::{reserve_destination_status, seeds},
    error::ChanceryError,
};

// ─── Discriminator ────────────────────────────────────────────────────────────
pub const RESERVE_DESTINATION_DISCRIMINATOR: [u8; 8] =
    [0x72, 0x73, 0x76, 0x64, 0x73, 0x74, 0x00, 0x00];

// ─── Destination flags ────────────────────────────────────────────────────────
pub mod destination_flag {
    /// Destination is an external treasury / ops wallet.
    pub const TREASURY:           u64 = 1 << 0;

    /// Destination is a downstream custodian feed.
    pub const DOWNSTREAM_CUSTODY: u64 = 1 << 1;

    /// Destination is an internal operations account.
    pub const OPERATIONS:         u64 = 1 << 2;

    /// Destination is a recovery or seizure account.
    pub const RECOVERY:           u64 = 1 << 3;

    /// Retired pre-launch compatibility bit. Status is now represented by
    /// `ReserveDestination.status`; new registrations reject this bit.
    pub const DISABLED:           u64 = 1 << 4;

    pub const PURPOSE_MASK: u64 = TREASURY
        | DOWNSTREAM_CUSTODY
        | OPERATIONS
        | RECOVERY;
}

// ─── Account size ─────────────────────────────────────────────────────────────
//   [u8;8]   discriminator               =  8 @ 0
//   u16      version                     =  2 @ 8
//   u8       bump                        =  1 @ 10
//   u8       status                      =  1 @ 11
//   [u8;4]   _pad0                       =  4 @ 12  -> Pubkey at 16 (align 1)
//   [u8;32]  asset_mint                  = 32 @ 16
//   [u8;32]  destination_token_account   = 32 @ 48
//   [u8;32]  destination_owner           = 32 @ 80
//   u64      destination_flags           =  8 @ 112  (112%8=0 Y)
//   [u8;32]  approved_by                 = 32 @ 120
//   [u8;32]  withdrawal_limit_policy_id  = 32 @ 152
//   [u8;32]  _reserved                   = 32 @ 184
//                                         ─────
//                                         216 bytes
pub const RESERVE_DESTINATION_RESERVED_SIZE: usize = 32;
pub const RESERVE_DESTINATION_SIZE:          usize = 216;

// ─── State ────────────────────────────────────────────────────────────────────

/// Approved destination for reserve outflows.
/// Seed: [b"reserve-destination", asset_mint, destination_token_account]
///
/// All reserve withdrawals must target an approved destination PDA.
/// Vec-based allowlists from the legacy program are modeled as
/// individual purpose-specific PDAs.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ReserveDestination {
    pub discriminator:             [u8; 8],
    pub version:                   u16,
    pub bump:                      u8,

    /// One of `reserve_destination_status::*`.
    pub status:                    u8,
    pub _pad0:                     [u8; 4],

    /// Collateral asset mint this destination is approved for.
    pub asset_mint:                Pubkey,

    /// The token account that receives reserve outflows.
    pub destination_token_account: Pubkey,

    /// Owner of the destination token account.
    pub destination_owner:         Pubkey,

    /// Bitfield using `destination_flag::*` constants.
    pub destination_flags:         u64,

    /// The authority recorded for the latest registration/status transition.
    pub approved_by:               Pubkey,

    /// Mandatory DESTINATION-scoped policy used by every reserve withdrawal.
    /// The policy must enforce both a non-zero per-transaction maximum and a
    /// non-zero fixed UTC-day maximum.
    pub withdrawal_limit_policy_id: [u8; 32],

    /// Append-only layout headroom. Must remain zero until consumed by a
    /// versioned layout change, and must be replenished in that same change.
    pub _reserved:                  [u8; 32],
}

// ─── Helpers ──────────────────────────────────────────────────────────────────
impl ReserveDestination {
    pub fn pda(
        asset_mint:                &Pubkey,
        destination_token_account: &Pubkey,
        program_id:                &Pubkey,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                seeds::RESERVE_DESTINATION,
                asset_mint.as_ref(),
                destination_token_account.as_ref(),
            ],
            program_id,
        )
    }

    pub fn verify_pda(
        account:                   &AccountInfo,
        asset_mint:                &Pubkey,
        destination_token_account: &Pubkey,
        program_id:                &Pubkey,
    ) -> Result<u8, ProgramError> {
        let (expected, bump) =
            Self::pda(asset_mint, destination_token_account, program_id);

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
            RESERVE_DESTINATION_SIZE,
            RESERVE_DESTINATION_DISCRIMINATOR,
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
        let expected_bump = Self::verify_pda(account, &state.asset_mint, &state.destination_token_account, &crate::id())?;

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
            RESERVE_DESTINATION_SIZE,
            RESERVE_DESTINATION_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    pub(crate) fn load_mut_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        asset_mint: &Pubkey,
        destination_token_account: &Pubkey,
        expected_bump: u8,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;

        if state.asset_mint != *asset_mint
            || state.destination_token_account != *destination_token_account
        {
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
        let expected_bump = Self::verify_pda(account, &state.asset_mint, &state.destination_token_account, &crate::id())?;

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
            RESERVE_DESTINATION_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    // ── Guards ────────────────────────────────────────────────────────────────

    pub fn validate_status(status: u8) -> Result<(), ProgramError> {
        match status {
            reserve_destination_status::DISABLED
            | reserve_destination_status::ENABLED
            | reserve_destination_status::DEPRECATED => Ok(()),
            _ => Err(ChanceryError::ReserveDestinationNotApproved.into()),
        }
    }

    pub fn validate_purpose_flags(flags: u64) -> Result<(), ProgramError> {
        if flags & !destination_flag::PURPOSE_MASK != 0 {
            return Err(ChanceryError::ReserveDestinationUnknownFlag.into());
        }

        let purposes = flags & destination_flag::PURPOSE_MASK;
        if purposes == 0 {
            return Err(ChanceryError::ReserveDestinationPurposeRequired.into());
        }

        if purposes.count_ones() != 1 {
            return Err(ChanceryError::ReserveDestinationPurposeAmbiguous.into());
        }

        Ok(())
    }

    pub fn assert_enabled(&self) -> Result<(), ProgramError> {
        match self.status {
            reserve_destination_status::ENABLED => Ok(()),
            reserve_destination_status::DISABLED => {
                Err(ChanceryError::ReserveDestinationDisabled.into())
            }
            reserve_destination_status::DEPRECATED => {
                Err(ChanceryError::ReserveDestinationDeprecated.into())
            }
            _ => Err(ChanceryError::ReserveDestinationNotApproved.into()),
        }
    }

    pub fn assert_asset(&self, asset_mint: &Pubkey) -> Result<(), ProgramError> {
        if &self.asset_mint != asset_mint {
            return Err(ChanceryError::ReserveDestinationAssetMismatch.into());
        }

        Ok(())
    }

    pub fn assert_destination_account(
        &self,
        token_account: &Pubkey,
    ) -> Result<(), ProgramError> {
        if &self.destination_token_account != token_account {
            return Err(ChanceryError::ReserveDestinationNotApproved.into());
        }

        Ok(())
    }
}

// ─── Size assertion ───────────────────────────────────────────────────────────
const _: () = assert!(
    core::mem::size_of::<ReserveDestination>() == RESERVE_DESTINATION_SIZE,
    "ReserveDestination size mismatch - update RESERVE_DESTINATION_SIZE",
);
