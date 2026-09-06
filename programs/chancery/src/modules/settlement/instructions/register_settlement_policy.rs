/// register_settlement_policy
///
/// Settlement policies are immutable once registered. Changing any binding
/// requires a new content/policy ID and a new account, preventing silent policy
/// substitution beneath an already-created intent.
///
/// Accounts:
///   0  chancery_config       writable  PDA - sequence nonce
///   1  event_authority       readable  PDA [b"event-authority"]
///   2  settlement_policy     writable  PDA [b"settlement-policy", policy_id]
///   3  payer                 signer
///   4  operations_authority  signer
///   5  system_program

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    constants::seeds,
    error::ChanceryError,
    modules::{
        core::{instructions::create_pda_account, state::chancery_config::ChanceryConfig},
        evidence::emit::{emit_settlement_policy_registered, SettlementPolicyRegistered},
        settlement::state::settlement_policy::{
            SettlementPolicy, SETTLEMENT_POLICY_DISCRIMINATOR, SETTLEMENT_POLICY_SIZE,
        },
    },
};

const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const SETTLEMENT_POLICY:      usize = 2;
const PAYER:                  usize = 3;
const OPERATIONS_AUTHORITY:   usize = 4;
const SYSTEM_PROGRAM:         usize = 5;
const REQUIRED_ACCOUNT_COUNT: usize = 6;

#[derive(BorshDeserialize)]
pub struct RegisterSettlementPolicyArgs {
    pub policy_id:                  [u8; 32],
    pub policy_flags:               u64,
    pub allowed_settlement_modes:   u32,
    pub allowed_asset_mint:         Pubkey,
    pub allowed_principal_a:        Pubkey,
    pub allowed_principal_b:        Pubkey,
    pub designated_executor:        Pubkey,
    pub max_notional:               u64,
    pub min_notional:               u64,
    pub valid_after_unix_timestamp: i64,
    pub expires_at_unix_timestamp:  i64,
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info      = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];
    let settlement_policy_account_info    = &accounts[SETTLEMENT_POLICY];
    let payer_account_info                = &accounts[PAYER];
    let operations_authority_account_info = &accounts[OPERATIONS_AUTHORITY];
    let system_program_account_info       = &accounts[SYSTEM_PROGRAM];

    if !payer_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !operations_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !settlement_policy_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;
    if operations_authority_account_info.key != &chancery_config.operations_authority {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    let args = RegisterSettlementPolicyArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let program_id = crate::id();
    let (expected_key, bump) = SettlementPolicy::pda(&args.policy_id, &program_id);
    if settlement_policy_account_info.key != &expected_key {
        return Err(ChanceryError::InvalidPda.into());
    }

    if settlement_policy_account_info.data_is_empty() {
        create_pda_account(
            payer_account_info,
            settlement_policy_account_info,
            system_program_account_info,
            &program_id,
            &[seeds::SETTLEMENT_POLICY, args.policy_id.as_ref(), &[bump]],
            SETTLEMENT_POLICY_SIZE,
        )?;
    } else if settlement_policy_account_info.data_len() != SETTLEMENT_POLICY_SIZE {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    let mut policy = SettlementPolicy::load_uninitialized_mut(settlement_policy_account_info)?;

    policy.discriminator              = SETTLEMENT_POLICY_DISCRIMINATOR;
    policy.version                    = 1;
    policy.bump                       = bump;
    policy._pad0                      = [0u8; 5];
    policy.policy_flags               = args.policy_flags;
    policy.allowed_settlement_modes   = args.allowed_settlement_modes;
    policy._pad1                      = [0u8; 4];
    policy.policy_id                  = args.policy_id;
    policy.allowed_asset_mint         = args.allowed_asset_mint;
    policy.allowed_principal_a        = args.allowed_principal_a;
    policy.allowed_principal_b        = args.allowed_principal_b;
    policy.designated_executor        = args.designated_executor;
    policy.max_notional               = args.max_notional;
    policy.min_notional               = args.min_notional;
    policy.valid_after_unix_timestamp = args.valid_after_unix_timestamp;
    policy.expires_at_unix_timestamp  = args.expires_at_unix_timestamp;
    policy.created_by                 = *operations_authority_account_info.key;
    policy._reserved                  = [0u8; 32];

    policy.assert_parameter_sanity()?;
    drop(policy);

    let clock                   = Clock::get()?;
    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    emit_settlement_policy_registered(
        event_authority_account_info,
        event_authority_bump,
        SettlementPolicyRegistered {
            sequence_nonce,
            chancery:          *chancery_config_account_info.key,
            slot:              clock.slot,
            unix_timestamp:    clock.unix_timestamp,
            risk_class:        0,
            settlement_policy: *settlement_policy_account_info.key,
            policy_id:         args.policy_id,
            registered_by:     *operations_authority_account_info.key,
        },
    )?;

    Ok(())
}
