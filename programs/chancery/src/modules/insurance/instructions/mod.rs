use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;

use crate::{constants::ix::insurance as ix_id, error::ChanceryError};

pub mod open_insurance_claim_notice;
pub mod register_insurance_policy;
pub mod update_insurance_claim_notice_status;
pub mod update_insurance_policy;

pub fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
    if data.is_empty() {
        return Err(ChanceryError::InstructionDataTooShort.into());
    }
    match data[0] {
        ix_id::OPEN_INSURANCE_CLAIM_NOTICE          => open_insurance_claim_notice::handle(accounts, &data[1..]),
        ix_id::REGISTER_INSURANCE_POLICY            => register_insurance_policy::handle(accounts, &data[1..]),
        ix_id::UPDATE_INSURANCE_CLAIM_NOTICE_STATUS => update_insurance_claim_notice_status::handle(accounts, &data[1..]),
        ix_id::UPDATE_INSURANCE_POLICY              => update_insurance_policy::handle(accounts, &data[1..]),
        _                                           => Err(ChanceryError::UnknownInstruction.into()),
    }
}
