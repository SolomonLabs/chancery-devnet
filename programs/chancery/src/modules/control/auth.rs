//! Module activation gating for disableable MVP modules.
//!
//! Wire convention for disableable modules (settlement, fees, limits, -):
//!   accounts[0]    module_activation_state  readable PDA
//!   accounts[1..]  instruction-specific accounts (unchanged layout)
//!
//! `dispatch_gated` verifies the PDA, classifies the exact `(module_id, ix_id)`
//! pair, and then applies the module status. Administrative/setup instructions
//! accept `ACTIVE` or `ADMIN_ONLY`; value-moving and lifecycle hot paths require
//! `ACTIVE`. Unknown instruction IDs fail before module status is consulted.

use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_program_error::ProgramError;

use crate::{
    constants::{ix, module},
    error::ChanceryError,
    modules::control::state::module_activation_state::ModuleActivationState,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstructionActivationTier {
    Admin,
    HotPath,
}

/// Classify only instructions compiled into this program version.
///
/// Returning `None` for every unknown pair prevents a future instruction ID
/// from inheriting `ADMIN_ONLY` merely because an earlier version classified
/// its whole module as administrative.
pub fn instruction_activation_tier(
    module_id: u8,
    ix_id:     u8,
) -> Option<InstructionActivationTier> {
    use InstructionActivationTier::{Admin, HotPath};

    match module_id {
        module::PERMISSIONS => match ix_id {
            ix::permissions::UPSERT_PERMISSION
            | ix::permissions::REVOKE_PERMISSION
            | ix::permissions::UPSERT_PERMISSION_WITH_PENDING_CHANGE => Some(Admin),
            _ => None,
        },

        module::PATHWAY => match ix_id {
            ix::pathway::REGISTER_PATHWAY_POLICY
            | ix::pathway::UPDATE_PATHWAY_POLICY
            | ix::pathway::SET_PATHWAY_STATUS
            | ix::pathway::SET_PATHWAY_STATUS_WITH_PENDING_CHANGE
            | ix::pathway::UPDATE_PATHWAY_POLICY_WITH_PENDING_CHANGE => Some(Admin),
            _ => None,
        },

        module::SETTLEMENT => match ix_id {
            ix::settlement::REGISTER_SETTLEMENT_POLICY => Some(Admin),
            ix::settlement::CREATE_SETTLEMENT_INTENT
            | ix::settlement::MINT_DIRECT
            | ix::settlement::REDEEM_DIRECT
            | ix::settlement::MINT_DELEGATED
            | ix::settlement::REDEEM_DELEGATED
            | ix::settlement::MINT_TRILATERAL
            | ix::settlement::REDEEM_TRILATERAL
            | ix::settlement::CLOSE_EXPIRED_SETTLEMENT_INTENT
            | ix::settlement::CANCEL_SETTLEMENT_INTENT => Some(HotPath),
            _ => None,
        },

        module::LIMITS => match ix_id {
            ix::limits::REGISTER_LIMIT_POLICY
            | ix::limits::UPDATE_LIMIT_POLICY
            | ix::limits::UPDATE_LIMIT_POLICY_WITH_PENDING_CHANGE => Some(Admin),
            _ => None,
        },

        module::FEES => match ix_id {
            ix::fees::REGISTER_FEE_POLICY
            | ix::fees::UPDATE_FEE_POLICY
            | ix::fees::UPDATE_FEE_POLICY_WITH_PENDING_CHANGE => Some(Admin),
            _ => None,
        },

        module::RESERVE => match ix_id {
            ix::reserve::WITHDRAW_RESERVE => Some(HotPath),
            ix::reserve::SET_RESERVE_DESTINATION_STATUS
            | ix::reserve::SET_RESERVE_DESTINATION_STATUS_WITH_PENDING
            | ix::reserve::REGISTER_RESERVE_DESTINATION_WITH_PENDING => Some(Admin),
            _ => None,
        },

        module::MIGRATION => match ix_id {
            ix::migration::ENABLE_LEGACY_MIGRATION => Some(Admin),
            ix::migration::MIGRATE_LEGACY_TO_TOKEN2022 => Some(HotPath),
            _ => None,
        },

        module::ISSUED_TOKEN_CONTROL => match ix_id {
            ix::issued_token_control::INITIALIZE_ISSUED_TOKEN_CONTROL
            | ix::issued_token_control::UPDATE_ISSUED_TOKEN_CONTROL
            | ix::issued_token_control::ACTIVATE_ISSUED_TOKEN_MODULE
            | ix::issued_token_control::DEACTIVATE_ISSUED_TOKEN_MODULE
            | ix::issued_token_control::SET_TRANSFER_HOOK_PROGRAM
            | ix::issued_token_control::SET_PERMANENT_DELEGATE
            | ix::issued_token_control::INITIALIZE_TOKEN_METADATA
            | ix::issued_token_control::UPDATE_TOKEN_METADATA
            | ix::issued_token_control::SET_DEFAULT_ACCOUNT_STATE
            | ix::issued_token_control::SET_TOKEN_PAUSE_STATE
            | ix::issued_token_control::CONFIGURE_CONFIDENTIAL_TRANSFER_MINT
            | ix::issued_token_control::CONFIGURE_CONFIDENTIAL_TRANSFER_ACCOUNT
            | ix::issued_token_control::REFRESH_ASSET_EXTENSION_OBSERVATION
            | ix::issued_token_control::UPDATE_ASSET_EXTENSION_POLICY
            | ix::issued_token_control::VERIFY_ISSUED_TOKEN_DEPLOYMENT
            | ix::issued_token_control::REFRESH_ISSUED_TOKEN_EXTENSION_OBSERVATION => Some(Admin),
            _ => None,
        },

        module::CROSS_CHAIN => match ix_id {
            ix::cross_chain::REGISTER_REMOTE_DOMAIN_POLICY
            | ix::cross_chain::UPDATE_REMOTE_DOMAIN_POLICY
            | ix::cross_chain::REGISTER_CROSS_CHAIN_SIGNER_SET
            | ix::cross_chain::ROTATE_CROSS_CHAIN_SIGNER_SET
            | ix::cross_chain::RESTRICT_REMOTE_DOMAIN_PAUSE
            | ix::cross_chain::RELAX_REMOTE_DOMAIN_PAUSE
            | ix::cross_chain::UPDATE_REMOTE_DOMAIN_POLICY_WITH_PENDING_CHANGE
            | ix::cross_chain::RELAX_REMOTE_DOMAIN_PAUSE_WITH_PENDING_CHANGE => Some(Admin),
            ix::cross_chain::CONSUME_INBOUND_MESSAGE
            | ix::cross_chain::EMIT_OUTBOUND_MESSAGE
            | ix::cross_chain::EXPIRE_INBOUND_MESSAGE
            | ix::cross_chain::RECLAIM_EXPIRED_OUTBOUND => Some(HotPath),
            _ => None,
        },

        _ => None,
    }
}

pub fn is_admin_tier_instruction(module_id: u8, ix_id: u8) -> bool {
    matches!(
        instruction_activation_tier(module_id, ix_id),
        Some(InstructionActivationTier::Admin)
    )
}

fn load_module_activation_state<'a>(
    activation_account_info: &'a AccountInfo<'a>,
) -> Result<core::cell::Ref<'a, ModuleActivationState>, ProgramError> {
    let expected_bump =
        ModuleActivationState::verify_pda(activation_account_info, &crate::id())?;

    ModuleActivationState::load_for_verified_pda(
        activation_account_info,
        expected_bump,
    )
}

