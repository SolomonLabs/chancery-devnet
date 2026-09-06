//! cross_chain - controlled cross-chain mint/redeem (spec 10).
//!
//! Compiled and dispatch-wired, but default-DISABLED via module activation
//! state; the module ships disabled for launch. Governance activation through
//! the pending config-change timelock is additionally gated on the EVM
//! daughter contract and two-chain readiness evidence (spec 10 §10.12).
//!
//! Implemented scope:
//!   - state PDAs: RemoteDomainPolicy, CrossChainSignerSet,
//!     RemoteNonce, OutboundReclaimRecord (spec 15, permanent single-shot)
//!   - admin handlers: register/update domain policy, register/rotate signer
//!     set, restrict/relax pause
//!   - hot-path handlers: emit_outbound_message and consume_inbound_message
//!   - expiry/reclaim handlers (spec 15): expire_inbound_message (A) and
//!     reclaim_expired_outbound (B); daughter mirrors C/D live in spec 11
//!   - TS client builders/tests and EVM daughter-contract spec alignment
//!
//! This module is wired into the active modules registry at module id 0x10.

use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;

use crate::modules::ChanceryModule;

pub mod attestation;
pub mod asset_binding;
pub mod auth;
pub mod change_detection;
pub mod instructions;
pub mod message_hash;
pub mod reclaim_digest;
pub mod state;
pub mod usage;

#[cfg(test)]
mod tests;

pub struct Module;

impl ChanceryModule for Module {
    const MODULE_ID: u8 = crate::constants::module::CROSS_CHAIN;

    fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
        crate::modules::control::auth::dispatch_gated(
            Self::MODULE_ID,
            accounts,
            data,
            instructions::dispatch,
        )
    }
}
