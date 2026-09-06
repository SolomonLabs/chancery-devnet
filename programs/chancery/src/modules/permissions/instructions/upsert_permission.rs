/// upsert_permission - direct restrictive/routine permission mutation.
///
/// This handler never creates a capability-bearing record. New records, role
/// additions, expiry extension/removal, and dangerous-role additions
/// are classified from current state and rejected with
/// `ConfigChangeRequiresTimelock`.
///
/// The permissions module is dispatch-gated; wire account 0 is the module
/// activation state. Indices below are handler-local after that prefix.
///
/// Accounts:
///   0  chancery_config          writable  PDA - sequence nonce bump on emit
///   1  event_authority          readable  PDA [b"event-authority"]
///   2  permission_record        writable  PDA [b"permission", subject, scope_kind, scope_key]
///   3  granting_authority       signer    governance, operations, or scoped grant/revoke holder
///   4  grantor_permission       optional  required for a non-admin granting authority

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    constants::{change_kind, seeds},
    error::ChanceryError,
    modules::{
        control::{
            change_risk::assert_direct_config_change_allowed,
            pending_change::compute_config_change_hash,
        },
        core::state::chancery_config::ChanceryConfig,
        evidence::emit::{emit_permission_upserted, PermissionUpserted},
        permissions::{
            auth::{
                load_canonical_permission_record_mut,
                load_optional_canonical_permission_record,
            },
            change_detection::{classify_permission_change, PermissionValue},
            mutation::{
                assert_permission_mutation_authority,
                assert_permission_value_invariants,
                permission_change_rewrites_grant_provenance,
                write_permission_record,
            },
        },
    },
};

const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const PERMISSION_RECORD:      usize = 2;
const GRANTING_AUTHORITY:     usize = 3;
const GRANTOR_PERMISSION:     usize = 4;
const REQUIRED_ACCOUNT_COUNT: usize = 4;

#[derive(BorshDeserialize)]
pub struct UpsertPermissionArgs {
    pub subject:               Pubkey,
    pub scope_kind:            u8,
    pub scope_key:             Pubkey,
    pub role_bits:             [u64; 2],
    pub expiry_unix_timestamp: i64,
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info    = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info    = &accounts[EVENT_AUTHORITY];
    let permission_record_account_info  = &accounts[PERMISSION_RECORD];
    let granting_authority_account_info = &accounts[GRANTING_AUTHORITY];
    let grantor_permission_account_info = if accounts.len() > GRANTOR_PERMISSION {
        Some(&accounts[GRANTOR_PERMISSION])
    } else {
        None
    };

    if !granting_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !permission_record_account_info.is_writable
        || !chancery_config_account_info.is_writable
    {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let args = UpsertPermissionArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;
    let program_id = crate::id();
    let (expected_key, bump) = Pubkey::find_program_address(
        &[
            seeds::PERMISSION,
            args.subject.as_ref(),
            &[args.scope_kind],
            args.scope_key.as_ref(),
        ],
        &program_id,
    );

    if permission_record_account_info.key != &expected_key {
        return Err(ChanceryError::InvalidPda.into());
    }

    let current = load_optional_canonical_permission_record(
        permission_record_account_info,
        &program_id,
    )?;
    let permission_generation = current
        .as_ref()
        .map_or(0, |record| record.effective_permission_generation());
    let mut proposed = PermissionValue {
        subject: args.subject,
        scope_kind: args.scope_kind,
        scope_key: args.scope_key,
        role_bits: args.role_bits,
        permission_generation,
        expiry_unix_timestamp: args.expiry_unix_timestamp,
    };
    let classification = classify_permission_change(current.as_ref(), &proposed)?;
    let clock = Clock::get()?;
    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    assert_permission_value_invariants(
        &proposed,
        current.is_some(),
        classification.risk,
        clock.unix_timestamp,
    )?;
    assert_direct_config_change_allowed(
        classification.risk,
        &chancery_config,
        granting_authority_account_info,
    )?;
    assert_permission_mutation_authority(
        &chancery_config,
        granting_authority_account_info,
        grantor_permission_account_info,
        &proposed,
        classification,
        clock.unix_timestamp,
        &program_id,
    )?;

    let current = current.ok_or(ChanceryError::ConfigChangeRequiresTimelock)?;
    let old_generation = current.effective_permission_generation();
    let new_generation = if classification.change_mask == 0 {
        old_generation
    } else {
        old_generation
            .checked_add(1)
            .ok_or(ChanceryError::ArithmeticOverflow)?
    };
    proposed.permission_generation = new_generation;
    let old_value = PermissionValue::from_record(&current);
    let old_hash = compute_config_change_hash(
        change_kind::UPSERT_PERMISSION,
        permission_record_account_info.key,
        classification.risk.as_u8(),
        &old_value.to_payload(),
    );
    let new_hash = compute_config_change_hash(
        change_kind::UPSERT_PERMISSION,
        permission_record_account_info.key,
        classification.risk.as_u8(),
        &proposed.to_payload(),
    );

    let rewrite_grant_provenance =
        permission_change_rewrites_grant_provenance(classification);
    let (final_granted_by, final_permission_flags) = {
        let mut record = load_canonical_permission_record_mut(
            permission_record_account_info,
            &proposed.subject,
            proposed.scope_kind,
            &proposed.scope_key,
            &program_id,
        )?;
        write_permission_record(
            &mut record,
            proposed,
            bump,
            clock.unix_timestamp,
            *granting_authority_account_info.key,
            rewrite_grant_provenance,
        );
        (record.granted_by, record.permission_flags)
    };

    drop(chancery_config);
    
    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    emit_permission_upserted(
        event_authority_account_info,
        event_authority_bump,
        PermissionUpserted {
            sequence_nonce,
            chancery: *chancery_config_account_info.key,
            slot: clock.slot,
            unix_timestamp: clock.unix_timestamp,
            risk_class: classification.risk.as_u8(),
            permission_record: *permission_record_account_info.key,
            subject: proposed.subject,
            scope_kind: proposed.scope_kind,
            scope_key: proposed.scope_key,
            role_bits: proposed.role_bits,
            permission_flags: final_permission_flags,
            expiry_unix_timestamp: proposed.expiry_unix_timestamp,
            granted_by: final_granted_by,
            modified_by: *granting_authority_account_info.key,
            change_id: [0u8; 32],
            old_value_hash: old_hash,
            new_value_hash: new_hash,
            change_mask: classification.change_mask,
            approved_by: Pubkey::default(),
        },
    )?;

    Ok(())
}
