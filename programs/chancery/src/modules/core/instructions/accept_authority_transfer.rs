/// accept_authority_transfer
///
/// The proposed new authority signs to accept and execute the transfer.
/// Updates chancery_config in place and zeroes the authority_transfer PDA.
///
/// Accounts:
///   0  chancery_config     writable  PDA
///   1  event_authority     readable  PDA [b"event-authority"]
///   2  authority_transfer  writable  PDA [b"authority-transfer", role_kind]
///   3  new_authority       signer    must match authority_transfer.proposed_authority

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_sysvar::Sysvar;

use crate::{
    account_security::assert_external_signer,
    constants::authority_role,
    error::ChanceryError,
    modules::core::state::{
        authority_transfer::AuthorityTransfer,
        chancery_config::ChanceryConfig,
    },
};
use crate::modules::evidence::emit::{emit_authority_transfer_accepted, AuthorityTransferAccepted};

// ─── Account indices ──────────────────────────────────────────────────────────
const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const AUTHORITY_TRANSFER:     usize = 2;
const NEW_AUTHORITY:          usize = 3;
const REQUIRED_ACCOUNT_COUNT: usize = 4;

// ─── Args ─────────────────────────────────────────────────────────────────────
#[derive(BorshDeserialize)]
pub struct AcceptAuthorityTransferArgs {
    pub role_kind: u8,
}

// ─── Handler ──────────────────────────────────────────────────────────────────
pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info    = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info    = &accounts[EVENT_AUTHORITY];
    let authority_transfer_account_info = &accounts[AUTHORITY_TRANSFER];
    let new_authority_account_info      = &accounts[NEW_AUTHORITY];

    assert_external_signer(new_authority_account_info, &crate::id())?;

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !authority_transfer_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let args = AcceptAuthorityTransferArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let clock = Clock::get()?;

    // ── Load + validate the pending transfer ──────────────────────────────────
    let authority_transfer = AuthorityTransfer::load_verified(authority_transfer_account_info)?;

    if authority_transfer.role_kind != args.role_kind {
        return Err(ChanceryError::AuthorityTransferNotPending.into());
    }

    if new_authority_account_info.key != &authority_transfer.proposed_authority {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    authority_transfer.assert_timelock_elapsed(clock.slot)?;

    // ── Acceptance-window expiry ──────────────────────────────────────────────
    // A proposal that outlives its window is void: no stale rotation may
    // linger as a latent, indefinitely-executable authority change.
    if clock.slot > authority_transfer.expires_at_slot {
        return Err(ChanceryError::AuthorityTransferExpired.into());
    }

    let old_authority        = authority_transfer.old_authority;
    let new_authority        = authority_transfer.proposed_authority;
    let role_kind            = authority_transfer.role_kind;
    let proposing_governance = authority_transfer.proposing_governance;

    // Release the immutable account-data guard before zeroing the transfer
    // record below. The verified loader intentionally retains RefCell borrow
    // tracking for the lifetime of `authority_transfer`.
    drop(authority_transfer);

    // ── Apply to chancery_config ─────────────────────────────────────────────
    let mut chancery_config = ChanceryConfig::load_verified_mut(chancery_config_account_info)?;

    // ── Proposer legitimacy ───────────────────────────────────────────────────
    // The proposal's authorization derives from the governance authority that
    // created it. If governance has rotated since, every outstanding proposal
    // from the previous governance is automatically void.
    if proposing_governance != chancery_config.governance_authority {
        return Err(ChanceryError::AuthorityTransferProposerSuperseded.into());
    }

    // ── Baseline staleness ────────────────────────────────────────────────────
    // The role's current holder must still be the one recorded at proposal
    // time; a proposal whose baseline drifted must be re-proposed.
    let current_holder = match role_kind {
        authority_role::GOVERNANCE      => chancery_config.governance_authority,
        authority_role::OPS             => chancery_config.operations_authority,
        authority_role::EMERGENCY       => chancery_config.emergency_authority,
        authority_role::ENFORCEMENT     => chancery_config.enforcement_authority,
        authority_role::INSURANCE_ADMIN => chancery_config.insurance_admin_authority,
        _                               => return Err(ChanceryError::UnknownInstruction.into()),
    };
    if current_holder != old_authority {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    chancery_config.assert_authority_assignment_distinct(role_kind, &new_authority)?;

    match role_kind {
        authority_role::GOVERNANCE      => chancery_config.governance_authority      = new_authority,
        authority_role::OPS             => chancery_config.operations_authority      = new_authority,
        authority_role::EMERGENCY       => chancery_config.emergency_authority       = new_authority,
        authority_role::ENFORCEMENT     => chancery_config.enforcement_authority     = new_authority,
        authority_role::INSURANCE_ADMIN => chancery_config.insurance_admin_authority = new_authority,
        _                               => return Err(ChanceryError::UnknownInstruction.into()),
    }

    // ── Zero the transfer record ──────────────────────────────────────────────
    // Overwrite with zeros in place - account stays allocated for reuse.
    {
        let mut data = authority_transfer_account_info.try_borrow_mut_data()?;
        data.fill(0);
    }

    // ── Emit evidence ─────────────────────────────────────────────────────────
    let event_authority_bump = chancery_config.event_authority_bump;
    let seq                  = chancery_config.next_sequence_nonce()?;

    emit_authority_transfer_accepted(
        event_authority_account_info,
        event_authority_bump,
        AuthorityTransferAccepted {
            sequence_nonce: seq,
            chancery:       *chancery_config_account_info.key,
            slot:           clock.slot,
            unix_timestamp: clock.unix_timestamp,
            role_kind:      role_kind,
            old_authority:  old_authority,
            new_authority:  new_authority,
        },
    )?;

    Ok(())
}
