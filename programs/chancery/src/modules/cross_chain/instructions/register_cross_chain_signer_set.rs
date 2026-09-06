//! Governance/timelocked cross-chain signer-set registration.
//!
//! A signer root is a live trust anchor as soon as a remote policy references
//! it, so allocation and initialization consume an accepted `Dangerous`
//! pending change. The operational registering authority must independently
//! hold the signer-set-scoped role; governance cannot bypass that role check.
//!
//! Handler-local accounts after the module-activation prefix:
//!   0  chancery_config                writable
//!   1  event_authority                readable
//!   2  pending_config_change          writable
//!   3  signer_set                     writable  canonical signer-set PDA
//!   4  payer                          signer
//!   5  registering_authority          signer
//!   6  governance_authority           signer
//!   7  authority_permission_record    readable  CAN_REGISTER_CROSS_CHAIN_SIGNER_SET @ signer_set
//!   8  system_program                 readable

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    constants::{change_kind, role, scope, seeds, status_flag},
    error::ChanceryError,
    modules::{
        control::{
            change_risk::ConfigChangeRiskClass,
            pending_change::{
                assert_accepted_pending_change, compute_config_change_hash, consume_and_close_pending_change,
            },
        },
        core::{instructions::create_pda_account, state::chancery_config::ChanceryConfig},
        cross_chain::{
            auth::assert_signer_holds_role,
            change_detection::{
                CrossChainSignerSetValue, CROSS_CHAIN_SIGNER_SET_VALUE_PAYLOAD_SIZE,
            },
            instructions::{
                assert_disjoint_cross_chain_account_index_groups,
                assert_distinct_cross_chain_account_indexes,
            },
            state::cross_chain_signer_set::{
                CrossChainSignerSet, CROSS_CHAIN_SIGNER_SET_DISCRIMINATOR,
                CROSS_CHAIN_SIGNER_SET_SIZE,
            },
        },
        evidence::emit::{
            emit_cross_chain_signer_set_registered, CrossChainSignerSetRegistered,
        },
    },
};

const CHANCERY_CONFIG:              usize = 0;
const EVENT_AUTHORITY:              usize = 1;
const PENDING:                      usize = 2;
const SIGNER_SET:                   usize = 3;
const PAYER:                        usize = 4;
const REGISTERING_AUTHORITY:        usize = 5;
const GOVERNANCE_AUTHORITY:         usize = 6;
const AUTHORITY_PERMISSION_RECORD:  usize = 7;
const SYSTEM_PROGRAM:               usize = 8;
const REQUIRED_ACCOUNT_COUNT:       usize = 9;

// Refund conveyance: the account whose key equals the record's stored
// rent_refund_recipient. Required whenever this instruction closes the
// record - the refund is conveyed, never redirected.
const RENT_REFUND_RECIPIENT:        usize = 9;

const PROTECTED_ACCOUNT_INDEXES: &[usize] = &[
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    PENDING,
    SIGNER_SET,
    SYSTEM_PROGRAM,
];
const IDENTITY_ACCOUNT_INDEXES:   &[usize] = &[
    PAYER,
    REGISTERING_AUTHORITY,
    GOVERNANCE_AUTHORITY,
    RENT_REFUND_RECIPIENT,
];
const PERMISSION_ACCOUNT_INDEXES: &[usize] = &[AUTHORITY_PERMISSION_RECORD];

#[derive(BorshDeserialize)]
pub struct RegisterCrossChainSignerSetArgs {
    pub signer_set_id:              [u8; 32],
    pub threshold:                  u8,
    pub signer_count:               u8,
    pub signer_root:                [u8; 32],
    pub valid_after_unix_timestamp: i64,
    pub expires_at_unix_timestamp:  i64,
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
    let signer_set_account_info                  = &accounts[SIGNER_SET];
    let payer_account_info                       = &accounts[PAYER];
    let registering_authority_account_info       = &accounts[REGISTERING_AUTHORITY];
    let governance_authority_account_info        = &accounts[GOVERNANCE_AUTHORITY];
    let authority_permission_record_account_info = &accounts[AUTHORITY_PERMISSION_RECORD];
    let system_program_account_info              = &accounts[SYSTEM_PROGRAM];

