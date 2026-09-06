//! Governance/timelocked remote-domain policy update.
//!
//! Handler-local accounts after the module-activation prefix:
//!   0  chancery_config                writable
//!   1  event_authority                readable
//!   2  pending_config_change          writable
//!   3  remote_domain_policy           writable
//!   4  cross_chain_signer_set         readable  proposed signer-set reference
//!   5  updating_authority             signer
//!   6  governance_authority           signer
//!   7  authority_permission_record    readable  CAN_UPDATE_REMOTE_DOMAIN_POLICY @ policy
//!
//! Binding-time window accounts, required when the post-update policy has
//! per_day_maximum != 0 and the corridor's stable daily window may not exist:
//!   8  payer                          signer    rent for a newly bound window
//!   9  system_program                 readable
//!  10  remote_daily_usage_window      writable  PDA [b"usage-window", sha256(0x05 || policy PDA), DAILY]

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    constants::{change_kind, role, scope, window_kind},
    error::ChanceryError,
    modules::{
        control::{
            pending_change::{assert_accepted_pending_change, consume_and_close_pending_change},
        },
        core::state::chancery_config::ChanceryConfig,
        limits::state::usage_window::{create_usage_window_if_missing, UsageWindow},
        cross_chain::{
            usage::remote_domain_usage_window_scope_hash,
            auth::assert_signer_holds_role,
            change_detection::{
                assert_signer_set_compatible, classify_remote_domain_policy_change,
                RemoteDomainPolicyValue,
            },
            instructions::{
                assert_disjoint_cross_chain_account_index_groups,
                assert_distinct_cross_chain_account_indexes,
            },
            instructions::update_remote_domain_policy::{
                proposed_remote_domain_policy_value, remote_domain_policy_update_hashes,
                write_remote_domain_policy_value, UpdateRemoteDomainPolicyArgs,
            },
            state::{
                cross_chain_signer_set::CrossChainSignerSet,
                remote_domain_policy::RemoteDomainPolicy,
            },
        },
        evidence::emit::{emit_remote_domain_policy_updated, emit_usage_window_initialized,
            RemoteDomainPolicyUpdated, UsageWindowInitialized},
    },
};

const CHANCERY_CONFIG:               usize = 0;
const EVENT_AUTHORITY:               usize = 1;
const PENDING:                       usize = 2;
const REMOTE_DOMAIN_POLICY:          usize = 3;
const SIGNER_SET:                    usize = 4;
const UPDATING_AUTHORITY:            usize = 5;
const GOVERNANCE_AUTHORITY:          usize = 6;
const AUTHORITY_PERMISSION_RECORD:   usize = 7;
const PAYER:                         usize = 8;
const SYSTEM_PROGRAM:                usize = 9;
const REMOTE_DAILY_USAGE_WINDOW:     usize = 10;
const REQUIRED_ACCOUNT_COUNT:        usize = 10;

// Refund conveyance: the account whose key equals the record's stored
// rent_refund_recipient. Required whenever this instruction closes the
// record - the refund is conveyed, never redirected.
const RENT_REFUND_RECIPIENT:         usize = 11;

const PROTECTED_ACCOUNT_INDEXES:  &[usize] = &[
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    PENDING,
    REMOTE_DOMAIN_POLICY,
    SIGNER_SET,
    SYSTEM_PROGRAM,
    REMOTE_DAILY_USAGE_WINDOW,
];
const IDENTITY_ACCOUNT_INDEXES:   &[usize] = &[
    UPDATING_AUTHORITY,
    GOVERNANCE_AUTHORITY,
    PAYER,
    RENT_REFUND_RECIPIENT,
];
const PERMISSION_ACCOUNT_INDEXES: &[usize] = &[AUTHORITY_PERMISSION_RECORD];

#[derive(BorshDeserialize)]
pub struct UpdateRemoteDomainPolicyWithPendingChangeArgs {
    pub remote_chain_kind:             u8,
    pub remote_domain_id:              u64,
    pub minimum_attestation_threshold: Option<u8>,
    /// Attestor-SOP minimum remote confirmation depth; not an on-chain
    /// consume predicate because Solana cannot observe the remote chain.
    pub required_finality_depth:       Option<u64>,
    pub message_expiry_seconds:        Option<u64>,
    pub per_message_maximum:           Option<u64>,
    pub per_day_maximum:               Option<u64>,
    pub remote_domain_separator:       Option<[u8; 32]>,
    pub remote_chancery_contract:      Option<[u8; 32]>,
    pub remote_issued_token:           Option<[u8; 32]>,
    pub signer_set_id:                 Option<[u8; 32]>,
    pub mode:                          Option<u8>,
    pub remote_asset:                  Option<[u8; 32]>,
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
    let updating_authority_account_info          = &accounts[UPDATING_AUTHORITY];
    let governance_authority_account_info        = &accounts[GOVERNANCE_AUTHORITY];
    let authority_permission_record_account_info = &accounts[AUTHORITY_PERMISSION_RECORD];

