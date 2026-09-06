/// update_fee_policy
///
/// Direct path only accepts RoutineOps (no-op) changes; any field
/// mutation classifies as Widening and must route through
/// `handle_with_pending_change`.
///
/// Accounts (direct):
///   0  chancery_config         writable  PDA  (sequence_nonce bump for evidence)
///   1  fee_policy              writable  PDA
///   2  operations_authority    signer
///   3  event_authority         readable  PDA [b"event-authority"]
///
/// Accounts (with_pending_change):
///   0  chancery_config         writable  PDA
///   1  event_authority         readable  PDA [b"event-authority"]
///   2  pending_config_change   writable  PDA
///   3  fee_policy              writable  PDA
///   4  governance_authority    signer

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
        control::change_risk::{
            assert_direct_config_change_allowed,
            ConfigChangeRiskClass,
        },
        control::pending_change::compute_config_change_hash,
        core::state::chancery_config::ChanceryConfig,
        evidence::emit::{emit_fee_policy_updated, FeePolicyUpdated},
        fees::state::fee_policy::{fee_flag, FeePolicy},
    },
};

const CHANCERY_CONFIG:                usize = 0;
const FEE_POLICY:                     usize = 1;
const OPERATIONS_AUTHORITY:           usize = 2;
const EVENT_AUTHORITY_DIRECT:         usize = 3;
const REQUIRED_ACCOUNT_COUNT:         usize = 4;

#[derive(BorshDeserialize)]
pub struct UpdateFeePolicyArgs {
    pub fee_policy_id:                  [u8; 32],
    pub fee_policy_flags:               Option<u64>,
    pub flat_fee_in_asset:              Option<u64>,
    pub flat_fee_in_issued_token:       Option<u64>,
    pub percent_fee_bps:                Option<u32>,
    pub fee_cap_amount:                 Option<u64>,
    pub minimum_fee_amount:             Option<u64>,
    pub rebate_flat_amount:             Option<u64>,
    pub rebate_bps:                     Option<u32>,
    pub rebate_cap_amount:              Option<u64>,
    pub net_fee_floor_zero:             Option<bool>,
    pub fee_recipient_key:              Option<Pubkey>,
    pub effective_from_unix_timestamp:  Option<i64>,
    pub effective_until_unix_timestamp: Option<i64>,
}

const FEE_DENOMINATION_MASK: u64 =
    fee_flag::FEE_IN_ASSET | fee_flag::FEE_IN_ISSUED_TOKEN;

pub(super) fn assert_fee_policy_denomination_unchanged(
    current:  &FeePolicy,
    proposed: &FeePolicy,
) -> ProgramResult {
    if (current.fee_policy_flags & FEE_DENOMINATION_MASK)
        != (proposed.fee_policy_flags & FEE_DENOMINATION_MASK)
    {
        return Err(ChanceryError::FeePolicyDenominationMismatch.into());
    }

    Ok(())
}

pub(super) fn apply_fee_policy_update(p: &mut FeePolicy, args: &UpdateFeePolicyArgs) {
    if let Some(v) = args.fee_policy_flags {
        p.fee_policy_flags                 = v;
    }

    if let Some(v) = args.flat_fee_in_asset {
        p.flat_fee_in_asset                = v;
    }

    if let Some(v) = args.flat_fee_in_issued_token {
        p.flat_fee_in_issued_token         = v;
    }

    if let Some(v) = args.percent_fee_bps {
        p.percent_fee_bps                  = v;
    }

    if let Some(v) = args.fee_cap_amount {
        p.fee_cap_amount                   = v;
    }

    if let Some(v) = args.minimum_fee_amount {
        p.minimum_fee_amount               = v;
    }

    if let Some(v) = args.rebate_flat_amount {
        p.rebate_flat_amount               = v;
    }

    if let Some(v) = args.rebate_bps {
        p.rebate_bps                       = v;
    }

    if let Some(v) = args.rebate_cap_amount {
        p.rebate_cap_amount                = v;
    }

    if let Some(v) = args.net_fee_floor_zero {
        p.net_fee_floor_zero               = v as u8;
    }

    if let Some(v) = args.fee_recipient_key {
        p.fee_recipient_key                = v;
    }

    if let Some(v) = args.effective_from_unix_timestamp {
        p.effective_from_unix_timestamp    = v;
    }

    if let Some(v) = args.effective_until_unix_timestamp {
        p.effective_until_unix_timestamp = v;
    }
}

