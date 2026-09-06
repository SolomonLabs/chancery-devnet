/// revoke_permission
///
/// Replaces a PermissionRecord with an initialized paused tombstone, revoking
/// all roles while preserving a generation-distinct baseline for regrant.
/// The account stays allocated for future re-use through the pending-change upsert path.
///
/// Accounts:
///   0  chancery_config        writable  PDA - seq nonce bump on emit
///   1  event_authority        readable  PDA [b"event-authority"]
///   2  permission_record      writable  PDA [b"permission", subject, scope_kind, scope_key]
///   3  revoking_authority     signer    must be ops or governance

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_sysvar::Sysvar;

use crate::{
    error::ChanceryError,
    modules::{
        core::state::chancery_config::ChanceryConfig,
        evidence::emit::{emit_permission_revoked, PermissionRevoked},
        permissions::{
            auth::{
                load_canonical_permission_record, load_canonical_permission_record_mut,
            },
            state::permission_record::PERMISSION_FLAG_PAUSED,
        },
    },
};

// ─── Account indices ──────────────────────────────────────────────────────────
const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const PERMISSION_RECORD:      usize = 2;
const REVOKING_AUTHORITY:     usize = 3;
const REQUIRED_ACCOUNT_COUNT: usize = 4;

// ─── Args ─────────────────────────────────────────────────────────────────────
/// No args needed - the account itself identifies the record.
/// Borsh-deserialize will succeed on empty slice.
#[derive(BorshDeserialize)]
pub struct RevokePermissionArgs {}

// ─── Handler ──────────────────────────────────────────────────────────────────
pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info    = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info    = &accounts[EVENT_AUTHORITY];
    let permission_record_account_info  = &accounts[PERMISSION_RECORD];
    let revoking_authority_account_info = &accounts[REVOKING_AUTHORITY];

    if !revoking_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !permission_record_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let _args = RevokePermissionArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    let is_admin = revoking_authority_account_info.key == &chancery_config.governance_authority
        || revoking_authority_account_info.key == &chancery_config.operations_authority;

    if !is_admin {
        return Err(ChanceryError::InsufficientRole.into());
    }

    // Verify owner + size + discriminator before mutation: a same-size foreign
    // account (e.g. LimitPolicy) must be rejected, not destroyed (issue #6).
    // Capture the complete semantic identity for evidence and PDA verification.
    let (
        revoked_subject,
        revoked_scope_kind,
        revoked_scope_key,
        already_revoked,
        next_permission_generation,
    ) = {
        let permission_record = load_canonical_permission_record(
            permission_record_account_info,
            &crate::id(),
        )?;
        let already_revoked = permission_record.is_revoked_tombstone();
        let next_permission_generation = if already_revoked {
            None
        } else {
            Some(
                permission_record
                    .effective_permission_generation()
                    .checked_add(1)
                    .ok_or(ChanceryError::ArithmeticOverflow)?,
            )
        };
        (
            permission_record.subject,
            permission_record.scope_kind,
            permission_record.scope_key,
            already_revoked,
            next_permission_generation,
        )
    };

    let clock = Clock::get()?;

    // Retain an initialized, paused tombstone instead of returning the PDA to
    // the same semantic empty baseline used by a never-created grant. This
    // invalidates accepted grant proposals that predate the revocation while
    // preserving the allocated slot for a fresh timelocked regrant. Repeated
    // revocation is a semantic no-op so lower-trust operations cannot churn
    // the tombstone while governance is reviewing that regrant.
    if !already_revoked {
        let mut permission_record = load_canonical_permission_record_mut(
            permission_record_account_info,
            &revoked_subject,
            revoked_scope_kind,
            &revoked_scope_key,
            &crate::id(),
        )?;
        permission_record.role_bits = [0u64; 2];
        permission_record.permission_flags = PERMISSION_FLAG_PAUSED;
        permission_record.issued_at_unix_timestamp = clock.unix_timestamp;
        permission_record.expiry_unix_timestamp = 0;
        permission_record.granted_by = *revoking_authority_account_info.key;
        permission_record.role_schema_version = crate::constants::role::PERMISSION_ROLE_SCHEMA_VERSION;
        permission_record.permission_generation = next_permission_generation
            .ok_or(ChanceryError::ArithmeticOverflow)?;
        permission_record._reserved = [0u8; 24];
    }

    // ── Evidence (emitted last, after state write) ──────────────────────────────

    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    emit_permission_revoked(
        event_authority_account_info,
        event_authority_bump,
        PermissionRevoked {
            sequence_nonce,
            chancery:          *chancery_config_account_info.key,
            slot:              clock.slot,
            unix_timestamp:    clock.unix_timestamp,
            risk_class:        0,
            permission_record: *permission_record_account_info.key,
            subject:           revoked_subject,
            scope_kind:        revoked_scope_kind,
            scope_key:         revoked_scope_key,
            revoked_by:        *revoking_authority_account_info.key,
        },
    )?;

    Ok(())
}
