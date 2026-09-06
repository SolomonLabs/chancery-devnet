use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;

use crate::{constants::ix::provenance as ix_id, error::ChanceryError};

pub mod approve_provenance_case;
pub mod close_provenance_case;
pub mod open_provenance_case;

pub fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
    if data.is_empty() {
        return Err(ChanceryError::InstructionDataTooShort.into());
    }

    match data[0] {
        ix_id::OPEN_PROVENANCE_CASE    => open_provenance_case::handle(accounts, &data[1..]),
        ix_id::APPROVE_PROVENANCE_CASE => approve_provenance_case::handle(accounts, &data[1..]),
        ix_id::CLOSE_PROVENANCE_CASE   => close_provenance_case::handle(accounts, &data[1..]),
        _                              => Err(ChanceryError::UnknownInstruction.into()),
    }
}
