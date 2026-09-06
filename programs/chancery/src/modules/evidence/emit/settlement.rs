//! Settlement events: intent creation + mint/redeem completions.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::modules::evidence::event::{emit_event, ChanceryEvent};



#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct SettlementPolicyRegistered {
    pub sequence_nonce:    u64,
    pub chancery:          Pubkey,
    pub slot:              u64,
    pub unix_timestamp:    i64,

    pub risk_class:        u8,
    pub settlement_policy: Pubkey,
    pub policy_id:         [u8; 32],
    pub registered_by:     Pubkey,
}
impl ChanceryEvent for SettlementPolicyRegistered {
    const NAME: &'static str = "SettlementPolicyRegistered";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x4D, 0x3F, 0x8E, 0x89, 0xA2, 0x66, 0xFD, 0xFA]
    }
}

pub fn emit_settlement_policy_registered(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            SettlementPolicyRegistered,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct SettlementIntentCreated {
    pub sequence_nonce:              u64,
    pub chancery:                    Pubkey,
    pub slot:                        u64,
    pub unix_timestamp:              i64,

    pub risk_class:                  u8,
    pub settlement_intent:           Pubkey,
    pub intent_id:                   [u8; 32],
    pub pathway_id:                  [u8; 32],
    pub pathway_policy:              Pubkey,
    pub settlement_mode:             u8,
    pub settlement_action:           u8,
    pub principal_a:                 Pubkey,
    pub principal_b:                 Pubkey,
    pub executor:                    Pubkey,
    pub asset_mint:                  Pubkey,
    pub issued_token_mint:           Pubkey,
    pub asset_amount:                u64,
    pub issued_token_amount:         u64,
    pub minimum_asset_amount:        u64,
    pub minimum_issued_token_amount: u64,
    pub nonce:                       u64,
    pub valid_after_unix_timestamp:  i64,
    pub expires_at_unix_timestamp:   i64,
    pub policy_id:                   [u8; 32],
    pub intent_hash:                 [u8; 32],
    pub created_by:                  Pubkey,
}
impl ChanceryEvent for SettlementIntentCreated {
    const NAME: &'static str = "SettlementIntentCreated";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x86, 0xFC, 0x5F, 0x48, 0xAA, 0x1A, 0xB2, 0xD2]
    }
}

/// Terminal record for an intent swept after expiry: the account is closed
/// and rent refunded to the creation-time recipient. Permissionless cleanup -
/// the caller cannot redirect the refund, only trigger it. Executed intents
/// close in the settlement handler and are recorded by the settlement event
/// itself; this event exists only for intents that die of old age.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct SettlementIntentExpired {
    pub sequence_nonce:              u64,
    pub chancery:                    Pubkey,
    pub slot:                        u64,
    pub unix_timestamp:              i64,

    pub settlement_intent:           Pubkey,
    pub intent_id:                   [u8; 32],
    pub pathway_id:                  [u8; 32],
    pub settlement_mode:             u8,
    pub settlement_action:           u8,
    pub principal_a:                 Pubkey,
    pub principal_b:                 Pubkey,
    pub executor:                    Pubkey,
    pub asset_mint:                  Pubkey,
    pub issued_token_mint:           Pubkey,
    pub asset_amount:                u64,
    pub issued_token_amount:         u64,
    pub minimum_asset_amount:        u64,
    pub minimum_issued_token_amount: u64,
    pub nonce:                       u64,
    pub valid_after_unix_timestamp:  i64,
    pub expires_at_unix_timestamp:   i64,
    pub policy_id:                   [u8; 32],
    pub intent_hash:                 [u8; 32],
    pub rent_refund_recipient:       Pubkey,
}
impl ChanceryEvent for SettlementIntentExpired {
    const NAME: &'static str = "SettlementIntentExpired";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x07, 0x86, 0x22, 0xB2, 0xE3, 0x21, 0xD0, 0xC8]
    }
}

pub fn emit_settlement_intent_expired(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            SettlementIntentExpired,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

pub fn emit_settlement_intent_created(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            SettlementIntentCreated,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

/// Principal-authorized terminal cancellation. As with execution and expiry,
/// the intent account is closed and the durable record is evidence rather
/// than a persistent tombstone.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct SettlementIntentCancelled {
    pub sequence_nonce:              u64,
    pub chancery:                    Pubkey,
    pub slot:                        u64,
    pub unix_timestamp:              i64,

    pub settlement_intent:           Pubkey,
    pub intent_id:                   [u8; 32],
    pub pathway_id:                  [u8; 32],
    pub settlement_mode:             u8,
    pub settlement_action:           u8,
    pub principal_a:                 Pubkey,
    pub principal_b:                 Pubkey,
    pub executor:                    Pubkey,
    pub asset_mint:                  Pubkey,
    pub issued_token_mint:           Pubkey,
    pub asset_amount:                u64,
    pub issued_token_amount:         u64,
    pub minimum_asset_amount:        u64,
    pub minimum_issued_token_amount: u64,
    pub nonce:                       u64,
    pub valid_after_unix_timestamp:  i64,
    pub expires_at_unix_timestamp:   i64,
    pub policy_id:                   [u8; 32],
    pub intent_hash:                 [u8; 32],
    pub cancelled_by:                Pubkey,
    pub rent_refund_recipient:       Pubkey,
}
impl ChanceryEvent for SettlementIntentCancelled {
    const NAME: &'static str = "SettlementIntentCancelled";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x53, 0xAE, 0x8A, 0xE4, 0xA8, 0xAF, 0x65, 0x9E]
    }
}

