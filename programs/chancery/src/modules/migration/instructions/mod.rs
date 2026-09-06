use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;

use crate::{constants::ix::migration as ix_id, error::ChanceryError};

pub mod enable_legacy_migration;
pub mod migrate_legacy_to_token2022;

pub fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
    if data.is_empty() {
        return Err(ChanceryError::InstructionDataTooShort.into());
    }

    match data[0] {
        ix_id::ENABLE_LEGACY_MIGRATION      => enable_legacy_migration::handle(accounts, &data[1..]),
        ix_id::MIGRATE_LEGACY_TO_TOKEN2022  => migrate_legacy_to_token2022::handle(accounts, &data[1..]),
        _                                   => Err(ChanceryError::UnknownInstruction.into()),
    }
}
