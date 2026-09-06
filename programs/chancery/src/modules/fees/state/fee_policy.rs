use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    account_security::assert_external_identity,
    constants::{fee_recipient, rounding, seeds},
    error::ChanceryError,
};

// ─── Discriminator ────────────────────────────────────────────────────────────
pub const FEE_POLICY_DISCRIMINATOR: [u8; 8] =
    [0x66, 0x65, 0x65, 0x70, 0x6f, 0x6c, 0x79, 0x00];

// ─── Account size ─────────────────────────────────────────────────────────────
//   [u8;8]   discriminator              =  8 @ 0
//   u16      version                    =  2 @ 8
//   u8       bump                       =  1 @ 10
//   u8       fee_recipient_policy       =  1 @ 11
//   u8       rounding_mode              =  1 @ 12
//   u8       net_fee_floor_zero         =  1 @ 13  (bool as u8)
//   [u8;2]   _pad0                      =  2 @ 14  -> [u8;32] at 16
//   [u8;32]  fee_policy_id              = 32 @ 16
//   u64      fee_policy_flags           =  8 @ 48  (48%8=0 Y)
//   u64      flat_fee_in_asset          =  8 @ 56
//   u64      flat_fee_in_issued_token   =  8 @ 64
//   u32      percent_fee_basis points   =  4 @ 72
//   [u8;4]   _pad1                      =  4 @ 76  -> u64 at 80
//   u64      fee_cap_amount             =  8 @ 80
//   u64      minimum_fee_amount         =  8 @ 88
//   u64      rebate_flat_amount         =  8 @ 96
//   u32      rebate_basis points        =  4 @ 104
//   [u8;4]   _pad2                      =  4 @ 108 -> u64 at 112
//   u64      rebate_cap_amount          =  8 @ 112
//   [u8;32]  fee_recipient_key          = 32 @ 120
//   i64      effective_from             =  8 @ 152  (152%8=0 Y)
//   i64      effective_until            =  8 @ 160
//   [u8;32]  _reserved                  = 32 @ 168
//                                        ─────
//                                        200 bytes
pub const FEE_POLICY_SIZE: usize = 200;

// ─── fee_policy_flags bits ────────────────────────────────────────────────────
pub mod fee_flag {
    /// Fee is denominated in asset (collateral), not issued token.
    pub const FEE_IN_ASSET:        u64 = 1 << 0;
    
    /// Fee is denominated in issued token.
    pub const FEE_IN_ISSUED_TOKEN: u64 = 1 << 1;
    
    /// Percent fee is applied to gross input (not gross output).
    pub const PCT_ON_INPUT:        u64 = 1 << 2;
    
    /// Rebate applies to principal-side recipient only.
    pub const REBATE_TO_PRINCIPAL: u64 = 1 << 3;
    
    /// Policy is currently active.
    pub const ACTIVE:              u64 = 1 << 4;
}

/// Every fee flag understood by this program version. Unknown bits are
/// rejected at registration and update so an upgrade cannot silently turn a
/// previously inert bit into economic authority.
pub const KNOWN_FEE_FLAGS: u64 = fee_flag::FEE_IN_ASSET
    | fee_flag::FEE_IN_ISSUED_TOKEN
    | fee_flag::PCT_ON_INPUT
    | fee_flag::REBATE_TO_PRINCIPAL
    | fee_flag::ACTIVE;

/// Flags with transfer-path semantics in this binary. `PCT_ON_INPUT` is kept
/// reserved for wire compatibility but is intentionally not persistable: when
/// input and output mints or rates differ, subtracting an input-denominated
/// percentage from output units is dimensionally invalid without an additional
/// input-side fee leg.
pub const SUPPORTED_FEE_FLAGS: u64 = KNOWN_FEE_FLAGS & !fee_flag::PCT_ON_INPUT;

/// Complete fee result used by settlement, evidence, and conservation tests.
/// `nominal_rebate` is the configured rebate before the optional zero floor;
/// `effective_rebate` is the amount actually credited to the principal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FeeComputation {
    pub assessed_fee:     u64,
    pub nominal_rebate:   u64,
    pub effective_rebate: u64,
    pub net_fee:          u64,
    pub net_output:       u64,
}

// ─── State ────────────────────────────────────────────────────────────────────

