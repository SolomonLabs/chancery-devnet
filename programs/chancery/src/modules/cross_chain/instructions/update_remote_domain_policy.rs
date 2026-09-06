//! Direct containment-only remote-domain policy update.
//!
//! Handler-local accounts after the module-activation prefix:
//!   0  chancery_config                writable
//!   1  event_authority                readable
//!   2  remote_domain_policy           writable
//!   3  authority                      signer
//!   4  authority_permission_record    readable  CAN_UPDATE_REMOTE_DOMAIN_POLICY @ policy
//!   5  cross_chain_signer_set         readable  proposed signer-set reference
//!
//! Any trust/economic widening returns `ConfigChangeRequiresTimelock` and must
//! use `update_remote_domain_policy_with_pending_change`.

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_sysvar::Sysvar;

use crate::{
    constants::{change_kind, role, scope, status_flag},
    error::ChanceryError,
    modules::{
        control::{
            change_risk::assert_direct_config_change_allowed,
            pending_change::compute_config_change_hash,
        },
        core::state::chancery_config::ChanceryConfig,
        cross_chain::{
            auth::assert_signer_holds_role,
            instructions::{
                assert_disjoint_cross_chain_account_index_groups,
                assert_distinct_cross_chain_account_indexes,
            },
            usage::remote_domain_usage_window_scope_hash,
            change_detection::{
                assert_remote_domain_policy_value_valid, assert_signer_set_compatible,
                classify_remote_domain_policy_change, RemoteDomainPolicyClassification,
                RemoteDomainPolicyValue,
            },
            state::{
                cross_chain_signer_set::CrossChainSignerSet,
                remote_domain_policy::RemoteDomainPolicy,
            },
        },
        evidence::emit::{
            emit_remote_domain_policy_updated, emit_usage_window_initialized,
            RemoteDomainPolicyUpdated, UsageWindowInitialized,
        },
    },
};

const CHANCERY_CONFIG:               usize = 0;
const EVENT_AUTHORITY:               usize = 1;
const REMOTE_DOMAIN_POLICY:          usize = 2;
const AUTHORITY:                     usize = 3;
const AUTHORITY_PERMISSION_RECORD:   usize = 4;
const SIGNER_SET:                    usize = 5;
const REQUIRED_ACCOUNT_COUNT:        usize = 8;

// Binding accounts. payer/system_program are always present (payer signs);
// the windows are optional in slot-order, required iff this update enables
// the per-day cap (0 -> finite) and the corridor's stable daily window does
// not exist yet: enabling a cap is a binding surface, and an enforced cap
// without its accumulator would fail the corridor closed forever.
const PAYER:                         usize = 6;
const SYSTEM_PROGRAM:                usize = 7;
const REMOTE_DAILY_USAGE_WINDOW:     usize = 8;

const PROTECTED_ACCOUNT_INDEXES:  &[usize] = &[
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    REMOTE_DOMAIN_POLICY,
    SIGNER_SET,
    SYSTEM_PROGRAM,
    REMOTE_DAILY_USAGE_WINDOW,
];
const IDENTITY_ACCOUNT_INDEXES:   &[usize] = &[AUTHORITY, PAYER];
const PERMISSION_ACCOUNT_INDEXES: &[usize] = &[AUTHORITY_PERMISSION_RECORD];

