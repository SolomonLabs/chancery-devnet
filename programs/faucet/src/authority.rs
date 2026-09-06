use chancery::constants::authority_role;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::constants::CHANCERY_AUTHORITY_SEED;

pub const ROLES: [u8; 5] = [
    authority_role::GOVERNANCE,
    authority_role::OPS,
    authority_role::EMERGENCY,
    authority_role::ENFORCEMENT,
    authority_role::INSURANCE_ADMIN,
];
pub const ALL_ROLES_MASK: u8 = 0x1f;

pub fn validate_role_mask(mask: u8) -> Result<(), ProgramError> {
    if mask == 0 || mask & !ALL_ROLES_MASK != 0 {
        return Err(ProgramError::InvalidInstructionData);
    }
    Ok(())
}

pub fn authority_address(role: u8, program_id: &Pubkey) -> Result<(Pubkey, u8), ProgramError> {
    if !ROLES.contains(&role) {
        return Err(ProgramError::InvalidInstructionData);
    }
    Ok(Pubkey::find_program_address(&[CHANCERY_AUTHORITY_SEED, &[role]], program_id))
}
