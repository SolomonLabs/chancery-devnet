use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::seeds,
    error::ChanceryError,
};

// ─── Discriminator ────────────────────────────────────────────────────────────
pub const OUTBOUND_RECLAIM_RECORD_DISCRIMINATOR: [u8; 8] =
    [0x6f, 0x75, 0x74, 0x72, 0x65, 0x63, 0x6c, 0x6d];  // "outreclm"

// ─── Account size ─────────────────────────────────────────────────────────────
//   [u8;8]   discriminator                 =  8 @ 0
//   u16      version                       =  2 @ 8
//   u8       bump                          =  1 @ 10
//   u8       message_kind                  =  1 @ 11
//   u8       remote_chain_kind             =  1 @ 12
//   u8       retirement_reason             =  1 @ 13
//   [u8;2]   _pad0                         =  2 @ 14  -> u64 at 16
//   u64      remote_domain_id              =  8 @ 16  (16%8=0 Y)
//   u64      source_nonce                  =  8 @ 24
//   [u64;2]  amount_words                  = 16 @ 32  (u128 as words, invariant #2)
//   u64      reclaimed_at_slot             =  8 @ 48
//   i64      reclaimed_at_unix_timestamp   =  8 @ 56
//   [u8;32]  epoch_free_content_hash       = 32 @ 64
//   [u8;32]  reclaim_digest                = 32 @ 96
//   [u8;32]  sender                        = 32 @ 128
//   [u8;32]  attesting_signer_set_id       = 32 @ 160
//   [u8;32]  _reserved                     = 32 @ 192
//                                           ─────
//                                           224 bytes
pub const OUTBOUND_RECLAIM_RECORD_SIZE: usize = 224;

// ─── State ────────────────────────────────────────────────────────────────────

/// Permanent single-shot double-reclaim guard - spec 15 §15.5, instruction B
/// (H-01 remediated derivation).
///
/// Seed: [b"outbound-reclaim", remote_chain_kind (1), remote_domain_id BE (8),
///        source_nonce BE (8)]
///
/// The guard is keyed by the EMISSION IDENTITY - corridor plus outbound
/// nonce - and enforces the on-chain invariant
/// `reclaim_count(corridor, source_nonce) <= 1`.
///
/// It is deliberately NOT keyed by the epoch-free content hash: Chancery
/// keeps no per-emission records (spec 14), so it cannot verify on-chain
/// which content was emitted at a nonce, only that a quorum attests it. A
/// content-keyed guard would therefore let a malicious or inconsistently
/// validating quorum sign two DIFFERENT content objects for the same
/// historical nonce and mint twice - distinct hashes, distinct PDAs, no
/// collision (H-01). Keying by (corridor, nonce) collapses every content
/// claim for one emission onto one record: at most one reclaim per emitted
/// nonce, total reclaims bounded by `next_outbound_nonce`, mirroring the
/// daughter's per-nonce `EMITTED -> RECLAIMED` flip (spec 15 instruction D).
/// The attested content is retained in the record (`epoch_free_content_hash`,
/// `reclaim_digest`) for offline audit against the emission event.
///
/// Class in the §13.1 taxonomy: identifier-keyed (arg-derived), but
/// creation-gated by the full quorum-attested reclaim proof, and lamport
/// squatting cannot block creation (`create_pda_account` tops up and
/// allocates over pre-funded empty accounts).
///
/// Lifecycle (spec 14, permanent, justified):
///   - Created at `reclaim_expired_outbound`; existence == this emission was
///     already reclaimed, under ANY content claim.
///   - **NEVER closeable.** Closure would re-enable the reclaim and permit a
///     double re-mint of the burned emission. No close instruction may ever
///     be added for this family.
///   - Growth is bounded by the anomaly count of terminally retired outbound emissions
///     (steady-state growth per successful operation is zero: successful
///     corridors consume, they do not expire). Rent is paid by the reclaimer.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct OutboundReclaimRecord {
    pub discriminator:               [u8; 8],
    pub version:                     u16,
    pub bump:                        u8,

    /// The reclaimed emission's message kind (an `OUTBOUND_*` burn kind).
    pub message_kind:                u8,

    /// Destination corridor chain kind of the reclaimed emission.
    pub remote_chain_kind:           u8,

    /// One of `inbound_message_retirement_reason::*` from the authenticated
    /// destination retirement evidence.
    pub retirement_reason:           u8,
    pub _pad0:                       [u8; 2],

    /// Destination corridor domain id of the reclaimed emission.
    pub remote_domain_id:            u64,

    /// The emission's outbound nonce (primary key of the recovered emission).
    pub source_nonce:                u64,

    /// Recovered canonical net principal as u128 words (see `usage_window::words_to_u128`).
    pub amount_words:                [u64; 2],

    /// Slot at which the reclaim executed.
    pub reclaimed_at_slot:           u64,

    /// Unix timestamp at which the reclaim executed.
    pub reclaimed_at_unix_timestamp: i64,

    /// §15.4 rotation-stable content binding retained for offline audit.
    pub epoch_free_content_hash:     [u8; 32],

    /// The reclaim digest R actually proven (audit linkage to the quorum).
    pub reclaim_digest:              [u8; 32],

    /// Net-principal recovery recipient: the original emitter from the
    /// attested preimage, never a caller-chosen account.
    pub sender:                      [u8; 32],

    /// Which epoch's quorum proved the reclaim.
    pub attesting_signer_set_id:     [u8; 32],

    /// Append-only layout headroom. Must remain zero until consumed by a
    /// layout change, and must be replenished in that same change.
    pub _reserved:                   [u8; 32],
}

// ─── Helpers ──────────────────────────────────────────────────────────────────
impl OutboundReclaimRecord {
    pub fn pda(
        remote_chain_kind: u8,
        remote_domain_id:  u64,
        source_nonce:      u64,
        program_id:        &Pubkey,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                seeds::OUTBOUND_RECLAIM_RECORD,
                &[remote_chain_kind],
                &remote_domain_id.to_be_bytes(),
                &source_nonce.to_be_bytes(),
            ],
            program_id,
        )
    }

    pub fn verify_pda(
        account:           &AccountInfo,
        remote_chain_kind: u8,
        remote_domain_id:  u64,
        source_nonce:      u64,
        program_id:        &Pubkey,
    ) -> Result<u8, ProgramError> {
        let (expected, bump) = Self::pda(remote_chain_kind, remote_domain_id, source_nonce, program_id);

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
            OUTBOUND_RECLAIM_RECORD_SIZE,
            OUTBOUND_RECLAIM_RECORD_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    /// Load and bind a reclaim record to its canonical emission-identity PDA.
    pub fn load_verified<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;
        let expected_bump = Self::verify_pda(
            account,
            state.remote_chain_kind,
            state.remote_domain_id,
            state.source_nonce,
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
            OUTBOUND_RECLAIM_RECORD_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }
}

// ─── Size assertion ───────────────────────────────────────────────────────────
const _: () = assert!(
    core::mem::size_of::<OutboundReclaimRecord>() == OUTBOUND_RECLAIM_RECORD_SIZE,
    "OutboundReclaimRecord size mismatch - update OUTBOUND_RECLAIM_RECORD_SIZE",
);
