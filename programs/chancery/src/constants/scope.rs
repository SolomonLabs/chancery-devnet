//! Permission scope kinds - u8 discriminant on `Permission.scope_kind`.
//!
//! Append-only.

pub mod scope {
    pub const GLOBAL:                 u8 = 0x00;
    pub const ASSET:                  u8 = 0x01;
    pub const PATHWAY:                u8 = 0x02;
    pub const DESTINATION:            u8 = 0x03;
    pub const EXECUTOR:               u8 = 0x04;
    pub const MIGRATION:              u8 = 0x05;
    pub const ENFORCEMENT:            u8 = 0x06;

    /// Cross-chain remote domain (scope_key = sha256(chain_kind || domain_id_be)).
    pub const REMOTE_DOMAIN:          u8 = 0x07;

    // ── ──
    pub const ISSUED_TOKEN_CONTROL:   u8 = 0x08;
    pub const MODULE:                 u8 = 0x09;
    pub const CROSS_CHAIN_SIGNER_SET: u8 = 0x0A;
    pub const TOKEN_ACCOUNT:          u8 = 0x0B;

    /// Per-counterparty limit dimension (scope_key = Pubkey::default() on the
    /// template LimitPolicy; usage windows are keyed per counterparty key).
    pub const COUNTERPARTY:           u8 = 0x0C;
}

/// Extension additions to permission scope kinds. Exposed under a separate
/// module so call sites can disambiguate baseline scope kinds from extension
/// scope kinds while sharing the same on-wire u8 namespace.
pub mod scope_extensions {
    pub use super::scope::{
        COUNTERPARTY, CROSS_CHAIN_SIGNER_SET, ISSUED_TOKEN_CONTROL, MODULE,
        TOKEN_ACCOUNT,
    };
}
