/// upsert_permission_with_pending_change.
///
/// Applies a Widening, HighImpact, or Dangerous permission mutation after an
/// accepted governance pending-change ceremony. The accepted record pins the
/// exact current and proposed semantic permission values.
///
/// The permissions module is dispatch-gated; wire account 0 is the module
/// activation state. Indices below are handler-local after that prefix.
///
/// Accounts:
///   0  chancery_config          writable  PDA
///   1  event_authority          readable  PDA [b"event-authority"]
///   2  pending_config_change    writable  accepted pending change PDA
///   3  permission_record        writable  PDA [b"permission", subject, scope_kind, scope_key]
///   4  payer                    signer    funds first allocation
///   5  granting_authority       signer    recorded grantor; must satisfy grant constraints
///   6  governance_authority     signer    must equal configured governance authority
///   7  system_program           readable
///   8  grantor_permission       optional  required for a non-admin granting authority
///
/// On record creation only (never on mutation of an existing record), the
/// subject's per-party dimension accumulators are bound in the same
/// transaction:
///   9  counterparty_daily_usage_window  writable  PDA [b"usage-window", sha256(COUNTERPARTY || subject), DAILY]
///  10  executor_daily_usage_window      writable  PDA [b"usage-window", sha256(EXECUTOR || subject), DAILY]
///
/// Both are created unconditionally at grant (create-if-missing, 2 x 144 B)
/// rather than per role bit: later role additions are widenings on this same
/// record and must not depend on a second binding surface. Existing windows
/// are verified and left untouched.

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    constants::{change_kind, scope, seeds, window_kind},
    error::ChanceryError,
    modules::{
        control::{
            pending_change::{
                assert_accepted_pending_change, compute_config_change_hash, consume_and_close_pending_change,
            },
        },
        core::{
            instructions::create_pda_account,
            state::chancery_config::ChanceryConfig,
        },
        evidence::emit::{
            emit_permission_upserted, emit_usage_window_initialized,
            PermissionUpserted, UsageWindowInitialized,
        },
        limits::{
            dimension::dimension_window_scope_hash,
            state::usage_window::{create_usage_window_if_missing, UsageWindow},
        },
        permissions::{
            auth::{
                load_canonical_permission_record_mut,
                load_optional_canonical_permission_record,
                load_uninitialized_permission_record_mut,
            },
            change_detection::{classify_permission_change, PermissionValue},
            mutation::{
                assert_permission_mutation_authority,
                assert_permission_value_invariants,
                permission_change_rewrites_grant_provenance,
                write_permission_record,
            },
            state::permission_record::PERMISSION_RECORD_SIZE,
        },
    },
};

const CHANCERY_CONFIG:                 usize = 0;
const EVENT_AUTHORITY:                 usize = 1;
const PENDING:                         usize = 2;
const PERMISSION_RECORD:               usize = 3;
const PAYER:                           usize = 4;
const GRANTING_AUTHORITY:              usize = 5;
const GOVERNANCE_AUTHORITY:            usize = 6;
const SYSTEM_PROGRAM:                  usize = 7;
const GRANTOR_PERMISSION:              usize = 8;
const COUNTERPARTY_DAILY_USAGE_WINDOW: usize = 9;
const EXECUTOR_DAILY_USAGE_WINDOW:     usize = 10;
const REQUIRED_ACCOUNT_COUNT:          usize = 8;

// Refund conveyance: the account whose key equals the record's stored
// rent_refund_recipient. Required whenever this instruction closes the
// record - the refund is conveyed, never redirected.
const RENT_REFUND_RECIPIENT:           usize = 11;

#[derive(BorshDeserialize)]
pub struct UpsertPermissionWithPendingChangeArgs {
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

