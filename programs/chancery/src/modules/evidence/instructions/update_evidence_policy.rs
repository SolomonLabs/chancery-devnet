/// update_evidence_policy
///
/// Direct updates are restricted to no-op or policy-tightening transitions.
/// Any widening transition must be executed by governance through
/// `update_evidence_policy_with_pending_change`.
///
/// Accounts:
///   0  chancery_config       writable  PDA - sequence nonce
///   1  event_authority       readable  PDA [b"event-authority"]
///   2  evidence_policy       writable  PDA [b"evidence-policy", evidence_policy_id]
///   3  operations_authority  signer

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
            change_risk::{assert_direct_config_change_allowed, ConfigChangeRiskClass},
            pending_change::compute_config_change_hash,
        },
        core::state::chancery_config::ChanceryConfig,
        evidence::{
            change_detection::classify_evidence_policy_update,
            emit::{emit_evidence_policy_updated, EvidencePolicyUpdated},
            state::evidence_policy::EvidencePolicy,
        },
    },
};

const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const EVIDENCE_POLICY:        usize = 2;
const OPERATIONS_AUTHORITY:   usize = 3;
const REQUIRED_ACCOUNT_COUNT: usize = 4;

#[derive(BorshDeserialize)]
pub struct UpdateEvidencePolicyArgs {
    pub evidence_policy_id:                 [u8; 32],
    pub required_field_mask:                Option<[u64; 2]>,
    pub counterparty_reporting_schema_hash: Option<[u8; 32]>,
    pub allow_freeform_counterparty_fields: Option<bool>,
    pub maximum_freeform_field_count:       Option<u16>,
    pub maximum_freeform_value_bytes:       Option<u16>,
    pub retention_flags:                    Option<u64>,
}

pub(super) fn apply_evidence_policy_update(
    policy: &mut EvidencePolicy,
    args:   &UpdateEvidencePolicyArgs,
) {
    if let Some(value) = args.required_field_mask {
        policy.required_field_mask = value;
    }

    if let Some(value) = args.counterparty_reporting_schema_hash {
        policy.counterparty_reporting_schema_hash = value;
    }

    if let Some(value) = args.allow_freeform_counterparty_fields {
        policy.allow_freeform_counterparty_fields = value as u8;
    }

    if let Some(value) = args.maximum_freeform_field_count {
        policy.maximum_freeform_field_count = value;
    }

    if let Some(value) = args.maximum_freeform_value_bytes {
        policy.maximum_freeform_value_bytes = value;
    }

    if let Some(value) = args.retention_flags {
        policy.retention_flags = value;
    }
}

pub(super) fn evidence_policy_update_hashes(
    evidence_policy_key: &Pubkey,
    risk:                ConfigChangeRiskClass,
    current:             &EvidencePolicy,
    proposed:            &EvidencePolicy,
) -> ([u8; 32], [u8; 32]) {
    let current_payload = current.config_change_payload();
    let proposed_payload = proposed.config_change_payload();

    let old_hash = compute_config_change_hash(
        change_kind::UPDATE_EVIDENCE_POLICY,
        evidence_policy_key,
        risk.as_u8(),
        &current_payload,
    );
    let new_hash = compute_config_change_hash(
        change_kind::UPDATE_EVIDENCE_POLICY,
        evidence_policy_key,
        risk.as_u8(),
        &proposed_payload,
    );

    (old_hash, new_hash)
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info      = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];
    let evidence_policy_account_info      = &accounts[EVIDENCE_POLICY];
    let operations_authority_account_info = &accounts[OPERATIONS_AUTHORITY];

    if !operations_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !chancery_config_account_info.is_writable
        || !evidence_policy_account_info.is_writable
    {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) =
        ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    if operations_authority_account_info.key != &chancery_config.operations_authority {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    let args = UpdateEvidencePolicyArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

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
    assert_direct_config_change_allowed(
        risk,
        &chancery_config,
        operations_authority_account_info,
    )?;

    let (old_value_hash, new_value_hash) = evidence_policy_update_hashes(
        evidence_policy_account_info.key,
        risk,
        &current,
        &proposed,
    );

    *EvidencePolicy::load_mut_for_verified_pda(
        evidence_policy_account_info,
        &args.evidence_policy_id,
        evidence_policy_bump,
    )? = proposed;

    let clock = Clock::get()?;
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
            change_id:       [0u8; 32],
            evidence_policy: *evidence_policy_account_info.key,
            old_value_hash,
            new_value_hash,
            proposed_by:     *operations_authority_account_info.key,
            updated_by:      *operations_authority_account_info.key,
        },
    )
}

#[cfg(test)]
mod tests {
    use bytemuck::Zeroable;

    use super::*;
    use crate::modules::evidence::state::evidence_policy::required_field;

    fn policy() -> EvidencePolicy {
        let mut policy = EvidencePolicy::zeroed();
        policy.evidence_policy_id = [0x11; 32];
        policy
    }

    #[test]
    fn apply_update_preserves_identity_and_updates_mutable_fields() {
        let current = policy();
        let args = UpdateEvidencePolicyArgs {
            evidence_policy_id:                 current.evidence_policy_id,
            required_field_mask:                Some([required_field::PATHWAY_ID, 0]),
            counterparty_reporting_schema_hash: Some([0u8; 32]),
            allow_freeform_counterparty_fields: Some(false),
            maximum_freeform_field_count:       Some(0),
            maximum_freeform_value_bytes:       Some(0),
            retention_flags:                    Some(0),
        };
        let mut proposed = current;

        apply_evidence_policy_update(&mut proposed, &args);

        assert_eq!(proposed.evidence_policy_id, current.evidence_policy_id);
        assert_eq!(proposed.required_field_mask, [required_field::PATHWAY_ID, 0]);
    }

    #[test]
    fn update_hashes_bind_risk_and_semantic_payload() {
        let current = policy();
        let mut proposed = current;
        proposed.required_field_mask = [required_field::PATHWAY_ID, 0];
        let key = Pubkey::new_unique();

        let restrictive = evidence_policy_update_hashes(
            &key,
            ConfigChangeRiskClass::RestrictiveImmediate,
            &current,
            &proposed,
        );
        let widening = evidence_policy_update_hashes(
            &key,
            ConfigChangeRiskClass::Widening,
            &current,
            &proposed,
        );

        assert_ne!(restrictive, widening);
    }
}
