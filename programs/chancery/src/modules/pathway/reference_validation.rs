use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    account_security::assert_distinct_present_accounts,
    constants::{pathway_kind, scope},
    error::ChanceryError,
    modules::{
        evidence::state::evidence_policy::EvidencePolicy,
        fees::state::fee_policy::FeePolicy,
        limits::{
            dimension::load_dimension_policy,
            state::limit_policy::LimitPolicy,
        },
        pathway::state::pathway_policy::PathwayPolicy,
    },
};

/// Fixed account family used by every pathway registration/update instruction.
/// Disabled references still occupy their slot with the default public key, so
/// account order never depends on the proposed policy contents.
pub struct PathwayPolicyReferenceAccounts<'a> {
    pub limit_policy:               &'a AccountInfo<'a>,
    pub evidence_policy:            &'a AccountInfo<'a>,
    pub fee_policy:                 &'a AccountInfo<'a>,
    pub insurance_policy:           &'a AccountInfo<'a>,
    pub asset_mint_limit_policy:    &'a AccountInfo<'a>,
    pub asset_redeem_limit_policy:  &'a AccountInfo<'a>,
    pub counterparty_limit_policy:  &'a AccountInfo<'a>,
    pub executor_limit_policy:      &'a AccountInfo<'a>,
}

fn required_reference<'a>(
    account: &'a AccountInfo<'a>,
) -> Result<&'a AccountInfo<'a>, ProgramError> {
    if account.key == &Pubkey::default() {
        return Err(ChanceryError::PathwayDependencyMissing.into());
    }
    Ok(account)
}

fn assert_reference_presence(
    reference_id: &[u8; 32],
    account:      &AccountInfo,
) -> ProgramResult {
    if reference_id == &[0u8; 32] {
        if account.key != &Pubkey::default() {
            return Err(ChanceryError::AccountKeyMismatch.into());
        }
    } else if account.key == &Pubkey::default() {
        return Err(ChanceryError::PathwayDependencyMissing.into());
    }
    Ok(())
}

pub fn validate_pathway_policy_references<'a>(
    pathway_policy_account: &Pubkey,
    policy: &PathwayPolicy,
    accounts: &PathwayPolicyReferenceAccounts<'a>,
) -> ProgramResult {
    let program_id = crate::id();

    assert_distinct_present_accounts(&[
        accounts.limit_policy,
        accounts.evidence_policy,
        accounts.fee_policy,
        accounts.insurance_policy,
        accounts.asset_mint_limit_policy,
        accounts.asset_redeem_limit_policy,
        accounts.counterparty_limit_policy,
        accounts.executor_limit_policy,
    ])?;

    assert_reference_presence(&policy.limit_policy_id, accounts.limit_policy)?;
    assert_reference_presence(&policy.evidence_policy_id, accounts.evidence_policy)?;
    assert_reference_presence(&policy.fee_policy_id, accounts.fee_policy)?;
    assert_reference_presence(&policy.insurance_policy_id, accounts.insurance_policy)?;
    assert_reference_presence(
        &policy.asset_mint_limit_policy_id,
        accounts.asset_mint_limit_policy,
    )?;
    assert_reference_presence(
        &policy.asset_redeem_limit_policy_id,
        accounts.asset_redeem_limit_policy,
    )?;
    assert_reference_presence(
        &policy.counterparty_limit_policy_id,
        accounts.counterparty_limit_policy,
    )?;
    assert_reference_presence(
        &policy.executor_limit_policy_id,
        accounts.executor_limit_policy,
    )?;

    if policy.has_limit_policy() {
        let account = required_reference(accounts.limit_policy)?;
        let expected_bump =
            LimitPolicy::verify_pda(account, &policy.limit_policy_id, &program_id)?;
        let limit_policy = LimitPolicy::load_for_verified_pda(
            account,
            &policy.limit_policy_id,
            expected_bump,
        )?;
        if limit_policy.scope_kind != scope::PATHWAY
            || &limit_policy.scope_key != pathway_policy_account
        {
            return Err(ChanceryError::LimitPolicyScopeMismatch.into());
        }
    }

    if policy.has_evidence_policy() {
        let account = required_reference(accounts.evidence_policy)?;
        let expected_bump =
            EvidencePolicy::verify_pda(account, &policy.evidence_policy_id, &program_id)?;
        let evidence_policy = EvidencePolicy::load_for_verified_pda(
            account,
            &policy.evidence_policy_id,
            expected_bump,
        )?;
        evidence_policy.assert_runtime_supported()?;
    }

    if policy.has_fee_policy() {
        let account = required_reference(accounts.fee_policy)?;
        let expected_bump =
            FeePolicy::verify_pda(account, &policy.fee_policy_id, &program_id)?;
        let fee_policy = FeePolicy::load_for_verified_pda(
            account,
            &policy.fee_policy_id,
            expected_bump,
        )?;
        fee_policy.assert_parameter_sanity()?;

        match policy.pathway_kind {
            pathway_kind::DIRECT
            | pathway_kind::DELEGATED
            | pathway_kind::TRILATERAL => {
                // A local pathway stores one fee-policy reference. The policy
                // itself has exactly one denomination, so a fee-bearing pathway
                // is intentionally executable in only the matching direction:
                // issued-token denomination for mint, asset denomination for
                // redeem. A pathway with no fee policy may execute both.
            }
            pathway_kind::CROSS_CHAIN_MINT => {
                // Inbound mint currently has no fee accounts or transfer path.
                return Err(ChanceryError::PathwayDependencyMissing.into());
            }
            pathway_kind::CROSS_CHAIN_REDEEM => {
                if !fee_policy.fee_in_issued_token() {
                    return Err(ChanceryError::FeePolicyDenominationMismatch.into());
                }
            }
            _ => return Err(ChanceryError::PathwayKindMismatch.into()),
        }
    }

    if policy.insurance_policy_id != [0u8; 32] {
        // Insurance is excluded from the active module registry and generated
        // runtime surface. `PathwayPolicy::assert_runtime_supported` rejects
        // this earlier; keep this defensive guard without compiling inactive
        // insurance state into the production program.
        return Err(ChanceryError::ModuleNotEnabled.into());
    }

    if policy.has_asset_mint_limit_policy() {
        let account = required_reference(accounts.asset_mint_limit_policy)?;
        load_dimension_policy(
            account,
            &policy.asset_mint_limit_policy_id,
            scope::ASSET,
            &policy.asset_mint,
            &program_id,
        )?;
    }

    if policy.has_asset_redeem_limit_policy() {
        let account = required_reference(accounts.asset_redeem_limit_policy)?;
        load_dimension_policy(
            account,
            &policy.asset_redeem_limit_policy_id,
            scope::ASSET,
            &policy.asset_mint,
            &program_id,
        )?;
    }

    if policy.has_counterparty_limit_policy() {
        let account = required_reference(accounts.counterparty_limit_policy)?;
        let counterparty_policy = load_dimension_policy(
            account,
            &policy.counterparty_limit_policy_id,
            scope::COUNTERPARTY,
            &Pubkey::default(),
            &program_id,
        )?;

        if policy.pathway_kind == pathway_kind::DELEGATED {
            counterparty_policy.assert_required_daily_dimension_cap()?;
        }
    }

    if policy.has_executor_limit_policy() {
        let account = required_reference(accounts.executor_limit_policy)?;
        load_dimension_policy(
            account,
            &policy.executor_limit_policy_id,
            scope::EXECUTOR,
            &Pubkey::default(),
            &program_id,
        )?;
    }

    Ok(())
}
