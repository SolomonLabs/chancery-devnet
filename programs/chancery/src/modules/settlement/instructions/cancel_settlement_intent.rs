//! cancel_settlement_intent.
//!
//! Principal-A-authorized terminal cancellation for a pending delegated or
//! trilateral intent. The PDA is closed to the rent recipient committed at
//! creation. Identical content may be recreated only when the exact historical
//! address is fresh and available. A new lifecycle should use a fresh
//! caller-selected nonce rather than depend on address recycling; no global
//! monotonically fetched counter is required (spec 14 §14.3.1).
//!
//! Accounts:
//!   0  chancery_config       writable  PDA (sequence nonce)
//!   1  event_authority       readable  PDA [b"event-authority"]
//!   2  settlement_intent     writable  PDA [b"settlement-intent", intent_id]
//!   3  principal_a           signer    cancellation authority
//!   4  rent_refund_recipient writable  exact recipient stored in the intent

use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_sysvar::Sysvar;

use crate::{
    account_security::{assert_distinct_present_accounts, assert_external_signer},
    error::ChanceryError,
    modules::{
        core::{
            instructions::close_pda_to_recipient_account,
            state::chancery_config::ChanceryConfig,
        },
        evidence::emit::{
            emit_settlement_intent_cancelled, SettlementIntentCancelled,
        },
        settlement::state::settlement_intent::SettlementIntent,
    },
};

const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const SETTLEMENT_INTENT:      usize = 2;
const PRINCIPAL_A:            usize = 3;
const RENT_REFUND_RECIPIENT:  usize = 4;
const REQUIRED_ACCOUNT_COUNT: usize = 5;

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args: &[u8]) -> ProgramResult {
    if !args.is_empty() {
        return Err(ChanceryError::ArgsDeserializationFailed.into());
    }

    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info       = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info       = &accounts[EVENT_AUTHORITY];
    let settlement_intent_account_info     = &accounts[SETTLEMENT_INTENT];
    let principal_a_account_info           = &accounts[PRINCIPAL_A];
    let rent_refund_recipient_account_info = &accounts[RENT_REFUND_RECIPIENT];

    assert_external_signer(principal_a_account_info, &crate::id())?;

    if !chancery_config_account_info.is_writable
        || !settlement_intent_account_info.is_writable
        || !rent_refund_recipient_account_info.is_writable
    {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    assert_distinct_present_accounts(&[
        chancery_config_account_info,
        event_authority_account_info,
        settlement_intent_account_info,
    ])?;

    let intent_snapshot = {
        let intent = SettlementIntent::load_verified(settlement_intent_account_info)?;
        intent.assert_pending()?;

        if &intent.principal_a != principal_a_account_info.key {
            return Err(ChanceryError::PermissionScopeMismatch.into());
        }

        *intent
    };

    let clock = Clock::get()?;

    // Once the committed expiry is reached, only the permissionless expiry
    // close path may terminate the intent. This preserves the distinct
    // cancellation-versus-expiry evidence semantics and rent-recovery route.
    if intent_snapshot.expires_at_unix_timestamp != 0
        && clock.unix_timestamp >= intent_snapshot.expires_at_unix_timestamp
    {
        return Err(ChanceryError::IntentExpired.into());
    }

    close_pda_to_recipient_account(
        settlement_intent_account_info,
        rent_refund_recipient_account_info,
        &intent_snapshot.rent_refund_recipient,
    )?;

    let mut chancery_config = ChanceryConfig::load_verified_mut(chancery_config_account_info)?;
    let event_authority_bump = chancery_config.event_authority_bump;
    let sequence_nonce = chancery_config.next_sequence_nonce()?;

    emit_settlement_intent_cancelled(
        event_authority_account_info,
        event_authority_bump,
        SettlementIntentCancelled {
            sequence_nonce,
            chancery:                    *chancery_config_account_info.key,
            slot:                        clock.slot,
            unix_timestamp:              clock.unix_timestamp,
            settlement_intent:           *settlement_intent_account_info.key,
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
            cancelled_by:                *principal_a_account_info.key,
            rent_refund_recipient:       intent_snapshot.rent_refund_recipient,
        },
    )
}
