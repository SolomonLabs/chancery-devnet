//! Insurance events (later module).

use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::modules::evidence::event::{emit_event, ChanceryEvent};

/// Emitted on `open_insurance_claim_notice`.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct ClaimNoticeOpened {
    pub sequence_nonce:      u64,
    pub chancery:            Pubkey,
    pub slot:                u64,
    pub unix_timestamp:      i64,

    pub notice_id:           [u8; 32],
    pub insurance_policy_id: [u8; 32],
    pub claimant:            Pubkey,
    pub amount_claimed:      u64,
    
    /// Categorical reason code.
    pub reason_code:         u32,
}
impl ChanceryEvent for ClaimNoticeOpened {
    const NAME: &'static str = "ClaimNoticeOpened";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x9D, 0x82, 0xBF, 0x67, 0x5A, 0x14, 0x6D, 0x61]
    }
}

pub fn emit_claim_notice_opened(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            ClaimNoticeOpened,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}