    if !updating_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }
    if !governance_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }
    if !pending_account_info.is_writable
        || !policy_account_info.is_writable
        || !chancery_config_account_info.is_writable
    {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;
    if governance_authority_account_info.key != &chancery_config.governance_authority {
        return Err(ChanceryError::ConfigChangeWideningRequiresGovernance.into());
    }

    let local = UpdateRemoteDomainPolicyWithPendingChangeArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;
    let args = UpdateRemoteDomainPolicyArgs {
        remote_chain_kind: local.remote_chain_kind,
        remote_domain_id: local.remote_domain_id,
        minimum_attestation_threshold: local.minimum_attestation_threshold,
        required_finality_depth: local.required_finality_depth,
        message_expiry_seconds: local.message_expiry_seconds,
        per_message_maximum: local.per_message_maximum,
        per_day_maximum: local.per_day_maximum,
        remote_domain_separator: local.remote_domain_separator,
        remote_chancery_contract: local.remote_chancery_contract,
        remote_issued_token: local.remote_issued_token,
        signer_set_id: local.signer_set_id,
        mode: local.mode,
        remote_asset: local.remote_asset,
    };
    let program_id = crate::id();

    let remote_policy_bump = RemoteDomainPolicy::verify_pda(
        policy_account_info,
        args.remote_chain_kind,
        args.remote_domain_id,
        &program_id,
    )?;
    assert_signer_holds_role(
        updating_authority_account_info,
        authority_permission_record_account_info,
        role::CAN_UPDATE_REMOTE_DOMAIN_POLICY,
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
    let proposed = proposed_remote_domain_policy_value(&current, &args)?;
    let classification = classify_remote_domain_policy_change(&current, &proposed)?;
    if !classification.risk.requires_timelock() {
        return Err(ChanceryError::ConfigChangeNotAccepted.into());
    }

    let clock = Clock::get()?;
    let signer_set = CrossChainSignerSet::load_verified(signer_set_account_info)?;
    assert_signer_set_compatible(&proposed, &signer_set, clock.unix_timestamp, true)?;
    let (old_hash, new_hash) = remote_domain_policy_update_hashes(
        policy_account_info.key,
        classification,
        &current,
        &proposed,
    );
    let pending = assert_accepted_pending_change(
        pending_account_info,
        change_kind::UPDATE_REMOTE_DOMAIN_POLICY,
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
        write_remote_domain_policy_value(&mut policy, proposed, clock.slot);
    }

    if accounts.len() <= RENT_REFUND_RECIPIENT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let rent_refund_recipient_account_info = &accounts[RENT_REFUND_RECIPIENT];

    consume_and_close_pending_change(pending_account_info, rent_refund_recipient_account_info)?;

    drop(chancery_config);

    // ── Binding-time corridor daily window (widening that enables the cap) ────
    let mut initialized_window: Option<(Pubkey, [u8; 32], i64)> = None;

    if proposed.per_day_maximum != 0 {
        if accounts.len() <= REMOTE_DAILY_USAGE_WINDOW {
            return Err(ChanceryError::MissingAccount.into());
        }

        let payer_account_info          = &accounts[PAYER];
        let system_program_account_info = &accounts[SYSTEM_PROGRAM];

        if !payer_account_info.is_signer {
            return Err(ChanceryError::AccountNotSigner.into());
        }

        let window_scope_hash =
            remote_domain_usage_window_scope_hash(policy_account_info.key);

        let created = create_usage_window_if_missing(
            &accounts[REMOTE_DAILY_USAGE_WINDOW],
            payer_account_info,
            system_program_account_info,
            &window_scope_hash,
            window_kind::DAILY,
            clock.unix_timestamp,
            payer_account_info.key,
            &crate::id(),
        )?;

        if created {
            let window = UsageWindow::load_verified(&accounts[REMOTE_DAILY_USAGE_WINDOW])?;
            initialized_window = Some((
                *accounts[REMOTE_DAILY_USAGE_WINDOW].key,
                window_scope_hash,
                window.window_start_unix_timestamp,
            ));
        }
    }

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    emit_remote_domain_policy_updated(
        event_authority_account_info,
        event_authority_bump,
        RemoteDomainPolicyUpdated {
            sequence_nonce,
            chancery:             *chancery_config_account_info.key,
            slot:                 clock.slot,
            unix_timestamp:       clock.unix_timestamp,
            risk_class:           classification.risk.as_u8(),
            change_id,
            remote_domain_policy: *policy_account_info.key,
            remote_chain_kind:    proposed.remote_chain_kind,
            remote_domain_id:     proposed.remote_domain_id,
            signer_set_id:        proposed.signer_set_id,
            old_value_hash:       old_hash,
            new_value_hash:       new_hash,
            change_mask:          classification.change_mask,
            proposed_by,
            updated_by:           *updating_authority_account_info.key,
        },
    )?;

    if let Some((window, window_scope_hash, window_start)) = initialized_window {
        let window_sequence_nonce = chancery_config_mut.next_sequence_nonce()?;

        emit_usage_window_initialized(
            event_authority_account_info,
            event_authority_bump,
            UsageWindowInitialized {
                sequence_nonce:              window_sequence_nonce,
                chancery:                    *chancery_config_account_info.key,
                slot:                        clock.slot,
                unix_timestamp:              clock.unix_timestamp,
                risk_class:                  classification.risk.as_u8(),
                usage_window:                window,
                scope_hash:                  window_scope_hash,
                window_kind:                 window_kind::DAILY,
                window_start_unix_timestamp: window_start,
                rent_refund_recipient:       *accounts[PAYER].key,
                created_by:                  *updating_authority_account_info.key,
            },
        )?;
    }

    Ok(())
}
