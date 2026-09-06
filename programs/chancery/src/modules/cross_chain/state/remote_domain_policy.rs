use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::{remote_domain_pause_bit, seeds, status_flag},
    error::ChanceryError,
};

// ─── Discriminator ────────────────────────────────────────────────────────────
pub const REMOTE_DOMAIN_POLICY_DISCRIMINATOR: [u8; 8] =
    [0x72, 0x6d, 0x64, 0x6d, 0x70, 0x6f, 0x6c, 0x79];  // "rmdmpoly"

// ─── Account size ─────────────────────────────────────────────────────────────
//   [u8;8]   discriminator                  =  8 @ 0
//   u16      version                        =  2 @ 8
//   u8       bump                           =  1 @ 10
//   u8       remote_chain_kind              =  1 @ 11
//   u8       minimum_attestation_threshold  =  1 @ 12
//   [u8;3]   _pad0                          =  3 @ 13  -> u64 at 16
//   u64      remote_domain_id               =  8 @ 16  (16%8=0 Y)
//   u64      required_finality_depth        =  8 @ 24
//   u64      message_expiry_seconds         =  8 @ 32
//   u64      per_message_maximum            =  8 @ 40
//   u64      per_day_maximum                =  8 @ 48
//   u64      status_flags                   =  8 @ 56  (init bit + pause bits)
//   u64      updated_at_slot                =  8 @ 64
//   [u8;32]  remote_domain_separator        = 32 @ 72
//   [u8;32]  remote_chancery_contract       = 32 @ 104
//   [u8;32]  remote_issued_token            = 32 @ 136
//   [u8;32]  signer_set_id                  = 32 @ 168
//   [u8;32]  created_by                     = 32 @ 200
//   u8       mode                           =  1 @ 232
//   [u8;7]   _pad_mode                      =  7 @ 233
//   [u8;32]  remote_asset                   = 32 @ 240
//   Pubkey    local_asset_mint               = 32 @ 272
//   [u8;32]  _reserved                      = 32 @ 304
//                                            ─────
//                                            336 bytes
pub const REMOTE_DOMAIN_POLICY_SIZE: usize = 336;

/// Whether the current finite corridor policy rejects a message expiry.
///
/// Use the same predicate in consume and terminal retirement so every
/// policy-rejected strict nonce head has a liveness path. `i128` arithmetic
/// keeps `now + window` defined across the full signed message-time domain.
pub(crate) fn expiry_violates_current_window(
    expires_at_unix_timestamp: i64,
    now:                       i64,
    message_expiry_seconds:    u64,
) -> bool {
    if message_expiry_seconds == 0 {
        return false;
    }

    if expires_at_unix_timestamp == 0 {
        return true;
    }

    let max_expiry = i128::from(now) + i128::from(message_expiry_seconds);
    i128::from(expires_at_unix_timestamp) > max_expiry
}

// ─── State ────────────────────────────────────────────────────────────────────

