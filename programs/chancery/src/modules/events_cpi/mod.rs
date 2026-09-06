//! events_cpi - inner-instruction-based event channel.
//!
//! This module exists to receive self-CPIs from `evidence::event::emit_event`.
//! Its only instruction is `EMIT`, a no-op handler whose presence in the
//! transaction's inner-instruction tree IS the evidence record.
//!
//! Off-chain decoders read `meta.innerInstructions` from `getTransaction` and
//! look for instructions where:
//!   - `programIdIndex` resolves to chancery's program ID, AND
//!   - `data[0] == module::EVENTS_CPI` AND `data[1] == ix::events_cpi::EMIT`
//!
//! The remaining bytes (`data[2..]`) are `[ discriminator(8) | borsh_payload ]`
//! for the event, decoded via the same scheme as `evidence::events::*`.
//!
//! Trust model: `instructions::emit::handle` requires the canonical
//! `event-authority` PDA to be a signer. Only Chancery can produce that
//! signature through signed self-CPI, so directly submitted external EMIT
//! instructions are rejected. Indexers must also require a complete finalized
//! transaction response with present metadata and `meta.err` present and exactly
//! null before inspecting inner instructions. The signed CPI proves provenance;
//! the successful transaction envelope proves commitment.

use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;

use crate::modules::ChanceryModule;

pub mod instructions;

pub struct Module;

impl ChanceryModule for Module {
    const MODULE_ID: u8 = crate::constants::module::EVENTS_CPI;

    fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
        instructions::dispatch(accounts, data)
    }
}
