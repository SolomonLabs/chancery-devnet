/// set_asset_pause
///
/// Sets or clears bits in an AssetPauseState PDA for a single asset.
///
/// Accounts:
///   0  chancery_config    writable  PDA
///   1  event_authority    readable  PDA [b"event-authority"]
///   2  asset_pause_state  writable  PDA [b"asset-pause", asset_mint]
///   3  asset_mint         readable
///   4  payer              signer
///   5  authority          signer    emergency | ops | governance
///   6  system_program

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    constants::{scope, seeds},
    error::ChanceryError,
    modules::{
        control::{
            pause_transition::{compute_pause_clear_transition, compute_pause_set_transition},
            state::asset_pause_state::{
                AssetPauseState, ASSET_PAUSE_STATE_DISCRIMINATOR, ASSET_PAUSE_STATE_SIZE,
            },
        },
        core::{
            instructions::create_pda_account,
            state::chancery_config::ChanceryConfig,
        },
        evidence::emit::{emit_pause_state_change, PauseStateChange},
    },
};

const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const ASSET_PAUSE_STATE:      usize = 2;
const ASSET_MINT:             usize = 3;
const PAYER:                  usize = 4;
const AUTHORITY:              usize = 5;
const SYSTEM_PROGRAM:         usize = 6;
const REQUIRED_ACCOUNT_COUNT: usize = 7;

#[derive(BorshDeserialize)]
pub struct SetAssetPauseArgs {
    pub pause_bits:      u64,
    pub is_clear:        bool,
    pub reason_code:     u32,
    pub expires_at_slot: u64,
}

