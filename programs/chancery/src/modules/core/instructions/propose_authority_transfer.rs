/// propose_authority_transfer
///
/// Opens a pending authority transfer for a single role.
/// The new authority must call `accept_authority_transfer` after the timelock.
///
/// Accounts:
///   0  chancery_config            writable  PDA [b"chancery-config"]
///   1  event_authority            readable  PDA [b"event-authority"]
///   2  authority_transfer         writable  PDA [b"authority-transfer", role_kind]
///   3  payer                      signer    funds rent
///   4  governance_authority       signer    must be chancery_config.governance_authority
///   5  system_program
///
/// Only governance_authority may propose transfers.
/// The timelock slot is set by the governance authority in args.

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    account_security::{assert_external_identity, assert_external_signer},
    constants::{seeds, MINIMUM_AUTHORITY_TRANSFER_TIMELOCK_SLOTS},
    error::ChanceryError,
    modules::core::{
        instructions::create_pda_account,
        state::{
            authority_transfer::{
                AuthorityTransfer, AUTHORITY_TRANSFER_DISCRIMINATOR,
                AUTHORITY_TRANSFER_RESERVED_SIZE, AUTHORITY_TRANSFER_SIZE,
            },
            chancery_config::ChanceryConfig,
        },
    },
};
use crate::modules::evidence::emit::{
    emit_authority_transfer_cancelled, emit_authority_transfer_proposed,
    AuthorityTransferCancelled, AuthorityTransferProposed,
};

// ─── Account indices ──────────────────────────────────────────────────────────
const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const AUTHORITY_TRANSFER:     usize = 2;
const PAYER:                  usize = 3;
const GOVERNANCE_AUTHORITY:   usize = 4;
const SYSTEM_PROGRAM:         usize = 5;
const REQUIRED_ACCOUNT_COUNT: usize = 6;

// ─── Args ─────────────────────────────────────────────────────────────────────
#[derive(BorshDeserialize)]
pub struct ProposeAuthorityTransferArgs {
    /// One of `authority_role::*` constants.
    pub role_kind:          u8,
    pub proposed_authority: Pubkey,

    /// Number of slots that must pass before the transfer can be accepted.
    /// Must be >= `MINIMUM_AUTHORITY_TRANSFER_TIMELOCK_SLOTS`.
    pub timelock_slots:     u64,
}

