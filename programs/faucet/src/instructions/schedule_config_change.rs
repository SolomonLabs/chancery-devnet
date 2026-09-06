use borsh::BorshDeserialize;
use chancery::{
    constants::{authority_role, ix, module},
    modules::control::{
        change_risk::ConfigChangeRiskClass,
        instructions::propose_config_change::ProposeConfigChangeArgs,
        pending_change::minimum_timelock_seconds_for_risk,
    },
};
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_error::{ProgramError, ProgramResult};
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use super::execute_chancery;

/// Payload: governance role mask, u64 delay, u64 lifetime, Chancery proposal.
/// The proposal's two timestamps are replaced using the current on-chain clock.
pub fn handle<'a>(program_id: &Pubkey, accounts: &'a [AccountInfo<'a>], payload: &[u8]) -> ProgramResult {
    if payload.len() < 17 || payload[0] != (1 << authority_role::GOVERNANCE) {
        return Err(ProgramError::InvalidInstructionData);
    }
    let delay = u64::from_le_bytes(payload[1..9].try_into().map_err(|_| ProgramError::InvalidInstructionData)?);
    let lifetime = u64::from_le_bytes(payload[9..17].try_into().map_err(|_| ProgramError::InvalidInstructionData)?);
    let data = schedule(&payload[17..], Clock::get()?.unix_timestamp, delay, lifetime)?;
    let mut forwarded = Vec::with_capacity(data.len() + 1);
    forwarded.push(payload[0]);
    forwarded.extend_from_slice(&data);
    execute_chancery::handle(program_id, accounts, &forwarded)
}

fn schedule(data: &[u8], now: i64, delay: u64, lifetime: u64) -> Result<Vec<u8>, ProgramError> {
    if data.len() < 2 || data[0] != module::CONTROL || data[1] != ix::control::PROPOSE_CONFIG_CHANGE {
        return Err(ProgramError::InvalidInstructionData);
    }
    let proposal = ProposeConfigChangeArgs::try_from_slice(&data[2..])
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let risk = ConfigChangeRiskClass::try_from_u8(proposal.risk_class)?;
    let delay = i64::try_from(delay).map_err(|_| ProgramError::InvalidInstructionData)?;
    let lifetime = i64::try_from(lifetime).map_err(|_| ProgramError::InvalidInstructionData)?;
    if !risk.requires_timelock() || delay < minimum_timelock_seconds_for_risk(risk) || lifetime <= 0 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let executable_after = now.checked_add(delay).ok_or(ProgramError::InvalidInstructionData)?;
    let expires_at = executable_after.checked_add(lifetime).ok_or(ProgramError::InvalidInstructionData)?;
    let mut scheduled = data.to_vec();
    // The validated Borsh proposal ends in two i64 timestamps and one u64 nonce.
    let timestamp_offset = scheduled.len() - 24;
    scheduled[timestamp_offset..timestamp_offset + 8].copy_from_slice(&executable_after.to_le_bytes());
    scheduled[timestamp_offset + 8..timestamp_offset + 16].copy_from_slice(&expires_at.to_le_bytes());
    Ok(scheduled)
}

#[cfg(test)]
#[path = "../tests/schedule_config_change.rs"]
mod tests;
