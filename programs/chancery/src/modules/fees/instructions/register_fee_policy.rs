/// register_fee_policy
///
/// Accounts:
///   0  chancery_config         writable  PDA - seq bump
///   1  event_authority         readable  PDA [b"event-authority"]
///   2  fee_policy              writable  PDA [b"fee-policy", fee_policy_id]
///   3  payer                   signer
///   4  operations_authority    signer
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
        evidence::emit::{emit_fee_policy_registered, FeePolicyRegistered},
        fees::state::fee_policy::{fee_flag, FeePolicy, FEE_POLICY_DISCRIMINATOR, FEE_POLICY_SIZE},
    },
};

const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const FEE_POLICY:             usize = 2;
const PAYER:                  usize = 3;
const OPERATIONS_AUTHORITY:   usize = 4;
const SYSTEM_PROGRAM:         usize = 5;
const REQUIRED_ACCOUNT_COUNT: usize = 6;

#[derive(BorshDeserialize)]
pub struct RegisterFeePolicyArgs {
    pub fee_policy_id:                  [u8; 32],
    pub fee_policy_flags:               u64,
    pub flat_fee_in_asset:              u64,
    pub flat_fee_in_issued_token:       u64,
    pub percent_fee_bps:                u32,
    pub fee_cap_amount:                 u64,
    pub minimum_fee_amount:             u64,
    pub rebate_flat_amount:             u64,
    pub rebate_bps:                     u32,
    pub rebate_cap_amount:              u64,
    pub net_fee_floor_zero:             bool,
    pub fee_recipient_policy:           u8,
    pub rounding_mode:                  u8,
    pub fee_recipient_key:              Pubkey,
    pub effective_from_unix_timestamp:  i64,
    pub effective_until_unix_timestamp: i64,
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info      = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];
    let fee_policy_account_info           = &accounts[FEE_POLICY];
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

    if !fee_policy_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;


    if operations_authority_account_info.key != &chancery_config.operations_authority {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    let args = RegisterFeePolicyArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let program_id           = crate::id();
    let (expected_key, bump) = Pubkey::find_program_address(
        &[seeds::FEE_POLICY, args.fee_policy_id.as_ref()],
        &program_id,
    );

    if fee_policy_account_info.key != &expected_key {
        return Err(ChanceryError::InvalidPda.into());
    }

    if fee_policy_account_info.data_is_empty() {
        create_pda_account(
            payer_account_info, fee_policy_account_info, system_program_account_info, &program_id,
            &[seeds::FEE_POLICY, args.fee_policy_id.as_ref(), &[bump]],
            FEE_POLICY_SIZE,
        )?;
    } else if fee_policy_account_info.data_len() != FEE_POLICY_SIZE {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    let mut p = FeePolicy::load_uninitialized_mut(fee_policy_account_info)?;

    p.discriminator                  = FEE_POLICY_DISCRIMINATOR;
    p.version                        = 1;
    p.bump                           = bump;
    p.fee_recipient_policy           = args.fee_recipient_policy;
    p.rounding_mode                  = args.rounding_mode;
    p.net_fee_floor_zero             = args.net_fee_floor_zero as u8;
    p._pad0                          = [0u8; 2];
    p.fee_policy_id                  = args.fee_policy_id;
    // issue-75: registration is inert. Strip ACTIVE so a newly registered fee
    // policy is not effective on the spot (assert_effective returns
    // FeePolicyNotFound while the flag is clear); activation is a governed step.
    p.fee_policy_flags               = args.fee_policy_flags & !fee_flag::ACTIVE;
    p.flat_fee_in_asset              = args.flat_fee_in_asset;
    p.flat_fee_in_issued_token       = args.flat_fee_in_issued_token;
    p.percent_fee_bps                = args.percent_fee_bps;
    p._pad1                          = [0u8; 4];
    p.fee_cap_amount                 = args.fee_cap_amount;
    p.minimum_fee_amount             = args.minimum_fee_amount;
    p.rebate_flat_amount             = args.rebate_flat_amount;
    p.rebate_bps                     = args.rebate_bps;
    p._pad2                          = [0u8; 4];
    p.rebate_cap_amount              = args.rebate_cap_amount;
    p.fee_recipient_key              = args.fee_recipient_key;
    p.effective_from_unix_timestamp  = args.effective_from_unix_timestamp;
    p.effective_until_unix_timestamp = args.effective_until_unix_timestamp;
    p._reserved                      = [0u8; 32];

    p.assert_parameter_sanity()?;
    // Reject policies whose flag / flat-fee fields disagree about denomination.
    p.assert_denomination_consistent()?;

    // Drop the mutable borrow on fee_policy account data before the next borrows.
    drop(p);

    // ── Evidence (emitted last, after all state writes) ─────────────────────────
    let clock = Clock::get()?;

    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    emit_fee_policy_registered(
        event_authority_account_info,
        event_authority_bump,
        FeePolicyRegistered {
            sequence_nonce,
            chancery:       *chancery_config_account_info.key,
            slot:           clock.slot,
            unix_timestamp: clock.unix_timestamp,
            risk_class:     0,
            fee_policy:     *fee_policy_account_info.key,
            registered_by:  *operations_authority_account_info.key,
        },
    )?;

    Ok(())
}
