//! set_module_status_with_pending_change.
//!
//! Widening+ path. Consumes an accepted `PendingConfigChange` of kind
//! `change_kind::SET_MODULE_STATUS` and applies the transition.
//!
//! Wire format:
//!   [ CONTROL(0x09) | SET_MODULE_STATUS_WITH_PENDING_CHANGE(0x0C) | borsh(args) ]
//!
//! Accounts:
//!   0  module_activation_state  writable PDA
//!   1  chancery_config          writable PDA (sequence_nonce updated for evidence)
//!   2  event_authority          readable PDA [b"event-authority"]
//!   3  pending_config_change    writable PDA
//!   4  governance_authority     signer

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_sysvar::Sysvar;

use crate::{
    constants::{change_kind, module_status},
    error::ChanceryError,
    modules::{
        core::state::chancery_config::ChanceryConfig,
        control::{
            module_activation_classifier::classify_module_status_transition,
            pending_change::{
                assert_accepted_pending_change, compute_config_change_hash, consume_and_close_pending_change,
            },
            state::module_activation_state::ModuleActivationState,
        },
        evidence::emit::{emit_module_status_changed, ModuleStatusChanged},
    },
};

const ACTIVATION:             usize = 0;
const CHANCERY_CONFIG:        usize = 1;
const EVENT_AUTHORITY:        usize = 2;
const PENDING:                usize = 3;
const GOVERNANCE_AUTHORITY:   usize = 4;
const REQUIRED_ACCOUNT_COUNT: usize = 5;

// Refund conveyance: the account whose key equals the record's stored
// rent_refund_recipient. Required whenever this instruction closes the
// record - the refund is conveyed, never redirected.
const RENT_REFUND_RECIPIENT:  usize = 5;

#[derive(BorshDeserialize)]
pub struct SetModuleStatusWithPendingChangeArgs {
    pub module_id:  u8,
    pub new_status: u8,
}

/// The irreversible deprecation review must start after the reversible disable
/// transition. Strict ordering also blocks a same-transaction disable followed
/// by consumption of a preloaded deprecation approval.
pub(crate) fn assert_deprecation_review_after_disable(
    new_status:          u8,
    proposed_at_slot:    u64,
    last_updated_at_slot: u64,
) -> Result<(), ChanceryError> {
    if new_status == module_status::DEPRECATED && proposed_at_slot <= last_updated_at_slot {
        return Err(ChanceryError::ModuleDeprecationReviewPrecedesDisable);
    }

    Ok(())
}

