//! Bootstrap & init events.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::modules::evidence::event::{emit_event, ChanceryEvent};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct ChanceryInitialized {
    pub sequence_nonce:            u64,
    pub chancery:                  Pubkey,
    pub slot:                      u64,
    pub unix_timestamp:            i64,

    pub governance_authority:      Pubkey,
    pub operations_authority:      Pubkey,
    pub emergency_authority:       Pubkey,
    pub enforcement_authority:     Pubkey,
    pub insurance_admin_authority: Pubkey,
    pub issued_token_mint:         Pubkey,
    pub issued_token_program:      Pubkey,
    pub legacy_token_mint:         Pubkey,
    pub legacy_token_program:      Pubkey,
    pub mint_authority_pda:        Pubkey,
    pub freeze_authority_pda:      Pubkey,
    pub domain_separator:          [u8; 32],
}
impl ChanceryEvent for ChanceryInitialized {
    const NAME: &'static str = "ChanceryInitialized";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x45, 0x8B, 0x44, 0xDF, 0xFB, 0x36, 0xA6, 0xE6]
    }
}

pub fn emit_chancery_initialized(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            ChanceryInitialized,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct ModuleActivationStateInitialized {
    pub sequence_nonce:   u64,
    pub chancery:         Pubkey,
    pub slot:             u64,
    pub unix_timestamp:   i64,

    pub initialized_by:   Pubkey,
    pub default_statuses: Vec<u8>,
}
impl ChanceryEvent for ModuleActivationStateInitialized {
    const NAME: &'static str = "ModuleActivationStateInitialized";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xF1, 0x37, 0xA9, 0xCF, 0x7D, 0xCC, 0x94, 0x4D]
    }
}

pub fn emit_module_activation_state_initialized(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            ModuleActivationStateInitialized,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}
