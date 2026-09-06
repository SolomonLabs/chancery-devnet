/// update_pathway_policy - selective field update on an existing PathwayPolicyPda.
///
/// Direct path only accepts adding forbidden collateral-extension bits
/// (RestrictiveImmediate) or a semantic no-op (RoutineOps). Every executor or
/// policy-reference rebind, and any forbidden-mask bit removal, is Widening and
/// must route through `handle_with_pending_change`.
///
/// Accounts (direct):
///   0  chancery_config                 writable  PDA  (sequence_nonce bump for evidence)
///   1  pathway_policy                  writable  PDA [b"pathway-policy", pathway_id]
///   2  operations_authority            signer    chancery_config.operations_authority
///   3  event_authority                 readable  PDA [b"event-authority"]
///   4  limit_policy                    optional
///   5  evidence_policy                 optional
///   6  fee_policy                      optional
///   7  insurance_policy                optional
///   8  asset_mint_limit_policy         optional
///   9  asset_redeem_limit_policy       optional
///  10  counterparty_limit_policy       optional
///  11  executor_limit_policy           optional
///
/// Accounts (with_pending_change):
///   0  chancery_config                 writable  PDA
///   1  event_authority                 readable  PDA [b"event-authority"]
///   2  pending_config_change           writable  PDA
///   3  pathway_policy                  writable  PDA
///   4  governance_authority            signer

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    constants::change_kind,
    error::ChanceryError,
    modules::{
        control::{
            change_detection::{
                classify_mask_change,
                MaskSemantics,
            },
            change_risk::{
                assert_direct_config_change_allowed,
                max_risk_many,
                ConfigChangeRiskClass,
            },
            pending_change::compute_config_change_hash,
        },
        core::state::chancery_config::ChanceryConfig,
        evidence::emit::{emit_pathway_policy_updated, PathwayPolicyUpdated},
        pathway::{
            reference_validation::{validate_pathway_policy_references, PathwayPolicyReferenceAccounts},
            state::pathway_policy::PathwayPolicy,
        },
    },
};

const CHANCERY_CONFIG:           usize = 0;
const PATHWAY_POLICY:            usize = 1;
const OPERATIONS_AUTHORITY:      usize = 2;
const EVENT_AUTHORITY_DIRECT:    usize = 3;
const LIMIT_POLICY:              usize = 4;
const EVIDENCE_POLICY:           usize = 5;
const FEE_POLICY:                usize = 6;
const INSURANCE_POLICY:          usize = 7;
const ASSET_MINT_LIMIT_POLICY:   usize = 8;
const ASSET_REDEEM_LIMIT_POLICY: usize = 9;
const COUNTERPARTY_LIMIT_POLICY: usize = 10;
const EXECUTOR_LIMIT_POLICY:     usize = 11;
const REQUIRED_ACCOUNT_COUNT:    usize = 12;

#[derive(BorshDeserialize)]
pub struct UpdatePathwayPolicyArgs {
    pub pathway_id:                          [u8; 32],  // identifies which policy
    pub designated_executor:                 Option<Pubkey>,
    pub limit_policy_id:                     Option<[u8; 32]>,
    pub evidence_policy_id:                  Option<[u8; 32]>,
    pub fee_policy_id:                       Option<[u8; 32]>,
    pub insurance_policy_id:                 Option<[u8; 32]>,
    pub forbidden_collateral_extension_mask: Option<[u64; 2]>,
    pub asset_mint_limit_policy_id:          Option<[u8; 32]>,
    pub asset_redeem_limit_policy_id:        Option<[u8; 32]>,
    pub counterparty_limit_policy_id:        Option<[u8; 32]>,
    pub executor_limit_policy_id:            Option<[u8; 32]>,
}

