use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::{case_status, seeds},
    error::ChanceryError,
};

// ─── Discriminator ────────────────────────────────────────────────────────────
pub const INSURANCE_CLAIM_NOTICE_DISCRIMINATOR: [u8; 8] =
    [0x69, 0x63, 0x6e, 0x6f, 0x74, 0x63, 0x65, 0x00];

// ─── Account size ─────────────────────────────────────────────────────────────
//   [u8;8]   discriminator          =  8 @ 0
//   u16      version                =  2 @ 8
//   u8       bump                   =  1 @ 10
//   u8       status                 =  1 @ 11
//   [u8;4]   _pad0                  =  4 @ 12  -> [u8;32] at 16
//   [u8;32]  claim_notice_id        = 32 @ 16
//   [u8;32]  insurance_policy_id    = 32 @ 48
//   [u8;32]  related_intent_id      = 32 @ 80
//   [u8;32]  related_pathway_id     = 32 @ 112
//   [u8;32]  event_hash             = 32 @ 144
//   [u8;32]  notice_hash            = 32 @ 176
//   [u8;32]  _reserved              = 32 @ 208
//                                    ─────
//                                    240 bytes
pub const INSURANCE_CLAIM_NOTICE_SIZE: usize = 240;

// ─── State ────────────────────────────────────────────────────────────────────

/// On-chain anchor for a claim notice submitted against an InsurancePolicyPda.
/// Seed: [b"insurance-claim-notice", claim_notice_id]
///
/// Characterization (spec 01 §1.6):
///   - Purely evidentiary - anchors the existence of a claim notice
///     with a hash commitment to off-chain documentation.
///   - Does NOT trigger any on-chain payment or fund movement.
///   - Does NOT create any holder rights.
///   - Status tracks the administrative lifecycle of the notice only.
///
/// The `notice_hash` field is sha256(claim notice document + timestamp),
/// computed off-chain by the operator and anchored here for immutability.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct InsuranceClaimNotice {
    pub discriminator:       [u8; 8],
    pub version:             u16,
    pub bump:                u8,
    
    /// One of `case_status::*` constants.
    pub status:              u8,
    pub _pad0:               [u8; 4],

    pub claim_notice_id:     [u8; 32],
    pub insurance_policy_id: [u8; 32],
    
    /// The settlement intent that triggered the notice. Zero if not applicable.
    pub related_intent_id:   [u8; 32],
    
    /// The pathway under which the triggering event occurred.
    pub related_pathway_id:  [u8; 32],
    
    /// SHA-256 of the on-chain evidence event that triggered this notice.
    pub event_hash:          [u8; 32],
    
    /// SHA-256 of the off-chain claim notice document.
    pub notice_hash:         [u8; 32],

    pub _reserved:           [u8; 32],
}

// ─── Helpers ──────────────────────────────────────────────────────────────────
impl InsuranceClaimNotice {
    pub fn pda(
        claim_notice_id: &[u8; 32],
        program_id:      &Pubkey,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[seeds::INSURANCE_CLAIM_NOTICE, claim_notice_id.as_ref()],
            program_id,
        )
    }

    pub fn verify_pda(
        account:         &AccountInfo,
        claim_notice_id: &[u8; 32],
        program_id:      &Pubkey,
    ) -> Result<u8, ProgramError> {
        let (expected, bump) = Self::pda(claim_notice_id, program_id);

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
            INSURANCE_CLAIM_NOTICE_SIZE,
            INSURANCE_CLAIM_NOTICE_DISCRIMINATOR,
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
        let expected_bump = Self::verify_pda(account, &state.claim_notice_id, &crate::id())?;

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
            INSURANCE_CLAIM_NOTICE_SIZE,
            INSURANCE_CLAIM_NOTICE_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    /// Mutable counterpart to `load_verified`.
    pub fn load_verified_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;
        let expected_bump = Self::verify_pda(account, &state.claim_notice_id, &crate::id())?;

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
            INSURANCE_CLAIM_NOTICE_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    pub fn assert_open(&self) -> Result<(), ProgramError> {
        if self.status != case_status::OPEN {
            return Err(ChanceryError::InsurancePolicyNotFound.into());
        }

        Ok(())
    }
}

// ─── Size assertion ───────────────────────────────────────────────────────────
const _: () = assert!(
    core::mem::size_of::<InsuranceClaimNotice>() == INSURANCE_CLAIM_NOTICE_SIZE,
    "InsuranceClaimNotice size mismatch - update INSURANCE_CLAIM_NOTICE_SIZE",
);
