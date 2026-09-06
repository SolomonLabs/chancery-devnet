use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::modules::ChanceryModule;

pub mod auth;
pub mod change_detection;
pub mod dangerous_roles;
pub mod instructions;
pub mod mutation;
pub mod state;

#[cfg(test)]
pub mod tests;

pub struct Module;

impl ChanceryModule for Module {
    const MODULE_ID: u8 = crate::constants::module::PERMISSIONS;

    fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
        crate::modules::control::auth::dispatch_gated(
            Self::MODULE_ID,
            accounts,
            data,
            instructions::dispatch,
        )
    }
}
