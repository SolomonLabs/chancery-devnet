//! refresh_asset_extension_observation
//!
//! Re-parses the on-chain collateral mint TLV, writes the actual observation,
//! bumps `extension_observed_at_slot`, and emits `AssetExtensionRefreshed`
//! evidence.
//!
//! Observation is detection, not authorization. A collateral mask that is no
//! longer approved (including an activated TransferHook) is still persisted so
//! the account cannot retain a misleading last-known-good mask. Fund-moving
//! consumers enforce `AssetConfig` policy and fail closed on the newly recorded
//! mask. This instruction must return success after recording such a change;
//! returning an error would roll the observation back with the transaction.
//!
//! Issued-token extension refresh uses the dedicated
//! `refresh_issued_token_extension_observation` instruction so live supply does
//! not block freshness maintenance.
//!
//! Pathway-level forbidden masks are enforced at settlement, not here.
//!
//! Wire format:
//!   [ ISSUED_TOKEN_CONTROL(0x0B) | REFRESH_ASSET_EXTENSION_OBSERVATION(0x0C) | (no args) ]
//!
//! Accounts:
//!   0  chancery_config        writable  PDA
//!   1  event_authority        readable  PDA [b"event-authority"]
//!   2  asset_config           writable  PDA [b"asset-config", asset_mint]
//!   3  asset_mint             readable  collateral mint (TLV source)
//!   4  operations_authority   signer    must be chancery_config.operations_authority

use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_program_error::ProgramError;
use solana_sysvar::Sysvar;

use crate::{
    error::ChanceryError,
    modules::{
        control::change_risk::ConfigChangeRiskClass,
        core::state::{asset_config::AssetConfig, chancery_config::ChanceryConfig},
        evidence::emit::{emit_asset_extension_refreshed, AssetExtensionRefreshed},
        issuance::tlv::tlv_parser::parse_mint_extension_mask_full,
    },
};

const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const ASSET_CONFIG:           usize = 2;
const ASSET_MINT:             usize = 3;
const OPERATIONS_AUTHORITY:   usize = 4;
const REQUIRED_ACCOUNT_COUNT: usize = 5;

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args: &[u8]) -> ProgramResult {
    if !args.is_empty() {
        return Err(ChanceryError::ArgsDeserializationFailed.into());
    }

    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info      = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];
    let asset_config_account_info         = &accounts[ASSET_CONFIG];
    let asset_mint_account_info           = &accounts[ASSET_MINT];
    let operations_authority_account_info = &accounts[OPERATIONS_AUTHORITY];

    if !operations_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !asset_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    if operations_authority_account_info.key != &chancery_config.operations_authority {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    let mut cfg = AssetConfig::load_verified_mut(asset_config_account_info)?;

    if &cfg.asset_mint != asset_mint_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if asset_mint_account_info.owner != &cfg.asset_token_program {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mint_data = asset_mint_account_info.try_borrow_data()?;
    let observed  = parse_mint_extension_mask_full(&mint_data)?;

    // The canonical issued-token mint has a dedicated refresh instruction that
    // records negative verification evidence and controls settlement readiness.
    // Preserve this defensive policy check if it is passed to the collateral
    // refresh path. Collateral observations are deliberately not rejected here:
    // they must be committed before consumers enforce the configured policy.
    if &cfg.asset_mint == &chancery_config.issued_token_mint {
        AssetConfig::assert_mask_subset_of_approved(
            observed,
            cfg.approved_extension_mask,
            cfg.forbidden_extension_mask,
        )?;
    }

    let clock        = Clock::get()?;
    let old_observed = cfg.observed_extension_mask;

    cfg.observed_extension_mask    = observed;
    cfg.extension_observed_at_slot = clock.slot;

    drop(chancery_config);

    let (event_authority_bump, sequence_nonce) = {
        let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
        let event_authority_bump    = chancery_config_mut.event_authority_bump;
        let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;
        (event_authority_bump, sequence_nonce)
    };

    emit_asset_extension_refreshed(
        event_authority_account_info,
        event_authority_bump,
        AssetExtensionRefreshed {
            sequence_nonce,
            chancery:                    *chancery_config_account_info.key,
            slot:                        clock.slot,
            unix_timestamp:              clock.unix_timestamp,
            risk_class:                  ConfigChangeRiskClass::RoutineOps.as_u8(),
            asset_config:                *asset_config_account_info.key,
            asset_mint:                  *asset_mint_account_info.key,
            old_observed_extension_mask: old_observed,
            new_observed_extension_mask: observed,
            refreshed_by:                *operations_authority_account_info.key,
        },
    )?;

    Ok(())
}
