/// close_provenance_case
///
/// Closes a ProvenanceCasePda by setting its status to CLOSED.
/// Closed cases remain on-chain as an immutable audit record.
/// The account is NOT reclaimed - closure is a status update only.
///
/// Accounts:
///   0  chancery_config            readable  PDA
///   1  provenance_case            writable  PDA [b"provenance-case", case_id]
///   2  operations_authority       signer    chancery_config.operations_authority
///
/// Status: MODULE NOT ENABLED

use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::error::ChanceryError;

pub fn handle<'a>(_accounts: &'a [AccountInfo<'a>], _args_data: &[u8]) -> ProgramResult {
    Err(ChanceryError::ModuleNotEnabled.into())
}
