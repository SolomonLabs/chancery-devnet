//! Pending config-change lifecycle events.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::modules::evidence::event::{emit_event, ChanceryEvent};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct ConfigChangeProposed {
    pub sequence_nonce:                  u64,
    pub chancery:                        Pubkey,
    pub slot:                            u64,
    pub unix_timestamp:                  i64,

    pub risk_class:                      u8,
    pub change_id:                       [u8; 32],
    pub change_kind:                     u16,
    pub target_account:                  Pubkey,
    pub old_value_hash:                  [u8; 32],
    pub new_value_hash:                  [u8; 32],
    pub proposed_by:                     Pubkey,
    pub proposer_nonce:                  u64,
    pub executable_after_unix_timestamp: i64,
    pub expires_at_unix_timestamp:       i64,
}
impl ChanceryEvent for ConfigChangeProposed {
    const NAME: &'static str = "ConfigChangeProposed";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x39, 0xD4, 0x60, 0x03, 0x09, 0x1D, 0xE1, 0xC6]
    }
}

pub fn emit_config_change_proposed(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            ConfigChangeProposed,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct ConfigChangeAccepted {
    pub sequence_nonce: u64,
    pub chancery:       Pubkey,
    pub slot:           u64,
    pub unix_timestamp: i64,

    pub risk_class:     u8,
    pub change_id:      [u8; 32],
    pub accepted_by:    Pubkey,
}
impl ChanceryEvent for ConfigChangeAccepted {
    const NAME: &'static str = "ConfigChangeAccepted";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x45, 0xC1, 0x1B, 0x19, 0xB4, 0x3D, 0x87, 0x13]
    }
}

pub fn emit_config_change_accepted(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            ConfigChangeAccepted,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct ConfigChangeCancelled {
    pub sequence_nonce: u64,
    pub chancery:       Pubkey,
    pub slot:           u64,
    pub unix_timestamp: i64,

    pub risk_class:     u8,
    pub change_id:      [u8; 32],
    pub cancelled_by:   Pubkey,
}
impl ChanceryEvent for ConfigChangeCancelled {
    const NAME: &'static str = "ConfigChangeCancelled";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x42, 0x30, 0x81, 0x8D, 0x68, 0x21, 0x71, 0x02]
    }
}

/// Terminal record for a pending change swept after its expiry: the account
/// is closed and rent refunded to the propose-time recipient. Permissionless
/// cleanup - `swept_by` is whoever ran the sweep, never the refund target.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct ConfigChangeExpired {
    pub sequence_nonce:            u64,
    pub chancery:                  Pubkey,
    pub slot:                      u64,
    pub unix_timestamp:            i64,

    pub risk_class:                u8,
    pub change_id:                 [u8; 32],
    pub expires_at_unix_timestamp: i64,
    pub rent_refund_recipient:     Pubkey,
}
impl ChanceryEvent for ConfigChangeExpired {
    const NAME: &'static str = "ConfigChangeExpired";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x4A, 0x34, 0x15, 0xF1, 0x01, 0xF5, 0xB2, 0xAA]
    }
}

pub fn emit_config_change_expired(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            ConfigChangeExpired,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

pub fn emit_config_change_cancelled(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            ConfigChangeCancelled,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

