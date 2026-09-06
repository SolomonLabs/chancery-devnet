//! Token-2022 / SPL Token CPI helpers for freeze/thaw.
//!
//! Control-plane CPI helpers, kept separate from settlement CPIs.
//! Freeze/thaw signed via the chancery freeze_authority PDA.
//!
//! SPL Token instruction tags:
//!   FreezeAccount = 10
//!   ThawAccount   = 11
//! Token-2022 reuses the same wire format for these.

use solana_account_info::AccountInfo;
use solana_cpi::invoke_signed;
use solana_instruction::{AccountMeta, Instruction};
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::{
    constants::{seeds, token_program},
    error::ChanceryError,
};

const TOKEN_IX_FREEZE_ACCOUNT: u8 = 10;
const TOKEN_IX_THAW_ACCOUNT:   u8 = 11;

/// Freeze a token account using the chancery freeze_authority PDA.
pub(super) fn cpi_freeze_account<'a>(
    token_program_account_info:    &AccountInfo<'a>,
    token_account_info:            &AccountInfo<'a>,
    mint_account_info:             &AccountInfo<'a>,
    freeze_authority_account_info: &AccountInfo<'a>,
    freeze_authority_bump:         u8,
) -> ProgramResult {
    if token_program_account_info.key != &token_program::TOKEN_2022 {
        return Err(ChanceryError::TokenProgramMismatch.into());
    }

    let ix = Instruction {
        program_id: *token_program_account_info.key,
        accounts: vec![
            AccountMeta::new(*token_account_info.key, false),
            AccountMeta::new_readonly(*mint_account_info.key, false),
            AccountMeta::new_readonly(*freeze_authority_account_info.key, true),
        ],
        data: vec![TOKEN_IX_FREEZE_ACCOUNT],
    };

    invoke_signed(
        &ix,
        &[
            token_account_info.clone(),
            mint_account_info.clone(),
            freeze_authority_account_info.clone(),
            token_program_account_info.clone(),
        ],
        &[&[seeds::FREEZE_AUTHORITY, &[freeze_authority_bump]]],
    )
}

/// Thaw a token account using the chancery freeze_authority PDA.
pub(super) fn cpi_thaw_account<'a>(
    token_program_account_info:    &AccountInfo<'a>,
    token_account_info:            &AccountInfo<'a>,
    mint_account_info:             &AccountInfo<'a>,
    freeze_authority_account_info: &AccountInfo<'a>,
    freeze_authority_bump:         u8,
) -> ProgramResult {
    if token_program_account_info.key != &token_program::TOKEN_2022 {
        return Err(ChanceryError::TokenProgramMismatch.into());
    }

    let ix = Instruction {
        program_id: *token_program_account_info.key,
        accounts: vec![
            AccountMeta::new(*token_account_info.key, false),
            AccountMeta::new_readonly(*mint_account_info.key, false),
            AccountMeta::new_readonly(*freeze_authority_account_info.key, true),
        ],
        data: vec![TOKEN_IX_THAW_ACCOUNT],
    };

    invoke_signed(
        &ix,
        &[
            token_account_info.clone(),
            mint_account_info.clone(),
            freeze_authority_account_info.clone(),
            token_program_account_info.clone(),
        ],
        &[&[seeds::FREEZE_AUTHORITY, &[freeze_authority_bump]]],
    )
}

/// Cached PDA bump for the chancery freeze_authority. Derive once at handler
/// entry; re-deriving every CPI is wasteful but correct.
#[inline]
pub fn freeze_authority_bump(program_id: &Pubkey) -> u8 {
    let (_, bump) = Pubkey::find_program_address(&[seeds::FREEZE_AUTHORITY], program_id);

    bump
}
