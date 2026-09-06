/// set_counterparty_pause
///
/// Sets or clears the PERMISSION_FLAG_PAUSED bit on a counterparty's PermissionRecord.
/// Identical logic to `set_executor_pause` - the distinction is semantic:
/// the caller passes the counterparty's permission record rather than an
/// executor's. Settlement handlers check the flag before allowing principal-side
/// participation.
///
/// Accounts:
///   0  chancery_config        writable  PDA  (sequence_nonce bump for evidence)
///   1  event_authority        readable  PDA [b"event-authority"]
///   2  permission_record      writable  PDA [b"permission", counterparty, scope_kind, scope_key]
///   3  authority              signer    ops | governance (clear) | + emergency (set)

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    error::ChanceryError,
    modules::{
        control::instructions::{is_counterparty_pause_eligible, PERMISSION_FLAG_PAUSED},
        core::state::chancery_config::ChanceryConfig,
        evidence::emit::{emit_pause_state_change, PauseStateChange},
        permissions::{
            auth::load_canonical_permission_record_mut,
            state::permission_record::role_bits_as_u128,
        },
    },
};

const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const PERMISSION_RECORD:      usize = 2;
const AUTHORITY:              usize = 3;
const REQUIRED_ACCOUNT_COUNT: usize = 4;

#[derive(BorshDeserialize)]
pub struct SetCounterpartyPauseArgs {
    /// Subject the target PermissionRecord PDA must bind to (issue #54).
    pub subject:     Pubkey,
    pub scope_kind:  u8,
    pub scope_key:   Pubkey,
    pub is_paused:   bool,
    pub reason_code: u32,
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info   = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info   = &accounts[EVENT_AUTHORITY];
    let permission_record_account_info = &accounts[PERMISSION_RECORD];
    let authority_account_info         = &accounts[AUTHORITY];

    if !authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !permission_record_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;
    let args = SetCounterpartyPauseArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let is_governance = authority_account_info.key == &chancery_config.governance_authority;
    let is_ops        = authority_account_info.key == &chancery_config.operations_authority;
    let is_emergency  = authority_account_info.key == &chancery_config.emergency_authority;

    if !args.is_paused && !(is_governance || is_ops) {
        return Err(ChanceryError::InsufficientRole.into());
    }

    if args.is_paused && !(is_governance || is_ops || is_emergency) {
        return Err(ChanceryError::InsufficientRole.into());
    }

    // Mixed principal/executor settlement roles may coexist on one pathway
    // record. Keep those records containable while rejecting recovery, reserve,
    // governance, and other unrelated capabilities.
    let (record_scope_kind, record_scope_key, previous_bits, final_bits) = {
        let mut record = load_canonical_permission_record_mut(
            permission_record_account_info,
            &args.subject,
            args.scope_kind,
            &args.scope_key,
            &crate::id(),
        )?;

        let roles = role_bits_as_u128(record.role_bits);
        if !is_counterparty_pause_eligible(record.scope_kind, roles) {
            return Err(ChanceryError::PermissionScopeMismatch.into());
        }

        let previous = record.permission_flags & PERMISSION_FLAG_PAUSED;

        if args.is_paused {
            record.permission_flags |= PERMISSION_FLAG_PAUSED;
        } else {
            record.permission_flags &= !PERMISSION_FLAG_PAUSED;
        }

        let final_bits = record.permission_flags & PERMISSION_FLAG_PAUSED;
        (
            record.scope_kind,
            record.scope_key,
            previous,
            final_bits,
        )
    };

    // ── Evidence (invariant #9: emit last) ────────────────────────────────────
    let clock                   = Clock::get()?;
    drop(chancery_config);

    let mut chancery_config_mut  = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump     = chancery_config_mut.event_authority_bump;
    let seq                      = chancery_config_mut.next_sequence_nonce()?;

    emit_pause_state_change(
        event_authority_account_info,
        event_authority_bump,
        PauseStateChange {
            sequence_nonce:       seq,
            chancery:             *chancery_config_account_info.key,
            slot:                 clock.slot,
            unix_timestamp:       clock.unix_timestamp,
            previous_pause_bits:  previous_bits,
            changed_pause_bits:   previous_bits ^ final_bits,
            effective_pause_bits: final_bits,
            is_clear:             !args.is_paused,
            // Scoped pauses carry no stored auto-expiry: they persist until
            // explicitly cleared.
            expires_at_slot:      0,
            reason_code:          args.reason_code,
            acting_authority:     *authority_account_info.key,
            activated_by:         Pubkey::default(),
            scope_kind:           record_scope_kind,
            scope_key:            record_scope_key,
            subject:              args.subject,
        },
    )?;

    Ok(())
}
