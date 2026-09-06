/// register_limit_policy
///
/// Accounts:
///   0  chancery_config    writable  PDA - seq bump
///   1  event_authority    readable  PDA [b"event-authority"]
///   2  limit_policy       writable  PDA [b"limit-policy", limit_policy_id]
///   3  payer              signer
///   4  operations_authority      signer
///   5  system_program
///
/// Binding-time window accounts (required per enabled cap, except for
/// counterparty/executor template policies whose per-party windows are
/// created at permission grant):
///   6  hourly_usage_window   writable  PDA [b"usage-window", scope_hash, HOURLY]
///   7  daily_usage_window    writable  PDA [b"usage-window", scope_hash, DAILY]
///   8  weekly_usage_window   writable  PDA [b"usage-window", scope_hash, WEEKLY]
///   9  monthly_usage_window  writable  PDA [b"usage-window", scope_hash, MONTHLY]
///
/// A window is only ever active through a binding, so the binding creates it:
/// a registered policy whose enabled caps lack their stable accumulators is
/// unrepresentable. Creation is create-if-missing - windows are keyed by
/// scope_hash, not policy id, so a shared-scope sibling policy may already
/// have bound them; re-binding never resets a live cap.

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    constants::{scope, seeds, window_kind},
    error::ChanceryError,
    modules::{
        core::{instructions::create_pda_account, state::chancery_config::ChanceryConfig},
        evidence::emit::{
            emit_limit_policy_registered, emit_usage_window_initialized,
            LimitPolicyRegistered, UsageWindowInitialized,
        },
        limits::state::{
            limit_policy::{LimitPolicy, LIMIT_POLICY_DISCRIMINATOR, LIMIT_POLICY_SIZE},
            usage_window::create_usage_window_if_missing,
        },
    },
};

const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const LIMIT_POLICY:           usize = 2;
const PAYER:                  usize = 3;
const OPERATIONS_AUTHORITY:   usize = 4;
const SYSTEM_PROGRAM:         usize = 5;
const HOURLY_USAGE_WINDOW:    usize = 6;
const DAILY_USAGE_WINDOW:     usize = 7;
const WEEKLY_USAGE_WINDOW:    usize = 8;
const MONTHLY_USAGE_WINDOW:   usize = 9;
const REQUIRED_ACCOUNT_COUNT: usize = 6;

