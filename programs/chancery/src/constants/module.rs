//! Module IDs, module activation status, and the undisableable list.
//!
//! Module IDs are part of the on-chain wire format. Renumbering them is a
//! breaking change for off-chain clients and any cached transactions.

// ─── Module IDs ──────────────────────────────────────────────────────────────
//
// Three-tier layout:
//   0x00-0x01  Infrastructure (root + event sink)
//   0x02-0x0B  MVP modules
//   0x0C-0x0F  Later modules (handlers stub to UnknownModule until implemented)
//   0x10       Cross-chain MVP module
//   0x11-...   Reserved later modules
pub mod module {
    // ── Infrastructure ──
    pub const CORE:                 u8 = 0x00;
    pub const EVENTS_CPI:           u8 = 0x01;

    // ── MVP ──
    pub const PERMISSIONS:          u8 = 0x02;
    pub const PATHWAY:              u8 = 0x03;
    pub const SETTLEMENT:           u8 = 0x04;
    pub const LIMITS:               u8 = 0x05;
    pub const EVIDENCE:             u8 = 0x06;
    pub const FEES:                 u8 = 0x07;
    pub const RESERVE:              u8 = 0x08;
    pub const CONTROL:              u8 = 0x09;
    pub const MIGRATION:            u8 = 0x0A;
    pub const ISSUED_TOKEN_CONTROL: u8 = 0x0B;

    // ── Later ──
    pub const COMPARTMENTS:         u8 = 0x0C;
    pub const PROVENANCE:           u8 = 0x0D;
    pub const INSURANCE:            u8 = 0x0E;
    pub const ENFORCEMENT:          u8 = 0x0F;

    // ── Cross-chain MVP ──
    /// Cross-chain mint/redeem. Spec §10. Active module dispatch.
    pub const CROSS_CHAIN:          u8 = 0x10;
}

/// Exact module IDs compiled and dispatch-wired in this program version.
///
/// This allowlist is intentionally independent of the numeric high-water mark:
/// reserved holes must remain unaddressable until an upgrade both implements the
/// module and appends its ID here. That prevents an earlier governance action from
/// pre-populating the future module's activation slot.
pub mod compiled_modules {
    use super::module;

    pub const IDS: [u8; 13] = [
        module::CORE,
        module::EVENTS_CPI,
        module::PERMISSIONS,
        module::PATHWAY,
        module::SETTLEMENT,
        module::LIMITS,
        module::EVIDENCE,
        module::FEES,
        module::RESERVE,
        module::CONTROL,
        module::MIGRATION,
        module::ISSUED_TOKEN_CONTROL,
        module::CROSS_CHAIN,
    ];
}

// ─── Module activation status ──────────────────────────────────────
pub mod module_status {
    pub const NONE:               u8 = 0;
    pub const DISABLED:           u8 = 0;  // alias of NONE
    pub const ADMIN_ONLY:         u8 = 1;
    pub const ACTIVE:             u8 = 2;
    pub const EMERGENCY_DISABLED: u8 = 3;
    pub const DEPRECATED:         u8 = 4;
}

// ─── Modules that cannot be disabled ───────────────────────────────
//
// Disabling these would brick the program. set_module_status rejects
// transitions on these IDs.
pub mod undisableable_modules {
    use super::module;
    pub const IDS: [u8; 4] = [
        module::CORE,        // 0x00
        module::EVENTS_CPI,  // 0x01
        module::EVIDENCE,    // 0x06
        module::CONTROL,     // 0x09 (owns set_module_status itself)
    ];
}

/// Modules whose permanent removal is supported by this program version.
///
/// Every currently compiled module is infrastructure or MVP value-flow surface,
/// so none may enter the one-way `DEPRECATED` state. A future optional module
/// must be added here in the same release that wires its deprecation lifecycle.
pub mod deprecatable_modules {
    #[inline]
    pub fn contains(_module_id: u8) -> bool {
        false
    }
}
