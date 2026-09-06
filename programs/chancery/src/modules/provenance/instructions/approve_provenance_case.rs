/// approve_provenance_case
///
/// Marks a ProvenanceCasePda as APPROVED after review is complete.
/// An approved case may be supplied as a reference account when calling
/// promote_compartment_balance if the source compartment has
/// REQUIRE_PROVENANCE set in its policy_flags.
///
/// Accounts:
///   0  chancery_config        readable  PDA
///   1  provenance_case        writable  PDA [b"provenance-case", case_id]
///   2  approving_authority    signer    CAN_APPROVE_PROVENANCE_CASE or governance
///
/// Status: MODULE NOT ENABLED

use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::error::ChanceryError;

pub fn handle<'a>(_accounts: &'a [AccountInfo<'a>], _args_data: &[u8]) -> ProgramResult {
    Err(ChanceryError::ModuleNotEnabled.into())
}
