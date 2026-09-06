//! Module activation state.
//!
//! Single switchboard for module status. One PDA per chancery deployment.
//! Settlement and admin handlers gate on this state's per-module status byte.
//!
//! Per
//!   - 5-state status (DISABLED / ADMIN_ONLY / ACTIVE / EMERGENCY_DISABLED / DEPRECATED)
//!   - Default-by-class: core/MVP=ACTIVE; later/dangerous=DISABLED
//!   - `assert_active` for hot path; `assert_admin_or_active` for admin handlers
//!   - Required bootstrap: settlement gates on existence
//!   - Hard-fail on missing/uninitialized
//!   - DEPRECATED only for non-MVP modules

use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::{compiled_modules, deprecatable_modules, module_status, undisableable_modules},
    error::ChanceryError,
};

pub const MODULE_ACTIVATION_STATE_DISCRIMINATOR: [u8; 8] =
    [0x6d, 0x6f, 0x64, 0x61, 0x63, 0x74, 0x76, 0x01];  // "modactv\x01"

//   [u8;8]    discriminator        =  8 @ 0
//   u16       version              =  2 @ 8
//   u8        bump                 =  1 @ 10
//   [u8;5]    _pad0                =  5 @ 11
//   [u8;32]   module_statuses      = 32 @ 16   (status byte per module ID 0x00..0x1F)
//   [u8;32]   last_updated_by      = 32 @ 48
//   u64       last_updated_at_slot =  8 @ 80
//   u64       last_event_sequence_nonce    =  8 @ 88
//   [u8;32]   _reserved            = 32 @ 96
//                                   ─────
//                                   128 bytes
pub const MODULE_ACTIVATION_STATE_SIZE: usize = 128;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ModuleActivationState {
    pub discriminator:             [u8; 8],
    pub version:                   u16,
    pub bump:                      u8,
    pub _pad0:                     [u8; 5],
    pub module_statuses:           [u8; 32],
    pub last_updated_by:           Pubkey,
    pub last_updated_at_slot:      u64,
    pub last_event_sequence_nonce: u64,
    pub _reserved:                 [u8; 32],
}

impl ModuleActivationState {
    pub fn pda(program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[crate::constants::seeds::MODULE_ACTIVATION_STATE],
            program_id,
        )
    }

    pub fn verify_pda(account: &AccountInfo, program_id: &Pubkey) -> Result<u8, ProgramError> {
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
            MODULE_ACTIVATION_STATE_SIZE,
            MODULE_ACTIVATION_STATE_DISCRIMINATOR,
            ChanceryError::ModuleActivationStateNotInitialized,
        )?;

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
            MODULE_ACTIVATION_STATE_SIZE,
            MODULE_ACTIVATION_STATE_DISCRIMINATOR,
            ChanceryError::ModuleActivationStateNotInitialized,
        )?;

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
            MODULE_ACTIVATION_STATE_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    /// Per-module status byte. Out-of-range module IDs return DISABLED.
    #[inline]
    pub fn status_for(&self, module_id: u8) -> u8 {
        if (module_id as usize) >= self.module_statuses.len() {
            return module_status::DISABLED;
        }

        self.module_statuses[module_id as usize]
    }

    /// Hot-path gate. Module must be ACTIVE.
    pub fn assert_active(&self, module_id: u8) -> Result<(), ProgramError> {
        match self.status_for(module_id) {
            module_status::ACTIVE => Ok(()),
            module_status::ADMIN_ONLY         => Err(ChanceryError::ModuleAdminOnly.into()),
            module_status::EMERGENCY_DISABLED => Err(ChanceryError::ModuleEmergencyDisabled.into()),
            module_status::DEPRECATED         => Err(ChanceryError::ModuleDeprecated.into()),
            _                                 => Err(ChanceryError::ModuleNotEnabled.into()),
        }
    }

    /// Admin-handler gate. Module must be ACTIVE or ADMIN_ONLY.
    pub fn assert_admin_or_active(&self, module_id: u8) -> Result<(), ProgramError> {
        match self.status_for(module_id) {
            module_status::ACTIVE | module_status::ADMIN_ONLY => Ok(()),
            module_status::EMERGENCY_DISABLED => Err(ChanceryError::ModuleEmergencyDisabled.into()),
            module_status::DEPRECATED         => Err(ChanceryError::ModuleDeprecated.into()),
            _                                 => Err(ChanceryError::ModuleNotEnabled.into()),
        }
    }

    /// Reject transitions on undisableable modules.
    /// Reject module IDs outside the compiled module set. Unknown IDs must
    /// fail closed: an out-of-range ID would index past `module_statuses`
    /// (panic/abort instead of a clean error), and an unassigned in-range ID
    /// could pre-poison the status slot of a module a future program version
    /// assigns to it.
    pub fn assert_known_module(module_id: u8) -> Result<(), ProgramError> {
        for &compiled_module_id in compiled_modules::IDS.iter() {
            if compiled_module_id == module_id {
                return Ok(());
            }
        }

        Err(ChanceryError::UnknownModule.into())
    }

    pub fn assert_disableable(module_id: u8) -> Result<(), ProgramError> {
        for &id in undisableable_modules::IDS.iter() {
            if id == module_id {
                return Err(ChanceryError::ModuleUndisableable.into());
            }
        }

        Ok(())
    }

    pub fn assert_deprecatable(module_id: u8) -> Result<(), ProgramError> {
        if deprecatable_modules::contains(module_id) {
            return Ok(());
        }

        Err(ChanceryError::InvalidModuleStatus.into())
    }
}

