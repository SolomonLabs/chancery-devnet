/// update_evidence_policy_with_pending_change
///
/// Governance-only consumer for evidence-policy widening transitions.
///
/// Accounts:
///   0  chancery_config         writable  PDA
///   1  event_authority         readable  PDA [b"event-authority"]
///   2  pending_config_change   writable  PDA
///   3  evidence_policy         writable  PDA [b"evidence-policy", evidence_policy_id]
///   4  governance_authority    signer
///   5  rent_refund_recipient   writable  exact recipient stored in pending change

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_sysvar::Sysvar;

use crate::{
    constants::change_kind,
    error::ChanceryError,
    modules::{
        control::pending_change::{
            assert_accepted_pending_change,
            consume_and_close_pending_change,
        },
        core::state::chancery_config::ChanceryConfig,
        evidence::{
            change_detection::classify_evidence_policy_update,
            emit::{emit_evidence_policy_updated, EvidencePolicyUpdated},
            state::evidence_policy::EvidencePolicy,
        },
    },
};

use super::update_evidence_policy::{
    apply_evidence_policy_update,
    evidence_policy_update_hashes,
    UpdateEvidencePolicyArgs,
};

const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const PENDING_CONFIG_CHANGE:  usize = 2;
const EVIDENCE_POLICY:        usize = 3;
const GOVERNANCE_AUTHORITY:   usize = 4;
const REQUIRED_ACCOUNT_COUNT: usize = 6;
const RENT_REFUND_RECIPIENT:  usize = 5;

#[derive(BorshDeserialize)]
pub struct UpdateEvidencePolicyWithPendingChangeArgs {
    pub evidence_policy_id:                 [u8; 32],
    pub required_field_mask:                Option<[u64; 2]>,
    pub counterparty_reporting_schema_hash: Option<[u8; 32]>,
    pub allow_freeform_counterparty_fields: Option<bool>,
    pub maximum_freeform_field_count:       Option<u16>,
    pub maximum_freeform_value_bytes:       Option<u16>,
    pub retention_flags:                    Option<u64>,
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info       = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info       = &accounts[EVENT_AUTHORITY];
    let pending_config_change_account_info = &accounts[PENDING_CONFIG_CHANGE];
    let evidence_policy_account_info       = &accounts[EVIDENCE_POLICY];
    let governance_authority_account_info  = &accounts[GOVERNANCE_AUTHORITY];

    if !governance_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !chancery_config_account_info.is_writable
        || !pending_config_change_account_info.is_writable
        || !evidence_policy_account_info.is_writable
    {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) =
        ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    if governance_authority_account_info.key != &chancery_config.governance_authority {
        return Err(ChanceryError::ConfigChangeWideningRequiresGovernance.into());
    }

    let local = UpdateEvidencePolicyWithPendingChangeArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;
    let args = UpdateEvidencePolicyArgs {
        evidence_policy_id:                 local.evidence_policy_id,
        required_field_mask:                local.required_field_mask,
        counterparty_reporting_schema_hash: local.counterparty_reporting_schema_hash,
        allow_freeform_counterparty_fields: local.allow_freeform_counterparty_fields,
        maximum_freeform_field_count:       local.maximum_freeform_field_count,
        maximum_freeform_value_bytes:       local.maximum_freeform_value_bytes,
        retention_flags:                    local.retention_flags,
    };

    let evidence_policy_bump = EvidencePolicy::verify_pda(
        evidence_policy_account_info,
        &args.evidence_policy_id,
        &crate::id(),
    )?;

    let current = *EvidencePolicy::load_for_verified_pda(
        evidence_policy_account_info,
        &args.evidence_policy_id,
        evidence_policy_bump,
    )?;
    let mut proposed = current;

    apply_evidence_policy_update(&mut proposed, &args);
    proposed.assert_runtime_supported()?;

    let risk = classify_evidence_policy_update(&current, &proposed);
    if !risk.requires_timelock() {
        return Err(ChanceryError::ConfigChangeNotAccepted.into());
    }

    let (old_value_hash, new_value_hash) = evidence_policy_update_hashes(
        evidence_policy_account_info.key,
        risk,
        &current,
        &proposed,
    );

    let pending = assert_accepted_pending_change(
        pending_config_change_account_info,
        change_kind::UPDATE_EVIDENCE_POLICY,
        risk,
        evidence_policy_account_info.key,
        &old_value_hash,
        &new_value_hash,
        &chancery_config.governance_authority,
    )?;
    let change_id = pending.change_id;
    let proposed_by = pending.proposed_by;

    *EvidencePolicy::load_mut_for_verified_pda(
        evidence_policy_account_info,
        &args.evidence_policy_id,
        evidence_policy_bump,
    )? = proposed;

    let clock = Clock::get()?;
    drop(pending);

    if accounts.len() <= RENT_REFUND_RECIPIENT {
        return Err(ChanceryError::MissingAccount.into());
    }

    consume_and_close_pending_change(
        pending_config_change_account_info,
        &accounts[RENT_REFUND_RECIPIENT],
    )?;

    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(
        chancery_config_account_info,
        chancery_config_verified_bump,
    )?;
    let event_authority_bump = chancery_config_mut.event_authority_bump;
    let sequence_nonce = chancery_config_mut.next_sequence_nonce()?;

    emit_evidence_policy_updated(
        event_authority_account_info,
        event_authority_bump,
        EvidencePolicyUpdated {
            sequence_nonce,
            chancery:        *chancery_config_account_info.key,
            slot:            clock.slot,
            unix_timestamp:  clock.unix_timestamp,
            risk_class:      risk.as_u8(),
            change_id,
            evidence_policy: *evidence_policy_account_info.key,
            old_value_hash,
            new_value_hash,
            proposed_by,
            updated_by:      *governance_authority_account_info.key,
        },
    )
}
