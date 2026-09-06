//! cancel_config_change.
//!
//! Cancels a `PendingConfigChange` in PROPOSED or ACCEPTED status. Governance
//! signature required. Cancellation is terminal and closes the account,
//! refunding rent to the recipient stored at propose time (which must appear
//! as a writable account in the transaction). A closed record cannot be
//! re-cancelled: the account no longer exists. The `ConfigChangeCancelled`
//! event is the canonical terminal record.
//!
//! Wire format:
//!   [ CONTROL(0x09) | CANCEL_CONFIG_CHANGE(0x09) | borsh(args) ]
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
        control::state::{
            module_activation_state::ModuleActivationState,
            pending_config_change::{config_change_status, PendingConfigChange},
        },
        core::{
            instructions::close_pda_to_recipient_account,
            state::chancery_config::ChanceryConfig,
        },
        evidence::emit::{emit_config_change_cancelled, ConfigChangeCancelled},
    },
};

const ACTIVATION_STATE:       usize = 0;
const CHANCERY_CONFIG:        usize = 1;
const EVENT_AUTHORITY:        usize = 2;
const PENDING:                usize = 3;
const GOVERNANCE_AUTHORITY:   usize = 4;
const REQUIRED_ACCOUNT_COUNT: usize = 5;

// Refund conveyance: the account whose key equals the record's stored
// rent_refund_recipient. Required whenever this instruction closes the
// record - the refund is conveyed, never redirected.
const RENT_REFUND_RECIPIENT:  usize = 5;

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

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(cfg_account_info)?;

    if governance_authority_account_info.key != &chancery_config.governance_authority {
        return Err(ChanceryError::ConfigChangeWideningRequiresGovernance.into());
    }

    let clock = Clock::get()?;

    let (risk_class, change_id, rent_refund_recipient) = {
        let p = PendingConfigChange::load_verified_mut(pending_account_info)?;

        match p.status {
            config_change_status::PROPOSED | config_change_status::ACCEPTED => {}
            _ => return Err(ChanceryError::NotInitialized.into()),
        }

        (p.risk_class, p.change_id, p.rent_refund_recipient)
    };

    if accounts.len() <= RENT_REFUND_RECIPIENT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let rent_refund_recipient_account_info = &accounts[RENT_REFUND_RECIPIENT];

    close_pda_to_recipient_account(pending_account_info, rent_refund_recipient_account_info, &rent_refund_recipient)?;

    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(cfg_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    emit_config_change_cancelled(
        event_authority_account_info,
        event_authority_bump,
        ConfigChangeCancelled {
            sequence_nonce,
            chancery:       *cfg_account_info.key,
            slot:           clock.slot,
            unix_timestamp: clock.unix_timestamp,
            risk_class,
            change_id,
            cancelled_by:   *governance_authority_account_info.key,
        },
    )?;

    Ok(())
}
