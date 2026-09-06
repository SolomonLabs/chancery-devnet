//! Governance/timelocked remote-domain pause relaxation.
//!
//! The pending hashes bind the full remote-domain semantic value, not only the
//! pause bits. Any direct policy mutation after proposal invalidates the
//! accepted change.
//!
//! Handler-local accounts after the module-activation prefix:
//!   0  chancery_config                writable
//!   1  event_authority                readable
//!   2  pending_config_change          writable
//!   3  remote_domain_policy           writable
//!   4  cross_chain_signer_set         readable
//!   5  relaxing_authority             signer
//!   6  governance_authority           signer
//!   7  authority_permission_record    readable  CAN_RELAX_REMOTE_DOMAIN_PAUSE @ policy

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_sysvar::Sysvar;

use crate::{
    constants::{change_kind, role, scope},
    error::ChanceryError,
    modules::{
        control::{
            pending_change::{
                assert_accepted_pending_change, compute_config_change_hash, consume_and_close_pending_change,
            },
        },
        core::state::chancery_config::ChanceryConfig,
        cross_chain::{
            auth::assert_signer_holds_role,
            change_detection::{
                assert_signer_set_compatible,
                RemoteDomainPolicyValue,
            },
            instructions::{
                assert_disjoint_cross_chain_account_index_groups,
                assert_distinct_cross_chain_account_indexes,
            },
            state::{
                cross_chain_signer_set::CrossChainSignerSet,
                remote_domain_policy::RemoteDomainPolicy,
            },
        },
        evidence::emit::{emit_remote_domain_pause_relaxed, RemoteDomainPauseRelaxed},
    },
};

use super::relax_remote_domain_pause::proposed_remote_domain_pause_relaxation;

const CHANCERY_CONFIG:              usize = 0;
const EVENT_AUTHORITY:              usize = 1;
const PENDING:                      usize = 2;
const REMOTE_DOMAIN_POLICY:         usize = 3;
const SIGNER_SET:                   usize = 4;
const RELAXING_AUTHORITY:           usize = 5;
const GOVERNANCE_AUTHORITY:         usize = 6;
const AUTHORITY_PERMISSION_RECORD:  usize = 7;
const REQUIRED_ACCOUNT_COUNT:       usize = 8;

// Refund conveyance: the account whose key equals the record's stored
// rent_refund_recipient. Required whenever this instruction closes the
// record - the refund is conveyed, never redirected.
const RENT_REFUND_RECIPIENT:        usize = 8;

const PROTECTED_ACCOUNT_INDEXES: &[usize] = &[
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    PENDING,
    REMOTE_DOMAIN_POLICY,
    SIGNER_SET,
];
const IDENTITY_ACCOUNT_INDEXES:  &[usize] = &[
    RELAXING_AUTHORITY,
    GOVERNANCE_AUTHORITY,
    RENT_REFUND_RECIPIENT,
];
const PERMISSION_ACCOUNT_INDEXES: &[usize] = &[AUTHORITY_PERMISSION_RECORD];

#[derive(BorshDeserialize)]
pub struct RelaxRemoteDomainPauseWithPendingChangeArgs {
    pub remote_chain_kind:   u8,
    pub remote_domain_id:    u64,
    pub pause_bits_to_clear: u64,
    pub reason_code:         u32,
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
    let pending_account_info                     = &accounts[PENDING];
    let policy_account_info                      = &accounts[REMOTE_DOMAIN_POLICY];
    let signer_set_account_info                  = &accounts[SIGNER_SET];
    let relaxing_authority_account_info          = &accounts[RELAXING_AUTHORITY];
    let governance_authority_account_info        = &accounts[GOVERNANCE_AUTHORITY];
    let authority_permission_record_account_info = &accounts[AUTHORITY_PERMISSION_RECORD];

