use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::{asset_mode, assert_extension_observation_fresh, collateral_default_forbidden, seeds, RATE_PRECISION_E9},
    error::ChanceryError,
};

// ─── Discriminator ────────────────────────────────────────────────────────────
pub const ASSET_CONFIG_DISCRIMINATOR: [u8; 8] =
    [0x61, 0x73, 0x73, 0x74, 0x63, 0x66, 0x67, 0x00];

// ─── Account size ─────────────────────────────────────────────────────────────
// u128 is avoided - extension masks use [u64; 2] for safe alignment on BPF.
//   [u8;8]   discriminator                            =  8 @ 0
//   u16      version                                  =  2 @ 8
//   u8       bump                                     =  1 @ 10
//   u8       decimals                                 =  1 @ 11
//   u8       mode                                     =  1 @ 12
//   [u8;3]   _pad0                                    =  3 @ 13
//   u64      asset_flags                              =  8 @ 16
//   [u8;32]  asset_mint                               = 32 @ 24
//   [u8;32]  asset_token_program                      = 32 @ 56
//   [u8;32]  primary_reserve_compartment              = 32 @ 88
//   [u64;2]  approved_extension_mask                  = 16 @ 120  (8-aligned Y)
//   [u64;2]  observed_extension_mask                  = 16 @ 136
//   u64      deposit_rate_e9                          =  8 @ 152
//   u64      redeem_rate_e9                           =  8 @ 160
//   u64      minimum_deposit_amount                   =  8 @ 168
//   u64      minimum_redeem_amount                    =  8 @ 176
//   u64      maximum_single_settlement                =  8 @ 184
//   u64      status_flags                             =  8 @ 192
//   [u64;2]  forbidden_extension_mask                 = 16 @ 200  (8-aligned Y)
//   [u64;2]  required_module_mask                     = 16 @ 216
//   u64      extension_observed_at_slot               =  8 @ 232
//   u64      max_extension_observation_age_slots      =  8 @ 240
//   [u8;32]  _reserved                                = 32 @ 248
//                                                       ─────
//                                                       280 bytes
pub const ASSET_CONFIG_RESERVED_SIZE: usize = 32;
pub const ASSET_CONFIG_SIZE: usize          = 280;

// ─── State ────────────────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct AssetConfig {
    pub discriminator:                       [u8; 8],
    pub version:                             u16,
    pub bump:                                u8,

    /// Token decimals - must match the on-chain mint.
    pub decimals:                            u8,

    /// Asset lifecycle mode - one of `asset_mode::*`.
    pub mode:                                u8,
    pub _pad0:                               [u8; 3],

    /// Bitfield using `status_flag::*` constants.
    pub asset_flags:                         u64,

    /// Collateral asset mint this config governs.
    pub asset_mint:                          Pubkey,

    /// Token program owning `asset_mint`.
    pub asset_token_program:                 Pubkey,

    /// ID of the primary reserve compartment for intake routing.
    /// Zero if compartment module is inactive.
    pub primary_reserve_compartment_id:      [u8; 32],

    /// Bitmask of Token-2022 extensions explicitly approved for this asset.
    /// Split into two u64 words (low, high) to avoid u128 alignment hazards on BPF.
    pub approved_extension_mask:             [u64; 2],

    /// Bitmask of extensions observed on the mint (TLV-derived at registration
    /// or `refresh_asset_extension_observation`).
    pub observed_extension_mask:             [u64; 2],

    /// Fixed-point deposit rate (asset -> issued token): rate × 10^9.
    pub deposit_rate_e9:                     u64,

    /// Fixed-point redeem rate (issued token -> asset): rate × 10^9.
    pub redeem_rate_e9:                      u64,
    pub minimum_deposit_amount:              u64,
    pub minimum_redeem_amount:               u64,
    pub maximum_single_settlement_amount:    u64,
    pub status_flags:                        u64,

    /// Bitmask of Token-2022 extensions that must NOT be present on collateral.
    /// Any observed extension matching this mask will be rejected.
    pub forbidden_extension_mask:            [u64; 2],

    /// Reserved for a future asset-level module dependency feature. The current
    /// binary requires this field to remain unchanged (new registrations write
    /// zero) and does not represent it as an active settlement policy.
    pub required_module_mask:                [u64; 2],

    /// Slot at which `observed_extension_mask` was last refreshed by a
    /// verify pass. the specification freshness gate - settlement rejects when
    /// `(current_slot - extension_observed_at_slot) >
    /// max_extension_observation_age_slots`.
    pub extension_observed_at_slot:          u64,

    /// Max age in slots before `observed_extension_mask` is considered
    /// stale. Zero is the deliberate no-age-ceiling sentinel: chronology and
    /// mask checks remain mandatory, but periodic freshness expiry is disabled.
    /// The value is fixed at registration in this layout; a later mutation path
    /// must be introduced as a versioned policy change if required.
    pub max_extension_observation_age_slots: u64,

    /// Append-only layout headroom. Must remain zero until consumed by a
    /// versioned layout change, and must be replenished in that same change.
    pub _reserved:                           [u8; 32],
}

