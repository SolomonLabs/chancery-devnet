//! Hash + change-id derivation, minimum-timelock table, and consume-side
//! guard for `PendingConfigChange`.

use core::cell::Ref;

use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;
use solana_sha256_hasher::hashv;
use solana_sysvar::Sysvar;

use crate::{
    error::ChanceryError,
    modules::control::{
        change_risk::ConfigChangeRiskClass,
        state::pending_config_change::{PendingConfigChange, config_change_status},
    },
};

const HASH_DOMAIN_CONFIG_CHANGE: &[u8] = b"chancery:pending-config-change:v1";
const HASH_DOMAIN_CHANGE_ID: &[u8]     = b"chancery:pending-config-change-id:v1";

/// Domain-separated hash of `(change_kind, target, risk, payload)`. Target
/// handlers recompute this from current state + args and compare against the
/// pending record before mutating. `payload` must be the canonical semantic
/// encoding only: trailing `_reserved` layout headroom is excluded so later
/// versioned use of reserved storage cannot invalidate a pin.
pub fn compute_config_change_hash(
    change_kind: u16,
    target_account: &Pubkey,
    risk_class: u8,
    payload: &[u8],
) -> [u8; 32] {
    let kind_be = change_kind.to_be_bytes();
    let risk = [risk_class];

    let h = hashv(&[
        HASH_DOMAIN_CONFIG_CHANGE,
        &kind_be,
        target_account.as_ref(),
        &risk,
        payload,
    ]);

    h.to_bytes()
}

/// Domain-separated `change_id`; uniquely identifies a single proposal.
/// The client-selected nonce makes the PDA deterministic before execution and
/// provides replay separation. The actual proposal slot is recorded in state
/// and evidence, but is deliberately not part of PDA derivation.
pub fn compute_change_id(
    change_kind: u16,
    target_account: &Pubkey,
    new_value_hash: &[u8; 32],
    proposer: &Pubkey,
    proposer_nonce: u64,
) -> [u8; 32] {
    let kind_be = change_kind.to_be_bytes();
    let nonce_be = proposer_nonce.to_be_bytes();

    let h = hashv(&[
        HASH_DOMAIN_CHANGE_ID,
        &kind_be,
        target_account.as_ref(),
        new_value_hash,
        proposer.as_ref(),
        &nonce_be,
    ]);

    h.to_bytes()
}

// Per-class minimum timelocks. This is the devnet build: every timelocked
// class carries the smallest floor that still forces a distinct clock reading
// between proposal and execution, so a full propose/accept/consume cycle
// completes inside a test run. The severity ordering of the production
// schedule is deliberately not reproduced here - all four classes are equal -
// so no devnet observation can be read as evidence about production timing.
const WIDENING_TIMELOCK_SECONDS:     i64 = 1;
const HIGH_IMPACT_TIMELOCK_SECONDS:  i64 = 1;
const DANGEROUS_TIMELOCK_SECONDS:    i64 = 1;
const IRREVERSIBLE_TIMELOCK_SECONDS: i64 = 1;

/// Minimum timelock by risk class, in seconds.
pub fn minimum_timelock_seconds_for_risk(risk: ConfigChangeRiskClass) -> i64 {
    match risk {
        ConfigChangeRiskClass::RestrictiveImmediate =>  0,
        ConfigChangeRiskClass::RoutineOps           =>  0,
        ConfigChangeRiskClass::Widening             => WIDENING_TIMELOCK_SECONDS,
        ConfigChangeRiskClass::HighImpact           => HIGH_IMPACT_TIMELOCK_SECONDS,
        ConfigChangeRiskClass::Dangerous            => DANGEROUS_TIMELOCK_SECONDS,
        ConfigChangeRiskClass::Irreversible         => IRREVERSIBLE_TIMELOCK_SECONDS,
    }
}

/// Consume-side guard. Target handlers call this once they've recomputed the
/// proposed-value hash from current state + their handler args.
///
/// Returns the loaded record on success so the caller can read fields like
/// `proposed_by` for evidence emission.
pub fn assert_accepted_pending_change<'a>(
    pending_account_info:         &'a AccountInfo<'a>,
    expected_kind:                u16,
    expected_risk:                ConfigChangeRiskClass,
    expected_target:              &Pubkey,
    expected_old:                 &[u8; 32],
    expected_new:                 &[u8; 32],
    current_governance_authority: &Pubkey,
) -> Result<Ref<'a, PendingConfigChange>, ProgramError> {
    let pending = PendingConfigChange::load_verified(pending_account_info)?;

    if pending.status != config_change_status::ACCEPTED {
        return Err(ChanceryError::ConfigChangeNotAccepted.into());
    }

    if pending.change_kind != expected_kind {
        return Err(ChanceryError::ConfigChangeKindMismatch.into());
    }

    if pending.risk_class != expected_risk.as_u8() {
        return Err(ChanceryError::ConfigChangeRiskClassMismatch.into());
    }

    if pending.target_account != *expected_target {
        return Err(ChanceryError::ConfigChangeWrongTarget.into());
    }

    if &pending.old_value_hash != expected_old {
        return Err(ChanceryError::ConfigChangeHashMismatch.into());
    }

    if &pending.new_value_hash != expected_new {
        return Err(ChanceryError::ConfigChangeHashMismatch.into());
    }

    pending.assert_proposed_by_current_governance(current_governance_authority)?;

    let clock = Clock::get()?;

    pending.assert_not_expired(clock.unix_timestamp)?;

    Ok(pending)
}

