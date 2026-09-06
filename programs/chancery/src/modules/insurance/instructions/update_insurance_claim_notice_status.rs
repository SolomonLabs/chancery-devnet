/// update_insurance_claim_notice_status
///
/// Advances the status of an InsuranceClaimNoticePda.
/// Status transitions: OPEN -> APPROVED, OPEN -> REJECTED, APPROVED -> CLOSED.
///
/// Accounts:
///   0  chancery_config            readable  PDA
///   1  insurance_claim_notice     writable  PDA [b"insurance-claim-notice", claim_notice_id]
///   2  insurance_admin_authority  signer    chancery_config.insurance_admin_authority
///
/// Status: MODULE NOT ENABLED

use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::error::ChanceryError;

pub fn handle<'a>(_accounts: &'a [AccountInfo<'a>], _args_data: &[u8]) -> ProgramResult {
    Err(ChanceryError::ModuleNotEnabled.into())
}
