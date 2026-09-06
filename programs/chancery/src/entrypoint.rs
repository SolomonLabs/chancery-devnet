use solana_account_info::AccountInfo;
use solana_program_entrypoint::entrypoint;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::processor;

entrypoint!(process_instruction);

fn process_instruction<'a>(
    program_id:       &Pubkey,
    accounts:         &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    processor::process_instruction(program_id, accounts, instruction_data)
}
