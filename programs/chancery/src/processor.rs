use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_msg::msg;
use solana_pubkey::Pubkey;

use crate::{
    constants::{WIRE_IX_OFFSET, WIRE_MINIMUM_LEN, WIRE_MODULE_OFFSET},
    error::ChanceryError,
    modules,
};

/// Top-level instruction processor.
///
/// Wire format:
///   byte 0  - module_id  (routes to module)
///   byte 1  - ix_id      (routes within module)
///   byte 2+ - borsh-serialised instruction args
///
/// Responsibilities here are minimal by design:
///   1. reject undersized payloads
///   2. extract module_id
///   3. forward [ix_id, ...args] slice to the module registry
///
/// All account validation, permission checks, and economic logic live
/// inside the target module's instruction handler.
pub fn process_instruction<'a>(
    _program_id: &Pubkey,
    accounts:    &'a [AccountInfo<'a>],
    data:        &[u8],
) -> ProgramResult {
    if data.len() < WIRE_MINIMUM_LEN {
        msg!(
            "chancery: instruction data too short ({} bytes, need >= {})",
            data.len(),
            WIRE_MINIMUM_LEN,
        );

        return Err(ChanceryError::InstructionDataTooShort.into());
    }

    let module_id           = data[WIRE_MODULE_OFFSET];
    let instruction_payload = &data[WIRE_IX_OFFSET..];  // [ix_id, ...args]

    modules::dispatch(module_id, accounts, instruction_payload)
}