#[inline]
pub(super) fn classify_fee_policy_update(current: &FeePolicy, proposed: &FeePolicy) -> ConfigChangeRiskClass {
    // 102051 reviewed and declined: every non-identical fee-policy update
    // (including recipient redirection) deliberately remains Widening -
    // governance signature plus the Widening timelock floor. Identity is
    // compared over the canonical semantic payload, not raw account bytes.
    if current.config_change_payload() == proposed.config_change_payload() {
        ConfigChangeRiskClass::RoutineOps
    } else {
        ConfigChangeRiskClass::Widening
    }
}

pub(super) fn fee_policy_update_hashes(
    fee_policy_key: &Pubkey,
    risk: ConfigChangeRiskClass,
    current: &FeePolicy,
    proposed: &FeePolicy,
) -> ([u8; 32], [u8; 32]) {
    let current_payload = current.config_change_payload();
    let proposed_payload = proposed.config_change_payload();
    let old_hash = compute_config_change_hash(
        change_kind::UPDATE_FEE_POLICY,
        fee_policy_key,
        risk.as_u8(),
        &current_payload,
    );

    let new_hash = compute_config_change_hash(
        change_kind::UPDATE_FEE_POLICY,
        fee_policy_key,
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
    let fee_policy_account_info            = &accounts[FEE_POLICY];
    let operations_authority_account_info  = &accounts[OPERATIONS_AUTHORITY];

    if !operations_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !fee_policy_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;


    if operations_authority_account_info.key != &chancery_config.operations_authority {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    let args = UpdateFeePolicyArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let (expected_key, policy_bump) = solana_pubkey::Pubkey::find_program_address(
        &[
            crate::constants::seeds::FEE_POLICY,
            args.fee_policy_id.as_ref(),
        ],
        &crate::id(),
    );

    if fee_policy_account_info.key != &expected_key {
        return Err(ChanceryError::InvalidPda.into());
    }

    let current = *FeePolicy::load_for_verified_pda(
        fee_policy_account_info,
        &args.fee_policy_id,
        policy_bump,
    )?;
    let mut proposed = current;

    apply_fee_policy_update(&mut proposed, &args);

    proposed.assert_parameter_sanity()?;
    proposed.assert_denomination_consistent()?;
    assert_fee_policy_denomination_unchanged(&current, &proposed)?;

    let risk         = classify_fee_policy_update(&current, &proposed);

    assert_direct_config_change_allowed(risk, &chancery_config, operations_authority_account_info)?;

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (old_hash, new_hash) =
        fee_policy_update_hashes(fee_policy_account_info.key, risk, &current, &proposed);

    *FeePolicy::load_mut_for_verified_pda(
        fee_policy_account_info,
        &args.fee_policy_id,
        policy_bump,
    )? = proposed;

    let clock                   = Clock::get()?;
    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    emit_fee_policy_updated(
        &accounts[EVENT_AUTHORITY_DIRECT],
        event_authority_bump,
        FeePolicyUpdated {
            sequence_nonce,
            chancery:       *chancery_config_account_info.key,
            slot:           clock.slot,
            unix_timestamp: clock.unix_timestamp,
            risk_class:     risk.as_u8(),
            change_id:      [0u8; 32],
            fee_policy:     *fee_policy_account_info.key,
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
    use solana_pubkey::Pubkey;

    use super::*;
    use crate::modules::fees::state::fee_policy::{
        fee_flag, FEE_POLICY_DISCRIMINATOR,
    };

    fn valid_fee_policy() -> FeePolicy {
        let mut policy = FeePolicy::zeroed();
        policy.discriminator = FEE_POLICY_DISCRIMINATOR;
        policy.version = 1;
        policy.bump = 255;
        policy.fee_policy_id = [0x11; 32];
        policy.fee_policy_flags = fee_flag::FEE_IN_ISSUED_TOKEN;
        policy.net_fee_floor_zero = 1;
        policy
    }

    fn empty_update() -> UpdateFeePolicyArgs {
        UpdateFeePolicyArgs {
            fee_policy_id:                  [0x11; 32],
            fee_policy_flags:               None,
            flat_fee_in_asset:              None,
            flat_fee_in_issued_token:       None,
            percent_fee_bps:                None,
            fee_cap_amount:                 None,
            minimum_fee_amount:             None,
            rebate_flat_amount:             None,
            rebate_bps:                     None,
            rebate_cap_amount:              None,
            net_fee_floor_zero:             None,
            fee_recipient_key:              None,
            effective_from_unix_timestamp:  None,
            effective_until_unix_timestamp: None,
        }
    }

    fn assert_widening(current: &FeePolicy, mutate: impl FnOnce(&mut FeePolicy)) {
        let mut proposed = *current;
        mutate(&mut proposed);
        assert_eq!(
            classify_fee_policy_update(current, &proposed),
            ConfigChangeRiskClass::Widening,
        );
    }

    #[test]
    fn apply_update_writes_every_mutable_field_and_preserves_identity_and_layout_fields() {
        let current = valid_fee_policy();
        let recipient = Pubkey::new_unique();
        let args = UpdateFeePolicyArgs {
            fee_policy_id:                  current.fee_policy_id,
            fee_policy_flags:               Some(fee_flag::FEE_IN_ASSET),
            flat_fee_in_asset:              Some(1),
            flat_fee_in_issued_token:       Some(2),
            percent_fee_bps:                Some(3),
            fee_cap_amount:                 Some(4),
            minimum_fee_amount:             Some(5),
            rebate_flat_amount:             Some(6),
            rebate_bps:                     Some(7),
            rebate_cap_amount:              Some(8),
            net_fee_floor_zero:             Some(false),
            fee_recipient_key:              Some(recipient),
            effective_from_unix_timestamp:  Some(9),
            effective_until_unix_timestamp: Some(10),
        };
        let mut proposed = current;

        apply_fee_policy_update(&mut proposed, &args);

        assert_eq!(proposed.fee_policy_flags, fee_flag::FEE_IN_ASSET);
        assert_eq!(proposed.flat_fee_in_asset, 1);
        assert_eq!(proposed.flat_fee_in_issued_token, 2);
        assert_eq!(proposed.percent_fee_bps, 3);
        assert_eq!(proposed.fee_cap_amount, 4);
        assert_eq!(proposed.minimum_fee_amount, 5);
        assert_eq!(proposed.rebate_flat_amount, 6);
        assert_eq!(proposed.rebate_bps, 7);
        assert_eq!(proposed.rebate_cap_amount, 8);
        assert_eq!(proposed.net_fee_floor_zero, 0);
        assert_eq!(proposed.fee_recipient_key, recipient);
        assert_eq!(proposed.effective_from_unix_timestamp, 9);
        assert_eq!(proposed.effective_until_unix_timestamp, 10);

        assert_eq!(proposed.discriminator, current.discriminator);
        assert_eq!(proposed.version, current.version);
        assert_eq!(proposed.bump, current.bump);
        assert_eq!(proposed.fee_recipient_policy, current.fee_recipient_policy);
        assert_eq!(proposed.rounding_mode, current.rounding_mode);
        assert_eq!(proposed._pad0, current._pad0);
        assert_eq!(proposed.fee_policy_id, current.fee_policy_id);
        assert_eq!(proposed._pad1, current._pad1);
        assert_eq!(proposed._pad2, current._pad2);
        assert_eq!(proposed._reserved, current._reserved);
    }

    #[test]
    fn denomination_is_immutable_after_registration() {
        let current = valid_fee_policy();
        let mut proposed = current;
        proposed.fee_policy_flags &= !fee_flag::FEE_IN_ISSUED_TOKEN;
        proposed.fee_policy_flags |= fee_flag::FEE_IN_ASSET;

        assert_eq!(
            assert_fee_policy_denomination_unchanged(&current, &proposed),
            Err(ChanceryError::FeePolicyDenominationMismatch.into()),
        );

        proposed = current;
        proposed.fee_policy_flags ^= fee_flag::ACTIVE;
        assert!(assert_fee_policy_denomination_unchanged(&current, &proposed).is_ok());
    }

    #[test]
    fn empty_update_is_an_exact_semantic_no_op() {
        let current = valid_fee_policy();
        let mut proposed = current;

        apply_fee_policy_update(&mut proposed, &empty_update());

        assert_eq!(proposed.config_change_payload(), current.config_change_payload());
        assert_eq!(
            classify_fee_policy_update(&current, &proposed),
            ConfigChangeRiskClass::RoutineOps,
        );
    }

    #[test]
    fn every_mutable_field_change_is_widening() {
        let current = valid_fee_policy();

        assert_widening(&current, |p| p.fee_policy_flags ^= fee_flag::ACTIVE);
        assert_widening(&current, |p| p.flat_fee_in_asset = 1);
        assert_widening(&current, |p| p.flat_fee_in_issued_token = 1);
        assert_widening(&current, |p| p.percent_fee_bps = 1);
        assert_widening(&current, |p| p.fee_cap_amount = 1);
        assert_widening(&current, |p| p.minimum_fee_amount = 1);
        assert_widening(&current, |p| p.rebate_flat_amount = 1);
        assert_widening(&current, |p| p.rebate_bps = 1);
        assert_widening(&current, |p| p.rebate_cap_amount = 1);
        assert_widening(&current, |p| p.net_fee_floor_zero = 0);
        assert_widening(&current, |p| p.fee_recipient_key = Pubkey::new_unique());
        assert_widening(&current, |p| p.effective_from_unix_timestamp = 1);
        assert_widening(&current, |p| p.effective_until_unix_timestamp = 2);
    }

    #[test]
    fn account_headers_padding_and_reserved_tail_are_excluded_from_semantic_identity() {
        let current = valid_fee_policy();
        let mut proposed = current;

        proposed.discriminator = [0xAA; 8];
        proposed.version = 99;
        proposed.bump = 7;
        proposed._pad0 = [0xBB; 2];
        proposed._pad1 = [0xCC; 4];
        proposed._pad2 = [0xDD; 4];
        proposed._reserved = [0xEE; 32];

        assert_eq!(current.config_change_payload(), proposed.config_change_payload());
        assert_eq!(
            classify_fee_policy_update(&current, &proposed),
            ConfigChangeRiskClass::RoutineOps,
        );
    }

    #[test]
    fn fee_policy_hashes_bind_semantic_value_target_and_risk() {
        let key = Pubkey::new_unique();
        let current = valid_fee_policy();
        let mut proposed = current;

        let (no_op_old, no_op_new) = fee_policy_update_hashes(
            &key,
            ConfigChangeRiskClass::RoutineOps,
            &current,
            &proposed,
        );
        assert_eq!(no_op_old, no_op_new);

        proposed.percent_fee_bps = 1;
        let (old_hash, new_hash) = fee_policy_update_hashes(
            &key,
            ConfigChangeRiskClass::Widening,
            &current,
            &proposed,
        );
        assert_ne!(old_hash, new_hash);

        let (other_target_old, _) = fee_policy_update_hashes(
            &Pubkey::new_unique(),
            ConfigChangeRiskClass::Widening,
            &current,
            &proposed,
        );
        assert_ne!(old_hash, other_target_old);

        let (other_risk_old, _) = fee_policy_update_hashes(
            &key,
            ConfigChangeRiskClass::HighImpact,
            &current,
            &proposed,
        );
        assert_ne!(old_hash, other_risk_old);
    }
}
