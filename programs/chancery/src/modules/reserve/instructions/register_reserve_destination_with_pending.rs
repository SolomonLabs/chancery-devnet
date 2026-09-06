//! register_reserve_destination_with_pending.
//!
//! Consumes an accepted `PendingConfigChange` of kind
//! `change_kind::REGISTER_RESERVE_DESTINATION` (risk = Dangerous) and creates
//! the `ReserveDestination` PDA. Governance proposes + accepts + consumes;
//! ops is out of the registration loop entirely.
//!
//! Wire format:
//!   [ RESERVE(0x08) | REGISTER_RESERVE_DESTINATION_WITH_PENDING(0x04) | borsh(args) ]
//!
//! Accounts:
//!   0  chancery_config            writable  PDA - seq bump
//!   1  event_authority            readable  PDA [b"event-authority"]
//!   2  pending_config_change      writable  PDA
//!   3  reserve_destination        writable  PDA [b"reserve-destination", asset_mint, destination_token_account]
//!   4  asset_mint                 readable
//!   5  destination_token_account  readable
//!   6  payer                      signer
//!   7  governance_authority       signer
//!   8  system_program
//!   9  withdrawal_limit_policy readable  PDA [b"limit-policy", withdrawal_limit_policy_id]

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    account_security::assert_external_identity,
    constants::{change_kind, reserve_destination_status, seeds, token_program},
    error::ChanceryError,
    modules::{
        control::{
            change_risk::ConfigChangeRiskClass,
            pending_change::{
                assert_accepted_pending_change, compute_config_change_hash, consume_and_close_pending_change,
            },
        },
        core::{instructions::create_pda_account, state::chancery_config::ChanceryConfig},
        evidence::emit::{emit_reserve_destination_registered, ReserveDestinationRegistered},
        limits::state::limit_policy::LimitPolicy,
        reserve::state::reserve_destination::{
            ReserveDestination, RESERVE_DESTINATION_DISCRIMINATOR,
            RESERVE_DESTINATION_RESERVED_SIZE, RESERVE_DESTINATION_SIZE,
        },
    },
};

const CHANCERY_CONFIG:           usize = 0;
const EVENT_AUTHORITY:           usize = 1;
const PENDING:                   usize = 2;
const RESERVE_DESTINATION:       usize = 3;
const ASSET_MINT:                usize = 4;
const DESTINATION_TOKEN_ACCOUNT: usize = 5;
const PAYER:                     usize = 6;
const GOVERNANCE_AUTHORITY:      usize = 7;
const SYSTEM_PROGRAM:            usize = 8;
const WITHDRAWAL_LIMIT_POLICY:   usize = 9;
const REQUIRED_ACCOUNT_COUNT:    usize = 10;

// Refund conveyance: the account whose key equals the record's stored
// rent_refund_recipient. Required whenever this instruction closes the
// record - the refund is conveyed, never redirected.
const RENT_REFUND_RECIPIENT:     usize = 10;

#[derive(BorshDeserialize)]
pub struct RegisterReserveDestinationWithPendingArgs {
    pub destination_owner:           Pubkey,
    pub destination_flags:           u64,
    pub withdrawal_limit_policy_id:  [u8; 32],
}

