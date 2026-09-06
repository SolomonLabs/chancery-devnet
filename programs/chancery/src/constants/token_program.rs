//! Known SPL token program ids. Used to allowlist `issued_token_program`
//! and `legacy_token_program` at chancery initialization so a misconfigured
//! init cannot wedge settlement handlers against an arbitrary program id.

use solana_pubkey::{pubkey, Pubkey};

pub mod token_program {
    use super::*;

    /// Classic SPL Token program.
    pub const SPL_TOKEN: Pubkey =
        pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

    /// Token-2022 program.
    pub const TOKEN_2022: Pubkey =
        pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

    /// True iff `key` is one of the two accepted token program ids.
    #[inline]
    pub fn is_accepted(key: &Pubkey) -> bool {
        key == &SPL_TOKEN || key == &TOKEN_2022
    }
}