/// Explicit per-remote-domain policy. Spec §2.9 / §9.4.
/// Seed: [b"remote-domain-policy", remote_chain_kind (1 byte), remote_domain_id_be (8 bytes BE)]
///
/// One PDA per (chain_kind, domain_id) pair. Carries the seven §9.10 pause
/// bits in `status_flags` alongside `status_flag::INITIALIZED` (bit 0). Pause
/// bits live at bit 16+ (see `remote_domain_pause_bit::*`).
///
/// Remote contract addresses (`remote_chancery_contract`, `remote_issued_token`)
/// are 32-byte fields that accommodate any chain kind. EVM 20-byte addresses
/// are stored left-padded with 12 zero bytes. The EVM daughter contract uses
/// the same convention when computing the message hash preimage.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct RemoteDomainPolicy {
    pub discriminator:                 [u8; 8],
    pub version:                       u16,
    pub bump:                          u8,
    
    /// One of `chain_kind::*` constants.
    pub remote_chain_kind:             u8,
    
    /// Minimum number of valid attestations required to consume an inbound
    /// message under this domain's signer set.
    pub minimum_attestation_threshold: u8,
    pub _pad0:                         [u8; 3],

    /// Operator-chosen domain id, unique within `remote_chain_kind`.
    /// For EVM, the natural choice is the EIP-155 chain id.
    pub remote_domain_id:              u64,
    
    /// Attestor-enforced remote-block depth required before signing. Solana
    /// cannot observe remote heights and no consume/retirement handler evaluates
    /// this field; the attestor SOP must retain the corresponding remote-chain
    /// evidence. Lowering it is still a trust-policy widening.
    pub required_finality_depth:       u64,

    /// Max forward window (seconds) for a message's signed expiry. Enforced on
    /// consume and used to classify terminal retirement; zero disables.
    pub message_expiry_seconds:        u64,
    
    /// Per-message hard cap, in issued-token base units.
    pub per_message_maximum:           u64,
    
    /// Fixed UTC-day cap, in issued-token base units. Enforced via a
    /// `UsageWindow` keyed by the remote-domain scope hash.
    pub per_day_maximum:               u64,
    
    /// `status_flag::INITIALIZED` (bit 0) plus `remote_domain_pause_bit::*`
    /// (bits 16+). See module-level docstring for layout.
    pub status_flags:                  u64,
    
    /// Slot of the most recent state mutation.
    pub updated_at_slot:               u64,

    /// Domain-separator hash committed to in every cross-chain message
    /// preimage. Computed off-chain as `sha256(remote_program_id || nonce)`
    /// or equivalent stable construction; opaque on-chain.
    pub remote_domain_separator:       [u8; 32],
    
    /// 32-byte address of the daughter contract on the remote chain.
    pub remote_chancery_contract:      [u8; 32],
    
    /// 32-byte address of the daughter issued-token contract in MINT mode.
    /// Required and nonzero in MINT mode; may be the all-zero sentinel in
    /// RELEASE mode, where `remote_asset` is the remote value identity.
    pub remote_issued_token:           [u8; 32],
    
    /// ID of the active `CrossChainSignerSet` for this domain. Rotation
    /// updates this field and writes evidence.
    pub signer_set_id:                 [u8; 32],

    /// Authority that created the policy.
    pub created_by:                    Pubkey,

    /// One of `remote_domain_mode::*` constants. Determines whether
    /// inbound/outbound flows are MINT-mode (remote chain mints/burns its
    /// own representation; `remote_asset` MUST be zero) or RELEASE-mode
    /// (remote chain locks/releases an existing asset; `remote_asset` MUST
    /// be the configured asset address). the specification
    pub mode:                          u8,
    pub _pad_mode:                     [u8; 7],
    
    /// 32-byte address of the asset locked/released on the remote chain in
    /// RELEASE mode. Distinct from `remote_issued_token`, which is the
    /// daughter token contract used in MINT mode. EVM addresses are
    /// left-padded with 12 zero bytes. All-zero in MINT mode.
    pub remote_asset:                  [u8; 32],

    /// Immutable Solana collateral mint expected by the remote daughter as
    /// `motherAsset`. Every pathway paired with this corridor must bind this
    /// exact mint before the corridor nonce can advance or tokens can burn.
    pub local_asset_mint:              Pubkey,

    pub _reserved:                     [u8; 32],
}

// ─── Helpers ──────────────────────────────────────────────────────────────────
impl RemoteDomainPolicy {
    pub fn pda(
        remote_chain_kind: u8,
        remote_domain_id:  u64,
        program_id:        &Pubkey,
    ) -> (Pubkey, u8) {
        let id_be = remote_domain_id.to_be_bytes();
        Pubkey::find_program_address(
            &[
                seeds::REMOTE_DOMAIN_POLICY,
                &[remote_chain_kind],
                &id_be,
            ],
            program_id,
        )
    }

