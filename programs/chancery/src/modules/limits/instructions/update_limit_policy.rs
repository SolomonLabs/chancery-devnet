/// update_limit_policy
///
/// Direct path only accepts RestrictiveImmediate / RoutineOps (cap
/// reductions, flag tightening). Cap raises and required-flag removals
/// classify as Widening and must route through `handle_with_pending_change`.
///
/// Accounts (direct):
///   0  chancery_config         writable  PDA  (sequence_nonce bump for evidence)
///   1  limit_policy            writable  PDA [b"limit-policy", limit_policy_id]
///   2  operations_authority    signer
///   3  event_authority         readable  PDA [b"event-authority"]
///
/// Accounts (with_pending_change):
///   0  chancery_config         writable  PDA
///   1  event_authority         readable  PDA [b"event-authority"]
///   2  pending_config_change   writable  PDA
///   3  limit_policy            writable  PDA
///   4  governance_authority    signer

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_sysvar::Sysvar;

use crate::{
    constants::change_kind,
    error::ChanceryError,
    modules::{
        control::{
            change_detection::{
                classify_optional_u32_cap_change,
                classify_optional_u64_cap_change,
            },
            change_risk::{
                assert_direct_config_change_allowed,
                max_risk_many,
                ConfigChangeRiskClass,
            },
            pending_change::compute_config_change_hash,
        },
        core::state::chancery_config::ChanceryConfig,
        evidence::emit::{emit_limit_policy_updated, emit_usage_window_initialized,
            LimitPolicyUpdated, UsageWindowInitialized},
        limits::state::limit_policy::LimitPolicy,
    },
};

const CHANCERY_CONFIG:                usize = 0;
const LIMIT_POLICY:                   usize = 1;
const OPERATIONS_AUTHORITY:           usize = 2;
const EVENT_AUTHORITY_DIRECT:         usize = 3;
const REQUIRED_ACCOUNT_COUNT:         usize = 6;

// Binding accounts. payer/system_program are always present (payer signs);
// the windows are optional in slot-order, required iff this update enables
// a cap whose stable window does not exist yet: enabling a cap - even as a
// restrictive 0 -> finite tightening - is a binding surface, and an enforced
// cap without its accumulator would fail settlement closed forever.
const PAYER:                          usize = 4;
const SYSTEM_PROGRAM:                 usize = 5;
const HOURLY_USAGE_WINDOW:            usize = 6;
const DAILY_USAGE_WINDOW:             usize = 7;
const WEEKLY_USAGE_WINDOW:            usize = 8;
const MONTHLY_USAGE_WINDOW:           usize = 9;

