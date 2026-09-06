/// open_enforcement_case
///
/// Opens a bounded enforcement case against a specific subject and asset.
/// The case remains in OPEN status until a separate `approve_enforcement_case`
/// call promotes it to APPROVED. No action is taken at open time.
///
/// Spec §5.8 - design constraints:
///   - Bounded: case specifies exactly one subject, one asset, one action.
///   - Separate from normal settlement - no pathway required.
///   - Fully auditable: reason code + evidence hash committed on-chain.
///   - Does not grant god-mode access - execution requires APPROVED status.
///
/// Accounts:
///   0  chancery_config             readable  PDA
///   1  enforcement_case            writable  PDA [b"enforcement-case", case_id]
///   2  payer                       signer    funds rent
///   3  enforcement_authority       signer    chancery_config.enforcement_authority
///   4  system_program
///
/// Status: MODULE NOT ENABLED

use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::error::ChanceryError;

pub fn handle<'a>(_accounts: &'a [AccountInfo<'a>], _args_data: &[u8]) -> ProgramResult {
    Err(ChanceryError::ModuleNotEnabled.into())
}
