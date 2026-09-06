use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::seeds,
    error::ChanceryError,
};

// ─── Discriminator ────────────────────────────────────────────────────────────
pub const INSURANCE_POLICY_DISCRIMINATOR: [u8; 8] =
    [0x69, 0x6e, 0x73, 0x70, 0x6f, 0x6c, 0x79, 0x00];

// ─── Account size ─────────────────────────────────────────────────────────────
//   [u8;8]   discriminator              =  8 @ 0
//   u16      version                    =  2 @ 8
//   u8       bump                       =  1 @ 10
//   u8       coverage_kind              =  1 @ 11
//   u8       beneficiary_scope_kind     =  1 @ 12
//   [u8;3]   _pad0                      =  3 @ 13  -> [u8;32] at 16
//   [u8;32]  insurance_policy_id        = 32 @ 16
//   [u8;32]  provider_identity_hash     = 32 @ 48
//   [u8;32]  beneficiary_scope_key      = 32 @ 80
//   [u8;32]  covered_pathway_id         = 32 @ 112
//   [u8;32]  covered_asset_mint         = 32 @ 144
//   u64      coverage_limit             =  8 @ 176  (176%8=0 Y)
//   u64      deductible                 =  8 @ 184
//   i64      effective_from             =  8 @ 192
//   i64      effective_until            =  8 @ 200
//   [u8;32]  claim_notice_policy_hash   = 32 @ 208
//   u64      status_flags               =  8 @ 240
//   [u8;32]  _reserved                  = 32 @ 248
//                                        ─────
//                                        280 bytes
pub const INSURANCE_POLICY_SIZE: usize = 280;

// ─── State ────────────────────────────────────────────────────────────────────

/// Reference record anchoring an external insurance policy to a chancery pathway.
/// Seed: [b"insurance-policy", insurance_policy_id]
///
/// Characterization (spec 01 §1.6, spec 05 §5.12):
///   - This is a REFERENCE to an external commercial insurance arrangement.
///   - It does NOT create token-native holder rights.
///   - It does NOT imply any on-chain insurance payout mechanism.
///   - It does NOT create a yield or return promise.
///   - Presence in a pathway policy is purely documentary/evidentiary.
///
/// The `coverage_limit` and `deductible` fields are informational and
/// are included in evidence events for reconciliation - they do not
/// trigger any on-chain economic action.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct InsurancePolicy {
    pub discriminator:                  [u8; 8],
    pub version:                        u16,
    pub bump:                           u8,
    
    /// Category of coverage (operator-defined; e.g. operational, counterparty).
    pub coverage_kind:                  u8,
    
    /// One of `scope::*` constants.
    pub beneficiary_scope_kind:         u8,
    pub _pad0:                          [u8; 3],

    pub insurance_policy_id:            [u8; 32],
    
    /// SHA-256 of the off-chain provider identity document.
    pub provider_identity_hash:         [u8; 32],
    
    /// The beneficiary - asset mint, counterparty key, etc. per scope kind.
    pub beneficiary_scope_key:          Pubkey,
    
    /// The pathway this policy covers. Zero = covers all pathways.
    pub covered_pathway_id:             [u8; 32],
    
    /// The asset this policy covers. Zero = covers all registered assets.
    pub covered_asset_mint:             Pubkey,

    /// Informational coverage limit (not enforced on-chain).
    pub coverage_limit:                 u64,
    
    /// Informational deductible amount.
    pub deductible:                     u64,
    
    /// Unix timestamp: policy not yet effective before this.
    pub effective_from_unix_timestamp:  i64,
    
    /// Unix timestamp: policy expired after this. 0 = no expiry.
    pub effective_until_unix_timestamp: i64,

    /// SHA-256 of the claim notice procedure / contact document.
    pub claim_notice_policy_hash:       [u8; 32],
    
    /// Status flags (active, suspended, etc.).
    pub status_flags:                   u64,

    pub _reserved:                      [u8; 32],
}

// ─── Helpers ──────────────────────────────────────────────────────────────────
impl InsurancePolicy {
    pub fn pda(
        insurance_policy_id: &[u8; 32],
        program_id:          &Pubkey,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[seeds::INSURANCE_POLICY, insurance_policy_id.as_ref()],
            program_id,
        )
    }

    pub fn verify_pda(
        account:             &AccountInfo,
        insurance_policy_id: &[u8; 32],
        program_id:          &Pubkey,
    ) -> Result<u8, ProgramError> {
        let (expected, bump) = Self::pda(insurance_policy_id, program_id);

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
            INSURANCE_POLICY_SIZE,
            INSURANCE_POLICY_DISCRIMINATOR,
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
        let expected_bump = Self::verify_pda(account, &state.insurance_policy_id, &crate::id())?;

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
            INSURANCE_POLICY_SIZE,
            INSURANCE_POLICY_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    /// Mutable counterpart to `load_verified`.
    pub fn load_verified_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;
        let expected_bump = Self::verify_pda(account, &state.insurance_policy_id, &crate::id())?;

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
            INSURANCE_POLICY_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    pub fn assert_effective(&self, now: i64) -> Result<(), ProgramError> {
        if self.effective_from_unix_timestamp != 0 && now < self.effective_from_unix_timestamp {
            return Err(ChanceryError::InsurancePolicyNotFound.into());
        }

        if self.effective_until_unix_timestamp != 0 && now >= self.effective_until_unix_timestamp {
            return Err(ChanceryError::InsurancePolicyExpired.into());
        }

        Ok(())
    }
}

// ─── Size assertion ───────────────────────────────────────────────────────────
const _: () = assert!(
    core::mem::size_of::<InsurancePolicy>() == INSURANCE_POLICY_SIZE,
    "InsurancePolicy size mismatch - update INSURANCE_POLICY_SIZE",
);
