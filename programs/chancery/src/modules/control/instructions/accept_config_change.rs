//! accept_config_change.
//!
//! Flips a `PendingConfigChange` from PROPOSED to ACCEPTED. Validates timelock
//! elapsed and not expired. Governance signature required.
//!
//! Wire format:
//!   [ CONTROL(0x09) | ACCEPT_CONFIG_CHANGE(0x08) | borsh(args) ]
//!
//! Accounts:
//!   0  module_activation_state    readable PDA
//!   1  chancery_config            writable PDA (sequence_nonce updated for evidence)
//!   2  event_authority            readable PDA [b"event-authority"]
//!   3  pending_config_change      writable PDA
//!   4  governance_authority       signer

use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_sysvar::Sysvar;

use crate::{
    constants::module,
    error::ChanceryError,
    modules::{
        core::state::chancery_config::ChanceryConfig,
        control::state::{
            module_activation_state::ModuleActivationState,
            pending_config_change::{config_change_status, PendingConfigChange},
        },
        evidence::emit::{emit_config_change_accepted, ConfigChangeAccepted},
    },
};

const ACTIVATION_STATE:       usize = 0;
const CHANCERY_CONFIG:        usize = 1;
const EVENT_AUTHORITY:        usize = 2;
const PENDING:                usize = 3;
const GOVERNANCE_AUTHORITY:   usize = 4;
const REQUIRED_ACCOUNT_COUNT: usize = 5;

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args: &[u8]) -> ProgramResult {
    if !args.is_empty() {
        return Err(ChanceryError::ArgsDeserializationFailed.into());
    }

    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let activation_account_info           = &accounts[ACTIVATION_STATE];
    let cfg_account_info                  = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];
    let pending_account_info              = &accounts[PENDING];
    let governance_authority_account_info = &accounts[GOVERNANCE_AUTHORITY];

    if !governance_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !cfg_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !pending_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let activation = ModuleActivationState::load_verified(activation_account_info)?;

    activation.assert_active(module::CONTROL)?;

    let mut chancery_config = ChanceryConfig::load_verified_mut(cfg_account_info)?;

    if governance_authority_account_info.key != &chancery_config.governance_authority {
        return Err(ChanceryError::ConfigChangeWideningRequiresGovernance.into());
    }

    let clock = Clock::get()?;
    let mut p     = PendingConfigChange::load_verified_mut(pending_account_info)?;

    if p.status != config_change_status::PROPOSED {
        return Err(ChanceryError::ConfigChangeAlreadyAccepted.into());
    }

    p.assert_proposed_by_current_governance(&chancery_config.governance_authority)?;
    p.assert_timelock_elapsed(clock.unix_timestamp)?;
    p.assert_not_expired(clock.unix_timestamp)?;

    p.status           = config_change_status::ACCEPTED;
    p.accepted_at_slot = clock.slot;

    let risk_class = p.risk_class;
    let change_id  = p.change_id;

    let event_authority_bump = chancery_config.event_authority_bump;
    let sequence_nonce       = chancery_config.next_sequence_nonce()?;

    p.last_event_sequence_nonce = sequence_nonce;

    emit_config_change_accepted(
        event_authority_account_info,
        event_authority_bump,
        ConfigChangeAccepted {
            sequence_nonce,
            chancery:       *cfg_account_info.key,
            slot:           clock.slot,
            unix_timestamp: clock.unix_timestamp,
            risk_class,
            change_id,
            accepted_by:    *governance_authority_account_info.key,
        },
    )?;

    Ok(())
}
