//! Direct remote-domain pause relaxation gate.
//!
//! Actual pause-bit clearing is a `Dangerous` activation/relaxation and is
//! rejected by this direct path. Use
//! `relax_remote_domain_pause_with_pending_change` after governance proposal
//! and acceptance. A semantic no-op remains accepted for idempotent tooling.
//!
//! Handler-local accounts after the module-activation prefix:
//!   0  chancery_config                writable
//!   1  event_authority                readable
//!   2  remote_domain_policy           writable
//!   3  authority                      signer
//!   4  authority_permission_record    readable  CAN_RELAX_REMOTE_DOMAIN_PAUSE @ policy

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    constants::{remote_domain_pause_bit, role, scope},
    error::ChanceryError,
    modules::{
        control::change_risk::assert_direct_config_change_allowed,
        core::state::chancery_config::ChanceryConfig,
        cross_chain::{
            auth::assert_signer_holds_role,
            change_detection::{
                classify_remote_domain_policy_change,
                RemoteDomainPolicyClassification,
                RemoteDomainPolicyValue,
            },
            instructions::{
                assert_disjoint_cross_chain_account_index_groups,
                assert_distinct_cross_chain_account_indexes,
            },
            state::remote_domain_policy::RemoteDomainPolicy,
        },
        evidence::emit::{emit_pause_state_change, PauseStateChange},
    },
};

const CHANCERY_CONFIG:               usize = 0;
const EVENT_AUTHORITY:               usize = 1;
const REMOTE_DOMAIN_POLICY:          usize = 2;
const AUTHORITY:                     usize = 3;
const AUTHORITY_PERMISSION_RECORD:   usize = 4;
const REQUIRED_ACCOUNT_COUNT:        usize = 5;

const PROTECTED_ACCOUNT_INDEXES:  &[usize] = &[
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    REMOTE_DOMAIN_POLICY,
];
const IDENTITY_ACCOUNT_INDEXES:   &[usize] = &[AUTHORITY];
const PERMISSION_ACCOUNT_INDEXES: &[usize] = &[AUTHORITY_PERMISSION_RECORD];

#[derive(BorshDeserialize)]
pub struct RelaxRemoteDomainPauseArgs {
    pub remote_chain_kind:   u8,
    pub remote_domain_id:    u64,
    pub pause_bits_to_clear: u64,
    pub reason_code:         u32,
}

