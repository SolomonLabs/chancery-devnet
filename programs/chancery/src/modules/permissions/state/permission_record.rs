use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    constants::{role, seeds},
    error::ChanceryError,
};

// ─── Discriminator ────────────────────────────────────────────────────────────
pub const PERMISSION_RECORD_DISCRIMINATOR: [u8; 8] =
    [0x70, 0x65, 0x72, 0x6d, 0x72, 0x65, 0x63, 0x00];

// ─── Account size ─────────────────────────────────────────────────────────────
// Role bits use [u64; 2] - avoids u128 alignment hazards on BPF.
//
//   [u8;8]   discriminator             =  8 @ 0
//   u16      version                   =  2 @ 8
//   u8       bump                      =  1 @ 10
//   u8       scope_kind                =  1 @ 11
//   [u8;4]   _pad0                     =  4 @ 12  -> u64 at 16
//   [u8;32]  subject                   = 32 @ 16
//   [u8;32]  scope_key                 = 32 @ 48
//   [u64;2]  role_bits                 = 16 @ 80  (80%8=0 Y)
//   u64      permission_flags          =  8 @ 96
//   i64      issued_at_unix_timestamp  =  8 @ 104
//   i64      expiry_unix_timestamp     =  8 @ 112
//   [u8;32]  granted_by                = 32 @ 120
//   u16      role_schema_version       =  2 @ 152
//   [u8;6]   _pad1                     =  6 @ 154 -> u64 at 160
//   u64      permission_generation     =  8 @ 160
//   [u8;24]  _reserved                 = 24 @ 168
//                                       ─────
//                                       192 bytes
pub const PERMISSION_RECORD_SIZE: usize = 192;

// ─── Role bit helpers ─────────────────────────────────────────────────────────
/// Pack two u64 words into a u128 for bitwise role checks.
#[inline]
pub fn role_bits_as_u128(words: [u64; 2]) -> u128 {
    (words[0] as u128) | ((words[1] as u128) << 64)
}

/// Unpack a u128 role mask into the two-word wire representation.
#[inline]
pub fn u128_to_role_bits(mask: u128) -> [u64; 2] {
    [mask as u64, (mask >> 64) as u64]
}

// ─── State ────────────────────────────────────────────────────────────────────

/// Per-subject, per-scope permission record.
/// Seed: [b"permission", subject, scope_kind (1 byte), scope_key]
///
/// One PDA per (subject, scope_kind, scope_key) triple.
/// Adding a new scope type = new scope_kind byte; no structural change.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PermissionRecord {
    pub discriminator:            [u8; 8],
    pub version:                  u16,
    pub bump:                     u8,

    /// One of `scope::*` constants.
    pub scope_kind:               u8,
    pub _pad0:                    [u8; 4],

    /// The wallet or program being granted permissions.
    pub subject:                  Pubkey,

    /// Contextual key for the scope - asset mint, pathway id hash, etc.
    /// For `scope::GLOBAL` this is `Pubkey::default()`.
    pub scope_key:                Pubkey,

    /// Permission role bits - low word then high word.
    /// Use `role_bits_as_u128` / `u128_to_role_bits` for manipulation.
    pub role_bits:                [u64; 2],

    /// Additional per-permission flags (reserved for future use).
    pub permission_flags:         u64,

    /// Unix timestamp when this record was issued.
    pub issued_at_unix_timestamp: i64,

    /// Unix timestamp after which this record is expired.
    /// Zero means no expiry.
    pub expiry_unix_timestamp:    i64,

    /// The authority that signed the grant instruction.
    pub granted_by:               Pubkey,

    /// Exact role vocabulary against which `role_bits` was validated.
    pub role_schema_version:      u16,
    pub _pad1:                    [u8; 6],

    /// Monotonic semantic-state generation. Governance hashes bind both the
    /// current generation and the proposed successor so an accepted proposal
    /// cannot become valid again after an intervening semantic state cycle.
    /// Operational pause changes and semantic no-ops MUST NOT modify it.
    pub permission_generation:    u64,

    pub _reserved:                [u8; 24],
}

// ─── Helpers ──────────────────────────────────────────────────────────────────
impl PermissionRecord {
    /// A revoked record remains allocated as a paused, role-free tombstone.
    #[inline]
    pub fn is_revoked_tombstone(&self) -> bool {
        self.role_bits == [0u64; 2]
            && self.permission_flags == PERMISSION_FLAG_PAUSED
            && self.expiry_unix_timestamp == 0
            && self.role_schema_version == role::PERMISSION_ROLE_SCHEMA_VERSION
    }