/// Verify `module_activation_state` PDA and require `module_id` to be ACTIVE.
/// Hot-path gate - rejects `ADMIN_ONLY`.
pub fn assert_module_active<'a>(
    activation_account_info: &'a AccountInfo<'a>,
    module_id:               u8,
) -> Result<(), ProgramError> {
    let activation = load_module_activation_state(activation_account_info)?;

    activation.assert_active(module_id)
}

/// Verify the activation PDA and gate an exact compiled `(module_id, ix_id)`.
pub fn assert_module_gate<'a>(
    activation_account_info: &'a AccountInfo<'a>,
    module_id:               u8,
    ix_id:                   u8,
) -> Result<(), ProgramError> {
    let activation = load_module_activation_state(activation_account_info)?;
    let tier = instruction_activation_tier(module_id, ix_id)
        .ok_or(ChanceryError::UnknownInstruction)?;

    match tier {
        InstructionActivationTier::Admin => activation.assert_admin_or_active(module_id),
        InstructionActivationTier::HotPath => activation.assert_active(module_id),
    }
}

/// Gate on `accounts[0]`, then invoke the module dispatcher on `accounts[1..]`.
pub fn dispatch_gated<'a>(
    module_id: u8,
    accounts:  &'a [AccountInfo<'a>],
    data:      &[u8],
    inner:     fn(&'a [AccountInfo<'a>], &[u8]) -> ProgramResult,
) -> ProgramResult {
    if accounts.is_empty() {
        return Err(ChanceryError::MissingAccount.into());
    }

    if data.is_empty() {
        return Err(ChanceryError::InstructionDataTooShort.into());
    }

    assert_module_gate(&accounts[0], module_id, data[0])?;

    inner(&accounts[1..], data)
}

