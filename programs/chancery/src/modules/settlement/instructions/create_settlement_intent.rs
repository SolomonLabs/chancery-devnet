/// create_settlement_intent
///
/// Pre-commits a settlement intent PDA that delegated and trilateral handlers
/// consume atomically. Direct settlement does not require a prior intent -
/// it can proceed without one.
///
/// intent_id is content-addressed: intent_id == intent_hash ==
/// sha256(domain_separator || canonical encoding of all economic fields).
/// Any squat-attempt with mismatched content produces a different intent_id,
/// so the same intent_id can only ever be allocated for one specific set of
/// fields.
///
/// Accounts:
///   0  chancery_config               writable  PDA - seq bump for evidence
///   1  event_authority               readable  PDA [b"event-authority"]
///   2  settlement_intent             writable  PDA [b"settlement-intent", intent_id]
///   3  payer                         signer    funds rent
///   4  creator                       signer    principal_a/b, or executor with CAN_EXECUTE_SETTLEMENT
///   5  system_program
///   6  pathway_policy                readable  canonical active pathway committed by pathway_id
///   7  principal_a_permission_record readable PDA - action-specific delegated role
///   8  principal_b_permission_record readable PDA - trilateral role; default placeholder for delegated
///   9  executor_permission_record    readable PDA - CAN_EXECUTE_SETTLEMENT
///  10  settlement_policy             readable PDA - required iff policy_id is non-zero
///  11  fee_policy                    readable PDA - required iff pathway fee_policy_id is non-zero

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;
use solana_sha256_hasher::hashv;
use solana_sysvar::Sysvar;

use crate::{
    account_security::{
        assert_distinct_present_accounts, assert_external_identity,
        assert_external_signer,
    },
    constants::{intent_status, pathway_kind, role, scope, seeds, settlement_action, settlement_mode},
    error::ChanceryError,
    modules::{
        core::{
            instructions::create_pda_account,
            state::chancery_config::ChanceryConfig,
        },
        fees::state::fee_policy::FeePolicy,
        pathway::state::pathway_policy::PathwayPolicy,
        permissions::auth::assert_subject_holds_role,
        settlement::{
            instructions::assert_settlement_policy_for_terms,
            state::settlement_intent::{
                SettlementIntent, SETTLEMENT_INTENT_DISCRIMINATOR,
                SETTLEMENT_INTENT_RESERVED_SIZE, SETTLEMENT_INTENT_SIZE,
            },
        },
        evidence::emit::{emit_settlement_intent_created, SettlementIntentCreated},
    },
};

const CHANCERY_CONFIG:               usize = 0;
const EVENT_AUTHORITY:               usize = 1;
const SETTLEMENT_INTENT:             usize = 2;
const PAYER:                         usize = 3;
const CREATOR:                       usize = 4;
const SYSTEM_PROGRAM:                usize = 5;
const PATHWAY_POLICY:                usize = 6;
const PRINCIPAL_A_PERMISSION_RECORD: usize = 7;
const PRINCIPAL_B_PERMISSION_RECORD: usize = 8;
const EXECUTOR_PERMISSION_RECORD:    usize = 9;
const REQUIRED_ACCOUNT_COUNT:        usize = 10;

const SETTLEMENT_POLICY:             usize = 10; // optional; required iff policy_id is non-zero
const FEE_POLICY:                    usize = 11; // optional; required iff pathway has fee policy

#[derive(BorshDeserialize, Clone, Copy)]
pub struct CreateSettlementIntentArgs {
    pub intent_id:                   [u8; 32],
    pub pathway_id:                  [u8; 32],
    pub settlement_mode:             u8,
    pub settlement_action:           u8,
    pub principal_a:                 Pubkey,
    pub principal_b:                 Pubkey,

    /// Zero for direct settlement.
    pub executor:                    Pubkey,
    pub asset_mint:                  Pubkey,
    pub issued_token_mint:           Pubkey,

