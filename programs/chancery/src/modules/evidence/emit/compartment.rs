//! Compartment events (later module).

use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::modules::evidence::event::{emit_event, ChanceryEvent};

/// Emitted on `promote_compartment_balance`.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct CompartmentPromoted {
    pub sequence_nonce: u64,
    pub chancery:       Pubkey,
    pub slot:           u64,
    pub unix_timestamp: i64,

    pub compartment_id: [u8; 32],
    pub asset_mint:     Pubkey,
    pub amount:         u64,
    pub promoted_by:    Pubkey,
    
    /// Bits describing the compartment kind transition.
    pub kind_flags:     u64,
}
impl ChanceryEvent for CompartmentPromoted {
    const NAME: &'static str = "CompartmentPromoted";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xFB, 0x2B, 0x58, 0x23, 0x1E, 0x2C, 0xF7, 0x85]
    }
}

pub fn emit_compartment_promoted(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            CompartmentPromoted,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}
