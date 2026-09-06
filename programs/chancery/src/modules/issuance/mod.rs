use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::{
    constants::ix::issued_token_control as ix_id,
    modules::ChanceryModule,
};

pub mod instructions;
pub mod state;
pub mod tlv;

#[cfg(test)]
pub mod tests;

pub struct Module;

impl ChanceryModule for Module {
    const MODULE_ID: u8 = crate::constants::module::ISSUED_TOKEN_CONTROL;

    fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
        if data.is_empty() {
            return Err(crate::error::ChanceryError::InstructionDataTooShort.into());
        }

        // verify_issued_token_deployment owns activation at accounts[0].
        if data[0] == ix_id::VERIFY_ISSUED_TOKEN_DEPLOYMENT {
            return instructions::verify_issued_token_deployment::handle(accounts, &data[1..]);
        }

        crate::modules::control::auth::dispatch_gated(
            Self::MODULE_ID,
            accounts,
            data,
            instructions::dispatch,
        )
    }
}
