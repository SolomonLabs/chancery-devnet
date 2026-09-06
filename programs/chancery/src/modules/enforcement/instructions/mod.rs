use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;

use crate::{constants::ix::enforcement as ix_id, error::ChanceryError};

pub mod approve_enforcement_case;
pub mod execute_forced_burn_case;
pub mod execute_freeze_case;
pub mod execute_thaw_case;
pub mod open_enforcement_case;

pub fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
    if data.is_empty() {
        return Err(ChanceryError::InstructionDataTooShort.into());
    }

    match data[0] {
        ix_id::APPROVE_ENFORCEMENT_CASE => approve_enforcement_case::handle(accounts, &data[1..]),
        ix_id::EXECUTE_FORCED_BURN_CASE => execute_forced_burn_case::handle(accounts, &data[1..]),
        ix_id::EXECUTE_FREEZE_CASE      => execute_freeze_case::handle(accounts, &data[1..]),
        ix_id::EXECUTE_THAW_CASE        => execute_thaw_case::handle(accounts, &data[1..]),
        ix_id::OPEN_ENFORCEMENT_CASE    => open_enforcement_case::handle(accounts, &data[1..]),
        _                               => Err(ChanceryError::UnknownInstruction.into()),
    }
}
