use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::constants::ix::reserve as ix_id;
use crate::error::ChanceryError;

pub mod register_reserve_destination_with_pending;
pub mod set_reserve_destination_status;
pub mod set_reserve_destination_status_with_pending;
pub mod withdraw_reserve;

pub fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
    if data.is_empty() {
        return Err(ChanceryError::InstructionDataTooShort.into());
    }

    match data[0] {
        ix_id::REGISTER_RESERVE_DESTINATION_WITH_PENDING   => register_reserve_destination_with_pending::handle(accounts, &data[1..]),
        ix_id::SET_RESERVE_DESTINATION_STATUS               => set_reserve_destination_status::handle(accounts, &data[1..]),
        ix_id::SET_RESERVE_DESTINATION_STATUS_WITH_PENDING  => set_reserve_destination_status_with_pending::handle(accounts, &data[1..]),
        ix_id::WITHDRAW_RESERVE                            => withdraw_reserve::handle(accounts, &data[1..]),

        _                                                => Err(ChanceryError::UnknownInstruction.into()),
    }
}