    pub fn verify_pda(
        account:           &AccountInfo,
        remote_chain_kind: u8,
        remote_domain_id:  u64,
        program_id:        &Pubkey,
    ) -> Result<u8, ProgramError> {
        let (expected, bump) = Self::pda(remote_chain_kind, remote_domain_id, program_id);

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
            REMOTE_DOMAIN_POLICY_SIZE,
            REMOTE_DOMAIN_POLICY_DISCRIMINATOR,
            ChanceryError::RemoteDomainPolicyNotFound,
        )?;

        Ok(state)
    }

    pub(crate) fn load_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        remote_chain_kind: u8,
        remote_domain_id: u64,
        expected_bump: u8,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;

        if state.remote_chain_kind != remote_chain_kind
            || state.remote_domain_id != remote_domain_id
        {
            return Err(ChanceryError::InvalidPda.into());
        }

        crate::state_loader::assert_stored_bump_matches(state.bump, expected_bump)?;

        Ok(state)
    }

    /// Load initialized state and prove that its identity fields resolve to
    /// this account's canonical PDA and stored bump.
    pub fn load_verified<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;
        let expected_bump = Self::verify_pda(account, state.remote_chain_kind, state.remote_domain_id, &crate::id())?;

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
            REMOTE_DOMAIN_POLICY_SIZE,
            REMOTE_DOMAIN_POLICY_DISCRIMINATOR,
            ChanceryError::RemoteDomainPolicyNotFound,
        )?;

        Ok(state)
    }

    pub(crate) fn load_mut_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        remote_chain_kind: u8,
        remote_domain_id: u64,
        expected_bump: u8,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;

        if state.remote_chain_kind != remote_chain_kind
            || state.remote_domain_id != remote_domain_id
        {
            return Err(ChanceryError::InvalidPda.into());
        }

        crate::state_loader::assert_stored_bump_matches(state.bump, expected_bump)?;

        Ok(state)
    }

    /// Mutable counterpart to `load_verified`.
    pub fn load_verified_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;
        let expected_bump = Self::verify_pda(account, state.remote_chain_kind, state.remote_domain_id, &crate::id())?;

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
            REMOTE_DOMAIN_POLICY_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    // ── Status / pause guards ────────────────────────────────────────────────

    #[inline]
    pub fn is_initialized(&self) -> bool {
        self.status_flags & status_flag::INITIALIZED != 0
    }

    #[inline]
    pub fn is_paused_for(&self, pause_bit: u64) -> bool {
        self.status_flags & pause_bit != 0
    }

    /// Assert the policy is initialized AND has no domain-level pause bits set
    /// at all. Use `assert_not_paused_for` for fine-grained per-operation checks.
    pub fn assert_active(&self) -> Result<(), ProgramError> {
        if !self.is_initialized() {
            return Err(ChanceryError::RemoteDomainInactive.into());
        }

        if self.status_flags & remote_domain_pause_bit::ALL != 0 {
            return Err(ChanceryError::RemoteDomainPaused.into());
        }

        Ok(())
    }

    /// Assert no specific pause bit is set on this domain. Initialized check
    /// is included so callers don't need to chain it.
    pub fn assert_not_paused_for(&self, pause_bit: u64) -> Result<(), ProgramError> {
        if !self.is_initialized() {
            return Err(ChanceryError::RemoteDomainInactive.into());
        }

        if self.status_flags & pause_bit != 0 {
            return Err(ChanceryError::RemoteDomainPaused.into());
        }

        Ok(())
    }

    // ── Field-binding guards ──────────────────────────────────────────────────

    pub fn assert_signer_set_id_matches(
        &self,
        expected: &[u8; 32],
    ) -> Result<(), ProgramError> {
        if &self.signer_set_id != expected {
            return Err(ChanceryError::SignerSetIdMismatch.into());
        }

        Ok(())
    }

    pub fn assert_remote_domain_separator_matches(
        &self,
        expected: &[u8; 32],
    ) -> Result<(), ProgramError> {
        if &self.remote_domain_separator != expected {
            return Err(ChanceryError::DomainSeparatorMismatch.into());
        }

        Ok(())
    }

    pub fn assert_remote_chancery_contract_matches(
        &self,
        expected: &[u8; 32],
    ) -> Result<(), ProgramError> {
        if &self.remote_chancery_contract != expected {
            return Err(ChanceryError::DaughterContractMismatch.into());
        }

        Ok(())
    }

    pub fn assert_mode(&self, expected: u8) -> Result<(), ProgramError> {
        if self.mode != expected {
            return Err(ChanceryError::RemoteDomainModeMismatch.into());
        }

        Ok(())
    }

    pub fn assert_local_asset_mint_matches(
        &self,
        expected: &Pubkey,
    ) -> Result<(), ProgramError> {
        if &self.local_asset_mint != expected {
            return Err(ChanceryError::RemoteAssetBindingMismatch.into());
        }

        Ok(())
    }

    pub fn assert_per_message_within_cap(
        &self,
        amount: u64,
    ) -> Result<(), ProgramError> {
        if self.per_message_maximum != 0 && amount > self.per_message_maximum {
            return Err(ChanceryError::PerMessageLimitBreached.into());
        }

        Ok(())
    }

    /// When the cap is set, require a bounded expiry within `now + window`.
    pub fn assert_expiry_within_window(
        &self,
        expires_at_unix_timestamp: i64,
        now:                       i64,
    ) -> Result<(), ProgramError> {
        if expiry_violates_current_window(
            expires_at_unix_timestamp,
            now,
            self.message_expiry_seconds,
        ) {
            return Err(ChanceryError::MessageExpiryWindowExceeded.into());
        }

        Ok(())
    }

    /// `window_gross` is u128: see `LimitPolicy::assert_window_volume` for
    /// the rationale. Truncating the cumulative window total to u64 would
    /// silently let a saturated corridor pass the per-day cap check.
    pub fn assert_per_day_within_cap(
        &self,
        window_gross: u128,
        additional  : u64,
    ) -> Result<(), ProgramError> {
        if self.per_day_maximum == 0 {
            return Ok(());
        }

        let new_total: u128 = window_gross
            .checked_add(additional as u128)
            .ok_or(ChanceryError::ArithmeticOverflow)?;

        if new_total > self.per_day_maximum as u128 {
            return Err(ChanceryError::CrossChainPerDayLimitBreached.into());
        }

        Ok(())
    }

    // ── Mutation helpers ──────────────────────────────────────────────────────

    /// Move toward restriction only (bitwise-OR). Used by emergency authority.
    /// Setting bit 0 (`INITIALIZED`) here is rejected - initialization happens
    /// once via `register_remote_domain_policy`.
    pub fn restrict_pause_bits(&mut self, bits: u64) -> Result<(), ProgramError> {
        if bits & status_flag::INITIALIZED != 0 {
            return Err(ChanceryError::AlreadyInitialized.into());
        }

        self.status_flags |= bits & remote_domain_pause_bit::ALL;

        Ok(())
    }

    /// Move toward relaxation (bitwise-AND-NOT). Used by governance authority
    /// only - emergency authority must not call this.
    pub fn relax_pause_bits(&mut self, bits: u64) {
        let pause_only = bits & remote_domain_pause_bit::ALL;

        self.status_flags &= !pause_only;
    }
}

