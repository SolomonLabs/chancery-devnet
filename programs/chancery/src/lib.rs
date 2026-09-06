/// chancery - policy-governed mint/redeem settlement rail.
///
/// Crate layout:
///   entrypoint  - solana entrypoint macro (feature-gated)
///   processor   - top-level instruction dispatch
///   error       - ChanceryError
///   constants   - all seeds, IDs, masks, discriminants
///   modules/    - one sub-module per domain (state + instructions together)

use solana_pubkey::declare_id;

// Devnet program address, stamped by `yarn identity` from the program keypair
// at `.devnet/program-keypair.json`. The all-zero sentinel below means the
// identity has not been stamped yet; the guard underneath refuses to compile
// against it, and the deploy path refuses to run against it.
declare_id!("11111111111111111111111111111111");

// ─── unstamped identity guard ───────────────────────────────────────────────
//
// A build carrying the sentinel would produce an ELF whose declared id is the
// System Program, so every PDA derivation in the program would disagree with
// the deployed address. Fail at compile time instead.
const _: () = {
    const SENTINEL_PROGRAM_ID_BYTES: [u8; 32] = [0u8; 32];
    const SELECTED_PROGRAM_ID_BYTES: [u8; 32] = ID.to_bytes();
    let mut identical = true;
    let mut index = 0;
    while index < 32 {
        if SELECTED_PROGRAM_ID_BYTES[index] != SENTINEL_PROGRAM_ID_BYTES[index] {
            identical = false;
        }
        index += 1;
    }
    assert!(
        !identical,
        "program identity is unstamped; run yarn identity before building"
    );
};

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

pub mod account_security;
pub mod constants;
pub mod error;
pub mod modules;
pub mod processor;
pub mod state_loader;

#[cfg(test)]
mod decode_robustness_lint;
#[cfg(test)]
mod layout_register_lint;
#[cfg(test)]
mod state_owner_guard_lint;
