use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::{authority_role, seeds, status_flag},
    error::ChanceryError,
};

// ─── Discriminator ────────────────────────────────────────────────────────────
pub const CHANCERY_CONFIG_DISCRIMINATOR: [u8; 8] =
    [0x76, 0x61, 0x75, 0x6c, 0x74, 0x63, 0x66, 0x67];

// ─── Account size ─────────────────────────────────────────────────────────────
// Field layout for #[repr(C)] - all [u8;N] have alignment 1, u64 alignment 8.
// Explicit _pad fields ensure no implicit compiler padding, satisfying bytemuck Pod.
//   [u8;8]  discriminator                        =   8 @ 0
//   u16     version                              =   2 @ 8
//   u8      bump                                 =   1 @ 10
//   u8      _pad0                                =   1 @ 11
//   [u8;4]  _pad1                                =   4 @ 12  -> u64 at 16 is 8-aligned
//   u64     status_flags                         =   8 @ 16
//   [u8;32] × 11 Pubkeys                         = 352 @ 24
//   u64     event_sequence_nonce                 =   8 @ 376
//   [u8;32] domain_separator                     =  32 @ 384
//   u8      event_authority_bump                 =   1 @ 416
//   u8      mint_authority_bump                  =   1 @ 417
//   u8      reserve_authority_bump               =   1 @ 418
//   u8      authority_bump_cache_version         =   1 @ 419
//   [u8;4]  _pad_authority_bumps                 =   4 @ 420   -> 4-aligned at 424
//   u32     total_remote_domains_registered      = 4 @ 424
//   u32     total_signer_sets_registered         = 4 @ 428
//   u32     allocated_permission_record_slots    = 4 @ 432
//   u32     total_reserve_destinations_registered= 4 @ 436
//   u32     total_limit_policies_registered      = 4 @ 440
//   u32     total_pathway_policies_registered    = 4 @ 444
//   [u8;24] _reserved                            =  24 @ 448
//                                                ─────
//                                                472 bytes
//
// Layout note: three authority bumps, a one-byte cache-format marker, and four
// alignment bytes occupy the pre-existing eight-byte bump region, so the six
// family counters retain their original offsets and the account remains exactly
// 472 bytes. Legacy accounts have a zero cache marker and interpret bytes
// 417..423 as padding; runtime accessors derive mint/reserve bumps for them.
//
// Counter semantics (doc 14 §14.6.4): registration handlers already hold this
// account writable for the sequence nonce, so increments are nearly free.
// Remote domains and signer sets are total-ever (tombstone model - never
// decremented); allocated_permission_record_slots counts canonical PermissionRecord
// accounts that have ever been allocated and does not change on zero-in-place
// revoke/regrant; policy/destination counters are total-ever until their close
// instructions ship. All increments are checked_add fail-closed.
pub const CHANCERY_CONFIG_SIZE: usize = 472;

/// Cache-format marker stored in former padding. The account version remains 1
/// so the repository-wide state loader retains its existing compatibility gate.
pub const AUTHORITY_BUMP_CACHE_VERSION: u8 = 1;

// ─── State ────────────────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ChanceryConfig {
    pub discriminator:                         [u8; 8],
    pub version:                               u16,
    pub bump:                                  u8,
    pub _pad0:                                 u8,
    pub _pad1:                                 [u8; 4],
    pub status_flags:                          u64,
    pub governance_authority:                  Pubkey,
    pub operations_authority:                  Pubkey,
    pub emergency_authority:                   Pubkey,
    pub enforcement_authority:                 Pubkey,
    pub insurance_admin_authority:             Pubkey,
    pub issued_token_mint:                     Pubkey,
    pub issued_token_program:                  Pubkey,
    pub legacy_token_mint:                     Pubkey,
    pub legacy_token_program:                  Pubkey,
    pub mint_authority_pda:                    Pubkey,
    pub freeze_authority_pda:                  Pubkey,
    pub event_sequence_nonce:                  u64,
    pub domain_separator:                      [u8; 32],

    /// Cached bump for the `event-authority` PDA used to sign evidence CPIs.
    /// Populated by `initialize_chancery`; consumed by every `emit_*` helper.
    pub event_authority_bump:                  u8,
    /// Cached bump for the `mint-authority` PDA when the cache marker is set.
    pub mint_authority_bump:                   u8,
    /// Cached bump for the `reserve-authority` PDA when the cache marker is set.
    pub reserve_authority_bump:                u8,
    /// Zero on legacy accounts; `AUTHORITY_BUMP_CACHE_VERSION` on new accounts.
    pub authority_bump_cache_version:          u8,
    pub _pad_authority_bumps:                  [u8; 4],

    /// Total-ever corridor registrations (tombstone model - never decrements).
    pub total_remote_domains_registered:       u32,
    /// Total-ever signer-set registrations (tombstone model).
    pub total_signer_sets_registered:          u32,
    /// Canonical PermissionRecord account slots allocated at least once.
    /// Tombstone revoke/regrant reuses a slot and does not change this count.
    pub allocated_permission_record_slots:     u32,
    /// Total-ever reserve destination registrations.
    pub total_reserve_destinations_registered: u32,
    /// Total-ever limit policy registrations.
    pub total_limit_policies_registered:       u32,
    /// Total-ever pathway policy registrations.
    pub total_pathway_policies_registered:     u32,

    /// Append-only layout headroom. Must remain zero until consumed by a
    /// layout change, and must be replenished in that same change.
    pub _reserved:                             [u8; 24],
}

