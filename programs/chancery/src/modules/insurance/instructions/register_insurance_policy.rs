/// register_insurance_policy
///
/// Creates an InsurancePolicyPda anchoring an external commercial
/// insurance reference to a chancery pathway.
///
/// This is a REFERENCE record only - it does not create any on-chain
/// payment mechanism, holder rights, or yield promise (spec 05 §5.12).
///
/// Accounts:
///   0  chancery_config            readable  PDA
///   1  insurance_policy           writable  PDA [b"insurance-policy", insurance_policy_id]
///   2  payer                      signer
///   3  operations_authority       signer    chancery_config.operations_authority
///   4  system_program
///
/// Status: MODULE NOT ENABLED

use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::error::ChanceryError;

pub fn handle<'a>(_accounts: &'a [AccountInfo<'a>], _args_data: &[u8]) -> ProgramResult {
    Err(ChanceryError::ModuleNotEnabled.into())
}
