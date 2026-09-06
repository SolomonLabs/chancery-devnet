/// open_provenance_case
///
/// Creates a ProvenanceCasePda to begin a source-of-funds or diligence
/// review on a counterparty's incoming assets.
///
/// Spec §5.5 - use cases:
///   - Suspicious source review.
///   - Enhanced diligence prior to promotion.
///   - Sanitation record before downstream custody routing.
///   - Clear audit trail for compliance.
///
/// Accounts:
///   0  chancery_config        readable  PDA
///   1  provenance_case        writable  PDA [b"provenance-case", case_id]
///   2  payer                  signer    funds rent
///   3  operations_authority   signer    chancery_config.operations_authority or CAN_OPEN_PROVENANCE_CASE
///   4  system_program
///
/// Status: MODULE NOT ENABLED

use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::error::ChanceryError;

pub fn handle<'a>(_accounts: &'a [AccountInfo<'a>], _args_data: &[u8]) -> ProgramResult {
    Err(ChanceryError::ModuleNotEnabled.into())
}