/// Consume the pending change terminally: verify it is `ACCEPTED`, then close
/// the account and refund rent to the recipient stored at propose time.
/// Called by target handlers after successful mutation; the terminal record
/// is the handler's evidence event (which carries `change_id`), not the
/// account. When the exact historical address is fresh and available, reusing
/// a closed `change_id` reproduces the identical pinned mutation and must pass
/// acceptance plus a full fresh timelock. Exact-address availability is not a
/// liveness guarantee; normal retry uses a fresh proposer nonce (spec 14
/// §14.3.1).
///
/// The target handler must pass the stored `rent_refund_recipient` from its
/// documented fixed account position as a writable account.
pub fn consume_and_close_pending_change<'a>(
    pending_account_info:       &'a AccountInfo<'a>,
    refund_recipient_account:   &'a AccountInfo<'a>,
) -> Result<(), ProgramError> {
    let rent_refund_recipient = {
        let m = PendingConfigChange::load_verified_mut(pending_account_info)?;

        if m.status != config_change_status::ACCEPTED {
            return Err(ChanceryError::ConfigChangeNotAccepted.into());
        }

        m.rent_refund_recipient
    };

    crate::modules::core::instructions::close_pda_to_recipient_account(
        pending_account_info,
        refund_recipient_account,
        &rent_refund_recipient,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_change_hash_is_deterministic() {
        let target = Pubkey::new_from_array([0xAA; 32]);
        let h1 = compute_config_change_hash(0x0001, &target, 2, b"payload");
        let h2 = compute_config_change_hash(0x0001, &target, 2, b"payload");

        assert_eq!(h1, h2);
    }

    #[test]
    fn config_change_hash_differs_with_kind() {
        let target = Pubkey::new_from_array([0xAA; 32]);
        let h1 = compute_config_change_hash(0x0001, &target, 2, b"payload");
        let h2 = compute_config_change_hash(0x0002, &target, 2, b"payload");

        assert_ne!(h1, h2);
    }

    #[test]
    fn config_change_hash_differs_with_target() {
        let t1 = Pubkey::new_from_array([0xAA; 32]);
        let t2 = Pubkey::new_from_array([0xBB; 32]);
        let h1 = compute_config_change_hash(0x0001, &t1, 2, b"payload");
        let h2 = compute_config_change_hash(0x0001, &t2, 2, b"payload");

        assert_ne!(h1, h2);
    }

    #[test]
    fn change_id_differs_with_nonce() {
        let target = Pubkey::new_from_array([0xAA; 32]);
        let proposer = Pubkey::new_from_array([0xCC; 32]);
        let new_hash = [0xDDu8; 32];
        let id1 = compute_change_id(0x0001, &target, &new_hash, &proposer, 0);
        let id2 = compute_change_id(0x0001, &target, &new_hash, &proposer, 1);

        assert_ne!(id1, id2);
    }

    #[test]
    fn change_id_is_deterministic_before_execution() {
        let target = Pubkey::new_from_array([0xAA; 32]);
        let proposer = Pubkey::new_from_array([0xCC; 32]);
        let new_hash = [0xDDu8; 32];
        let id1 = compute_change_id(0x0001, &target, &new_hash, &proposer, 7);
        let id2 = compute_change_id(0x0001, &target, &new_hash, &proposer, 7);

        assert_eq!(id1, id2);
    }

    #[test]
    fn minimum_timelock_zero_for_immediate() {
        assert_eq!(
            minimum_timelock_seconds_for_risk(ConfigChangeRiskClass::RestrictiveImmediate),
            0
        );
        assert_eq!(
            minimum_timelock_seconds_for_risk(ConfigChangeRiskClass::RoutineOps),
            0
        );
    }

    #[test]
    fn every_timelocked_class_carries_the_devnet_floor() {
        for risk in [
            ConfigChangeRiskClass::Widening,
            ConfigChangeRiskClass::HighImpact,
            ConfigChangeRiskClass::Dangerous,
            ConfigChangeRiskClass::Irreversible,
        ] {
            assert_eq!(minimum_timelock_seconds_for_risk(risk), 1);
        }
    }

    #[test]
    fn timelocked_classes_still_outrank_the_immediate_classes() {
        let immediate = minimum_timelock_seconds_for_risk(ConfigChangeRiskClass::RoutineOps);

        for risk in [
            ConfigChangeRiskClass::Widening,
            ConfigChangeRiskClass::HighImpact,
            ConfigChangeRiskClass::Dangerous,
            ConfigChangeRiskClass::Irreversible,
        ] {
            assert!(minimum_timelock_seconds_for_risk(risk) > immediate);
        }
    }
}