// ─── Evidence context ─────────────────────────────────────────────────────────

/// Bundle of fields every emit wrapper needs. Built once per
/// state-mutating handler via `ChanceryConfig::next_evidence_context` and
/// passed by reference to each `emit_*` call.
pub struct EvidenceContext {
    pub event_authority_bump: u8,
    pub sequence_nonce:       u64,
    pub slot:                 u64,
    pub unix_timestamp:       i64,
}

// ─── Helpers ──────────────────────────────────────────────────────────────────
impl ChanceryConfig {
    pub fn pda(program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[seeds::CHANCERY_CONFIG], program_id)
    }

    pub fn verify_pda(
        account:    &AccountInfo,
        program_id: &Pubkey,
    ) -> Result<u8, ProgramError> {
        let (expected, bump) = Self::pda(program_id);

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
            CHANCERY_CONFIG_SIZE,
            CHANCERY_CONFIG_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        if state.status_flags & status_flag::INITIALIZED == 0 {
            return Err(ChanceryError::NotInitialized.into());
        }

        state.assert_authority_bump_cache_supported()?;

        Ok(state)
    }

    pub(crate) fn load_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        expected_bump: u8,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;

        crate::state_loader::assert_stored_bump_matches(state.bump, expected_bump)?;

        Ok(state)
    }

    /// Load canonical initialized config and return the verified bump so a
    /// later mutable reborrow can reuse the same PDA derivation.
    pub(crate) fn load_verified_with_bump<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<(core::cell::Ref<'a, Self>, u8), ProgramError> {
        let expected_bump = Self::verify_pda(account, &crate::id())?;
        let state = Self::load_for_verified_pda(account, expected_bump)?;

        Ok((state, expected_bump))
    }

    /// Load initialized state and prove that its identity fields resolve to
    /// this account's canonical PDA and stored bump.
    pub fn load_verified<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;
        let expected_bump = Self::verify_pda(account, &crate::id())?;

        if state.bump != expected_bump {
            return Err(ChanceryError::StoredBumpMismatch.into());
        }

        Ok(state)
    }

    pub fn load_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = crate::state_loader::load_state_mut::<Self>(
            account,
            CHANCERY_CONFIG_SIZE,
            CHANCERY_CONFIG_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        if state.status_flags & status_flag::INITIALIZED == 0 {
            return Err(ChanceryError::NotInitialized.into());
        }

        state.assert_authority_bump_cache_supported()?;

        Ok(state)
    }

    pub(crate) fn load_mut_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        expected_bump: u8,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;

        crate::state_loader::assert_stored_bump_matches(state.bump, expected_bump)?;

        Ok(state)
    }

    /// Mutable counterpart to `load_verified`.
    pub fn load_verified_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;
        let expected_bump = Self::verify_pda(account, &crate::id())?;

        if state.bump != expected_bump {
            return Err(ChanceryError::StoredBumpMismatch.into());
        }

        Ok(state)
    }

    pub fn load_uninitialized_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        crate::state_loader::load_uninitialized_state_mut::<Self>(
            account,
            CHANCERY_CONFIG_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    #[inline]
    pub fn is_emergency_authority(&self, authority: &Pubkey) -> bool {
        authority == &self.emergency_authority
    }

    /// Reject assignment of a role to a key already occupying another role.
    ///
    /// Bootstrap intentionally starts with one governance key in every slot so
    /// the deployment can be initialized atomically. Transfers then converge
    /// toward separation one role at a time: this check ignores the role being
    /// replaced, but never permits a proposal to introduce or preserve a new
    /// cross-role alias. Acceptance re-runs the check against current state so
    /// concurrent rotations cannot create an overlap after proposal.
    pub fn assert_authority_assignment_distinct(
        &self,
        role_kind:          u8,
        proposed_authority: &Pubkey,
    ) -> Result<(), ProgramError> {
        let authorities = [
            (authority_role::GOVERNANCE, self.governance_authority),
            (authority_role::OPS, self.operations_authority),
            (authority_role::EMERGENCY, self.emergency_authority),
            (authority_role::ENFORCEMENT, self.enforcement_authority),
            (authority_role::INSURANCE_ADMIN, self.insurance_admin_authority),
        ];

        let mut index = 0usize;
        while index < authorities.len() {
            let (other_role_kind, other_authority) = authorities[index];

            if other_role_kind != role_kind && &other_authority == proposed_authority {
                return Err(ChanceryError::AuthorityRoleOverlap.into());
            }

            index += 1;
        }

        Ok(())
    }

    fn assert_authority_bump_cache_supported(&self) -> Result<(), ProgramError> {
        if self.authority_bump_cache_version > AUTHORITY_BUMP_CACHE_VERSION {
            return Err(ChanceryError::UnsupportedStateVersion.into());
        }

        Ok(())
    }

    /// Return the cached mint-authority bump when present. Legacy account
    /// layouts stored zero padding here, so derive only for those accounts.
    #[inline]
    pub fn resolved_mint_authority_bump(&self, program_id: &Pubkey) -> u8 {
        if self.authority_bump_cache_version == AUTHORITY_BUMP_CACHE_VERSION {
            self.mint_authority_bump
        } else {
            Pubkey::find_program_address(&[seeds::MINT_AUTHORITY], program_id).1
        }
    }

    /// Return the cached reserve-authority bump when present. Legacy account
    /// layouts stored zero padding here, so derive only for those accounts.
    #[inline]
    pub fn resolved_reserve_authority_bump(&self, program_id: &Pubkey) -> u8 {
        if self.authority_bump_cache_version == AUTHORITY_BUMP_CACHE_VERSION {
            self.reserve_authority_bump
        } else {
            Pubkey::find_program_address(&[seeds::RESERVE_AUTHORITY], program_id).1
        }
    }

    /// Resolve the canonical reserve-authority address and bump. Cached
    /// accounts reconstruct the address with one hash instead of searching
    /// for the bump again; legacy accounts retain the canonical search path.
    #[inline]
    pub(crate) fn resolved_reserve_authority(
        &self,
        program_id: &Pubkey,
    ) -> Result<(Pubkey, u8), ProgramError> {
        if self.authority_bump_cache_version == AUTHORITY_BUMP_CACHE_VERSION {
            let bump      = self.reserve_authority_bump;
            let bump_seed = [bump];
            let address   = Pubkey::create_program_address(
                &[seeds::RESERVE_AUTHORITY, &bump_seed],
                program_id,
            )
            .map_err(|_| ProgramError::from(ChanceryError::InvalidPda))?;

            Ok((address, bump))
        } else {
            Ok(Pubkey::find_program_address(
                &[seeds::RESERVE_AUTHORITY],
                program_id,
            ))
        }
    }

    pub fn next_sequence_nonce(&mut self) -> Result<u64, ProgramError> {
        self.event_sequence_nonce = self
            .event_sequence_nonce
            .checked_add(1)
            .ok_or(ChanceryError::ArithmeticOverflow)?;

        Ok(self.event_sequence_nonce)
    }

    /// Build an `EvidenceContext` for the current handler invocation. Bumps
    /// `event_sequence_nonce` and snapshots the clock. Caller must pass `&Clock::get()?`
    /// so this state file does not depend on sysvar APIs.
    pub fn next_evidence_context(
        &mut self,
        clock: &solana_clock::Clock,
    ) -> Result<EvidenceContext, ProgramError> {
        Ok(EvidenceContext {
            event_authority_bump: self.event_authority_bump,
            sequence_nonce:       self.next_sequence_nonce()?,
            slot:                 clock.slot,
            unix_timestamp:       clock.unix_timestamp,
        })
    }
}

