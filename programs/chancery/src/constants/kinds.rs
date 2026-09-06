//! Kind / mode enums - u8 discriminants that describe "what variety" rather
//! than "what state".

// ─── Settlement modes (u8) ───────────────────────────────────────────────────
pub mod settlement_mode {
    pub const DIRECT_PRINCIPAL:        u8 = 0x00;
    pub const DELEGATED_NON_CUSTODIAL: u8 = 0x01;
    pub const TRILATERAL_ATOMIC:       u8 = 0x02;
}

// ─── Settlement actions (u8) ────────────────────────────────────────────────
pub mod settlement_action {
    pub const MINT:   u8 = 0x00;
    pub const REDEEM: u8 = 0x01;
}

// ─── Pathway kinds (u8) ──────────────────────────────────────────────────────
pub mod pathway_kind {
    pub const DIRECT:             u8 = 0x00;
    pub const DELEGATED:          u8 = 0x01;
    pub const TRILATERAL:         u8 = 0x02;

    /// Inbound: remote burn → local mint.
    pub const CROSS_CHAIN_MINT:   u8 = 0x03;
    
    /// Outbound: local burn → remote mint or release.
    pub const CROSS_CHAIN_REDEEM: u8 = 0x04;
}

// ─── Compartment kinds (u8) ──────────────────────────────────────────────────
pub mod compartment_kind {
    pub const INTAKE_QUARANTINE:       u8 = 0x00;
    pub const CLEARED_OPERATING:       u8 = 0x01;
    pub const RESTRICTED_REVIEW:       u8 = 0x02;
    pub const RECOVERY_OR_SEIZURE:     u8 = 0x03;
    pub const DOWNSTREAM_CUSTODY_FEED: u8 = 0x04;
}

// ─── Authority role kinds (u8) ───────────────────────────────────────────────
pub mod authority_role {
    pub const GOVERNANCE:      u8 = 0x00;
    pub const OPS:             u8 = 0x01;
    pub const EMERGENCY:       u8 = 0x02;
    pub const ENFORCEMENT:     u8 = 0x03;
    pub const INSURANCE_ADMIN: u8 = 0x04;
}

// ─── Window kinds for UsageWindowPda (u8) ────────────────────────────────────
pub mod window_kind {
    pub const HOURLY:  u8 = 0x00;
    pub const DAILY:   u8 = 0x01;
    pub const WEEKLY:  u8 = 0x02;
    pub const MONTHLY: u8 = 0x03;
}

// ─── Fee recipient policies (u8) ─────────────────────────────────────────────
pub mod fee_recipient {
    /// Non-routing. No external recipient: the net fee is not routed. On mint it
    /// is not minted; on redeem it stays in the reserve. Requires
    /// `fee_recipient_key = Pubkey::default()`.
    pub const NONE:                  u8 = 0x00;
    /// Routed to the protocol treasury token account. Requires a non-default
    /// approved `fee_recipient_key`.
    pub const PROTOCOL_TREASURY:     u8 = 0x01;
    /// Routed to an operator-owned wallet token account. Requires a non-default
    /// approved `fee_recipient_key`.
    pub const OPERATOR_OWNED_WALLET: u8 = 0x02;
    /// Routed to a pathway-specific recipient token account. Requires a
    /// non-default approved `fee_recipient_key`.
    pub const PATHWAY_SPECIFIC:      u8 = 0x03;
    /// Non-routing. The net fee is retained in the collateral reserve (redeem)
    /// or left unminted (mint). Requires `fee_recipient_key = Pubkey::default()`.
    pub const RESERVE_RETENTION:     u8 = 0x04;
}

// ─── Rounding modes (u8) ─────────────────────────────────────────────────────
pub mod rounding {
    pub const FLOOR:   u8 = 0x00;
    pub const CEILING: u8 = 0x01;
    pub const NEAREST: u8 = 0x02;
}