    /// Delegated output destinations are intentionally owner-and-mint bound.
    /// The permissioned executor chooses the concrete principal-A-owned token
    /// account at execution; no destination account address is committed here.
    pub asset_amount:                u64,
    pub issued_token_amount:         u64,
    pub minimum_asset_amount:        u64,
    pub minimum_issued_token_amount: u64,
    pub nonce:                       u64,
    pub valid_after_unix_timestamp:  i64,
    pub expires_at_unix_timestamp:   i64,
    pub policy_id:                   [u8; 32],
    /// sha256(domain_separator || canonical args encoding). Recomputed on
    /// chain - caller-supplied value must match exactly.
    pub intent_hash:                 [u8; 32],
}

/// Canonical preimage encoding for intent_hash. Mirrored off-chain.
fn compute_intent_hash(
    domain_separator: &[u8; 32],
    args:             &CreateSettlementIntentArgs,
) -> [u8; 32] {
    hashv(&[
        domain_separator,
        &args.pathway_id,
        &[args.settlement_mode],
        &[args.settlement_action],
        args.principal_a.as_ref(),
        args.principal_b.as_ref(),
        args.executor.as_ref(),
        args.asset_mint.as_ref(),
        args.issued_token_mint.as_ref(),
        &args.asset_amount.to_be_bytes(),
        &args.issued_token_amount.to_be_bytes(),
        &args.minimum_asset_amount.to_be_bytes(),
        &args.minimum_issued_token_amount.to_be_bytes(),
        &args.nonce.to_be_bytes(),
        &args.valid_after_unix_timestamp.to_be_bytes(),
        &args.expires_at_unix_timestamp.to_be_bytes(),
        &args.policy_id,
    ]).to_bytes()
}

fn input_notional(args: &CreateSettlementIntentArgs) -> Result<u64, ProgramError> {
    match args.settlement_action {
        settlement_action::MINT   => Ok(args.asset_amount),
        settlement_action::REDEEM => Ok(args.issued_token_amount),
        _                         => Err(ChanceryError::IntentInvalidParameters.into()),
    }
}

fn assert_executable_intent_parameters(
    args:                   &CreateSettlementIntentArgs,
    issued_token_mint:      &Pubkey,
    current_unix_timestamp: i64,
    program_id:             &Pubkey,
) -> ProgramResult {
    // Direct settlement is intentionally stateless and has no intent consumer.
    if args.settlement_mode != settlement_mode::DELEGATED_NON_CUSTODIAL
        && args.settlement_mode != settlement_mode::TRILATERAL_ATOMIC
    {
        return Err(ChanceryError::SettlementModeNotAllowed.into());
    }

    if args.pathway_id == [0u8; 32]
        || args.asset_mint == Pubkey::default()
        || args.issued_token_mint == Pubkey::default()
        || &args.issued_token_mint != issued_token_mint
        || args.asset_mint == args.issued_token_mint
    {
        return Err(ChanceryError::IntentInvalidParameters.into());
    }

    assert_external_identity(&args.principal_a, program_id)?;
    assert_external_identity(&args.principal_b, program_id)?;
    assert_external_identity(&args.executor, program_id)?;

    match args.settlement_action {
        settlement_action::MINT => {
            if args.asset_amount == 0
                || args.issued_token_amount != 0
                || args.minimum_asset_amount != 0
            {
                return Err(ChanceryError::IntentInvalidParameters.into());
            }
        }
        settlement_action::REDEEM => {
            if args.issued_token_amount == 0
                || args.asset_amount != 0
                || args.minimum_issued_token_amount != 0
            {
                return Err(ChanceryError::IntentInvalidParameters.into());
            }
        }
        _ => return Err(ChanceryError::IntentInvalidParameters.into()),
    }

    match args.settlement_mode {
        settlement_mode::DELEGATED_NON_CUSTODIAL => {
            // Delegated settlement has one principal. The shared wire layout
            // retains `principal_b`, but delegated intents must canonically
            // repeat principal A there, matching execution and evidence.
            if args.principal_b != args.principal_a
                || args.executor == args.principal_a
            {
                return Err(ChanceryError::IntentInvalidParameters.into());
            }
        }
        settlement_mode::TRILATERAL_ATOMIC => {}
        _ => return Err(ChanceryError::SettlementModeNotAllowed.into()),
    }

    if args.expires_at_unix_timestamp != 0 {
        if args.expires_at_unix_timestamp <= current_unix_timestamp {
            return Err(ChanceryError::IntentExpired.into());
        }

        if args.valid_after_unix_timestamp != 0
            && args.valid_after_unix_timestamp >= args.expires_at_unix_timestamp
        {
            return Err(ChanceryError::IntentInvalidParameters.into());
        }
    }

    Ok(())
}

