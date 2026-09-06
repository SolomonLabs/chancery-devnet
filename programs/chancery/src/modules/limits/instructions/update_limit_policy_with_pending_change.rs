/// update_limit_policy_with_pending_change
///
/// Timelocked (Widening) limit-policy updates: consumes an accepted
/// PendingConfigChange. Direct/RestrictiveImmediate/RoutineOps updates use
/// `update_limit_policy::handle`.
///
/// Accounts:
///   0  chancery_config         writable  PDA
///   1  event_authority         readable  PDA [b"event-authority"]
///   2  pending_config_change   writable  PDA
///   3  limit_policy            writable  PDA
///   4  governance_authority    signer
///
/// Binding-time window accounts, required when this consume enables a cap
/// whose stable window may not exist yet (skipped for counterparty/executor
/// template policies):
///   5  payer                 signer    rent for any newly bound window
///   6  system_program
///   7  hourly_usage_window   writable  PDA [b"usage-window", scope_hash, HOURLY]
///   8  daily_usage_window    writable  PDA [b"usage-window", scope_hash, DAILY]
///   9  weekly_usage_window   writable  PDA [b"usage-window", scope_hash, WEEKLY]
///  10  monthly_usage_window  writable  PDA [b"usage-window", scope_hash, MONTHLY]
///
/// Create-if-missing per enabled kind on the post-update policy: a widening
/// that turns a cap on binds its accumulator in the same transaction, so an
/// enforced cap without a window is unrepresentable. Existing windows are
/// verified and left untouched - a consume never resets a live cap.

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_sysvar::Sysvar;

use solana_pubkey::Pubkey;

use crate::{
    constants::{change_kind, scope, window_kind},
    error::ChanceryError,
    modules::{
        control::{
            pending_change::{assert_accepted_pending_change, consume_and_close_pending_change},
        },
        core::state::chancery_config::ChanceryConfig,
        evidence::emit::{
            emit_limit_policy_updated, emit_usage_window_initialized,
            LimitPolicyUpdated, UsageWindowInitialized,
        },
        limits::{
            dimension::dimension_window_scope_hash,
            state::{
                limit_policy::LimitPolicy,
                usage_window::{create_usage_window_if_missing, UsageWindow},
            },
        },
    },
};

use super::update_limit_policy::{
    apply_limit_policy_update, classify_limit_policy_update, limit_policy_update_hashes,
    UpdateLimitPolicyArgs,
};

const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const PENDING_CONFIG_CHANGE:  usize = 2;
const LIMIT_POLICY:           usize = 3;
const GOVERNANCE_AUTHORITY:   usize = 4;
const PAYER:                  usize = 5;
const SYSTEM_PROGRAM:         usize = 6;
const HOURLY_USAGE_WINDOW:    usize = 7;
const DAILY_USAGE_WINDOW:     usize = 8;
const WEEKLY_USAGE_WINDOW:    usize = 9;
const MONTHLY_USAGE_WINDOW:   usize = 10;
const REQUIRED_ACCOUNT_COUNT: usize = 7;

// Refund conveyance: the account whose key equals the record's stored
// rent_refund_recipient. Required whenever this instruction closes the
// record - the refund is conveyed, never redirected.
const RENT_REFUND_RECIPIENT:  usize = 11;

#[derive(BorshDeserialize)]
pub struct UpdateLimitPolicyWithPendingChangeArgs {
    pub limit_policy_id:           [u8; 32],
    pub per_transaction_maximum:   Option<u64>,
    pub per_hour_maximum:          Option<u64>,
    pub per_day_maximum:           Option<u64>,
    pub per_seven_day_maximum:     Option<u64>,
    pub per_thirty_day_maximum:    Option<u64>,
    pub maximum_actions_per_hour:  Option<u32>,
    pub maximum_actions_per_day:   Option<u32>,
    // Breach behavior is revert-only (issue RB-05): the former
    // hard_fail_flags / breach_action_flags args were inert and are not accepted.
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info        = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info        = &accounts[EVENT_AUTHORITY];
    let pending_config_change_account_info  = &accounts[PENDING_CONFIG_CHANGE];
    let limit_policy_account_info           = &accounts[LIMIT_POLICY];
    let governance_authority_account_info   = &accounts[GOVERNANCE_AUTHORITY];

    if !governance_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !pending_config_change_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !limit_policy_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    if governance_authority_account_info.key != &chancery_config.governance_authority {
        return Err(ChanceryError::ConfigChangeWideningRequiresGovernance.into());
    }

    let local = UpdateLimitPolicyWithPendingChangeArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let args = UpdateLimitPolicyArgs {
        limit_policy_id:           local.limit_policy_id,
        per_transaction_maximum:   local.per_transaction_maximum,
        per_hour_maximum:          local.per_hour_maximum,
        per_day_maximum:           local.per_day_maximum,
        per_seven_day_maximum:     local.per_seven_day_maximum,
        per_thirty_day_maximum:    local.per_thirty_day_maximum,
        maximum_actions_per_hour:  local.maximum_actions_per_hour,
        maximum_actions_per_day:   local.maximum_actions_per_day,
    };

    let limit_policy_bump = LimitPolicy::verify_pda(
        limit_policy_account_info,
        &args.limit_policy_id,
        &crate::id(),
    )?;

    let current      = *LimitPolicy::load_for_verified_pda(
        limit_policy_account_info,
        &args.limit_policy_id,
        limit_policy_bump,
    )?;
    let mut proposed = current;

    apply_limit_policy_update(&mut proposed, &args);
    proposed.assert_parameter_sanity()?;

    let risk = classify_limit_policy_update(&current, &args);

    if !risk.requires_timelock() {
        return Err(ChanceryError::ConfigChangeNotAccepted.into());
    }

