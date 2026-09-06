use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::{intent_status, seeds},
    error::ChanceryError,
};

// ─── Discriminator ────────────────────────────────────────────────────────────
pub const SETTLEMENT_INTENT_DISCRIMINATOR: [u8; 8] =
    [0x73, 0x74, 0x6c, 0x69, 0x6e, 0x74, 0x00, 0x00];

// ─── Account size ─────────────────────────────────────────────────────────────
//   [u8;8]   discriminator               =  8 @ 0
//   u16      version                     =  2 @ 8
//   u8       bump                        =  1 @ 10
//   u8       status                      =  1 @ 11
//   u8       settlement_mode             =  1 @ 12
//   u8       settlement_action           =  1 @ 13
//   [u8;2]   _pad0                       =  2 @ 14  -> [u8;32] at 16 (align 1, fine)
//   [u8;32]  intent_id                   = 32 @ 16
//   [u8;32]  principal_a                 = 32 @ 48
//   [u8;32]  principal_b                 = 32 @ 80
//   [u8;32]  executor                    = 32 @ 112
//   [u8;32]  asset_mint                  = 32 @ 144
//   [u8;32]  issued_token_mint           = 32 @ 176
//   u64      asset_amount                =  8 @ 208  (208%8=0 Y)
//   u64      issued_token_amount         =  8 @ 216
//   u64      minimum_asset_amount        =  8 @ 224
//   u64      minimum_issued_token_amount =  8 @ 232
//   u64      nonce                       =  8 @ 240
//   i64      valid_after_unix_timestamp  =  8 @ 248
//   i64      expires_at_unix_timestamp   =  8 @ 256
//   [u8;32]  policy_id                   = 32 @ 264
//   [u8;32]  intent_hash                 = 32 @ 296
//   [u8;32]  pathway_id                  = 32 @ 328
//   [u8;32]  rent_refund_recipient       = 32 @ 360
//   [u8;32]  _reserved                   = 32 @ 392
//                                         ─────
//                                         424 bytes
pub const SETTLEMENT_INTENT_RESERVED_SIZE: usize = 32;
pub const SETTLEMENT_INTENT_SIZE:          usize = 424;

// ─── State ────────────────────────────────────────────────────────────────────

/// Pre-committed settlement intent - created before execution and consumed
/// atomically on settlement.
/// Seed: [b"settlement-intent", intent_id]
///
/// At most one live account may exist for an exact content-addressed intent.
/// Terminal paths close the PDA without retaining a permanent history record.
/// The same terms may be recreated only when the exact historical address is
/// fresh and available; callers should use a fresh nonce for each new lifecycle.
/// The nonce provides caller-controlled separation without a globally fetched
/// sequence (spec 14 §14.3.1).
/// `intent_id == intent_hash == sha256(domain_separator || canonical fields)` -
/// content-addressed; the PDA namespace is squat-resistant by construction.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SettlementIntent {
    pub discriminator:               [u8; 8],
    pub version:                     u16,
    pub bump:                        u8,
    
    /// One of `intent_status::*` constants.
    pub status:                      u8,
    
    /// One of `settlement_mode::*` constants.
    pub settlement_mode:             u8,

    /// One of `settlement_action::*` constants. Committed into `intent_hash`
    /// so the same intent cannot be consumed through both directions.
    pub settlement_action:           u8,
    pub _pad0:                       [u8; 2],

    /// Content-addressed intent id; equals `intent_hash` at creation.
    pub intent_id:                   [u8; 32],
    
    /// Depositing / minting principal.
    pub principal_a:                 Pubkey,
    
    /// Second trilateral principal. Delegated intents must store principal A
    /// here as well; delegated settlement has one principal and one executor.
    pub principal_b:                 Pubkey,
    
    /// Executor coordinating the transaction (may be `Pubkey::default()`
    /// for direct settlement).
    pub executor:                    Pubkey,
    
    /// Collateral asset mint.
    pub asset_mint:                  Pubkey,
    
    /// Issued issued token mint.
    pub issued_token_mint:           Pubkey,

    /// Gross asset amount to be deposited (mint) or redeemed (redeem).
    pub asset_amount:                u64,
    
    /// Gross issued-token amount counterpart.
    pub issued_token_amount:         u64,
    
    /// Minimum acceptable asset output (slippage floor for redeem).
    pub minimum_asset_amount:        u64,
    
    /// Minimum acceptable issued-token output (slippage floor for mint).
    pub minimum_issued_token_amount: u64,
    
    /// Caller-chosen nonce committed into the content-addressed intent hash.
    /// Multiple live intents may use the same nonce because the complete hash
    /// is the identity. A fresh nonce is recommended for every new lifecycle;
    /// reuse of an identical historical intent is best-effort and requires its
    /// exact PDA to be fresh and available.
    pub nonce:                       u64,
    
    /// Intent is invalid before this timestamp.
    pub valid_after_unix_timestamp:  i64,
    
    /// Intent expires at this timestamp.
    pub expires_at_unix_timestamp:   i64,

    /// Settlement policy ID this intent was created under.
    pub policy_id:                   [u8; 32],
    
    /// Hash of all economic terms, used for off-chain signature binding.
    /// Computed as: sha256(domain_separator || intent fields).
    pub intent_hash:                 [u8; 32],

    /// Pathway this intent is authorized for. This field consumed the former
    /// tail; the layout now replenishes append headroom immediately after it.
    pub pathway_id:                  [u8; 32],

    /// Rent refund target recorded at creation (the create payer). Terminal
    /// paths (execution, expiry sweep) close the account and refund here and
    /// only here - cleanup can never redirect rent.
    pub rent_refund_recipient:       Pubkey,

    /// Append-only layout headroom. Must remain zero until consumed by a
    /// layout change, and must be replenished in that same change.
    pub _reserved:                   [u8; 32],
}

