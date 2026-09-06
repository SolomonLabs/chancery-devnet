//! Refresh the live issued-token mint extension observation.
//!
//! Unlike deployment verification, this path is valid with non-zero supply.
//! It requires that the deployment verification flags were previously
//! established, rechecks every active extension authority and forbidden-mask
//! layer, and updates freshness. A regression clears READY_FOR_SETTLEMENT and
//! emits evidence in the same successful transaction so settlement fails
//! closed and operators can observe the cause.
//!
//! Accounts:
//!   0  chancery_config          writable PDA
//!   1  event_authority          readable PDA
//!   2  asset_config             readable PDA for issued_token_mint
//!   3  issued_token_control     writable PDA
//!   4  issued_token_mint        readable
//!   5  operations_authority     signer
//!
//! The module activation account is the universal account prefix consumed by
//! `dispatch_gated` before this handler is entered.

use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_program_error::ProgramError;
use solana_sysvar::Sysvar;

use crate::{
    account_security::assert_external_signer,
    constants::issued_token_deployment_flag,
    error::ChanceryError,
    modules::{
        control::change_risk::ConfigChangeRiskClass,
        core::state::{asset_config::AssetConfig, chancery_config::ChanceryConfig},
        evidence::emit::{
            emit_issued_token_extension_observation_refreshed,
            IssuedTokenExtensionObservationRefreshed,
        },
        issuance::{
            state::issued_token_control::IssuedTokenControl,
            tlv::tlv_parser::{
                assert_mint_extension_authorities, parse_mint_extension_mask_full,
                read_spl_base_mint_decimals, ExpectedExtensionAuthorities,
            },
        },
    },
};

const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const ASSET_CONFIG:           usize = 2;
const ISSUED_TOKEN_CONTROL:   usize = 3;
const ISSUED_TOKEN_MINT:      usize = 4;
const OPERATIONS_AUTHORITY:   usize = 5;
const REQUIRED_ACCOUNT_COUNT: usize = 6;

const OFFSET_MINT_AUTHORITY_OPTION:   usize = 0;
const OFFSET_MINT_AUTHORITY:          usize = 4;
const OFFSET_FREEZE_AUTHORITY_OPTION: usize = 46;
const OFFSET_FREEZE_AUTHORITY:        usize = 50;


fn apply_observation_result(
    control: &mut IssuedTokenControl,
    observed: [u64; 2],
    observed_at_slot: u64,
    regression_error_code: u32,
) -> bool {
    let verification_succeeded = regression_error_code == 0;

    if verification_succeeded {
        control.active_mint_extension_mask = observed;
        control.extension_observed_at_slot = observed_at_slot;
        control.control_flags |= issued_token_deployment_flag::READY_FOR_SETTLEMENT;
    } else {
        control.control_flags &= !issued_token_deployment_flag::READY_FOR_SETTLEMENT;
    }

    verification_succeeded
}

