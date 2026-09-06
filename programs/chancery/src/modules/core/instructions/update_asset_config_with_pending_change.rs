/// update_asset_config_with_pending_change
///
/// Timelocked (Widening) asset-config updates: consumes an accepted
/// PendingConfigChange. Direct/RoutineOps updates use
/// `update_asset_config::handle`.
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
    constants::change_kind,
    error::ChanceryError,
    modules::{
        control::pending_change::{assert_accepted_pending_change, consume_and_close_pending_change},
        core::state::{asset_config::AssetConfig, chancery_config::ChanceryConfig},
        evidence::emit::{emit_asset_config_updated, AssetConfigUpdated},
    },
};

use super::update_asset_config::{
    asset_config_update_hashes, classify_asset_config_update, proposed_asset_config,
    UpdateAssetConfigArgs,
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
pub struct UpdateAssetConfigWithPendingChangeArgs {
    pub approved_extension_mask:           Option<[u64; 2]>,
    pub observed_extension_mask:           Option<[u64; 2]>,
    pub deposit_rate_e9:                   Option<u64>,
    pub redeem_rate_e9:                    Option<u64>,
    pub minimum_deposit_amount:            Option<u64>,
    pub minimum_redeem_amount:             Option<u64>,
    pub maximum_single_settlement_amount:  Option<u64>,
    pub forbidden_extension_mask:          Option<[u64; 2]>,
    pub required_module_mask:              Option<[u64; 2]>,
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
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

    let local = UpdateAssetConfigWithPendingChangeArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let args = UpdateAssetConfigArgs {
        approved_extension_mask:          local.approved_extension_mask,
        observed_extension_mask:          local.observed_extension_mask,
        deposit_rate_e9:                  local.deposit_rate_e9,
        redeem_rate_e9:                   local.redeem_rate_e9,
        minimum_deposit_amount:           local.minimum_deposit_amount,
        minimum_redeem_amount:            local.minimum_redeem_amount,
        maximum_single_settlement_amount: local.maximum_single_settlement_amount,
        forbidden_extension_mask:         local.forbidden_extension_mask,
        required_module_mask:             local.required_module_mask,
    };

    let current  = *AssetConfig::load_verified(asset_config_account_info)?;
    let proposed = proposed_asset_config(
        &current,
        &args,
        &chancery_config.issued_token_mint,
    )?;
    let risk     = classify_asset_config_update(&current, &proposed);

    if !risk.requires_timelock() {
        return Err(ChanceryError::ConfigChangeNotAccepted.into());
    }

    let (old_hash, new_hash) =
        asset_config_update_hashes(asset_config_account_info.key, risk, &current, &proposed);

    let pending = assert_accepted_pending_change(
        pending_config_change_account_info,
        change_kind::UPDATE_ASSET_CONFIG,
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
        &current.asset_mint,
        current.bump,
    )? = proposed;

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

    emit_asset_config_updated(
        event_authority_account_info,
        event_authority_bump,
        AssetConfigUpdated {
            sequence_nonce,
            chancery:       *chancery_config_account_info.key,
            slot:           clock.slot,
            unix_timestamp: clock.unix_timestamp,
            risk_class:     risk.as_u8(),
            change_id,
            asset_config:   *asset_config_account_info.key,
            old_value_hash: old_hash,
            new_value_hash: new_hash,
            proposed_by,
            updated_by:     *governance_authority_account_info.key,
        },
    )?;

    Ok(())
}
