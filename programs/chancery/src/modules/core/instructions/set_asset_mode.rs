/// set_asset_mode
///
/// Changes the lifecycle mode of an asset (Active / WindDown / Frozen).
/// Direct path only carries RestrictiveImmediate transitions; widening
/// transitions go through `handle_with_pending_change`.
///
/// Mode transition rules (spec 04 §4.6):
///   Active    -> WindDown  Y  (ops/governance, direct)
///   Active    -> Frozen    Y  (governance/emergency, direct - emergency freeze)
///   WindDown  -> Frozen    Y  (governance/emergency, direct)
///   WindDown  -> Active    Y  (governance, timelock)
///   Frozen    -> WindDown  Y  (governance, timelock)
///   Frozen    -> Active    Y  (governance, timelock)
///
/// Accounts (direct):
///   0  chancery_config   writable  PDA  (sequence_nonce bump for evidence)
///   1  asset_config      writable  PDA [b"asset-config", asset_mint]
///   2  authority         signer    ops OR governance OR emergency per transition
///   3  event_authority   readable  PDA [b"event-authority"]
///
/// Accounts (with_pending_change):
///   0  chancery_config         writable  PDA
///   1  event_authority         readable  PDA [b"event-authority"]
///   2  pending_config_change   writable  PDA
///   3  asset_config            writable  PDA
///   4  governance_authority    signer

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_sysvar::Sysvar;

use crate::{
    constants::{asset_mode, change_kind},
    error::ChanceryError,
    modules::{
        control::change_risk::{
            assert_direct_config_change_allowed,
            ConfigChangeRiskClass,
        },
        control::pending_change::compute_config_change_hash,
        core::state::{asset_config::AssetConfig, chancery_config::ChanceryConfig},
        evidence::emit::{emit_asset_mode_changed, AssetModeChanged},
    },
};

// ─── Account indices ──────────────────────────────────────────────────────────
const CHANCERY_CONFIG:                usize = 0;
const ASSET_CONFIG:                   usize = 1;
const AUTHORITY:                      usize = 2;
const EVENT_AUTHORITY_DIRECT:         usize = 3;
const REQUIRED_ACCOUNT_COUNT:         usize = 4;

// ─── Args ─────────────────────────────────────────────────────────────────────
#[derive(BorshDeserialize)]
pub struct SetAssetModeArgs {
    pub new_mode: u8,
}

pub(super) fn classify_asset_mode_transition(old_mode: u8, new_mode: u8) -> ConfigChangeRiskClass {
    if old_mode == new_mode {
        return ConfigChangeRiskClass::RoutineOps;
    }

    match (old_mode, new_mode) {
        (m, n) if m == asset_mode::ACTIVE    && n == asset_mode::WIND_DOWN => {
            ConfigChangeRiskClass::RestrictiveImmediate
        }
        (m, n) if m == asset_mode::ACTIVE    && n == asset_mode::FROZEN    => {
            ConfigChangeRiskClass::RestrictiveImmediate
        }
        (m, n) if m == asset_mode::WIND_DOWN && n == asset_mode::FROZEN => {
            ConfigChangeRiskClass::RestrictiveImmediate
        }
        (m, n) if m == asset_mode::WIND_DOWN && n == asset_mode::ACTIVE => {
            ConfigChangeRiskClass::Widening
        }
        (m, n) if m == asset_mode::FROZEN    && n == asset_mode::WIND_DOWN => {
            ConfigChangeRiskClass::Widening
        }
        (m, n) if m == asset_mode::FROZEN    && n == asset_mode::ACTIVE    => {
            ConfigChangeRiskClass::Widening
        }
        _ => ConfigChangeRiskClass::HighImpact,
    }
}

pub(super) fn asset_mode_hashes(
    asset_config_key: &solana_pubkey::Pubkey,
    risk: ConfigChangeRiskClass,
    current: &AssetConfig,
    proposed: &AssetConfig,
) -> ([u8; 32], [u8; 32]) {
    let current_payload = current.config_change_payload();
    let proposed_payload = proposed.config_change_payload();
    let old_hash = compute_config_change_hash(
        change_kind::SET_ASSET_MODE,
        asset_config_key,
        risk.as_u8(),
        &current_payload,
    );

    let new_hash = compute_config_change_hash(
        change_kind::SET_ASSET_MODE,
        asset_config_key,
        risk.as_u8(),
        &proposed_payload,
    );

    (old_hash, new_hash)
}

