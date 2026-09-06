/// initialize_issued_token_control
///
/// Creates and initializes the singleton IssuedTokenControlPda.
/// Records which Token-2022 extensions are reserved for future activation,
/// and pre-derives all authority PDAs.
///
/// Accounts:
///   0  issued_token_control       writable  PDA [b"issued-token-control"]
///   1  chancery_config            writable  PDA (sequence_nonce updated for evidence)
///   2  event_authority            readable  PDA [b"event-authority"]
///   3  payer                      signer    funds rent-exempt reserve
///   4  governance_authority       signer    must be chancery_config.governance_authority
///   5  system_program

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    account_security::assert_external_signer,
    constants::{seeds, status_flag, token_program},
    error::ChanceryError,
    modules::core::{
        instructions::create_pda_account,
        state::chancery_config::ChanceryConfig,
    },
    modules::evidence::emit::{emit_issued_token_control_initialized, IssuedTokenControlInitialized},
    modules::issuance::state::issued_token_control::{
        IssuedTokenControl, ISSUED_TOKEN_CONTROL_DISCRIMINATOR, ISSUED_TOKEN_CONTROL_SIZE,
    },
};

// ─── Account indices ──────────────────────────────────────────────────────────
const ISSUED_TOKEN_CONTROL:   usize = 0;
const CHANCERY_CONFIG:        usize = 1;
const EVENT_AUTHORITY:        usize = 2;
const PAYER:                  usize = 3;
const GOVERNANCE_AUTHORITY:   usize = 4;
const SYSTEM_PROGRAM:         usize = 5;
const REQUIRED_ACCOUNT_COUNT: usize = 6;

// ─── Args ─────────────────────────────────────────────────────────────────────
#[derive(BorshDeserialize)]
pub struct InitializeIssuedTokenControlArgs {
    pub issued_token_mint:               Pubkey,
    pub issued_token_program:            Pubkey,
    pub reserved_mint_extension_mask:    [u64; 2],
    pub reserved_account_extension_mask: [u64; 2],

    /// Max age in slots before issued-token extension observation is stale at
    /// settlement. Zero disables only maximum-age expiry; future observations
    /// remain invalid.
    pub max_extension_observation_age_slots: u64,
}

