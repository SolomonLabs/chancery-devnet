//! Cross-chain usage-window scope conventions.
//!
//! The generic `UsageWindow` PDA is keyed by `(scope_hash, window_kind,
//! window_start)` where `scope_hash = sha256(scope_kind_byte || scope_key)`.
//! This module pins the convention for **remote-domain-scoped** windows so
//! both Solana handlers and off-chain callers (TS client, audit tools)
//! derive the same PDA.
//!
//! The remote-domain window tracks corridor-level volume:
//!
//!   - `record_outflow(amount)` after a successful `emit_outbound_message`
//!     (issued tokens burned, canonical message dispatched).
//!   - `record_inflow(amount)` after a successful `consume_inbound_message`
//!     (issued tokens minted to recipient).
//!
//! The cap on either direction is `RemoteDomainPolicy.per_day_maximum`. The
//! accumulator is a fixed UTC-day bucket, not a sliding 24-hour window. Both
//! directions share the same cap value but enforce against their respective
//! `gross_in` / `gross_output_amount` accumulators.
//!
//! NB: `RemoteDomainPolicy.per_day_maximum == 0` means "no cap" (consistent
//! with the policy's `assert_per_day_within_cap` early-return semantics).

use solana_pubkey::Pubkey;
use solana_sha256_hasher::hashv as sha256_hashv;

use crate::constants::scope;

/// Scope-kind byte for the remote-domain usage window. Combined with the
/// 32-byte `RemoteDomainPolicy` PDA via SHA-256 to produce the
/// `UsageWindow.scope_hash` seed component.
///
/// Kept as a stable public constant - changing it is a wire-format break
/// (every existing remote-domain window PDA stops resolving). The TS client
/// mirrors this value at `USAGE_WINDOW_SCOPE_KIND.REMOTE_DOMAIN`.
pub const REMOTE_DOMAIN_USAGE_SCOPE_KIND: u8 = scope::REMOTE_DOMAIN;

/// Compute the `scope_hash` for a `UsageWindow` keyed to the given
/// `RemoteDomainPolicy` PDA. Same value off-chain and on-chain.
pub fn remote_domain_usage_window_scope_hash(
    remote_domain_policy_pda: &Pubkey,
) -> [u8; 32] {
    sha256_hashv(&[
        &[REMOTE_DOMAIN_USAGE_SCOPE_KIND],
        remote_domain_policy_pda.as_ref(),
    ])
    .to_bytes()
}