pub fn emit_settlement_intent_cancelled(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            SettlementIntentCancelled,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

/// Emitted on every successful mint settlement (direct, delegated, trilateral).
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct SettlementMint {
    pub sequence_nonce:              u64,
    pub chancery:                    Pubkey,
    pub slot:                        u64,
    pub unix_timestamp:              i64,

    pub pathway_id:                  [u8; 32],
    
    /// One of `settlement_mode::*`.
    pub settlement_mode:             u8,
    pub intent_id:                   [u8; 32],
    pub settlement_policy_id:        [u8; 32],
    pub limit_policy_id:             [u8; 32],
    pub evidence_policy_id:          [u8; 32],
    pub fee_policy_id:               [u8; 32],
    
    /// Zero if insurance module inactive.
    pub insurance_policy_id:         [u8; 32],
    pub principal_a:                 Pubkey,
    pub principal_b:                 Pubkey,
    
    /// Zero for direct settlement.
    pub executor:                    Pubkey,
    pub asset_mint:                  Pubkey,
    pub issued_token_mint:           Pubkey,
    pub asset_token_program:         Pubkey,
    pub issued_token_program:        Pubkey,
    pub source_account:              Pubkey,
    pub destination_account:         Pubkey,

    /// Authenticated owner and concrete token account for the external fee
    /// leg. Both are zero when no fee was routed externally.
    pub fee_recipient_owner:         Pubkey,
    pub fee_recipient_token_account: Pubkey,
    
    /// Zero if compartment module inactive.
    pub reserve_compartment_id:      [u8; 32],
    pub gross_amount_in:             u64,
    pub gross_amount_out:            u64,

    /// Pre-rebate fee assessed by the policy. The amount routed to an external
    /// recipient is `fee_output_amount - rebate_amount`; retention policies
    /// route zero.
    pub fee_output_amount:           u64,
    pub rebate_amount:               u64,
    pub net_output_amount:           u64,
    pub status_flags:                u64,
    pub reason_code:                 u32,

    /// Collateral actually credited to the reserve after Token-2022 transfer
    /// fees or other token-program deductions. Issuance arithmetic uses this
    /// measured amount; `gross_amount_in` remains the requested transfer.
    pub actual_asset_amount_in:      u64,
}
impl ChanceryEvent for SettlementMint {
    const NAME: &'static str = "SettlementMint";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xF7, 0xAE, 0xB5, 0x0A, 0xF6, 0xF7, 0x89, 0x2D]
    }
}

pub fn emit_settlement_mint(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            SettlementMint,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

/// Emitted on every successful redeem settlement (direct, delegated, trilateral).
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct SettlementRedeem {
    pub sequence_nonce:              u64,
    pub chancery:                    Pubkey,
    pub slot:                        u64,
    pub unix_timestamp:              i64,

    pub pathway_id:                  [u8; 32],
    pub settlement_mode:             u8,
    pub intent_id:                   [u8; 32],
    pub settlement_policy_id:        [u8; 32],
    pub limit_policy_id:             [u8; 32],
    pub evidence_policy_id:          [u8; 32],
    pub fee_policy_id:               [u8; 32],
    pub insurance_policy_id:         [u8; 32],
    pub principal_a:                 Pubkey,
    pub principal_b:                 Pubkey,
    pub executor:                    Pubkey,
    pub asset_mint:                  Pubkey,
    pub issued_token_mint:           Pubkey,
    pub asset_token_program:         Pubkey,
    pub issued_token_program:        Pubkey,
    pub source_account:              Pubkey,
    pub destination_account:         Pubkey,

    /// Authenticated owner and concrete token account for the external fee
    /// leg. Both are zero when no fee was routed externally.
    pub fee_recipient_owner:         Pubkey,
    pub fee_recipient_token_account: Pubkey,
    pub reserve_compartment_id:      [u8; 32],
    pub gross_amount_in:             u64,
    pub gross_amount_out:            u64,

    /// Pre-rebate fee assessed by the policy. The amount routed to an external
    /// recipient is `fee_output_amount - rebate_amount`; retention policies
    /// route zero.
    pub fee_output_amount:           u64,
    pub rebate_amount:               u64,
    pub net_output_amount:           u64,
    pub status_flags:                u64,
    pub reason_code:                 u32,

    /// Amount actually credited to the external fee-recipient token account.
    /// Zero when the policy retains fees or no fee is routed externally.
    pub actual_fee_recipient_amount: u64,
}
impl ChanceryEvent for SettlementRedeem {
    const NAME: &'static str = "SettlementRedeem";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x92, 0x58, 0x03, 0xDC, 0xC7, 0x1E, 0x6D, 0x10]
    }
}

pub fn emit_settlement_redeem(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            SettlementRedeem,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}