    if !relaxing_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }
    if !governance_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }
    if !chancery_config_account_info.is_writable
        || !pending_account_info.is_writable
        || !policy_account_info.is_writable
    {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;
    if governance_authority_account_info.key != &chancery_config.governance_authority {
        return Err(ChanceryError::ConfigChangeWideningRequiresGovernance.into());
    }

    let args = RelaxRemoteDomainPauseWithPendingChangeArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;
    let program_id = crate::id();
    let remote_policy_bump = RemoteDomainPolicy::verify_pda(
        policy_account_info,
        args.remote_chain_kind,
        args.remote_domain_id,
        &program_id,
    )?;
    assert_signer_holds_role(
        relaxing_authority_account_info,
        authority_permission_record_account_info,
        role::CAN_RELAX_REMOTE_DOMAIN_PAUSE,
        scope::REMOTE_DOMAIN,
        policy_account_info.key,
        &program_id,
    )?;

    let current_policy = RemoteDomainPolicy::load_for_verified_pda(
        policy_account_info,
        args.remote_chain_kind,
        args.remote_domain_id,
        remote_policy_bump,
    )?;
    let current = RemoteDomainPolicyValue::from_policy(&current_policy);
    let (proposed, classification) = proposed_remote_domain_pause_relaxation(
        &current,
        args.pause_bits_to_clear,
    )?;
    if !classification.risk.requires_timelock() {
        return Err(ChanceryError::ConfigChangeNotAccepted.into());
    }

    let clock = Clock::get()?;
    let signer_set = CrossChainSignerSet::load_verified(signer_set_account_info)?;
    assert_signer_set_compatible(&proposed, &signer_set, clock.unix_timestamp, true)?;
    let old_hash = compute_config_change_hash(
        change_kind::RELAX_REMOTE_DOMAIN_PAUSE,
        policy_account_info.key,
        classification.risk.as_u8(),
        &current.to_payload(),
    );
    let new_hash = compute_config_change_hash(
        change_kind::RELAX_REMOTE_DOMAIN_PAUSE,
        policy_account_info.key,
        classification.risk.as_u8(),
        &proposed.to_payload(),
    );
    let pending = assert_accepted_pending_change(
        pending_account_info,
        change_kind::RELAX_REMOTE_DOMAIN_PAUSE,
        classification.risk,
        policy_account_info.key,
        &old_hash,
        &new_hash,
        &chancery_config.governance_authority,
    )?;
    let change_id = pending.change_id;
    let proposed_by = pending.proposed_by;

    drop(pending);
    drop(signer_set);
    drop(current_policy);
    {
        let mut policy = RemoteDomainPolicy::load_mut_for_verified_pda(
            policy_account_info,
            args.remote_chain_kind,
            args.remote_domain_id,
            remote_policy_bump,
        )?;
        policy.relax_pause_bits(args.pause_bits_to_clear);
        policy.updated_at_slot = clock.slot;
    }

    if accounts.len() <= RENT_REFUND_RECIPIENT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let rent_refund_recipient_account_info = &accounts[RENT_REFUND_RECIPIENT];

    consume_and_close_pending_change(pending_account_info, rent_refund_recipient_account_info)?;

    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    emit_remote_domain_pause_relaxed(
        event_authority_account_info,
        event_authority_bump,
        RemoteDomainPauseRelaxed {
            sequence_nonce,
            chancery:             *chancery_config_account_info.key,
            slot:                 clock.slot,
            unix_timestamp:       clock.unix_timestamp,
            risk_class:           classification.risk.as_u8(),
            change_id,
            remote_domain_policy: *policy_account_info.key,
            old_pause_bits:       current.pause_bits,
            new_pause_bits:       proposed.pause_bits,
            reason_code:          args.reason_code,
            old_value_hash:       old_hash,
            new_value_hash:       new_hash,
            proposed_by,
            relaxed_by:           *relaxing_authority_account_info.key,
        },
    )?;

    Ok(())
}