    if !payer_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }
    if !registering_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }
    if !governance_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }
    if !pending_account_info.is_writable
        || !signer_set_account_info.is_writable
        || !chancery_config_account_info.is_writable
    {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;
    if governance_authority_account_info.key != &chancery_config.governance_authority {
        return Err(ChanceryError::ConfigChangeWideningRequiresGovernance.into());
    }

    let args = RegisterCrossChainSignerSetArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;
    let value = CrossChainSignerSetValue {
        signer_set_id: args.signer_set_id,
        threshold: args.threshold,
        signer_count: args.signer_count,
        signer_root: args.signer_root,
        valid_after_unix_timestamp: args.valid_after_unix_timestamp,
        expires_at_unix_timestamp: args.expires_at_unix_timestamp,
    };
    value.assert_valid()?;

    let program_id = crate::id();
    let (expected_key, bump) = Pubkey::find_program_address(
        &[seeds::CROSS_CHAIN_SIGNER_SET, args.signer_set_id.as_ref()],
        &program_id,
    );
    if signer_set_account_info.key != &expected_key {
        return Err(ChanceryError::InvalidPda.into());
    }
    if !signer_set_account_info.data_is_empty() {
        return Err(ChanceryError::AlreadyInitialized.into());
    }

    assert_signer_holds_role(
        registering_authority_account_info,
        authority_permission_record_account_info,
        role::CAN_REGISTER_CROSS_CHAIN_SIGNER_SET,
        scope::CROSS_CHAIN_SIGNER_SET,
        &expected_key,
        &program_id,
    )?;

    let risk = ConfigChangeRiskClass::Dangerous;
    let old_payload = [0u8; CROSS_CHAIN_SIGNER_SET_VALUE_PAYLOAD_SIZE];
    let new_payload = value.to_payload();
    let old_hash = compute_config_change_hash(
        change_kind::REGISTER_CROSS_CHAIN_SIGNER_SET,
        signer_set_account_info.key,
        risk.as_u8(),
        &old_payload,
    );
    let new_hash = compute_config_change_hash(
        change_kind::REGISTER_CROSS_CHAIN_SIGNER_SET,
        signer_set_account_info.key,
        risk.as_u8(),
        &new_payload,
    );
    let pending = assert_accepted_pending_change(
        pending_account_info,
        change_kind::REGISTER_CROSS_CHAIN_SIGNER_SET,
        risk,
        signer_set_account_info.key,
        &old_hash,
        &new_hash,
        &chancery_config.governance_authority,
    )?;
    let change_id = pending.change_id;
    let proposed_by = pending.proposed_by;
    drop(pending);

    create_pda_account(
        payer_account_info,
        signer_set_account_info,
        system_program_account_info,
        &program_id,
        &[
            seeds::CROSS_CHAIN_SIGNER_SET,
            value.signer_set_id.as_ref(),
            &[bump],
        ],
        CROSS_CHAIN_SIGNER_SET_SIZE,
    )?;

    {
        let mut signer_set = CrossChainSignerSet::load_uninitialized_mut(signer_set_account_info)?;
        signer_set.discriminator              = CROSS_CHAIN_SIGNER_SET_DISCRIMINATOR;
        signer_set.version                    = 1;
        signer_set.bump                       = bump;
        signer_set.threshold                  = value.threshold;
        signer_set.signer_count               = value.signer_count;
        signer_set._pad0                      = [0u8; 3];
        signer_set.valid_after_unix_timestamp = value.valid_after_unix_timestamp;
        signer_set.expires_at_unix_timestamp  = value.expires_at_unix_timestamp;
        signer_set.status_flags               = status_flag::INITIALIZED;
        signer_set.signer_set_id              = value.signer_set_id;
        signer_set.signer_root                = value.signer_root;
        signer_set.created_by                 = *registering_authority_account_info.key;
        signer_set._reserved                  = [0u8; 32];
    }

    let clock = Clock::get()?;

    if accounts.len() <= RENT_REFUND_RECIPIENT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let rent_refund_recipient_account_info = &accounts[RENT_REFUND_RECIPIENT];

    consume_and_close_pending_change(pending_account_info, rent_refund_recipient_account_info)?;

    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;

    // Total-ever signer-set count (tombstone model, doc 14 §14.6.4).
    chancery_config_mut.total_signer_sets_registered = chancery_config_mut
        .total_signer_sets_registered
        .checked_add(1)
        .ok_or(ChanceryError::ArithmeticOverflow)?;
    let event_authority_bump = chancery_config_mut.event_authority_bump;
    let sequence_nonce = chancery_config_mut.next_sequence_nonce()?;

    emit_cross_chain_signer_set_registered(
        event_authority_account_info,
        event_authority_bump,
        CrossChainSignerSetRegistered {
            sequence_nonce,
            chancery:                   *chancery_config_account_info.key,
            slot:                       clock.slot,
            unix_timestamp:             clock.unix_timestamp,
            risk_class:                 risk.as_u8(),
            change_id,
            cross_chain_signer_set:     *signer_set_account_info.key,
            signer_set_id:              value.signer_set_id,
            signer_root:                value.signer_root,
            signer_count:               value.signer_count,
            threshold:                  value.threshold,
            valid_after_unix_timestamp: value.valid_after_unix_timestamp,
            expires_at_unix_timestamp:  value.expires_at_unix_timestamp,
            old_value_hash:             old_hash,
            new_value_hash:             new_hash,
            proposed_by,
            registered_by:              *registering_authority_account_info.key,
        },
    )?;

    Ok(())
}