/// Applies the transition and returns the bits stored BEFORE it, which the
/// evidence event needs to make the transition replayable.
fn apply_asset_pause_transition(
    pause_state:            &mut AssetPauseState,
    args:                   &SetAssetPauseArgs,
    authority:              &Pubkey,
    current_slot:           u64,
    is_emergency_authority: bool,
) -> Result<u64, ProgramError> {
    let previous_bits = pause_state.asset_pause_bits;

    if args.is_clear {
        let transition = compute_pause_clear_transition(
            pause_state.asset_pause_bits,
            pause_state.expires_at_slot,
            args.pause_bits,
            args.expires_at_slot,
            current_slot,
        )?;

        pause_state.asset_pause_bits = transition.new_bits;
        pause_state.expires_at_slot  = transition.new_expires_at_slot;

        if transition.canonicalise {
            pause_state.reason_code       = 0;
            pause_state._pad0             = [0u8; 5];
            pause_state._pad1             = [0u8; 4];
            pause_state.activated_by      = Pubkey::default();
            pause_state.activated_at_slot = 0;
        }

        return Ok(previous_bits);
    }

    let transition = compute_pause_set_transition(
        pause_state.asset_pause_bits,
        pause_state.expires_at_slot,
        args.pause_bits,
        args.expires_at_slot,
        current_slot,
        is_emergency_authority,
    )?;

    pause_state.asset_pause_bits = transition.new_bits;
    pause_state.expires_at_slot  = transition.new_expires_at_slot;

    if transition.write_metadata {
        pause_state.reason_code       = args.reason_code;
        pause_state._pad0             = [0u8; 5];
        pause_state._pad1             = [0u8; 4];
        pause_state.activated_by      = *authority;
        pause_state.activated_at_slot = current_slot;
    }

    Ok(previous_bits)
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info   = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info   = &accounts[EVENT_AUTHORITY];
    let asset_pause_state_account_info = &accounts[ASSET_PAUSE_STATE];
    let asset_mint_account_info        = &accounts[ASSET_MINT];
    let payer_account_info             = &accounts[PAYER];
    let authority_account_info         = &accounts[AUTHORITY];
    let system_program_account_info    = &accounts[SYSTEM_PROGRAM];

    if !payer_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !asset_pause_state_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;
    let args = SetAssetPauseArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let is_governance = authority_account_info.key == &chancery_config.governance_authority;
    let is_ops        = authority_account_info.key == &chancery_config.operations_authority;
    let is_emergency  = authority_account_info.key == &chancery_config.emergency_authority;

    if args.is_clear && !(is_governance || is_ops) {
        return Err(ChanceryError::InsufficientRole.into());
    }

    if !args.is_clear && !(is_governance || is_ops || is_emergency) {
        return Err(ChanceryError::InsufficientRole.into());
    }

    let program_id = crate::id();
    let (expected_key, bump) = Pubkey::find_program_address(
        &[seeds::ASSET_PAUSE, asset_mint_account_info.key.as_ref()],
        &program_id,
    );

    if asset_pause_state_account_info.key != &expected_key {
        return Err(ChanceryError::InvalidPda.into());
    }

    let was_created = asset_pause_state_account_info.data_is_empty();

    // Clearing a never-created pause is an idempotent no-op. Do not allocate
    // an all-zero PDA, consume payer rent, or emit evidence for a transition
    // that did not occur.
    if args.is_clear && was_created {
        return Ok(());
    }

    let previous_bits: u64;

    if was_created {
        create_pda_account(
            payer_account_info,
            asset_pause_state_account_info,
            system_program_account_info,
            &program_id,
            &[
                seeds::ASSET_PAUSE,
                asset_mint_account_info.key.as_ref(),
                &[bump],
            ],
            ASSET_PAUSE_STATE_SIZE,
        )?;
    } else if asset_pause_state_account_info.data_len() != ASSET_PAUSE_STATE_SIZE {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    let clock = Clock::get()?;
    let is_emergency_only = is_emergency && !(is_governance || is_ops);

    if was_created {
        let mut pause_state = AssetPauseState::load_uninitialized_mut(asset_pause_state_account_info)?;
        pause_state.discriminator = ASSET_PAUSE_STATE_DISCRIMINATOR;
        pause_state.version       = 1;
        pause_state.bump          = bump;
        pause_state._pad0         = [0u8; 5];
        pause_state.asset_mint    = *asset_mint_account_info.key;
        pause_state._reserved     = [0u8; 32];

        previous_bits = apply_asset_pause_transition(
            &mut pause_state,
            &args,
            authority_account_info.key,
            clock.slot,
            is_emergency_only,
        )?;
    } else {
        let mut pause_state = AssetPauseState::load_mut_for_verified_pda(
            asset_pause_state_account_info,
            asset_mint_account_info.key,
            bump,
        )?;

        if pause_state.asset_mint != *asset_mint_account_info.key {
            return Err(ChanceryError::InvalidPda.into());
        }

        previous_bits = apply_asset_pause_transition(
            &mut pause_state,
            &args,
            authority_account_info.key,
            clock.slot,
            is_emergency_only,
        )?;
    }

    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    let (final_bits, final_expires_at_slot, final_activated_by) = {
        let pause_state = AssetPauseState::load_for_verified_pda(
            asset_pause_state_account_info,
            asset_mint_account_info.key,
            bump,
        )?;
        (
            pause_state.asset_pause_bits,
            pause_state.expires_at_slot,
            pause_state.activated_by,
        )
    };

    emit_pause_state_change(
        event_authority_account_info,
        event_authority_bump,
        PauseStateChange {
            sequence_nonce,
            chancery:             *chancery_config_account_info.key,
            slot:                 clock.slot,
            unix_timestamp:       clock.unix_timestamp,
            previous_pause_bits:  previous_bits,
            changed_pause_bits:   previous_bits ^ final_bits,
            effective_pause_bits: final_bits,
            is_clear:             args.is_clear,
            expires_at_slot:      final_expires_at_slot,
            reason_code:          args.reason_code,
            acting_authority:     *authority_account_info.key,
            activated_by:         final_activated_by,
            scope_kind:           scope::ASSET,
            scope_key:            *asset_mint_account_info.key,
            subject:              Pubkey::default(),
        },
    )?;

    Ok(())
}
