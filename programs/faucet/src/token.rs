use chancery::constants::token_program;
use solana_account_info::AccountInfo;
use solana_program_error::{ProgramError, ProgramResult};
use solana_pubkey::Pubkey;

use crate::constants::MAXIMUM_WHOLE_TOKENS_PER_CALL;

const MINT_BASE_LENGTH: usize = 82;

pub fn mint_decimals(data: &[u8], expected_authority: &Pubkey) -> Result<u8, ProgramError> {
    if data.len() < MINT_BASE_LENGTH {
        return Err(ProgramError::InvalidAccountData);
    }
    if data[45] != 1 {
        return Err(ProgramError::UninitializedAccount);
    }
    if data[..4] != [1, 0, 0, 0] || data[4..36] != expected_authority.to_bytes() {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(data[44])
}

pub fn maximum_base_units(decimals: u8) -> Result<u64, ProgramError> {
    let scale = 10u64.checked_pow(u32::from(decimals))
        .ok_or(ProgramError::InvalidAccountData)?;
    MAXIMUM_WHOLE_TOKENS_PER_CALL.checked_mul(scale)
        .ok_or(ProgramError::InvalidAccountData)
}

pub fn validate_token_program(program: &AccountInfo) -> ProgramResult {
    if !program.executable || !token_program::is_accepted(program.key) {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}
