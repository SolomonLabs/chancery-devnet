//! Canonical external program IDs. Hardcoded to avoid pulling in the
//! spl-* dependency tree; these addresses are genesis-stable.

pub mod programs {
    use solana_pubkey::{pubkey, Pubkey};

    /// Associated Token Account program (`ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL`).
    pub const ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey =
        pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
}
