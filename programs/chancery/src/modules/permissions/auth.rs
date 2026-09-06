//! Canonical permission-record gates.
//!
//! Every role-consuming handler must use this module so PDA binding, subject,
//! role, scope, expiry, and paused-grant semantics cannot drift between call
//! sites.

use core::cell::{Ref, RefMut};

use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    account_security::{assert_external_identity, assert_external_signer},
    constants::{role, scope},
    error::ChanceryError,
    modules::permissions::state::permission_record::{
        PermissionRecord, PERMISSION_RECORD_SIZE,
    },
};

pub fn load_canonical_permission_record<'a>(
    permission_record_account_info: &'a AccountInfo<'a>,
    program_id:                     &Pubkey,
) -> Result<Ref<'a, PermissionRecord>, ProgramError> {
    if permission_record_account_info.owner != program_id {
        return Err(ChanceryError::AccountOwnerMismatch.into());
    }

    PermissionRecord::load_verified(permission_record_account_info)
}

pub fn load_optional_canonical_permission_record<'a>(
    permission_record_account_info: &'a AccountInfo<'a>,
    program_id:                     &Pubkey,
) -> Result<Option<PermissionRecord>, ProgramError> {
    if permission_record_account_info.data_is_empty() {
        return Ok(None);
    }

    if permission_record_account_info.data_len() != PERMISSION_RECORD_SIZE {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    if permission_record_account_info.owner != program_id {
        return Err(ChanceryError::AccountOwnerMismatch.into());
    }

    let is_uninitialized = {
        let data = permission_record_account_info.try_borrow_data()?;
        data[0..8] == [0u8; 8]
    };

    if is_uninitialized {
        return Ok(None);
    }

    Ok(Some(*load_canonical_permission_record(
        permission_record_account_info,
        program_id,
    )?))
}

pub fn load_uninitialized_permission_record_mut<'a>(
    permission_record_account_info: &'a AccountInfo<'a>,
    subject:                        &Pubkey,
    scope_kind:                     u8,
    scope_key:                      &Pubkey,
    program_id:                     &Pubkey,
) -> Result<RefMut<'a, PermissionRecord>, ProgramError> {
    if permission_record_account_info.owner != program_id {
        return Err(ChanceryError::AccountOwnerMismatch.into());
    }

    PermissionRecord::verify_pda(
        permission_record_account_info,
        subject,
        scope_kind,
        scope_key,
        program_id,
    )?;

    PermissionRecord::load_uninitialized_mut(permission_record_account_info)
}

pub fn load_canonical_permission_record_mut<'a>(
    permission_record_account_info: &'a AccountInfo<'a>,
    subject:                        &Pubkey,
    scope_kind:                     u8,
    scope_key:                      &Pubkey,
    program_id:                     &Pubkey,
) -> Result<RefMut<'a, PermissionRecord>, ProgramError> {
    if permission_record_account_info.owner != program_id {
        return Err(ChanceryError::AccountOwnerMismatch.into());
    }

    let expected_bump = PermissionRecord::verify_pda(
        permission_record_account_info,
        subject,
        scope_kind,
        scope_key,
        program_id,
    )?;
    let permission_record = PermissionRecord::load_mut_for_verified_pda(
        permission_record_account_info,
        subject,
        scope_kind,
        scope_key,
        expected_bump,
    )?;

    Ok(permission_record)
}

pub fn assert_subject_holds_role<'a>(
    subject_pubkey:                 &Pubkey,
    permission_record_account_info: &'a AccountInfo<'a>,
    required_role_mask:             u128,
    expected_scope_kind:            u8,
    expected_scope_key:             &Pubkey,
    now_unix_timestamp:             i64,
    program_id:                     &Pubkey,
) -> Result<Ref<'a, PermissionRecord>, ProgramError> {
    assert_external_identity(subject_pubkey, program_id)?;

    let permission_record = load_canonical_permission_record(
        permission_record_account_info,
        program_id,
    )?;

    if &permission_record.subject != subject_pubkey {
        return Err(ChanceryError::PermissionScopeMismatch.into());
    }

    permission_record.assert_valid(
        required_role_mask,
        expected_scope_kind,
        expected_scope_key,
        now_unix_timestamp,
    )?;

    Ok(permission_record)
}

