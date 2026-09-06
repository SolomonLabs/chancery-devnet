/// update_fee_policy_with_pending_change
///
/// Timelocked (Widening) fee-policy updates: consumes an accepted
/// PendingConfigChange. Direct/RoutineOps updates use
/// `update_fee_policy::handle`.
///
/// Accounts:
///   0  chancery_config         writable  PDA
///   1  event_authority         readable  PDA [b"event-authority"]
///   2  pending_config_change   writable  PDA
///   3  fee_policy              writable  PDA
///   4  governance_authority    signer

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    constants::change_kind,
    error::ChanceryError,
    modules::{
        control::pending_change::{assert_accepted_pending_change, consume_and_close_pending_change},
        core::state::chancery_config::ChanceryConfig,
        evidence::emit::{emit_fee_policy_updated, FeePolicyUpdated},
        fees::state::fee_policy::FeePolicy,
    },
};

use super::update_fee_policy::{
    apply_fee_policy_update, assert_fee_policy_denomination_unchanged,
    classify_fee_policy_update, fee_policy_update_hashes, UpdateFeePolicyArgs,
};

const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const PENDING_CONFIG_CHANGE:  usize = 2;
const FEE_POLICY:             usize = 3;
const GOVERNANCE_AUTHORITY:   usize = 4;
const REQUIRED_ACCOUNT_COUNT: usize = 5;

// Refund conveyance: the account whose key equals the record's stored
// rent_refund_recipient. Required whenever this instruction closes the
// record - the refund is conveyed, never redirected.
const RENT_REFUND_RECIPIENT:  usize = 5;

#[derive(BorshDeserialize)]
pub struct UpdateFeePolicyWithPendingChangeArgs {
    pub fee_policy_id:                  [u8; 32],
    pub fee_policy_flags:               Option<u64>,
    pub flat_fee_in_asset:              Option<u64>,
    pub flat_fee_in_issued_token:       Option<u64>,
    pub percent_fee_bps:                Option<u32>,
    pub fee_cap_amount:                 Option<u64>,
    pub minimum_fee_amount:             Option<u64>,
    pub rebate_flat_amount:             Option<u64>,
    pub rebate_bps:                     Option<u32>,
    pub rebate_cap_amount:              Option<u64>,
    pub net_fee_floor_zero:             Option<bool>,
    pub fee_recipient_key:              Option<Pubkey>,
    pub effective_from_unix_timestamp:  Option<i64>,
    pub effective_until_unix_timestamp: Option<i64>,
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info        = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info        = &accounts[EVENT_AUTHORITY];
    let pending_config_change_account_info  = &accounts[PENDING_CONFIG_CHANGE];
    let fee_policy_account_info             = &accounts[FEE_POLICY];
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

    if !fee_policy_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    if governance_authority_account_info.key != &chancery_config.governance_authority {
        return Err(ChanceryError::ConfigChangeWideningRequiresGovernance.into());
    }

    let local = UpdateFeePolicyWithPendingChangeArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let args = UpdateFeePolicyArgs {
        fee_policy_id:                  local.fee_policy_id,
        fee_policy_flags:               local.fee_policy_flags,
        flat_fee_in_asset:              local.flat_fee_in_asset,
        flat_fee_in_issued_token:       local.flat_fee_in_issued_token,
        percent_fee_bps:                local.percent_fee_bps,
        fee_cap_amount:                 local.fee_cap_amount,
        minimum_fee_amount:             local.minimum_fee_amount,
        rebate_flat_amount:             local.rebate_flat_amount,
        rebate_bps:                     local.rebate_bps,
        rebate_cap_amount:              local.rebate_cap_amount,
        net_fee_floor_zero:             local.net_fee_floor_zero,
        fee_recipient_key:              local.fee_recipient_key,
        effective_from_unix_timestamp:  local.effective_from_unix_timestamp,
        effective_until_unix_timestamp: local.effective_until_unix_timestamp,
    };

    let fee_policy_bump = FeePolicy::verify_pda(fee_policy_account_info, &args.fee_policy_id, &crate::id())?;

    let current      = *FeePolicy::load_for_verified_pda(
        fee_policy_account_info,
        &args.fee_policy_id,
        fee_policy_bump,
    )?;
    let mut proposed = current;

    apply_fee_policy_update(&mut proposed, &args);

    // Pending changes must satisfy the same complete policy invariants as
    // registration and routine updates. Denomination-only validation would
    // allow a retention policy to acquire a routed recipient key.
    proposed.assert_parameter_sanity()?;
    proposed.assert_denomination_consistent()?;
    assert_fee_policy_denomination_unchanged(&current, &proposed)?;

    let risk         = classify_fee_policy_update(&current, &proposed);

    if !risk.requires_timelock() {
        return Err(ChanceryError::ConfigChangeNotAccepted.into());
    }

    let (old_hash, new_hash) =
        fee_policy_update_hashes(fee_policy_account_info.key, risk, &current, &proposed);

    let pending = assert_accepted_pending_change(
        pending_config_change_account_info,
        change_kind::UPDATE_FEE_POLICY,
        risk,
        fee_policy_account_info.key,
        &old_hash,
        &new_hash,
        &chancery_config.governance_authority,
    )?;

    let change_id   = pending.change_id;
    let proposed_by = pending.proposed_by;

    *FeePolicy::load_mut_for_verified_pda(
        fee_policy_account_info,
        &args.fee_policy_id,
        fee_policy_bump,
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

    emit_fee_policy_updated(
        event_authority_account_info,
        event_authority_bump,
        FeePolicyUpdated {
            sequence_nonce,
            chancery:       *chancery_config_account_info.key,
            slot:           clock.slot,
            unix_timestamp: clock.unix_timestamp,
            risk_class:     risk.as_u8(),
            change_id,
            fee_policy:     *fee_policy_account_info.key,
            old_value_hash: old_hash,
            new_value_hash: new_hash,
            proposed_by,
            updated_by:     *governance_authority_account_info.key,
        },
    )?;

    Ok(())
}
