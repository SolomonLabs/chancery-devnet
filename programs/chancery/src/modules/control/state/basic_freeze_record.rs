//! Basic freeze record.
//!
//! One PDA per issued-token account. Tracks freeze/thaw status, who acted,
//! when, and the operator-supplied reason code.
//!
//! Per
//!   - Hard-fail `reason_code == 0` at handler (zero = "no reason given")
//!   - status_flags=0 in events for MVP

use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::seeds,
    error::ChanceryError,
};

pub const BASIC_FREEZE_RECORD_DISCRIMINATOR: [u8; 8] =
    [0x62, 0x66, 0x72, 0x65, 0x65, 0x7a, 0x65, 0x01];  // "bfreeze\x01"

//   [u8;8]   discriminator                =  8 @ 0
//   u16      version                      =  2 @ 8
//   u8       bump                         =  1 @ 10
//   u8       status                       =  1 @ 11
//   u32      reason_code                  =  4 @ 12
//   [u8;32]  issued_token_account         = 32 @ 16
//   [u8;32]  issued_token_mint            = 32 @ 48
//   [u8;32]  freeze_authority_pda         = 32 @ 80
//   [u8;32]  frozen_by                    = 32 @ 112
//   [u8;32]  thawed_by                    = 32 @ 144
//   u64      frozen_at_slot               =  8 @ 176
//   u64      thawed_at_slot               =  8 @ 184
//   u64      last_event_sequence_nonce    =  8 @ 192
//   u64      freeze_flags                 =  8 @ 200
//   u64      thaw_flags                   =  8 @ 208
//   [u8;32]  rent_refund_recipient        = 32 @ 216
//   [u8;24]  _reserved                    = 24 @ 248
//                                          ─────
//                                          272 bytes
pub const BASIC_FREEZE_RECORD_SIZE: usize = 272;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct BasicFreezeRecord {
    pub discriminator:             [u8; 8],
    pub version:                   u16,
    pub bump:                      u8,
    pub status:                    u8,
    pub reason_code:               u32,
    pub issued_token_account:      Pubkey,
    pub issued_token_mint:         Pubkey,
    pub freeze_authority_pda:      Pubkey,
    pub frozen_by:                 Pubkey,
    pub thawed_by:                 Pubkey,
    pub frozen_at_slot:            u64,
    pub thawed_at_slot:            u64,
    pub last_event_sequence_nonce: u64,
    pub freeze_flags:              u64,
    pub thaw_flags:                u64,

    /// Rent refund target recorded at first freeze (the freeze payer). Thaw
    /// closes the record and refunds here and only here; a re-freeze
    /// recreates the record with the new payer.
    pub rent_refund_recipient:     Pubkey,

    /// Append-only layout headroom. Must remain zero until consumed by a
    /// layout change, and must be replenished in that same change.
    pub _reserved:                 [u8; 24],
}

impl BasicFreezeRecord {
    pub fn pda(token_account: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[seeds::BASIC_FREEZE_RECORD, token_account.as_ref()],
            program_id,
        )
    }

    pub fn verify_pda(
        account:       &AccountInfo,
        token_account: &Pubkey,
        program_id:    &Pubkey,
    ) -> Result<u8, ProgramError> {
        let (expected, bump) = Self::pda(token_account, program_id);

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
            BASIC_FREEZE_RECORD_SIZE,
            BASIC_FREEZE_RECORD_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    pub(crate) fn load_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        token_account: &Pubkey,
        expected_bump: u8,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;

        if state.issued_token_account != *token_account {
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
        let expected_bump = Self::verify_pda(account, &state.issued_token_account, &crate::id())?;

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
            BASIC_FREEZE_RECORD_SIZE,
            BASIC_FREEZE_RECORD_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    pub(crate) fn load_mut_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        token_account: &Pubkey,
        expected_bump: u8,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;

        if state.issued_token_account != *token_account {
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
        let expected_bump = Self::verify_pda(account, &state.issued_token_account, &crate::id())?;

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
            BASIC_FREEZE_RECORD_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }
}

const _: () = assert!(
    core::mem::size_of::<BasicFreezeRecord>() == BASIC_FREEZE_RECORD_SIZE,
    "BasicFreezeRecord size mismatch - update BASIC_FREEZE_RECORD_SIZE",
);
