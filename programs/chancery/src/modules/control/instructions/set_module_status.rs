//! set_module_status.
//!
//! Direct path. Permits RoutineOps no-ops and RestrictiveImmediate transitions. Widening+
//! routes to `set_module_status_with_pending_change`.
//!
//! Per account-list slot 0 is `module_activation_state`
//! universally - same convention as hot-path handlers.
//!
//! Wire format:
//!   [ CONTROL(0x09) | SET_MODULE_STATUS(0x0B) | borsh(args) ]
//!
//! Accounts:
//!   0  module_activation_state  writable PDA
//!   1  chancery_config          writable PDA (sequence_nonce updated for evidence)
//!   2  event_authority          readable PDA [b"event-authority"]
//!   3  authority                signer
//!     (governance|ops|emergency depending on transition target)

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_sysvar::Sysvar;

use crate::{
    constants::module_status,
    error::ChanceryError,
    modules::{
        core::state::chancery_config::ChanceryConfig,
        control::{
            module_activation_classifier::classify_module_status_transition,
            state::module_activation_state::ModuleActivationState,
        },
        evidence::emit::{emit_module_status_changed, ModuleStatusChanged},
    },
};

const ACTIVATION:             usize = 0;
const CHANCERY_CONFIG:        usize = 1;
const EVENT_AUTHORITY:        usize = 2;
const AUTHORITY:              usize = 3;
const REQUIRED_ACCOUNT_COUNT: usize = 4;

#[derive(BorshDeserialize)]
pub struct SetModuleStatusArgs {
    pub module_id:  u8,
    pub new_status: u8,
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let activation_account_info = &accounts[ACTIVATION];
    let cfg_account_info        = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info = &accounts[EVENT_AUTHORITY];
    let auth_account_info       = &accounts[AUTHORITY];

    if !auth_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !activation_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !cfg_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(cfg_account_info)?;

    let args = SetModuleStatusArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    if args.new_status > module_status::DEPRECATED {
        return Err(ChanceryError::InvalidModuleStatus.into());
    }

    ModuleActivationState::assert_known_module(args.module_id)?;
    ModuleActivationState::assert_disableable(args.module_id)?;

    let activation_bump =
        ModuleActivationState::verify_pda(activation_account_info, &crate::id())?;
    let s = ModuleActivationState::load_for_verified_pda(
        activation_account_info,
        activation_bump,
    )?;
    let old_status = s.status_for(args.module_id);

    if old_status == module_status::DEPRECATED {
        return Err(ChanceryError::ModuleDeprecated.into());
    }

    let risk = classify_module_status_transition(args.module_id, old_status, args.new_status);

    if risk.requires_timelock() {
        return Err(ChanceryError::ConfigChangeRequiresTimelock.into());
    }

    let authority_ok = match args.new_status {
        module_status::EMERGENCY_DISABLED => {
            auth_account_info.key == &chancery_config.emergency_authority
                || auth_account_info.key == &chancery_config.operations_authority
                || auth_account_info.key == &chancery_config.governance_authority
        }
        module_status::DISABLED | module_status::ADMIN_ONLY => {
            auth_account_info.key == &chancery_config.operations_authority
                || auth_account_info.key == &chancery_config.governance_authority
        }
        _ => false,
    };
    if !authority_ok {
        return Err(ChanceryError::InsufficientRole.into());
    }

    let clock = Clock::get()?;
    drop(s);

    let mut s_mut = ModuleActivationState::load_mut_for_verified_pda(
        activation_account_info,
        activation_bump,
    )?;

    if old_status != args.new_status {
        s_mut.module_statuses[args.module_id as usize] = args.new_status;
        s_mut.last_updated_by      = *auth_account_info.key;
        s_mut.last_updated_at_slot = clock.slot;
    }

    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(cfg_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    s_mut.last_event_sequence_nonce = sequence_nonce;

    emit_module_status_changed(
        event_authority_account_info,
        event_authority_bump,
        ModuleStatusChanged {
            sequence_nonce,
            chancery:       *cfg_account_info.key,
            slot:           clock.slot,
            unix_timestamp: clock.unix_timestamp,
            risk_class:     risk.as_u8(),
            module_id:      args.module_id,
            old_status,
            new_status:     args.new_status,
            changed_by:     *auth_account_info.key,
            change_id:      [0u8; 32],
        },
    )?;

    Ok(())
}
