use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::constants::ix::issued_token_control as ix_id;
use crate::error::ChanceryError;

pub mod initialize_issued_token_control;
pub mod refresh_asset_extension_observation;
pub mod refresh_issued_token_extension_observation;
pub mod verify_issued_token_deployment;

pub fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
    if data.is_empty() {
        return Err(ChanceryError::InstructionDataTooShort.into());
    }

    match data[0] {
        ix_id::INITIALIZE_ISSUED_TOKEN_CONTROL => {
            initialize_issued_token_control::handle(accounts, &data[1..])
        }
        ix_id::REFRESH_ASSET_EXTENSION_OBSERVATION => {
            refresh_asset_extension_observation::handle(accounts, &data[1..])
        }
        ix_id::VERIFY_ISSUED_TOKEN_DEPLOYMENT => {
            verify_issued_token_deployment::handle(accounts, &data[1..])
        }
        ix_id::REFRESH_ISSUED_TOKEN_EXTENSION_OBSERVATION => {
            refresh_issued_token_extension_observation::handle(accounts, &data[1..])
        }
        ix_id::UPDATE_ISSUED_TOKEN_CONTROL             |
        ix_id::ACTIVATE_ISSUED_TOKEN_MODULE            |
        ix_id::DEACTIVATE_ISSUED_TOKEN_MODULE          |
        ix_id::SET_TRANSFER_HOOK_PROGRAM               |
        ix_id::SET_PERMANENT_DELEGATE                  |
        ix_id::INITIALIZE_TOKEN_METADATA               |
        ix_id::UPDATE_TOKEN_METADATA                   |
        ix_id::SET_DEFAULT_ACCOUNT_STATE               |
        ix_id::SET_TOKEN_PAUSE_STATE                   |
        ix_id::CONFIGURE_CONFIDENTIAL_TRANSFER_MINT    |
        ix_id::CONFIGURE_CONFIDENTIAL_TRANSFER_ACCOUNT |
        ix_id::UPDATE_ASSET_EXTENSION_POLICY           => {
            let _ = accounts;
            Err(ChanceryError::ModuleNotEnabled.into())
        }
        _ => Err(ChanceryError::UnknownInstruction.into()),
    }
}
