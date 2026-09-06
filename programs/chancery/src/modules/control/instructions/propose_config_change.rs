//! propose_config_change.
//!
//! Creates a `PendingConfigChange` PDA in PROPOSED status. Widening+ risk
//! requires governance signature. `expires_at_unix_timestamp` must be strictly
//! greater than `executable_after_unix_timestamp`.
//!
//! Wire format:
//!   [ CONTROL(0x09) | PROPOSE_CONFIG_CHANGE(0x07) | borsh(args) ]
//!
//! Accounts:
//!   0  module_activation_state    readable  PDA  (slot 0 per the specification)
//!   1  chancery_config            writable  PDA  (sequence_nonce updated for evidence)
//!   2  event_authority            readable  PDA [b"event-authority"]
//!   3  pending_config_change      writable  PDA [b"pending-config-change", change_id]
//!   4  payer                      signer
//!   5  governance_authority       signer    (Widening+ requires governance)
//!   6  system_program

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    constants::{module, seeds},
    error::ChanceryError,
    modules::{
        core::{instructions::create_pda_account, state::chancery_config::ChanceryConfig},
        control::{
            change_risk::ConfigChangeRiskClass,
            pending_change::{compute_change_id, minimum_timelock_seconds_for_risk},
            state::{
                module_activation_state::ModuleActivationState,
                pending_config_change::{
                    config_change_status, PendingConfigChange,
                    PENDING_CONFIG_CHANGE_DISCRIMINATOR, PENDING_CONFIG_CHANGE_SIZE,
                },
            },
        },
        evidence::emit::{emit_config_change_proposed, ConfigChangeProposed},
    },
};

const ACTIVATION_STATE:       usize = 0;
const CHANCERY_CONFIG:        usize = 1;
const EVENT_AUTHORITY:        usize = 2;
const PENDING:                usize = 3;
const PAYER:                  usize = 4;
const GOVERNANCE_AUTHORITY:   usize = 5;
const SYSTEM:                 usize = 6;
const REQUIRED_ACCOUNT_COUNT: usize = 7;

#[derive(BorshDeserialize)]
pub struct ProposeConfigChangeArgs {
    pub change_kind:                     u16,
    pub risk_class:                      u8,
    pub target_account:                  Pubkey,
    pub old_value_hash:                  [u8; 32],
    pub new_value_hash:                  [u8; 32],
    pub executable_after_unix_timestamp: i64,
    pub expires_at_unix_timestamp:       i64,
    pub proposer_nonce:                  u64,
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let activation_account_info           = &accounts[ACTIVATION_STATE];
    let cfg_account_info                  = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];
    let pending_account_info              = &accounts[PENDING];
    let payer_account_info                = &accounts[PAYER];
    let governance_authority_account_info = &accounts[GOVERNANCE_AUTHORITY];
    let system_account_info               = &accounts[SYSTEM];

    if !payer_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !governance_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !cfg_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !pending_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let activation = ModuleActivationState::load_verified(activation_account_info)?;

    activation.assert_active(module::CONTROL)?;

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(cfg_account_info)?;

    let args = ProposeConfigChangeArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let risk = ConfigChangeRiskClass::try_from_u8(args.risk_class)?;

    if !risk.requires_timelock() {
        return Err(ChanceryError::ConfigChangeRequiresTimelock.into());
    }

    if governance_authority_account_info.key != &chancery_config.governance_authority {
        return Err(ChanceryError::ConfigChangeWideningRequiresGovernance.into());
    }

    let clock = Clock::get()?;

    let minimum_delay       = minimum_timelock_seconds_for_risk(risk);
    let earliest_executable = clock.unix_timestamp.saturating_add(minimum_delay);

    if args.executable_after_unix_timestamp < earliest_executable {
        return Err(ChanceryError::ConfigChangeTimelockNotElapsed.into());
    }

    if args.expires_at_unix_timestamp <= args.executable_after_unix_timestamp {
        return Err(ChanceryError::ConfigChangeExpired.into());
    }

    let change_id = compute_change_id(
        args.change_kind,
        &args.target_account,
        &args.new_value_hash,
        governance_authority_account_info.key,
        args.proposer_nonce,
    );

    let program_id = crate::id();
    let (expected_pda, bump) = PendingConfigChange::pda(&change_id, &program_id);

    if pending_account_info.key != &expected_pda {
        return Err(ChanceryError::InvalidPda.into());
    }

    // ── Fresh creation only ───────────────────────────────────────────────────
    // Consume, cancel, and expiry close terminal records (spec 14 §14.5.3).
    // Re-proposal under the same change_id is permitted only when that exact
    // historical address is fresh and available, and it still requires the
    // full acceptance + timelock ceremony. Normal retry uses a fresh
    // proposer_nonce (spec 14 §14.3.1). Any non-empty account here is either a
    // live (PROPOSED/ACCEPTED) record or an allocated zeroed shell - both
    // reject: initializing over a defunded, about-to-be-reaped account would
    // emit proposal evidence for state that vanishes with the transaction.
    if !pending_account_info.data_is_empty() {
        return Err(ChanceryError::AlreadyInitialized.into());
    }

    create_pda_account(
        payer_account_info,
        pending_account_info,
        system_account_info,
        &program_id,
        &[seeds::PENDING_CONFIG_CHANGE, &change_id, &[bump]],
        PENDING_CONFIG_CHANGE_SIZE,
    )?;

    let mut p = PendingConfigChange::load_uninitialized_mut(pending_account_info)?;

    p.discriminator                   = PENDING_CONFIG_CHANGE_DISCRIMINATOR;
    p.version                         = 1;
    p.bump                            = bump;
    p.status                          = config_change_status::PROPOSED;
    p.risk_class                      = risk.as_u8();
    p._pad0                           = [0u8; 3];
    p.change_id                       = change_id;
    p.change_kind                     = args.change_kind;
    p._pad1                           = [0u8; 6];
    p.target_account                  = args.target_account;
    p.old_value_hash                  = args.old_value_hash;
    p.new_value_hash                  = args.new_value_hash;
    p.proposed_by                     = *governance_authority_account_info.key;
    p.proposer_nonce                  = args.proposer_nonce;
    p.executable_after_unix_timestamp = args.executable_after_unix_timestamp;
    p.expires_at_unix_timestamp       = args.expires_at_unix_timestamp;
    p.proposed_at_slot                = clock.slot;
    p.accepted_at_slot                = 0;
    p.cancelled_at_slot               = 0;
    p.consumed_at_slot                = 0;
    p.last_event_sequence_nonce       = 0;
    p.rent_refund_recipient           = *payer_account_info.key;
    p._reserved                       = [0u8; 16];

    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(cfg_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    p.last_event_sequence_nonce = sequence_nonce;

    emit_config_change_proposed(
        event_authority_account_info,
        event_authority_bump,
        ConfigChangeProposed {
            sequence_nonce,
            chancery:                        *cfg_account_info.key,
            slot:                            clock.slot,
            unix_timestamp:                  clock.unix_timestamp,
            risk_class:                      risk.as_u8(),
            change_id,
            change_kind:                     args.change_kind,
            target_account:                  args.target_account,
            old_value_hash:                  args.old_value_hash,
            new_value_hash:                  args.new_value_hash,
            proposed_by:                     *governance_authority_account_info.key,
            proposer_nonce:                  args.proposer_nonce,
            executable_after_unix_timestamp: args.executable_after_unix_timestamp,
            expires_at_unix_timestamp:       args.expires_at_unix_timestamp,
        },
    )?;

    Ok(())
}
