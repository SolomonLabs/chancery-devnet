//! Governance/timelocked reserve-destination status transition.
//!
//! Accounts:
//!   0  chancery_config       writable  PDA
//!   1  event_authority       readable  PDA [b"event-authority"]
//!   2  pending_config_change writable  accepted pending change PDA
//!   3  reserve_destination   writable  canonical destination PDA
//!   4  governance_authority  signer

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_sysvar::Sysvar;

use crate::{
    constants::change_kind,
    error::ChanceryError,
    modules::{
        control::{
            pending_change::{
                assert_accepted_pending_change, compute_config_change_hash, consume_and_close_pending_change,
            },
        },
        core::state::chancery_config::ChanceryConfig,
        evidence::emit::{
            emit_reserve_destination_status_changed, ReserveDestinationStatusChanged,
        },
        reserve::{
            change_detection::classify_reserve_destination_status_transition,
            state::reserve_destination::ReserveDestination,
        },
    },
};

const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const PENDING:                usize = 2;
const RESERVE_DESTINATION:    usize = 3;
const GOVERNANCE_AUTHORITY:   usize = 4;
const REQUIRED_ACCOUNT_COUNT: usize = 5;

// Refund conveyance: the account whose key equals the record's stored
// rent_refund_recipient. Required whenever this instruction closes the
// record - the refund is conveyed, never redirected.
const RENT_REFUND_RECIPIENT:  usize = 5;

#[derive(BorshDeserialize)]
pub struct SetReserveDestinationStatusWithPendingArgs {
    pub new_status: u8,
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info      = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];
    let pending_account_info              = &accounts[PENDING];
    let reserve_destination_account_info  = &accounts[RESERVE_DESTINATION];
    let governance_authority_account_info = &accounts[GOVERNANCE_AUTHORITY];

    if !governance_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !pending_account_info.is_writable
        || !reserve_destination_account_info.is_writable
        || !chancery_config_account_info.is_writable
    {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;
    if governance_authority_account_info.key != &chancery_config.governance_authority {
        return Err(ChanceryError::ConfigChangeWideningRequiresGovernance.into());
    }

    let args = SetReserveDestinationStatusWithPendingArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;
    ReserveDestination::validate_status(args.new_status)?;

    let current = *ReserveDestination::load_verified(reserve_destination_account_info)?;
    if current.status == crate::constants::reserve_destination_status::DEPRECATED {
        return Err(ChanceryError::ReserveDestinationDeprecated.into());
    }

    let risk = classify_reserve_destination_status_transition(current.status, args.new_status);

    if !risk.requires_timelock() {
        return Err(ChanceryError::ConfigChangeNotAccepted.into());
    }

    let old_hash = compute_config_change_hash(
        change_kind::SET_RESERVE_DESTINATION_STATUS,
        reserve_destination_account_info.key,
        risk.as_u8(),
        &[current.status],
    );
    let new_hash = compute_config_change_hash(
        change_kind::SET_RESERVE_DESTINATION_STATUS,
        reserve_destination_account_info.key,
        risk.as_u8(),
        &[args.new_status],
    );

    let pending = assert_accepted_pending_change(
        pending_account_info,
        change_kind::SET_RESERVE_DESTINATION_STATUS,
        risk,
        reserve_destination_account_info.key,
        &old_hash,
        &new_hash,
        &chancery_config.governance_authority,
    )?;
    let change_id   = pending.change_id;
    let proposed_by = pending.proposed_by;
    drop(pending);

    {
        let mut destination = ReserveDestination::load_mut_for_verified_pda(
            reserve_destination_account_info,
            &current.asset_mint,
            &current.destination_token_account,
            current.bump,
        )?;
        destination.status      = args.new_status;
        destination.approved_by = proposed_by;
    }

    let clock = Clock::get()?;

    if accounts.len() <= RENT_REFUND_RECIPIENT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let rent_refund_recipient_account_info = &accounts[RENT_REFUND_RECIPIENT];

    consume_and_close_pending_change(pending_account_info, rent_refund_recipient_account_info)?;

    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    emit_reserve_destination_status_changed(
        event_authority_account_info,
        event_authority_bump,
        ReserveDestinationStatusChanged {
            sequence_nonce,
            chancery:            *chancery_config_account_info.key,
            slot:                clock.slot,
            unix_timestamp:      clock.unix_timestamp,
            risk_class:          risk.as_u8(),
            reserve_destination: *reserve_destination_account_info.key,
            old_status:          current.status,
            new_status:          args.new_status,
            approved_by:         proposed_by,
            change_id,
        },
    )?;

    Ok(())
}
