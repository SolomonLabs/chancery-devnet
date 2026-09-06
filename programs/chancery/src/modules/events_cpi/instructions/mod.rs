use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;

use crate::{constants::ix::events_cpi as ix_id, error::ChanceryError};

pub mod emit;

pub fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
    if data.is_empty() {
        return Err(ChanceryError::InstructionDataTooShort.into());
    }

    match data[0] {
        ix_id::EMIT => emit::handle(accounts, &data[1..]),
        _           => Err(ChanceryError::UnknownInstruction.into()),
    }
}
