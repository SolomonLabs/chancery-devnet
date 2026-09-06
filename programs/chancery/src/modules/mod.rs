use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;

use crate::{
    constants::module,
    error::ChanceryError,
};

// ── Sub-modules ───────────────────────────────────────────────────────────────
//
// Declaration order matches module ID layout (see constants::module).
// Three tiers: infrastructure, MVP, later.

// ── Infrastructure ──
pub mod core;
pub mod events_cpi;

// ── MVP ──
pub mod control;
pub mod evidence;
pub mod fees;
pub mod issuance;
pub mod limits;
pub mod migration;
pub mod permissions;
pub mod pathway;
pub mod reserve;
pub mod settlement;

// ── Later modules ── (stripped from MVP audit surface; re-enable when ready)
// pub mod compartments;
// pub mod enforcement;
// pub mod insurance;
// pub mod provenance;

/// Cross-chain mint/redeem (spec 10). Fully wired: admin handlers
/// (register/update domain policy, register/rotate signer set,
/// restrict/relax pause) and hot-path consume/emit handlers. Defaults
/// DISABLED via module activation state; ships disabled for launch.
pub mod cross_chain;

// ─── Trait ───────────────────────────────────────────────────────────────────

/// Every module implements this trait.
///
/// Wire layout entering `dispatch`:
///   `data[0]` = ix_id (u8)
///   `data[1..]` = borsh-serialised instruction args
///
/// The `MODULE_ID` constant is informational only; the registry below
/// is the authoritative routing table.
pub trait ChanceryModule {
    const MODULE_ID: u8;

    fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult;
}

// ─── Registry ────────────────────────────────────────────────────────────────

/// Route `[module_id, ix_id, ...args]` to the correct module.
///
/// Called exclusively from `processor::process_instruction` after the
/// two-byte header has been validated.
///
/// `module_id` = `data[0]` (already extracted by the processor).
/// `instruction_payload`      = `data[1..]` i.e. `[ix_id, ...args]`.
pub fn dispatch<'a>(
    module_id:           u8,
    accounts:            &'a [AccountInfo<'a>],
    instruction_payload: &[u8],
) -> ProgramResult {
    match module_id {
        // Infrastructure
        module::CORE                 => core::Module::dispatch(accounts, instruction_payload),
        module::EVENTS_CPI           => events_cpi::Module::dispatch(accounts, instruction_payload),

        // MVP
        module::PERMISSIONS          => permissions::Module::dispatch(accounts, instruction_payload),
        module::PATHWAY              => pathway::Module::dispatch(accounts, instruction_payload),
        module::SETTLEMENT           => settlement::Module::dispatch(accounts, instruction_payload),
        module::LIMITS               => limits::Module::dispatch(accounts, instruction_payload),
        module::EVIDENCE             => evidence::Module::dispatch(accounts, instruction_payload),
        module::FEES                 => fees::Module::dispatch(accounts, instruction_payload),
        module::RESERVE              => reserve::Module::dispatch(accounts, instruction_payload),
        module::CONTROL              => control::Module::dispatch(accounts, instruction_payload),
        module::MIGRATION            => migration::Module::dispatch(accounts, instruction_payload),
        module::ISSUED_TOKEN_CONTROL => issuance::Module::dispatch(accounts, instruction_payload),

        // Cross-chain MVP module.
        module::CROSS_CHAIN          => cross_chain::Module::dispatch(accounts, instruction_payload),

        // Later - reserved IDs that fail closed until implemented
        module::COMPARTMENTS         |
        module::PROVENANCE           |
        module::INSURANCE            |
        module::ENFORCEMENT          => Err(ChanceryError::UnknownModule.into()),
        _                            => Err(ChanceryError::UnknownModule.into()),
    }
}