/// Zero-expiry intents never leave the live set through the permissionless
/// expiry sweep, and cancellation is restricted to `principal_a`, so a payer
/// other than `principal_a` would have no self-service path to recover the
/// rent funding a never-expiring intent. `rent_refund_recipient` is always
/// the payer and every allowed settlement mode permits a third-party payer,
/// so the guard is mode-independent: zero expiry requires `principal_a` to
/// fund the account. The mode allowlist has already run in
/// `assert_executable_intent_parameters`.
fn assert_rent_recoverable(
    args:  &CreateSettlementIntentArgs,
    payer: &Pubkey,
) -> ProgramResult {
    if args.expires_at_unix_timestamp == 0 && payer != &args.principal_a {
        return Err(ChanceryError::IntentInvalidParameters.into());
    }

    Ok(())
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let program_id = crate::id();

    let chancery_config_account_info   = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info   = &accounts[EVENT_AUTHORITY];
    let settlement_intent_account_info = &accounts[SETTLEMENT_INTENT];
    let payer_account_info             = &accounts[PAYER];
    let creator_account_info           = &accounts[CREATOR];
    let system_program_account_info    = &accounts[SYSTEM_PROGRAM];
    let pathway_policy_account_info    = &accounts[PATHWAY_POLICY];

    assert_external_signer(payer_account_info, &program_id)?;
    assert_external_signer(creator_account_info, &program_id)?;

    if !settlement_intent_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    let args = CreateSettlementIntentArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let clock = Clock::get()?;
    assert_executable_intent_parameters(
        &args,
        &chancery_config.issued_token_mint,
        clock.unix_timestamp,
        &program_id,
    )?;
    assert_rent_recoverable(&args, payer_account_info.key)?;

    let earliest_execution_timestamp = if args.valid_after_unix_timestamp
        > clock.unix_timestamp
    {
        args.valid_after_unix_timestamp
    } else {
        clock.unix_timestamp
    };

    assert_distinct_present_accounts(&[
        chancery_config_account_info,
        event_authority_account_info,
        settlement_intent_account_info,
        system_program_account_info,
        pathway_policy_account_info,
    ])?;

    // Bind the intent to a live canonical pathway rather than merely deriving
    // the key off-chain. This prevents storing terms that no current consumer
    // can execute even at the instant of creation.
    let pathway_policy_bump = PathwayPolicy::verify_pda(
        pathway_policy_account_info,
        &args.pathway_id,
        &program_id,
    )?;

    let pathway = PathwayPolicy::load_for_verified_pda(
        pathway_policy_account_info,
        &args.pathway_id,
        pathway_policy_bump,
    )?;

    pathway.assert_active()?;
    pathway.assert_runtime_supported()?;

    if pathway.asset_mint != args.asset_mint
        || pathway.issued_token_mint != args.issued_token_mint
    {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    let expected_pathway_kind = match args.settlement_mode {
        settlement_mode::DELEGATED_NON_CUSTODIAL => pathway_kind::DELEGATED,
        settlement_mode::TRILATERAL_ATOMIC       => pathway_kind::TRILATERAL,
        _                                        => return Err(ChanceryError::SettlementModeNotAllowed.into()),
    };
    pathway.assert_kind(expected_pathway_kind)?;
    pathway.assert_executor(&args.executor)?;

    // The pathway stores one fee-policy reference. If present, its exactly-one
    // denomination must match the requested action, otherwise the intent would
    // be unexecutable by the corresponding consume handler from birth.
    if pathway.has_fee_policy() {
        if accounts.len() <= FEE_POLICY || accounts[FEE_POLICY].key == &Pubkey::default() {
            return Err(ChanceryError::MissingAccount.into());
        }

        let fee_policy_account_info = &accounts[FEE_POLICY];
        let fee_policy_bump = FeePolicy::verify_pda(
            fee_policy_account_info,
            &pathway.fee_policy_id,
            &program_id,
        )?;
        let fee_policy = FeePolicy::load_for_verified_pda(
            fee_policy_account_info,
            &pathway.fee_policy_id,
            fee_policy_bump,
        )?;
        fee_policy.assert_effective(earliest_execution_timestamp)?;

        let denomination_matches = match args.settlement_action {
            settlement_action::MINT   => fee_policy.fee_in_issued_token(),
            settlement_action::REDEEM => fee_policy.fee_in_asset(),
            _                         => false,
        };
        if !denomination_matches {
            return Err(ChanceryError::FeePolicyDenominationMismatch.into());
        }
    } else if accounts.len() > FEE_POLICY
        && accounts[FEE_POLICY].key != &Pubkey::default()
    {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    // Creator must be principal A (or principal B for trilateral intents), or
    // the committed executor. Authority to create and static executability are
    // separate checks: all consume-side role records are proven below even if
    // one identity occupies more than one role.
    let creator_is_principal = creator_account_info.key == &args.principal_a
        || (args.settlement_mode == settlement_mode::TRILATERAL_ATOMIC
            && creator_account_info.key == &args.principal_b);
    let creator_is_executor = creator_account_info.key == &args.executor;

    if !creator_is_principal && !creator_is_executor {
        return Err(ChanceryError::InsufficientRole.into());
    }

    let principal_a_permission_record_account_info =
        &accounts[PRINCIPAL_A_PERMISSION_RECORD];
    let principal_b_permission_record_account_info =
        &accounts[PRINCIPAL_B_PERMISSION_RECORD];
    let executor_permission_record_account_info =
        &accounts[EXECUTOR_PERMISSION_RECORD];

    if principal_a_permission_record_account_info.key == &Pubkey::default()
        || executor_permission_record_account_info.key == &Pubkey::default()
    {
        return Err(ChanceryError::MissingAccount.into());
    }

    let principal_a_required_role = match args.settlement_action {
        settlement_action::MINT => role::CAN_MINT_DELEGATED,
        settlement_action::REDEEM => role::CAN_REDEEM_DELEGATED,
        _ => return Err(ChanceryError::IntentInvalidParameters.into()),
    };

    let principal_a_permission_record = assert_subject_holds_role(
        &args.principal_a,
        principal_a_permission_record_account_info,
        principal_a_required_role,
        scope::PATHWAY,
        pathway_policy_account_info.key,
        earliest_execution_timestamp,
        &program_id,
    )?;

    drop(principal_a_permission_record);

    if args.settlement_mode == settlement_mode::TRILATERAL_ATOMIC {
        if principal_b_permission_record_account_info.key == &Pubkey::default() {
            return Err(ChanceryError::MissingAccount.into());
        }

        let principal_b_permission_record = assert_subject_holds_role(
            &args.principal_b,
            principal_b_permission_record_account_info,
            role::CAN_USE_TRILATERAL_PATHWAY,
            scope::PATHWAY,
            pathway_policy_account_info.key,
            earliest_execution_timestamp,
            &program_id,
        )?;
        drop(principal_b_permission_record);
    } else if principal_b_permission_record_account_info.key != &Pubkey::default() {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    let executor_permission_record = assert_subject_holds_role(
        &args.executor,
        executor_permission_record_account_info,
        role::CAN_EXECUTE_SETTLEMENT,
        scope::PATHWAY,
        pathway_policy_account_info.key,
        earliest_execution_timestamp,
        &program_id,
    )?;
    drop(executor_permission_record);

    // Settlement policy binds fail-closed: a nonzero policy id makes the
    // policy account mandatory (indexed here so the IDL derives it from
    // source, issue RB-03).
    if args.policy_id != [0u8; 32] {
        if accounts.len() <= SETTLEMENT_POLICY || accounts[SETTLEMENT_POLICY].key == &Pubkey::default() {
            return Err(ChanceryError::MissingAccount.into());
        }

        let policy_principal_b = if args.settlement_mode
            == settlement_mode::DELEGATED_NON_CUSTODIAL
        {
            args.principal_a
        } else {
            args.principal_b
        };

        assert_settlement_policy_for_terms(
            &accounts[SETTLEMENT_POLICY],
            &args.policy_id,
            args.settlement_mode,
            &args.asset_mint,
            &args.principal_a,
            &policy_principal_b,
            &args.executor,
            input_notional(&args)?,
            earliest_execution_timestamp,
        )?;
    } else if accounts.len() > SETTLEMENT_POLICY
        && accounts[SETTLEMENT_POLICY].key != &Pubkey::default()
    {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if accounts.len() > FEE_POLICY {
        assert_distinct_present_accounts(&[
            pathway_policy_account_info,
            &accounts[SETTLEMENT_POLICY],
            &accounts[FEE_POLICY],
        ])?;
    } else if accounts.len() > SETTLEMENT_POLICY {
        assert_distinct_present_accounts(&[
            pathway_policy_account_info,
            &accounts[SETTLEMENT_POLICY],
        ])?;
    }

    // Content-addressing: recompute the hash on chain and pin intent_id to it.
    // Any drift in any committed field produces a different intent_id, so the
    // PDA namespace becomes squat-resistant by construction.
    let recomputed_hash = compute_intent_hash(&chancery_config.domain_separator, &args);

    if args.intent_hash != recomputed_hash {
        return Err(ChanceryError::IntentHashMismatch.into());
    }

    if args.intent_id != recomputed_hash {
        return Err(ChanceryError::IntentHashMismatch.into());
    }

    let (expected_key, bump) = Pubkey::find_program_address(
        &[seeds::SETTLEMENT_INTENT, args.intent_id.as_ref()],
        &program_id,
    );

    if settlement_intent_account_info.key != &expected_key {
        return Err(ChanceryError::InvalidPda.into());
    }

    // Intent creation is allocation-only. A closed PDA remains program-owned
    // and allocated until the transaction ends; accepting a zero discriminator
    // here would let a later instruction re-fund and revive the terminal shell.
    // Requiring an empty system account makes close terminal within the current
    // transaction. Exact historical-address reuse is optional and not
    // guaranteed; a new lifecycle should use a fresh caller nonce (spec 14
    // §14.3.1).
    if !settlement_intent_account_info.data_is_empty() {
        return Err(ChanceryError::AlreadyInitialized.into());
    }

    create_pda_account(
        payer_account_info,
        settlement_intent_account_info,
        system_program_account_info,
        &program_id,
        &[seeds::SETTLEMENT_INTENT, args.intent_id.as_ref(), &[bump]],
        SETTLEMENT_INTENT_SIZE,
    )?;

    let mut intent = SettlementIntent::load_uninitialized_mut(settlement_intent_account_info)?;

    intent.discriminator               = SETTLEMENT_INTENT_DISCRIMINATOR;
    intent.version                     = 1;
    intent.bump                        = bump;
    intent.status                      = intent_status::PENDING;
    intent.settlement_mode             = args.settlement_mode;
    intent.settlement_action           = args.settlement_action;
    intent._pad0                       = [0u8; 2];
    intent.intent_id                   = args.intent_id;
    intent.principal_a                 = args.principal_a;
    intent.principal_b                 = args.principal_b;
    intent.executor                    = args.executor;
    intent.asset_mint                  = args.asset_mint;
    intent.issued_token_mint           = args.issued_token_mint;
    intent.asset_amount                = args.asset_amount;
    intent.issued_token_amount         = args.issued_token_amount;
    intent.minimum_asset_amount        = args.minimum_asset_amount;
    intent.minimum_issued_token_amount = args.minimum_issued_token_amount;
    intent.nonce                       = args.nonce;
    intent.valid_after_unix_timestamp  = args.valid_after_unix_timestamp;
    intent.expires_at_unix_timestamp   = args.expires_at_unix_timestamp;
    intent.policy_id                   = args.policy_id;
    intent.intent_hash                 = args.intent_hash;
    intent.pathway_id                  = args.pathway_id;
    intent.rent_refund_recipient       = *payer_account_info.key;
    intent._reserved                   = [0u8; SETTLEMENT_INTENT_RESERVED_SIZE];

    drop(intent);

    // ── Evidence (emitted last, after all state writes) ─────────────────────────
    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    emit_settlement_intent_created(
        event_authority_account_info,
        event_authority_bump,
        SettlementIntentCreated {
            sequence_nonce,
            chancery:                    *chancery_config_account_info.key,
            slot:                        clock.slot,
            unix_timestamp:              clock.unix_timestamp,
            risk_class:                  0,
            settlement_intent:           *settlement_intent_account_info.key,
            intent_id:                   args.intent_id,
            pathway_id:                  args.pathway_id,
            pathway_policy:              *pathway_policy_account_info.key,
            settlement_mode:             args.settlement_mode,
            settlement_action:           args.settlement_action,
            principal_a:                 args.principal_a,
            principal_b:                 args.principal_b,
            executor:                    args.executor,
            asset_mint:                  args.asset_mint,
            issued_token_mint:           args.issued_token_mint,
            asset_amount:                args.asset_amount,
            issued_token_amount:         args.issued_token_amount,
            minimum_asset_amount:        args.minimum_asset_amount,
            minimum_issued_token_amount: args.minimum_issued_token_amount,
            nonce:                       args.nonce,
            valid_after_unix_timestamp:  args.valid_after_unix_timestamp,
            expires_at_unix_timestamp:   args.expires_at_unix_timestamp,
            policy_id:                   args.policy_id,
            intent_hash:                 args.intent_hash,
            created_by:                  *creator_account_info.key,
        },
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(byte: u8) -> Pubkey {
        Pubkey::new_from_array([byte; 32])
    }

    fn assert_executable(
        args:                   &CreateSettlementIntentArgs,
        issued_token_mint:      &Pubkey,
        current_unix_timestamp: i64,
    ) -> ProgramResult {
        let program_id = crate::id();

        assert_executable_intent_parameters(
            args,
            issued_token_mint,
            current_unix_timestamp,
            &program_id,
        )
    }

    fn delegated_mint_args() -> CreateSettlementIntentArgs {
        CreateSettlementIntentArgs {
            intent_id: [0u8; 32],
            pathway_id: [1u8; 32],
            settlement_mode: settlement_mode::DELEGATED_NON_CUSTODIAL,
            settlement_action: settlement_action::MINT,
            principal_a: key(1),
            principal_b: key(1),
            executor: key(2),
            asset_mint: key(3),
            issued_token_mint: key(4),
            asset_amount: 1_000,
            issued_token_amount: 0,
            minimum_asset_amount: 0,
            minimum_issued_token_amount: 900,
            nonce: 7,
            valid_after_unix_timestamp: 0,
            expires_at_unix_timestamp: 0,
            policy_id: [0u8; 32],
            intent_hash: [0u8; 32],
        }
    }

    fn assert_invalid(args: &CreateSettlementIntentArgs) {
        assert_eq!(
            assert_executable(args, &key(4), 100),
            Err(ChanceryError::IntentInvalidParameters.into()),
        );
    }

    #[test]
    fn delegated_intents_are_principal_a_only() {
        let valid = delegated_mint_args();
        assert!(assert_executable(&valid, &key(4), 100).is_ok());

        let mut distinct_b = delegated_mint_args();
        distinct_b.principal_b = key(5);
        assert_invalid(&distinct_b);
    }

    #[test]
    fn invalid_action_and_amount_matrices_are_rejected() {
        let mut invalid = delegated_mint_args();
        invalid.settlement_action = 0xFF;
        assert_invalid(&invalid);

        invalid = delegated_mint_args();
        invalid.asset_amount = 0;
        assert_invalid(&invalid);

        invalid = delegated_mint_args();
        invalid.issued_token_amount = 1;
        assert_invalid(&invalid);

        invalid = delegated_mint_args();
        invalid.minimum_asset_amount = 1;
        assert_invalid(&invalid);

        let mut redeem = delegated_mint_args();
        redeem.settlement_action = settlement_action::REDEEM;
        redeem.asset_amount = 0;
        redeem.issued_token_amount = 1_000;
        redeem.minimum_asset_amount = 900;
        redeem.minimum_issued_token_amount = 0;
        assert!(assert_executable(&redeem, &key(4), 100).is_ok());

        redeem.asset_amount = 1;
        assert_invalid(&redeem);

        redeem = delegated_mint_args();
        redeem.settlement_action = settlement_action::REDEEM;
        redeem.asset_amount = 0;
        redeem.issued_token_amount = 0;
        redeem.minimum_issued_token_amount = 0;
        assert_invalid(&redeem);

        redeem = delegated_mint_args();
        redeem.settlement_action = settlement_action::REDEEM;
        redeem.asset_amount = 0;
        redeem.issued_token_amount = 1;
        redeem.minimum_issued_token_amount = 1;
        assert_invalid(&redeem);
    }

    #[test]
    fn invalid_identity_mode_and_time_combinations_are_rejected() {
        let mut invalid = delegated_mint_args();
        invalid.settlement_mode = settlement_mode::DIRECT_PRINCIPAL;
        assert_eq!(
            assert_executable(&invalid, &key(4), 100),
            Err(ChanceryError::SettlementModeNotAllowed.into()),
        );

        invalid = delegated_mint_args();
        invalid.asset_mint = invalid.issued_token_mint;
        assert_invalid(&invalid);

        invalid = delegated_mint_args();
        invalid.expires_at_unix_timestamp = 100;
        assert_eq!(
            assert_executable(&invalid, &key(4), 100),
            Err(ChanceryError::IntentExpired.into()),
        );

        invalid = delegated_mint_args();
        invalid.valid_after_unix_timestamp = 200;
        invalid.expires_at_unix_timestamp = 200;
        assert_invalid(&invalid);

        invalid = delegated_mint_args();
        invalid.valid_after_unix_timestamp = 201;
        invalid.expires_at_unix_timestamp = 200;
        assert_invalid(&invalid);
    }

    #[test]
    fn delegated_executor_must_be_distinct_from_principal() {
        let mut invalid = delegated_mint_args();
        invalid.executor = invalid.principal_a;
        assert_invalid(&invalid);
    }

    #[test]
    fn trilateral_role_identities_may_overlap() {
        let mut valid = delegated_mint_args();
        valid.settlement_mode = settlement_mode::TRILATERAL_ATOMIC;
        valid.principal_b = key(5);
        assert!(assert_executable(&valid, &key(4), 100).is_ok());

        let mut same_principals = valid;
        same_principals.principal_b = same_principals.principal_a;
        assert!(assert_executable(&same_principals, &key(4), 100).is_ok());

        let mut executor_is_principal = valid;
        executor_is_principal.executor = executor_is_principal.principal_a;
        assert!(assert_executable(&executor_is_principal, &key(4), 100).is_ok());
    }

    #[test]
    fn zero_expiry_requires_principal_a_to_pay_rent() {
        // Delegated mode: the previous guard was trilateral-only, so a
        // zero-expiry delegated intent funded by a third party stranded the
        // payer's rent behind principal-A-only cancellation (finding 102338).
        let mut args = delegated_mint_args();

        assert_eq!(
            assert_rent_recoverable(&args, &key(9)),
            Err(ChanceryError::IntentInvalidParameters.into()),
        );
        assert!(assert_rent_recoverable(&args, &args.principal_a).is_ok());

        args.settlement_mode = settlement_mode::TRILATERAL_ATOMIC;
        args.principal_b = key(5);

        assert_eq!(
            assert_rent_recoverable(&args, &key(9)),
            Err(ChanceryError::IntentInvalidParameters.into()),
        );
        assert!(assert_rent_recoverable(&args, &args.principal_a).is_ok());

        args.expires_at_unix_timestamp = 1_000;
        assert!(assert_rent_recoverable(&args, &key(9)).is_ok());
    }

    #[test]
    fn caller_nonce_is_not_a_global_sequence_dependency() {
        let domain_separator = [9u8; 32];
        let first = delegated_mint_args();
        let mut second = delegated_mint_args();
        second.asset_amount += 1;

        assert_eq!(first.nonce, second.nonce);
        assert_ne!(
            compute_intent_hash(&domain_separator, &first),
            compute_intent_hash(&domain_separator, &second),
        );
        assert_eq!(
            compute_intent_hash(&domain_separator, &first),
            compute_intent_hash(&domain_separator, &first),
        );
    }
}
