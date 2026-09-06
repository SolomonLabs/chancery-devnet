//! Legacy migration events.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::modules::evidence::event::{emit_event, ChanceryEvent};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct LegacyMigrationEnabled {
    pub sequence_nonce:         u64,
    pub chancery:               Pubkey,
    pub slot:                   u64,
    pub unix_timestamp:         i64,

    pub risk_class:             u8,
    pub migration_config:       Pubkey,
    pub legacy_program:         Pubkey,
    pub legacy_mint:            Pubkey,
    pub legacy_supply_snapshot: u64,
    pub enabled_by:             Pubkey,
}
impl ChanceryEvent for LegacyMigrationEnabled {
    const NAME: &'static str = "LegacyMigrationEnabled";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xCC, 0xA8, 0x03, 0xDD, 0x97, 0x01, 0xB8, 0xDC]
    }
}

pub fn emit_legacy_migration_enabled(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            LegacyMigrationEnabled,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

/// Emitted on `migrate_legacy_to_token2022`.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct LegacyMigration {
    pub sequence_nonce:       u64,
    pub chancery:             Pubkey,
    pub slot:                 u64,
    pub unix_timestamp:       i64,

    pub legacy_mint:          Pubkey,
    pub issued_token_mint:    Pubkey,
    pub burned_legacy_amount: u64,
    pub minted_issued_amount: u64,
    pub migrated_by:          Pubkey,
}
impl ChanceryEvent for LegacyMigration {
    const NAME: &'static str = "LegacyMigration";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x87, 0xAA, 0xC6, 0xF6, 0x55, 0x22, 0xE2, 0x8A]
    }
}

pub fn emit_legacy_migration(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            LegacyMigration,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}
