/// set_global_pause
///
/// Sets or clears bits in the singleton PauseState PDA.
///
/// Accounts:
///   0  chancery_config  writable  PDA  (sequence_nonce updated for evidence)
///   1  event_authority  readable  PDA [b"event-authority"] (signs evidence CPI)
///   2  pause_state      writable  PDA [b"pause-state"]
///   3  authority        signer    emergency, ops, or governance
///
/// Authority rules:
///   set (add bits)   -> emergency | ops | governance
///   clear (rm bits)  -> ops | governance only
///   Narrowest scope: emergency can only add bits (more restrictive), never clear.

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    constants::scope,
    error::ChanceryError,
    modules::{
        control::{
            pause_transition::{compute_pause_clear_transition, compute_pause_set_transition},
            state::pause_state::PauseState,
        },
        core::state::chancery_config::ChanceryConfig,
        evidence::emit::{emit_pause_state_change, PauseStateChange},
    },
};

// ─── Account indices ──────────────────────────────────────────────────────────
const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const PAUSE_STATE:            usize = 2;
const AUTHORITY:              usize = 3;
const REQUIRED_ACCOUNT_COUNT: usize = 4;

// ─── Args ─────────────────────────────────────────────────────────────────────
#[derive(BorshDeserialize)]
pub struct SetGlobalPauseArgs {
    /// Bits to set OR clear - interpretation depends on `is_clear`.
    pub pause_bits:      u64,
    /// If false: OR these bits in (pause). If true: AND NOT these bits (unpause).
    pub is_clear   : bool,
    pub reason_code: u32,
    /// Auto-expiry: slot after which pause lifts without an explicit clear.
    /// 0 = no auto-expiry.
    pub expires_at_slot: u64,
}

// ─── Handler ──────────────────────────────────────────────────────────────────
pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info = &accounts[EVENT_AUTHORITY];
    let pause_state_account_info     = &accounts[PAUSE_STATE];
    let authority_account_info       = &accounts[AUTHORITY];

    if !authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !pause_state_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    let args = SetGlobalPauseArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let is_governance = authority_account_info.key == &chancery_config.governance_authority;
    let is_ops        = authority_account_info.key == &chancery_config.operations_authority;
    let is_emergency  = authority_account_info.key == &chancery_config.emergency_authority;

    // Emergency cannot clear - only governance or ops can lift a pause.
    if args.is_clear && !(is_governance || is_ops) {
        return Err(ChanceryError::InsufficientRole.into());
    }

    if !args.is_clear && !(is_governance || is_ops || is_emergency) {
        return Err(ChanceryError::InsufficientRole.into());
    }

    let pause_state_bump = PauseState::verify_pda(pause_state_account_info, &crate::id())?;

    let clock = Clock::get()?;

    // ── Write state ───────────────────────────────────────────────────────────
    let previous_bits: u64;

    {
        let mut pause_state = PauseState::load_mut_for_verified_pda(pause_state_account_info, pause_state_bump)?;

        previous_bits = pause_state.global_pause_bits;

        if args.is_clear {
            let transition = compute_pause_clear_transition(
                pause_state.global_pause_bits,
                pause_state.expires_at_slot,
                args.pause_bits,
                args.expires_at_slot,
                clock.slot,
            )?;

            pause_state.global_pause_bits = transition.new_bits;
            pause_state.expires_at_slot   = transition.new_expires_at_slot;

            if transition.canonicalise {
                pause_state.reason_code       = 0;
                pause_state._pad0             = [0u8; 5];
                pause_state._pad1             = [0u8; 4];
                pause_state.activated_by      = Pubkey::default();
                pause_state.activated_at_slot = 0;
            }
        } else {
            let transition = compute_pause_set_transition(
                pause_state.global_pause_bits,
                pause_state.expires_at_slot,
                args.pause_bits,
                args.expires_at_slot,
                clock.slot,
                is_emergency && !(is_governance || is_ops),
            )?;

            pause_state.global_pause_bits = transition.new_bits;
            pause_state.expires_at_slot   = transition.new_expires_at_slot;

            if transition.write_metadata {
                pause_state.reason_code       = args.reason_code;
                pause_state._pad0             = [0u8; 5];
                pause_state._pad1             = [0u8; 4];
                pause_state.activated_by      = *authority_account_info.key;
                pause_state.activated_at_slot = clock.slot;
            }
        }
    }

    // ── Emit evidence ─────────────────────────────────────────────────────────
    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let seq                     = chancery_config_mut.next_sequence_nonce()?;

    let (final_bits, final_expires_at_slot, final_activated_by) = {
        let pause_state = PauseState::load_for_verified_pda(
            pause_state_account_info,
            pause_state_bump,
        )?;
        (
            pause_state.global_pause_bits,
            pause_state.expires_at_slot,
            pause_state.activated_by,
        )
    };

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
            is_clear:             args.is_clear,
            expires_at_slot:      final_expires_at_slot,
            // The reason this transition was submitted, not whatever the
            // episode record happens to still hold.
            reason_code:          args.reason_code,
            acting_authority:     *authority_account_info.key,
            activated_by:         final_activated_by,
            scope_kind:           scope::GLOBAL,
            scope_key:            Pubkey::default(),
            subject:              Pubkey::default(),
        },
    )?;

    Ok(())
}