pub fn assert_signer_holds_role<'a>(
    signer_account_info:            &'a AccountInfo<'a>,
    permission_record_account_info: &'a AccountInfo<'a>,
    required_role_mask:             u128,
    expected_scope_kind:            u8,
    expected_scope_key:             &Pubkey,
    program_id:                     &Pubkey,
) -> Result<Ref<'a, PermissionRecord>, ProgramError> {
    assert_external_signer(signer_account_info, program_id)?;

    assert_subject_holds_role(
        signer_account_info.key,
        permission_record_account_info,
        required_role_mask,
        expected_scope_kind,
        expected_scope_key,
        Clock::get()?.unix_timestamp,
        program_id,
    )
}

fn assert_can_manage_token_account<'a>(
    signer_account_info:            &'a AccountInfo<'a>,
    permission_record_account_info: &'a AccountInfo<'a>,
    token_account:                   &Pubkey,
    required_role_mask:             u128,
    program_id:                      &Pubkey,
) -> Result<Ref<'a, PermissionRecord>, ProgramError> {
    assert_external_signer(signer_account_info, program_id)?;

    let permission_record = load_canonical_permission_record(
        permission_record_account_info,
        program_id,
    )?;

    if &permission_record.subject != signer_account_info.key {
        return Err(ChanceryError::PermissionScopeMismatch.into());
    }

    permission_record.assert_role(required_role_mask)?;
    permission_record.assert_not_expired_now()?;

    if permission_record.permission_flags
        & crate::modules::permissions::state::permission_record::PERMISSION_FLAG_PAUSED
        != 0
    {
        return Err(ChanceryError::PermissionPaused.into());
    }

    if permission_record.scope_kind != scope::TOKEN_ACCOUNT {
        return Err(ChanceryError::DangerousPermissionRequiresNarrowScope.into());
    }

    if &permission_record.scope_key != token_account {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    Ok(permission_record)
}

pub fn assert_can_freeze_token_account<'a>(
    signer_account_info:            &'a AccountInfo<'a>,
    permission_record_account_info: &'a AccountInfo<'a>,
    token_account:                   &Pubkey,
    program_id:                      &Pubkey,
) -> Result<Ref<'a, PermissionRecord>, ProgramError> {
    assert_can_manage_token_account(
        signer_account_info,
        permission_record_account_info,
        token_account,
        role::CAN_FREEZE_TOKEN_ACCOUNT,
        program_id,
    )
}

pub fn assert_can_thaw_token_account<'a>(
    signer_account_info:            &'a AccountInfo<'a>,
    permission_record_account_info: &'a AccountInfo<'a>,
    token_account:                   &Pubkey,
    program_id:                      &Pubkey,
) -> Result<Ref<'a, PermissionRecord>, ProgramError> {
    assert_can_manage_token_account(
        signer_account_info,
        permission_record_account_info,
        token_account,
        role::CAN_THAW_TOKEN_ACCOUNT,
        program_id,
    )
}

pub fn assert_can_execute_settlement<'a>(
    signer_account_info:            &'a AccountInfo<'a>,
    permission_record_account_info: &'a AccountInfo<'a>,
    pathway_policy:                 &Pubkey,
    program_id:                     &Pubkey,
) -> Result<Ref<'a, PermissionRecord>, ProgramError> {
    assert_signer_holds_role(
        signer_account_info,
        permission_record_account_info,
        role::CAN_EXECUTE_SETTLEMENT,
        scope::PATHWAY,
        pathway_policy,
        program_id,
    )
}

pub fn assert_can_emit_outbound_message<'a>(
    signer_account_info:            &'a AccountInfo<'a>,
    permission_record_account_info: &'a AccountInfo<'a>,
    pathway_policy:                 &Pubkey,
    program_id:                     &Pubkey,
) -> Result<Ref<'a, PermissionRecord>, ProgramError> {
    assert_signer_holds_role(
        signer_account_info,
        permission_record_account_info,
        role::CAN_EMIT_OUTBOUND_MESSAGE,
        scope::PATHWAY,
        pathway_policy,
        program_id,
    )
}
