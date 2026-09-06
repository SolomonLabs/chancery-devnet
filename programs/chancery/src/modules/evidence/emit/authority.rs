//! Authority transfer events.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::modules::evidence::event::{emit_event, ChanceryEvent};

/// Emitted on `propose_authority_transfer`.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct AuthorityTransferProposed {
    pub sequence_nonce:        u64,
    pub chancery:              Pubkey,
    pub slot:                  u64,
    pub unix_timestamp:        i64,

    /// One of `authority_role::*` constants.
    pub role_kind:             u8,
    pub old_authority:         Pubkey,
    pub new_authority:         Pubkey,
    pub executable_after_slot: u64,
}
impl ChanceryEvent for AuthorityTransferProposed {
    const NAME: &'static str = "AuthorityTransferProposed";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x67, 0xF4, 0x1B, 0x74, 0xB1, 0x04, 0x64, 0x77]
    }
}

pub fn emit_authority_transfer_proposed(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            AuthorityTransferProposed,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

/// Emitted on `accept_authority_transfer`.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct AuthorityTransferAccepted {
    pub sequence_nonce: u64,
    pub chancery:       Pubkey,
    pub slot:           u64,
    pub unix_timestamp: i64,

    pub role_kind:      u8,
    pub old_authority:  Pubkey,
    pub new_authority:  Pubkey,
}
impl ChanceryEvent for AuthorityTransferAccepted {
    const NAME: &'static str = "AuthorityTransferAccepted";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x95, 0xA5, 0x8C, 0xDD, 0x68, 0xCB, 0xEF, 0x79]
    }
}

pub fn emit_authority_transfer_accepted(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            AuthorityTransferAccepted,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

/// Emitted on `propose_authority_transfer` when an existing live proposal for
/// the same role is overwritten in place (governance supersession). Carries
/// just enough to identify the cancelled proposal; the rest is recoverable by
/// joining on the prior `AuthorityTransferProposed` event.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct AuthorityTransferCancelled {
    pub sequence_nonce:      u64,
    pub chancery:            Pubkey,
    pub slot:                u64,
    pub unix_timestamp:      i64,

    pub role_kind:           u8,
    pub cancelled_authority: Pubkey,
    pub proposed_at_slot:    u64,
}
impl ChanceryEvent for AuthorityTransferCancelled {
    const NAME: &'static str = "AuthorityTransferCancelled";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x1F, 0xE4, 0xBB, 0x94, 0x14, 0x63, 0xED, 0x30]
    }
}

pub fn emit_authority_transfer_cancelled(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            AuthorityTransferCancelled,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}
