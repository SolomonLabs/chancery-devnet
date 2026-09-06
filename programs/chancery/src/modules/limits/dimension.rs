//! Dimension limit enforcement (spec §5.8).
//!
//! The pathway-scoped `LimitPolicy` + full window family covers the pathway
//! dimension (per-tx, hourly/daily/weekly/monthly volume, action counts).
//! The mandatory asset / counterparty / executor dimensions are enforced here
//! as per-transaction + fixed UTC-day caps against dimension-scoped policies
//! referenced from `PathwayPolicy` state:
//!
//!   - `asset_mint_limit_policy_id`   scope ASSET,        scope_key = asset_mint
//!   - `asset_redeem_limit_policy_id` scope ASSET,        scope_key = asset_mint
//!   - `counterparty_limit_policy_id` scope COUNTERPARTY, scope_key = default (template)
//!   - `executor_limit_policy_id`     scope EXECUTOR,     scope_key = default (template)
//!
//! The handler - never the caller - derives which dimensions are enforced
//! (from pathway state), which policy PDA is expected (from the referenced
//! id), and which usage-window PDA is expected (from the dimension scope kind
//! and the concrete party key). A caller can therefore neither skip an
//! enforced dimension nor substitute a foreign policy or window.
//!
//! Template dimensions (counterparty / executor) share one policy account but
//! accrue per-party: the window scope hash is computed over the concrete
//! counterparty / executor key, so every party gets an independent fixed UTC-day
//! accumulator under the same cap.
//!
//! All dimension volumes are asset-denominated: mint paths record/check the
//! deposited collateral gross (`gross_in`), redeem paths the gross asset out
//! (`gross_output_amount`). Caps apply per direction.

use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;
use solana_sha256_hasher::hashv;

use crate::{
    constants::{scope, window_kind},
    error::ChanceryError,
    modules::limits::state::{
        limit_policy::LimitPolicy,
        usage_window::{
            prepare_enforced_window_for_recording, ClosedPeriod, PreparedWindowProof,
        },
    },
};

/// sha256(scope_kind || party_key) - the usage-window seed hash for a
/// dimension. For ASSET this equals `LimitPolicy::scope_hash()` of the asset
/// policy; for template dimensions it is keyed by the concrete party.
pub fn dimension_window_scope_hash(scope_kind: u8, party_key: &Pubkey) -> [u8; 32] {
    hashv(&[&[scope_kind], party_key.as_ref()]).to_bytes()
}

/// Load and bind a dimension policy account referenced from pathway state.
///
/// The caller gates on the pathway reference (`has_*_limit_policy`) and on
/// account presence, mirroring the fee/limit optional-account pattern; this
/// helper then verifies the PDA derives from the referenced id, binds the
/// policy scope to the expected dimension, and rejects caps that dimension
/// enforcement cannot honor (`assert_dimension_scope_caps`) - nothing may be
/// stored-but-unenforced.
///
/// `expected_scope_key` is the asset mint for ASSET, `Pubkey::default()` for
/// the COUNTERPARTY / EXECUTOR templates.
pub fn load_dimension_policy<'a>(
    policy_account_info:  &'a AccountInfo<'a>,
    referenced_policy_id: &[u8; 32],
    expected_scope_kind:  u8,
    expected_scope_key:   &Pubkey,
    program_id:           &Pubkey,
) -> Result<LimitPolicy, ProgramError> {
    let expected_bump =
        LimitPolicy::verify_pda(policy_account_info, referenced_policy_id, program_id)?;
    let policy = *LimitPolicy::load_for_verified_pda(
        policy_account_info,
        referenced_policy_id,
        expected_bump,
    )?;

    if policy.scope_kind != expected_scope_kind {
        return Err(ChanceryError::LimitPolicyScopeMismatch.into());
    }

    if &policy.scope_key != expected_scope_key {
        return Err(ChanceryError::LimitPolicyScopeMismatch.into());
    }

    policy.assert_dimension_scope_caps()?;

    Ok(policy)
}

