//! compartments - LATER MODULE, EXCLUDED FROM THE MVP BUILD.
//!
//! Not part of the MVP audit surface. This module is NOT declared in
//! `modules/mod.rs`, so nothing in this directory compiles into the program.
//! The dispatcher returns `ChanceryError::UnknownModule` for reserved module
//! ID `module::COMPARTMENTS` (0x0C); its wire IDs stay reserved and
//! append-only. Its event structs in `evidence/emit/` compile but are
//! excluded from the canonical IDL and generated client.
//!
//! Activation requires the full checklist in CLAUDE.md ("Later module
//! activation checklist"): review every retained handler and state
//! transition, re-enable the `pub mod` declaration and registry arm,
//! regenerate the IDL/client, and run the complete release gate.

use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use crate::{error::ChanceryError, modules::ChanceryModule};

pub mod instructions;
pub mod state;

pub struct Module;

impl ChanceryModule for Module {
    const MODULE_ID: u8 = crate::constants::module::COMPARTMENTS;

    fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
        instructions::dispatch(accounts, data)
    }
}
