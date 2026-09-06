//! Pathway policy events.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::modules::evidence::event::{emit_event, ChanceryEvent};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PathwayPolicyRegistered {
    pub sequence_nonce: u64,
    pub chancery:       Pubkey,
    pub slot:           u64,
    pub unix_timestamp: i64,

    pub risk_class:     u8,
    pub pathway_policy: Pubkey,
    pub pathway_id:     [u8; 32],
    pub registered_by:  Pubkey,
}
impl ChanceryEvent for PathwayPolicyRegistered {
    const NAME: &'static str = "PathwayPolicyRegistered";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xBC, 0x6A, 0xEC, 0x48, 0x10, 0x8A, 0xB4, 0x3B]
    }
}

pub fn emit_pathway_policy_registered(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            PathwayPolicyRegistered,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PathwayPolicyUpdated {
    pub sequence_nonce: u64,
    pub chancery:       Pubkey,
    pub slot:           u64,
    pub unix_timestamp: i64,

    pub risk_class:     u8,
    pub change_id:      [u8; 32],
    pub pathway_policy: Pubkey,
    pub old_value_hash: [u8; 32],
    pub new_value_hash: [u8; 32],
    pub proposed_by:    Pubkey,
    pub updated_by:     Pubkey,
}
impl ChanceryEvent for PathwayPolicyUpdated {
    const NAME: &'static str = "PathwayPolicyUpdated";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xD9, 0xEE, 0xCF, 0x05, 0x03, 0x10, 0x9B, 0xB9]
    }
}

pub fn emit_pathway_policy_updated(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            PathwayPolicyUpdated,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PathwayStatusChanged {
    pub sequence_nonce: u64,
    pub chancery:       Pubkey,
    pub slot:           u64,
    pub unix_timestamp: i64,

    pub risk_class:     u8,
    pub pathway_policy: Pubkey,
    pub old_status:     u8,
    pub new_status:     u8,
    pub changed_by:     Pubkey,
    
    /// Zeroed for restrictive direct path.
    pub change_id:      [u8; 32],
}
impl ChanceryEvent for PathwayStatusChanged {
    const NAME: &'static str = "PathwayStatusChanged";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x4F, 0xF0, 0x1D, 0x0B, 0x97, 0x66, 0x66, 0x08]
    }
}

pub fn emit_pathway_status_changed(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            PathwayStatusChanged,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}
