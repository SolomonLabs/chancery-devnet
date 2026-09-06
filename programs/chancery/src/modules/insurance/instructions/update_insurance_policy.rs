/// update_insurance_policy
///
/// Updates mutable fields on an existing InsurancePolicyPda.
///
/// Accounts:
///   0  chancery_config        readable  PDA
///   1  insurance_policy       writable  PDA [b"insurance-policy", insurance_policy_id]
///   2  operations_authority   signer
///
/// Status: MODULE NOT ENABLED

use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::error::ChanceryError;

pub fn handle<'a>(_accounts: &'a [AccountInfo<'a>], _args_data: &[u8]) -> ProgramResult {
    Err(ChanceryError::ModuleNotEnabled.into())
}