/// Prepare and check one dimension daily window: require it present +
/// writable, verify the stable window PDA for the concrete party, roll it
/// forward to the canonical current UTC day (appending roll evidence to
/// `rolled`), and check the direction-appropriate fixed-window total against
/// the policy's daily cap. Only called when `policy.per_day_maximum != 0`.
pub fn prepare_dimension_window<'a>(
    accounts:       &'a [AccountInfo<'a>],
    index:          usize,
    policy:         &LimitPolicy,
    scope_kind:     u8,
    party_key:      &Pubkey,
    amount:         u64,
    is_inflow:      bool,
    unix_timestamp: i64,
    program_id:     &Pubkey,
    rolled:         &mut Vec<ClosedPeriod>,
) -> Result<(), ProgramError> {
    prepare_dimension_window_for_recording(
        accounts,
        index,
        policy,
        scope_kind,
        party_key,
        amount,
        is_inflow,
        unix_timestamp,
        program_id,
        rolled,
    )?;

    Ok(())
}

pub(crate) fn prepare_dimension_window_for_recording<'a>(
    accounts:       &'a [AccountInfo<'a>],
    index:          usize,
    policy:         &LimitPolicy,
    scope_kind:     u8,
    party_key:      &Pubkey,
    amount:         u64,
    is_inflow:      bool,
    unix_timestamp: i64,
    program_id:     &Pubkey,
    rolled:         &mut Vec<ClosedPeriod>,
) -> Result<PreparedWindowProof, ProgramError> {
    let scope_hash = dimension_window_scope_hash(scope_kind, party_key);

    let prepared = prepare_enforced_window_for_recording(
        accounts,
        index,
        &scope_hash,
        window_kind::DAILY,
        unix_timestamp,
        program_id,
        rolled,
    )?;

    let current_gross = if is_inflow {
        prepared.gross_in
    } else {
        prepared.gross_out
    };

    policy.assert_window_volume(current_gross, amount, ChanceryError::DailyLimitBreached)?;

    Ok(prepared)
}

/// Convenience: the expected scope kind / scope key binding for each pathway
/// dimension reference.
pub fn asset_dimension_binding(asset_mint: &Pubkey) -> (u8, Pubkey) {
    (scope::ASSET, *asset_mint)
}

pub fn counterparty_dimension_binding() -> (u8, Pubkey) {
    (scope::COUNTERPARTY, Pubkey::default())
}

pub fn executor_dimension_binding() -> (u8, Pubkey) {
    (scope::EXECUTOR, Pubkey::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_hash_matches_policy_scope_hash_for_asset_dimension() {
        use bytemuck::Zeroable;

        let asset_mint    = Pubkey::new_from_array([7u8; 32]);
        let mut policy    = LimitPolicy::zeroed();
        policy.scope_kind = scope::ASSET;
        policy.scope_key  = asset_mint;

        // The asset dimension window is shared by mint and redeem policies:
        // both bind scope_key = asset_mint, so the party-keyed hash and the
        // policy scope hash must agree.
        assert_eq!(
            dimension_window_scope_hash(scope::ASSET, &asset_mint),
            policy.scope_hash(),
        );
    }

    #[test]
    fn template_dimensions_key_windows_per_party() {
        let party_a = Pubkey::new_from_array([1u8; 32]);
        let party_b = Pubkey::new_from_array([2u8; 32]);

        let hash_a = dimension_window_scope_hash(scope::COUNTERPARTY, &party_a);
        let hash_b = dimension_window_scope_hash(scope::COUNTERPARTY, &party_b);

        assert_ne!(hash_a, hash_b);

        // Same party, different dimension kind → distinct window namespaces.
        let exec_a = dimension_window_scope_hash(scope::EXECUTOR, &party_a);
        assert_ne!(hash_a, exec_a);
    }
}