    let (old_hash, new_hash) =
        limit_policy_update_hashes(limit_policy_account_info.key, risk, &current, &proposed);

    let pending = assert_accepted_pending_change(
        pending_config_change_account_info,
        change_kind::UPDATE_LIMIT_POLICY,
        risk,
        limit_policy_account_info.key,
        &old_hash,
        &new_hash,
        &chancery_config.governance_authority,
    )?;

    let change_id   = pending.change_id;
    let proposed_by = pending.proposed_by;

    *LimitPolicy::load_mut_for_verified_pda(
        limit_policy_account_info,
        &args.limit_policy_id,
        limit_policy_bump,
    )? = proposed;

    let clock = Clock::get()?;
    drop(pending);

    // ── Binding-time stable-window creation for newly enabled caps ────────────
    let is_template = (proposed.scope_kind == scope::COUNTERPARTY
        || proposed.scope_kind == scope::EXECUTOR)
        && proposed.scope_key == Pubkey::default();

    let mut initialized_windows: Vec<(Pubkey, u8, i64)> = Vec::new();

    if !is_template {
        let scope_hash = dimension_window_scope_hash(proposed.scope_kind, &proposed.scope_key);

        let enabled = [
            (
                window_kind::HOURLY,
                HOURLY_USAGE_WINDOW,
                proposed.per_hour_maximum != 0 || proposed.maximum_actions_per_hour != 0,
            ),
            (
                window_kind::DAILY,
                DAILY_USAGE_WINDOW,
                proposed.per_day_maximum != 0 || proposed.maximum_actions_per_day != 0,
            ),
            (window_kind::WEEKLY,  WEEKLY_USAGE_WINDOW,  proposed.per_seven_day_maximum != 0),
            (window_kind::MONTHLY, MONTHLY_USAGE_WINDOW, proposed.per_thirty_day_maximum != 0),
        ];

        for (kind, index, on) in enabled {
            if !on {
                continue;
            }

            if accounts.len() <= index {
                return Err(ChanceryError::MissingAccount.into());
            }

            let payer_account_info          = &accounts[PAYER];
            let system_program_account_info = &accounts[SYSTEM_PROGRAM];

            if !payer_account_info.is_signer {
                return Err(ChanceryError::AccountNotSigner.into());
            }

            // Inline per-constant fetch at the mutating call: the IDL generator's
            // writability analysis follows `accounts[CONST]` passed directly to a
            // mutably-borrowing callee, but not conditional bindings.
            let created = if index == HOURLY_USAGE_WINDOW {
                create_usage_window_if_missing(
                &accounts[HOURLY_USAGE_WINDOW],
                payer_account_info,
                system_program_account_info,
                &scope_hash,
                kind,
                clock.unix_timestamp,
                payer_account_info.key,
                &crate::id(),
            )?
            } else if index == DAILY_USAGE_WINDOW {
                create_usage_window_if_missing(
                &accounts[DAILY_USAGE_WINDOW],
                payer_account_info,
                system_program_account_info,
                &scope_hash,
                kind,
                clock.unix_timestamp,
                payer_account_info.key,
                &crate::id(),
            )?
            } else if index == WEEKLY_USAGE_WINDOW {
                create_usage_window_if_missing(
                &accounts[WEEKLY_USAGE_WINDOW],
                payer_account_info,
                system_program_account_info,
                &scope_hash,
                kind,
                clock.unix_timestamp,
                payer_account_info.key,
                &crate::id(),
            )?
            } else {
                create_usage_window_if_missing(
                &accounts[MONTHLY_USAGE_WINDOW],
                payer_account_info,
                system_program_account_info,
                &scope_hash,
                kind,
                clock.unix_timestamp,
                payer_account_info.key,
                &crate::id(),
            )?
            };

            if created {
                let window = UsageWindow::load_verified(&accounts[index])?;
                initialized_windows.push((
                    *accounts[index].key,
                    kind,
                    window.window_start_unix_timestamp,
                ));
            }
        }
    }

    if accounts.len() <= RENT_REFUND_RECIPIENT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let rent_refund_recipient_account_info = &accounts[RENT_REFUND_RECIPIENT];

    consume_and_close_pending_change(pending_config_change_account_info, rent_refund_recipient_account_info)?;

    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    emit_limit_policy_updated(
        event_authority_account_info,
        event_authority_bump,
        LimitPolicyUpdated {
            sequence_nonce,
            chancery:       *chancery_config_account_info.key,
            slot:           clock.slot,
            unix_timestamp: clock.unix_timestamp,
            risk_class:     risk.as_u8(),
            change_id,
            limit_policy:   *limit_policy_account_info.key,
            old_value_hash: old_hash,
            new_value_hash: new_hash,
            proposed_by,
            updated_by:     *governance_authority_account_info.key,
        },
    )?;

    if !initialized_windows.is_empty() {
        let scope_hash = dimension_window_scope_hash(proposed.scope_kind, &proposed.scope_key);

        for (window, kind, window_start) in initialized_windows {
            let window_sequence_nonce = chancery_config_mut.next_sequence_nonce()?;

            emit_usage_window_initialized(
                event_authority_account_info,
                event_authority_bump,
                UsageWindowInitialized {
                    sequence_nonce:              window_sequence_nonce,
                    chancery:                    *chancery_config_account_info.key,
                    slot:                        clock.slot,
                    unix_timestamp:              clock.unix_timestamp,
                    risk_class:                  risk.as_u8(),
                    usage_window:                window,
                    scope_hash,
                    window_kind:                 kind,
                    window_start_unix_timestamp: window_start,
                    rent_refund_recipient:       *accounts[PAYER].key,
                    created_by:                  *governance_authority_account_info.key,
                },
            )?;
        }
    }

    Ok(())
}