    /// Generation committed by permission pending-change hashes.
    ///
    /// Tombstones created before the dedicated generation field existed have a
    /// zeroed reserved tail. Treat them as generation one so they remain
    /// distinct from a never-created record and continue to invalidate stale
    /// grant approvals after an upgrade.
    #[inline]
    pub fn effective_permission_generation(&self) -> u64 {
        if self.permission_generation == 0 && self.is_revoked_tombstone() {
            1
        } else {
            self.permission_generation
        }
    }

    pub fn pda(
        subject   : &Pubkey,
        scope_kind: u8,
        scope_key : &Pubkey,
        program_id: &Pubkey,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                seeds::PERMISSION,
                subject.as_ref(),
                &[scope_kind],
                scope_key.as_ref(),
            ],
            program_id,
        )
    }

    pub fn verify_pda(
        account:    &AccountInfo,
        subject:    &Pubkey,
        scope_kind: u8,
        scope_key : &Pubkey,
        program_id: &Pubkey,
    ) -> Result<u8, ProgramError> {
        let (expected, bump) = Self::pda(subject, scope_kind, scope_key, program_id);

        if account.key != &expected {
            return Err(ChanceryError::InvalidPda.into());
        }

        Ok(bump)
    }

    pub fn load<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = crate::state_loader::load_state::<Self>(
            account,
            PERMISSION_RECORD_SIZE,
            PERMISSION_RECORD_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    /// Load initialized state and prove that its identity fields resolve to
    /// this account's canonical PDA and stored bump.
    pub fn load_verified<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;
        let expected_bump = Self::verify_pda(account, &state.subject, state.scope_kind, &state.scope_key, &crate::id())?;

        if state.bump != expected_bump {
            return Err(ChanceryError::StoredBumpMismatch.into());
        }

        state.assert_role_schema()?;

        Ok(state)
    }

    pub fn load_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = crate::state_loader::load_state_mut::<Self>(
            account,
            PERMISSION_RECORD_SIZE,
            PERMISSION_RECORD_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    pub(crate) fn load_mut_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        subject: &Pubkey,
        scope_kind: u8,
        scope_key: &Pubkey,
        expected_bump: u8,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;

        if state.subject != *subject
            || state.scope_kind != scope_kind
            || state.scope_key != *scope_key
        {
            return Err(ChanceryError::InvalidPda.into());
        }

        crate::state_loader::assert_stored_bump_matches(state.bump, expected_bump)?;
        state.assert_role_schema()?;

        Ok(state)
    }

    /// Mutable counterpart to `load_verified`.
    pub fn load_verified_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;
        let expected_bump = Self::verify_pda(account, &state.subject, state.scope_kind, &state.scope_key, &crate::id())?;

        if state.bump != expected_bump {
            return Err(ChanceryError::StoredBumpMismatch.into());
        }

        state.assert_role_schema()?;

        Ok(state)
    }

    pub fn load_uninitialized_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        crate::state_loader::load_uninitialized_state_mut::<Self>(
            account,
            PERMISSION_RECORD_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    // ── Role checks ───────────────────────────────────────────────────────────

    /// Returns true if all bits in `required_mask` are set in this record's
    /// role_bits field.
    #[inline]
    pub fn has_role(&self, required_mask: u128) -> bool {
        role_bits_as_u128(self.role_bits) & required_mask == required_mask
    }

    /// Assert all bits in `required_mask` are present.
    #[inline]
    pub fn assert_role(&self, required_mask: u128) -> Result<(), ProgramError> {
        self.assert_role_schema()?;

        if !self.has_role(required_mask) {
            return Err(ChanceryError::InsufficientRole.into());
        }

        Ok(())
    }

    /// Fail closed unless the record was written against this binary's exact
    /// role schema and contains only currently active role bits.
    pub fn assert_role_schema(&self) -> Result<(), ProgramError> {
        if self.role_schema_version != role::PERMISSION_ROLE_SCHEMA_VERSION {
            return Err(ChanceryError::PermissionRoleSchemaMismatch.into());
        }

        if role_bits_as_u128(self.role_bits) & !role::ACTIVE_ROLE_MASK != 0 {
            return Err(ChanceryError::PermissionUnknownRoleBits.into());
        }

        Ok(())
    }

    // ── Expiry check ──────────────────────────────────────────────────────────

    /// Assert this record has not expired at the given unix timestamp.
    pub fn assert_not_expired(&self, now_unix_timestamp: i64) -> Result<(), ProgramError> {
        if self.expiry_unix_timestamp != 0 && now_unix_timestamp >= self.expiry_unix_timestamp {
            return Err(ChanceryError::PermissionExpired.into());
        }

        Ok(())
    }

    /// Assert not expired using the current cluster clock.
    pub fn assert_not_expired_now(&self) -> Result<(), ProgramError> {
        let clock = Clock::get()?;

        self.assert_not_expired(clock.unix_timestamp)
    }

    // ── Scope check ───────────────────────────────────────────────────────────

    pub fn assert_scope(
        &self,
        expected_kind: u8,
        expected_key:  &Pubkey,
    ) -> Result<(), ProgramError> {
        if self.scope_kind != expected_kind || &self.scope_key != expected_key {
            return Err(ChanceryError::PermissionScopeMismatch.into());
        }

        Ok(())
    }

    // ── Combined gate ─────────────────────────────────────────────────────────

    /// Single call to gate an instruction: checks role, expiry, and scope.
    pub fn assert_valid(
        &self,
        required_role_mask:  u128,
        expected_scope_kind: u8,
        expected_scope_key:  &Pubkey,
        now_unix_timestamp:  i64,
    ) -> Result<(), ProgramError> {
        self.assert_role(required_role_mask)?;
        self.assert_not_expired(now_unix_timestamp)?;
        self.assert_scope(expected_scope_kind, expected_scope_key)?;

        if self.permission_flags & PERMISSION_FLAG_PAUSED != 0 {
            return Err(ChanceryError::PermissionPaused.into());
        }

        Ok(())
    }
}