// ─── Size assertion ───────────────────────────────────────────────────────────
const _: () = assert!(
    core::mem::size_of::<RemoteDomainPolicy>() == REMOTE_DOMAIN_POLICY_SIZE,
    "RemoteDomainPolicy size mismatch - update REMOTE_DOMAIN_POLICY_SIZE",
);

#[cfg(test)]
mod tests {
    use super::*;
    use bytemuck::Zeroable;

    fn policy_with_expiry(message_expiry_seconds: u64) -> RemoteDomainPolicy {
        let mut p = RemoteDomainPolicy::zeroed();
        p.message_expiry_seconds = message_expiry_seconds;
        p
    }

    #[test]
    fn pause_masks_are_operation_specific() {
        let mut p = RemoteDomainPolicy::zeroed();
        p.status_flags = status_flag::INITIALIZED;

        let inbound_mask = remote_domain_pause_bit::INBOUND_MESSAGES
            | remote_domain_pause_bit::REMOTE_MINT
            | remote_domain_pause_bit::ATTESTATION_ACCEPTANCE
            | remote_domain_pause_bit::DAUGHTER_CONTRACT;
        let outbound_mask = remote_domain_pause_bit::OUTBOUND_MESSAGES
            | remote_domain_pause_bit::REMOTE_REDEEM
            | remote_domain_pause_bit::DAUGHTER_CONTRACT;

        for pause_bit in [
            remote_domain_pause_bit::INBOUND_MESSAGES,
            remote_domain_pause_bit::REMOTE_MINT,
            remote_domain_pause_bit::ATTESTATION_ACCEPTANCE,
        ] {
            p.status_flags = status_flag::INITIALIZED | pause_bit;
            assert_eq!(
                p.assert_not_paused_for(inbound_mask),
                Err(ChanceryError::RemoteDomainPaused.into()),
            );
            assert!(p.assert_not_paused_for(outbound_mask).is_ok());
        }

        for pause_bit in [
            remote_domain_pause_bit::OUTBOUND_MESSAGES,
            remote_domain_pause_bit::REMOTE_REDEEM,
        ] {
            p.status_flags = status_flag::INITIALIZED | pause_bit;
            assert!(p.assert_not_paused_for(inbound_mask).is_ok());
            assert_eq!(
                p.assert_not_paused_for(outbound_mask),
                Err(ChanceryError::RemoteDomainPaused.into()),
            );
        }

        p.status_flags = status_flag::INITIALIZED | remote_domain_pause_bit::DAUGHTER_CONTRACT;
        assert_eq!(
            p.assert_not_paused_for(inbound_mask),
            Err(ChanceryError::RemoteDomainPaused.into()),
        );
        assert_eq!(
            p.assert_not_paused_for(outbound_mask),
            Err(ChanceryError::RemoteDomainPaused.into()),
        );
    }