// ─── Extension mask helpers ───────────────────────────────────────────────────
/// Combine the two-word mask into a single u128 for bitwise checks.
#[inline]
fn mask_to_u128(words: [u64; 2]) -> u128 {
    (words[0] as u128) | ((words[1] as u128) << 64)
}

// ─── Helpers ──────────────────────────────────────────────────────────────────
impl AssetConfig {
    /// Canonical governance payload for pending-change hashing.
    ///
    /// The runtime observation is not governance-controlled state:
    /// observed_extension_mask is deliberately excluded, and
    /// extension_observed_at_slot is deliberately excluded, so an operations
    /// refresh cannot invalidate an otherwise unchanged governance proposal.
    /// Consume-time validation still applies the proposed policy to the latest
    /// live observation before any write.
    pub(crate) fn config_change_payload(&self) -> Vec<u8> {
        let mut payload = Vec::with_capacity(210);
        payload.push(self.decimals);
        payload.push(self.mode);
        payload.extend_from_slice(&self.asset_flags.to_le_bytes());
        payload.extend_from_slice(self.asset_mint.as_ref());
        payload.extend_from_slice(self.asset_token_program.as_ref());
        payload.extend_from_slice(&self.primary_reserve_compartment_id);
        payload.extend_from_slice(&self.approved_extension_mask[0].to_le_bytes());
        payload.extend_from_slice(&self.approved_extension_mask[1].to_le_bytes());
        payload.extend_from_slice(&self.deposit_rate_e9.to_le_bytes());
        payload.extend_from_slice(&self.redeem_rate_e9.to_le_bytes());
        payload.extend_from_slice(&self.minimum_deposit_amount.to_le_bytes());
        payload.extend_from_slice(&self.minimum_redeem_amount.to_le_bytes());
        payload.extend_from_slice(&self.maximum_single_settlement_amount.to_le_bytes());
        payload.extend_from_slice(&self.status_flags.to_le_bytes());
        payload.extend_from_slice(&self.forbidden_extension_mask[0].to_le_bytes());
        payload.extend_from_slice(&self.forbidden_extension_mask[1].to_le_bytes());
        payload.extend_from_slice(&self.required_module_mask[0].to_le_bytes());
        payload.extend_from_slice(&self.required_module_mask[1].to_le_bytes());
        payload.extend_from_slice(&self.max_extension_observation_age_slots.to_le_bytes());
        payload
    }

    #[inline]
    pub(crate) fn zero_reserved(&mut self) {
        self._reserved = [0u8; ASSET_CONFIG_RESERVED_SIZE];
    }

