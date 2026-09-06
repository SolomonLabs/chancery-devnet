/// execute_forced_burn_case
///
/// Executes a FORCED_BURN enforcement case.
///
/// Burns tokens from the subject's account using the chancery's freeze_authority
/// as the delegated burn authority. This is the most destructive action
/// available and is gated by the strongest approval chain:
///
///   open (enforcement_authority)
///   -> approve (enforcement_authority + governance_authority co-sign)
///   -> execute (enforcement_authority)
///
/// The freeze_authority PDA holds delegated burn authority on Token-2022
/// accounts. Before execution, the token account must be frozen (use
/// execute_freeze_case first if not already frozen) - this ensures no
/// in-flight transactions can race with the burn.
///
/// Spec §5.8 note: "forced-burn under separate review" - this instruction
/// is deliberately kept behind the module not-enabled gate until the
/// full legal and operational review of the forced-burn use case is
/// complete for the specific deployment.
///
/// Accounts:
///   0  chancery_config                writable   PDA (sequence_nonce for evidence)
///   1  enforcement_case               writable   PDA [b"enforcement-case", case_id]
///   2  subject_token_account          writable   must be frozen
///   3  issued_token_mint              writable   supply decreases
///   4  freeze_authority_pda           readable   PDA [b"freeze-authority"] - burn delegate
///   5  issued_token_program           readable
///   6  enforcement_authority          signer     chancery_config.enforcement_authority
///
/// Status: MODULE NOT ENABLED - pending legal + operational review per §5.8.

use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::error::ChanceryError;

pub fn handle<'a>(_accounts: &'a [AccountInfo<'a>], _args_data: &[u8]) -> ProgramResult {
    Err(ChanceryError::ModuleNotEnabled.into())
}
