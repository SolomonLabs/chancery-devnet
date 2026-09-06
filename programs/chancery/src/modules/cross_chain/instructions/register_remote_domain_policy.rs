//! Register an inactive remote-domain policy.
//!
//! Registration writes the complete domain identity and economic policy but
//! initializes every remote-domain pause bit. No inbound or outbound path can
//! use the policy until governance consumes an accepted pause-relaxation
//! change that hashes the full semantic policy value.
//!
//! Handler-local accounts after the module-activation prefix:
//!   0  chancery_config                writable  PDA
//!   1  event_authority                readable  PDA [b"event-authority"]
//!   2  remote_domain_policy           writable  canonical policy PDA
//!   3  payer                          signer
//!   4  authority                      signer
//!   5  authority_permission_record    readable  CAN_REGISTER_REMOTE_DOMAIN @ GLOBAL
//!   6  cross_chain_signer_set         readable  referenced signer-set PDA
//!   7  system_program                 readable
//!   8  remote_nonce                   writable  PDA [b"remote-nonce", chain_kind, domain_id_be, default]
//!   9  remote_daily_usage_window      writable  required iff per_day_maximum != 0;
//!                                               PDA [b"usage-window", sha256(0x05 || policy PDA), DAILY]
//!
//! The corridor's `RemoteNonce` counter pair is bound here (create-if-missing,
//! 136 B, one per corridor) so neither hot-path handler ever allocates:
//! `emit_outbound_message` and `consume_inbound_message` both require it.
//!
//! The corridor's daily accumulator is bound at registration when the cap is
//! enabled (create-if-missing), so consume/emit never allocate in the hot
//! path and an enforced corridor cap without its window is unrepresentable.

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    constants::{
        remote_domain_pause_bit, role, scope, seeds, status_flag, window_kind,
    },
    error::ChanceryError,
    modules::{
        control::change_risk::ConfigChangeRiskClass,
        core::{instructions::create_pda_account, state::chancery_config::ChanceryConfig},
        cross_chain::{
            auth::assert_signer_holds_role,
            change_detection::{
                assert_remote_domain_policy_value_valid, assert_signer_set_compatible,
                RemoteDomainPolicyValue,
            },
            instructions::{
                assert_disjoint_cross_chain_account_index_groups,
                assert_distinct_cross_chain_account_indexes,
            },
            state::{
                cross_chain_signer_set::CrossChainSignerSet,
                remote_domain_policy::{
                    RemoteDomainPolicy, REMOTE_DOMAIN_POLICY_DISCRIMINATOR,
                    REMOTE_DOMAIN_POLICY_SIZE,
                },
            },
        },
        evidence::emit::{
            emit_remote_domain_policy_registered, emit_usage_window_initialized,
            RemoteDomainPolicyRegistered, UsageWindowInitialized,
        },
        cross_chain::{
            state::remote_nonce::{
                RemoteNonce, REMOTE_NONCE_DISCRIMINATOR, REMOTE_NONCE_SIZE,
            },
            usage::remote_domain_usage_window_scope_hash,
        },
        limits::state::usage_window::{create_usage_window_if_missing, UsageWindow},
    },
};

const CHANCERY_CONFIG:              usize = 0;
const EVENT_AUTHORITY:              usize = 1;
const REMOTE_DOMAIN_POLICY:         usize = 2;
const PAYER:                        usize = 3;
const AUTHORITY:                    usize = 4;
const AUTHORITY_PERMISSION_RECORD:  usize = 5;
const SIGNER_SET:                   usize = 6;
const SYSTEM_PROGRAM:               usize = 7;
const REMOTE_NONCE_ACCOUNT_INDEX:   usize = 8;
const REMOTE_DAILY_USAGE_WINDOW:    usize = 9;
const REQUIRED_ACCOUNT_COUNT:       usize = 9;

const PROTECTED_ACCOUNT_INDEXES: &[usize] = &[
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    REMOTE_DOMAIN_POLICY,
    SIGNER_SET,
    SYSTEM_PROGRAM,
    REMOTE_NONCE_ACCOUNT_INDEX,
    REMOTE_DAILY_USAGE_WINDOW,
];
const IDENTITY_ACCOUNT_INDEXES:   &[usize] = &[PAYER, AUTHORITY];
const PERMISSION_ACCOUNT_INDEXES: &[usize] = &[AUTHORITY_PERMISSION_RECORD];