const _: () = assert!(
    core::mem::size_of::<ModuleActivationState>() == MODULE_ACTIVATION_STATE_SIZE,
    "ModuleActivationState size mismatch - update MODULE_ACTIVATION_STATE_SIZE",
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::module;

    fn empty_state() -> ModuleActivationState {
        let mut s = ModuleActivationState::zeroed();
        s.discriminator = MODULE_ACTIVATION_STATE_DISCRIMINATOR;

        s
    }

    #[test]
    fn struct_size_is_exactly_128() {
        assert_eq!(core::mem::size_of::<ModuleActivationState>(), 128);
    }

    #[test]
    fn unset_module_returns_disabled() {
        let s = empty_state();

        assert_eq!(s.status_for(module::CROSS_CHAIN), module_status::DISABLED);
    }

    #[test]
    fn out_of_range_module_returns_disabled() {
        let s = empty_state();

        assert_eq!(s.status_for(0xFF), module_status::DISABLED);
    }

    #[test]
    fn assert_active_fails_when_disabled() {
        let s = empty_state();
        let r = s.assert_active(module::CROSS_CHAIN);

        assert!(r.is_err());
    }

    #[test]
    fn assert_active_succeeds_when_active() {
        let mut s = empty_state();

        s.module_statuses[module::CROSS_CHAIN as usize] = module_status::ACTIVE;

        assert!(s.assert_active(module::CROSS_CHAIN).is_ok());
    }

    #[test]
    fn assert_active_fails_when_admin_only() {
        let mut s = empty_state();

        s.module_statuses[module::CROSS_CHAIN as usize] = module_status::ADMIN_ONLY;

        assert!(s.assert_active(module::CROSS_CHAIN).is_err());
    }

    #[test]
    fn assert_admin_or_active_succeeds_when_admin_only() {
        let mut s = empty_state();

        s.module_statuses[module::CROSS_CHAIN as usize] = module_status::ADMIN_ONLY;

        assert!(s.assert_admin_or_active(module::CROSS_CHAIN).is_ok());
    }

    #[test]
    fn assert_admin_or_active_fails_when_emergency_disabled() {
        let mut s = empty_state();

        s.module_statuses[module::CROSS_CHAIN as usize] = module_status::EMERGENCY_DISABLED;

        assert!(s.assert_admin_or_active(module::CROSS_CHAIN).is_err());
    }

    #[test]
    fn core_module_is_undisableable() {
        assert!(ModuleActivationState::assert_disableable(module::CORE).is_err());
        assert!(ModuleActivationState::assert_disableable(module::EVENTS_CPI).is_err());
        assert!(ModuleActivationState::assert_disableable(module::EVIDENCE).is_err());
        assert!(ModuleActivationState::assert_disableable(module::CONTROL).is_err());
    }

    #[test]
    fn cross_chain_is_disableable() {
        assert!(ModuleActivationState::assert_disableable(module::CROSS_CHAIN).is_ok());
    }

    #[test]
    fn exact_compiled_module_allowlist_accepts_every_dispatch_wired_id() {
        for &module_id in compiled_modules::IDS.iter() {
            assert!(ModuleActivationState::assert_known_module(module_id).is_ok());
        }
    }

    #[test]
    fn reserved_and_out_of_range_module_ids_fail_closed() {
        for module_id in [
            module::COMPARTMENTS,
            module::PROVENANCE,
            module::INSURANCE,
            module::ENFORCEMENT,
            0x11,
            0xFF,
        ] {
            assert_eq!(
                ModuleActivationState::assert_known_module(module_id),
                Err(ChanceryError::UnknownModule.into()),
            );
        }
    }
    #[test]
    fn compiled_mvp_modules_are_not_deprecatable() {
        for &module_id in compiled_modules::IDS.iter() {
            assert_eq!(
                ModuleActivationState::assert_deprecatable(module_id),
                Err(ChanceryError::InvalidModuleStatus.into()),
            );
        }
    }

}