/// Modular commercial fee/rebate layer decoupled from exchange rates.
/// Seed: [b"fee-policy", fee_policy_id]
///
/// Application order (spec §4.4):
///   1. gross output from rate
///   2. compute fee (flat or basis points, whichever applies)
///   3. apply cap and floor
///   4. compute rebate
///   5. apply rebate cap
///   6. net = gross - fee + rebate
///   7. enforce net_fee_floor_zero
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct FeePolicy {
    pub discriminator:                  [u8; 8],
    pub version:                        u16,
    pub bump:                           u8,
    
    /// One of `fee_recipient::*` constants.
    pub fee_recipient_policy:           u8,
    
    /// One of `rounding::*` constants for percentage fee assessment.
    /// Percentage rebates are always floored.
    pub rounding_mode:                  u8,
    
    /// If non-zero, net fee is floored at zero (rebate cannot exceed fee).
    pub net_fee_floor_zero:             u8,
    pub _pad0:                          [u8; 2],

    pub fee_policy_id:                  [u8; 32],
    
    /// Bitfield using `fee_flag::*` constants.
    pub fee_policy_flags:               u64,

    /// Fixed fee in units of collateral asset. 0 = disabled.
    pub flat_fee_in_asset:              u64,
    
    /// Fixed fee in units of issued token. 0 = disabled.
    pub flat_fee_in_issued_token:       u64,
    
    /// Percentage fee in basis points. 0 = disabled.
    pub percent_fee_bps:                u32,
    pub _pad1:                          [u8; 4],
    
    /// Maximum fee regardless of percent calculation. 0 = no cap.
    pub fee_cap_amount:                 u64,
    
    /// Minimum fee enforced after percent calculation. 0 = no minimum.
    pub minimum_fee_amount:             u64,

    /// Flat rebate in same denomination as fee. 0 = disabled.
    pub rebate_flat_amount:             u64,
    
    /// Rebate in basis points of fee charged. 0 = disabled.
    pub rebate_bps:                     u32,
    pub _pad2:                          [u8; 4],
    
    /// Maximum rebate regardless of basis points calculation. 0 = no cap.
    pub rebate_cap_amount:              u64,

    /// Account that receives the fee. Interpretation depends on
    /// `fee_recipient_policy`.
    pub fee_recipient_key:              Pubkey,

    /// Unix timestamp before which this policy is not yet effective. 0 = immediate.
    pub effective_from_unix_timestamp:  i64,
    
    /// Unix timestamp after which this policy is no longer effective. 0 = no expiry.
    pub effective_until_unix_timestamp: i64,

    pub _reserved:                      [u8; 32],
}

// ─── Helpers ──────────────────────────────────────────────────────────────────
impl FeePolicy {
    /// Canonical semantic payload for config-change hashing. Account headers,
    /// padding, and reserved layout headroom are deliberately excluded.
    pub(crate) fn config_change_payload(&self) -> Vec<u8> {
        let mut payload = Vec::with_capacity(147);
        payload.push(self.fee_recipient_policy);
        payload.push(self.rounding_mode);
        payload.push(self.net_fee_floor_zero);
        payload.extend_from_slice(&self.fee_policy_id);
        payload.extend_from_slice(&self.fee_policy_flags.to_le_bytes());
        payload.extend_from_slice(&self.flat_fee_in_asset.to_le_bytes());
        payload.extend_from_slice(&self.flat_fee_in_issued_token.to_le_bytes());
        payload.extend_from_slice(&self.percent_fee_bps.to_le_bytes());
        payload.extend_from_slice(&self.fee_cap_amount.to_le_bytes());
        payload.extend_from_slice(&self.minimum_fee_amount.to_le_bytes());
        payload.extend_from_slice(&self.rebate_flat_amount.to_le_bytes());
        payload.extend_from_slice(&self.rebate_bps.to_le_bytes());
        payload.extend_from_slice(&self.rebate_cap_amount.to_le_bytes());
        payload.extend_from_slice(self.fee_recipient_key.as_ref());
        payload.extend_from_slice(&self.effective_from_unix_timestamp.to_le_bytes());
        payload.extend_from_slice(&self.effective_until_unix_timestamp.to_le_bytes());
        payload
    }