#[derive(BorshDeserialize)]
pub struct RegisterLimitPolicyArgs {
    pub limit_policy_id:          [u8; 32],
    pub scope_kind:               u8,
    pub scope_key:                Pubkey,
    pub per_transaction_maximum:  u64,
    pub per_hour_maximum:         u64,
    pub per_day_maximum:          u64,
    pub per_seven_day_maximum:    u64,
    pub per_thirty_day_maximum:   u64,
    pub maximum_actions_per_hour: u32,
    pub maximum_actions_per_day:  u32,
    // Breach behavior is revert-only (issue RB-05): the former
    // hard_fail_flags / breach_action_flags args were inert and are not accepted.
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info      = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];
    let limit_policy_account_info         = &accounts[LIMIT_POLICY];
    let payer_account_info                = &accounts[PAYER];
    let operations_authority_account_info = &accounts[OPERATIONS_AUTHORITY];
    let system_program_account_info       = &accounts[SYSTEM_PROGRAM];

    if !payer_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !operations_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !limit_policy_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;


    if operations_authority_account_info.key != &chancery_config.operations_authority {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    let args = RegisterLimitPolicyArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let program_id = crate::id();
    let (expected_key, bump) = Pubkey::find_program_address(
        &[seeds::LIMIT_POLICY, args.limit_policy_id.as_ref()],
        &program_id,
    );

    if limit_policy_account_info.key != &expected_key {
        return Err(ChanceryError::InvalidPda.into());
    }

    if limit_policy_account_info.data_is_empty() {
        create_pda_account(
            payer_account_info, limit_policy_account_info, system_program_account_info, &program_id,
            &[seeds::LIMIT_POLICY, args.limit_policy_id.as_ref(), &[bump]],
            LIMIT_POLICY_SIZE,
        )?;
    } else if limit_policy_account_info.data_len() != LIMIT_POLICY_SIZE {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    let mut p = LimitPolicy::load_uninitialized_mut(limit_policy_account_info)?;

    p.discriminator            = LIMIT_POLICY_DISCRIMINATOR;
    p.version                  = 1;
    p.bump                     = bump;
    p.scope_kind               = args.scope_kind;
    p._pad0                    = [0u8; 4];
    p.limit_policy_id          = args.limit_policy_id;
    p.scope_key                = args.scope_key;
    p.per_transaction_maximum  = args.per_transaction_maximum;
    p.per_hour_maximum         = args.per_hour_maximum;
    p.per_day_maximum          = args.per_day_maximum;
    p.per_seven_day_maximum    = args.per_seven_day_maximum;
    p.per_thirty_day_maximum   = args.per_thirty_day_maximum;
    p.maximum_actions_per_hour = args.maximum_actions_per_hour;
    p.maximum_actions_per_day  = args.maximum_actions_per_day;
    p._reserved_breach_flags   = [0u64; 2];
    p.status_flags             = 0;
    p._reserved                = [0u8; 32];

    p.assert_parameter_sanity()?;

    // Drop the mutable borrow on limit_policy account data before the next borrows.
    drop(p);

    // ── Binding-time stable-window creation ───────────────────────────────────
    // Counterparty / executor template policies (scope_key = default) accrue in
    // per-party windows created at permission grant, never here.
    let clock = Clock::get()?;

    let is_template = (args.scope_kind == scope::COUNTERPARTY
        || args.scope_kind == scope::EXECUTOR)
        && args.scope_key == Pubkey::default();

    let mut initialized_windows: Vec<(Pubkey, u8, i64)> = Vec::new();

    if !is_template {
        let scope_hash = crate::modules::limits::dimension::dimension_window_scope_hash(
            args.scope_kind,
            &args.scope_key,
        );

        let enabled = [
            (
                window_kind::HOURLY,
                HOURLY_USAGE_WINDOW,
                args.per_hour_maximum != 0 || args.maximum_actions_per_hour != 0,
            ),
            (
                window_kind::DAILY,
                DAILY_USAGE_WINDOW,
                args.per_day_maximum != 0 || args.maximum_actions_per_day != 0,
            ),
            (window_kind::WEEKLY,  WEEKLY_USAGE_WINDOW,  args.per_seven_day_maximum != 0),
            (window_kind::MONTHLY, MONTHLY_USAGE_WINDOW, args.per_thirty_day_maximum != 0),
        ];

        for (kind, index, on) in enabled {
            if !on {
                continue;
            }

            if accounts.len() <= index {
                return Err(ChanceryError::MissingAccount.into());
            }

            // Inline per-constant fetch at the mutating call: the IDL
            // generator's writability analysis follows `accounts[CONST]`
            // passed directly to a mutably-borrowing callee, but not
            // conditional bindings.
            let created = if index == HOURLY_USAGE_WINDOW {
                create_usage_window_if_missing(
                    &accounts[HOURLY_USAGE_WINDOW],
                    payer_account_info,
                    system_program_account_info,
                    &scope_hash,
                    kind,
                    clock.unix_timestamp,
                    payer_account_info.key,
                    &program_id,
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
                    &program_id,
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
                    &program_id,
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
                    &program_id,
                )?
            };

            if created {
                let window = crate::modules::limits::state::usage_window::UsageWindow::load_verified(
                    &accounts[index],
                )?;
                initialized_windows.push((
                    *accounts[index].key,
                    kind,
                    window.window_start_unix_timestamp,
                ));
            }
        }
    }

    // ── Evidence (emitted last, after all state writes) ─────────────────────────

    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;

    // Total-ever limit-policy count (doc 14 §14.6.4).
    chancery_config_mut.total_limit_policies_registered = chancery_config_mut
        .total_limit_policies_registered
        .checked_add(1)
        .ok_or(ChanceryError::ArithmeticOverflow)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    emit_limit_policy_registered(
        event_authority_account_info,
        event_authority_bump,
        LimitPolicyRegistered {
            sequence_nonce,
            chancery:       *chancery_config_account_info.key,
            slot:           clock.slot,
            unix_timestamp: clock.unix_timestamp,
            risk_class:     0,
            limit_policy:   *limit_policy_account_info.key,
            registered_by:  *operations_authority_account_info.key,
        },
    )?;

    let scope_hash = crate::modules::limits::dimension::dimension_window_scope_hash(
        args.scope_kind,
        &args.scope_key,
    );

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
                risk_class:                  0,
                usage_window:                window,
                scope_hash,
                window_kind:                 kind,
                window_start_unix_timestamp: window_start,
                rent_refund_recipient:       *payer_account_info.key,
                created_by:                  *operations_authority_account_info.key,
            },
        )?;
    }

    Ok(())
}
