//! Evidence policy registration event.
//!
//! Named `evidence_policy` rather than `evidence` to avoid a name collision
//! with the parent module (`crate::modules::evidence`).

use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::modules::evidence::event::{emit_event, ChanceryEvent};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct EvidencePolicyRegistered {
    pub sequence_nonce:  u64,
    pub chancery:        Pubkey,
    pub slot:            u64,
    pub unix_timestamp:  i64,

    pub risk_class:      u8,
    pub evidence_policy: Pubkey,
    pub registered_by:   Pubkey,
}
impl ChanceryEvent for EvidencePolicyRegistered {
    const NAME: &'static str = "EvidencePolicyRegistered";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xF3, 0x3F, 0x5D, 0x4C, 0xD6, 0x2D, 0x51, 0x2D]
    }
}

pub fn emit_evidence_policy_registered(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            EvidencePolicyRegistered,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct EvidencePolicyUpdated {
    pub sequence_nonce:  u64,
    pub chancery:        Pubkey,
    pub slot:            u64,
    pub unix_timestamp:  i64,

    pub risk_class:      u8,
    pub change_id:       [u8; 32],
    pub evidence_policy: Pubkey,
    pub old_value_hash:  [u8; 32],
    pub new_value_hash:  [u8; 32],
    pub proposed_by:     Pubkey,
    pub updated_by:      Pubkey,
}
impl ChanceryEvent for EvidencePolicyUpdated {
    const NAME: &'static str = "EvidencePolicyUpdated";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xDC, 0xF2, 0xA7, 0x13, 0x98, 0xBE, 0x77, 0x5B]
    }
}

pub fn emit_evidence_policy_updated(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            EvidencePolicyUpdated,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}
