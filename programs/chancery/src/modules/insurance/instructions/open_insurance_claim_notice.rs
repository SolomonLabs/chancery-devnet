/// open_insurance_claim_notice
///
/// Anchors an insurance claim notice on-chain by creating an
/// InsuranceClaimNoticePda with hashed off-chain documentation.
///
/// Does NOT trigger any payment or fund movement.
/// Does NOT create holder rights or yield.
///
/// Accounts:
///   0  chancery_config              readable  PDA
///   1  insurance_policy             readable  PDA [b"insurance-policy", insurance_policy_id]
///   2  insurance_claim_notice       writable  PDA [b"insurance-claim-notice", claim_notice_id]
///   3  payer                        signer
///   4  insurance_admin_authority    signer    chancery_config.insurance_admin_authority
///   5  system_program
///
/// Status: MODULE NOT ENABLED

use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::error::ChanceryError;

pub fn handle<'a>(_accounts: &'a [AccountInfo<'a>], _args_data: &[u8]) -> ProgramResult {
    Err(ChanceryError::ModuleNotEnabled.into())
}
