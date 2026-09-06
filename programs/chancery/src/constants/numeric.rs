//! Numeric / size limits.

// ─── Decimals precision ──────────────────────────────────────────────────────
/// Rate fields are stored as fixed-point with 9 decimal places of precision.
pub const RATE_PRECISION_E9:            u64 = 1_000_000_000;

/// Maximum supported token decimals.
pub const MAX_TOKEN_DECIMALS:           u8 = 18;

// ─── Authority transfer ──────────────────────────────────────────────────────
/// Minimum slots that must elapse between `propose_authority_transfer` and
/// `accept_authority_transfer`. Devnet floor: one slot, the smallest value
/// that still keeps propose and accept in separate slots, so the rotation
/// path stays genuinely slot-gated while a transfer completes within a test
/// run. It provides no reaction window for off-chain monitors and carries no
/// relationship to the production floor.
pub const MINIMUM_AUTHORITY_TRANSFER_TIMELOCK_SLOTS: u64 = 1;

/// Slots after `executable_after_slot` during which an authority-transfer
/// proposal may still be accepted. Past this window the proposal is void and
/// must be re-proposed by the current governance authority, so stale
/// proposals can never linger as a latent rotation. 8 640 000 slots ≈ 30 days
/// at the current 300 ms target.
pub const AUTHORITY_TRANSFER_ACCEPTANCE_WINDOW_SLOTS: u64 = 8_640_000;

// ─── Freeform field limits ───────────────────────────────────────────────────
/// Maximum number of counterparty freeform report fields permitted by evidence
/// policy.
pub const MAXIMUM_FREEFORM_FIELD_COUNT: u16 = 32;

/// Maximum byte length of a single freeform field value hash.
pub const MAXIMUM_FREEFORM_VALUE_BYTES: u16 = 64;
