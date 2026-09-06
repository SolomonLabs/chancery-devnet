use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;

use crate::{constants::ix::compartments as ix_id, error::ChanceryError};

pub mod create_reserve_compartment;
pub mod freeze_compartment;
pub mod promote_compartment_balance;

pub fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
    if data.is_empty() {
        return Err(ChanceryError::InstructionDataTooShort.into());
    }

    match data[0] {
        ix_id::CREATE_RESERVE_COMPARTMENT  => create_reserve_compartment::handle(accounts, &data[1..]),
        ix_id::FREEZE_COMPARTMENT          => freeze_compartment::handle(accounts, &data[1..]),
        ix_id::PROMOTE_COMPARTMENT_BALANCE => promote_compartment_balance::handle(accounts, &data[1..]),
        _                                  => Err(ChanceryError::UnknownInstruction.into()),
    }
}
