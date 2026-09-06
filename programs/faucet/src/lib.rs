use solana_account_info::AccountInfo;
use solana_program_entrypoint::entrypoint;
use solana_program_error::{ProgramError, ProgramResult};
use solana_pubkey::{declare_id, Pubkey};

pub mod authority;
pub mod constants;
mod instructions;
mod token;

declare_id!("6JgE46kwvTqPpY7s2DDjFDvSqWdmfm9puoDBRqHubn85");

const _: () = {
    const SENTINEL: [u8; 32] = [0u8; 32];
    const SELECTED: [u8; 32] = ID.to_bytes();
    let mut identical = true;
    let mut index = 0;
    while index < 32 {
        if SELECTED[index] != SENTINEL[index] {
            identical = false;
        }
        index += 1;
    }
    assert!(
        !identical,
        "faucet program identity is unstamped; run yarn identity before building"
    );
};

entrypoint!(process_instruction);

pub fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    if program_id != &ID {
        return Err(ProgramError::IncorrectProgramId);
    }
    let (discriminant, payload) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    match *discriminant {
        constants::MINT_TEST_ASSET => instructions::mint_test_asset::handle(program_id, accounts, payload),
        constants::EXECUTE_CHANCERY => instructions::execute_chancery::handle(program_id, accounts, payload),
        constants::SCHEDULE_CONFIG_CHANGE => instructions::schedule_config_change::handle(program_id, accounts, payload),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

#[cfg(test)]
mod tests;