pub(super) fn apply_pathway_policy_update(p: &mut PathwayPolicy, args: &UpdatePathwayPolicyArgs) {
    if let Some(v) = args.designated_executor {
        p.designated_executor                   = v;
    }

    if let Some(v) = args.limit_policy_id {
        p.limit_policy_id                       = v;
    }

    if let Some(v) = args.evidence_policy_id {
        p.evidence_policy_id                    = v;
    }

    if let Some(v) = args.fee_policy_id {
        p.fee_policy_id                         = v;
    }

    if let Some(v) = args.insurance_policy_id {
        p.insurance_policy_id                   = v;
    }

    if let Some(v) = args.forbidden_collateral_extension_mask {
        p.forbidden_collateral_extension_mask   = v;
    }

    if let Some(v) = args.asset_mint_limit_policy_id {
        p.asset_mint_limit_policy_id            = v;
    }

    if let Some(v) = args.asset_redeem_limit_policy_id {
        p.asset_redeem_limit_policy_id          = v;
    }

    if let Some(v) = args.counterparty_limit_policy_id {
        p.counterparty_limit_policy_id          = v;
    }

    if let Some(v) = args.executor_limit_policy_id {
        p.executor_limit_policy_id              = v;
    }
}

fn push_widening_if_changed<T: PartialEq>(
    risks: &mut Vec<ConfigChangeRiskClass>,
    old: T,
    new_opt: Option<T>,
) {
    if let Some(new) = new_opt {
        if new != old {
            risks.push(ConfigChangeRiskClass::Widening);
        }
    }
}

pub(super) fn classify_pathway_policy_update(
    current: &PathwayPolicy,
    args: &UpdatePathwayPolicyArgs,
) -> ConfigChangeRiskClass {
    let mut risks: Vec<ConfigChangeRiskClass> = Vec::with_capacity(16);

    push_widening_if_changed(
        &mut risks,
        current.designated_executor,
        args.designated_executor,
    );
    push_widening_if_changed(&mut risks, current.limit_policy_id, args.limit_policy_id);
    push_widening_if_changed(
        &mut risks,
        current.evidence_policy_id,
        args.evidence_policy_id,
    );
    push_widening_if_changed(&mut risks, current.fee_policy_id, args.fee_policy_id);
    push_widening_if_changed(
        &mut risks,
        current.insurance_policy_id,
        args.insurance_policy_id,
    );
    push_widening_if_changed(
        &mut risks,
        current.asset_mint_limit_policy_id,
        args.asset_mint_limit_policy_id,
    );
    push_widening_if_changed(
        &mut risks,
        current.asset_redeem_limit_policy_id,
        args.asset_redeem_limit_policy_id,
    );
    push_widening_if_changed(
        &mut risks,
        current.counterparty_limit_policy_id,
        args.counterparty_limit_policy_id,
    );
    push_widening_if_changed(
        &mut risks,
        current.executor_limit_policy_id,
        args.executor_limit_policy_id,
    );

    if let Some(v) = args.forbidden_collateral_extension_mask {
        risks.push(classify_mask_change(
            current.forbidden_collateral_extension_mask,
            v,
            MaskSemantics::ForbiddenList,
        ));
    }

    if risks.is_empty() {
        ConfigChangeRiskClass::RoutineOps
    } else {
        max_risk_many(&risks)
    }
}