// ─── Handler ──────────────────────────────────────────────────────────────────
pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info      = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];
    let authority_transfer_account_info   = &accounts[AUTHORITY_TRANSFER];
    let payer_account_info                = &accounts[PAYER];
    let governance_authority_account_info = &accounts[GOVERNANCE_AUTHORITY];
    let system_program_account_info       = &accounts[SYSTEM_PROGRAM];

    if !payer_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    assert_external_signer(governance_authority_account_info, &crate::id())?;

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !authority_transfer_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    if governance_authority_account_info.key != &chancery_config.governance_authority {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    let args = ProposeAuthorityTransferArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    assert_external_identity(&args.proposed_authority, &crate::id())?;

    // ── Enforce minimum timelock ──────────────────────────────────────────────
    // Reject `timelock_slots` below the policy floor so authority rotations
    // always carry a real off-chain reaction window.
    if args.timelock_slots < MINIMUM_AUTHORITY_TRANSFER_TIMELOCK_SLOTS {
        return Err(ChanceryError::AuthorityTransferTimelockBelowMinimum.into());
    }

    // ── Resolve old authority for this role ───────────────────────────────────
    use crate::constants::authority_role;
    let old_authority = match args.role_kind {
        authority_role::GOVERNANCE      => chancery_config.governance_authority,
        authority_role::OPS             => chancery_config.operations_authority,
        authority_role::EMERGENCY       => chancery_config.emergency_authority,
        authority_role::ENFORCEMENT     => chancery_config.enforcement_authority,
        authority_role::INSURANCE_ADMIN => chancery_config.insurance_admin_authority,
        _                               => return Err(ChanceryError::UnknownInstruction.into()),
    };

    chancery_config.assert_authority_assignment_distinct(
        args.role_kind,
        &args.proposed_authority,
    )?;

    // ── Derive + verify PDA ───────────────────────────────────────────────────
    let program_id = crate::id();
    let (expected_key, bump) = Pubkey::find_program_address(
        &[seeds::AUTHORITY_TRANSFER, &[args.role_kind]],
        &program_id,
    );

    if authority_transfer_account_info.key != &expected_key {
        return Err(ChanceryError::InvalidPda.into());
    }

    let clock = Clock::get()?;
    let executable_after_slot = clock.slot
        .checked_add(args.timelock_slots)
        .ok_or(ChanceryError::ArithmeticOverflow)?;
    let expires_at_slot = executable_after_slot
        .checked_add(crate::constants::AUTHORITY_TRANSFER_ACCEPTANCE_WINDOW_SLOTS)
        .ok_or(ChanceryError::ArithmeticOverflow)?;

    // ── Classify the reusable PDA before writing ───────────────────────────────
    // The PDA has three legitimate states:
    //   1. unallocated;
    //   2. allocated and zeroed after an accepted transfer;
    //   3. a verified live proposal that governance is replacing.
    // Any other discriminator/version/PDA state is corruption and must fail
    // closed rather than being silently overwritten.
    let mut has_live_record = false;
    let cancelled_record: Option<AuthorityTransfer> = if authority_transfer_account_info.data_is_empty() {
        None
    } else {
        if authority_transfer_account_info.data_len() != AUTHORITY_TRANSFER_SIZE {
            return Err(ChanceryError::AccountDataLengthMismatch.into());
        }

        let discriminator = {
            let data = authority_transfer_account_info.try_borrow_data()?;
            let mut discriminator = [0u8; 8];
            discriminator.copy_from_slice(&data[..8]);
            discriminator
        };

        if discriminator == [0u8; 8] {
            None
        } else if discriminator == AUTHORITY_TRANSFER_DISCRIMINATOR {
            let existing    = AuthorityTransfer::load_for_verified_pda(
                authority_transfer_account_info,
                args.role_kind,
                bump,
            )?;
            has_live_record = true;

            Some(*existing)
        } else {
            return Err(ChanceryError::NotInitialized.into());
        }
    };

    // ── Create PDA if not yet allocated ──────────────────────────────────────
    if authority_transfer_account_info.data_is_empty() {
        create_pda_account(
            payer_account_info,
            authority_transfer_account_info,
            system_program_account_info,
            &program_id,
            &[seeds::AUTHORITY_TRANSFER, &[args.role_kind], &[bump]],
            AUTHORITY_TRANSFER_SIZE,
        )?;
    }

    // ── Write state ───────────────────────────────────────────────────────────
    // Re-proposing for the same role deliberately cancels the verified live
    // proposal (audit #30). A zeroed reusable PDA must still satisfy the normal
    // uninitialized-state checks.
    {
        let mut rec = if has_live_record {
            AuthorityTransfer::load_mut_for_verified_pda(
                authority_transfer_account_info,
                args.role_kind,
                bump,
            )?
        } else {
            AuthorityTransfer::load_uninitialized_mut(authority_transfer_account_info)?
        };

        rec.discriminator         = AUTHORITY_TRANSFER_DISCRIMINATOR;
        rec.version               = 1;
        rec.bump                  = bump;
        rec.role_kind             = args.role_kind;
        rec._pad0                 = [0u8; 4];
        rec.old_authority         = old_authority;
        rec.proposed_authority    = args.proposed_authority;
        rec.proposed_at_slot      = clock.slot;
        rec.executable_after_slot = executable_after_slot;
        rec.proposing_governance  = chancery_config.governance_authority;
        rec.expires_at_slot       = expires_at_slot;
        rec._reserved             = [0u8; AUTHORITY_TRANSFER_RESERVED_SIZE];
    }

    // ── Emit evidence ─────────────────────────────────────────────────────────
    // chancery_config is writable - increment sequence_nonce.
    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;

    // Emit cancellation first so the (Cancelled, Proposed) pair is ordered
    // and replayable off-chain.
    if let Some(cancelled) = cancelled_record {
        let cancel_seq = chancery_config_mut.next_sequence_nonce()?;
        emit_authority_transfer_cancelled(
            event_authority_account_info,
            event_authority_bump,
            AuthorityTransferCancelled {
                sequence_nonce:      cancel_seq,
                chancery:            *chancery_config_account_info.key,
                slot:                clock.slot,
                unix_timestamp:      clock.unix_timestamp,
                role_kind:           cancelled.role_kind,
                cancelled_authority: cancelled.proposed_authority,
                proposed_at_slot:    cancelled.proposed_at_slot,
            },
        )?;
    }

    let seq = chancery_config_mut.next_sequence_nonce()?;

    emit_authority_transfer_proposed(
        event_authority_account_info,
        event_authority_bump,
        AuthorityTransferProposed {
            sequence_nonce:        seq,
            chancery:              *chancery_config_account_info.key,
            slot:                  clock.slot,
            unix_timestamp:        clock.unix_timestamp,
            role_kind:             args.role_kind,
            old_authority:         old_authority,
            new_authority:         args.proposed_authority,
            executable_after_slot: executable_after_slot,
        },
    )?;

    Ok(())
}
