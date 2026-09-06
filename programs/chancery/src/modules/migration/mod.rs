use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::modules::ChanceryModule;

pub mod instructions;
pub mod state;

#[cfg(test)]
pub mod tests;

pub struct Module;

impl ChanceryModule for Module {
    const MODULE_ID: u8 = crate::constants::module::MIGRATION;

    fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
        crate::modules::control::auth::dispatch_gated(
            Self::MODULE_ID,
            accounts,
            data,
            instructions::dispatch,
        )
    }
}