fn program_error_code(error: ProgramError) -> u32 {
    match error {
        ProgramError::Custom(code) => code,
        _ => u32::MAX,
    }
}

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
    let issued_token_control_account_info = &accounts[ISSUED_TOKEN_CONTROL];
    let issued_token_mint_account_info    = &accounts[ISSUED_TOKEN_MINT];
    let operations_authority_account_info = &accounts[OPERATIONS_AUTHORITY];

    assert_external_signer(operations_authority_account_info, &crate::id())?;

    if !chancery_config_account_info.is_writable
        || !issued_token_control_account_info.is_writable
    {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let program_id = crate::id();

    let config_bump = ChanceryConfig::verify_pda(chancery_config_account_info, &program_id)?;
    let chancery_config = ChanceryConfig::load_for_verified_pda(
        chancery_config_account_info,
        config_bump,
    )?;

    if operations_authority_account_info.key != &chancery_config.operations_authority {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    if issued_token_mint_account_info.key != &chancery_config.issued_token_mint
        || issued_token_mint_account_info.owner != &chancery_config.issued_token_program
    {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    let asset_config_bump = AssetConfig::verify_pda(
        asset_config_account_info,
        issued_token_mint_account_info.key,
        &program_id,
    )?;
    let asset_config = AssetConfig::load_for_verified_pda(
        asset_config_account_info,
        issued_token_mint_account_info.key,
        asset_config_bump,
    )?;

    if asset_config.asset_token_program != chancery_config.issued_token_program {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    let control_bump = IssuedTokenControl::verify_pda(
        issued_token_control_account_info,
        &program_id,
    )?;
    let control = IssuedTokenControl::load_for_verified_pda(
        issued_token_control_account_info,
        control_bump,
    )?;

    if &control.issued_token_mint != issued_token_mint_account_info.key
        || control.issued_token_program != chancery_config.issued_token_program
    {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if control.control_flags & issued_token_deployment_flag::ALL_PRE_REQUIRED
        != issued_token_deployment_flag::ALL_PRE_REQUIRED
    {
        return Err(ChanceryError::IssuedTokenDeploymentNotVerified.into());
    }

    let expected_authorities = ExpectedExtensionAuthorities {
        permanent_delegate:              control.permanent_delegate_authority_pda.to_bytes(),
        transfer_hook_authority:         control.transfer_hook_authority_pda.to_bytes(),
        transfer_hook_program:           control.hook_program_id.to_bytes(),
        close_mint_authority:            control.close_mint_authority_pda.to_bytes(),
        pause_authority:                 control.pause_authority_pda.to_bytes(),
        metadata_pointer_authority:      control.metadata_pointer_authority_pda.to_bytes(),
        metadata_update_authority:       control.metadata_update_authority_pda.to_bytes(),
        confidential_transfer_authority: control.confidential_transfer_authority_pda.to_bytes(),
    };

    let previous_observed = control.active_mint_extension_mask;
    let mint_data = issued_token_mint_account_info.try_borrow_data()?;

    let parsed_observed = parse_mint_extension_mask_full(&mint_data)
        .map_err(ProgramError::from);
    let observed = parsed_observed.as_ref().copied().unwrap_or(previous_observed);

    let validation_result: Result<(), ProgramError> = (|| {
        let decimals = read_spl_base_mint_decimals(&mint_data)
            .map_err(ProgramError::from)?;

        if decimals != asset_config.decimals {
            return Err(ChanceryError::IssuedTokenDeploymentNotVerified.into());
        }

        let mint_authority_option = u32::from_le_bytes(
            mint_data[OFFSET_MINT_AUTHORITY_OPTION..OFFSET_MINT_AUTHORITY_OPTION + 4]
                .try_into()
                .map_err(|_| ChanceryError::Token2022TlvMalformed)?,
        );

        let expected_mint_authority = chancery_config.mint_authority_pda.to_bytes();

        if mint_authority_option != 1
            || mint_data[OFFSET_MINT_AUTHORITY..OFFSET_MINT_AUTHORITY + 32]
                != expected_mint_authority[..]
        {
            return Err(ChanceryError::IssuedTokenMintAuthorityInvalid.into());
        }

        let freeze_authority_option = u32::from_le_bytes(
            mint_data[OFFSET_FREEZE_AUTHORITY_OPTION..OFFSET_FREEZE_AUTHORITY_OPTION + 4]
                .try_into()
                .map_err(|_| ChanceryError::Token2022TlvMalformed)?,
        );

        let expected_freeze_authority = chancery_config.freeze_authority_pda.to_bytes();

        if freeze_authority_option != 1
            || mint_data[OFFSET_FREEZE_AUTHORITY..OFFSET_FREEZE_AUTHORITY + 32]
                != expected_freeze_authority[..]
        {
            return Err(ChanceryError::IssuedTokenFreezeAuthorityInvalid.into());
        }

        if let Err(error) = &parsed_observed {
            return Err(error.clone());
        }
        control.assert_observed_mint_mask_valid(observed)?;
        asset_config.assert_forbidden_extensions_absent(observed)?;
        assert_mint_extension_authorities(
            &mint_data,
            observed,
            control.reserved_mint_extension_mask,
            &expected_authorities,
        )
        .map_err(ProgramError::from)?;

        Ok(())
    })();

    let regression_error_code = validation_result
        .err()
        .map(program_error_code)
        .unwrap_or(0);

    drop(mint_data);
    drop(control);
    drop(asset_config);

    let clock = Clock::get()?;
    let verification_succeeded;
    let control_flags_after;

    {
        let mut control = IssuedTokenControl::load_mut_for_verified_pda(
            issued_token_control_account_info,
            control_bump,
        )?;

        verification_succeeded = apply_observation_result(
            &mut control,
            observed,
            clock.slot,
            regression_error_code,
        );
        control_flags_after = control.control_flags;
    }

    drop(chancery_config);

    let (event_authority_bump, sequence_nonce) = {
        let mut config = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, config_bump)?;
        (config.event_authority_bump, config.next_sequence_nonce()?)
    };

    emit_issued_token_extension_observation_refreshed(
        event_authority_account_info,
        event_authority_bump,
        IssuedTokenExtensionObservationRefreshed {
            sequence_nonce,
            chancery:                *chancery_config_account_info.key,
            slot:                    clock.slot,
            unix_timestamp:          clock.unix_timestamp,
            risk_class:              if verification_succeeded {
                ConfigChangeRiskClass::RoutineOps.as_u8()
            } else {
                ConfigChangeRiskClass::HighImpact.as_u8()
            },
            issued_token_control:    *issued_token_control_account_info.key,
            observed_extension_mask: observed,
            control_flags:           control_flags_after,
            verification_succeeded,
            regression_error_code,
            refreshed_by:            *operations_authority_account_info.key,
        },
    )?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use bytemuck::Zeroable;

    #[test]
    fn successful_refresh_updates_observation_and_sets_ready() {
        let mut control = IssuedTokenControl::zeroed();
        control.control_flags = issued_token_deployment_flag::ALL_PRE_REQUIRED;

        assert!(apply_observation_result(&mut control, [7, 9], 44, 0));
        assert_eq!(control.active_mint_extension_mask, [7, 9]);
        assert_eq!(control.extension_observed_at_slot, 44);
        assert_ne!(
            control.control_flags & issued_token_deployment_flag::READY_FOR_SETTLEMENT,
            0,
        );
    }

    #[test]
    fn regression_clears_ready_and_preserves_last_good_observation() {
        let mut control = IssuedTokenControl::zeroed();
        control.control_flags = issued_token_deployment_flag::ALL_PRE_REQUIRED
            | issued_token_deployment_flag::READY_FOR_SETTLEMENT;
        control.active_mint_extension_mask = [3, 5];
        control.extension_observed_at_slot = 17;

        assert!(!apply_observation_result(&mut control, [99, 101], 88, 0x0901));
        assert_eq!(control.active_mint_extension_mask, [3, 5]);
        assert_eq!(control.extension_observed_at_slot, 17);
        assert_eq!(
            control.control_flags & issued_token_deployment_flag::READY_FOR_SETTLEMENT,
            0,
        );
        assert_eq!(
            control.control_flags & issued_token_deployment_flag::ALL_PRE_REQUIRED,
            issued_token_deployment_flag::ALL_PRE_REQUIRED,
        );
    }

    #[test]
    fn later_success_recovers_ready_after_regression() {
        let mut control = IssuedTokenControl::zeroed();
        control.control_flags = issued_token_deployment_flag::ALL_PRE_REQUIRED
            | issued_token_deployment_flag::READY_FOR_SETTLEMENT;

        assert!(!apply_observation_result(&mut control, [1, 0], 10, 1));
        assert!(apply_observation_result(&mut control, [2, 0], 11, 0));
        assert_eq!(control.active_mint_extension_mask, [2, 0]);
        assert_eq!(control.extension_observed_at_slot, 11);
        assert_ne!(
            control.control_flags & issued_token_deployment_flag::READY_FOR_SETTLEMENT,
            0,
        );
    }
}
