/// execute_thaw_case
///
/// Executes a THAW_TOKEN_ACCOUNT enforcement case.
///
/// Thaws a previously frozen token account via the chancery's freeze_authority PDA.
/// Mirrors execute_freeze_case exactly, except the CPI calls
/// token_program::thaw_account instead of freeze_account.
///
/// Guards:
///   - case.status == APPROVED
///   - case.action_kind == THAW_TOKEN_ACCOUNT
///   - case.subject matches the token account owner
///
/// CPI: token_program::thaw_account(token_account, mint, freeze_authority_pda)
///
/// Accounts:
///   0  chancery_config                readable
///   1  enforcement_case               writable   PDA [b"enforcement-case", case_id]
///   2  subject_token_account          writable
///   3  issued_token_mint              readable
///   4  freeze_authority_pda           readable   PDA [b"freeze-authority"]
///   5  issued_token_program           readable
///   6  enforcement_authority          signer
///
/// Status: MODULE NOT ENABLED

use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::error::ChanceryError;

pub fn handle<'a>(_accounts: &'a [AccountInfo<'a>], _args_data: &[u8]) -> ProgramResult {
    Err(ChanceryError::ModuleNotEnabled.into())
}