    let chancery_config_account_info      = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];
    let pending_account_info              = &accounts[PENDING];
    let permission_record_account_info    = &accounts[PERMISSION_RECORD];
    let payer_account_info                = &accounts[PAYER];
    let granting_authority_account_info   = &accounts[GRANTING_AUTHORITY];
    let governance_authority_account_info = &accounts[GOVERNANCE_AUTHORITY];
    let system_program_account_info       = &accounts[SYSTEM_PROGRAM];
    let grantor_permission_account_info   = if accounts.len() > GRANTOR_PERMISSION {
        Some(&accounts[GRANTOR_PERMISSION])
    } else {
        None
    };

    if !payer_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !granting_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !governance_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !pending_account_info.is_writable
        || !permission_record_account_info.is_writable
        || !chancery_config_account_info.is_writable
    {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let args = UpsertPermissionWithPendingChangeArgs::try_from_slice(args_data)
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

    if !classification.risk.requires_timelock() {
        return Err(ChanceryError::ConfigChangeNotAccepted.into());
    }

    let clock = Clock::get()?;
    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    if governance_authority_account_info.key != &chancery_config.governance_authority {
        return Err(ChanceryError::ConfigChangeWideningRequiresGovernance.into());
    }

    assert_permission_value_invariants(
        &proposed,
        current.is_some(),
        classification.risk,
        clock.unix_timestamp,
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

    let old_value = current
        .as_ref()
        .map(PermissionValue::from_record)
        .unwrap_or_else(|| PermissionValue::empty(
            proposed.subject,
            proposed.scope_kind,
            proposed.scope_key,
        ));
    let old_generation = current
        .as_ref()
        .map_or(0, |record| record.effective_permission_generation());
    let new_generation = old_generation
        .checked_add(1)
        .ok_or(ChanceryError::ArithmeticOverflow)?;
    proposed.permission_generation = new_generation;
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

    let pending = assert_accepted_pending_change(
        pending_account_info,
        change_kind::UPSERT_PERMISSION,
        classification.risk,
        permission_record_account_info.key,
        &old_hash,
        &new_hash,
        &chancery_config.governance_authority,
    )?;
    let change_id = pending.change_id;
    let approved_by = pending.proposed_by;
    drop(pending);

    let allocated_new_permission_record_slot = current.is_none()
        && permission_record_account_info.data_is_empty();

    if allocated_new_permission_record_slot {
        create_pda_account(
            payer_account_info,
            permission_record_account_info,
            system_program_account_info,
            &program_id,
            &[
                seeds::PERMISSION,
                proposed.subject.as_ref(),
                &[proposed.scope_kind],
                proposed.scope_key.as_ref(),
                &[bump],
            ],
            PERMISSION_RECORD_SIZE,
        )?;
    }

    let rewrite_grant_provenance =
        permission_change_rewrites_grant_provenance(classification);
    let (final_granted_by, final_permission_flags) = if current.is_some() {
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
    } else {
        let mut record = load_uninitialized_permission_record_mut(
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
            true,
        );
        (record.granted_by, record.permission_flags)
    };

    // ── Binding-time per-party dimension windows (record creation only) ──────
    // The subject's counterparty and executor daily accumulators come into
    // existence with the record that makes the subject a possible party.
    let mut initialized_windows: Vec<(Pubkey, [u8; 32], u8, i64)> = Vec::new();

    if current.is_none() {
        let party_windows = [
            (scope::COUNTERPARTY, COUNTERPARTY_DAILY_USAGE_WINDOW),
            (scope::EXECUTOR,     EXECUTOR_DAILY_USAGE_WINDOW),
        ];

        for (party_scope_kind, index) in party_windows {
            if accounts.len() <= index {
                return Err(ChanceryError::MissingAccount.into());
            }

            let scope_hash = dimension_window_scope_hash(party_scope_kind, &proposed.subject);

            // Inline per-constant fetch at the mutating call: the IDL generator's
            // writability analysis follows `accounts[CONST]` passed directly to a
            // mutably-borrowing callee, but not conditional bindings.
            let created = if index == COUNTERPARTY_DAILY_USAGE_WINDOW {
                create_usage_window_if_missing(
                &accounts[COUNTERPARTY_DAILY_USAGE_WINDOW],
                payer_account_info,
                system_program_account_info,
                &scope_hash,
                window_kind::DAILY,
                clock.unix_timestamp,
                payer_account_info.key,
                &program_id,
            )?
            } else {
                create_usage_window_if_missing(
                &accounts[EXECUTOR_DAILY_USAGE_WINDOW],
                payer_account_info,
                system_program_account_info,
                &scope_hash,
                window_kind::DAILY,
                clock.unix_timestamp,
                payer_account_info.key,
                &program_id,
            )?
            };

            if created {
                let window = UsageWindow::load_verified(&accounts[index])?;
                initialized_windows.push((
                    *accounts[index].key,
                    scope_hash,
                    window_kind::DAILY,
                    window.window_start_unix_timestamp,
                ));
            }
        }
    }

    if accounts.len() <= RENT_REFUND_RECIPIENT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let rent_refund_recipient_account_info = &accounts[RENT_REFUND_RECIPIENT];

    consume_and_close_pending_change(pending_account_info, rent_refund_recipient_account_info)?;

    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;

    // Allocated PermissionRecord slot count (doc 14 §14.6.4): increment only
    // when the system account is allocated for the first time. Revoke writes a
    // canonical zero value in place, so a later regrant reuses the same slot.
    if allocated_new_permission_record_slot {
        chancery_config_mut.allocated_permission_record_slots = chancery_config_mut
            .allocated_permission_record_slots
            .checked_add(1)
            .ok_or(ChanceryError::ArithmeticOverflow)?;
    }

    let event_authority_bump = chancery_config_mut.event_authority_bump;
    let sequence_nonce = chancery_config_mut.next_sequence_nonce()?;

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
            change_id,
            old_value_hash: old_hash,
            new_value_hash: new_hash,
            change_mask: classification.change_mask,
            approved_by,
        },
    )?;

    for (window, window_scope_hash, kind, window_start) in initialized_windows {
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
                window_kind:                 kind,
                window_start_unix_timestamp: window_start,
                rent_refund_recipient:       *payer_account_info.key,
                created_by:                  *granting_authority_account_info.key,
            },
        )?;
    }

    Ok(())
}
