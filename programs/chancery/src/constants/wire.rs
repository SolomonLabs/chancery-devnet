//! Wire format constants.
//!
//! Instruction data layout: [ MODULE_ID: u8 | IX_ID: u8 | ...borsh payload ]

pub const WIRE_MODULE_OFFSET: usize = 0;
pub const WIRE_IX_OFFSET:     usize = 1;
pub const WIRE_ARGS_OFFSET:   usize = 2;
pub const WIRE_MINIMUM_LEN:   usize = 2;