#[derive(BorshDeserialize)]
pub struct UpdateLimitPolicyArgs {
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

pub(super) fn apply_limit_policy_update(p: &mut LimitPolicy, args: &UpdateLimitPolicyArgs) {
    if let Some(v) = args.per_transaction_maximum {
        p.per_transaction_maximum  = v;
    }

    if let Some(v) = args.per_hour_maximum {
        p.per_hour_maximum         = v;
    }

    if let Some(v) = args.per_day_maximum {
        p.per_day_maximum          = v;
    }

    if let Some(v) = args.per_seven_day_maximum {
        p.per_seven_day_maximum    = v;
    }

    if let Some(v) = args.per_thirty_day_maximum {
        p.per_thirty_day_maximum   = v;
    }

    if let Some(v) = args.maximum_actions_per_hour {
        p.maximum_actions_per_hour = v;
    }

    if let Some(v) = args.maximum_actions_per_day {
        p.maximum_actions_per_day  = v;
    }

}

pub(super) fn classify_limit_policy_update(
    current: &LimitPolicy,
    args: &UpdateLimitPolicyArgs,
) -> ConfigChangeRiskClass {
    let mut risks: Vec<ConfigChangeRiskClass> = Vec::with_capacity(9);

    if args.per_transaction_maximum.is_some() {
        risks.push(classify_optional_u64_cap_change(
            current.per_transaction_maximum,
            args.per_transaction_maximum,
        ));
    }

    if args.per_hour_maximum.is_some() {
        risks.push(classify_optional_u64_cap_change(
            current.per_hour_maximum,
            args.per_hour_maximum,
        ));
    }

    if args.per_day_maximum.is_some() {
        risks.push(classify_optional_u64_cap_change(
            current.per_day_maximum,
            args.per_day_maximum,
        ));
    }

    if args.per_seven_day_maximum.is_some() {
        risks.push(classify_optional_u64_cap_change(
            current.per_seven_day_maximum,
            args.per_seven_day_maximum,
        ));
    }

    if args.per_thirty_day_maximum.is_some() {
        risks.push(classify_optional_u64_cap_change(
            current.per_thirty_day_maximum,
            args.per_thirty_day_maximum,
        ));
    }

    if args.maximum_actions_per_hour.is_some() {
        risks.push(classify_optional_u32_cap_change(
            current.maximum_actions_per_hour,
            args.maximum_actions_per_hour,
        ));
    }

    if args.maximum_actions_per_day.is_some() {
        risks.push(classify_optional_u32_cap_change(
            current.maximum_actions_per_day,
            args.maximum_actions_per_day,
        ));
    }

    if risks.is_empty() {
        ConfigChangeRiskClass::RoutineOps
    } else {
        max_risk_many(&risks)
    }
}

pub(super) fn limit_policy_update_hashes(
    limit_policy_key: &solana_pubkey::Pubkey,
    risk: ConfigChangeRiskClass,
    current: &LimitPolicy,
    proposed: &LimitPolicy,
) -> ([u8; 32], [u8; 32]) {
    let current_payload  = current.config_change_payload();
    let proposed_payload = proposed.config_change_payload();
    let old_hash = compute_config_change_hash(
        change_kind::UPDATE_LIMIT_POLICY,
        limit_policy_key,
        risk.as_u8(),
        &current_payload,
    );

    let new_hash = compute_config_change_hash(
        change_kind::UPDATE_LIMIT_POLICY,
        limit_policy_key,
        risk.as_u8(),
        &proposed_payload,
    );

    (old_hash, new_hash)
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info       = &accounts[CHANCERY_CONFIG];
    let limit_policy_account_info          = &accounts[LIMIT_POLICY];
    let operations_authority_account_info  = &accounts[OPERATIONS_AUTHORITY];

    if !operations_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !limit_policy_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    if operations_authority_account_info.key != &chancery_config.operations_authority {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    let args = UpdateLimitPolicyArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let program_id        = crate::id();
    let (expected_key, policy_bump) = solana_pubkey::Pubkey::find_program_address(
        &[
            crate::constants::seeds::LIMIT_POLICY,
            args.limit_policy_id.as_ref(),
        ],
        &program_id,
    );

    if limit_policy_account_info.key != &expected_key {
        return Err(ChanceryError::InvalidPda.into());
    }

    let current = *LimitPolicy::load_for_verified_pda(
        limit_policy_account_info,
        &args.limit_policy_id,
        policy_bump,
    )?;
    let mut proposed = current;

    apply_limit_policy_update(&mut proposed, &args);
    proposed.assert_parameter_sanity()?;

    let risk = classify_limit_policy_update(&current, &args);

    assert_direct_config_change_allowed(risk, &chancery_config, operations_authority_account_info)?;

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (old_hash, new_hash) =
        limit_policy_update_hashes(limit_policy_account_info.key, risk, &current, &proposed);

    *LimitPolicy::load_mut_for_verified_pda(
        limit_policy_account_info,
        &args.limit_policy_id,
        policy_bump,
    )? = proposed;

    let clock                = Clock::get()?;

    let mut initialized_windows: Vec<(solana_pubkey::Pubkey, u8, i64)> = Vec::new();
    let mut binding_scope_hash: [u8; 32] = [0u8; 32];

    // ── Binding-time stable-window creation for caps this update enables ─────
    // Idempotent per (scope, kind): create-if-missing verifies the canonical
    // initialized account and never resets a live accumulator. Template
    // policies (counterparty/executor with default scope key) accrue in
    // per-party windows created at permission grant, never here.
    {
        let is_template = (proposed.scope_kind == crate::constants::scope::COUNTERPARTY
            || proposed.scope_kind == crate::constants::scope::EXECUTOR)
            && proposed.scope_key == solana_pubkey::Pubkey::new_from_array([0u8; 32]);

        let enabled = [
            (
                crate::constants::window_kind::HOURLY,
                HOURLY_USAGE_WINDOW,
                proposed.per_hour_maximum != 0 || proposed.maximum_actions_per_hour != 0,
                current.per_hour_maximum != 0 || current.maximum_actions_per_hour != 0,
            ),
            (
                crate::constants::window_kind::DAILY,
                DAILY_USAGE_WINDOW,
                proposed.per_day_maximum != 0 || proposed.maximum_actions_per_day != 0,
                current.per_day_maximum != 0 || current.maximum_actions_per_day != 0,
            ),
            (
                crate::constants::window_kind::WEEKLY,
                WEEKLY_USAGE_WINDOW,
                proposed.per_seven_day_maximum != 0,
                current.per_seven_day_maximum != 0,
            ),
            (
                crate::constants::window_kind::MONTHLY,
                MONTHLY_USAGE_WINDOW,
                proposed.per_thirty_day_maximum != 0,
                current.per_thirty_day_maximum != 0,
            ),
        ];

        let needs_binding = !is_template
            && enabled.iter().any(|(_, _, now_on, was_on)| *now_on && !*was_on);

        if needs_binding {
            if accounts.len() <= SYSTEM_PROGRAM {
                return Err(ChanceryError::MissingAccount.into());
            }

            let payer_account_info          = &accounts[PAYER];
            let system_program_account_info = &accounts[SYSTEM_PROGRAM];

            if !payer_account_info.is_signer {
                return Err(ChanceryError::AccountNotSigner.into());
            }

            binding_scope_hash = crate::modules::limits::dimension::dimension_window_scope_hash(
                proposed.scope_kind,
                &proposed.scope_key,
            );

            for (kind, index, now_on, was_on) in enabled {
                if !now_on || was_on {
                    continue;
                }

                if accounts.len() <= index {
                    return Err(ChanceryError::MissingAccount.into());
                }

            // Inline per-constant fetch at the mutating call: the IDL generator's
            // writability analysis follows `accounts[CONST]` passed directly to a
            // mutably-borrowing callee, but not conditional bindings.
            let created = if index == HOURLY_USAGE_WINDOW {
                crate::modules::limits::state::usage_window::create_usage_window_if_missing(
                    &accounts[HOURLY_USAGE_WINDOW],
                    payer_account_info,
                    system_program_account_info,
                    &binding_scope_hash,
                    kind,
                    clock.unix_timestamp,
                    payer_account_info.key,
                    &program_id,
                )?
            } else if index == DAILY_USAGE_WINDOW {
                crate::modules::limits::state::usage_window::create_usage_window_if_missing(
                    &accounts[DAILY_USAGE_WINDOW],
                    payer_account_info,
                    system_program_account_info,
                    &binding_scope_hash,
                    kind,
                    clock.unix_timestamp,
                    payer_account_info.key,
                    &program_id,
                )?
            } else if index == WEEKLY_USAGE_WINDOW {
                crate::modules::limits::state::usage_window::create_usage_window_if_missing(
                    &accounts[WEEKLY_USAGE_WINDOW],
                    payer_account_info,
                    system_program_account_info,
                    &binding_scope_hash,
                    kind,
                    clock.unix_timestamp,
                    payer_account_info.key,
                    &program_id,
                )?
            } else {
                crate::modules::limits::state::usage_window::create_usage_window_if_missing(
                    &accounts[MONTHLY_USAGE_WINDOW],
                    payer_account_info,
                    system_program_account_info,
                    &binding_scope_hash,
                    kind,
                    clock.unix_timestamp,
                    payer_account_info.key,
                    &program_id,
                )?
            };

                if created {
                    initialized_windows.push((
                        *accounts[index].key,
                        kind,
                        crate::modules::limits::state::usage_window::canonical_window_start(
                            kind,
                            clock.unix_timestamp,
                        )?,
                    ));
                }
            }
        }
    }
    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    emit_limit_policy_updated(
        &accounts[EVENT_AUTHORITY_DIRECT],
        event_authority_bump,
        LimitPolicyUpdated {
            sequence_nonce,
            chancery:       *chancery_config_account_info.key,
            slot:           clock.slot,
            unix_timestamp: clock.unix_timestamp,
            risk_class:     risk.as_u8(),
            change_id:      [0u8; 32],
            limit_policy:   *limit_policy_account_info.key,
            old_value_hash: old_hash,
            new_value_hash: new_hash,
            proposed_by:    *operations_authority_account_info.key,
            updated_by:     *operations_authority_account_info.key,
        },
    )?;

    for (window, kind, window_start) in initialized_windows {
        let window_sequence_nonce = chancery_config_mut.next_sequence_nonce()?;

        emit_usage_window_initialized(
            &accounts[EVENT_AUTHORITY_DIRECT],
            event_authority_bump,
            UsageWindowInitialized {
                sequence_nonce:              window_sequence_nonce,
                chancery:                    *chancery_config_account_info.key,
                slot:                        clock.slot,
                unix_timestamp:              clock.unix_timestamp,
                risk_class:                  risk.as_u8(),
                usage_window:                window,
                scope_hash:                  binding_scope_hash,
                window_kind:                 kind,
                window_start_unix_timestamp: window_start,
                rent_refund_recipient:       *accounts[PAYER].key,
                created_by:                  *operations_authority_account_info.key,
            },
        )?;
    }

    Ok(())
}