pub(super) fn pathway_policy_update_hashes(
    pathway_policy_key: &Pubkey,
    risk: ConfigChangeRiskClass,
    current: &PathwayPolicy,
    proposed: &PathwayPolicy,
) -> ([u8; 32], [u8; 32]) {
    let current_payload = current.config_change_payload();
    let proposed_payload = proposed.config_change_payload();
    let old_hash = compute_config_change_hash(
        change_kind::UPDATE_PATHWAY_POLICY,
        pathway_policy_key,
        risk.as_u8(),
        &current_payload,
    );

    let new_hash = compute_config_change_hash(
        change_kind::UPDATE_PATHWAY_POLICY,
        pathway_policy_key,
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
    let pathway_policy_account_info        = &accounts[PATHWAY_POLICY];
    let operations_authority_account_info  = &accounts[OPERATIONS_AUTHORITY];

    if !operations_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !pathway_policy_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;


    if operations_authority_account_info.key != &chancery_config.operations_authority {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    let args = UpdatePathwayPolicyArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    // Verify the PDA matches the pathway_id in args.
    let program_id        = crate::id();
    let (expected_key, policy_bump) = solana_pubkey::Pubkey::find_program_address(
        &[
            crate::constants::seeds::PATHWAY_POLICY,
            args.pathway_id.as_ref(),
        ],
        &program_id,
    );

    if pathway_policy_account_info.key != &expected_key {
        return Err(ChanceryError::InvalidPda.into());
    }

    let current = *PathwayPolicy::load_for_verified_pda(
        pathway_policy_account_info,
        &args.pathway_id,
        policy_bump,
    )?;
    let mut proposed = current;

    apply_pathway_policy_update(&mut proposed, &args);
    proposed.zero_reserved();
    proposed.assert_runtime_supported()?;
    let reference_accounts = PathwayPolicyReferenceAccounts {
        limit_policy:              &accounts[LIMIT_POLICY],
        evidence_policy:           &accounts[EVIDENCE_POLICY],
        fee_policy:                &accounts[FEE_POLICY],
        insurance_policy:          &accounts[INSURANCE_POLICY],
        asset_mint_limit_policy:   &accounts[ASSET_MINT_LIMIT_POLICY],
        asset_redeem_limit_policy: &accounts[ASSET_REDEEM_LIMIT_POLICY],
        counterparty_limit_policy: &accounts[COUNTERPARTY_LIMIT_POLICY],
        executor_limit_policy:     &accounts[EXECUTOR_LIMIT_POLICY],
    };
    validate_pathway_policy_references(
        pathway_policy_account_info.key,
        &proposed,
        &reference_accounts,
    )?;

    let risk         = classify_pathway_policy_update(&current, &args);

    assert_direct_config_change_allowed(risk, &chancery_config, operations_authority_account_info)?;

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (old_hash, new_hash) =
        pathway_policy_update_hashes(pathway_policy_account_info.key, risk, &current, &proposed);

    *PathwayPolicy::load_mut_for_verified_pda(
        pathway_policy_account_info,
        &args.pathway_id,
        policy_bump,
    )? = proposed;

    let clock                   = Clock::get()?;
    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    emit_pathway_policy_updated(
        &accounts[EVENT_AUTHORITY_DIRECT],
        event_authority_bump,
        PathwayPolicyUpdated {
            sequence_nonce,
            chancery:       *chancery_config_account_info.key,
            slot:           clock.slot,
            unix_timestamp: clock.unix_timestamp,
            risk_class:     risk.as_u8(),
            change_id:      [0u8; 32],
            pathway_policy: *pathway_policy_account_info.key,
            old_value_hash: old_hash,
            new_value_hash: new_hash,
            proposed_by:    *operations_authority_account_info.key,
            updated_by:     *operations_authority_account_info.key,
        },
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use bytemuck::Zeroable;

    use super::*;

    fn empty_update(pathway_id: [u8; 32]) -> UpdatePathwayPolicyArgs {
        UpdatePathwayPolicyArgs {
            pathway_id,
            designated_executor:                 None,
            limit_policy_id:                     None,
            evidence_policy_id:                  None,
            fee_policy_id:                       None,
            insurance_policy_id:                 None,
            forbidden_collateral_extension_mask: None,
            asset_mint_limit_policy_id:          None,
            asset_redeem_limit_policy_id:        None,
            counterparty_limit_policy_id:        None,
            executor_limit_policy_id:            None,
        }
    }

    fn assert_widening(current: &PathwayPolicy, args: UpdatePathwayPolicyArgs) {
        assert_eq!(
            classify_pathway_policy_update(current, &args),
            ConfigChangeRiskClass::Widening,
        );
    }

    #[test]
    fn apply_update_writes_every_argument_field_and_preserves_identity() {
        let mut policy = PathwayPolicy::zeroed();
        policy.pathway_id = [0x11; 32];
        policy.asset_mint = Pubkey::new_unique();
        policy.issued_token_mint = Pubkey::new_unique();
        let original_asset_mint = policy.asset_mint;
        let original_issued_token_mint = policy.issued_token_mint;

        let designated_executor = Pubkey::new_unique();
        let args = UpdatePathwayPolicyArgs {
            pathway_id:                          policy.pathway_id,
            designated_executor:                 Some(designated_executor),
            limit_policy_id:                     Some([1; 32]),
            evidence_policy_id:                  Some([2; 32]),
            fee_policy_id:                       Some([3; 32]),
            insurance_policy_id:                 Some([4; 32]),
            forbidden_collateral_extension_mask: Some([5, 6]),
            asset_mint_limit_policy_id:          Some([7; 32]),
            asset_redeem_limit_policy_id:        Some([8; 32]),
            counterparty_limit_policy_id:        Some([9; 32]),
            executor_limit_policy_id:            Some([10; 32]),
        };

        apply_pathway_policy_update(&mut policy, &args);

        assert_eq!(policy.designated_executor, designated_executor);
        assert_eq!(policy.limit_policy_id, [1; 32]);
        assert_eq!(policy.evidence_policy_id, [2; 32]);
        assert_eq!(policy.fee_policy_id, [3; 32]);
        assert_eq!(policy.insurance_policy_id, [4; 32]);
        assert_eq!(policy.forbidden_collateral_extension_mask, [5, 6]);
        assert_eq!(policy.asset_mint_limit_policy_id, [7; 32]);
        assert_eq!(policy.asset_redeem_limit_policy_id, [8; 32]);
        assert_eq!(policy.counterparty_limit_policy_id, [9; 32]);
        assert_eq!(policy.executor_limit_policy_id, [10; 32]);
        assert_eq!(policy.pathway_id, [0x11; 32]);
        assert_eq!(policy.asset_mint, original_asset_mint);
        assert_eq!(policy.issued_token_mint, original_issued_token_mint);
    }

    #[test]
    fn actual_instruction_classifier_accepts_none_and_same_values_as_routine() {
        let mut current                             = PathwayPolicy::zeroed();
        current.pathway_id                          = [0x22; 32];
        current.designated_executor                 = Pubkey::new_unique();
        current.limit_policy_id                     = [1; 32];
        current.evidence_policy_id                  = [2; 32];
        current.fee_policy_id                       = [3; 32];
        current.insurance_policy_id                 = [4; 32];
        current.forbidden_collateral_extension_mask = [5, 6];
        current.asset_mint_limit_policy_id          = [7; 32];
        current.asset_redeem_limit_policy_id        = [8; 32];
        current.counterparty_limit_policy_id        = [9; 32];
        current.executor_limit_policy_id            = [10; 32];

        assert_eq!(
            classify_pathway_policy_update(&current, &empty_update(current.pathway_id)),
            ConfigChangeRiskClass::RoutineOps,
        );

        let args = UpdatePathwayPolicyArgs {
            pathway_id:                          current.pathway_id,
            designated_executor:                 Some(current.designated_executor),
            limit_policy_id:                     Some(current.limit_policy_id),
            evidence_policy_id:                  Some(current.evidence_policy_id),
            fee_policy_id:                       Some(current.fee_policy_id),
            insurance_policy_id:                 Some(current.insurance_policy_id),
            forbidden_collateral_extension_mask: Some(current.forbidden_collateral_extension_mask),
            asset_mint_limit_policy_id:          Some(current.asset_mint_limit_policy_id),
            asset_redeem_limit_policy_id:        Some(current.asset_redeem_limit_policy_id),
            counterparty_limit_policy_id:        Some(current.counterparty_limit_policy_id),
            executor_limit_policy_id:            Some(current.executor_limit_policy_id),
        };
        assert_eq!(
            classify_pathway_policy_update(&current, &args),
            ConfigChangeRiskClass::RoutineOps,
        );
    }

    #[test]
    fn actual_instruction_classifier_marks_every_executor_and_reference_rebind_as_widening() {
        let mut current = PathwayPolicy::zeroed();
        current.pathway_id = [0x33; 32];

        let mut args = empty_update(current.pathway_id);
        args.designated_executor = Some(Pubkey::new_unique());
        assert_widening(&current, args);


        let mut args = empty_update(current.pathway_id);
        args.limit_policy_id = Some([1; 32]);
        assert_widening(&current, args);

        let mut args = empty_update(current.pathway_id);
        args.evidence_policy_id = Some([2; 32]);
        assert_widening(&current, args);

        let mut args = empty_update(current.pathway_id);
        args.fee_policy_id = Some([3; 32]);
        assert_widening(&current, args);

        let mut args = empty_update(current.pathway_id);
        args.insurance_policy_id = Some([4; 32]);
        assert_widening(&current, args);

        let mut args = empty_update(current.pathway_id);
        args.asset_mint_limit_policy_id = Some([5; 32]);
        assert_widening(&current, args);

        let mut args = empty_update(current.pathway_id);
        args.asset_redeem_limit_policy_id = Some([6; 32]);
        assert_widening(&current, args);

        let mut args = empty_update(current.pathway_id);
        args.counterparty_limit_policy_id = Some([7; 32]);
        assert_widening(&current, args);

        let mut args = empty_update(current.pathway_id);
        args.executor_limit_policy_id = Some([8; 32]);
        assert_widening(&current, args);
    }

    #[test]
    fn forbidden_collateral_mask_additions_are_restrictive_and_removals_are_widening() {
        let mut current = PathwayPolicy::zeroed();
        current.pathway_id = [0x44; 32];

        let mut add = empty_update(current.pathway_id);
        add.forbidden_collateral_extension_mask = Some([1, 1]);
        assert_eq!(
            classify_pathway_policy_update(&current, &add),
            ConfigChangeRiskClass::RestrictiveImmediate,
        );

        current.forbidden_collateral_extension_mask = [3, 3];
        let mut remove = empty_update(current.pathway_id);
        remove.forbidden_collateral_extension_mask = Some([1, 1]);
        assert_eq!(
            classify_pathway_policy_update(&current, &remove),
            ConfigChangeRiskClass::Widening,
        );

        let mut mixed = empty_update(current.pathway_id);
        mixed.forbidden_collateral_extension_mask = Some([5, 1]);
        assert_eq!(
            classify_pathway_policy_update(&current, &mixed),
            ConfigChangeRiskClass::Widening,
        );
    }

    #[test]
    fn widening_fields_dominate_a_simultaneous_restrictive_mask_addition() {
        let mut current = PathwayPolicy::zeroed();
        current.pathway_id = [0x55; 32];
        let mut args = empty_update(current.pathway_id);
        args.designated_executor = Some(Pubkey::new_unique());
        args.forbidden_collateral_extension_mask = Some([1, 0]);

        assert_eq!(
            classify_pathway_policy_update(&current, &args),
            ConfigChangeRiskClass::Widening,
        );
    }

    #[test]
    fn reserved_tail_is_excluded_from_pathway_value_hashes() {
        let key                  = Pubkey::new_unique();
        let mut current          = PathwayPolicy::zeroed();
        let mut proposed         = current;

        current._reserved        = [0xAA; 64];
        proposed._reserved       = [0x55; 64];

        let (old_hash, new_hash) = pathway_policy_update_hashes(
            &key,
            ConfigChangeRiskClass::RoutineOps,
            &current,
            &proposed,
        );

        assert_eq!(old_hash, new_hash);
    }

    #[test]
    fn pathway_pause_does_not_invalidate_policy_update_hashes() {
        let key = Pubkey::new_unique();
        let mut current = PathwayPolicy::zeroed();
        current.status_flags = crate::constants::status_flag::INITIALIZED;
        let mut paused = current;
        paused.status_flags |= crate::constants::status_flag::PATHWAY_PAUSE;

        let (old_hash, paused_hash) = pathway_policy_update_hashes(
            &key,
            ConfigChangeRiskClass::RoutineOps,
            &current,
            &paused,
        );

        assert_eq!(old_hash, paused_hash);
    }

    #[test]
    fn pathway_hashes_bind_target_risk_and_semantic_value() {
        let key = Pubkey::new_unique();
        let current = PathwayPolicy::zeroed();
        let mut proposed = current;
        proposed.forbidden_collateral_extension_mask = [1, 0];

        let (old_hash, new_hash) = pathway_policy_update_hashes(
            &key,
            ConfigChangeRiskClass::RestrictiveImmediate,
            &current,
            &proposed,
        );
        assert_ne!(old_hash, new_hash);

        let (other_target_old, _) = pathway_policy_update_hashes(
            &Pubkey::new_unique(),
            ConfigChangeRiskClass::RestrictiveImmediate,
            &current,
            &proposed,
        );
        assert_ne!(old_hash, other_target_old);

        let (other_risk_old, _) = pathway_policy_update_hashes(
            &key,
            ConfigChangeRiskClass::Widening,
            &current,
            &proposed,
        );
        assert_ne!(old_hash, other_risk_old);
    }
}
