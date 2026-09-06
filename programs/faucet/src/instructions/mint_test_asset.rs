use solana_account_info::AccountInfo;
use solana_cpi::invoke_signed;
use solana_instruction::{AccountMeta, Instruction};
use solana_program_error::{ProgramError, ProgramResult};
use solana_pubkey::Pubkey;

use crate::{constants::FAUCET_AUTHORITY_SEED, token};

pub fn handle(program_id: &Pubkey, accounts: &[AccountInfo], payload: &[u8]) -> ProgramResult {
    let amount = u64::from_le_bytes(payload.try_into()
        .map_err(|_| ProgramError::InvalidInstructionData)?);
    if amount == 0 || accounts.len() != 4 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let mint = &accounts[0];
    let destination = &accounts[1];
    let authority = &accounts[2];
    let token_program = &accounts[3];
    token::validate_token_program(token_program)?;
    if mint.owner != token_program.key || destination.owner != token_program.key {
        return Err(ProgramError::IncorrectProgramId);
    }
    if !mint.is_writable || !destination.is_writable {
        return Err(ProgramError::InvalidArgument);
    }
    let (expected_authority, bump) = Pubkey::find_program_address(&[FAUCET_AUTHORITY_SEED], program_id);
    if authority.key != &expected_authority {
        return Err(ProgramError::InvalidSeeds);
    }
    let decimals = token::mint_decimals(&mint.try_borrow_data()?, &expected_authority)?;
    if amount > token::maximum_base_units(decimals)? {
        return Err(ProgramError::InvalidInstructionData);
    }
    let mut data = Vec::with_capacity(10);
    data.push(14); // SPL Token MintToChecked.
    data.extend_from_slice(&amount.to_le_bytes());
    data.push(decimals);
    invoke_signed(
        &Instruction {
            program_id: *token_program.key,
            accounts: vec![
                AccountMeta::new(*mint.key, false),
                AccountMeta::new(*destination.key, false),
                AccountMeta::new_readonly(expected_authority, true),
            ],
            data,
        },
        accounts,
        &[&[FAUCET_AUTHORITY_SEED, &[bump]]],
    )
}
