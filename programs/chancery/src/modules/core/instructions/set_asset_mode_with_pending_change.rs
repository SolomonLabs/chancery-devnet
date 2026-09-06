/// set_asset_mode_with_pending_change
///
/// Timelocked (Widening) asset-mode transitions: consumes an accepted
/// PendingConfigChange. Direct/RestrictiveImmediate transitions use
/// `set_asset_mode::handle`.
///
/// Accounts:
///   0  chancery_config         writable  PDA
///   1  event_authority         readable  PDA [b"event-authority"]
///   2  pending_config_change   writable  PDA
///   3  asset_config            writable  PDA
///   4  governance_authority    signer

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_sysvar::Sysvar;

use crate::{
    constants::{asset_mode, change_kind},
    error::ChanceryError,
    modules::{
        control::pending_change::{assert_accepted_pending_change, consume_and_close_pending_change},
        core::state::{asset_config::AssetConfig, chancery_config::ChanceryConfig},
        evidence::emit::{emit_asset_mode_changed, AssetModeChanged},
    },
};

use super::set_asset_mode::{
    asset_mode_hashes, classify_asset_mode_transition, validate_asset_mode_transition,
};

const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const PENDING_CONFIG_CHANGE:  usize = 2;
const ASSET_CONFIG:           usize = 3;
const GOVERNANCE_AUTHORITY:   usize = 4;
const REQUIRED_ACCOUNT_COUNT: usize = 5;

// Refund conveyance: the account whose key equals the record's stored
// rent_refund_recipient. Required whenever this instruction closes the
// record - the refund is conveyed, never redirected.
const RENT_REFUND_RECIPIENT:  usize = 5;

#[derive(BorshDeserialize)]
pub struct SetAssetModeWithPendingChangeArgs {
    pub new_mode: u8,
}

pub fn handle<'a>(
    accounts: &'a [AccountInfo<'a>],
    args_data: &[u8],
) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info        = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info        = &accounts[EVENT_AUTHORITY];
    let pending_config_change_account_info  = &accounts[PENDING_CONFIG_CHANGE];
    let asset_config_account_info           = &accounts[ASSET_CONFIG];
    let governance_authority_account_info   = &accounts[GOVERNANCE_AUTHORITY];

    if !governance_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !pending_config_change_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !asset_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    if governance_authority_account_info.key != &chancery_config.governance_authority {
        return Err(ChanceryError::ConfigChangeWideningRequiresGovernance.into());
    }

    let args = SetAssetModeWithPendingChangeArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let new_mode = args.new_mode;

    if new_mode != asset_mode::ACTIVE
        && new_mode != asset_mode::WIND_DOWN
        && new_mode != asset_mode::FROZEN
    {
        return Err(ChanceryError::AssetModeForbids.into());
    }

    let current_state = *AssetConfig::load_verified(asset_config_account_info)?;
    let current       = current_state.mode;

    if current == new_mode {
        return Err(ChanceryError::ConfigChangeNotAccepted.into());
    }

    validate_asset_mode_transition(current, new_mode, true, false, false)?;

    let mut proposed_state = current_state;
    proposed_state.mode    = new_mode;
    proposed_state.zero_reserved();

    let risk = classify_asset_mode_transition(current, new_mode);

    if !risk.requires_timelock() {
        return Err(ChanceryError::ConfigChangeNotAccepted.into());
    }

    let (old_hash, new_hash) = asset_mode_hashes(
        asset_config_account_info.key,
        risk,
        &current_state,
        &proposed_state,
    );

    let pending = assert_accepted_pending_change(
        pending_config_change_account_info,
        change_kind::SET_ASSET_MODE,
        risk,
        asset_config_account_info.key,
        &old_hash,
        &new_hash,
        &chancery_config.governance_authority,
    )?;

    let change_id   = pending.change_id;
    let proposed_by = pending.proposed_by;

    *AssetConfig::load_mut_for_verified_pda(
        asset_config_account_info,
        &current_state.asset_mint,
        current_state.bump,
    )? = proposed_state;

    let clock = Clock::get()?;
    drop(pending);

    if accounts.len() <= RENT_REFUND_RECIPIENT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let rent_refund_recipient_account_info = &accounts[RENT_REFUND_RECIPIENT];

    consume_and_close_pending_change(pending_config_change_account_info, rent_refund_recipient_account_info)?;

    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;


    emit_asset_mode_changed(
        event_authority_account_info,
        event_authority_bump,
        AssetModeChanged {
            sequence_nonce,
            chancery:       *chancery_config_account_info.key,
            slot:           clock.slot,
            unix_timestamp: clock.unix_timestamp,
            risk_class:     risk.as_u8(),
            change_id,
            asset_config:   *asset_config_account_info.key,
            old_mode:       current,
            new_mode,
            proposed_by,
            updated_by:     *governance_authority_account_info.key,
        },
    )?;

    Ok(())
}