#[cfg(test)]
mod tests {
    use super::{
        instruction_activation_tier,
        is_admin_tier_instruction,
        InstructionActivationTier,
    };
    use crate::constants::{ix, module};

    fn assert_admin(module_id: u8, instruction_ids: &[u8]) {
        for &ix_id in instruction_ids.iter() {
            assert_eq!(
                instruction_activation_tier(module_id, ix_id),
                Some(InstructionActivationTier::Admin),
            );
            assert!(is_admin_tier_instruction(module_id, ix_id));
        }
    }

    fn assert_hot_path(module_id: u8, instruction_ids: &[u8]) {
        for &ix_id in instruction_ids.iter() {
            assert_eq!(
                instruction_activation_tier(module_id, ix_id),
                Some(InstructionActivationTier::HotPath),
            );
            assert!(!is_admin_tier_instruction(module_id, ix_id));
        }
    }

    #[test]
    fn pure_configuration_modules_enumerate_only_current_instruction_ids() {
        assert_admin(module::PERMISSIONS, &[
            ix::permissions::UPSERT_PERMISSION,
            ix::permissions::REVOKE_PERMISSION,
            ix::permissions::UPSERT_PERMISSION_WITH_PENDING_CHANGE,
        ]);
        assert_admin(module::PATHWAY, &[
            ix::pathway::REGISTER_PATHWAY_POLICY,
            ix::pathway::UPDATE_PATHWAY_POLICY,
            ix::pathway::SET_PATHWAY_STATUS,
            ix::pathway::SET_PATHWAY_STATUS_WITH_PENDING_CHANGE,
            ix::pathway::UPDATE_PATHWAY_POLICY_WITH_PENDING_CHANGE,
        ]);
        assert_admin(module::LIMITS, &[
            ix::limits::REGISTER_LIMIT_POLICY,
            ix::limits::UPDATE_LIMIT_POLICY,
            ix::limits::UPDATE_LIMIT_POLICY_WITH_PENDING_CHANGE,
        ]);
        assert_admin(module::FEES, &[
            ix::fees::REGISTER_FEE_POLICY,
            ix::fees::UPDATE_FEE_POLICY,
            ix::fees::UPDATE_FEE_POLICY_WITH_PENDING_CHANGE,
        ]);
        assert_admin(module::ISSUED_TOKEN_CONTROL, &[
            ix::issued_token_control::INITIALIZE_ISSUED_TOKEN_CONTROL,
            ix::issued_token_control::UPDATE_ISSUED_TOKEN_CONTROL,
            ix::issued_token_control::ACTIVATE_ISSUED_TOKEN_MODULE,
            ix::issued_token_control::DEACTIVATE_ISSUED_TOKEN_MODULE,
            ix::issued_token_control::SET_TRANSFER_HOOK_PROGRAM,
            ix::issued_token_control::SET_PERMANENT_DELEGATE,
            ix::issued_token_control::INITIALIZE_TOKEN_METADATA,
            ix::issued_token_control::UPDATE_TOKEN_METADATA,
            ix::issued_token_control::SET_DEFAULT_ACCOUNT_STATE,
            ix::issued_token_control::SET_TOKEN_PAUSE_STATE,
            ix::issued_token_control::CONFIGURE_CONFIDENTIAL_TRANSFER_MINT,
            ix::issued_token_control::CONFIGURE_CONFIDENTIAL_TRANSFER_ACCOUNT,
            ix::issued_token_control::REFRESH_ASSET_EXTENSION_OBSERVATION,
            ix::issued_token_control::UPDATE_ASSET_EXTENSION_POLICY,
            ix::issued_token_control::VERIFY_ISSUED_TOKEN_DEPLOYMENT,
            ix::issued_token_control::REFRESH_ISSUED_TOKEN_EXTENSION_OBSERVATION,
        ]);
    }

