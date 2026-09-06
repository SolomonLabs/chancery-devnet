/// execute_freeze_case
///
/// Executes a FREEZE_TOKEN_ACCOUNT enforcement case.
///
/// Freezes the subject's token account via the chancery's freeze_authority PDA.
/// Only Token-2022 issued tokens support freeze - collateral accounts
/// are frozen via their own mint's freeze authority, which the chancery
/// does not hold by default.
///
/// Guards:
///   - case.status == APPROVED
///   - case.action_kind == FREEZE_TOKEN_ACCOUNT
///   - case.subject matches the token account owner
///   - case.asset_mint matches the mint of the token account
///
/// CPI: token_program::freeze_account(token_account, mint, freeze_authority_pda)
///
/// Evidence emitted: (later - reuses existing event infrastructure)
///
/// Accounts:
///   0  chancery_config                readable   PDA (for freeze_authority derivation)
///   1  enforcement_case               writable   PDA [b"enforcement-case", case_id]
///   2  subject_token_account          writable   the account to freeze
///   3  issued_token_mint              readable   must be chancery's issued_token_mint
///   4  freeze_authority_pda           readable   PDA [b"freeze-authority"]
///   5  issued_token_program           readable
///   6  enforcement_authority          signer     chancery_config.enforcement_authority
///
/// Status: MODULE NOT ENABLED

use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::error::ChanceryError;

pub fn handle<'a>(_accounts: &'a [AccountInfo<'a>], _args_data: &[u8]) -> ProgramResult {
    Err(ChanceryError::ModuleNotEnabled.into())
}
