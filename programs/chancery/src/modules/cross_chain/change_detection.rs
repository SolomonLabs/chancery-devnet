//! Canonical cross-chain policy/signer-set semantic values and risk classifiers.
//!
//! Pending-change hashes bind capability-bearing state only. Provenance and
//! mutation timestamps are deliberately excluded so accepted proposals remain
//! reproducible at execution time.

use solana_program_error::ProgramError;

use crate::{
    constants::{
        chain_kind, remote_domain_mode, remote_domain_pause_bit,
    },
    error::ChanceryError,
    modules::{
        control::change_risk::{max_risk, ConfigChangeRiskClass},
        cross_chain::{
            attestation::{MAX_ATTESTATION_THRESHOLD, MAX_CROSS_CHAIN_SIGNER_COUNT},
            state::{
                cross_chain_signer_set::CrossChainSignerSet,
                remote_domain_policy::RemoteDomainPolicy,
            },
        },
    },
};

pub const REMOTE_DOMAIN_POLICY_VALUE_PAYLOAD_SIZE:   usize = 243;
pub const CROSS_CHAIN_SIGNER_SET_VALUE_PAYLOAD_SIZE: usize = 82;

pub mod remote_domain_change_bit {
    pub const MINIMUM_ATTESTATION_THRESHOLD: u64 = 1 << 0;
    pub const REQUIRED_FINALITY_DEPTH:       u64 = 1 << 1;
    pub const MESSAGE_EXPIRY_SECONDS:        u64 = 1 << 2;
    pub const PER_MESSAGE_MAXIMUM:           u64 = 1 << 3;
    pub const PER_DAY_MAXIMUM:               u64 = 1 << 4;
    pub const REMOTE_DOMAIN_SEPARATOR:       u64 = 1 << 5;
    pub const REMOTE_CHANCERY_CONTRACT:      u64 = 1 << 6;
    pub const REMOTE_ISSUED_TOKEN:           u64 = 1 << 7;
    pub const SIGNER_SET_ID:                 u64 = 1 << 8;
    pub const MODE:                          u64 = 1 << 9;
    pub const REMOTE_ASSET:                  u64 = 1 << 10;
    pub const PAUSE_BITS:                    u64 = 1 << 11;
    pub const LOCAL_ASSET_MINT:              u64 = 1 << 12;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RemoteDomainPolicyValue {
    pub remote_chain_kind:             u8,
    pub minimum_attestation_threshold: u8,
    pub remote_domain_id:              u64,
    pub required_finality_depth:       u64,
    pub message_expiry_seconds:        u64,
    pub per_message_maximum:           u64,
    pub per_day_maximum:               u64,
    pub pause_bits:                    u64,
    pub remote_domain_separator:       [u8; 32],
    pub remote_chancery_contract:      [u8; 32],
    pub remote_issued_token:           [u8; 32],
    pub signer_set_id:                 [u8; 32],
    pub mode:                          u8,
    pub remote_asset:                  [u8; 32],
    pub local_asset_mint:              [u8; 32],
}

impl RemoteDomainPolicyValue {
    pub fn from_policy(policy: &RemoteDomainPolicy) -> Self {
        Self {
            remote_chain_kind:             policy.remote_chain_kind,
            minimum_attestation_threshold: policy.minimum_attestation_threshold,
            remote_domain_id:              policy.remote_domain_id,
            required_finality_depth:       policy.required_finality_depth,
            message_expiry_seconds:        policy.message_expiry_seconds,
            per_message_maximum:           policy.per_message_maximum,
            per_day_maximum:               policy.per_day_maximum,
            pause_bits:                    policy.status_flags & remote_domain_pause_bit::ALL,
            remote_domain_separator:       policy.remote_domain_separator,
            remote_chancery_contract:      policy.remote_chancery_contract,
            remote_issued_token:           policy.remote_issued_token,
            signer_set_id:                 policy.signer_set_id,
            mode:                          policy.mode,
            remote_asset:                  policy.remote_asset,
            local_asset_mint:              policy.local_asset_mint.to_bytes(),
        }
    }

