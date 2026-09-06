use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::seeds,
    error::ChanceryError,
};

// ─── Discriminator ────────────────────────────────────────────────────────────
pub const REMOTE_NONCE_DISCRIMINATOR: [u8; 8] =
    [0x72, 0x6d, 0x74, 0x6e, 0x6f, 0x6e, 0x63, 0x65];  // "rmtnonce"

// ─── Account size ─────────────────────────────────────────────────────────────
//   [u8;8]   discriminator               =  8 @ 0
//   u16      version                     =  2 @ 8
//   u8       bump                        =  1 @ 10
//   [u8;5]   _pad0                       =  5 @ 11  -> u64 at 16
//   u64      remote_domain_id            =  8 @ 16  (16%8=0 Y)
//   u64      next_inbound_nonce          =  8 @ 24
//   u64      next_outbound_nonce         =  8 @ 32
//   [u8;32]  scope_key                   = 32 @ 40
//   [u8;32]  last_consumed_message_hash  = 32 @ 72
//   [u8;32]  last_emitted_message_hash   = 32 @ 104
//   [u8;32]  _reserved                   = 32 @ 136
//                                         ─────
//                                         168 bytes
pub const REMOTE_NONCE_SIZE: usize = 168;

// ─── State ────────────────────────────────────────────────────────────────────

/// Monotonic nonce counter for cross-chain message ordering. Spec §2.9 / §9.6.
///
/// Seed: [b"remote-nonce", remote_chain_kind (1 byte), remote_domain_id_be (8 bytes BE),
///        scope_key (32 bytes)]
///
/// Note: the spec lists the seed as `[b"remote-nonce", remote_domain_id,
/// counterparty_or_contract]` without `chain_kind`. We add `chain_kind` to
/// the seed for collision safety - different chain kinds may legitimately
/// share a `remote_domain_id` value (e.g. EIP-155 chain id 1 vs Cosmos hub-1)
/// and we want their nonce counters held in distinct PDAs. This chain-kind
/// component is part of the canonical live seed and is verified by every
/// registration and hot-path loader.
///
/// `scope_key` is intentionally opaque - it is the 32-byte identifier under
/// which the nonce is partitioned. Callers may pass a counterparty address
/// (left-padded for EVM), a per-pathway hash, or a per-asset hash. The choice
/// of partitioning lives with the operator and is determined by the bridge
/// model on the remote side.
///
/// Lifecycle:
///   - Created or verified by `register_remote_domain_policy` for the corridor
///     before activation. Inbound consumption, authenticated retirement,
///     outbound emission, and reclaim require an initialized counter and never
///     allocate it in the hot path.
///   - `next_inbound_nonce` is enforced strictly monotonic: consumption must
///     present `expected == next_inbound_nonce`, then bumps to `expected + 1`.
///   - `next_outbound_nonce` is bumped on emission and committed into the
///     message preimage.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct RemoteNonce {
    pub discriminator:              [u8; 8],
    pub version:                    u16,
    pub bump:                       u8,
    pub _pad0:                      [u8; 5],

    pub remote_domain_id:           u64,
    
    /// Strictly monotonic; next inbound source_nonce expected from this scope.
    pub next_inbound_nonce:         u64,
    
    /// Monotonic; next outbound nonce to commit into the next emit preimage.
    pub next_outbound_nonce:        u64,

    pub scope_key:                  [u8; 32],
    
    /// Hash of the most recently consumed inbound message under this scope.
    /// Useful for off-chain audit reconstruction; zero before first consume.
    pub last_consumed_message_hash: [u8; 32],

    /// Hash of the most recent outbound message emitted on this corridor.
    /// Operational reconciliation cursor - symmetric with
    /// `last_consumed_message_hash` on the inbound side. Never part of any
    /// replay decision: strict nonce equality is the exactly-once primitive.
    pub last_emitted_message_hash:  [u8; 32],

    /// Append-only layout headroom. Must remain zero until consumed by a
    /// layout change, and must be replenished in that same change.
    pub _reserved:                  [u8; 32],
}

