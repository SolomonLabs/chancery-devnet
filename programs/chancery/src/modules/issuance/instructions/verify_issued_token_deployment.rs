//! verify_issued_token_deployment.
//!
//! ISSUED_TOKEN_CONTROL ix 0x0E. Parses on-chain mint state (SPL base
//! offsets 0..82 + Token-2022 TLV) and sets verification flags on
//! IssuedTokenControl:
//!   MINT_VERIFIED              - base layout sane, decimals match, supply 0,
//!                                mint account owned by issued_token_program
//!   AUTHORITIES_VERIFIED       - mint_authority/freeze_authority point to PDAs
//!   EXTENSIONS_VERIFIED        - observed extension mask within reserved mask,
//!                                disjoint from protocol default forbidden set,
//!                                and disjoint from asset_config forbidden mask
//!   ACCOUNT_STRATEGY_VERIFIED  - auto-set to 1 for MVP per 4
//!   READY_FOR_SETTLEMENT       - set when all of the above are set
//!
//! This instruction is the zero-supply deployment gate only. Live extension
//! observations are refreshed by `refresh_issued_token_extension_observation`.
//!
//! Per (b): clearing READY_FOR_SETTLEMENT on any IssuedTokenControl authority
//! mutation is the responsibility of the mutation handler, not this one.
//!
//! Wire format:
//!   [ ISSUED_TOKEN_CONTROL(0x0B) | VERIFY_ISSUED_TOKEN_DEPLOYMENT(0x0E) | borsh(empty) ]
//!
//! Accounts:
//!   0  module_activation_state  readable PDA  (slot 0 universal)
//!   1  chancery_config          writable PDA  (sequence_nonce for evidence)
//!   2  event_authority          readable PDA
//!   3  asset_config             readable PDA
//!   4  issued_token_control     writable PDA
//!   5  issued_token_mint        readable
//!   6  payer                    signer
//!   7  governance_authority     signer  (issued-token deployment is governance only)

use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_sysvar::Sysvar;

use crate::{
    account_security::assert_external_signer,
    constants::{issued_token_deployment_flag, module},
    error::ChanceryError,
    modules::{
        core::state::{asset_config::AssetConfig, chancery_config::ChanceryConfig},
        control::{
            change_risk::ConfigChangeRiskClass,
            state::module_activation_state::ModuleActivationState,
        },
        evidence::emit::{emit_issued_token_deployment_verified, IssuedTokenDeploymentVerified},
        issuance::{
            state::issued_token_control::IssuedTokenControl,
            tlv::tlv_parser::{
            assert_mint_extension_authorities, parse_mint_extension_mask_full,
            read_spl_base_mint_decimals, ExpectedExtensionAuthorities,
            },
        },
    },
};

const ACTIVATION:             usize = 0;
const CHANCERY_CONFIG:        usize = 1;
const EVENT_AUTHORITY:        usize = 2;
const ASSET_CONFIG:           usize = 3;
const ISSUED_TOKEN_CONTROL:   usize = 4;
const ISSUED_TOKEN_MINT:      usize = 5;
const PAYER:                  usize = 6;
const GOVERNANCE_AUTHORITY:   usize = 7;
const REQUIRED_ACCOUNT_COUNT: usize = 8;

// SPL base mint offsets:
//   0..32   mint_authority (COption<Pubkey>: 4 bytes prefix + 32 bytes value, but in
//                           SPL Token base layout it's 36 bytes: 4-byte option flag + 32 bytes)
//   36..44  supply (u64 LE)
//   44..46  decimals + is_initialized (via read_spl_base_mint_decimals)
//   46..82  freeze_authority (COption<Pubkey>: 4 + 32 = 36 bytes)
//
// We read them with explicit offsets for clarity.

