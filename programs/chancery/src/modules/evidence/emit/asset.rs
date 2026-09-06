//! Asset registration & lifecycle events.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::modules::evidence::event::{emit_event, ChanceryEvent};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct AssetRegistered {
    pub sequence_nonce:       u64,
    pub chancery:             Pubkey,
    pub slot:                 u64,
    pub unix_timestamp:       i64,

    pub risk_class:           u8,
    pub asset_config:         Pubkey,
    pub asset_mint:           Pubkey,
    pub asset_token_program:  Pubkey,
    pub mode:                 u8,
    pub is_issued_token_mint: bool,
    pub registered_by:        Pubkey,
}
impl ChanceryEvent for AssetRegistered {
    const NAME: &'static str = "AssetRegistered";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xFC, 0x38, 0x2F, 0x59, 0x08, 0xB4, 0xF9, 0xFE]
    }
}

pub fn emit_asset_registered(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            AssetRegistered,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct AssetConfigUpdated {
    pub sequence_nonce: u64,
    pub chancery:       Pubkey,
    pub slot:           u64,
    pub unix_timestamp: i64,

    pub risk_class:     u8,
    pub change_id:      [u8; 32],
    pub asset_config:   Pubkey,
    pub old_value_hash: [u8; 32],
    pub new_value_hash: [u8; 32],
    pub proposed_by:    Pubkey,
    pub updated_by:     Pubkey,
}
impl ChanceryEvent for AssetConfigUpdated {
    const NAME: &'static str = "AssetConfigUpdated";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x0C, 0x61, 0x40, 0x5E, 0x7B, 0xB1, 0x6F, 0x0A]
    }
}

pub fn emit_asset_config_updated(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            AssetConfigUpdated,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct AssetModeChanged {
    pub sequence_nonce: u64,
    pub chancery:       Pubkey,
    pub slot:           u64,
    pub unix_timestamp: i64,

    pub risk_class:     u8,
    pub change_id:      [u8; 32],
    pub asset_config:   Pubkey,
    pub old_mode:       u8,
    pub new_mode:       u8,
    pub proposed_by:    Pubkey,
    pub updated_by:     Pubkey,
}
impl ChanceryEvent for AssetModeChanged {
    const NAME: &'static str = "AssetModeChanged";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x41, 0xF6, 0xA0, 0x23, 0x96, 0xA5, 0x74, 0xFC]
    }
}

pub fn emit_asset_mode_changed(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            AssetModeChanged,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct AssetExtensionRefreshed {
    pub sequence_nonce:              u64,
    pub chancery:                    Pubkey,
    pub slot:                        u64,
    pub unix_timestamp:              i64,

    pub risk_class:                  u8,
    pub asset_config:                Pubkey,
    pub asset_mint:                  Pubkey,
    pub old_observed_extension_mask: [u64; 2],
    pub new_observed_extension_mask: [u64; 2],
    pub refreshed_by:                Pubkey,
}
impl ChanceryEvent for AssetExtensionRefreshed {
    const NAME: &'static str = "AssetExtensionRefreshed";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x78, 0x44, 0xDD, 0x58, 0x58, 0x11, 0xC0, 0xF4]
    }
}

pub fn emit_asset_extension_refreshed(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            AssetExtensionRefreshed,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}
