/// approve_enforcement_case
///
/// Advances an enforcement case from OPEN to APPROVED, enabling execution.
///
/// Two-party approval model (spec 06 §6.8 - "bounded"):
///   - FREEZE / THAW: enforcement_authority approval is sufficient.
///   - FORCED_BURN:   requires BOTH governance_authority AND enforcement_authority
///                    to sign in the same transaction (strongest gate).
///
/// This dual-signature requirement for forced burns prevents any single
/// authority from unilaterally destroying holder-facing tokens.
///
/// Accounts:
///   0  chancery_config                readable  PDA
///   1  enforcement_case               writable  PDA [b"enforcement-case", case_id]
///   2  enforcement_authority          signer    chancery_config.enforcement_authority
///   Optional 3: governance_authority  signer  required only for FORCED_BURN cases
///
/// Status: MODULE NOT ENABLED

use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::error::ChanceryError;

pub fn handle<'a>(_accounts: &'a [AccountInfo<'a>], _args_data: &[u8]) -> ProgramResult {
    Err(ChanceryError::ModuleNotEnabled.into())
}
