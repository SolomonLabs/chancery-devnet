use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::constants::ix::limits as ix_id;
use crate::error::ChanceryError;

pub mod register_limit_policy;
pub mod update_limit_policy;
pub mod update_limit_policy_with_pending_change;

pub fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
    if data.is_empty() {
        return Err(ChanceryError::InstructionDataTooShort.into());
    }

    match data[0] {
        ix_id::REGISTER_LIMIT_POLICY => register_limit_policy::handle(accounts, &data[1..]),
        ix_id::UPDATE_LIMIT_POLICY   => update_limit_policy::handle(accounts, &data[1..]),
        ix_id::UPDATE_LIMIT_POLICY_WITH_PENDING_CHANGE => {
            update_limit_policy_with_pending_change::handle(accounts, &data[1..])
        }
        _ => Err(ChanceryError::UnknownInstruction.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_id_still_reports_unknown() {
        assert_eq!(dispatch(&[], &[0xFF]), Err(ChanceryError::UnknownInstruction.into()));
    }
}