    pub fn pda(fee_policy_id: &[u8; 32], program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[seeds::FEE_POLICY, fee_policy_id.as_ref()],
            program_id,
        )
    }

    pub fn verify_pda(
        account:       &AccountInfo,
        fee_policy_id: &[u8; 32],
        program_id:    &Pubkey,
    ) -> Result<u8, ProgramError> {
        let (expected, bump) = Self::pda(fee_policy_id, program_id);

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
            FEE_POLICY_SIZE,
            FEE_POLICY_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    pub(crate) fn load_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        fee_policy_id: &[u8; 32],
        expected_bump: u8,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;

        if state.fee_policy_id != *fee_policy_id {
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
        let expected_bump = Self::verify_pda(account, &state.fee_policy_id, &crate::id())?;

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
            FEE_POLICY_SIZE,
            FEE_POLICY_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    pub(crate) fn load_mut_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        fee_policy_id: &[u8; 32],
        expected_bump: u8,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;

        if state.fee_policy_id != *fee_policy_id {
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
        let expected_bump = Self::verify_pda(account, &state.fee_policy_id, &crate::id())?;

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
            FEE_POLICY_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    // ── Validity ──────────────────────────────────────────────────────────────

    /// Reject incoherent operator-supplied fields (register/update handlers).
    pub fn assert_parameter_sanity(&self) -> Result<(), ProgramError> {
        if self.fee_policy_flags & !SUPPORTED_FEE_FLAGS != 0
            || self.net_fee_floor_zero > 1
            || self.percent_fee_bps > 10_000
            || self.rebate_bps > 10_000
        {
            return Err(ChanceryError::FeePolicyInvalidParameters.into());
        }

        if self.fee_cap_amount != 0 && self.minimum_fee_amount > self.fee_cap_amount {
            return Err(ChanceryError::FeePolicyInvalidParameters.into());
        }

        if self.effective_from_unix_timestamp != 0
            && self.effective_until_unix_timestamp != 0
            && self.effective_from_unix_timestamp >= self.effective_until_unix_timestamp
        {
            return Err(ChanceryError::FeePolicyInvalidParameters.into());
        }

        if self.fee_recipient_policy > fee_recipient::RESERVE_RETENTION
            || self.rounding_mode > rounding::NEAREST
        {
            return Err(ChanceryError::FeePolicyInvalidParameters.into());
        }

        // NONE and RESERVE_RETENTION are non-routing policies. A recipient key
        // on either mode is incoherent and risks callers assuming that an
        // unvalidated account may receive the fee. Routed policies must name a
        // concrete recipient owner up front.
        if self.retains_fee() {
            if self.fee_recipient_key != Pubkey::default() {
                return Err(ChanceryError::FeePolicyInvalidParameters.into());
            }
        } else if self.fee_recipient_key == Pubkey::default() {
            return Err(ChanceryError::FeePolicyInvalidParameters.into());
        } else {
            // issue-10 / spec/16 §16.4: a routed policy names a concrete
            // recipient owner. That owner must be an external identity, never
            // one of Chancery's own signer-capable PDAs - the fifth identity
            // surface alongside permission subjects, authorities, principals,
            // and executors. assert_parameter_sanity re-runs at consumption
            // (compute_*_fee / assert_effective), so this also fails closed for
            // any already-persisted policy rather than stranding fees.
            assert_external_identity(&self.fee_recipient_key, &crate::id())?;
        }

        // The runtime has one unambiguous fee formula. A flat fee and a
        // percentage fee may not compete for precedence, and the two rebate
        // forms may not compete either.
        if self.percent_fee_bps != 0
            && (self.flat_fee_in_asset != 0 || self.flat_fee_in_issued_token != 0)
        {
            return Err(ChanceryError::FeePolicyInvalidParameters.into());
        }

        if self.rebate_flat_amount != 0 && self.rebate_bps != 0 {
            return Err(ChanceryError::FeePolicyInvalidParameters.into());
        }

        let has_rebate = self.rebate_flat_amount != 0 || self.rebate_bps != 0;
        let rebate_to_principal =
            self.fee_policy_flags & fee_flag::REBATE_TO_PRINCIPAL != 0;

        // The current runtime only supports principal-side rebates. Requiring
        // the flag to exactly mirror configured rebate state prevents the
        // account from claiming semantics the transfer path does not perform.
        if has_rebate != rebate_to_principal
            || (self.rebate_cap_amount != 0 && !has_rebate)
        {
            return Err(ChanceryError::FeePolicyInvalidParameters.into());
        }

        if has_rebate && !self.has_fee() {
            return Err(ChanceryError::FeePolicyInvalidParameters.into());
        }

        self.assert_denomination_consistent()
    }

    pub fn assert_effective(&self, now: i64) -> Result<(), ProgramError> {
        // Treat malformed persisted state as unusable. This is intentionally
        // repeated at consumption time rather than relying only on the
        // register/update handlers: a corrupted or pre-upgrade account must
        // never acquire new economic semantics after an upgrade.
        self.assert_parameter_sanity()?;

        if self.fee_policy_flags & fee_flag::ACTIVE == 0 {
            return Err(ChanceryError::FeePolicyNotFound.into());
        }

        if self.effective_from_unix_timestamp != 0 && now < self.effective_from_unix_timestamp {
            return Err(ChanceryError::FeePolicyNotFound.into());
        }

        if self.effective_until_unix_timestamp != 0 && now >= self.effective_until_unix_timestamp {
            return Err(ChanceryError::FeePolicyExpired.into());
        }

        Ok(())
    }

    // ── Denomination consistency ──────────────────────────────────────────────

    /// Exactly one of `FEE_IN_ASSET` / `FEE_IN_ISSUED_TOKEN` must be set; the
    /// flat-fee field of the other denomination must be zero. A pathway stores
    /// one fee-policy reference, so a fee-bearing local pathway is executable
    /// only in the direction matching this denomination. A pathway with no fee
    /// policy may remain usable in both directions. Update handlers keep the
    /// selected denomination immutable after registration.
    pub fn assert_denomination_consistent(&self) -> Result<(), ProgramError> {
        let in_asset = self.fee_policy_flags & fee_flag::FEE_IN_ASSET != 0;
        let in_issued = self.fee_policy_flags & fee_flag::FEE_IN_ISSUED_TOKEN != 0;

        if in_asset == in_issued {
            return Err(ChanceryError::FeePolicyDenominationAmbiguous.into());
        }

        if in_asset && self.flat_fee_in_issued_token != 0 {
            return Err(ChanceryError::FeePolicyDenominationMismatch.into());
        }

        if in_issued && self.flat_fee_in_asset != 0 {
            return Err(ChanceryError::FeePolicyDenominationMismatch.into());
        }

        Ok(())
    }

    // ── Fee computation ───────────────────────────────────────────────────────

    fn floored_basis_points(amount: u64, basis_points: u32) -> Result<u64, ProgramError> {
        let product = (amount as u128)
            .checked_mul(basis_points as u128)
            .ok_or(ChanceryError::ArithmeticOverflow)?;

        u64::try_from(product / 10_000u128)
            .map_err(|_| ChanceryError::ArithmeticOverflow.into())
    }

    fn rounded_basis_points(&self, amount: u64, basis_points: u32) -> Result<u64, ProgramError> {
        let product = (amount as u128)
            .checked_mul(basis_points as u128)
            .ok_or(ChanceryError::ArithmeticOverflow)?;
        let quotient = product / 10_000u128;
        let remainder = product % 10_000u128;
        let rounded = match self.rounding_mode {
            rounding::FLOOR => quotient,
            rounding::CEILING => quotient
                .checked_add(if remainder != 0 { 1 } else { 0 })
                .ok_or(ChanceryError::ArithmeticOverflow)?,
            rounding::NEAREST => quotient
                .checked_add(if remainder >= 5_000 { 1 } else { 0 })
                .ok_or(ChanceryError::ArithmeticOverflow)?,
            _ => return Err(ChanceryError::FeePolicyInvalidParameters.into()),
        };

        u64::try_from(rounded).map_err(|_| ChanceryError::ArithmeticOverflow.into())
    }

    pub fn compute_issued_token_fee_with_input(
        &self,
        gross_input:  u64,
        gross_output: u64,
    ) -> Result<FeeComputation, ProgramError> {
        self.assert_parameter_sanity()?;
        if self.fee_policy_flags & fee_flag::FEE_IN_ISSUED_TOKEN == 0 {
            return Err(ChanceryError::FeePolicyDenominationMismatch.into());
        }
        self.compute_internal(gross_input, gross_output, self.flat_fee_in_issued_token)
    }

    pub fn compute_asset_fee_with_input(
        &self,
        gross_input:  u64,
        gross_output: u64,
    ) -> Result<FeeComputation, ProgramError> {
        self.assert_parameter_sanity()?;
        if self.fee_policy_flags & fee_flag::FEE_IN_ASSET == 0 {
            return Err(ChanceryError::FeePolicyDenominationMismatch.into());
        }
        self.compute_internal(gross_input, gross_output, self.flat_fee_in_asset)
    }

    /// Compatibility wrapper for callers that have the same input/output
    /// notional. New settlement paths use the detailed method above.
    pub fn compute_issued_token_fee(
        &self,
        gross_issued: u64,
    ) -> Result<(u64, u64, u64), ProgramError> {
        let result = self.compute_issued_token_fee_with_input(gross_issued, gross_issued)?;
        Ok((result.assessed_fee, result.effective_rebate, result.net_output))
    }

    /// Compatibility wrapper for callers that have the same input/output
    /// notional. New settlement paths use the detailed method above.
    pub fn compute_asset_fee(
        &self,
        gross_asset: u64,
    ) -> Result<(u64, u64, u64), ProgramError> {
        let result = self.compute_asset_fee_with_input(gross_asset, gross_asset)?;
        Ok((result.assessed_fee, result.effective_rebate, result.net_output))
    }

    fn compute_internal(
        &self,
        gross_input:  u64,
        gross_output: u64,
        flat_fee:     u64,
    ) -> Result<FeeComputation, ProgramError> {
        // `assert_parameter_sanity` rejects PCT_ON_INPUT. Retain the input
        // parameter in the ABI so a future version can add a real input-side
        // fee leg without silently changing existing call sites.
        let _ = gross_input;
        let percent_base = gross_output;

        let raw_fee = if flat_fee != 0 {
            flat_fee
        } else if self.percent_fee_bps != 0 {
            self.rounded_basis_points(percent_base, self.percent_fee_bps)?
        } else {
            0
        };

        let capped_fee = if self.fee_cap_amount != 0 {
            raw_fee.min(self.fee_cap_amount)
        } else {
            raw_fee
        };
        let assessed_fee = capped_fee.max(self.minimum_fee_amount);

        let raw_rebate = if self.rebate_flat_amount != 0 {
            self.rebate_flat_amount
        } else if self.rebate_bps != 0 {
            // Percentage rebates always round down. Applying the fee's
            // CEILING mode to a one-unit assessed fee would round any nonzero
            // rebate rate up to one unit and let settlement splitting erase
            // the entire fee.
            Self::floored_basis_points(assessed_fee, self.rebate_bps)?
        } else {
            0
        };
        let nominal_rebate = if self.rebate_cap_amount != 0 {
            raw_rebate.min(self.rebate_cap_amount)
        } else {
            raw_rebate
        };

        let effective_rebate = if self.net_fee_floor_zero != 0 {
            nominal_rebate.min(assessed_fee)
        } else {
            if nominal_rebate > assessed_fee {
                return Err(ChanceryError::FeePolicyNegativeNetFeeNotAllowed.into());
            }
            nominal_rebate
        };
        let net_fee = assessed_fee
            .checked_sub(effective_rebate)
            .ok_or(ChanceryError::ArithmeticUnderflow)?;
        let net_output = gross_output
            .checked_sub(net_fee)
            .ok_or(ChanceryError::NetOutputZero)?;

        if net_output == 0 {
            return Err(ChanceryError::NetOutputZero.into());
        }

        Ok(FeeComputation {
            assessed_fee,
            nominal_rebate,
            effective_rebate,
            net_fee,
            net_output,
        })
    }

    #[inline]
    pub fn has_fee(&self) -> bool {
        self.flat_fee_in_asset != 0
            || self.flat_fee_in_issued_token != 0
            || self.percent_fee_bps != 0
            || self.minimum_fee_amount != 0
    }

    #[inline]
    pub fn fee_in_asset(&self) -> bool {
        self.fee_policy_flags & fee_flag::FEE_IN_ASSET != 0
    }

    #[inline]
    pub fn fee_in_issued_token(&self) -> bool {
        self.fee_policy_flags & fee_flag::FEE_IN_ISSUED_TOKEN != 0
    }

    /// Whether the fee remains inside the settlement economics rather than
    /// being routed to an external token account.
    ///
    /// Mint: the net fee is not minted, so issued supply increases only by the
    /// principal's net output.
    /// Outbound burn: the net fee remains burned, so only net principal crosses
    /// the corridor.
    /// Redeem: the net fee remains in the collateral reserve.
    ///
    /// NONE is retained as the legacy no-recipient spelling; RESERVE_RETENTION
    /// is the explicit spelling. Both are non-routing and require a zero
    /// `fee_recipient_key`.
    #[inline]
    pub fn retains_fee(&self) -> bool {
        matches!(
            self.fee_recipient_policy,
            fee_recipient::NONE | fee_recipient::RESERVE_RETENTION
        )
    }

    #[inline]
    pub fn routes_fee_to_recipient(&self) -> bool {
        !self.retains_fee()
    }


    /// Verify that the supplied token account is the SPL token account
    /// owned by `fee_recipient_key` and holding `expected_mint`.
    /// SPL Token account layout: mint @ 0..32, owner @ 32..64.
    pub fn assert_recipient_token_account(
        &self,
        fee_recipient_token_account_info: &AccountInfo,
        token_program_account_info:        &AccountInfo,
        expected_mint:                    &Pubkey,
    ) -> Result<(), ProgramError> {
        if fee_recipient_token_account_info.owner != token_program_account_info.key {
            return Err(ChanceryError::TokenProgramMismatch.into());
        }

        let data = fee_recipient_token_account_info.try_borrow_data()?;

        if data.len() < 64 {
            return Err(ChanceryError::AccountDataLengthMismatch.into());
        }

        if data[0..32] != expected_mint.to_bytes() {
            return Err(ChanceryError::FeeRecipientMintMismatch.into());
        }

        if data[32..64] != self.fee_recipient_key.to_bytes() {
            return Err(ChanceryError::FeeRecipientOwnerMismatch.into());
        }

        Ok(())
    }
}

// ─── Size assertion ───────────────────────────────────────────────────────────
const _: () = assert!(
    core::mem::size_of::<FeePolicy>() == FEE_POLICY_SIZE,
    "FeePolicy size mismatch - update FEE_POLICY_SIZE",
);


#[cfg(test)]
mod property_tests {
    use super::*;

    fn base_policy(flags: u64) -> FeePolicy {
        let mut policy = FeePolicy::zeroed();
        policy.fee_policy_flags = flags | fee_flag::ACTIVE;
        policy.fee_recipient_policy = fee_recipient::RESERVE_RETENTION;
        policy.rounding_mode = rounding::FLOOR;
        policy
    }

    fn assert_custom_error(error: ProgramError, expected: ChanceryError) {
        assert_eq!(error, ProgramError::Custom(expected as u32));
    }

    fn next(seed: &mut u64) -> u64 {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        *seed
    }

    #[test]
    fn deterministic_fee_corpus_preserves_conservation_and_bounds() {
        let mut seed = 0x9e37_79b9_7f4a_7c15u64;
        for case_index in 0..10_000u32 {
            let gross = 1 + next(&mut seed) % 10_000_000_000;
            let mut policy = base_policy(fee_flag::FEE_IN_ASSET);
            policy.net_fee_floor_zero = 1;
            policy.percent_fee_bps = (next(&mut seed) % 10_001) as u32;
            policy.fee_cap_amount = match next(&mut seed) % 4 {
                0 => 0,
                _ => next(&mut seed) % (gross + 1),
            };
            let maximum_floor = if policy.fee_cap_amount == 0 {
                gross
            } else {
                policy.fee_cap_amount.min(gross)
            };
            policy.minimum_fee_amount = next(&mut seed) % (maximum_floor + 1);
            policy.rebate_bps = (next(&mut seed) % 10_001) as u32;
            if policy.rebate_bps != 0 {
                policy.fee_policy_flags |= fee_flag::REBATE_TO_PRINCIPAL;
            }
            policy.rebate_cap_amount = match next(&mut seed) % 4 {
                0 => 0,
                _ if policy.rebate_bps != 0 => next(&mut seed) % (gross + 1),
                _ => 0,
            };

            let result = policy.compute_asset_fee_with_input(gross, gross);
            match result {
                Ok(computation) => {
                    assert!(computation.net_output > 0, "case {case_index}");
                    assert!(computation.net_output <= gross, "case {case_index}");
                    assert!(
                        computation.effective_rebate <= computation.nominal_rebate,
                        "case {case_index}",
                    );
                    assert_eq!(
                        computation.assessed_fee - computation.effective_rebate,
                        computation.net_fee,
                        "case {case_index}",
                    );
                    assert_eq!(
                        computation.net_output as u128 + computation.net_fee as u128,
                        gross as u128,
                        "case {case_index}",
                    );
                    assert_eq!(
                        policy.compute_asset_fee_with_input(gross, gross).unwrap(),
                        computation,
                    );
                }
                Err(error) => {
                    assert_custom_error(error, ChanceryError::NetOutputZero);
                }
            }
        }
    }

    #[test]
    fn every_known_flag_has_enforced_runtime_semantics() {
        let mut asset = base_policy(fee_flag::FEE_IN_ASSET);
        asset.flat_fee_in_asset = 4;
        assert_eq!(asset.compute_asset_fee(20).unwrap(), (4, 0, 16));
        assert_custom_error(
            asset.compute_issued_token_fee(20).unwrap_err(),
            ChanceryError::FeePolicyDenominationMismatch,
        );

        let mut issued = base_policy(fee_flag::FEE_IN_ISSUED_TOKEN);
        issued.flat_fee_in_issued_token = 5;
        assert_eq!(issued.compute_issued_token_fee(20).unwrap(), (5, 0, 15));
        assert_custom_error(
            issued.compute_asset_fee(20).unwrap_err(),
            ChanceryError::FeePolicyDenominationMismatch,
        );

        let mut input_percent = base_policy(
            fee_flag::FEE_IN_ASSET | fee_flag::PCT_ON_INPUT,
        );
        input_percent.percent_fee_bps = 1_000;
        assert_custom_error(
            input_percent.compute_asset_fee_with_input(200, 100).unwrap_err(),
            ChanceryError::FeePolicyInvalidParameters,
        );

        let mut rebate = base_policy(
            fee_flag::FEE_IN_ASSET | fee_flag::REBATE_TO_PRINCIPAL,
        );
        rebate.flat_fee_in_asset = 10;
        rebate.rebate_flat_amount = 3;
        let computation = rebate.compute_asset_fee(100).unwrap();
        assert_eq!(computation, (10, 3, 93));

        let mut inactive = base_policy(fee_flag::FEE_IN_ASSET);
        inactive.fee_policy_flags &= !fee_flag::ACTIVE;
        assert_custom_error(
            inactive.assert_effective(1).unwrap_err(),
            ChanceryError::FeePolicyNotFound,
        );
    }

    #[test]
    fn unknown_fee_bits_fail_closed_before_and_after_upgrade() {
        for bit_index in 5..64 {
            let mut policy = base_policy(fee_flag::FEE_IN_ASSET | (1u64 << bit_index));
            policy.flat_fee_in_asset = 1;
            assert_custom_error(
                policy.assert_parameter_sanity().unwrap_err(),
                ChanceryError::FeePolicyInvalidParameters,
            );
            assert_custom_error(
                policy.compute_asset_fee(10).unwrap_err(),
                ChanceryError::FeePolicyInvalidParameters,
            );
        }
    }

    #[test]
    fn all_rounding_modes_are_applied() {
        let cases = [
            (rounding::FLOOR, 1),
            (rounding::CEILING, 2),
            (rounding::NEAREST, 2),
        ];
        for (rounding_mode, expected_fee) in cases {
            let mut policy = base_policy(fee_flag::FEE_IN_ASSET);
            policy.rounding_mode = rounding_mode;
            policy.percent_fee_bps = 5_000;
            let computation = policy.compute_asset_fee(3).unwrap();
            assert_eq!(computation, (expected_fee, 0, 3 - expected_fee));
        }

        let mut nearest_below_half = base_policy(fee_flag::FEE_IN_ASSET);
        nearest_below_half.rounding_mode = rounding::NEAREST;
        nearest_below_half.percent_fee_bps = 4_999;
        assert_eq!(nearest_below_half.compute_asset_fee(1).unwrap(), (0, 0, 1));
    }

    #[test]
    fn percentage_rebate_rounds_down_even_when_fee_rounds_up() {
        let mut policy = base_policy(
            fee_flag::FEE_IN_ASSET | fee_flag::REBATE_TO_PRINCIPAL,
        );
        policy.rounding_mode = rounding::CEILING;
        policy.percent_fee_bps = 1;
        policy.rebate_bps = 1;

        let computation = policy.compute_asset_fee_with_input(1_000, 1_000).unwrap();
        assert_eq!(computation.assessed_fee, 1);
        assert_eq!(computation.nominal_rebate, 0);
        assert_eq!(computation.net_fee, 1);
        assert_eq!(computation.net_output, 999);
    }

    #[test]
    fn nominal_and_effective_rebate_are_distinct_and_conservative() {
        let mut policy = base_policy(
            fee_flag::FEE_IN_ASSET | fee_flag::REBATE_TO_PRINCIPAL,
        );
        policy.flat_fee_in_asset = 10;
        policy.rebate_flat_amount = 25;
        policy.net_fee_floor_zero = 1;

        let computation = policy.compute_asset_fee_with_input(100, 100).unwrap();
        assert_eq!(computation.assessed_fee, 10);
        assert_eq!(computation.nominal_rebate, 25);
        assert_eq!(computation.effective_rebate, 10);
        assert_eq!(computation.net_fee, 0);
        assert_eq!(computation.net_output, 100);

        policy.net_fee_floor_zero = 0;
        assert_custom_error(
            policy.compute_asset_fee(100).unwrap_err(),
            ChanceryError::FeePolicyNegativeNetFeeNotAllowed,
        );
    }

    #[test]
    fn one_policy_cannot_price_both_pathway_directions() {
        let mut policy = base_policy(
            fee_flag::FEE_IN_ASSET | fee_flag::FEE_IN_ISSUED_TOKEN,
        );
        policy.flat_fee_in_asset = 7;
        policy.flat_fee_in_issued_token = 11;

        assert_custom_error(
            policy.assert_parameter_sanity().unwrap_err(),
            ChanceryError::FeePolicyDenominationAmbiguous,
        );
        assert_custom_error(
            policy.compute_asset_fee(100).unwrap_err(),
            ChanceryError::FeePolicyDenominationAmbiguous,
        );
        assert_custom_error(
            policy.compute_issued_token_fee(100).unwrap_err(),
            ChanceryError::FeePolicyDenominationAmbiguous,
        );
    }

    #[test]
    fn malformed_fee_semantics_are_rejected() {
        let mut policy = base_policy(fee_flag::FEE_IN_ASSET);
        policy.net_fee_floor_zero = 2;
        assert!(policy.assert_parameter_sanity().is_err());

        let mut policy = base_policy(fee_flag::FEE_IN_ASSET);
        policy.rounding_mode = 0xff;
        assert!(policy.assert_parameter_sanity().is_err());

        let mut policy = base_policy(fee_flag::FEE_IN_ASSET);
        policy.percent_fee_bps = 1;
        policy.flat_fee_in_asset = 1;
        assert!(policy.assert_parameter_sanity().is_err());

        let mut policy = base_policy(fee_flag::FEE_IN_ASSET);
        policy.rebate_flat_amount = 1;
        assert!(policy.assert_parameter_sanity().is_err());

        let mut policy = base_policy(fee_flag::FEE_IN_ASSET | fee_flag::REBATE_TO_PRINCIPAL);
        policy.flat_fee_in_asset = 1;
        assert!(policy.assert_parameter_sanity().is_err());

        let policy = base_policy(fee_flag::FEE_IN_ASSET | fee_flag::PCT_ON_INPUT);
        assert!(policy.assert_parameter_sanity().is_err());
    }

    #[test]
    fn routed_fee_recipient_cannot_be_a_protocol_signer_pda() {
        // issue-10 / spec/16 §16.4: fee_recipient_key is the fifth external-
        // identity surface. A routed policy naming one of Chancery's own
        // signer-capable PDAs must be rejected, exactly like permission
        // subjects, authorities, principals, and executors. Because
        // assert_parameter_sanity re-runs at consumption, an already-persisted
        // policy also fails closed rather than stranding fees at an
        // unrecoverable address.
        let program_id = crate::id();
        let protocol_seeds: [&[u8]; 3] = [
            seeds::MINT_AUTHORITY,
            seeds::RESERVE_AUTHORITY,
            seeds::EVENT_AUTHORITY,
        ];

        for seed in protocol_seeds {
            let (protocol_pda, _) = Pubkey::find_program_address(&[seed], &program_id);

            let mut policy = base_policy(fee_flag::FEE_IN_ASSET);
            policy.fee_recipient_policy = fee_recipient::PROTOCOL_TREASURY;
            policy.flat_fee_in_asset = 5;
            policy.fee_recipient_key = protocol_pda;

            assert_custom_error(
                policy.assert_parameter_sanity().unwrap_err(),
                ChanceryError::ProtocolSignerIdentityForbidden,
            );
        }

        // Control: a normal external (on-curve) recipient is still accepted.
        let mut recipient = Pubkey::new_unique();
        while !recipient.is_on_curve() {
            recipient = Pubkey::new_unique();
        }
        let mut policy = base_policy(fee_flag::FEE_IN_ASSET);
        policy.fee_recipient_policy = fee_recipient::PROTOCOL_TREASURY;
        policy.flat_fee_in_asset = 5;
        policy.fee_recipient_key = recipient;
        assert!(policy.assert_parameter_sanity().is_ok());
    }
}
