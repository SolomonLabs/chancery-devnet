use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::constants::ix::pathway as ix_id;
use crate::error::ChanceryError;

pub mod register_pathway_policy;
pub mod update_pathway_policy;
pub mod update_pathway_policy_with_pending_change;

pub fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
    if data.is_empty() {
        return Err(ChanceryError::InstructionDataTooShort.into());
    }
    match data[0] {
        ix_id::REGISTER_PATHWAY_POLICY => register_pathway_policy::handle(accounts, &data[1..]),
        ix_id::UPDATE_PATHWAY_POLICY   => update_pathway_policy::handle(accounts, &data[1..]),
        ix_id::UPDATE_PATHWAY_POLICY_WITH_PENDING_CHANGE => {
            update_pathway_policy_with_pending_change::handle(accounts, &data[1..])
        }

        // Advertised in the wire format but not yet implemented.
        ix_id::SET_PATHWAY_STATUS                     |
        ix_id::SET_PATHWAY_STATUS_WITH_PENDING_CHANGE => Err(ChanceryError::ModuleNotEnabled.into()),

        _                              => Err(ChanceryError::UnknownInstruction.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unimplemented_ids_report_module_not_enabled() {
        for id in [
            ix_id::SET_PATHWAY_STATUS,
            ix_id::SET_PATHWAY_STATUS_WITH_PENDING_CHANGE,
        ] {
            assert_eq!(dispatch(&[], &[id]), Err(ChanceryError::ModuleNotEnabled.into()));
        }
    }

    #[test]
    fn unknown_id_still_reports_unknown() {
        assert_eq!(dispatch(&[], &[0xFF]), Err(ChanceryError::UnknownInstruction.into()));
    }
}
