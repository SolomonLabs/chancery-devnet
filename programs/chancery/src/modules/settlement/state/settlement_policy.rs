use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    account_security::assert_optional_external_identity,
    constants::seeds,
    error::ChanceryError,
};

// ─── Discriminator ────────────────────────────────────────────────────────────
pub const SETTLEMENT_POLICY_DISCRIMINATOR: [u8; 8] =
    [0x73, 0x74, 0x6c, 0x70, 0x6f, 0x6c, 0x79, 0x00];

// ─── Account size ─────────────────────────────────────────────────────────────
//   [u8;8]   discriminator               =  8 @ 0
//   u16      version                     =  2 @ 8
//   u8       bump                        =  1 @ 10
//   [u8;5]   _pad0                       =  5 @ 11  -> u64 at 16
//   u64      policy_flags                =  8 @ 16
//   u32      allowed_settlement_modes    =  4 @ 24
//   [u8;4]   _pad1                       =  4 @ 28  -> [u8;32] at 32
//   [u8;32]  policy_id                   = 32 @ 32
//   [u8;32]  allowed_asset_mint          = 32 @ 64
//   [u8;32]  allowed_principal_a         = 32 @ 96
//   [u8;32]  allowed_principal_b         = 32 @ 128
//   [u8;32]  designated_executor         = 32 @ 160
//   u64      max_notional                =  8 @ 192  (192%8=0 Y)
//   u64      min_notional                =  8 @ 200
//   i64      valid_after_unix_timestamp  =  8 @ 208
//   i64      expires_at_unix_timestamp   =  8 @ 216
//   [u8;32]  created_by                  = 32 @ 224
//   [u8;32]  _reserved                   = 32 @ 256
//                                         ─────
//                                         288 bytes
pub const SETTLEMENT_POLICY_SIZE: usize = 288;

// ─── State ────────────────────────────────────────────────────────────────────

/// Optional pre-approved settlement policy binding principals, executor,
/// asset, and limits for a class of intents.
/// Seed: [b"settlement-policy", policy_id]
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SettlementPolicy {
    pub discriminator:              [u8; 8],
    pub version:                    u16,
    pub bump:                       u8,
    pub _pad0:                      [u8; 5],

    /// Additional policy-level flags (reserved).
    pub policy_flags:               u64,
    /// Bitmask of `settlement_mode::*` values permitted under this policy.
    pub allowed_settlement_modes:   u32,
    pub _pad1:                      [u8; 4],

    /// Opaque policy identifier.
    pub policy_id:                  [u8; 32],

    /// Permitted collateral asset mint. `Pubkey::default()` means any registered asset.
    pub allowed_asset_mint:         Pubkey,

    /// Principal A - minting/depositing side. `default()` = any permitted principal.
    pub allowed_principal_a:        Pubkey,

    /// Principal B - redeeming/receiving side. `default()` = any permitted principal.
    pub allowed_principal_b:        Pubkey,

    /// Exact executor identity permitted by this policy. The executor must
    /// also hold the pathway role and sign. `default()` permits any such actor.
    pub designated_executor:        Pubkey,

    /// Maximum notional amount per intent bound by this policy.
    pub max_notional:               u64,

    /// Minimum notional amount per intent.
    pub min_notional:               u64,

    /// Intent is invalid before this timestamp.
    pub valid_after_unix_timestamp: i64,

    /// Policy expires at this timestamp; intents after this are rejected.
    pub expires_at_unix_timestamp:  i64,

    /// The authority that created this policy.
    pub created_by:                 Pubkey,
    pub _reserved:                  [u8; 32],
}