const OFFSET_MINT_AUTHORITY_OPTION:   usize = 0;  // 4-byte COption flag
const OFFSET_MINT_AUTHORITY:          usize = 4;  // 32-byte pubkey
const OFFSET_SUPPLY:                  usize = 36;
const OFFSET_FREEZE_AUTHORITY_OPTION: usize = 46;
const OFFSET_FREEZE_AUTHORITY:        usize = 50;

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args: &[u8]) -> ProgramResult {
    if !args.is_empty() {
        return Err(ChanceryError::ArgsDeserializationFailed.into());
    }

    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let activation_account_info           = &accounts[ACTIVATION];
    let cfg_account_info                  = &accounts[CHANCERY_CONFIG];
    let asset_cfg_account_info            = &accounts[ASSET_CONFIG];
    let issued_token_control_account_info = &accounts[ISSUED_TOKEN_CONTROL];
    let issued_token_mint_account_info    = &accounts[ISSUED_TOKEN_MINT];
    let governance_authority_account_info = &accounts[GOVERNANCE_AUTHORITY];
    let payer_account_info                = &accounts[PAYER];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];

    assert_external_signer(governance_authority_account_info, &crate::id())?;

    if !payer_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !cfg_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !issued_token_control_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let program_id = crate::id();
    let activation_bump = ModuleActivationState::verify_pda(activation_account_info, &program_id)?;
    let activation = ModuleActivationState::load_for_verified_pda(
        activation_account_info,
        activation_bump,
    )?;

    activation.assert_admin_or_active(module::ISSUED_TOKEN_CONTROL)?;

    let config_bump = ChanceryConfig::verify_pda(cfg_account_info, &program_id)?;
    let chancery_config = ChanceryConfig::load_for_verified_pda(
        cfg_account_info,
        config_bump,
    )?;

    if governance_authority_account_info.key != &chancery_config.governance_authority {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    if &chancery_config.issued_token_mint != issued_token_mint_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    // Same mint-owner gate as register_asset / refresh_asset_extension_observation;
    // audit #65: reject before reading SPL layout bytes.
    if issued_token_mint_account_info.owner != &chancery_config.issued_token_program {
        return Err(ChanceryError::AccountOwnerMismatch.into());
    }

    // Read mint account.
    let mint_data = issued_token_mint_account_info.try_borrow_data()?;

    // ── 1. MINT_VERIFIED: base layout sanity ─────────────────────────────
    let on_chain_decimals = match read_spl_base_mint_decimals(&mint_data) {
        Ok(decimals) => decimals,
        Err(ChanceryError::NotInitialized) => {
            return Err(ChanceryError::IssuedTokenDeploymentNotVerified.into());
        }
        Err(e) => return Err(e.into()),
    };

    let mut supply_bytes: [u8; 8] = [0u8; 8];

    supply_bytes.copy_from_slice(&mint_data[OFFSET_SUPPLY..OFFSET_SUPPLY + 8]);

    let supply: u64 = u64::from_le_bytes(supply_bytes);

    if supply != 0 {
        return Err(ChanceryError::IssuedTokenDeploymentNotVerified.into());
    }

    // Decimals must match the asset_config registered for this issued mint.
    // ChanceryConfig itself does not carry a decimals field - `AssetConfig` is
    // the source of truth (it already enforces decimals match the on-chain
    // mint at registration time, the specification).
    let asset_config_bump = AssetConfig::verify_pda(
        asset_cfg_account_info,
        issued_token_mint_account_info.key,
        &program_id,
    )?;
    let asset_config = AssetConfig::load_for_verified_pda(
        asset_cfg_account_info,
        issued_token_mint_account_info.key,
        asset_config_bump,
    )?;

    if asset_config.asset_token_program != chancery_config.issued_token_program {
        return Err(ChanceryError::TokenProgramMismatch.into());
    }

    let issued_control_bump = IssuedTokenControl::verify_pda(
        issued_token_control_account_info,
        &program_id,
    )?;

    if on_chain_decimals != asset_config.decimals {
        return Err(ChanceryError::IssuedTokenDeploymentNotVerified.into());
    }

    // ── 2. AUTHORITIES_VERIFIED: mint+freeze authority point to chancery PDAs ──
    let mut mint_authority_option_bytes: [u8; 4] = [0u8; 4];

    mint_authority_option_bytes.copy_from_slice(&mint_data[OFFSET_MINT_AUTHORITY_OPTION..OFFSET_MINT_AUTHORITY_OPTION + 4]);

    let mint_authority_option: u32 = u32::from_le_bytes(mint_authority_option_bytes);

    if mint_authority_option != 1 {
        return Err(ChanceryError::IssuedTokenMintAuthorityInvalid.into());
    }

    let mut on_chain_mint_authority: [u8; 32] = [0u8; 32];

    on_chain_mint_authority.copy_from_slice(&mint_data[OFFSET_MINT_AUTHORITY..OFFSET_MINT_AUTHORITY + 32]);

    if on_chain_mint_authority != chancery_config.mint_authority_pda.to_bytes() {
        return Err(ChanceryError::IssuedTokenMintAuthorityInvalid.into());
    }

    let mut freeze_authority_option_bytes: [u8; 4] = [0u8; 4];

    freeze_authority_option_bytes.copy_from_slice(&mint_data[OFFSET_FREEZE_AUTHORITY_OPTION..OFFSET_FREEZE_AUTHORITY_OPTION + 4]);

    let freeze_authority_option: u32 = u32::from_le_bytes(freeze_authority_option_bytes);

    if freeze_authority_option != 1 {
        return Err(ChanceryError::IssuedTokenFreezeAuthorityInvalid.into());
    }

    let mut on_chain_freeze_authority: [u8; 32] = [0u8; 32];
    on_chain_freeze_authority.copy_from_slice(&mint_data[OFFSET_FREEZE_AUTHORITY..OFFSET_FREEZE_AUTHORITY + 32]);

    if on_chain_freeze_authority != chancery_config.freeze_authority_pda.to_bytes() {
        return Err(ChanceryError::IssuedTokenFreezeAuthorityInvalid.into());
    }

    // ── 3. EXTENSIONS_VERIFIED: TLV parse + mask checks + authority binding ──
    let observed = parse_mint_extension_mask_full(&mint_data)?;

    // ── 4. Set flags on IssuedTokenControl ──────────────────────────────
    let clock = Clock::get()?;
    let control_flags_after: u64;
    {
        let mut ctrl = IssuedTokenControl::load_mut_for_verified_pda(
            issued_token_control_account_info,
            issued_control_bump,
        )?;

        if &ctrl.issued_token_mint != issued_token_mint_account_info.key
            || ctrl.issued_token_program != chancery_config.issued_token_program
        {
            return Err(ChanceryError::AccountKeyMismatch.into());
        }

        // Two-layer forbidden policy (audit #64):
        //   1. protocol default (`MINT_FORBIDDEN_*`) + reserved mask
        //   2. operator runtime knob (`asset_config.forbidden_extension_mask`)
        // Default hits keep `IssuedTokenForbiddenDefaultExtensionActive`; asset-level
        // hits return `ForbiddenExtension` (same variant as collateral settlement).
        ctrl.assert_observed_mint_mask_valid(observed)?;
        asset_config.assert_forbidden_extensions_absent(observed)?;

        // Audit issue #72: every present reserved authority-bearing extension
        // must either bind to a known Chancery PDA or fail closed.
        assert_mint_extension_authorities(
            &mint_data,
            observed,
            ctrl.reserved_mint_extension_mask,
            &ExpectedExtensionAuthorities {
                permanent_delegate:              ctrl.permanent_delegate_authority_pda.to_bytes(),
                transfer_hook_authority:         ctrl.transfer_hook_authority_pda.to_bytes(),
                transfer_hook_program:           ctrl.hook_program_id.to_bytes(),
                close_mint_authority:            ctrl.close_mint_authority_pda.to_bytes(),
                pause_authority:                 ctrl.pause_authority_pda.to_bytes(),
                metadata_pointer_authority:      ctrl.metadata_pointer_authority_pda.to_bytes(),
                metadata_update_authority:       ctrl.metadata_update_authority_pda.to_bytes(),
                confidential_transfer_authority: ctrl.confidential_transfer_authority_pda.to_bytes(),
            },
        )?;

        let flags_pre: u64 = ctrl.control_flags;
        let flags: u64 = flags_pre
            | issued_token_deployment_flag::MINT_VERIFIED
            | issued_token_deployment_flag::AUTHORITIES_VERIFIED
            | issued_token_deployment_flag::EXTENSIONS_VERIFIED
            | issued_token_deployment_flag::ACCOUNT_STRATEGY_VERIFIED;

        let ready: bool = (flags & issued_token_deployment_flag::ALL_PRE_REQUIRED)
            == issued_token_deployment_flag::ALL_PRE_REQUIRED;

        ctrl.control_flags = if ready {
            flags | issued_token_deployment_flag::READY_FOR_SETTLEMENT
        } else {
            flags
        };

        ctrl.active_mint_extension_mask = observed;
        ctrl.extension_observed_at_slot = clock.slot;
        control_flags_after             = ctrl.control_flags;
    }
    drop(mint_data);

    drop(chancery_config);

    let (event_authority_bump, sequence_nonce) = {
        let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(cfg_account_info, config_bump)?;
        let event_authority_bump    = chancery_config_mut.event_authority_bump;
        let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;
        (event_authority_bump, sequence_nonce)
    };

    emit_issued_token_deployment_verified(
        event_authority_account_info,
        event_authority_bump,
        IssuedTokenDeploymentVerified {
            sequence_nonce,
            chancery:                *cfg_account_info.key,
            slot:                    clock.slot,
            unix_timestamp:          clock.unix_timestamp,
            risk_class:              ConfigChangeRiskClass::HighImpact.as_u8(),
            issued_token_control:    *issued_token_control_account_info.key,
            observed_extension_mask: observed,
            control_flags:           control_flags_after,
            verified_by:            *governance_authority_account_info.key,
        },
    )?;

    Ok(())
}