/// Bind a module-status approval to the activation-state generation observed
/// when it was proposed. A real status transition recorded in a later slot
/// changes `last_updated_at_slot`, invalidating older accepted approvals even
/// if the status byte later cycles back to the same value.
pub(crate) fn module_status_change_payload(
    module_id:             u8,
    old_status:            u8,
    new_status:            u8,
    last_updated_at_slot:  u64,
) -> [u8; 11] {
    let mut payload = [0u8; 11];
    payload[0] = module_id;
    payload[1] = old_status;
    payload[2] = new_status;
    payload[3..].copy_from_slice(&last_updated_at_slot.to_le_bytes());
    payload
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let activation_account_info           = &accounts[ACTIVATION];
    let cfg_account_info                  = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];
    let pending_account_info              = &accounts[PENDING];
    let governance_authority_account_info = &accounts[GOVERNANCE_AUTHORITY];

    if !governance_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !activation_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !cfg_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !pending_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(cfg_account_info)?;

    if governance_authority_account_info.key != &chancery_config.governance_authority {
        return Err(ChanceryError::ConfigChangeWideningRequiresGovernance.into());
    }

    let args = SetModuleStatusWithPendingChangeArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    if args.new_status > module_status::DEPRECATED {
        return Err(ChanceryError::InvalidModuleStatus.into());
    }

    ModuleActivationState::assert_known_module(args.module_id)?;
    ModuleActivationState::assert_disableable(args.module_id)?;

    let activation_bump =
        ModuleActivationState::verify_pda(activation_account_info, &crate::id())?;
    let s = ModuleActivationState::load_for_verified_pda(
        activation_account_info,
        activation_bump,
    )?;
    let old_status = s.status_for(args.module_id);

    // Deprecation is one-way and is supported only for explicitly optional
    // modules. Current infrastructure and MVP value-flow modules are never
    // eligible for this terminal state.
    if args.new_status == module_status::DEPRECATED {
        ModuleActivationState::assert_deprecatable(args.module_id)?;
        if old_status != module_status::DISABLED {
            return Err(ChanceryError::InvalidModuleStatus.into());
        }
    }

    if old_status == module_status::DEPRECATED {
        return Err(ChanceryError::ModuleDeprecated.into());
    }

    let risk = classify_module_status_transition(args.module_id, old_status, args.new_status);

    if !risk.requires_timelock() {
        return Err(ChanceryError::ConfigChangeNotAccepted.into());
    }

    let old_payload = module_status_change_payload(
        args.module_id,
        old_status,
        old_status,
        s.last_updated_at_slot,
    );
    let new_payload = module_status_change_payload(
        args.module_id,
        old_status,
        args.new_status,
        s.last_updated_at_slot,
    );

    let old_hash = compute_config_change_hash(
        change_kind::SET_MODULE_STATUS,
        activation_account_info.key,
        risk.as_u8(),
        &old_payload,
    );

    let new_hash = compute_config_change_hash(
        change_kind::SET_MODULE_STATUS,
        activation_account_info.key,
        risk.as_u8(),
        &new_payload,
    );

    let pending = assert_accepted_pending_change(
        pending_account_info,
        change_kind::SET_MODULE_STATUS,
        risk,
        activation_account_info.key,
        &old_hash,
        &new_hash,
        &chancery_config.governance_authority,
    )?;

    assert_deprecation_review_after_disable(
        args.new_status,
        pending.proposed_at_slot,
        s.last_updated_at_slot,
    )?;

    let change_id = pending.change_id;

    let clock = Clock::get()?;
    drop(s);

    let mut s_mut = ModuleActivationState::load_mut_for_verified_pda(
        activation_account_info,
        activation_bump,
    )?;

    s_mut.module_statuses[args.module_id as usize] = args.new_status;
    s_mut.last_updated_by      = *governance_authority_account_info.key;
    s_mut.last_updated_at_slot = clock.slot;

    drop(pending);

    if accounts.len() <= RENT_REFUND_RECIPIENT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let rent_refund_recipient_account_info = &accounts[RENT_REFUND_RECIPIENT];

    consume_and_close_pending_change(pending_account_info, rent_refund_recipient_account_info)?;

    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(cfg_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    s_mut.last_event_sequence_nonce = sequence_nonce;

    emit_module_status_changed(
        event_authority_account_info,
        event_authority_bump,
        ModuleStatusChanged {
            sequence_nonce,
            chancery:       *cfg_account_info.key,
            slot:           clock.slot,
            unix_timestamp: clock.unix_timestamp,
            risk_class:     risk.as_u8(),
            module_id:      args.module_id,
            old_status,
            new_status:     args.new_status,
            changed_by:     *governance_authority_account_info.key,
            change_id,
        },
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preloaded_or_same_slot_deprecation_review_is_rejected() {
        assert_eq!(
            assert_deprecation_review_after_disable(module_status::DEPRECATED, 99, 100),
            Err(ChanceryError::ModuleDeprecationReviewPrecedesDisable),
        );
        assert_eq!(
            assert_deprecation_review_after_disable(module_status::DEPRECATED, 100, 100),
            Err(ChanceryError::ModuleDeprecationReviewPrecedesDisable),
        );
    }

    #[test]
    fn deprecation_review_started_after_disable_is_accepted() {
        assert!(assert_deprecation_review_after_disable(
            module_status::DEPRECATED,
            101,
            100,
        ).is_ok());
    }

    #[test]
    fn non_deprecation_transitions_do_not_use_the_disable_review_ordering_gate() {
        assert!(assert_deprecation_review_after_disable(
            module_status::ACTIVE,
            0,
            u64::MAX,
        ).is_ok());
    }
    #[test]
    fn status_payload_binds_module_transition_and_generation() {
        let payload = module_status_change_payload(7, 1, 2, 0x0807_0605_0403_0201);

        assert_eq!(&payload[..3], &[7, 1, 2]);
        assert_eq!(
            &payload[3..],
            &0x0807_0605_0403_0201u64.to_le_bytes(),
        );
        assert_ne!(
            payload,
            module_status_change_payload(7, 1, 2, 0x0807_0605_0403_0202),
        );
    }

}
