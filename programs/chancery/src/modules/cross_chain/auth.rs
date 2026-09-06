//! Cross-chain admin authority gating.
//!
//! Helpers were relocated to `permissions::auth` because they're shared with
//! the settlement handlers. This shim re-exports them so existing cross-chain
//! import paths keep working.

pub use crate::modules::permissions::auth::{
    assert_signer_holds_role, assert_subject_holds_role,
};
