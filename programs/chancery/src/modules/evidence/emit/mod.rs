//! Event emission API.
//!
//! Per-domain modules collocate the event payload struct, its
//! `ChanceryEvent` impl, and the `emit_*` wrapper. Each wrapper takes the
//! typed struct directly and delegates to `event::emit_event`, which
//! performs a signed self-CPI to `events_cpi::EMIT`.
//!
//! Convention:
//!   - `pub fn emit_<snake_case_event_name>(authority_account_info, bump, p: Event)
//!     -> ProgramResult`
//!   - Wrappers are 1-line passthroughs; they exist for discoverability
//!     and as a hook point for future per-event instrumentation.
//!
//! All sub-modules are flattened into `evidence::emit::*` so call sites
//! reference both event types and emitters from a single path:
//!     `use crate::modules::evidence::emit::{SettlementMint, emit_settlement_mint};`
//!
//! Invariant #9: evidence is emitted last in every handler, after all CPIs
//! have succeeded and after `sequence_nonce` has been incremented on
//! `ChanceryConfig`.

mod asset;
mod authority;
mod bootstrap;
mod compartment;
mod config_change;
mod cross_chain;
mod evidence_policy;
mod fee;
mod insurance;
mod issued_token;
mod limit;
mod migration;
mod module;
mod pathway;
mod pause;
mod permission;
mod provenance;
mod reserve;
mod settlement;

pub use asset::*;
pub use authority::*;
pub use bootstrap::*;
pub use compartment::*;
pub use config_change::*;
pub use cross_chain::*;
pub use evidence_policy::*;
pub use fee::*;
pub use insurance::*;
pub use issued_token::*;
pub use limit::*;
pub use migration::*;
pub use module::*;
pub use pathway::*;
pub use pause::*;
pub use permission::*;
pub use provenance::*;
pub use reserve::*;
pub use settlement::*;