// ─── Helpers ──────────────────────────────────────────────────────────────────
impl SettlementIntent {
    pub fn pda(intent_id: &[u8; 32], program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[seeds::SETTLEMENT_INTENT, intent_id.as_ref()],
            program_id,
        )
    }

    pub fn verify_pda(
        account   : &AccountInfo,
        intent_id : &[u8; 32],
        program_id: &Pubkey,
    ) -> Result<u8, ProgramError> {
        let (expected, bump) = Self::pda(intent_id, program_id);

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
            SETTLEMENT_INTENT_SIZE,
            SETTLEMENT_INTENT_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    pub(crate) fn load_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        intent_id: &[u8; 32],
        expected_bump: u8,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;

        if state.intent_id != *intent_id {
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
        let expected_bump = Self::verify_pda(account, &state.intent_id, &crate::id())?;

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
            SETTLEMENT_INTENT_SIZE,
            SETTLEMENT_INTENT_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    /// Mutable counterpart to `load_verified`.
    pub fn load_verified_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;
        let expected_bump = Self::verify_pda(account, &state.intent_id, &crate::id())?;

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
            SETTLEMENT_INTENT_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    // ── Status guards ─────────────────────────────────────────────────────────

    pub fn assert_pending(&self) -> Result<(), ProgramError> {
        match self.status {
            s if s == intent_status::PENDING   => Ok(()),
            s if s == intent_status::EXPIRED   => Err(ChanceryError::IntentExpired.into()),
            s if s == intent_status::EXECUTED
                || s == intent_status::CANCELLED => {
                    Err(ChanceryError::IntentAlreadyExecuted.into())
                }
            _ => Err(ChanceryError::IntentNotFound.into()),
        }
    }

    pub fn assert_not_expired(&self, now: i64) -> Result<(), ProgramError> {
        if self.expires_at_unix_timestamp != 0 && now >= self.expires_at_unix_timestamp {
            return Err(ChanceryError::IntentExpired.into());
        }

        Ok(())
    }

    pub fn assert_valid_after(&self, now: i64) -> Result<(), ProgramError> {
        if self.valid_after_unix_timestamp != 0 && now < self.valid_after_unix_timestamp {
            return Err(ChanceryError::IntentNotYetValid.into());
        }

        Ok(())
    }

    /// Combined temporal gate: valid_after <= now < expires_at.
    pub fn assert_temporally_valid(&self, now: i64) -> Result<(), ProgramError> {
        self.assert_valid_after(now)?;
        self.assert_not_expired(now)?;

        Ok(())
    }

    /// Reject consumption of an intent created for a different settlement
    /// mode (e.g. a trilateral intent being consumed via the delegated
    /// handler, which would skip principal_b's signature).
    pub fn assert_settlement_mode(&self, expected: u8) -> Result<(), ProgramError> {
        if self.settlement_mode != expected {
            return Err(ChanceryError::SettlementModeNotAllowed.into());
        }

        Ok(())
    }

    /// Prove that the stored economic terms unambiguously describe a mint.
    ///
    /// An intent commits to one settlement direction through
    /// `settlement_action`: exactly one gross input side is populated, and the
    /// minimum on the unused output side must be zero. This prevents a caller
    /// from creating one content-addressed intent and choosing mint or redeem
    /// only at execution time.
    pub fn assert_mint_terms(&self) -> Result<(), ProgramError> {
        if self.settlement_action != crate::constants::settlement_action::MINT
            || self.asset_amount == 0
            || self.issued_token_amount != 0
            || self.minimum_asset_amount != 0
        {
            return Err(ChanceryError::IntentInvalidParameters.into());
        }

        Ok(())
    }

    /// Prove that the stored economic terms unambiguously describe a redeem.
    pub fn assert_redeem_terms(&self) -> Result<(), ProgramError> {
        if self.settlement_action != crate::constants::settlement_action::REDEEM
            || self.issued_token_amount == 0
            || self.asset_amount != 0
            || self.minimum_issued_token_amount != 0
        {
            return Err(ChanceryError::IntentInvalidParameters.into());
        }

        Ok(())
    }

    // `mark_executed` / `mark_expired` retired pre-deployment: terminal
    // transitions close the account (rent to `rent_refund_recipient`) instead
    // of flipping status, so the live set is exactly the pending intents.
    // `intent_status::EXECUTED/EXPIRED/CANCELLED` remain pinned for wire
    // stability of the status vocabulary in evidence.
}

// ─── Size assertion ───────────────────────────────────────────────────────────
const _: () = assert!(
    core::mem::size_of::<SettlementIntent>() == SETTLEMENT_INTENT_SIZE,
    "SettlementIntent size mismatch - update SETTLEMENT_INTENT_SIZE",
);