// ─── Size assertion ───────────────────────────────────────────────────────────
const _: () = assert!(
    core::mem::size_of::<PermissionRecord>() == PERMISSION_RECORD_SIZE,
    "PermissionRecord size mismatch - update PERMISSION_RECORD_SIZE",
);

// ─── Permission flag bits ──────────────────────────────────────────────────
/// Bit in `permission_flags` indicating this subject (executor or counterparty)
/// is currently paused. Checked by settlement handlers before allowing
/// participation in any flow.
pub const PERMISSION_FLAG_PAUSED: u64 = 1 << 0;

#[cfg(test)]
mod role_schema_tests {
    use super::*;

    fn valid_record(role_bits: u128) -> PermissionRecord {
        let mut record = PermissionRecord::zeroed();
        record.role_schema_version = role::PERMISSION_ROLE_SCHEMA_VERSION;
        record.role_bits = u128_to_role_bits(role_bits);
        record
    }

    #[test]
    fn role_schema_mismatch_fails_closed() {
        let record = valid_record(role::CAN_MINT_DIRECT);
        let mut mismatched = record;
        mismatched.role_schema_version = role::PERMISSION_ROLE_SCHEMA_VERSION + 1;

        assert_eq!(
            mismatched.assert_role(role::CAN_MINT_DIRECT),
            Err(ChanceryError::PermissionRoleSchemaMismatch.into()),
        );
    }

    #[test]
    fn unknown_or_retired_persisted_bits_fail_closed() {
        let cases = [
            role::RETIRED_ROLE_MASK,
            role::CAN_SET_INSURANCE_POLICY,
            1u128 << 90,
        ];
        let mut index = 0usize;
        while index < cases.len() {
            let record = valid_record(cases[index]);
            assert_eq!(
                record.assert_role_schema(),
                Err(ChanceryError::PermissionUnknownRoleBits.into()),
            );
            index += 1;
        }
    }

    #[test]
    fn legacy_revoked_tombstone_has_effective_generation_one() {
        let mut record = valid_record(0);
        record.permission_flags = PERMISSION_FLAG_PAUSED;

        assert_eq!(record.permission_generation, 0);
        assert_eq!(record.effective_permission_generation(), 1);
    }

    #[test]
    fn ordinary_zero_generation_record_remains_generation_zero() {
        let record = valid_record(role::CAN_MINT_DIRECT);

        assert_eq!(record.effective_permission_generation(), 0);
    }

}
