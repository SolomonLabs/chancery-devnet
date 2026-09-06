use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;

use crate::{
    constants::{ix::control as ix_id, role},
    error::ChanceryError,
};

pub mod cpi;
pub mod accept_config_change;
pub mod cancel_config_change;
pub mod close_expired_config_change;
pub mod freeze_issued_token_account;
pub mod initialize_module_activation_state;
pub mod propose_config_change;
pub mod set_asset_pause;
pub mod set_counterparty_pause;
pub mod set_executor_pause;
pub mod set_global_pause;
pub mod set_module_status;
pub mod set_module_status_with_pending_change;
pub mod set_pathway_pause;
pub mod thaw_issued_token_account;

/// Principal/counterparty settlement roles that the narrow counterparty pause
/// endpoint is authorized to contain.
pub(super) const COUNTERPARTY_PAUSABLE_ROLE_MASK: u128 =
    role::CAN_MINT_DIRECT
        | role::CAN_REDEEM_DIRECT
        | role::CAN_MINT_DELEGATED
        | role::CAN_REDEEM_DELEGATED
        | role::CAN_USE_TRILATERAL_PATHWAY;

/// Every settlement role that may coexist on one pathway-scoped permission
/// record without turning either narrow pause endpoint into a governance or
/// recovery-role suppression surface.
pub(super) const SETTLEMENT_PAUSABLE_ROLE_MASK: u128 =
    COUNTERPARTY_PAUSABLE_ROLE_MASK | role::CAN_EXECUTE_SETTLEMENT;

#[inline]
pub(super) fn is_counterparty_pause_eligible(scope_kind: u8, roles: u128) -> bool {
    scope_kind == crate::constants::scope::PATHWAY
        && roles & COUNTERPARTY_PAUSABLE_ROLE_MASK != 0
        && roles & !SETTLEMENT_PAUSABLE_ROLE_MASK == 0
}

#[inline]
pub(super) fn is_executor_pause_eligible(scope_kind: u8, roles: u128) -> bool {
    scope_kind == crate::constants::scope::PATHWAY
        && roles & role::CAN_EXECUTE_SETTLEMENT != 0
        && roles & !SETTLEMENT_PAUSABLE_ROLE_MASK == 0
}

pub fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
    if data.is_empty() {
        return Err(ChanceryError::InstructionDataTooShort.into());
    }

    match data[0] {
        ix_id::ACCEPT_CONFIG_CHANGE                  => accept_config_change::handle(accounts, &data[1..]),
        ix_id::CANCEL_CONFIG_CHANGE                  => cancel_config_change::handle(accounts, &data[1..]),
        ix_id::CLOSE_EXPIRED_CONFIG_CHANGE           => close_expired_config_change::handle(accounts, &data[1..]),
        ix_id::FREEZE_ISSUED_TOKEN_ACCOUNT           => freeze_issued_token_account::handle(accounts, &data[1..]),
        ix_id::INITIALIZE_MODULE_ACTIVATION_STATE    => initialize_module_activation_state::handle(accounts, &data[1..]),
        ix_id::PROPOSE_CONFIG_CHANGE                 => propose_config_change::handle(accounts, &data[1..]),
        ix_id::SET_ASSET_PAUSE                       => set_asset_pause::handle(accounts, &data[1..]),
        ix_id::SET_COUNTERPARTY_PAUSE                => set_counterparty_pause::handle(accounts, &data[1..]),
        ix_id::SET_EXECUTOR_PAUSE                    => set_executor_pause::handle(accounts, &data[1..]),
        ix_id::SET_GLOBAL_PAUSE                      => set_global_pause::handle(accounts, &data[1..]),
        ix_id::SET_MODULE_STATUS                     => set_module_status::handle(accounts, &data[1..]),
        ix_id::SET_MODULE_STATUS_WITH_PENDING_CHANGE => set_module_status_with_pending_change::handle(accounts, &data[1..]),
        ix_id::SET_PATHWAY_PAUSE                     => set_pathway_pause::handle(accounts, &data[1..]),
        ix_id::THAW_ISSUED_TOKEN_ACCOUNT             => thaw_issued_token_account::handle(accounts, &data[1..]),
        _                                            => Err(ChanceryError::UnknownInstruction.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::scope;

    #[test]
    fn mixed_counterparty_and_executor_roles_are_narrowly_pausable() {
        let roles = role::CAN_MINT_DELEGATED | role::CAN_EXECUTE_SETTLEMENT;

        assert!(is_counterparty_pause_eligible(scope::PATHWAY, roles));
        assert!(is_executor_pause_eligible(scope::PATHWAY, roles));
    }

    #[test]
    fn each_pause_surface_requires_its_target_role() {
        assert!(!is_counterparty_pause_eligible(
            scope::PATHWAY,
            role::CAN_EXECUTE_SETTLEMENT,
        ));
        assert!(!is_executor_pause_eligible(
            scope::PATHWAY,
            role::CAN_REDEEM_DIRECT,
        ));
    }

    #[test]
    fn unrelated_capabilities_remain_out_of_scope() {
        let mixed_with_governance = role::CAN_EXECUTE_SETTLEMENT | role::CAN_SET_MODULE_STATUS;

        assert!(!is_counterparty_pause_eligible(scope::PATHWAY, mixed_with_governance));
        assert!(!is_executor_pause_eligible(scope::PATHWAY, mixed_with_governance));
        assert!(!is_executor_pause_eligible(scope::GLOBAL, role::CAN_EXECUTE_SETTLEMENT));
    }
}

// Re-export from the canonical location for convenience.
pub use crate::modules::permissions::state::permission_record::PERMISSION_FLAG_PAUSED;
