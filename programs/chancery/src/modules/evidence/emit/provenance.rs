//! Provenance case events (later module).

use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::modules::evidence::event::{emit_event, ChanceryEvent};

/// Emitted on `open_provenance_case`.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct ProvenanceCaseOpened {
    pub sequence_nonce: u64,
    pub chancery:       Pubkey,
    pub slot:           u64,
    pub unix_timestamp: i64,

    pub case_id:        [u8; 32],
    pub asset_mint:     Pubkey,
    
    /// Entity under inquiry (token account, principal, or pathway key).
    pub subject:        Pubkey,
    pub opened_by:      Pubkey,
    
    /// Categorical reason code (free-use space for governance).
    pub reason_code:    u32,
}
impl ChanceryEvent for ProvenanceCaseOpened {
    const NAME: &'static str = "ProvenanceCaseOpened";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x4F, 0xA2, 0x03, 0xCC, 0xF0, 0x1B, 0x1E, 0xFA]
    }
}

pub fn emit_provenance_case_opened(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            ProvenanceCaseOpened,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

/// Emitted on `approve_provenance_case`.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct ProvenanceCaseApproved {
    pub sequence_nonce: u64,
    pub chancery:       Pubkey,
    pub slot:           u64,
    pub unix_timestamp: i64,

    pub case_id:        [u8; 32],
    pub approved_by:    Pubkey,
    
    /// Bits indicating the disposition (e.g. cleared, escalated, frozen).
    pub outcome_flags:  u64,
}
impl ChanceryEvent for ProvenanceCaseApproved {
    const NAME: &'static str = "ProvenanceCaseApproved";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x42, 0xD2, 0x15, 0xC5, 0xD5, 0x4B, 0x28, 0x7A]
    }
}

pub fn emit_provenance_case_approved(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            ProvenanceCaseApproved,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}
