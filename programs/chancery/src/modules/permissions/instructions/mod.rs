use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;

use crate::{constants::ix::permissions as ix_id, error::ChanceryError};

pub mod revoke_permission;
pub mod upsert_permission;
pub mod upsert_permission_with_pending_change;

pub fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
    if data.is_empty() {
        return Err(ChanceryError::InstructionDataTooShort.into());
    }

    match data[0] {
        ix_id::UPSERT_PERMISSION => upsert_permission::handle(accounts, &data[1..]),
        ix_id::REVOKE_PERMISSION => revoke_permission::handle(accounts, &data[1..]),
        ix_id::UPSERT_PERMISSION_WITH_PENDING_CHANGE => {
            upsert_permission_with_pending_change::handle(accounts, &data[1..])
        }
        _ => Err(ChanceryError::UnknownInstruction.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pending_permission_id_is_active() {
        assert_ne!(
            dispatch(&[], &[ix_id::UPSERT_PERMISSION_WITH_PENDING_CHANGE]),
            Err(ChanceryError::ModuleNotEnabled.into()),
        );
    }

    #[test]
    fn unknown_id_still_reports_unknown() {
        assert_eq!(dispatch(&[], &[0xFF]), Err(ChanceryError::UnknownInstruction.into()));
    }
}