// ─── Helpers ──────────────────────────────────────────────────────────────────
impl RemoteNonce {
    pub fn pda(
        remote_chain_kind: u8,
        remote_domain_id : u64,
        scope_key        : &[u8; 32],
        program_id       : &Pubkey,
    ) -> (Pubkey, u8) {
        let id_be = remote_domain_id.to_be_bytes();
        Pubkey::find_program_address(
            &[
                seeds::REMOTE_NONCE,
                &[remote_chain_kind],
                &id_be,
                scope_key.as_ref(),
            ],
            program_id,
        )
    }

    pub fn verify_pda(
        account          : &AccountInfo,
        remote_chain_kind: u8,
        remote_domain_id : u64,
        scope_key : &[u8; 32],
        program_id: &Pubkey,
    ) -> Result<u8, ProgramError> {
        let (expected, bump) = Self::pda(
            remote_chain_kind,
            remote_domain_id,
            scope_key,
            program_id,
        );

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
            REMOTE_NONCE_SIZE,
            REMOTE_NONCE_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    pub(crate) fn load_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        remote_domain_id: u64,
        scope_key: &[u8; 32],
        expected_bump: u8,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;

        if state.remote_domain_id != remote_domain_id || state.scope_key != *scope_key {
            return Err(ChanceryError::InvalidPda.into());
        }

        crate::state_loader::assert_stored_bump_matches(state.bump, expected_bump)?;

        Ok(state)
    }

    /// Load and bind a remote nonce to its canonical domain/scope PDA.
    pub fn load_verified<'a>(
        account: &'a AccountInfo<'a>,
        remote_chain_kind: u8,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;
        let expected_bump = Self::verify_pda(
            account,
            remote_chain_kind,
            state.remote_domain_id,
            &state.scope_key,
            &crate::id(),
        )?;

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
            REMOTE_NONCE_SIZE,
            REMOTE_NONCE_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    pub(crate) fn load_mut_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        remote_domain_id: u64,
        scope_key: &[u8; 32],
        expected_bump: u8,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;

        if state.remote_domain_id != remote_domain_id || state.scope_key != *scope_key {
            return Err(ChanceryError::InvalidPda.into());
        }

        crate::state_loader::assert_stored_bump_matches(state.bump, expected_bump)?;

        Ok(state)
    }

    /// Mutable counterpart to `load_verified`.
    pub fn load_verified_mut<'a>(
        account: &'a AccountInfo<'a>,
        remote_chain_kind: u8,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;
        let expected_bump = Self::verify_pda(
            account,
            remote_chain_kind,
            state.remote_domain_id,
            &state.scope_key,
            &crate::id(),
        )?;

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
            REMOTE_NONCE_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    // ── Guards ────────────────────────────────────────────────────────────────

    /// Caller passes the source_nonce from the inbound message preimage.
    /// Must equal `self.next_inbound_nonce` exactly - strict ordering, no gaps.
    /// Use `RemoteNonceGap` semantics if the spec ever permits skipping.
    pub fn assert_inbound_nonce(&self, expected: u64) -> Result<(), ProgramError> {
        if expected != self.next_inbound_nonce {
            return Err(ChanceryError::RemoteNonceMismatch.into());
        }

        Ok(())
    }

    /// Increment `next_inbound_nonce` and return the new value. Caller is
    /// responsible for writing `last_consumed_message_hash` separately.
    pub fn bump_inbound(&mut self) -> Result<u64, ProgramError> {
        let next = self
            .next_inbound_nonce
            .checked_add(1)
            .ok_or(ChanceryError::ArithmeticOverflow)?;

        self.next_inbound_nonce = next;

        Ok(next)
    }

    /// Returns the current `next_outbound_nonce` (to commit into the preimage)
    /// and increments the stored counter. Atomic from the caller's perspective.
    pub fn take_outbound(&mut self) -> Result<u64, ProgramError> {
        let current = self.next_outbound_nonce;
        let next    = current
            .checked_add(1)
            .ok_or(ChanceryError::ArithmeticOverflow)?;

        self.next_outbound_nonce = next;

        Ok(current)
    }
}

// ─── Size assertion ───────────────────────────────────────────────────────────
const _: () = assert!(
    core::mem::size_of::<RemoteNonce>() == REMOTE_NONCE_SIZE,
    "RemoteNonce size mismatch - update REMOTE_NONCE_SIZE",
);
