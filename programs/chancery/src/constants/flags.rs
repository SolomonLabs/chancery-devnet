//! u64 bitflag groups.
//!
//! Each submodule defines disjoint bit positions inside a single u64 field
//! on some on-chain account. Append-only - retired bits must never be reused.

// ─── Global status flag bits (u64) ───────────────────────────────────────────
pub mod status_flag {
    pub const INITIALIZED:        u64 = 1 << 0;
    // Bit 1 retired: GLOBAL_PAUSE on ChanceryConfig was non-functional (no
    // instruction ever set it). Global pause now lives exclusively on the
    // PauseState PDA. Do not reuse this bit.
    // 1 << 2 reserved - former ASSET_PAUSE (issue-73); asset pausing is the AssetPauseState PDA
    pub const PATHWAY_PAUSE:      u64 = 1 << 3;
    pub const MINT_PAUSED:        u64 = 1 << 4;
    pub const REDEEM_PAUSED:      u64 = 1 << 5;
    pub const MIGRATION_ENABLED:  u64 = 1 << 6;
    pub const MODULE_ACTIVE:      u64 = 1 << 7;
    pub const COMPARTMENT_FROZEN: u64 = 1 << 8;
}

// ─── Breach behavior ─────────────────────────────────────────────────────────
//
// Limit breaches are revert-only in the MVP: every breached cap fails the
// instruction and Solana atomicity rolls the transaction back. The previous
// `breach_action::*` bit inventory (emit-event / scoped-pause escalation) was
// stored but never executable inside the reverting transaction, so it was
// removed rather than shipped as inert policy state. A future breach design
// must run in a separate transaction (e.g. keeper-finalized breach records)
// and will reintroduce its own flag namespace; `LimitPolicy` reserves the two
// u64 slots (`_reserved_breach_flags`) for that layout.

// ─── Reserve destination purpose flags ─────────────
//
// `ReserveDestination.destination_flags` u64 holds purpose flags only after
// the specification Status moves to its own byte (see
// `reserve_destination_status`). The DISABLED bit (was 1<<4) is REMOVED.
pub mod destination_purpose_flag {
    pub const TREASURY:           u64 = 1 << 0;
    pub const DOWNSTREAM_CUSTODY: u64 = 1 << 1;
    pub const OPERATIONS:         u64 = 1 << 2;
    pub const RECOVERY:           u64 = 1 << 3;

    pub const PURPOSE_MASK: u64 = 
          TREASURY 
        | DOWNSTREAM_CUSTODY 
        | OPERATIONS 
        | RECOVERY;
}
