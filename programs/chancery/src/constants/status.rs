//! Status enums and pending-config-change kind discriminants.
//!
//! All `*_status` modules use a u8 status byte instead of bit-encoded flags
//! for clarity. `change_kind` uses u16 because the discriminant space is
//! shared across every governable handler.

// ─── Asset modes (u8) ────────────────────────────────────────────────────────
pub mod asset_mode {
    pub const ACTIVE:    u8 = 0x00;
    pub const WIND_DOWN: u8 = 0x01;
    pub const FROZEN:    u8 = 0x02;
}

// ─── Settlement intent status (u8) ───────────────────────────────────────────
pub mod intent_status {
    pub const PENDING:   u8 = 0x00;
    pub const EXECUTED:  u8 = 0x01;
    pub const CANCELLED: u8 = 0x02;
    pub const EXPIRED:   u8 = 0x03;
}

// ─── Provenance / enforcement case status (u8) ───────────────────────────────
pub mod case_status {
    pub const OPEN:     u8 = 0x00;
    pub const APPROVED: u8 = 0x01;
    pub const CLOSED:   u8 = 0x02;
    pub const REJECTED: u8 = 0x03;
}

// ─── Basic freeze record status ────────────────────────────────────
pub mod basic_freeze_status {
    pub const NONE:   u8 = 0;
    pub const FROZEN: u8 = 1;
    pub const THAWED: u8 = 2;
}

// ─── Pending config change status ──────────────────────────────────
pub mod config_change_status {
    pub const NONE:      u8 = 0;
    pub const PROPOSED:  u8 = 1;
    pub const ACCEPTED:  u8 = 2;
    pub const CANCELLED: u8 = 3;
    pub const CONSUMED:  u8 = 4;
}

// ─── Pathway status ────────────────────────────────────────────────
pub mod pathway_status {
    pub const NONE:               u8 = 0;
    pub const DISABLED:           u8 = 1;
    pub const ACTIVE:             u8 = 2;
    pub const EMERGENCY_DISABLED: u8 = 3;
    pub const DEPRECATED:         u8 = 4;
}

// ─── Reserve destination status ────────────────────────────────────
pub mod reserve_destination_status {
    pub const NONE:       u8 = 0;
    pub const DISABLED:   u8 = 1;
    pub const ENABLED:    u8 = 2;
    pub const DEPRECATED: u8 = 3;
}

// ─── Change-kind discriminants ─────────────────────────────────────
//
// Each pending config change pins to a u16 change_kind that uniquely
// identifies its target handler. Append-only.
pub mod change_kind {
    pub const UPDATE_LIMIT_POLICY:             u16 = 0x0001;
    pub const UPDATE_FEE_POLICY:               u16 = 0x0002;
    pub const UPDATE_PATHWAY_POLICY:           u16 = 0x0003;
    pub const SET_PATHWAY_STATUS:              u16 = 0x0004;
    pub const UPSERT_PERMISSION:               u16 = 0x0005;
    pub const REGISTER_RESERVE_DESTINATION:    u16 = 0x0006;
    pub const SET_RESERVE_DESTINATION_STATUS:  u16 = 0x0007;
    pub const UPDATE_ISSUED_TOKEN_CONTROL:     u16 = 0x0008;
    pub const ACTIVATE_ISSUED_TOKEN_MODULE:    u16 = 0x0009;
    pub const SET_TRANSFER_HOOK_PROGRAM:       u16 = 0x000A;
    pub const SET_PERMANENT_DELEGATE:          u16 = 0x000B;
    pub const CONFIGURE_CONFIDENTIAL_TRANSFER: u16 = 0x000C;
    pub const UPDATE_REMOTE_DOMAIN_POLICY:     u16 = 0x000D;
    pub const ROTATE_CROSS_CHAIN_SIGNER_SET:   u16 = 0x000E;
    pub const SET_MODULE_STATUS:               u16 = 0x000F;
    pub const UPDATE_ASSET_CONFIG:             u16 = 0x0010;
    pub const SET_ASSET_MODE:                  u16 = 0x0011;
    pub const REGISTER_CROSS_CHAIN_SIGNER_SET: u16 = 0x0012;
    pub const RELAX_REMOTE_DOMAIN_PAUSE:       u16 = 0x0013;
    pub const UPDATE_EVIDENCE_POLICY:          u16 = 0x0014;
}