// ─── Size assertion ───────────────────────────────────────────────────────────
const _: () = assert!(
    core::mem::size_of::<ChanceryConfig>() == CHANCERY_CONFIG_SIZE,
    "ChanceryConfig size mismatch - update CHANCERY_CONFIG_SIZE",
);

#[cfg(test)]
mod tests {
    use super::*;
    use bytemuck::Zeroable;

    #[test]
    fn cache_marker_uses_cached_authority_bumps() {
        let mut config = ChanceryConfig::zeroed();
        config.version = 1;
        config.authority_bump_cache_version = AUTHORITY_BUMP_CACHE_VERSION;
        config.mint_authority_bump = 17;
        config.reserve_authority_bump = 23;

        assert_eq!(config.resolved_mint_authority_bump(&crate::id()), 17);
        assert_eq!(config.resolved_reserve_authority_bump(&crate::id()), 23);
    }

    #[test]
    fn legacy_zero_cache_marker_derives_authority_bumps_from_canonical_seeds() {
        let mut config = ChanceryConfig::zeroed();
        config.version = 1;
        config.authority_bump_cache_version = 0;
        config.mint_authority_bump = 0;
        config.reserve_authority_bump = 0;

        let expected_mint = Pubkey::find_program_address(
            &[seeds::MINT_AUTHORITY],
            &crate::id(),
        ).1;
        let expected_reserve = Pubkey::find_program_address(
            &[seeds::RESERVE_AUTHORITY],
            &crate::id(),
        ).1;

        assert_eq!(config.resolved_mint_authority_bump(&crate::id()), expected_mint);
        assert_eq!(config.resolved_reserve_authority_bump(&crate::id()), expected_reserve);
    }