fn assert_destination_owner_is_external(destination_owner: &Pubkey) -> ProgramResult {
    assert_external_identity(destination_owner, &crate::id())
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info      = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];
    let pending_account_info              = &accounts[PENDING];
    let reserve_destination_account_info  = &accounts[RESERVE_DESTINATION];
    let asset_mint_account_info           = &accounts[ASSET_MINT];
    let destination_token_account_info    = &accounts[DESTINATION_TOKEN_ACCOUNT];
    let payer_account_info                = &accounts[PAYER];
    let governance_authority_account_info = &accounts[GOVERNANCE_AUTHORITY];
    let system_program_account_info       = &accounts[SYSTEM_PROGRAM];
    let withdrawal_limit_policy_account_info = &accounts[WITHDRAWAL_LIMIT_POLICY];

    if !payer_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !governance_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !pending_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !reserve_destination_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    if governance_authority_account_info.key != &chancery_config.governance_authority {
        return Err(ChanceryError::ConfigChangeWideningRequiresGovernance.into());
    }

    let args = RegisterReserveDestinationWithPendingArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    // Reserve destinations are external payees. A Chancery signer PDA can
    // receive tokens, but no reserve instruction can authorize that PDA-owned
    // destination account to send them back out.
    assert_destination_owner_is_external(&args.destination_owner)?;

    // ── Destination token account: validate program owner + mint/owner binding ─
    // SPL Token / Token-2022 base layout: bytes 0..32 = mint, bytes 32..64 = owner.
    // The owning program is read directly off the account (no extra account in
    // the instruction); guards against registering a destination whose token
    // account is not a real SPL token account or does not belong to this
    // asset's mint / the stated owner.
    if !token_program::is_accepted(destination_token_account_info.owner) {
        return Err(ProgramError::IncorrectProgramId);
    }

    {
        let token_data = destination_token_account_info.try_borrow_data()?;

        if token_data.len() < 64 {
            return Err(ChanceryError::AccountDataLengthMismatch.into());
        }

        if &token_data[0..32] != asset_mint_account_info.key.as_ref() {
            return Err(ChanceryError::ReserveDestinationTokenMintMismatch.into());
        }

        if &token_data[32..64] != args.destination_owner.as_ref() {
            return Err(ChanceryError::ReserveDestinationOwnerMismatch.into());
        }
    }

    ReserveDestination::validate_purpose_flags(args.destination_flags)?;

    // ── Derive + verify the target PDA address ────────────────────────────────
    let program_id = crate::id();
    let (expected_reserve_destination_key, bump) = Pubkey::find_program_address(
        &[
            seeds::RESERVE_DESTINATION,
            asset_mint_account_info.key.as_ref(),
            destination_token_account_info.key.as_ref(),
        ],
        &program_id,
    );

    if reserve_destination_account_info.key != &expected_reserve_destination_key {
        return Err(ChanceryError::InvalidPda.into());
    }

    let limit_policy_bump = LimitPolicy::verify_pda(
        withdrawal_limit_policy_account_info,
        &args.withdrawal_limit_policy_id,
        &program_id,
    )?;
    LimitPolicy::load_for_verified_pda(
        withdrawal_limit_policy_account_info,
        &args.withdrawal_limit_policy_id,
        limit_policy_bump,
    )?
        .assert_reserve_withdrawal_caps(&expected_reserve_destination_key)?;

    // asset_mint + destination_token_account are bound by the target PDA;
    // the complete mutable semantic value (owner, purpose, withdrawal policy)
    // is committed into the pending-change payload.
    let mut new_payload = [0u8; 72];
    new_payload[0..32].copy_from_slice(args.destination_owner.as_ref());
    new_payload[32..40].copy_from_slice(&args.destination_flags.to_be_bytes());
    new_payload[40..72].copy_from_slice(&args.withdrawal_limit_policy_id);

    let old_payload: [u8; 72] = [0u8; 72];

    let risk     = ConfigChangeRiskClass::Dangerous;
    let old_hash = compute_config_change_hash(
        change_kind::REGISTER_RESERVE_DESTINATION,
        &expected_reserve_destination_key,
        risk.as_u8(),
        &old_payload,
    );
    let new_hash = compute_config_change_hash(
        change_kind::REGISTER_RESERVE_DESTINATION,
        &expected_reserve_destination_key,
        risk.as_u8(),
        &new_payload,
    );

    // ── Consume the accepted pending change ───────────────────────────────────
    let pending = assert_accepted_pending_change(
        pending_account_info,
        change_kind::REGISTER_RESERVE_DESTINATION,
        risk,
        &expected_reserve_destination_key,
        &old_hash,
        &new_hash,
        &chancery_config.governance_authority,
    )?;
    let change_id = pending.change_id;
    drop(pending);

    // ── Allocate the destination PDA ──────────────────────────────────────────
    if reserve_destination_account_info.data_is_empty() {
        create_pda_account(
            payer_account_info,
            reserve_destination_account_info,
            system_program_account_info,
            &program_id,
            &[
                seeds::RESERVE_DESTINATION,
                asset_mint_account_info.key.as_ref(),
                destination_token_account_info.key.as_ref(),
                &[bump],
            ],
            RESERVE_DESTINATION_SIZE,
        )?;
    } else if reserve_destination_account_info.data_len() != RESERVE_DESTINATION_SIZE {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    {
        let mut destination =
            ReserveDestination::load_uninitialized_mut(reserve_destination_account_info)?;
        destination.discriminator              = RESERVE_DESTINATION_DISCRIMINATOR;
        destination.version                    = 1;
        destination.bump                       = bump;
        destination.status                     = reserve_destination_status::DISABLED;
        destination._pad0                      = [0u8; 4];
        destination.asset_mint                 = *asset_mint_account_info.key;
        destination.destination_token_account  = *destination_token_account_info.key;
        destination.destination_owner          = args.destination_owner;
        destination.destination_flags          = args.destination_flags;
        destination.approved_by                = *governance_authority_account_info.key;
        destination.withdrawal_limit_policy_id = args.withdrawal_limit_policy_id;
        destination._reserved                  = [0u8; RESERVE_DESTINATION_RESERVED_SIZE];
    }

    // ── Finalize the pending record ───────────────────────────────────────────
    let clock = Clock::get()?;

    if accounts.len() <= RENT_REFUND_RECIPIENT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let rent_refund_recipient_account_info = &accounts[RENT_REFUND_RECIPIENT];

    consume_and_close_pending_change(pending_account_info, rent_refund_recipient_account_info)?;

    // ── Evidence (emitted last, after all state writes) ─────────────────────────
    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;

    // Total-ever destination count (doc 14 §14.6.4).
    chancery_config_mut.total_reserve_destinations_registered = chancery_config_mut
        .total_reserve_destinations_registered
        .checked_add(1)
        .ok_or(ChanceryError::ArithmeticOverflow)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    emit_reserve_destination_registered(
        event_authority_account_info,
        event_authority_bump,
        ReserveDestinationRegistered {
            sequence_nonce,
            chancery:                   *chancery_config_account_info.key,
            slot:                       clock.slot,
            unix_timestamp:             clock.unix_timestamp,
            risk_class:                 risk.as_u8(),
            change_id,
            reserve_destination:        *reserve_destination_account_info.key,
            destination_token:          *destination_token_account_info.key,
            purpose_flag:               args.destination_flags,
            registered_by:              *governance_authority_account_info.key,
            withdrawal_limit_policy_id: args.withdrawal_limit_policy_id,
        },
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reserve_destination_owner_must_be_external() {
        let program_id = crate::id();
        let (mint_authority, _) =
            Pubkey::find_program_address(&[seeds::MINT_AUTHORITY], &program_id);

        assert_eq!(
            assert_destination_owner_is_external(&mint_authority),
            Err(ChanceryError::ProtocolSignerIdentityForbidden.into()),
        );
        assert_eq!(
            assert_destination_owner_is_external(&Pubkey::default()),
            Err(ChanceryError::ProtocolSignerIdentityForbidden.into()),
        );

        // Off-curve identities controlled by other programs, such as external
        // multisig vault PDAs, remain valid reserve owners.
        let external_program = Pubkey::new_unique();
        let (external_vault, _) =
            Pubkey::find_program_address(&[b"external-vault"], &external_program);
        assert!(assert_destination_owner_is_external(&external_vault).is_ok());
    }
}
