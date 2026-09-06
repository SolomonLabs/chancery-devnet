//! Fee policy events.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::modules::evidence::event::{emit_event, ChanceryEvent};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct FeePolicyRegistered {
    pub sequence_nonce: u64,
    pub chancery:       Pubkey,
    pub slot:           u64,
    pub unix_timestamp: i64,

    pub risk_class:     u8,
    pub fee_policy:     Pubkey,
    pub registered_by:  Pubkey,
}
impl ChanceryEvent for FeePolicyRegistered {
    const NAME: &'static str = "FeePolicyRegistered";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xDA, 0x61, 0x16, 0xF1, 0xD2, 0x6C, 0x8F, 0xBF]
    }
}

pub fn emit_fee_policy_registered(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            FeePolicyRegistered,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct FeePolicyUpdated {
    pub sequence_nonce: u64,
    pub chancery:       Pubkey,
    pub slot:           u64,
    pub unix_timestamp: i64,

    pub risk_class:     u8,
    pub change_id:      [u8; 32],
    pub fee_policy:     Pubkey,
    pub old_value_hash: [u8; 32],
    pub new_value_hash: [u8; 32],
    pub proposed_by:    Pubkey,
    pub updated_by:     Pubkey,
}
impl ChanceryEvent for FeePolicyUpdated {
    const NAME: &'static str = "FeePolicyUpdated";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x62, 0x59, 0x6B, 0xBD, 0xA1, 0x59, 0xE7, 0xF3]
    }
}

pub fn emit_fee_policy_updated(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            FeePolicyUpdated,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}
