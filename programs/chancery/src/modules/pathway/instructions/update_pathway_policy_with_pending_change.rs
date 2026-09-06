/// update_pathway_policy_with_pending_change
///
/// Timelocked (Widening) pathway-policy updates: consumes an accepted
/// PendingConfigChange. Direct/RestrictiveImmediate/RoutineOps updates use
/// `update_pathway_policy::handle`.
///
/// Accounts:
///   0  chancery_config                 writable  PDA
///   1  event_authority                 readable  PDA [b"event-authority"]
///   2  pending_config_change           writable  PDA
///   3  pathway_policy                  writable  PDA
///   4  governance_authority            signer
///   5  limit_policy                    optional
///   6  evidence_policy                 optional
///   7  fee_policy                      optional
///   8  insurance_policy                optional
///   9  asset_mint_limit_policy         optional
///  10  asset_redeem_limit_policy       optional
///  11  counterparty_limit_policy       optional
///  12  executor_limit_policy           optional

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
        control::{
            pending_change::{assert_accepted_pending_change, consume_and_close_pending_change},
        },
        core::state::chancery_config::ChanceryConfig,
        evidence::emit::{emit_pathway_policy_updated, PathwayPolicyUpdated},
        pathway::{
            reference_validation::{validate_pathway_policy_references, PathwayPolicyReferenceAccounts},
            state::pathway_policy::PathwayPolicy,
        },
    },
};

use super::update_pathway_policy::{
    apply_pathway_policy_update, classify_pathway_policy_update,
    pathway_policy_update_hashes, UpdatePathwayPolicyArgs,
};

const CHANCERY_CONFIG:           usize = 0;
const EVENT_AUTHORITY:           usize = 1;
const PENDING_CONFIG_CHANGE:     usize = 2;
const PATHWAY_POLICY:            usize = 3;
const GOVERNANCE_AUTHORITY:      usize = 4;
const LIMIT_POLICY:              usize = 5;
const EVIDENCE_POLICY:           usize = 6;
const FEE_POLICY:                usize = 7;
const INSURANCE_POLICY:          usize = 8;
const ASSET_MINT_LIMIT_POLICY:   usize = 9;
const ASSET_REDEEM_LIMIT_POLICY: usize = 10;
const COUNTERPARTY_LIMIT_POLICY: usize = 11;
const EXECUTOR_LIMIT_POLICY:     usize = 12;
const REQUIRED_ACCOUNT_COUNT:    usize = 13;

// Refund conveyance: the account whose key equals the record's stored
// rent_refund_recipient. Required whenever this instruction closes the
// record - the refund is conveyed, never redirected.
const RENT_REFUND_RECIPIENT:     usize = 13;

#[derive(BorshDeserialize)]
pub struct UpdatePathwayPolicyWithPendingChangeArgs {
    pub pathway_id:                          [u8; 32],
    pub designated_executor:                 Option<Pubkey>,
    pub limit_policy_id:                     Option<[u8; 32]>,
    pub evidence_policy_id:                  Option<[u8; 32]>,
    pub fee_policy_id:                       Option<[u8; 32]>,
    pub insurance_policy_id:                 Option<[u8; 32]>,
    pub forbidden_collateral_extension_mask: Option<[u64; 2]>,
    pub asset_mint_limit_policy_id:          Option<[u8; 32]>,
    pub asset_redeem_limit_policy_id:        Option<[u8; 32]>,
    pub counterparty_limit_policy_id:        Option<[u8; 32]>,
    pub executor_limit_policy_id:            Option<[u8; 32]>,
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info        = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info        = &accounts[EVENT_AUTHORITY];
    let pending_config_change_account_info  = &accounts[PENDING_CONFIG_CHANGE];
    let pathway_policy_account_info         = &accounts[PATHWAY_POLICY];
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

    if !pathway_policy_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    if governance_authority_account_info.key != &chancery_config.governance_authority {
        return Err(ChanceryError::ConfigChangeWideningRequiresGovernance.into());
    }

    let local = UpdatePathwayPolicyWithPendingChangeArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let args = UpdatePathwayPolicyArgs {
        pathway_id:                          local.pathway_id,
        designated_executor:                 local.designated_executor,
        limit_policy_id:                     local.limit_policy_id,
        evidence_policy_id:                  local.evidence_policy_id,
        fee_policy_id:                       local.fee_policy_id,
        insurance_policy_id:                 local.insurance_policy_id,
        forbidden_collateral_extension_mask: local.forbidden_collateral_extension_mask,
        asset_mint_limit_policy_id:          local.asset_mint_limit_policy_id,
        asset_redeem_limit_policy_id:        local.asset_redeem_limit_policy_id,
        counterparty_limit_policy_id:        local.counterparty_limit_policy_id,
        executor_limit_policy_id:            local.executor_limit_policy_id,
    };

    let pathway_policy_bump = PathwayPolicy::verify_pda(pathway_policy_account_info, &args.pathway_id, &crate::id())?;

    let current      = *PathwayPolicy::load_for_verified_pda(
        pathway_policy_account_info,
        &args.pathway_id,
        pathway_policy_bump,
    )?;
    let mut proposed = current;

    apply_pathway_policy_update(&mut proposed, &args);

    proposed.zero_reserved();
    proposed.assert_runtime_supported()?;

    let reference_accounts = PathwayPolicyReferenceAccounts {
        limit_policy:              &accounts[LIMIT_POLICY],
        evidence_policy:           &accounts[EVIDENCE_POLICY],
        fee_policy:                &accounts[FEE_POLICY],
        insurance_policy:          &accounts[INSURANCE_POLICY],
        asset_mint_limit_policy:   &accounts[ASSET_MINT_LIMIT_POLICY],
        asset_redeem_limit_policy: &accounts[ASSET_REDEEM_LIMIT_POLICY],
        counterparty_limit_policy: &accounts[COUNTERPARTY_LIMIT_POLICY],
        executor_limit_policy:     &accounts[EXECUTOR_LIMIT_POLICY],
    };
    validate_pathway_policy_references(
        pathway_policy_account_info.key,
        &proposed,
        &reference_accounts,
    )?;

    let risk         = classify_pathway_policy_update(&current, &args);

    if !risk.requires_timelock() {
        return Err(ChanceryError::ConfigChangeNotAccepted.into());
    }

    let (old_hash, new_hash) =
        pathway_policy_update_hashes(pathway_policy_account_info.key, risk, &current, &proposed);

    let pending = assert_accepted_pending_change(
        pending_config_change_account_info,
        change_kind::UPDATE_PATHWAY_POLICY,
        risk,
        pathway_policy_account_info.key,
        &old_hash,
        &new_hash,
        &chancery_config.governance_authority,
    )?;

    let change_id   = pending.change_id;
    let proposed_by = pending.proposed_by;

    *PathwayPolicy::load_mut_for_verified_pda(
        pathway_policy_account_info,
        &args.pathway_id,
        pathway_policy_bump,
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

    emit_pathway_policy_updated(
        event_authority_account_info,
        event_authority_bump,
        PathwayPolicyUpdated {
            sequence_nonce,
            chancery:       *chancery_config_account_info.key,
            slot:           clock.slot,
            unix_timestamp: clock.unix_timestamp,
            risk_class:     risk.as_u8(),
            change_id,
            pathway_policy: *pathway_policy_account_info.key,
            old_value_hash: old_hash,
            new_value_hash: new_hash,
            proposed_by,
            updated_by:     *governance_authority_account_info.key,
        },
    )?;

    Ok(())
}