// ─── Helpers ──────────────────────────────────────────────────────────────────
impl SettlementPolicy {
    pub fn pda(policy_id: &[u8; 32], program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[seeds::SETTLEMENT_POLICY, policy_id.as_ref()],
            program_id,
        )
    }

    pub fn verify_pda(
        account   : &AccountInfo,
        policy_id : &[u8; 32],
        program_id: &Pubkey,
    ) -> Result<u8, ProgramError> {
        let (expected, bump) = Self::pda(policy_id, program_id);

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
            SETTLEMENT_POLICY_SIZE,
            SETTLEMENT_POLICY_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    pub(crate) fn load_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        policy_id: &[u8; 32],
        expected_bump: u8,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;

        if state.policy_id != *policy_id {
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
        let expected_bump = Self::verify_pda(account, &state.policy_id, &crate::id())?;

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
            SETTLEMENT_POLICY_SIZE,
            SETTLEMENT_POLICY_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    /// Mutable counterpart to `load_verified`.
    pub fn load_verified_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;
        let expected_bump = Self::verify_pda(account, &state.policy_id, &crate::id())?;

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
            SETTLEMENT_POLICY_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    // ── Guards ────────────────────────────────────────────────────────────────

    pub fn assert_parameter_sanity(&self) -> Result<(), ProgramError> {
        // Bit 0 (DIRECT_PRINCIPAL) is reserved: direct settlement does not
        // load a SettlementPolicy. Only intent-backed modes are enforceable.
        let known_mode_mask = (1u32 << crate::constants::settlement_mode::DELEGATED_NON_CUSTODIAL)
            | (1u32 << crate::constants::settlement_mode::TRILATERAL_ATOMIC);

        if self.policy_id == [0u8; 32]
            || self.policy_flags != 0
            || self.allowed_settlement_modes == 0
            || self.allowed_settlement_modes & !known_mode_mask != 0
            || (self.max_notional != 0 && self.min_notional > self.max_notional)
            || (self.expires_at_unix_timestamp != 0
                && self.valid_after_unix_timestamp >= self.expires_at_unix_timestamp)
        {
            return Err(ChanceryError::SettlementPolicyInvalidParameters.into());
        }

        let program_id = crate::id();
        assert_optional_external_identity(&self.allowed_principal_a, &program_id)?;
        assert_optional_external_identity(&self.allowed_principal_b, &program_id)?;
        assert_optional_external_identity(&self.designated_executor, &program_id)?;

        Ok(())
    }

    pub fn assert_not_expired(&self, now: i64) -> Result<(), ProgramError> {
        if self.expires_at_unix_timestamp != 0 && now >= self.expires_at_unix_timestamp {
            return Err(ChanceryError::SettlementPolicyExpired.into());
        }

        Ok(())
    }

    pub fn assert_effective(&self, now: i64) -> Result<(), ProgramError> {
        self.assert_parameter_sanity()?;

        if self.valid_after_unix_timestamp != 0 && now < self.valid_after_unix_timestamp {
            return Err(ChanceryError::IntentNotYetValid.into());
        }

        self.assert_not_expired(now)
    }

    pub fn assert_settlement_mode_allowed(
        &self,
        mode: u8,
    ) -> Result<(), ProgramError> {
        if !matches!(
            mode,
            crate::constants::settlement_mode::DELEGATED_NON_CUSTODIAL
                | crate::constants::settlement_mode::TRILATERAL_ATOMIC
        ) {
            return Err(ChanceryError::SettlementModeNotAllowed.into());
        }

        if self.allowed_settlement_modes & (1u32 << mode) == 0 {
            return Err(ChanceryError::SettlementModeNotAllowed.into());
        }

        Ok(())
    }

    pub fn assert_asset(&self, asset_mint: &Pubkey) -> Result<(), ProgramError> {
        let zero = Pubkey::default();

        if self.allowed_asset_mint != zero && &self.allowed_asset_mint != asset_mint {
            return Err(ChanceryError::AssetNotRegistered.into());
        }

        Ok(())
    }

    pub fn assert_notional(&self, amount: u64) -> Result<(), ProgramError> {
        if amount < self.min_notional {
            return Err(ChanceryError::AmountBelowMinimum.into());
        }

        if self.max_notional != 0 && amount > self.max_notional {
            return Err(ChanceryError::AmountExceedsMaximum.into());
        }

        Ok(())
    }

    pub fn assert_principals(
        &self,
        principal_a: &Pubkey,
        principal_b: &Pubkey,
    ) -> Result<(), ProgramError> {
        let zero = Pubkey::default();

        if self.allowed_principal_a != zero && &self.allowed_principal_a != principal_a {
            return Err(ChanceryError::PermissionScopeMismatch.into());
        }

        if self.allowed_principal_b != zero && &self.allowed_principal_b != principal_b {
            return Err(ChanceryError::PermissionScopeMismatch.into());
        }

        Ok(())
    }

    pub fn assert_executor(&self, executor: &Pubkey) -> Result<(), ProgramError> {
        if self.designated_executor != Pubkey::default()
            && &self.designated_executor != executor
        {
            return Err(ChanceryError::ExecutorNotPermitted.into());
        }

        Ok(())
    }
}

// ─── Size assertion ───────────────────────────────────────────────────────────
const _: () = assert!(
    core::mem::size_of::<SettlementPolicy>() == SETTLEMENT_POLICY_SIZE,
    "SettlementPolicy size mismatch - update SETTLEMENT_POLICY_SIZE",
);

#[cfg(test)]
mod tests {
    use super::*;
    use bytemuck::Zeroable;

    fn valid_policy() -> SettlementPolicy {
        let mut policy = SettlementPolicy::zeroed();
        policy.policy_id = [1u8; 32];
        policy.allowed_settlement_modes =
            1u32 << crate::constants::settlement_mode::DELEGATED_NON_CUSTODIAL;
        policy.min_notional = 10;
        policy.max_notional = 100;
        policy
    }

    #[test]
    fn valid_policy_accepts_bound_terms() {
        let policy = valid_policy();
        assert!(policy.assert_parameter_sanity().is_ok());
        assert!(policy.assert_effective(0).is_ok());
        assert!(policy
            .assert_settlement_mode_allowed(
                crate::constants::settlement_mode::DELEGATED_NON_CUSTODIAL,
            )
            .is_ok());
        assert!(policy.assert_notional(10).is_ok());
        assert!(policy.assert_notional(100).is_ok());
    }

    #[test]
    fn invalid_mode_and_notional_are_rejected() {
        let policy = valid_policy();
        assert!(policy
            .assert_settlement_mode_allowed(crate::constants::settlement_mode::TRILATERAL_ATOMIC)
            .is_err());
        assert!(policy.assert_settlement_mode_allowed(u8::MAX).is_err());
        assert!(policy.assert_notional(9).is_err());
        assert!(policy.assert_notional(101).is_err());
    }

    #[test]
    fn expired_and_not_yet_effective_policies_are_rejected() {
        let mut policy = valid_policy();
        policy.valid_after_unix_timestamp = 100;
        policy.expires_at_unix_timestamp = 200;
        assert!(policy.assert_effective(99).is_err());
        assert!(policy.assert_effective(100).is_ok());
        assert!(policy.assert_effective(200).is_err());
    }

    #[test]
    fn reserved_policy_flags_are_rejected() {
        let mut policy = valid_policy();
        policy.policy_flags = 1;
        assert!(policy.assert_parameter_sanity().is_err());
    }

    #[test]
    fn executor_binding_is_enforced() {
        let mut policy = valid_policy();
        let executor = Pubkey::new_from_array([2u8; 32]);
        policy.designated_executor = executor;
        assert!(policy.assert_executor(&executor).is_ok());
        assert!(policy.assert_executor(&Pubkey::new_from_array([3u8; 32])).is_err());
    }

    #[test]
    fn protocol_signer_identities_are_rejected_from_subject_bindings() {
        let (protocol_signer, _) = Pubkey::find_program_address(
            &[crate::constants::seeds::MINT_AUTHORITY],
            &crate::id(),
        );

        for field in 0..3 {
            let mut policy = valid_policy();
            match field {
                0 => policy.allowed_principal_a = protocol_signer,
                1 => policy.allowed_principal_b = protocol_signer,
                _ => policy.designated_executor = protocol_signer,
            }
            assert!(policy.assert_parameter_sanity().is_err());
        }
    }
}
