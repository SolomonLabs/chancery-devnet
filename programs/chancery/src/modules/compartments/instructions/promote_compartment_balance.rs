/// promote_compartment_balance
///
/// Moves funds from a source compartment (e.g. IntakeQuarantine) to a
/// target compartment (e.g. ClearedOperating) after policy approval.
///
/// Spec §5.4 - promotion flow:
///   1. Source compartment must permit promotion (policy_flag::PROMOTION_PERMITTED).
///   2. Target must be ClearedOperating (or permitted by policy).
///   3. If REQUIRE_PROVENANCE flag set on source, a closed ProvenanceCase
///      must be supplied as additional account proving approval.
///   4. CPI: token transfer from source reserve_compartment_token_account -> target reserve_compartment_token_account.
///      Both accounts are program-owned; signed via program PDA.
///
/// Accounts:
///   0  chancery_config                                readable  PDA
///   1  source_compartment                             writable  PDA [b"reserve-compartment", ...]
///   2  target_compartment                             readable  PDA [b"reserve-compartment", ...]
///   3  source_reserve_compartment_token_account       writable  program-owned
///   4  target_reserve_compartment_token_account       writable  program-owned
///   5  asset_mint                                     readable
///   6  asset_token_program                            readable
///   7  operations_authority                           signer
///   Optional 8: provenance_case                       readable  required if REQUIRE_PROVENANCE set
///
/// Status: MODULE NOT ENABLED

use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::error::ChanceryError;

pub fn handle<'a>(_accounts: &'a [AccountInfo<'a>], _args_data: &[u8]) -> ProgramResult {
    Err(ChanceryError::ModuleNotEnabled.into())
}