pub(super) fn proposed_remote_domain_pause_relaxation(
    current:             &RemoteDomainPolicyValue,
    pause_bits_to_clear: u64,
) -> Result<(RemoteDomainPolicyValue, RemoteDomainPolicyClassification), ChanceryError> {
    let mut proposed = *current;
    proposed.pause_bits &= !(pause_bits_to_clear & remote_domain_pause_bit::ALL);
    let classification = classify_remote_domain_policy_change(current, &proposed)?;

    Ok((proposed, classification))
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    assert_distinct_cross_chain_account_indexes(accounts, PROTECTED_ACCOUNT_INDEXES)?;
    assert_disjoint_cross_chain_account_index_groups(
        accounts,
        PROTECTED_ACCOUNT_INDEXES,
        IDENTITY_ACCOUNT_INDEXES,
    )?;
    assert_disjoint_cross_chain_account_index_groups(
        accounts,
        PROTECTED_ACCOUNT_INDEXES,
        PERMISSION_ACCOUNT_INDEXES,
    )?;
    assert_disjoint_cross_chain_account_index_groups(
        accounts,
        IDENTITY_ACCOUNT_INDEXES,
        PERMISSION_ACCOUNT_INDEXES,
    )?;

    let chancery_config_account_info             = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info             = &accounts[EVENT_AUTHORITY];
    let policy_account_info                      = &accounts[REMOTE_DOMAIN_POLICY];
    let authority_account_info                   = &accounts[AUTHORITY];
    let authority_permission_record_account_info = &accounts[AUTHORITY_PERMISSION_RECORD];

    if !chancery_config_account_info.is_writable || !policy_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let args = RelaxRemoteDomainPauseArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;
    let program_id = crate::id();
    let remote_policy_bump = RemoteDomainPolicy::verify_pda(
        policy_account_info,
        args.remote_chain_kind,
        args.remote_domain_id,
        &program_id,
    )?;
    assert_signer_holds_role(
        authority_account_info,
        authority_permission_record_account_info,
        role::CAN_RELAX_REMOTE_DOMAIN_PAUSE,
        scope::REMOTE_DOMAIN,
        policy_account_info.key,
        &program_id,
    )?;

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;
    let current_policy = RemoteDomainPolicy::load_for_verified_pda(
        policy_account_info,
        args.remote_chain_kind,
        args.remote_domain_id,
        remote_policy_bump,
    )?;
    let current = RemoteDomainPolicyValue::from_policy(&current_policy);
    let (_, classification) = proposed_remote_domain_pause_relaxation(
        &current,
        args.pause_bits_to_clear,
    )?;
    assert_direct_config_change_allowed(
        classification.risk,
        &chancery_config,
        authority_account_info,
    )?;

    let clock = Clock::get()?;
    drop(current_policy);
    {
        let mut policy = RemoteDomainPolicy::load_mut_for_verified_pda(
            policy_account_info,
            args.remote_chain_kind,
            args.remote_domain_id,
            remote_policy_bump,
        )?;
        policy.updated_at_slot = clock.slot;
    }
    drop(chancery_config);

    // Read the effective bits back from stored state rather than reporting the
    // locally-computed `proposed`, so the event cannot claim a relaxation that
    // was not persisted.
    let effective_bits = {
        let policy = RemoteDomainPolicy::load_for_verified_pda(
            policy_account_info,
            args.remote_chain_kind,
            args.remote_domain_id,
            remote_policy_bump,
        )?;
        policy.status_flags & remote_domain_pause_bit::ALL
    };

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    emit_pause_state_change(
        event_authority_account_info,
        event_authority_bump,
        PauseStateChange {
            sequence_nonce,
            chancery:             *chancery_config_account_info.key,
            slot:                 clock.slot,
            unix_timestamp:       clock.unix_timestamp,
            previous_pause_bits:  current.pause_bits,
            changed_pause_bits:   current.pause_bits ^ effective_bits,
            effective_pause_bits: effective_bits,
            is_clear:             true,
            // Remote-domain pauses carry no stored auto-expiry.
            expires_at_slot:      0,
            reason_code:          args.reason_code,
            acting_authority:     *authority_account_info.key,
            activated_by:         Pubkey::default(),
            scope_kind:           scope::REMOTE_DOMAIN,
            scope_key:            *policy_account_info.key,
            subject:              Pubkey::default(),
        },
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        constants::{chain_kind, remote_domain_mode},
        modules::{
            control::change_risk::ConfigChangeRiskClass,
            cross_chain::change_detection::remote_domain_change_bit,
        },
    };

    fn base_policy() -> RemoteDomainPolicyValue {
        RemoteDomainPolicyValue {
            remote_chain_kind:             chain_kind::EVM,
            minimum_attestation_threshold: 2,
            remote_domain_id:              1,
            required_finality_depth:       12,
            message_expiry_seconds:        3_600,
            per_message_maximum:           1_000,
            per_day_maximum:               10_000,
            pause_bits:                    remote_domain_pause_bit::ALL,
            remote_domain_separator:       [1; 32],
            remote_chancery_contract:      [2; 32],
            remote_issued_token:           [3; 32],
            signer_set_id:                 [4; 32],
            mode:                          remote_domain_mode::MINT,
            remote_asset:                  [0; 32],
            local_asset_mint:              [5; 32],
        }
    }

    #[test]
    fn real_relaxation_clears_only_requested_known_bits_and_is_dangerous() {
        let current = base_policy();
        let unknown_bit = 1u64 << 63;
        let (proposed, classification) = proposed_remote_domain_pause_relaxation(
            &current,
            remote_domain_pause_bit::INBOUND_MESSAGES | unknown_bit,
        )
        .expect("pause relaxation proposal must classify");

        assert_eq!(
            proposed.pause_bits,
            current.pause_bits & !remote_domain_pause_bit::INBOUND_MESSAGES,
        );
        assert_eq!(classification.risk, ConfigChangeRiskClass::Dangerous);
        assert_eq!(
            classification.change_mask,
            remote_domain_change_bit::PAUSE_BITS,
        );

        let mut expected = current;
        expected.pause_bits = proposed.pause_bits;
        assert_eq!(proposed, expected);
    }

    #[test]
    fn already_clear_or_unknown_bits_are_a_semantic_no_op() {
        let mut current = base_policy();
        current.pause_bits = remote_domain_pause_bit::OUTBOUND_MESSAGES;
        let unknown_bit = 1u64 << 63;

        let (proposed, classification) = proposed_remote_domain_pause_relaxation(
            &current,
            remote_domain_pause_bit::INBOUND_MESSAGES | unknown_bit,
        )
        .expect("semantic no-op must classify");

        assert_eq!(proposed, current);
        assert_eq!(classification.risk, ConfigChangeRiskClass::RoutineOps);
        assert_eq!(classification.change_mask, 0);
    }

    #[test]
    fn clearing_multiple_active_pause_bits_remains_dangerous() {
        let current = base_policy();
        let bits_to_clear = remote_domain_pause_bit::INBOUND_MESSAGES
            | remote_domain_pause_bit::OUTBOUND_MESSAGES;
        let (proposed, classification) = proposed_remote_domain_pause_relaxation(
            &current,
            bits_to_clear,
        )
        .expect("multi-bit relaxation must classify");

        assert_eq!(proposed.pause_bits, current.pause_bits & !bits_to_clear);
        assert_eq!(classification.risk, ConfigChangeRiskClass::Dangerous);
    }
}