    pub fn pda(asset_mint: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[seeds::ASSET_CONFIG, asset_mint.as_ref()],
            program_id,
        )
    }

    pub fn verify_pda(
        account:    &AccountInfo,
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
            ASSET_CONFIG_SIZE,
            ASSET_CONFIG_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

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
            ASSET_CONFIG_SIZE,
            ASSET_CONFIG_DISCRIMINATOR,
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
            ASSET_CONFIG_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    // ── Mode guards ───────────────────────────────────────────────────────────

    #[inline]
    pub fn is_active(&self) -> bool { self.mode == asset_mode::ACTIVE }

    #[inline]
    pub fn is_wind_down(&self) -> bool { self.mode == asset_mode::WIND_DOWN }

    #[inline]
    pub fn is_frozen(&self) -> bool { self.mode == asset_mode::FROZEN }

    pub fn assert_deposits_permitted(&self) -> Result<(), ProgramError> {
        match self.mode {
            asset_mode::ACTIVE => Ok(()),
            asset_mode::WIND_DOWN | asset_mode::FROZEN => {
                Err(ChanceryError::AssetModeForbids.into())
            }
            _ => Err(ChanceryError::AssetModeForbids.into()),
        }
    }

    pub fn assert_redeems_permitted(&self) -> Result<(), ProgramError> {
        match self.mode {
            asset_mode::ACTIVE | asset_mode::WIND_DOWN => Ok(()),
            asset_mode::FROZEN => Err(ChanceryError::AssetModeForbids.into()),
            _ => Err(ChanceryError::AssetModeForbids.into()),
        }
    }

    // ── Extension guard ───────────────────────────────────────────────────────

    pub fn assert_extensions_approved(&self) -> Result<(), ProgramError> {
        self.assert_observed_mask_allowed(self.observed_extension_mask)
    }

    /// TLV-derived mask must be ⊆ `approved_extension_mask` and disjoint from
    /// `forbidden_extension_mask`.
    pub fn assert_observed_mask_allowed(
        &self,
        observed: [u64; 2],
    ) -> Result<(), ProgramError> {
        let effective_forbidden = [
            self.forbidden_extension_mask[0] | collateral_default_forbidden::MINT_FORBIDDEN_LO,
            self.forbidden_extension_mask[1] | collateral_default_forbidden::MINT_FORBIDDEN_HI,
        ];

        Self::assert_mask_subset_of_approved(
            observed,
            self.approved_extension_mask,
            effective_forbidden,
        )
    }

    /// Reject when `observed` overlaps the asset-level forbidden mask.
    /// Used by issued-token deployment verification (spec 09 §9.4).
    pub fn assert_forbidden_extensions_absent(
        &self,
        observed: [u64; 2],
    ) -> Result<(), ProgramError> {
        if mask_to_u128(observed) & mask_to_u128(self.forbidden_extension_mask) != 0 {
            return Err(ChanceryError::ForbiddenExtension.into());
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

    /// Observed mint TLV mask must be ⊆ approved and disjoint from forbidden.
    pub fn assert_mask_subset_of_approved(
        observed:  [u64; 2],
        approved:  [u64; 2],
        forbidden: [u64; 2],
    ) -> Result<(), ProgramError> {
        let observed_u  = mask_to_u128(observed);
        let approved_u  = mask_to_u128(approved);
        let forbidden_u = mask_to_u128(forbidden);

        if observed_u & !approved_u != 0 {
            return Err(ChanceryError::UnsupportedTokenExtension.into());
        }

        if observed_u & forbidden_u != 0 {
            return Err(ChanceryError::ForbiddenExtension.into());
        }

        Ok(())
    }

    // ── Amount guards ─────────────────────────────────────────────────────────

    /// Persisted bounds must describe at least one executable positive amount.
    /// A zero maximum or a minimum above the maximum would permanently brick
    /// every settlement using this asset.
    pub fn assert_amount_invariants(
        minimum_deposit_amount: u64,
        minimum_redeem_amount: u64,
        maximum_single_settlement_amount: u64,
    ) -> Result<(), ProgramError> {
        if maximum_single_settlement_amount == 0
            || minimum_deposit_amount > maximum_single_settlement_amount
            || minimum_redeem_amount > maximum_single_settlement_amount
        {
            return Err(ChanceryError::AssetEconomicsInvariantViolated.into());
        }
        Ok(())
    }

    /// Prove that each configured amount interval contains at least one
    /// positive input whose rate conversion is both nonzero and representable
    /// as a u64. Conversion is monotonic, so checking the smallest positive
    /// allowed input for overflow and the maximum input for zero output is
    /// sufficient to rule out ranges where every settlement deterministically
    /// fails at execution time.
    pub fn assert_amount_rate_ranges_executable(
        deposit_rate_e9:                 u64,
        redeem_rate_e9:                  u64,
        minimum_deposit_amount:          u64,
        minimum_redeem_amount:           u64,
        maximum_single_settlement_amount: u64,
    ) -> Result<(), ProgramError> {
        fn assert_range(
            rate_e9: u64,
            minimum_amount: u64,
            maximum_amount: u64,
        ) -> Result<(), ProgramError> {
            let smallest_positive_amount = minimum_amount.max(1);
            if smallest_positive_amount > maximum_amount {
                return Err(ChanceryError::AssetEconomicsInvariantViolated.into());
            }

            let smallest_output = (smallest_positive_amount as u128)
                .checked_mul(rate_e9 as u128)
                .ok_or(ChanceryError::ArithmeticOverflow)?
                / RATE_PRECISION_E9 as u128;
            let maximum_output = (maximum_amount as u128)
                .checked_mul(rate_e9 as u128)
                .ok_or(ChanceryError::ArithmeticOverflow)?
                / RATE_PRECISION_E9 as u128;

            if smallest_output > u64::MAX as u128 || maximum_output == 0 {
                return Err(ChanceryError::AssetEconomicsInvariantViolated.into());
            }

            Ok(())
        }

        assert_range(
            deposit_rate_e9,
            minimum_deposit_amount,
            maximum_single_settlement_amount,
        )?;
        assert_range(
            redeem_rate_e9,
            minimum_redeem_amount,
            maximum_single_settlement_amount,
        )
    }

    pub fn assert_deposit_amount(&self, amount: u64) -> Result<(), ProgramError> {
        if amount < self.minimum_deposit_amount {
            return Err(ChanceryError::AmountBelowMinimum.into());
        }

        if amount > self.maximum_single_settlement_amount {
            return Err(ChanceryError::AmountExceedsMaximum.into());
        }

        Ok(())
    }

    pub fn assert_redeem_amount(&self, amount: u64) -> Result<(), ProgramError> {
        if amount < self.minimum_redeem_amount {
            return Err(ChanceryError::AmountBelowMinimum.into());
        }

        if amount > self.maximum_single_settlement_amount {
            return Err(ChanceryError::AmountExceedsMaximum.into());
        }

        Ok(())
    }

    // ── Economic safety invariants ────────────────────────────────────────────

    /// Solvency invariants over the conversion rates. Enforced at every write
    /// path (`register_asset`, direct and pending-change config updates):
    ///
    ///   1. Both rates are nonzero - a zero rate silently converts deposits or
    ///      redemptions to zero output while still moving the input leg.
    ///   2. `deposit_rate_e9 × redeem_rate_e9 ≤ 10^18` - a full
    ///      deposit→redeem cycle can never return more asset than was
    ///      deposited. This bound is decimals-independent because the two
    ///      conversions cancel, so it is exactly the no-free-money invariant
    ///      for round trips regardless of mint decimal configuration.
    pub fn assert_economic_invariants(
        deposit_rate_e9: u64,
        redeem_rate_e9:  u64,
    ) -> Result<(), ProgramError> {
        if deposit_rate_e9 == 0 || redeem_rate_e9 == 0 {
            return Err(ChanceryError::AssetEconomicsInvariantViolated.into());
        }

        let round_trip = (deposit_rate_e9 as u128)
            .checked_mul(redeem_rate_e9 as u128)
            .ok_or(ChanceryError::ArithmeticOverflow)?;

        if round_trip > (RATE_PRECISION_E9 as u128) * (RATE_PRECISION_E9 as u128) {
            return Err(ChanceryError::AssetEconomicsInvariantViolated.into());
        }

        Ok(())
    }

    // ── Rate math ─────────────────────────────────────────────────────────────

    /// output = (asset_amount × deposit_rate_e9) / 10^9
    pub fn compute_mint_output(&self, asset_amount: u64) -> Result<u64, ProgramError> {
        let out = (asset_amount as u128)
            .checked_mul(self.deposit_rate_e9 as u128)
            .ok_or(ChanceryError::ArithmeticOverflow)?
            .checked_div(RATE_PRECISION_E9 as u128)
            .ok_or(ChanceryError::DivisionByZero)?;

        u64::try_from(out).map_err(|_| ChanceryError::ArithmeticOverflow.into())
    }

    /// output = (token_amount × redeem_rate_e9) / 10^9
    pub fn compute_redeem_output(
        &self,
        token_amount: u64,
    ) -> Result<u64, ProgramError> {
        let out = (token_amount as u128)
            .checked_mul(self.redeem_rate_e9 as u128)
            .ok_or(ChanceryError::ArithmeticOverflow)?
            .checked_div(RATE_PRECISION_E9 as u128)
            .ok_or(ChanceryError::DivisionByZero)?;

        u64::try_from(out).map_err(|_| ChanceryError::ArithmeticOverflow.into())
    }
}

// ─── Size assertion ───────────────────────────────────────────────────────────
const _: () = assert!(
    core::mem::size_of::<AssetConfig>() == ASSET_CONFIG_SIZE,
    "AssetConfig size mismatch - update ASSET_CONFIG_SIZE",
);