    #[test]
    fn cached_reserve_authority_reconstructs_the_canonical_address() {
        let program_id = crate::id();
        let expected   = Pubkey::find_program_address(
            &[seeds::RESERVE_AUTHORITY],
            &program_id,
        );
        let mut config = ChanceryConfig::zeroed();
        config.version = 1;
        config.authority_bump_cache_version = AUTHORITY_BUMP_CACHE_VERSION;
        config.reserve_authority_bump = expected.1;

        assert_eq!(config.resolved_reserve_authority(&program_id), Ok(expected));
    }

    #[test]
    fn legacy_reserve_authority_resolution_retains_the_canonical_search_path() {
        let program_id = crate::id();
        let expected   = Pubkey::find_program_address(
            &[seeds::RESERVE_AUTHORITY],
            &program_id,
        );
        let mut config = ChanceryConfig::zeroed();
        config.version = 1;
        config.authority_bump_cache_version = 0;

        assert_eq!(config.resolved_reserve_authority(&program_id), Ok(expected));
    }

    #[test]
    fn unknown_cache_marker_is_rejected() {
        let mut config = ChanceryConfig::zeroed();
        config.authority_bump_cache_version = AUTHORITY_BUMP_CACHE_VERSION + 1;

        assert_eq!(
            config.assert_authority_bump_cache_supported(),
            Err(ChanceryError::UnsupportedStateVersion.into()),
        );
    }

    #[test]
    fn authority_transfer_may_separate_one_bootstrap_role_at_a_time() {
        let bootstrap = Pubkey::new_unique();
        let replacement = Pubkey::new_unique();
        let mut config = ChanceryConfig::zeroed();
        config.governance_authority = bootstrap;
        config.operations_authority = bootstrap;
        config.emergency_authority = bootstrap;
        config.enforcement_authority = bootstrap;
        config.insurance_admin_authority = bootstrap;

        assert!(config
            .assert_authority_assignment_distinct(authority_role::OPS, &replacement)
            .is_ok());
    }

    #[test]
    fn authority_transfer_cannot_assign_another_roles_key() {
        let mut config = ChanceryConfig::zeroed();
        config.governance_authority = Pubkey::new_unique();
        config.operations_authority = Pubkey::new_unique();
        config.emergency_authority = Pubkey::new_unique();
        config.enforcement_authority = Pubkey::new_unique();
        config.insurance_admin_authority = Pubkey::new_unique();

        assert_eq!(
            config.assert_authority_assignment_distinct(
                authority_role::EMERGENCY,
                &config.governance_authority,
            ),
            Err(ChanceryError::AuthorityRoleOverlap.into()),
        );
    }
}