#[derive(BorshDeserialize)]
pub struct RegisterRemoteDomainPolicyArgs {
    pub remote_chain_kind:             u8,
    pub remote_domain_id:              u64,
    pub minimum_attestation_threshold: u8,
    /// Attestor-SOP minimum remote confirmation depth; not an on-chain
    /// consume predicate because Solana cannot observe the remote chain.
    pub required_finality_depth:       u64,
    pub message_expiry_seconds:        u64,
    pub per_message_maximum:           u64,
    pub per_day_maximum:               u64,
    pub remote_domain_separator:       [u8; 32],
    pub remote_chancery_contract:      [u8; 32],
    pub remote_issued_token:           [u8; 32],
    pub signer_set_id:                 [u8; 32],
    pub mode:                          u8,
    pub remote_asset:                  [u8; 32],
    /// Immutable local collateral mint configured as `motherAsset` on the
    /// remote daughter contract.
    pub local_asset_mint:              Pubkey,
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
    let payer_account_info                       = &accounts[PAYER];
    let authority_account_info                   = &accounts[AUTHORITY];
    let authority_permission_record_account_info = &accounts[AUTHORITY_PERMISSION_RECORD];
    let signer_set_account_info                  = &accounts[SIGNER_SET];
    let system_program_account_info              = &accounts[SYSTEM_PROGRAM];

