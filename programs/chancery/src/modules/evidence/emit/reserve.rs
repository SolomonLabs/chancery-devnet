//! Reserve destination & withdrawal events.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::modules::evidence::event::{emit_event, ChanceryEvent};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct ReserveDestinationRegistered {
    pub sequence_nonce:             u64,
    pub chancery:                   Pubkey,
    pub slot:                       u64,
    pub unix_timestamp:             i64,

    pub risk_class:                 u8,
    pub change_id:                  [u8; 32],
    pub reserve_destination:        Pubkey,
    pub destination_token:          Pubkey,
    pub purpose_flag:               u64,
    pub registered_by:              Pubkey,
    pub withdrawal_limit_policy_id: [u8; 32],
}
impl ChanceryEvent for ReserveDestinationRegistered {
    const NAME: &'static str = "ReserveDestinationRegistered";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x51, 0x6D, 0x01, 0x53, 0x9A, 0x2C, 0x9A, 0x65]
    }
}

pub fn emit_reserve_destination_registered(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            ReserveDestinationRegistered,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct ReserveDestinationStatusChanged {
    pub sequence_nonce:      u64,
    pub chancery:            Pubkey,
    pub slot:                u64,
    pub unix_timestamp:      i64,

    pub risk_class:          u8,
    pub reserve_destination: Pubkey,
    pub old_status:          u8,
    pub new_status:          u8,
    pub approved_by:         Pubkey,
    pub change_id:           [u8; 32],
}
impl ChanceryEvent for ReserveDestinationStatusChanged {
    const NAME: &'static str = "ReserveDestinationStatusChanged";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xCA, 0xA4, 0xC0, 0x4D, 0x86, 0xAE, 0xAC, 0xB8]
    }
}

pub fn emit_reserve_destination_status_changed(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            ReserveDestinationStatusChanged,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

/// Emitted on `withdraw_reserve`.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct ReserveWithdrawal {
    pub sequence_nonce:            u64,
    pub chancery:                  Pubkey,
    pub slot:                      u64,
    pub unix_timestamp:            i64,

    pub asset_mint:                Pubkey,
    pub destination_token_account: Pubkey,
    /// Requested source debit from the reserve.
    pub amount:                    u64,
    pub initiated_by:              Pubkey,

    /// Amount actually credited to the destination token account after any
    /// Token-2022 transfer fee. This can be lower than `amount`.
    pub actual_destination_amount: u64,
}
impl ChanceryEvent for ReserveWithdrawal {
    const NAME: &'static str = "ReserveWithdrawal";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x21, 0xEA, 0x99, 0x63, 0x38, 0x13, 0x3F, 0xD6]
    }
}

pub fn emit_reserve_withdrawal(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            ReserveWithdrawal,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}
