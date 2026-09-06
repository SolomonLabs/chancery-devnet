//! Module activation status events.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::modules::evidence::event::{emit_event, ChanceryEvent};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct ModuleStatusChanged {
    pub sequence_nonce: u64,
    pub chancery:       Pubkey,
    pub slot:           u64,
    pub unix_timestamp: i64,

    pub risk_class:     u8,
    pub module_id:      u8,
    pub old_status:     u8,
    pub new_status:     u8,
    pub changed_by:     Pubkey,
    pub change_id:      [u8; 32],
}
impl ChanceryEvent for ModuleStatusChanged {
    const NAME: &'static str = "ModuleStatusChanged";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x60, 0x7C, 0x4D, 0xBC, 0x76, 0xC0, 0xD3, 0xBF]
    }
}

pub fn emit_module_status_changed(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            ModuleStatusChanged,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}
