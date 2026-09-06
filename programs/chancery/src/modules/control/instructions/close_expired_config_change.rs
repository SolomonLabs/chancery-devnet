//! close_expired_config_change.
//!
//! Permissionless terminal sweep for a `PendingConfigChange` whose expiry has
//! passed without consumption. Closes the account and refunds rent to the
//! recipient stored at propose time - the caller cannot redirect the refund,
//! only trigger it, so no signer gate is needed beyond the transaction fee
//! payer. `ConfigChangeExpired` is the canonical terminal record.
//!
//! Consume and cancel already close in-path; this instruction exists solely
//! for proposals that die of old age, keeping the live set equal to the set
//! of currently actionable proposals.
//!
//! Wire format:
//!   [ CONTROL(0x09) | CLOSE_EXPIRED_CONFIG_CHANGE(0x0D) ]  (no args)
//!
//! Accounts:
//!   0  module_activation_state    readable PDA
//!   1  chancery_config            writable PDA (sequence_nonce updated for evidence)
//!   2  event_authority            readable PDA [b"event-authority"]
//!   3  pending_config_change      writable PDA
//!
//! The final account must be the stored `rent_refund_recipient`, writable.

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
        evidence::emit::{emit_config_change_expired, ConfigChangeExpired},
    },
};

const ACTIVATION_STATE:       usize = 0;
const CHANCERY_CONFIG:        usize = 1;
const EVENT_AUTHORITY:        usize = 2;
const PENDING:                usize = 3;
const REQUIRED_ACCOUNT_COUNT: usize = 4;

// Refund conveyance: the account whose key equals the record's stored
// rent_refund_recipient. Required whenever this instruction closes the
// record - the refund is conveyed, never redirected.
const RENT_REFUND_RECIPIENT:  usize = 4;

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args: &[u8]) -> ProgramResult {
    if !args.is_empty() {
        return Err(ChanceryError::ArgsDeserializationFailed.into());
    }

    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let activation_account_info      = &accounts[ACTIVATION_STATE];
    let cfg_account_info             = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info = &accounts[EVENT_AUTHORITY];
    let pending_account_info         = &accounts[PENDING];

    if !cfg_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !pending_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let activation = ModuleActivationState::load_verified(activation_account_info)?;

    activation.assert_active(module::CONTROL)?;

    let clock = Clock::get()?;

    let (risk_class, change_id, expires_at_unix_timestamp, rent_refund_recipient) = {
        let p = PendingConfigChange::load_verified(pending_account_info)?;

        match p.status {
            config_change_status::PROPOSED | config_change_status::ACCEPTED => {}
            _ => return Err(ChanceryError::NotInitialized.into()),
        }

        if clock.unix_timestamp < p.expires_at_unix_timestamp {
            return Err(ChanceryError::ConfigChangeNotExpired.into());
        }

        (
            p.risk_class,
            p.change_id,
            p.expires_at_unix_timestamp,
            p.rent_refund_recipient,
        )
    };

    if accounts.len() <= RENT_REFUND_RECIPIENT {
        return Err(ChanceryError::MissingAccount.into());
    }
    
    let rent_refund_recipient_account_info = &accounts[RENT_REFUND_RECIPIENT];
    
    close_pda_to_recipient_account(pending_account_info, rent_refund_recipient_account_info, &rent_refund_recipient)?;

    // ── Evidence (emitted last, after all state changes) ──────────────────────
    let mut chancery_config_mut  = ChanceryConfig::load_verified_mut(cfg_account_info)?;
    let event_authority_bump = chancery_config_mut.event_authority_bump;
    let sequence_nonce       = chancery_config_mut.next_sequence_nonce()?;

    emit_config_change_expired(
        event_authority_account_info,
        event_authority_bump,
        ConfigChangeExpired {
            sequence_nonce,
            chancery:       *cfg_account_info.key,
            slot:           clock.slot,
            unix_timestamp: clock.unix_timestamp,
            risk_class,
            change_id,
            expires_at_unix_timestamp,
            rent_refund_recipient,
        },
    )?;

    Ok(())
}
