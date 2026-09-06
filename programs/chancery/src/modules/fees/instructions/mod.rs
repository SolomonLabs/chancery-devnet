use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::constants::ix::fees as ix_id;
use crate::error::ChanceryError;

pub mod register_fee_policy;
pub mod update_fee_policy;
pub mod update_fee_policy_with_pending_change;

pub fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
    if data.is_empty() {
        return Err(ChanceryError::InstructionDataTooShort.into());
    }

    match data[0] {
        ix_id::REGISTER_FEE_POLICY => register_fee_policy::handle(accounts, &data[1..]),
        ix_id::UPDATE_FEE_POLICY   => update_fee_policy::handle(accounts, &data[1..]),
        ix_id::UPDATE_FEE_POLICY_WITH_PENDING_CHANGE => {
            update_fee_policy_with_pending_change::handle(accounts, &data[1..])
        }
        _                          => Err(ChanceryError::UnknownInstruction.into()),
    }
}
