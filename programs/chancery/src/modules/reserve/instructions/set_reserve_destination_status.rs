//! Restrictive direct reserve-destination status transition.
//!
//! Direct execution permits only transitions classified as
//! `RestrictiveImmediate` or `RoutineOps`. Re-enablement and deprecation use
//! `set_reserve_destination_status_with_pending`.
//!
//! Accounts:
//!   0  chancery_config      writable  PDA
//!   1  event_authority      readable  PDA [b"event-authority"]
//!   2  reserve_destination  writable  canonical destination PDA
//!   3  authority            signer    governance | operations | emergency(disable only)

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_sysvar::Sysvar;

use crate::{
    constants::reserve_destination_status,
    error::ChanceryError,
    modules::{
        control::change_risk::assert_direct_config_change_allowed,
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
const RESERVE_DESTINATION:    usize = 2;
const AUTHORITY:              usize = 3;
const REQUIRED_ACCOUNT_COUNT: usize = 4;

#[derive(BorshDeserialize)]
pub struct SetReserveDestinationStatusArgs {
    pub new_status: u8,
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info     = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info     = &accounts[EVENT_AUTHORITY];
    let reserve_destination_account_info = &accounts[RESERVE_DESTINATION];
    let authority_account_info           = &accounts[AUTHORITY];

    if !authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !reserve_destination_account_info.is_writable
        || !chancery_config_account_info.is_writable
    {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;
    let args = SetReserveDestinationStatusArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    ReserveDestination::validate_status(args.new_status)?;

    let current = *ReserveDestination::load_verified(reserve_destination_account_info)?;
    if current.status == crate::constants::reserve_destination_status::DEPRECATED {
        return Err(ChanceryError::ReserveDestinationDeprecated.into());
    }

    let risk = classify_reserve_destination_status_transition(current.status, args.new_status);

    assert_direct_config_change_allowed(risk, &chancery_config, authority_account_info)?;

    let is_governance = authority_account_info.key == &chancery_config.governance_authority;
    let is_operations = authority_account_info.key == &chancery_config.operations_authority;
    let is_emergency_disable = authority_account_info.key == &chancery_config.emergency_authority
        && args.new_status == reserve_destination_status::DISABLED;

    if !(is_governance || is_operations || is_emergency_disable) {
        return Err(ChanceryError::InsufficientRole.into());
    }

    // Authorized idempotent requests do not rewrite approval attribution,
    // consume sequence numbers, or emit a false state-change event.
    if current.status == args.new_status {
        return Ok(());
    }

    {
        let mut destination     = ReserveDestination::load_mut_for_verified_pda(
            reserve_destination_account_info,
            &current.asset_mint,
            &current.destination_token_account,
            current.bump,
        )?;
        destination.status      = args.new_status;
        destination.approved_by = *authority_account_info.key;
    }

    let clock                   = Clock::get()?;
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
            approved_by:         *authority_account_info.key,
            change_id:           [0u8; 32],
        },
    )?;

    Ok(())
}
