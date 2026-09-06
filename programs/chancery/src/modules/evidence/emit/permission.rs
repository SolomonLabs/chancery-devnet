//! Permission grant/revoke events.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::modules::evidence::event::{emit_event, ChanceryEvent};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PermissionUpserted {
    pub sequence_nonce:        u64,
    pub chancery:              Pubkey,
    pub slot:                  u64,
    pub unix_timestamp:        i64,

    pub risk_class:            u8,
    pub permission_record:     Pubkey,
    pub subject:               Pubkey,
    pub scope_kind:            u8,
    pub scope_key:             Pubkey,

    /// u128 split as two little-endian u64 limbs over the wire.
    pub role_bits:             [u64; 2],
    pub permission_flags:      u64,
    pub expiry_unix_timestamp: i64,

    /// Authority whose grant constraints were applied and recorded on state.
    pub granted_by:            Pubkey,

    /// Signer that applied this mutation. On a pure restrictive edit this can
    /// differ from `granted_by`, which remains the original grant provenance.
    pub modified_by:           Pubkey,

    /// Zero for direct RestrictiveImmediate/RoutineOps mutations.
    pub change_id:             [u8; 32],

    /// Domain-separated hashes of the semantic old/new permission values.
    pub old_value_hash:        [u8; 32],
    pub new_value_hash:        [u8; 32],

    /// Stable bitmask describing role, pause, expiry, and creation deltas.
    pub change_mask:           u64,

    /// Governance proposer for a pending mutation; zero for direct mutation.
    pub approved_by:           Pubkey,
}
impl ChanceryEvent for PermissionUpserted {
    const NAME: &'static str = "PermissionUpserted";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xB0, 0xD4, 0x6E, 0x4B, 0xBA, 0xFF, 0x79, 0x52]
    }
}

pub fn emit_permission_upserted(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            PermissionUpserted,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PermissionRevoked {
    pub sequence_nonce:    u64,
    pub chancery:          Pubkey,
    pub slot:              u64,
    pub unix_timestamp:    i64,

    /// Always RestrictiveImmediate.
    pub risk_class:        u8,
    pub permission_record: Pubkey,
    pub subject:           Pubkey,
    pub scope_kind:        u8,
    pub scope_key:         Pubkey,
    pub revoked_by:        Pubkey,
}
impl ChanceryEvent for PermissionRevoked {
    const NAME: &'static str = "PermissionRevoked";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xBE, 0xDE, 0x5E, 0x8C, 0xFD, 0x8A, 0x1F, 0xAC]
    }
}

pub fn emit_permission_revoked(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            PermissionRevoked,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}
