use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::modules::ChanceryModule;

pub mod change_detection;
pub mod emit;
pub mod event;
pub mod instructions;
pub mod state;

#[cfg(test)]
pub mod tests;

pub struct Module;

impl ChanceryModule for Module {
    const MODULE_ID: u8 = crate::constants::module::EVIDENCE;

    fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
        instructions::dispatch(accounts, data)
    }
}
