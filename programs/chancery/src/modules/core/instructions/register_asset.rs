/// register_asset
///
/// Creates an AssetConfigPda for a new collateral asset mint.
///
/// `observed_extension_mask` is always derived from the collateral mint TLV at
/// registration (not caller-supplied). An optional hint must match or
/// registration fails with `Token2022ObservedMaskMismatch`.
///
/// Accounts:
///   0  chancery_config            writable  PDA - asserts initialized; seq bump
///
/// Asset registration is policy administration, not economic execution. It
/// intentionally remains reachable while global mint/redeem/reserve pauses are
/// active so operators can register replacement or recovery assets during an
/// incident; settlement remains blocked by the relevant execution pause bits.
///   1  event_authority            readable  PDA [b"event-authority"]
///   2  asset_config               writable  PDA [b"asset-config", asset_mint]
///   3  asset_mint                 readable  the collateral mint to register
///   4  payer                      signer
///   5  operations_authority       signer    must be chancery_config.operations_authority
///   6  system_program
///
/// Permission: operations_authority (no PermissionRecord lookup - authority key check
/// is sufficient for admin-only instructions on core config).

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    constants::{asset_mode, seeds, status_flag, token_program},
    error::ChanceryError,
    modules::{
        core::{
            instructions::create_pda_account,
            state::{
                asset_config::{
                    AssetConfig, ASSET_CONFIG_DISCRIMINATOR, ASSET_CONFIG_SIZE,
                },
                chancery_config::ChanceryConfig,
            },
        },
        issuance::tlv::tlv_parser::{parse_mint_extension_mask_full, read_spl_base_mint_decimals},
        evidence::emit::{emit_asset_registered, AssetRegistered},
    },
};

// ─── Account indices ──────────────────────────────────────────────────────────
const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const ASSET_CONFIG:           usize = 2;
const ASSET_MINT:             usize = 3;
const PAYER:                  usize = 4;
const OPERATIONS_AUTHORITY:   usize = 5;
const SYSTEM_PROGRAM:         usize = 6;
const REQUIRED_ACCOUNT_COUNT: usize = 7;

// ─── Args ─────────────────────────────────────────────────────────────────────
#[derive(BorshDeserialize)]
pub struct RegisterAssetArgs {
    /// Initial mode - must be `asset_mode::WIND_DOWN` (issue-75). Promotion to
    /// ACTIVE is governance-only and timelocked via set_asset_mode.
    pub mode:                             u8,

    /// Approved Token-2022 extension mask (two u64 words, low then high).
    pub approved_extension_mask:          [u64; 2],

    /// Optional operator hint; when present must match the TLV-derived mask or
    /// registration fails with `Token2022ObservedMaskMismatch`. On-chain
    /// observed storage is always taken from the mint account.
    pub observed_extension_mask_hint:     Option<[u64; 2]>,

    /// deposit_rate × 10^9. E.g. 1:1 = 1_000_000_000.
    pub deposit_rate_e9:                  u64,

    /// redeem_rate × 10^9.
    pub redeem_rate_e9:                   u64,
    pub minimum_deposit_amount:           u64,
    pub minimum_redeem_amount:            u64,
    pub maximum_single_settlement_amount: u64,

    /// Max age in slots before collateral extension observation is stale at
    /// settlement. Zero disables only maximum-age expiry; future observations
    /// remain invalid.
    pub max_extension_observation_age_slots: u64,

    /// Token program that owns the asset mint (SPL or Token-2022 program key).
    pub asset_token_program:              Pubkey,
}


fn assert_initial_asset_mode(mode: u8) -> ProgramResult {
    // issue-75: registration is inert. New assets start WIND_DOWN (redeem-only,
    // no deposits); reaching ACTIVE requires the governance-only, timelocked
    // WIND_DOWN -> ACTIVE transition in set_asset_mode. This keeps the "an asset
    // becomes ACTIVE" event on a single governed path — an ops key can no longer
    // stand up live collateral (and drain real reserves against it) in one instant
    // instruction. FROZEN is not a valid initial mode either.
    if mode != asset_mode::WIND_DOWN {
        return Err(ChanceryError::AssetModeForbids.into());
    }
    Ok(())
}

