/// freeze_compartment
///
/// Sets or clears the COMPARTMENT_FROZEN status bit on a ReserveCompartment.
/// A frozen compartment blocks all inflow and outflow - it is effectively
/// quarantined from all settlement routing and promotion operations.
///
/// Spec §5.7:
///   - Emergency authority or ops may freeze.
///   - Only governance or ops may unfreeze.
///
/// Accounts:
///   0  chancery_config        readable  PDA
///   1  reserve_compartment    writable  PDA [b"reserve-compartment", asset_mint, compartment_id]
///   2  authority              signer    emergency | ops | governance
///
/// Status: MODULE NOT ENABLED

use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::error::ChanceryError;

pub fn handle<'a>(_accounts: &'a [AccountInfo<'a>], _args_data: &[u8]) -> ProgramResult {
    Err(ChanceryError::ModuleNotEnabled.into())
}
