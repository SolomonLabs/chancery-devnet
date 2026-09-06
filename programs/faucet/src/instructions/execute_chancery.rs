use chancery::{
    constants::{ix, module, token_program},
    modules::core::state::chancery_config::ChanceryConfig,
};
use solana_account_info::AccountInfo;
use solana_cpi::invoke_signed;
use solana_instruction::{AccountMeta, Instruction};
use solana_program_error::{ProgramError, ProgramResult};
use solana_pubkey::Pubkey;

use crate::{
    authority::{authority_address, validate_role_mask, ROLES},
    constants::{CHANCERY_AUTHORITY_SEED, FAUCET_AUTHORITY_SEED},
    token,
};
use super::allowed_chancery_instruction::is_allowed;

/// Header accounts: caller signer, executable Chancery program.
/// Remaining accounts retain the exact Chancery instruction ordering.
/// Payload: role-mask byte followed by the unmodified Chancery instruction.
pub fn handle<'a>(program_id: &Pubkey, accounts: &'a [AccountInfo<'a>], payload: &[u8]) -> ProgramResult {
    if accounts.len() < 3 || payload.len() < 3 {
        return Err(ProgramError::InvalidInstructionData);
    }
    if !accounts[0].is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if accounts[1].key != &chancery::id() || !accounts[1].executable {
        return Err(ProgramError::IncorrectProgramId);
    }
    let mask = payload[0];
    validate_role_mask(mask)?;
    let data = &payload[1..];
    if !is_allowed(data[0], data[1]) {
        return Err(ProgramError::InvalidInstructionData);
    }
    let forwarded = &accounts[2..];
    if data[0] == module::CORE && data[1] == ix::core::REGISTER_ASSET {
        validate_registered_mint(program_id, forwarded)?;
    }

    let mut authorities = Vec::with_capacity(ROLES.len());
    let mut seed_bytes = Vec::with_capacity(ROLES.len());
    for role in ROLES {
        if mask & (1 << role) == 0 {
            continue;
        }
        let (address, bump) = authority_address(role, program_id)?;
        if !forwarded.iter().any(|account| account.key == &address) {
            return Err(ProgramError::InvalidSeeds);
        }
        authorities.push(address);
        seed_bytes.push(([role], [bump]));
    }
    let signer_seeds: Vec<[&[u8]; 3]> = seed_bytes.iter()
        .map(|(role, bump)| [CHANCERY_AUTHORITY_SEED, role.as_slice(), bump.as_slice()])
        .collect();
    let signer_seed_slices: Vec<&[&[u8]]> = signer_seeds.iter().map(|seeds| seeds.as_slice()).collect();
    let instruction = Instruction {
        program_id: chancery::id(),
        accounts: forwarded.iter().map(|account| AccountMeta {
            pubkey: *account.key,
            is_writable: account.is_writable,
            is_signer: account.is_signer || authorities.contains(account.key),
        }).collect(),
        data: data.to_vec(),
    };
    invoke_signed(&instruction, accounts, &signer_seed_slices)
}

fn validate_registered_mint<'a>(program_id: &Pubkey, accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    if accounts.len() < 4 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let configuration = ChanceryConfig::load_verified(&accounts[0])?;
    let mint = &accounts[3];
    if !token_program::is_accepted(mint.owner) {
        return Err(ProgramError::IncorrectProgramId);
    }
    if mint.key == &configuration.issued_token_mint {
        return Ok(());
    }
    let (authority, _) = Pubkey::find_program_address(&[FAUCET_AUTHORITY_SEED], program_id);
    let decimals = token::mint_decimals(&mint.try_borrow_data()?, &authority)?;
    token::maximum_base_units(decimals)?;
    Ok(())
}