// ─── Handler ──────────────────────────────────────────────────────────────────
pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info      = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];
    let asset_config_account_info         = &accounts[ASSET_CONFIG];
    let asset_mint_account_info           = &accounts[ASSET_MINT];
    let payer_account_info                = &accounts[PAYER];
    let operations_authority_account_info = &accounts[OPERATIONS_AUTHORITY];
    let system_program_account_info       = &accounts[SYSTEM_PROGRAM];

    // ── Signers ───────────────────────────────────────────────────────────────
    if !payer_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !operations_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !asset_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    // ── Load + validate chancery config ──────────────────────────────────────────
    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    if operations_authority_account_info.key != &chancery_config.operations_authority {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    // ── Args ──────────────────────────────────────────────────────────────────
    let args = RegisterAssetArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    // Registration accepts only the two documented initial lifecycle states.
    assert_initial_asset_mode(args.mode)?;

    AssetConfig::assert_economic_invariants(args.deposit_rate_e9, args.redeem_rate_e9)?;
    AssetConfig::assert_amount_invariants(
        args.minimum_deposit_amount,
        args.minimum_redeem_amount,
        args.maximum_single_settlement_amount,
    )?;
    AssetConfig::assert_amount_rate_ranges_executable(
        args.deposit_rate_e9,
        args.redeem_rate_e9,
        args.minimum_deposit_amount,
        args.minimum_redeem_amount,
        args.maximum_single_settlement_amount,
    )?;

    // Settlement later trusts this stored program id to move real collateral via
    // CPI, so it must be a genuine token program - the same allowlist init applies
    // to the issued/legacy programs (issue-71). Without this a malicious executable
    // could be registered as the collateral program, no-op the transfer CPI, and
    // let permissioned minters receive real issued tokens against no collateral.
    if !token_program::is_accepted(&args.asset_token_program) {
        return Err(ChanceryError::TokenProgramMismatch.into());
    }

    if asset_mint_account_info.owner != &args.asset_token_program {
        return Err(ProgramError::IncorrectProgramId);
    }

    // ── Derive observed extension mask from on-chain mint TLV ─────────────────
    let mint_data = asset_mint_account_info.try_borrow_data()?;
    let decimals  = read_spl_base_mint_decimals(&mint_data)?;
    let observed  = parse_mint_extension_mask_full(&mint_data)?;

    if let Some(declared) = args.observed_extension_mask_hint {
        if declared != observed {
            return Err(ChanceryError::Token2022ObservedMaskMismatch.into());
        }
    }

    // The canonical issued mint retains an AssetConfig because deployment and
    // extension-observation instructions bind to it. It is not collateral:
    // PathwayPolicy independently rejects asset_mint == issued_token_mint. The
    // collateral default-forbidden mask therefore applies only to actual
    // collateral mints; issued-mint extension policy is governed by
    // IssuedTokenControl plus this AssetConfig's explicit masks.
    let is_issued_token_mint = asset_mint_account_info.key == &chancery_config.issued_token_mint;
    let collateral_forbidden = if is_issued_token_mint {
        [0u64; 2]
    } else {
        [
            crate::constants::collateral_default_forbidden::MINT_FORBIDDEN_LO,
            crate::constants::collateral_default_forbidden::MINT_FORBIDDEN_HI,
        ]
    };

    if (args.approved_extension_mask[0] & collateral_forbidden[0] != 0)
        || (args.approved_extension_mask[1] & collateral_forbidden[1] != 0)
    {
        return Err(ChanceryError::ForbiddenExtension.into());
    }

    AssetConfig::assert_mask_subset_of_approved(
        observed,
        args.approved_extension_mask,
        collateral_forbidden,
    )?;

    let clock = Clock::get()?;

    // ── Derive + verify asset_config PDA ─────────────────────────────────────
    let program_id = crate::id();
    let (expected_key, bump) = Pubkey::find_program_address(
        &[seeds::ASSET_CONFIG, asset_mint_account_info.key.as_ref()],
        &program_id,
    );

    if asset_config_account_info.key != &expected_key {
        return Err(ChanceryError::InvalidPda.into());
    }

    // ── Create PDA account ────────────────────────────────────────────────────
    if asset_config_account_info.data_is_empty() {
        create_pda_account(
            payer_account_info,
            asset_config_account_info,
            system_program_account_info,
            &program_id,
            &[seeds::ASSET_CONFIG, asset_mint_account_info.key.as_ref(), &[bump]],
            ASSET_CONFIG_SIZE,
        )?;
    } else if asset_config_account_info.data_len() != ASSET_CONFIG_SIZE {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    // ── Write state ───────────────────────────────────────────────────────────
    let mut cfg = AssetConfig::load_uninitialized_mut(asset_config_account_info)?;

    cfg.discriminator                       = ASSET_CONFIG_DISCRIMINATOR;
    cfg.version                             = 1;
    cfg.bump                                = bump;
    cfg.decimals                            = decimals;
    cfg.mode                                = args.mode;
    cfg._pad0                               = [0u8; 3];
    cfg.asset_flags                         = 0;
    cfg.asset_mint                          = *asset_mint_account_info.key;
    cfg.asset_token_program                 = args.asset_token_program;
    cfg.primary_reserve_compartment_id      = [0u8; 32];  // set via update if compartments active
    cfg.approved_extension_mask             = args.approved_extension_mask;
    cfg.observed_extension_mask             = observed;
    cfg.deposit_rate_e9                     = args.deposit_rate_e9;
    cfg.redeem_rate_e9                      = args.redeem_rate_e9;
    cfg.minimum_deposit_amount              = args.minimum_deposit_amount;
    cfg.minimum_redeem_amount               = args.minimum_redeem_amount;
    cfg.maximum_single_settlement_amount    = args.maximum_single_settlement_amount;
    cfg.status_flags                        = status_flag::INITIALIZED;
    cfg.forbidden_extension_mask            = [0u64; 2];
    cfg.required_module_mask                = [0u64; 2];
    cfg.extension_observed_at_slot          = clock.slot;
    cfg.max_extension_observation_age_slots = args.max_extension_observation_age_slots;
    cfg.zero_reserved();

    // ── Evidence (emitted last, after all state writes) ─────────────────────────
    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    emit_asset_registered(
        event_authority_account_info,
        event_authority_bump,
        AssetRegistered {
            sequence_nonce,
            chancery:             *chancery_config_account_info.key,
            slot:                 clock.slot,
            unix_timestamp:       clock.unix_timestamp,
            risk_class:           0,
            asset_config:         *asset_config_account_info.key,
            asset_mint:           *asset_mint_account_info.key,
            asset_token_program:  args.asset_token_program,
            mode:                 args.mode,
            is_issued_token_mint,
            registered_by:        *operations_authority_account_info.key,
        },
    )?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_asset_mode_accepts_only_wind_down() {
        // issue-75: WIND_DOWN is the only valid initial mode; ACTIVE must go
        // through the governance-only, timelocked promotion path.
        assert!(assert_initial_asset_mode(asset_mode::WIND_DOWN).is_ok());
        assert_eq!(
            assert_initial_asset_mode(asset_mode::ACTIVE),
            Err(ChanceryError::AssetModeForbids.into()),
        );
        assert_eq!(
            assert_initial_asset_mode(asset_mode::FROZEN),
            Err(ChanceryError::AssetModeForbids.into()),
        );
        assert_eq!(
            assert_initial_asset_mode(3),
            Err(ChanceryError::AssetModeForbids.into()),
        );
        assert_eq!(
            assert_initial_asset_mode(u8::MAX),
            Err(ChanceryError::AssetModeForbids.into()),
        );
    }

    #[test]
    fn amount_bounds_must_leave_an_executable_domain() {
        assert!(AssetConfig::assert_amount_invariants(1, 1, 1).is_ok());
        assert_eq!(
            AssetConfig::assert_amount_invariants(0, 0, 0),
            Err(ChanceryError::AssetEconomicsInvariantViolated.into()),
        );
        assert_eq!(
            AssetConfig::assert_amount_invariants(11, 1, 10),
            Err(ChanceryError::AssetEconomicsInvariantViolated.into()),
        );
        assert_eq!(
            AssetConfig::assert_amount_invariants(1, 11, 10),
            Err(ChanceryError::AssetEconomicsInvariantViolated.into()),
        );
    }

}
