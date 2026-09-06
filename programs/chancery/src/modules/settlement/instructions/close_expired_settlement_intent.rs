//! close_expired_settlement_intent.
//!
//! Permissionless terminal sweep for a `SettlementIntent` whose expiry has
//! passed without execution. Closes the account and refunds rent to the
//! recipient stored at creation - the caller cannot redirect the refund,
//! only trigger it. `SettlementIntentExpired` is the canonical terminal
//! record.
//!
//! Executed intents close inside the executing settlement handler; this
//! instruction exists solely for intents that die of old age, keeping the
//! live set equal to the currently pending intents.
//!
//! Wire format:
//!   [ SETTLEMENT(module byte) | CLOSE_EXPIRED_SETTLEMENT_INTENT(0x08) ]  (no args)
//!
//! Accounts:
//!   0  chancery_config       writable  PDA (sequence_nonce updated for evidence)
//!   1  event_authority       readable  PDA [b"event-authority"]
//!   2  settlement_intent     writable  PDA [b"settlement-intent", intent_id]
//!
//! The final account must be the stored `rent_refund_recipient`, writable.

use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_sysvar::Sysvar;

use crate::{
    error::ChanceryError,
    modules::{
        core::{
            instructions::close_pda_to_recipient_account,
            state::chancery_config::ChanceryConfig,
        },
        evidence::emit::{emit_settlement_intent_expired, SettlementIntentExpired},
        settlement::state::settlement_intent::SettlementIntent,
    },
};

const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const SETTLEMENT_INTENT:      usize = 2;
const REQUIRED_ACCOUNT_COUNT: usize = 3;

// Refund conveyance: the account whose key equals the record's stored
// rent_refund_recipient. Required whenever this instruction closes the
// record - the refund is conveyed, never redirected.
const RENT_REFUND_RECIPIENT:  usize = 3;

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args: &[u8]) -> ProgramResult {
    if !args.is_empty() {
        return Err(ChanceryError::ArgsDeserializationFailed.into());
    }

    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info = &accounts[EVENT_AUTHORITY];
    let intent_account_info          = &accounts[SETTLEMENT_INTENT];

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !intent_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let clock = Clock::get()?;

    let intent_snapshot = {
        let intent = SettlementIntent::load_verified(intent_account_info)?;

        // Valid terminal handlers close their intent atomically, so every live
        // account should be pending. Keep the guard explicit so corrupted or
        // future migrated state cannot be mislabeled as an expiry record.
        intent.assert_pending()?;

        // A zero expiry means no auto-expiry: such intents only leave the live
        // set through execution, and this sweep can never touch them.
        if intent.expires_at_unix_timestamp == 0
            || clock.unix_timestamp < intent.expires_at_unix_timestamp
        {
            return Err(ChanceryError::IntentNotExpired.into());
        }

        *intent
    };

    if accounts.len() <= RENT_REFUND_RECIPIENT {
        return Err(ChanceryError::MissingAccount.into());
    }
    
    let rent_refund_recipient_account_info = &accounts[RENT_REFUND_RECIPIENT];

    close_pda_to_recipient_account(
        intent_account_info,
        rent_refund_recipient_account_info,
        &intent_snapshot.rent_refund_recipient,
    )?;

    // ── Evidence (emitted last, after all state changes) ──────────────────────
    let mut chancery_config_mut  = ChanceryConfig::load_verified_mut(chancery_config_account_info)?;
    let event_authority_bump = chancery_config_mut.event_authority_bump;
    let sequence_nonce       = chancery_config_mut.next_sequence_nonce()?;

    emit_settlement_intent_expired(
        event_authority_account_info,
        event_authority_bump,
        SettlementIntentExpired {
            sequence_nonce,
            chancery:                    *chancery_config_account_info.key,
            slot:                        clock.slot,
            unix_timestamp:              clock.unix_timestamp,
            settlement_intent:           *intent_account_info.key,
            intent_id:                   intent_snapshot.intent_id,
            pathway_id:                  intent_snapshot.pathway_id,
            settlement_mode:             intent_snapshot.settlement_mode,
            settlement_action:           intent_snapshot.settlement_action,
            principal_a:                 intent_snapshot.principal_a,
            principal_b:                 intent_snapshot.principal_b,
            executor:                    intent_snapshot.executor,
            asset_mint:                  intent_snapshot.asset_mint,
            issued_token_mint:           intent_snapshot.issued_token_mint,
            asset_amount:                intent_snapshot.asset_amount,
            issued_token_amount:         intent_snapshot.issued_token_amount,
            minimum_asset_amount:        intent_snapshot.minimum_asset_amount,
            minimum_issued_token_amount: intent_snapshot.minimum_issued_token_amount,
            nonce:                       intent_snapshot.nonce,
            valid_after_unix_timestamp:  intent_snapshot.valid_after_unix_timestamp,
            expires_at_unix_timestamp:   intent_snapshot.expires_at_unix_timestamp,
            policy_id:                   intent_snapshot.policy_id,
            intent_hash:                 intent_snapshot.intent_hash,
            rent_refund_recipient:       intent_snapshot.rent_refund_recipient,
        },
    )?;

    Ok(())
}