    pub fn to_payload(&self) -> [u8; REMOTE_DOMAIN_POLICY_VALUE_PAYLOAD_SIZE] {
        let mut output = [0u8; REMOTE_DOMAIN_POLICY_VALUE_PAYLOAD_SIZE];
        let mut offset = 0usize;

        output[offset] = self.remote_chain_kind;
        offset += 1;
        output[offset] = self.minimum_attestation_threshold;
        offset += 1;

        for value in [
            self.remote_domain_id,
            self.required_finality_depth,
            self.message_expiry_seconds,
            self.per_message_maximum,
            self.per_day_maximum,
            self.pause_bits,
        ] {
            output[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
            offset += 8;
        }

        for value in [
            self.remote_domain_separator,
            self.remote_chancery_contract,
            self.remote_issued_token,
            self.signer_set_id,
        ] {
            output[offset..offset + 32].copy_from_slice(&value);
            offset += 32;
        }

        output[offset] = self.mode;
        offset += 1;
        output[offset..offset + 32].copy_from_slice(&self.remote_asset);
        offset += 32;
        output[offset..offset + 32].copy_from_slice(&self.local_asset_mint);
        offset += 32;

        debug_assert_eq!(offset, REMOTE_DOMAIN_POLICY_VALUE_PAYLOAD_SIZE);
        output
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RemoteDomainPolicyClassification {
    pub risk:        ConfigChangeRiskClass,
    pub change_mask: u64,
}

#[inline]
fn classify_minimum_change(old: u64, new: u64) -> ConfigChangeRiskClass {
    if new > old {
        ConfigChangeRiskClass::RestrictiveImmediate
    } else if new < old {
        ConfigChangeRiskClass::Dangerous
    } else {
        ConfigChangeRiskClass::RoutineOps
    }
}

#[inline]
fn classify_maximum_or_unbounded_change(old: u64, new: u64) -> ConfigChangeRiskClass {
    match (old, new) {
        (a, b) if a == b => ConfigChangeRiskClass::RoutineOps,
        (0, b) if b != 0 => ConfigChangeRiskClass::RestrictiveImmediate,
        (a, 0) if a != 0 => ConfigChangeRiskClass::Dangerous,
        (a, b) if b < a => ConfigChangeRiskClass::RestrictiveImmediate,
        _ => ConfigChangeRiskClass::Dangerous,
    }
}

pub fn classify_remote_domain_policy_change(
    current:  &RemoteDomainPolicyValue,
    proposed: &RemoteDomainPolicyValue,
) -> Result<RemoteDomainPolicyClassification, ChanceryError> {
    if current.remote_chain_kind != proposed.remote_chain_kind
        || current.remote_domain_id != proposed.remote_domain_id
    {
        return Err(ChanceryError::ImmutableFieldChange);
    }

    let mut risk: Option<ConfigChangeRiskClass> = None;
    let mut change_mask = 0u64;

    macro_rules! raise_risk {
        ($candidate:expr) => {
            risk = Some(match risk {
                Some(current) => max_risk(current, $candidate),
                None => $candidate,
            });
        };
    }

    macro_rules! classify_field {
        ($bit:expr, $old:expr, $new:expr, $candidate:expr) => {
            if $old != $new {
                change_mask |= $bit;
                raise_risk!($candidate);
            }
        };
    }

    classify_field!(
        remote_domain_change_bit::MINIMUM_ATTESTATION_THRESHOLD,
        current.minimum_attestation_threshold,
        proposed.minimum_attestation_threshold,
        classify_minimum_change(
            current.minimum_attestation_threshold as u64,
            proposed.minimum_attestation_threshold as u64,
        )
    );
    classify_field!(
        remote_domain_change_bit::REQUIRED_FINALITY_DEPTH,
        current.required_finality_depth,
        proposed.required_finality_depth,
        classify_minimum_change(
            current.required_finality_depth,
            proposed.required_finality_depth,
        )
    );
    classify_field!(
        remote_domain_change_bit::MESSAGE_EXPIRY_SECONDS,
        current.message_expiry_seconds,
        proposed.message_expiry_seconds,
        classify_maximum_or_unbounded_change(
            current.message_expiry_seconds,
            proposed.message_expiry_seconds,
        )
    );
    classify_field!(
        remote_domain_change_bit::PER_MESSAGE_MAXIMUM,
        current.per_message_maximum,
        proposed.per_message_maximum,
        classify_maximum_or_unbounded_change(
            current.per_message_maximum,
            proposed.per_message_maximum,
        )
    );
    classify_field!(
        remote_domain_change_bit::PER_DAY_MAXIMUM,
        current.per_day_maximum,
        proposed.per_day_maximum,
        classify_maximum_or_unbounded_change(
            current.per_day_maximum,
            proposed.per_day_maximum,
        )
    );

    // H-02 note: endpoint, mode, remote-asset, and local-asset fields are
    // immutable at the args boundary. Signer-set rotation remains legal, but
    // dangerous. These arms also fail future value constructors toward the
    // dangerous class if any identity drift reaches classification.
    for (bit, changed) in [
        (
            remote_domain_change_bit::REMOTE_DOMAIN_SEPARATOR,
            current.remote_domain_separator != proposed.remote_domain_separator,
        ),
        (
            remote_domain_change_bit::REMOTE_CHANCERY_CONTRACT,
            current.remote_chancery_contract != proposed.remote_chancery_contract,
        ),
        (
            remote_domain_change_bit::REMOTE_ISSUED_TOKEN,
            current.remote_issued_token != proposed.remote_issued_token,
        ),
        (
            remote_domain_change_bit::SIGNER_SET_ID,
            current.signer_set_id != proposed.signer_set_id,
        ),
        (
            remote_domain_change_bit::MODE,
            current.mode != proposed.mode,
        ),
        (
            remote_domain_change_bit::REMOTE_ASSET,
            current.remote_asset != proposed.remote_asset,
        ),
        (
            remote_domain_change_bit::LOCAL_ASSET_MINT,
            current.local_asset_mint != proposed.local_asset_mint,
        ),
    ] {
        if changed {
            change_mask |= bit;
            raise_risk!(ConfigChangeRiskClass::Dangerous);
        }
    }

    if current.pause_bits != proposed.pause_bits {
        change_mask |= remote_domain_change_bit::PAUSE_BITS;
        let set_bits = proposed.pause_bits & !current.pause_bits;
        let cleared_bits = current.pause_bits & !proposed.pause_bits;

        if set_bits != 0 {
            raise_risk!(ConfigChangeRiskClass::RestrictiveImmediate);
        }
        if cleared_bits != 0 {
            raise_risk!(ConfigChangeRiskClass::Dangerous);
        }
    }

    Ok(RemoteDomainPolicyClassification {
        risk: risk.unwrap_or(ConfigChangeRiskClass::RoutineOps),
        change_mask,
    })
}

pub fn assert_supported_chain_kind(remote_chain_kind: u8) -> Result<(), ChanceryError> {
    match remote_chain_kind {
        chain_kind::SOLANA
        | chain_kind::EVM
        | chain_kind::COSMOS
        | chain_kind::APTOS
        | chain_kind::SUI => Ok(()),
        _ => Err(ChanceryError::ChainKindNotSupported),
    }
}

pub fn assert_mode_asset_consistent(
    mode:         u8,
    remote_asset: &[u8; 32],
) -> Result<(), ChanceryError> {
    let is_zero = remote_asset.iter().all(|byte| *byte == 0);

    match mode {
        remote_domain_mode::MINT if is_zero => Ok(()),
        remote_domain_mode::MINT => Err(ChanceryError::RemoteAssetForbiddenInMintMode),
        remote_domain_mode::RELEASE if !is_zero => Ok(()),
        remote_domain_mode::RELEASE => Err(ChanceryError::RemoteAssetRequiredInReleaseMode),
        _ => Err(ChanceryError::RemoteDomainModeMismatch),
    }
}

pub fn assert_remote_domain_policy_value_valid(
    value: &RemoteDomainPolicyValue,
) -> Result<(), ChanceryError> {
    assert_supported_chain_kind(value.remote_chain_kind)?;

    if value.minimum_attestation_threshold == 0
        || value.minimum_attestation_threshold > MAX_ATTESTATION_THRESHOLD
    {
        return Err(ChanceryError::InvalidSignerSetParameters);
    }

    if value.pause_bits & !remote_domain_pause_bit::ALL != 0 {
        return Err(ChanceryError::PauseStateInvalidParameters);
    }

    // Message timestamps use i64 throughout the wire format and Solana clock.
    // Reject a wider stored duration at registration/update time rather than
    // accepting a corridor policy that every emit/consume call cannot apply.
    if value.message_expiry_seconds > i64::MAX as u64 {
        return Err(ChanceryError::MessageExpiryWindowExceeded);
    }

    assert_mode_asset_consistent(value.mode, &value.remote_asset)?;

    // These immutable fields are hash-critical corridor identity anchors.
    // An all-zero separator, remote contract, or local daughter-bound asset
    // identifies no executable endpoint and would brick a new corridor.
    if value.remote_domain_separator == [0u8; 32]
        || value.remote_chancery_contract == [0u8; 32]
        || value.local_asset_mint == [0u8; 32]
    {
        return Err(ChanceryError::RemoteIdentityAnchorZero);
    }

    // `remote_issued_token` identifies the daughter representation in MINT
    // mode. RELEASE mode instead binds the nonzero `remote_asset` validated
    // above, so its issued-token field may remain the zero sentinel.
    if value.mode == remote_domain_mode::MINT
        && value.remote_issued_token == [0u8; 32]
    {
        return Err(ChanceryError::RemoteIssuedTokenRequired);
    }

    Ok(())
}

pub fn assert_signer_set_compatible(
    value:            &RemoteDomainPolicyValue,
    signer_set:       &CrossChainSignerSet,
    now:              i64,
    require_temporal: bool,
) -> Result<(), ProgramError> {
    signer_set.assert_id_matches(&value.signer_set_id)?;
    signer_set.assert_active()?;

    if signer_set.threshold < value.minimum_attestation_threshold {
        return Err(ChanceryError::InvalidSignerSetParameters.into());
    }

    if require_temporal {
        signer_set.assert_temporally_valid(now)?;
        signer_set.assert_non_expiring_for_live_corridor()?;
    }

    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CrossChainSignerSetValue {
    pub signer_set_id:              [u8; 32],
    pub threshold:                  u8,
    pub signer_count:               u8,
    pub signer_root:                [u8; 32],
    pub valid_after_unix_timestamp: i64,
    pub expires_at_unix_timestamp:  i64,
}

impl CrossChainSignerSetValue {
    pub fn to_payload(&self) -> [u8; CROSS_CHAIN_SIGNER_SET_VALUE_PAYLOAD_SIZE] {
        let mut output = [0u8; CROSS_CHAIN_SIGNER_SET_VALUE_PAYLOAD_SIZE];
        let mut offset = 0usize;

        output[offset..offset + 32].copy_from_slice(&self.signer_set_id);
        offset += 32;
        output[offset] = self.threshold;
        offset += 1;
        output[offset] = self.signer_count;
        offset += 1;
        output[offset..offset + 32].copy_from_slice(&self.signer_root);
        offset += 32;
        output[offset..offset + 8].copy_from_slice(&self.valid_after_unix_timestamp.to_le_bytes());
        offset += 8;
        output[offset..offset + 8].copy_from_slice(&self.expires_at_unix_timestamp.to_le_bytes());
        offset += 8;

        debug_assert_eq!(offset, CROSS_CHAIN_SIGNER_SET_VALUE_PAYLOAD_SIZE);
        output
    }

    pub fn assert_valid(&self) -> Result<(), ChanceryError> {
        if self.threshold == 0
            || self.signer_count == 0
            || self.threshold > self.signer_count
            || self.threshold > MAX_ATTESTATION_THRESHOLD
            || self.signer_count > MAX_CROSS_CHAIN_SIGNER_COUNT
            || self.signer_set_id == [0u8; 32]
            || self.signer_root == [0u8; 32]
        {
            return Err(ChanceryError::InvalidSignerSetParameters);
        }

        if self.expires_at_unix_timestamp != 0
            && self.valid_after_unix_timestamp != 0
            && self.valid_after_unix_timestamp >= self.expires_at_unix_timestamp
        {
            return Err(ChanceryError::InvalidSignerSetParameters);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_policy() -> RemoteDomainPolicyValue {
        RemoteDomainPolicyValue {
            remote_chain_kind: chain_kind::EVM,
            minimum_attestation_threshold: 2,
            remote_domain_id: 1,
            required_finality_depth: 12,
            message_expiry_seconds: 3_600,
            per_message_maximum: 1_000,
            per_day_maximum: 10_000,
            pause_bits: remote_domain_pause_bit::ALL,
            remote_domain_separator: [1; 32],
            remote_chancery_contract: [2; 32],
            remote_issued_token: [3; 32],
            signer_set_id: [4; 32],
            mode: remote_domain_mode::MINT,
            remote_asset: [0; 32],
            local_asset_mint: [5; 32],
        }
    }

    #[test]
    fn policy_payload_is_fixed_width_and_deterministic() {
        let value = base_policy();
        assert_eq!(value.to_payload().len(), REMOTE_DOMAIN_POLICY_VALUE_PAYLOAD_SIZE);
        assert_eq!(value.to_payload(), value.to_payload());
    }

    #[test]
    fn expiry_window_must_fit_the_signed_timestamp_domain() {
        let mut value = base_policy();
        value.message_expiry_seconds = i64::MAX as u64 + 1;
        assert_eq!(
            assert_remote_domain_policy_value_valid(&value),
            Err(ChanceryError::MessageExpiryWindowExceeded),
        );

        value.message_expiry_seconds = i64::MAX as u64;
        assert!(assert_remote_domain_policy_value_valid(&value).is_ok());
    }

    #[test]
    fn semantic_no_op_is_routine() {
        let current = base_policy();
        let classification = classify_remote_domain_policy_change(&current, &current).unwrap();
        assert_eq!(classification.risk, ConfigChangeRiskClass::RoutineOps);
        assert_eq!(classification.change_mask, 0);
    }

    #[test]
    fn finite_expiry_reduction_is_restrictive() {
        let current = base_policy();
        let mut proposed = current;
        proposed.message_expiry_seconds = 1_800;
        let classification = classify_remote_domain_policy_change(&current, &proposed).unwrap();
        assert_eq!(classification.risk, ConfigChangeRiskClass::RestrictiveImmediate);
        assert_eq!(
            classification.change_mask,
            remote_domain_change_bit::MESSAGE_EXPIRY_SECONDS,
        );
    }

    #[test]
    fn finite_expiry_increase_is_dangerous() {
        let current = base_policy();
        let mut proposed = current;
        proposed.message_expiry_seconds = 7_200;
        let classification = classify_remote_domain_policy_change(&current, &proposed).unwrap();
        assert_eq!(classification.risk, ConfigChangeRiskClass::Dangerous);
    }

    #[test]
    fn threshold_increase_is_restrictive() {
        let current = base_policy();
        let mut proposed = current;
        proposed.minimum_attestation_threshold = 3;
        let classification = classify_remote_domain_policy_change(&current, &proposed).unwrap();
        assert_eq!(classification.risk, ConfigChangeRiskClass::RestrictiveImmediate);
    }

    #[test]
    fn threshold_decrease_is_dangerous() {
        let mut current = base_policy();
        current.minimum_attestation_threshold = 3;
        let mut proposed = current;
        proposed.minimum_attestation_threshold = 2;
        let classification = classify_remote_domain_policy_change(&current, &proposed).unwrap();
        assert_eq!(classification.risk, ConfigChangeRiskClass::Dangerous);
    }

    #[test]
    fn finite_cap_to_unbounded_is_dangerous() {
        let current = base_policy();
        let mut proposed = current;
        proposed.per_day_maximum = 0;
        let classification = classify_remote_domain_policy_change(&current, &proposed).unwrap();
        assert_eq!(classification.risk, ConfigChangeRiskClass::Dangerous);
    }

    #[test]
    fn unbounded_cap_to_finite_is_restrictive() {
        let mut current = base_policy();
        current.per_day_maximum = 0;
        let mut proposed = current;
        proposed.per_day_maximum = 100;
        let classification = classify_remote_domain_policy_change(&current, &proposed).unwrap();
        assert_eq!(classification.risk, ConfigChangeRiskClass::RestrictiveImmediate);
    }

    #[test]
    fn identity_change_is_dangerous() {
        let current = base_policy();
        let mut proposed = current;
        proposed.remote_chancery_contract = [9; 32];
        let classification = classify_remote_domain_policy_change(&current, &proposed).unwrap();
        assert_eq!(classification.risk, ConfigChangeRiskClass::Dangerous);
    }

    #[test]
    fn local_asset_binding_change_is_dangerous() {
        let current = base_policy();
        let mut proposed = current;
        proposed.local_asset_mint = [9; 32];
        let classification = classify_remote_domain_policy_change(&current, &proposed).unwrap();
        assert_eq!(classification.risk, ConfigChangeRiskClass::Dangerous);
        assert_eq!(
            classification.change_mask,
            remote_domain_change_bit::LOCAL_ASSET_MINT,
        );
    }

    #[test]
    fn adding_pause_bits_is_restrictive() {
        let mut current = base_policy();
        current.pause_bits = 0;
        let mut proposed = current;
        proposed.pause_bits = remote_domain_pause_bit::INBOUND_MESSAGES;
        let classification = classify_remote_domain_policy_change(&current, &proposed).unwrap();
        assert_eq!(classification.risk, ConfigChangeRiskClass::RestrictiveImmediate);
    }

    #[test]
    fn clearing_pause_bits_is_dangerous() {
        let current = base_policy();
        let mut proposed = current;
        proposed.pause_bits = current.pause_bits & !remote_domain_pause_bit::INBOUND_MESSAGES;
        let classification = classify_remote_domain_policy_change(&current, &proposed).unwrap();
        assert_eq!(classification.risk, ConfigChangeRiskClass::Dangerous);
    }

    #[test]
    fn mixed_restrictive_and_dangerous_change_is_dangerous() {
        let current = base_policy();
        let mut proposed = current;
        proposed.required_finality_depth = 20;
        proposed.per_message_maximum = 0;
        let classification = classify_remote_domain_policy_change(&current, &proposed).unwrap();
        assert_eq!(classification.risk, ConfigChangeRiskClass::Dangerous);
    }

    #[test]
    fn mode_asset_validation_rejects_inconsistent_pair() {
        let mut value = base_policy();
        value.mode = remote_domain_mode::RELEASE;
        assert_eq!(
            assert_remote_domain_policy_value_valid(&value),
            Err(ChanceryError::RemoteAssetRequiredInReleaseMode),
        );
    }

    #[test]
    fn policy_validation_rejects_zero_remote_endpoint_anchors() {
        let mut value = base_policy();
        value.remote_domain_separator = [0u8; 32];
        assert_eq!(
            assert_remote_domain_policy_value_valid(&value),
            Err(ChanceryError::RemoteIdentityAnchorZero),
        );

        value = base_policy();
        value.remote_chancery_contract = [0u8; 32];
        assert_eq!(
            assert_remote_domain_policy_value_valid(&value),
            Err(ChanceryError::RemoteIdentityAnchorZero),
        );

        value = base_policy();
        value.local_asset_mint = [0u8; 32];
        assert_eq!(
            assert_remote_domain_policy_value_valid(&value),
            Err(ChanceryError::RemoteIdentityAnchorZero),
        );
    }

    #[test]
    fn mint_mode_requires_nonzero_remote_issued_token() {
        let mut value = base_policy();
        value.remote_issued_token = [0u8; 32];
        assert_eq!(
            assert_remote_domain_policy_value_valid(&value),
            Err(ChanceryError::RemoteIssuedTokenRequired),
        );
    }

    #[test]
    fn release_mode_allows_zero_remote_issued_token() {
        let mut value = base_policy();
        value.mode = remote_domain_mode::RELEASE;
        value.remote_asset = [0xAAu8; 32];
        value.remote_issued_token = [0u8; 32];
        assert!(assert_remote_domain_policy_value_valid(&value).is_ok());
    }

    #[test]
    fn policy_identity_is_immutable() {
        let current = base_policy();
        let mut proposed = current;
        proposed.remote_domain_id = 2;
        assert_eq!(
            classify_remote_domain_policy_change(&current, &proposed),
            Err(ChanceryError::ImmutableFieldChange),
        );
    }

    #[test]
    fn signer_set_rejects_invalid_threshold_and_interval() {
        let mut value = CrossChainSignerSetValue {
            signer_set_id: [1; 32],
            threshold: 4,
            signer_count: 3,
            signer_root: [2; 32],
            valid_after_unix_timestamp: 10,
            expires_at_unix_timestamp: 20,
        };
        assert_eq!(value.assert_valid(), Err(ChanceryError::InvalidSignerSetParameters));

        value.threshold = 2;
        value.valid_after_unix_timestamp = 20;
        value.expires_at_unix_timestamp = 20;
        assert_eq!(value.assert_valid(), Err(ChanceryError::InvalidSignerSetParameters));
    }

    #[test]
    fn signer_set_payload_is_fixed_width() {
        let value = CrossChainSignerSetValue {
            signer_set_id: [1; 32],
            threshold: 2,
            signer_count: 3,
            signer_root: [2; 32],
            valid_after_unix_timestamp: 10,
            expires_at_unix_timestamp: 20,
        };
        assert_eq!(value.to_payload().len(), CROSS_CHAIN_SIGNER_SET_VALUE_PAYLOAD_SIZE);
        assert!(value.assert_valid().is_ok());
    }
}
