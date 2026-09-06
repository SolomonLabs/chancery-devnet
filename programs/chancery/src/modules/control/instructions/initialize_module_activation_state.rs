//! initialize_module_activation_state.
//!
//! Bootstrap handler. Per (b): required step. Settlement
//! handlers gate on this PDA's existence; missing PDA = hard-fail.
//!
//! Wire format:
//!   [ CONTROL(0x09) | INITIALIZE_MODULE_ACTIVATION_STATE(0x0A) | borsh(empty) ]
//!
//! Accounts:
//!   0  module_activation_state    writable PDA [b"module-activation-state"]
//!   1  chancery_config            writable PDA (sequence_nonce updated for evidence)
//!   2  event_authority            readable PDA [b"event-authority"]
//!   3  payer                      signer
//!   4  governance_authority       signer    must be chancery_config.governance_authority
//!   5  system_program

use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_sysvar::Sysvar;

use crate::{
    constants::{compiled_modules, seeds},
    error::ChanceryError,
    modules::{
        core::{instructions::create_pda_account, state::chancery_config::ChanceryConfig},
        control::{
            module_activation_classifier::default_status_for,
            state::module_activation_state::{
                ModuleActivationState, MODULE_ACTIVATION_STATE_DISCRIMINATOR,
                MODULE_ACTIVATION_STATE_SIZE,
            },
        },
        evidence::emit::{emit_module_activation_state_initialized, ModuleActivationStateInitialized},
    },
};

const ACTIVATION:             usize = 0;
const CHANCERY_CONFIG:        usize = 1;
const EVENT_AUTHORITY:        usize = 2;
const PAYER:                  usize = 3;
const GOVERNANCE_AUTHORITY:   usize = 4;
const SYSTEM:                 usize = 5;
const REQUIRED_ACCOUNT_COUNT: usize = 6;

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args: &[u8]) -> ProgramResult {
    if !args.is_empty() {
        return Err(ChanceryError::ArgsDeserializationFailed.into());
    }

    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let activation_account_info           = &accounts[ACTIVATION];
    let cfg_account_info                  = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];
    let payer_account_info                = &accounts[PAYER];
    let governance_authority_account_info = &accounts[GOVERNANCE_AUTHORITY];
    let system_account_info               = &accounts[SYSTEM];

    if !payer_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !governance_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !activation_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !cfg_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(cfg_account_info)?;

    if governance_authority_account_info.key != &chancery_config.governance_authority {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    let program_id = crate::id();
    let (expected, bump) = ModuleActivationState::pda(&program_id);

    if activation_account_info.key != &expected {
        return Err(ChanceryError::InvalidPda.into());
    }

    if activation_account_info.data_is_empty() {
        create_pda_account(
            payer_account_info,
            activation_account_info,
            system_account_info,
            &program_id,
            &[seeds::MODULE_ACTIVATION_STATE, &[bump]],
            MODULE_ACTIVATION_STATE_SIZE,
        )?;
    } else if activation_account_info.data_len() != MODULE_ACTIVATION_STATE_SIZE {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    let mut s = ModuleActivationState::load_uninitialized_mut(activation_account_info)?;

    s.discriminator = MODULE_ACTIVATION_STATE_DISCRIMINATOR;
    s.version       = 1;
    s.bump          = bump;
    s._pad0         = [0u8; 5];

    let mut statuses = [0u8; 32];

    for &module_id in compiled_modules::IDS.iter() {
        statuses[module_id as usize] = default_status_for(module_id);
    }

    s.module_statuses = statuses;

    let clock = Clock::get()?;

    s.last_updated_by           = *governance_authority_account_info.key;
    s.last_updated_at_slot      = clock.slot;
    s.last_event_sequence_nonce = 0;
    s._reserved                 = [0u8; 32];

    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(cfg_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    s.last_event_sequence_nonce = sequence_nonce;

    emit_module_activation_state_initialized(
        event_authority_account_info,
        event_authority_bump,
        ModuleActivationStateInitialized {
            sequence_nonce,
            chancery:          *cfg_account_info.key,
            slot:              clock.slot,
            unix_timestamp:    clock.unix_timestamp,
            initialized_by:    *governance_authority_account_info.key,
            default_statuses:  statuses.to_vec(),
        },
    )?;

    Ok(())
}
