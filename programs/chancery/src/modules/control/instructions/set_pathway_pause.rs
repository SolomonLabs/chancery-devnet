/// set_pathway_pause
///
/// Sets or clears the PATHWAY_PAUSE bit on a PathwayPolicy PDA.
/// Does not require a separate pause state account - the flag lives on
/// the pathway record itself, matching the spec 06 §6.7 "pathway pause via
/// policy or flags" model.
///
/// Accounts:
///   0  chancery_config    writable  PDA  (sequence_nonce bump for evidence)
///   1  event_authority    readable  PDA [b"event-authority"]
///   2  pathway_policy     writable  PDA [b"pathway-policy", pathway_id]
///   3  authority          signer    ops | governance (clear) | + emergency (set)

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    constants::{scope, status_flag},
    error::ChanceryError,
    modules::{
        core::state::chancery_config::ChanceryConfig,
        evidence::emit::{emit_pause_state_change, PauseStateChange},
        pathway::state::pathway_policy::PathwayPolicy,
    },
};

const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const PATHWAY_POLICY:         usize = 2;
const AUTHORITY:              usize = 3;
const REQUIRED_ACCOUNT_COUNT: usize = 4;

#[derive(BorshDeserialize)]
pub struct SetPathwayPauseArgs {
    pub pathway_id:  [u8; 32],
    pub is_paused:   bool,
    pub reason_code: u32,
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info = &accounts[EVENT_AUTHORITY];
    let pathway_policy_account_info  = &accounts[PATHWAY_POLICY];
    let authority_account_info       = &accounts[AUTHORITY];

    if !authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !pathway_policy_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    let args = SetPathwayPauseArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let is_governance = authority_account_info.key == &chancery_config.governance_authority;
    let is_ops        = authority_account_info.key == &chancery_config.operations_authority;
    let is_emergency  = authority_account_info.key == &chancery_config.emergency_authority;

    // Clearing (un-pausing) requires ops or governance.
    if !args.is_paused && !(is_governance || is_ops) {
        return Err(ChanceryError::InsufficientRole.into());
    }

    if args.is_paused && !(is_governance || is_ops || is_emergency) {
        return Err(ChanceryError::InsufficientRole.into());
    }

    // Verify PDA key matches the supplied pathway_id.
    let program_id = crate::id();
    let (expected_key, pathway_policy_bump) = solana_pubkey::Pubkey::find_program_address(
        &[crate::constants::seeds::PATHWAY_POLICY, args.pathway_id.as_ref()],
        &program_id,
    );

    if pathway_policy_account_info.key != &expected_key {
        return Err(ChanceryError::InvalidPda.into());
    }

    let (previous_bits, final_bits) = {
        let mut policy = PathwayPolicy::load_mut_for_verified_pda(
            pathway_policy_account_info,
            &args.pathway_id,
            pathway_policy_bump,
        )?;

        let previous = policy.status_flags & status_flag::PATHWAY_PAUSE;

        if args.is_paused {
            policy.status_flags |= status_flag::PATHWAY_PAUSE;
        } else {
            policy.status_flags &= !status_flag::PATHWAY_PAUSE;
        }

        (previous, policy.status_flags & status_flag::PATHWAY_PAUSE)
    };

    // ── Evidence (invariant #9: emit last) ────────────────────────────────────
    let clock                   = Clock::get()?;
    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let seq                     = chancery_config_mut.next_sequence_nonce()?;

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
            // Pathway pauses carry no stored auto-expiry.
            expires_at_slot:      0,
            reason_code:          args.reason_code,
            acting_authority:     *authority_account_info.key,
            activated_by:         Pubkey::default(),
            scope_kind:           scope::PATHWAY,
            scope_key:            *pathway_policy_account_info.key,
            subject:              Pubkey::default(),
        },
    )?;

    Ok(())
}
