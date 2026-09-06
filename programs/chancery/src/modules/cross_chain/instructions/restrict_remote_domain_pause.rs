/// restrict_remote_domain_pause
///
/// Sets one or more `remote_domain_pause_bit::*` bits on a
/// `RemoteDomainPolicy.status_flags`. OR-only - never clears bits, and
/// never touches `status_flag::INITIALIZED`. Emits `PauseStateChange`
/// with the post-mutation pause-bit slice (excluding the INITIALIZED bit).
///
/// Per spec 10 §10.10 this is the emergency-tier op: the role bit gates a
/// strictly-narrowing transition. Relaxation uses the separate
/// `CAN_RELAX_REMOTE_DOMAIN_PAUSE` role but can clear bits only through
/// `relax_remote_domain_pause_with_pending_change` after governance acceptance.
///
/// `assert_not_paused` on `ChanceryConfig` is intentionally NOT checked -
/// pause control must work even when the chancery is globally paused.
///
/// Accounts:
///   0  chancery_config                writable  PDA  (sequence_nonce incremented for evidence)
///   1  event_authority                readable  PDA [b"event-authority"]
///   2  remote_domain_policy           writable  PDA [b"remote-domain-policy", chain_kind, domain_id_be]
///   3  authority                      signer
///   4  authority_permission_record    readable  PDA [b"permission", authority, REMOTE_DOMAIN, policy_pda]
///
/// Authority: signer must hold `CAN_RESTRICT_REMOTE_DOMAIN_PAUSE` on a
/// `PermissionRecord` at `(scope::REMOTE_DOMAIN, policy_pda)`. The
/// scope_key MUST be the RemoteDomainPolicy PDA address - this binds
/// emergency-pause delegation per-domain.

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
        core::state::chancery_config::ChanceryConfig,
        cross_chain::{
            auth::assert_signer_holds_role,
            instructions::{
                assert_disjoint_cross_chain_account_index_groups,
                assert_distinct_cross_chain_account_indexes,
            },
            state::remote_domain_policy::RemoteDomainPolicy,
        },
        evidence::emit::{emit_pause_state_change, PauseStateChange},
    },
};

const CHANCERY_CONFIG:                usize = 0;
const EVENT_AUTHORITY:                usize = 1;
const REMOTE_DOMAIN_POLICY:           usize = 2;
const AUTHORITY:                      usize = 3;
const AUTHORITY_PERMISSION_RECORD:    usize = 4;
const REQUIRED_ACCOUNT_COUNT:         usize = 5;

const PROTECTED_ACCOUNT_INDEXES:   &[usize] = &[
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    REMOTE_DOMAIN_POLICY,
];
const IDENTITY_ACCOUNT_INDEXES:   &[usize] = &[AUTHORITY];
const PERMISSION_ACCOUNT_INDEXES: &[usize] = &[AUTHORITY_PERMISSION_RECORD];

#[derive(BorshDeserialize)]
pub struct RestrictRemoteDomainPauseArgs {
    pub remote_chain_kind: u8,
    pub remote_domain_id:  u64,

    /// Bits to OR into `status_flags`. Only `remote_domain_pause_bit::*`
    /// values take effect - any other bits in this argument are masked off
    /// inside `restrict_pause_bits`.
    pub pause_bits_to_set: u64,
    pub reason_code:       u32,
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

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !policy_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let args = RestrictRemoteDomainPauseArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    // ── Verify the policy PDA matches the args ────────────────────────────────
    let program_id = crate::id();

    let remote_policy_bump = RemoteDomainPolicy::verify_pda(
        policy_account_info,
        args.remote_chain_kind,
        args.remote_domain_id,
        &program_id,
    )?;

    // ── Authority: CAN_RESTRICT_REMOTE_DOMAIN_PAUSE @ (REMOTE_DOMAIN, policy) ─
    assert_signer_holds_role(
        authority_account_info,
        authority_permission_record_account_info,
        role::CAN_RESTRICT_REMOTE_DOMAIN_PAUSE,
        scope::REMOTE_DOMAIN,
        policy_account_info.key,
        &program_id,
    )?;

    let clock = Clock::get()?;

    // ── Apply restriction (OR-only, masked to pause bits) ────────────────────
    let previous_pause_bits = {
        let mut p = RemoteDomainPolicy::load_mut_for_verified_pda(
            policy_account_info,
            args.remote_chain_kind,
            args.remote_domain_id,
            remote_policy_bump,
        )?;

        let previous = p.status_flags & remote_domain_pause_bit::ALL;
        p.restrict_pause_bits(args.pause_bits_to_set)?;
        p.updated_at_slot = clock.slot;
        previous
    };

    // ── Emit evidence ─────────────────────────────────────────────────────────
    let mut chancery_config_mut  = ChanceryConfig::load_verified_mut(chancery_config_account_info)?;
    let event_authority_bump = chancery_config_mut.event_authority_bump;
    let seq                  = chancery_config_mut.next_sequence_nonce()?;

    let final_pause_bits = {
        let p = RemoteDomainPolicy::load_for_verified_pda(
            policy_account_info,
            args.remote_chain_kind,
            args.remote_domain_id,
            remote_policy_bump,
        )?;

        p.status_flags & remote_domain_pause_bit::ALL
    };

    emit_pause_state_change(
        event_authority_account_info,
        event_authority_bump,
        PauseStateChange {
            sequence_nonce:       seq,
            chancery:             *chancery_config_account_info.key,
            slot:                 clock.slot,
            unix_timestamp:       clock.unix_timestamp,
            previous_pause_bits,
            changed_pause_bits:   previous_pause_bits ^ final_pause_bits,
            effective_pause_bits: final_pause_bits,
            is_clear:             false,
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