    #[test]
    fn mixed_modules_enumerate_admin_and_hot_path_ids() {
        assert_admin(module::SETTLEMENT, &[
            ix::settlement::REGISTER_SETTLEMENT_POLICY,
        ]);
        assert_hot_path(module::SETTLEMENT, &[
            ix::settlement::CREATE_SETTLEMENT_INTENT,
            ix::settlement::MINT_DIRECT,
            ix::settlement::REDEEM_DIRECT,
            ix::settlement::MINT_DELEGATED,
            ix::settlement::REDEEM_DELEGATED,
            ix::settlement::MINT_TRILATERAL,
            ix::settlement::REDEEM_TRILATERAL,
            ix::settlement::CLOSE_EXPIRED_SETTLEMENT_INTENT,
            ix::settlement::CANCEL_SETTLEMENT_INTENT,
        ]);

        assert_admin(module::RESERVE, &[
            ix::reserve::SET_RESERVE_DESTINATION_STATUS,
            ix::reserve::SET_RESERVE_DESTINATION_STATUS_WITH_PENDING,
            ix::reserve::REGISTER_RESERVE_DESTINATION_WITH_PENDING,
        ]);
        assert_hot_path(module::RESERVE, &[ix::reserve::WITHDRAW_RESERVE]);

        assert_admin(module::MIGRATION, &[ix::migration::ENABLE_LEGACY_MIGRATION]);
        assert_hot_path(module::MIGRATION, &[ix::migration::MIGRATE_LEGACY_TO_TOKEN2022]);

        assert_admin(module::CROSS_CHAIN, &[
            ix::cross_chain::REGISTER_REMOTE_DOMAIN_POLICY,
            ix::cross_chain::UPDATE_REMOTE_DOMAIN_POLICY,
            ix::cross_chain::REGISTER_CROSS_CHAIN_SIGNER_SET,
            ix::cross_chain::ROTATE_CROSS_CHAIN_SIGNER_SET,
            ix::cross_chain::RESTRICT_REMOTE_DOMAIN_PAUSE,
            ix::cross_chain::RELAX_REMOTE_DOMAIN_PAUSE,
            ix::cross_chain::UPDATE_REMOTE_DOMAIN_POLICY_WITH_PENDING_CHANGE,
            ix::cross_chain::RELAX_REMOTE_DOMAIN_PAUSE_WITH_PENDING_CHANGE,
        ]);
        assert_hot_path(module::CROSS_CHAIN, &[
            ix::cross_chain::CONSUME_INBOUND_MESSAGE,
            ix::cross_chain::EMIT_OUTBOUND_MESSAGE,
            ix::cross_chain::EXPIRE_INBOUND_MESSAGE,
            ix::cross_chain::RECLAIM_EXPIRED_OUTBOUND,
        ]);
    }

    #[test]
    fn arbitrary_future_instruction_ids_are_never_classified() {
        for module_id in [
            module::PERMISSIONS,
            module::PATHWAY,
            module::SETTLEMENT,
            module::LIMITS,
            module::FEES,
            module::RESERVE,
            module::MIGRATION,
            module::ISSUED_TOKEN_CONTROL,
            module::CROSS_CHAIN,
        ] {
            assert_eq!(instruction_activation_tier(module_id, 0x7F), None);
            assert_eq!(instruction_activation_tier(module_id, 0xFF), None);
        }

        assert_eq!(instruction_activation_tier(0xFE, 0x00), None);
    }
}