    if !payer_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }
    if !policy_account_info.is_writable || !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;
    let args = RegisterRemoteDomainPolicyArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;
    let program_id = crate::id();

    assert_signer_holds_role(
        authority_account_info,
        authority_permission_record_account_info,
        role::CAN_REGISTER_REMOTE_DOMAIN,
        scope::GLOBAL,
        &Pubkey::default(),
        &program_id,
    )?;

    let value = RemoteDomainPolicyValue {
        remote_chain_kind: args.remote_chain_kind,
        minimum_attestation_threshold: args.minimum_attestation_threshold,
        remote_domain_id: args.remote_domain_id,
        required_finality_depth: args.required_finality_depth,
        message_expiry_seconds: args.message_expiry_seconds,
        per_message_maximum: args.per_message_maximum,
        per_day_maximum: args.per_day_maximum,
        pause_bits: remote_domain_pause_bit::ALL,
        remote_domain_separator: args.remote_domain_separator,
        remote_chancery_contract: args.remote_chancery_contract,
        remote_issued_token: args.remote_issued_token,
        signer_set_id: args.signer_set_id,
        mode: args.mode,
        remote_asset: args.remote_asset,
        local_asset_mint: args.local_asset_mint.to_bytes(),
    };
    assert_remote_domain_policy_value_valid(&value)?;

    let signer_set = CrossChainSignerSet::load_verified(signer_set_account_info)?;
    assert_signer_set_compatible(&value, &signer_set, 0, false)?;

    let id_be = args.remote_domain_id.to_be_bytes();
    let bump = RemoteDomainPolicy::verify_pda(
        policy_account_info,
        args.remote_chain_kind,
        args.remote_domain_id,
        &program_id,
    )?;

    if policy_account_info.data_is_empty() {
        create_pda_account(
            payer_account_info,
            policy_account_info,
            system_program_account_info,
            &program_id,
            &[
                seeds::REMOTE_DOMAIN_POLICY,
                &[args.remote_chain_kind],
                &id_be,
                &[bump],
            ],
            REMOTE_DOMAIN_POLICY_SIZE,
        )?;
    } else if policy_account_info.data_len() != REMOTE_DOMAIN_POLICY_SIZE {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    let clock = Clock::get()?;
    {
        let mut policy = RemoteDomainPolicy::load_uninitialized_mut(policy_account_info)?;
        policy.discriminator                 = REMOTE_DOMAIN_POLICY_DISCRIMINATOR;
        policy.version                       = 1;
        policy.bump                          = bump;
        policy.remote_chain_kind             = value.remote_chain_kind;
        policy.minimum_attestation_threshold = value.minimum_attestation_threshold;
        policy._pad0                         = [0u8; 3];
        policy.remote_domain_id              = value.remote_domain_id;
        policy.required_finality_depth       = value.required_finality_depth;
        policy.message_expiry_seconds        = value.message_expiry_seconds;
        policy.per_message_maximum           = value.per_message_maximum;
        policy.per_day_maximum               = value.per_day_maximum;
        policy.status_flags                  = status_flag::INITIALIZED | value.pause_bits;
        policy.updated_at_slot               = clock.slot;
        policy.remote_domain_separator       = value.remote_domain_separator;
        policy.remote_chancery_contract      = value.remote_chancery_contract;
        policy.remote_issued_token           = value.remote_issued_token;
        policy.signer_set_id                 = value.signer_set_id;
        policy.created_by                    = *authority_account_info.key;
        policy.mode                          = value.mode;
        policy._pad_mode                     = [0u8; 7];
        policy.remote_asset                  = value.remote_asset;
        policy.local_asset_mint              = Pubkey::new_from_array(value.local_asset_mint);
        policy._reserved                     = [0u8; 32];
    }

    drop(signer_set);
    drop(chancery_config);

    // ── Binding-time corridor nonce counters ──────────────────────────────────
    // One RemoteNonce per (chain_kind, domain_id) corridor. Create-if-missing:
    // re-registration attempts fail earlier on the policy account, but the
    // nonce PDA is keyed by corridor, not policy, so an allocated counter is
    // verified and left untouched - binding never resets delivery ordering.
    {
        let nonce_account_info = &accounts[REMOTE_NONCE_ACCOUNT_INDEX];


        let nonce_scope_key    = Pubkey::default();
        let id_be              = value.remote_domain_id.to_be_bytes();

        let (expected_nonce_key, nonce_bump) = Pubkey::find_program_address(
            &[
                seeds::REMOTE_NONCE,
                &[value.remote_chain_kind],
                &id_be,
                nonce_scope_key.as_ref(),
            ],
            &program_id,
        );

        if nonce_account_info.key != &expected_nonce_key {
            return Err(ChanceryError::InvalidPda.into());
        }

        if nonce_account_info.data_is_empty() {
            create_pda_account(
                payer_account_info,
                nonce_account_info,
                system_program_account_info,
                &program_id,
                &[
                    seeds::REMOTE_NONCE,
                    &[value.remote_chain_kind],
                    &id_be,
                    nonce_scope_key.as_ref(),
                    &[nonce_bump],
                ],
                REMOTE_NONCE_SIZE,
            )?;

            // Inline account expression: marks the slot writable in the IDL
            // generator's analysis (top-of-handler bindings only join its
            // index map; this one is block-scoped).
            let mut n = RemoteNonce::load_uninitialized_mut(&accounts[REMOTE_NONCE_ACCOUNT_INDEX])?;

            n.discriminator              = REMOTE_NONCE_DISCRIMINATOR;
            n.version                    = 1;
            n.bump                       = nonce_bump;
            n._pad0                      = [0u8; 5];
            n.remote_domain_id           = value.remote_domain_id;
            n.next_inbound_nonce         = 0;
            n.next_outbound_nonce        = 0;
            n.scope_key                  = nonce_scope_key.to_bytes();
            n.last_consumed_message_hash = [0u8; 32];
            n.last_emitted_message_hash  = [0u8; 32];
            n._reserved                  = [0u8; 32];
        } else {
            // Prove the allocated account is the canonical initialized counter.
            let n = RemoteNonce::load_verified(nonce_account_info, value.remote_chain_kind)?;

            if n.remote_domain_id != value.remote_domain_id {
                return Err(ChanceryError::AccountKeyMismatch.into());
            }
        }
    }

    // ── Binding-time corridor daily window ────────────────────────────────────
    let mut initialized_window: Option<(Pubkey, [u8; 32], i64)> = None;

    if value.per_day_maximum != 0 {
        if accounts.len() <= REMOTE_DAILY_USAGE_WINDOW {
            return Err(ChanceryError::MissingAccount.into());
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
            &program_id,
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

    // Total-ever corridor count (tombstone model, doc 14 §14.6.4).
    chancery_config_mut.total_remote_domains_registered = chancery_config_mut
        .total_remote_domains_registered
        .checked_add(1)
        .ok_or(ChanceryError::ArithmeticOverflow)?;
    let event_authority_bump = chancery_config_mut.event_authority_bump;
    let sequence_nonce = chancery_config_mut.next_sequence_nonce()?;

    emit_remote_domain_policy_registered(
        event_authority_account_info,
        event_authority_bump,
        RemoteDomainPolicyRegistered {
            sequence_nonce,
            chancery:             *chancery_config_account_info.key,
            slot:                 clock.slot,
            unix_timestamp:       clock.unix_timestamp,
            risk_class:           ConfigChangeRiskClass::RoutineOps.as_u8(),
            remote_domain_policy: *policy_account_info.key,
            remote_chain_kind:    value.remote_chain_kind,
            remote_domain_id:     value.remote_domain_id,
            mode:                 value.mode,
            pause_bits:           value.pause_bits,
            signer_set_id:        value.signer_set_id,
            registered_by:        *authority_account_info.key,
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
                risk_class:                  ConfigChangeRiskClass::RoutineOps.as_u8(),
                usage_window:                window,
                scope_hash:                  window_scope_hash,
                window_kind:                 window_kind::DAILY,
                window_start_unix_timestamp: window_start,
                rent_refund_recipient:       *payer_account_info.key,
                created_by:                  *authority_account_info.key,
            },
        )?;
    }

    Ok(())
}
