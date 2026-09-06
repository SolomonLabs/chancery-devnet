use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::modules::ChanceryModule;

pub mod auth;
pub mod change_detection;
pub mod change_risk;
pub mod instructions;
pub mod module_activation_classifier;
pub mod pending_change;
pub mod pause_transition;
pub mod state;

#[cfg(test)]
pub mod tests;

pub struct Module;

impl ChanceryModule for Module {
    const MODULE_ID: u8 = crate::constants::module::CONTROL;

    fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
        instructions::dispatch(accounts, data)
    }
}