    #[test]
    fn zero_window_disables_the_cap() {
        let p = policy_with_expiry(0);
        assert!(p.assert_expiry_within_window(0, 1_000).is_ok());
        assert!(p.assert_expiry_within_window(i64::MAX, 1_000).is_ok());
    }

    #[test]
    fn maximum_valid_window_does_not_overflow_signed_clock_arithmetic() {
        let p = policy_with_expiry(i64::MAX as u64);
        assert!(p.assert_expiry_within_window(i64::MAX, 1_000).is_ok());
    }

    #[test]
    fn unbounded_expiry_rejected_when_cap_set() {
        let p = policy_with_expiry(3_600);
        assert_eq!(
            p.assert_expiry_within_window(0, 1_000),
            Err(ChanceryError::MessageExpiryWindowExceeded.into()),
        );
    }

    #[test]
    fn expiry_at_window_edge_accepted_beyond_rejected() {
        let p   = policy_with_expiry(3_600);
        let now = 1_000i64;

        assert!(p.assert_expiry_within_window(now + 3_600, now).is_ok());
        assert!(p.assert_expiry_within_window(now + 1, now).is_ok());
        assert_eq!(
            p.assert_expiry_within_window(now + 3_601, now),
            Err(ChanceryError::MessageExpiryWindowExceeded.into()),
        );
    }

    #[test]
    fn local_asset_binding_rejects_an_unrelated_pathway_asset() {
        let mut policy = RemoteDomainPolicy::zeroed();
        policy.local_asset_mint = Pubkey::new_unique();

        assert!(policy
            .assert_local_asset_mint_matches(&policy.local_asset_mint)
            .is_ok());
        assert_eq!(
            policy.assert_local_asset_mint_matches(&Pubkey::new_unique()),
            Err(ChanceryError::RemoteAssetBindingMismatch.into()),
        );
    }
}
