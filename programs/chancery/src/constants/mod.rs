//! constants
//!
//! Single source of truth for every seed prefix, module ID, instruction
//! discriminant, role bit, status flag, mode byte, and mask used across
//! chancery.
//!
//! Rules:
//!   - No inline `b"..."` seeds anywhere else in the codebase.
//!   - No bit-mask literals duplicated outside this module.
//!   - Discriminants are exhaustive per submodule and must not overlap
//!     within a submodule.
//!
//! Files are thematic groupings; their contents are flattened into
//! `crate::constants::*` so call sites continue to use the original paths
//! (e.g. `constants::module::CORE`, `constants::seeds::ASSET_CONFIG`,
//! `constants::ix::settlement::MINT_DIRECT`).

mod cross_chain;
mod extensions;
mod flags;
mod kinds;
mod numeric;
mod status;
mod wire;

// Each of these files wraps its public items in a single `pub mod <same_name>`
// block. Loading the file under an aliased name lets the inner `pub mod` be
// re-exported as `crate::constants::<name>` without colliding with the file
// mod, preserving call sites like `constants::ix::settlement::MINT_DIRECT`.
#[path = "ix.rs"]            mod _ix;            pub use _ix::ix;
#[path = "seeds.rs"]         mod _seeds;         pub use _seeds::seeds;
#[path = "role.rs"]          mod _role;          pub use _role::role;
#[path = "module.rs"]        mod _module;        pub use _module::*;
#[path = "scope.rs"]         mod _scope;         pub use _scope::*;
#[path = "token_program.rs"] mod _token_program; pub use _token_program::token_program;
#[path = "programs.rs"]      mod _programs;      pub use _programs::programs;

pub use cross_chain::*;
pub use extensions::*;
pub use flags::*;
pub use kinds::*;
pub use numeric::*;
pub use status::*;
pub use wire::*;
