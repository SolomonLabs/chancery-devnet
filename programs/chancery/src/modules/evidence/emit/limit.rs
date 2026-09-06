//! Limit policy events + breach detection.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::modules::{
    core::state::chancery_config::ChanceryConfig,
    evidence::event::{emit_event, ChanceryEvent},
    limits::state::usage_window::ClosedPeriod,
};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct LimitPolicyRegistered {
    pub sequence_nonce: u64,
    pub chancery:       Pubkey,
    pub slot:           u64,
    pub unix_timestamp: i64,

    pub risk_class:     u8,
    pub limit_policy:   Pubkey,
    pub registered_by:  Pubkey,
}
impl ChanceryEvent for LimitPolicyRegistered {
    const NAME: &'static str = "LimitPolicyRegistered";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xEF, 0xE5, 0xC1, 0x71, 0xD1, 0xEA, 0x07, 0xF5]
    }
}

pub fn emit_limit_policy_registered(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            LimitPolicyRegistered,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}



#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct UsageWindowInitialized {
    pub sequence_nonce:               u64,
    pub chancery:                     Pubkey,
    pub slot:                         u64,
    pub unix_timestamp:               i64,

    pub risk_class:                   u8,
    pub usage_window:                 Pubkey,
    pub scope_hash:                   [u8; 32],
    pub window_kind:                  u8,
    pub window_start_unix_timestamp:  i64,
    pub rent_refund_recipient:        Pubkey,
    pub created_by:                   Pubkey,
}
impl ChanceryEvent for UsageWindowInitialized {
    const NAME: &'static str = "UsageWindowInitialized";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xFD, 0x75, 0xB0, 0x0C, 0xD4, 0x2C, 0x1B, 0xF8]
    }
}

pub fn emit_usage_window_initialized(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            UsageWindowInitialized,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

/// Final totals of a period closed by an in-place `roll_forward`. Program
/// state overwrites the closed period, so this event is the canonical
/// historical record of its accumulators.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct UsageWindowRolled {
    pub sequence_nonce:                       u64,
    pub chancery:                             Pubkey,
    pub slot:                                 u64,
    pub unix_timestamp:                       i64,

    pub usage_window:                         Pubkey,
    pub scope_hash:                           [u8; 32],
    pub window_kind:                          u8,
    pub previous_window_start_unix_timestamp: i64,
    pub new_window_start_unix_timestamp:      i64,
    pub final_gross_in:                       u128,
    pub final_gross_output_amount:            u128,
    pub final_net_flow:                       i128,
    pub final_action_count:                   u32,
}
impl ChanceryEvent for UsageWindowRolled {
    const NAME: &'static str = "UsageWindowRolled";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xAF, 0x97, 0x81, 0xC9, 0x03, 0xB8, 0xC8, 0x13]
    }
}

/// Emit one `UsageWindowRolled` per period closed during the check phase.
/// Called first in the handler's evidence section (evidence-last invariant:
/// after all state mutation and token CPIs, before or alongside the handler's
/// primary event) so closed-period finals reach the record even though the
/// account was overwritten in place.
pub fn emit_usage_windows_rolled(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    chancery_config:              &mut ChanceryConfig,
    chancery:                     &Pubkey,
    clock:                        &Clock,
    rolled:                       &[ClosedPeriod],
) -> ProgramResult {
    for closed in rolled {
        let sequence_nonce = chancery_config.next_sequence_nonce()?;

        emit_event(
            event_authority_account_info,
            event_authority_bump,
            &UsageWindowRolled {
                sequence_nonce,
                chancery:                             *chancery,
                slot:                                 clock.slot,
                unix_timestamp:                       clock.unix_timestamp,
                usage_window:                         closed.window,
                scope_hash:                           closed.scope_hash,
                window_kind:                          closed.window_kind,
                previous_window_start_unix_timestamp: closed.previous_window_start_unix_timestamp,
                new_window_start_unix_timestamp:      closed.new_window_start_unix_timestamp,
                final_gross_in:                       closed.final_gross_in,
                final_gross_output_amount:            closed.final_gross_output_amount,
                final_net_flow:                       closed.final_net_flow,
                final_action_count:                   closed.final_action_count,
            },
        )?;
    }

    Ok(())
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct LimitPolicyUpdated {
    pub sequence_nonce: u64,
    pub chancery:       Pubkey,
    pub slot:           u64,
    pub unix_timestamp: i64,

    pub risk_class:     u8,
    pub change_id:      [u8; 32],
    pub limit_policy:   Pubkey,
    pub old_value_hash: [u8; 32],
    pub new_value_hash: [u8; 32],
    pub proposed_by:    Pubkey,
    pub updated_by:     Pubkey,
}
impl ChanceryEvent for LimitPolicyUpdated {
    const NAME: &'static str = "LimitPolicyUpdated";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x8A, 0xEA, 0xDD, 0xC9, 0x8F, 0xF5, 0xF2, 0x04]
    }
}

pub fn emit_limit_policy_updated(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            LimitPolicyUpdated,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

// NOTE (issue RB-05): the former `LimitBreach` event and `emit_limit_breach`
// helper were removed. They were dead code - no settlement handler could emit
// breach evidence inside a reverting transaction (Solana atomicity rolls the
// event back with the failed instruction). Breach behavior is revert-only in
// the MVP; a future keeper-finalized breach design will define its own event
// in a separate, non-reverting transaction.