#[derive(BorshDeserialize)]
pub struct UpdateRemoteDomainPolicyArgs {
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

pub(crate) fn proposed_remote_domain_policy_value(
    current: &RemoteDomainPolicyValue,
    args:    &UpdateRemoteDomainPolicyArgs,
) -> Result<RemoteDomainPolicyValue, ChanceryError> {
    if current.remote_chain_kind != args.remote_chain_kind
        || current.remote_domain_id != args.remote_domain_id
    {
        return Err(ChanceryError::ImmutableFieldChange);
    }

    // H-02: every field that feeds historical message-hash reconstruction or
    // historical asset binding is IMMUTABLE for the life of the corridor.
    // Handlers rebuild the canonical preimage of in-flight and terminally retired messages
    // from CURRENT policy state; mutating these fields in place would strand
    // the strict inbound nonce head (unconsumable AND unverifiable for retirement - the
    // quorum signed the old reconstruction), stall every later nonce, and
    // break the epoch-free content hash that post-retirement reclaims (spec 15)
    // depend on. A timelock only delays that outcome; it does not preserve
    // the historical verification data. Corridor replacement (new daughter
    // deployment, new domain separator, new issued token, new mode/remote
    // asset/local asset mint) =
    // register a NEW remote domain policy under a fresh domain id - fresh
    // PDA, fresh nonces - and pause/drain this one. `signer_set_id` rotation
    // remains legal: it is deliberately OUTSIDE the epoch-free hash (§15.4),
    // and rotation-stranded wire hashes are recoverable via expiry+reclaim.
    // Idempotent `Some(current_value)` is tolerated as a no-op for tooling.
    if args.remote_domain_separator.is_some_and(|v| v != current.remote_domain_separator)
        || args.remote_chancery_contract.is_some_and(|v| v != current.remote_chancery_contract)
        || args.remote_issued_token.is_some_and(|v| v != current.remote_issued_token)
        || args.mode.is_some_and(|v| v != current.mode)
        || args.remote_asset.is_some_and(|v| v != current.remote_asset)
    {
        return Err(ChanceryError::ImmutableFieldChange);
    }

    let proposed = RemoteDomainPolicyValue {
        remote_chain_kind: current.remote_chain_kind,
        minimum_attestation_threshold: args
            .minimum_attestation_threshold
            .unwrap_or(current.minimum_attestation_threshold),
        remote_domain_id: current.remote_domain_id,
        required_finality_depth: args
            .required_finality_depth
            .unwrap_or(current.required_finality_depth),
        message_expiry_seconds: args
            .message_expiry_seconds
            .unwrap_or(current.message_expiry_seconds),
        per_message_maximum: args
            .per_message_maximum
            .unwrap_or(current.per_message_maximum),
        per_day_maximum: args
            .per_day_maximum
            .unwrap_or(current.per_day_maximum),
        pause_bits: current.pause_bits,
        remote_domain_separator: args
            .remote_domain_separator
            .unwrap_or(current.remote_domain_separator),
        remote_chancery_contract: args
            .remote_chancery_contract
            .unwrap_or(current.remote_chancery_contract),
        remote_issued_token: args
            .remote_issued_token
            .unwrap_or(current.remote_issued_token),
        signer_set_id: args.signer_set_id.unwrap_or(current.signer_set_id),
        mode: args.mode.unwrap_or(current.mode),
        remote_asset: args.remote_asset.unwrap_or(current.remote_asset),
        local_asset_mint: current.local_asset_mint,
    };

    assert_remote_domain_policy_value_valid(&proposed)?;
    Ok(proposed)
}

pub(super) fn remote_domain_policy_update_hashes(
    target:         &solana_pubkey::Pubkey,
    classification: RemoteDomainPolicyClassification,
    current:        &RemoteDomainPolicyValue,
    proposed:       &RemoteDomainPolicyValue,
) -> ([u8; 32], [u8; 32]) {
    let old_hash = compute_config_change_hash(
        change_kind::UPDATE_REMOTE_DOMAIN_POLICY,
        target,
        classification.risk.as_u8(),
        &current.to_payload(),
    );
    let new_hash = compute_config_change_hash(
        change_kind::UPDATE_REMOTE_DOMAIN_POLICY,
        target,
        classification.risk.as_u8(),
        &proposed.to_payload(),
    );
    (old_hash, new_hash)
}

pub(super) fn write_remote_domain_policy_value(
    policy:   &mut RemoteDomainPolicy,
    proposed: RemoteDomainPolicyValue,
    slot:     u64,
) {
    policy.minimum_attestation_threshold = proposed.minimum_attestation_threshold;
    policy.required_finality_depth       = proposed.required_finality_depth;
    policy.message_expiry_seconds        = proposed.message_expiry_seconds;
    policy.per_message_maximum           = proposed.per_message_maximum;
    policy.per_day_maximum               = proposed.per_day_maximum;
    policy.status_flags                  = status_flag::INITIALIZED | proposed.pause_bits;
    policy.updated_at_slot               = slot;
    policy.remote_domain_separator       = proposed.remote_domain_separator;
    policy.remote_chancery_contract      = proposed.remote_chancery_contract;
    policy.remote_issued_token           = proposed.remote_issued_token;
    policy.signer_set_id                 = proposed.signer_set_id;
    policy.mode                          = proposed.mode;
    policy.remote_asset                  = proposed.remote_asset;
    policy.local_asset_mint              = solana_pubkey::Pubkey::new_from_array(proposed.local_asset_mint);
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
    let signer_set_account_info                  = &accounts[SIGNER_SET];

    if !policy_account_info.is_writable || !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;
    let args = UpdateRemoteDomainPolicyArgs::try_from_slice(args_data)
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
    assert_direct_config_change_allowed(
        classification.risk,
        &chancery_config,
        authority_account_info,
    )?;

    let signer_set = CrossChainSignerSet::load_verified(signer_set_account_info)?;
    assert_signer_set_compatible(&proposed, &signer_set, 0, false)?;
    let (old_hash, new_hash) = remote_domain_policy_update_hashes(
        policy_account_info.key,
        classification,
        &current,
        &proposed,
    );

    drop(signer_set);
    drop(current_policy);
    let clock = Clock::get()?;

    let enables_daily_cap = current.per_day_maximum == 0 && proposed.per_day_maximum != 0;

    {
        let mut policy = RemoteDomainPolicy::load_mut_for_verified_pda(
            policy_account_info,
            args.remote_chain_kind,
            args.remote_domain_id,
            remote_policy_bump,
        )?;
        write_remote_domain_policy_value(&mut policy, proposed, clock.slot);
    }
    drop(chancery_config);

    // ── Binding-time corridor daily window (idempotent, verified) ─────────────
    let mut initialized_daily_window: Option<(solana_pubkey::Pubkey, i64)> = None;

    if enables_daily_cap {
        if accounts.len() <= REMOTE_DAILY_USAGE_WINDOW {
            return Err(ChanceryError::MissingAccount.into());
        }

        let payer_account_info          = &accounts[PAYER];
        let system_program_account_info = &accounts[SYSTEM_PROGRAM];
        let window_account_info         = &accounts[REMOTE_DAILY_USAGE_WINDOW];

        if !payer_account_info.is_signer {
            return Err(ChanceryError::AccountNotSigner.into());
        }

        let scope_hash = remote_domain_usage_window_scope_hash(policy_account_info.key);

        // Inline account expression: block-scoped bindings never join the IDL
        // generator's index map, so writability must be visible at the call.
        let created = crate::modules::limits::state::usage_window::create_usage_window_if_missing(
            &accounts[REMOTE_DAILY_USAGE_WINDOW],
            payer_account_info,
            system_program_account_info,
            &scope_hash,
            crate::constants::window_kind::DAILY,
            clock.unix_timestamp,
            payer_account_info.key,
            &program_id,
        )?;

        if created {
            initialized_daily_window = Some((
                *window_account_info.key,
                crate::modules::limits::state::usage_window::canonical_window_start(
                    crate::constants::window_kind::DAILY,
                    clock.unix_timestamp,
                )?,
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
            change_id:            [0u8; 32],
            remote_domain_policy: *policy_account_info.key,
            remote_chain_kind:    proposed.remote_chain_kind,
            remote_domain_id:     proposed.remote_domain_id,
            signer_set_id:        proposed.signer_set_id,
            old_value_hash:       old_hash,
            new_value_hash:       new_hash,
            change_mask:          classification.change_mask,
            proposed_by:          *authority_account_info.key,
            updated_by:           *authority_account_info.key,
        },
    )?;

    if let Some((window, window_start)) = initialized_daily_window {
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
                scope_hash:                  remote_domain_usage_window_scope_hash(policy_account_info.key),
                window_kind:                 crate::constants::window_kind::DAILY,
                window_start_unix_timestamp: window_start,
                rent_refund_recipient:       *accounts[PAYER].key,
                created_by:                  *authority_account_info.key,
            },
        )?;
    }

    Ok(())
}