pub(super) fn validate_asset_mode_transition(
    current: u8,
    new_mode: u8,
    is_governance: bool,
    is_ops: bool,
    is_emergency: bool,
) -> ProgramResult {
    let permitted = match (current, new_mode) {
        // Operations may wind down an asset immediately; reopening is
        // governance-only and separately requires the pending-change path.
        (m, n) if m == asset_mode::ACTIVE    && n == asset_mode::WIND_DOWN => is_ops || is_governance,
        (m, n) if m == asset_mode::WIND_DOWN && n == asset_mode::ACTIVE => is_governance,
        // Freezing requires governance or emergency.
        (m, n) if m == asset_mode::ACTIVE    && n == asset_mode::FROZEN    => {
            is_governance || is_emergency
        }
        (m, n) if m == asset_mode::WIND_DOWN && n == asset_mode::FROZEN => {
            is_governance || is_emergency
        }
        // Unfreezing is governance-only.
        (m, n) if m == asset_mode::FROZEN    && n == asset_mode::WIND_DOWN => is_governance,
        (m, n) if m == asset_mode::FROZEN    && n == asset_mode::ACTIVE    => is_governance,
        (m, n) if m == n => true,
        _ => false,
    };

    if !permitted {
        return Err(ChanceryError::InsufficientRole.into());
    }

    Ok(())
}

// ─── Handler ──────────────────────────────────────────────────────────────────
pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info = &accounts[CHANCERY_CONFIG];
    let asset_config_account_info    = &accounts[ASSET_CONFIG];
    let authority_account_info       = &accounts[AUTHORITY];

    if !authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !asset_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;
    // Global pause does not block mode changes - emergency freeze must still work.

    let args = SetAssetModeArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let new_mode = args.new_mode;

    if new_mode != asset_mode::ACTIVE
        && new_mode != asset_mode::WIND_DOWN
        && new_mode != asset_mode::FROZEN
    {
        return Err(ChanceryError::AssetModeForbids.into());
    }

    let current_state = *AssetConfig::load_verified(asset_config_account_info)?;
    let current       = current_state.mode;

    // ── Transition authority rules ────────────────────────────────────────────
    let is_governance = authority_account_info.key == &chancery_config.governance_authority;
    let is_ops        = authority_account_info.key == &chancery_config.operations_authority;
    let is_emergency  = authority_account_info.key == &chancery_config.emergency_authority;

    if current == new_mode {
        return Ok(());
    }

    validate_asset_mode_transition(current, new_mode, is_governance, is_ops, is_emergency)?;

    let mut proposed_state = current_state;
    proposed_state.mode    = new_mode;
    proposed_state.zero_reserved();

    let risk = classify_asset_mode_transition(current, new_mode);

    assert_direct_config_change_allowed(risk, &chancery_config, authority_account_info)?;

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    *AssetConfig::load_mut_for_verified_pda(
        asset_config_account_info,
        &current_state.asset_mint,
        current_state.bump,
    )? = proposed_state;

    let clock                   = Clock::get()?;
    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    emit_asset_mode_changed(
        &accounts[EVENT_AUTHORITY_DIRECT],
        event_authority_bump,
        AssetModeChanged {
            sequence_nonce,
            chancery:       *chancery_config_account_info.key,
            slot:           clock.slot,
            unix_timestamp: clock.unix_timestamp,
            risk_class:     risk.as_u8(),
            change_id:      [0u8; 32],
            asset_config:   *asset_config_account_info.key,
            old_mode:       current,
            new_mode,
            proposed_by:    *authority_account_info.key,
            updated_by:     *authority_account_info.key,
        },
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use bytemuck::Zeroable;

    use super::*;

    #[test]
    fn wind_down_reopening_is_governance_only() {
        assert!(validate_asset_mode_transition(
            asset_mode::WIND_DOWN,
            asset_mode::ACTIVE,
            true,
            false,
            false,
        )
        .is_ok());
        assert_eq!(
            validate_asset_mode_transition(
                asset_mode::WIND_DOWN,
                asset_mode::ACTIVE,
                false,
                true,
                false,
            ),
            Err(ChanceryError::InsufficientRole.into()),
        );
        assert_eq!(
            classify_asset_mode_transition(asset_mode::WIND_DOWN, asset_mode::ACTIVE),
            ConfigChangeRiskClass::Widening,
        );
    }

    #[test]
    fn reserved_tail_is_excluded_from_asset_mode_hashes() {
        let key            = solana_pubkey::Pubkey::new_unique();
        let mut current    = AssetConfig::zeroed();
        let mut proposed   = current;

        current._reserved  = [0xAA; 32];
        proposed._reserved = [0x55; 32];

        let (old_hash, new_hash) = asset_mode_hashes(
            &key,
            ConfigChangeRiskClass::RoutineOps,
            &current,
            &proposed,
        );

        assert_eq!(old_hash, new_hash);
    }
}