// ─── Handler ──────────────────────────────────────────────────────────────────
pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let issued_token_control_account_info = &accounts[ISSUED_TOKEN_CONTROL];
    let chancery_config_account_info      = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];
    let payer_account_info                = &accounts[PAYER];
    let governance_authority_account_info = &accounts[GOVERNANCE_AUTHORITY];
    let system_program_account_info       = &accounts[SYSTEM_PROGRAM];

    if !payer_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    assert_external_signer(governance_authority_account_info, &crate::id())?;

    if !issued_token_control_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    // chancery_config is written (sequence_nonce) for evidence emission.
    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    if governance_authority_account_info.key != &chancery_config.governance_authority {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    let args = InitializeIssuedTokenControlArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    if &args.issued_token_mint != &chancery_config.issued_token_mint {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }
    if &args.issued_token_program != &chancery_config.issued_token_program
        || args.issued_token_program != token_program::TOKEN_2022
    {
        return Err(ChanceryError::TokenProgramMismatch.into());
    }

    // Reject any reserved bit that overlaps the default-forbidden mask.
    {
        let m_lo = args.reserved_mint_extension_mask[0];
        let m_hi = args.reserved_mint_extension_mask[1];
        let a_lo = args.reserved_account_extension_mask[0];
        let a_hi = args.reserved_account_extension_mask[1];

        if m_lo & crate::constants::issued_token_default_forbidden::MINT_FORBIDDEN_LO != 0
            || m_hi & crate::constants::issued_token_default_forbidden::MINT_FORBIDDEN_HI != 0
            || a_lo & crate::constants::issued_token_default_forbidden::ACCOUNT_FORBIDDEN_LO != 0
            || a_hi & crate::constants::issued_token_default_forbidden::ACCOUNT_FORBIDDEN_HI != 0
        {
            return Err(ChanceryError::ForbiddenExtension.into());
        }
    }

    let program_id           = crate::id();
    let (expected_key, bump) =
        Pubkey::find_program_address(&[seeds::ISSUED_TOKEN_CONTROL], &program_id);

    if issued_token_control_account_info.key != &expected_key {
        return Err(ChanceryError::InvalidPda.into());
    }

    if issued_token_control_account_info.data_is_empty() {
        create_pda_account(
            payer_account_info,
            issued_token_control_account_info,
            system_program_account_info,
            &program_id,
            &[seeds::ISSUED_TOKEN_CONTROL, &[bump]],
            ISSUED_TOKEN_CONTROL_SIZE,
        )?;
    } else if issued_token_control_account_info.data_len() != ISSUED_TOKEN_CONTROL_SIZE {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    let mut ctl = IssuedTokenControl::load_uninitialized_mut(issued_token_control_account_info)?;

    ctl.discriminator                       = ISSUED_TOKEN_CONTROL_DISCRIMINATOR;
    ctl.version                             = 1;
    ctl.bump                                = bump;
    ctl._pad0                               = [0u8; 5];
    ctl.issued_token_mint                   = args.issued_token_mint;
    ctl.issued_token_program                = args.issued_token_program;

    ctl.reserved_mint_extension_mask        = args.reserved_mint_extension_mask;
    ctl.active_mint_extension_mask          = [0u64; 2];
    ctl.reserved_account_extension_mask     = args.reserved_account_extension_mask;
    ctl.active_account_extension_mask       = [0u64; 2];

    ctl.control_flags                       = status_flag::INITIALIZED;

    // Pre-derive all authority PDAs so callers can validate them on-chain.
    let derive = |seed: &[u8]| -> Pubkey {
        let (pk, _) = Pubkey::find_program_address(&[seed], &program_id);
        pk
    };

    ctl.mint_authority_pda                  = derive(seeds::MINT_AUTHORITY);
    ctl.freeze_authority_pda                = derive(seeds::FREEZE_AUTHORITY);
    ctl.close_mint_authority_pda            = derive(seeds::CLOSE_MINT_AUTHORITY);
    ctl.transfer_hook_authority_pda         = derive(seeds::TRANSFER_HOOK_AUTHORITY);
    ctl.permanent_delegate_authority_pda    = derive(seeds::PERMANENT_DELEGATE_AUTHORITY);
    ctl.metadata_pointer_authority_pda      = derive(seeds::METADATA_POINTER_AUTHORITY);
    ctl.metadata_update_authority_pda       = derive(seeds::METADATA_UPDATE_AUTHORITY);
    ctl.pause_authority_pda                 = derive(seeds::PAUSE_AUTHORITY);
    ctl.confidential_transfer_authority_pda = derive(seeds::CONFIDENTIAL_TRANSFER_AUTHORITY);
    ctl.default_account_state_authority_pda = derive(seeds::DEFAULT_ACCOUNT_STATE_AUTHORITY);

    ctl.hook_program_id                     = Pubkey::default();
    ctl.permanent_delegate                  = Pubkey::default();
    ctl.metadata_address                    = Pubkey::default();

    ctl.last_configured_at_slot             = 0;
    ctl.configured_by                       = *governance_authority_account_info.key;
    ctl.extension_observed_at_slot          = 0;
    ctl.max_extension_observation_age_slots = args.max_extension_observation_age_slots;
    ctl._reserved                           = [0u8; 32];

    // ── Evidence (emitted last, after all state writes succeed) ───────────────
    let clock                   = Clock::get()?;

    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    emit_issued_token_control_initialized(
        event_authority_account_info,
        event_authority_bump,
        IssuedTokenControlInitialized {
            sequence_nonce,
            chancery:                            *chancery_config_account_info.key,
            slot:                                clock.slot,
            unix_timestamp:                      clock.unix_timestamp,
            risk_class:                          0,
            issued_token_control:                *issued_token_control_account_info.key,
            reserved_mint_extension_mask:        args.reserved_mint_extension_mask,
            reserved_account_extension_mask:     args.reserved_account_extension_mask,
            max_extension_observation_age_slots: args.max_extension_observation_age_slots,
            initialized_by:                      *governance_authority_account_info.key,
        },
    )?;

    Ok(())
}
