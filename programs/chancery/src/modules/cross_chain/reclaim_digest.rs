//! Reclaim attestation digest (R) construction - spec 15 §15.5.
//!
//! Attesters observe a quorum-attested expiry event (E1 on Chancery, E2 on
//! the daughter) and sign, per corridor, the digest below. The executing side
//! verifies the quorum over R against **its own current** committed signer
//! root using its native conventions - on Chancery this is
//! `verify_attestation_quorum` (secp256k1 recoveries + sorted-pair Merkle
//! proofs over Keccak-256), identical to inbound message consumption.
//!
//! Preimage layout (160 bytes total). Every field is fixed-width and
//! big-endian; no length-prefixing, no optional fields.
//!
//!   ```text
//!   offset  size  field
//!   ──────  ────  ─────
//!        0    19  CHANCERY_RECLAIM_TAG        ("CHANCERY_RECLAIM_V1")
//!       19     2  canon_version               u16 BE  (shared with MESSAGE_HASH_CANON_VERSION)
//!       21     1  source_chain_kind           u8
//!       22     1  destination_chain_kind      u8
//!       23     8  source_domain_id            u64 BE
//!       31     8  destination_domain_id       u64 BE
//!       39    32  source_chancery_contract
//!       71    32  destination_chancery_contract
//!      103     8  source_nonce                u64 BE
//!      111    32  epoch_free_content_hash     (§15.4 - binds every economic field)
//!      143     1  retirement_reason           u8      (from the retirement event envelope)
//!      144     8  expired_at_unix_timestamp   i64 BE
//!      152     8  expired_at_slot_or_block    u64 BE
//!      160         end
//!   ```
//!
//! Replay separation (spec 15 §15.6): `CHANCERY_RECLAIM_TAG` differs from
//! `CROSS_CHAIN_PROTOCOL_TAG`, so the message-attestation and
//! reclaim-authorization families are mutually unreplayable; the corridor
//! identity fields prevent cross-corridor replay; `source_nonce` plus the
//! executing side's single-shot state prevent intra-corridor replay.
//!
//! This is the sole preproduction reclaim layout. Any tag, version, field-order,
//! or width change must be reflected by every daughter implementation before
//! corridor activation; there is no legacy decoder or migration branch.

use solana_sha256_hasher::hashv;

use crate::{
    constants::CHANCERY_RECLAIM_TAG,
    modules::cross_chain::message_hash::MESSAGE_HASH_CANON_VERSION,
};

/// Total reclaim-digest preimage size in bytes. Used by tests and offline
/// tooling; `compute_reclaim_digest` hashes via `hashv` without allocating.
pub const RECLAIM_DIGEST_PREIMAGE_LEN: usize = 160;

/// Borrowed view of every field that contributes to the reclaim digest R.
pub struct ReclaimDigestPreimage<'a> {
    pub source_chain_kind:             u8,
    pub destination_chain_kind:        u8,

    pub source_domain_id:              u64,
    pub destination_domain_id:         u64,

    pub source_chancery_contract:      &'a [u8; 32],
    pub destination_chancery_contract: &'a [u8; 32],

    /// The emission being recovered - the source's primary key into its
    /// emission record (daughter) or reclaim-record derivation (Chancery).
    pub source_nonce:                  u64,

    /// Rotation-stable binding of the message content (§15.4).
    pub epoch_free_content_hash:       &'a [u8; 32],

    /// One of `inbound_message_retirement_reason::*`, copied from the
    /// authenticated destination retirement event.
    pub retirement_reason:             u8,

    /// Authoritative retirement timestamp from the event envelope.
    pub expired_at_unix_timestamp:     i64,

    /// Slot (Solana E1) or block number (daughter E2) of the expiry.
    pub expired_at_slot_or_block:      u64,
}

/// Compute the 32-byte reclaim attestation digest R via SHA-256.
/// MUST produce identical output for identical inputs on every chancery
/// deployment; the daughter side computes the same bytes under its own hash
/// conventions per document 11.
pub fn compute_reclaim_digest(preimage: &ReclaimDigestPreimage) -> [u8; 32] {
    let canon_version_be         = MESSAGE_HASH_CANON_VERSION.to_be_bytes();
    let chain_kinds: [u8; 2]     = [preimage.source_chain_kind, preimage.destination_chain_kind];
    let source_domain_id_be      = preimage.source_domain_id.to_be_bytes();
    let destination_domain_id_be = preimage.destination_domain_id.to_be_bytes();
    let source_nonce_be          = preimage.source_nonce.to_be_bytes();
    let expired_at_ts_be         = preimage.expired_at_unix_timestamp.to_be_bytes();
    let expired_at_slot_be       = preimage.expired_at_slot_or_block.to_be_bytes();

    let h = hashv(&[
        CHANCERY_RECLAIM_TAG,
        &canon_version_be,
        &chain_kinds,
        &source_domain_id_be,
        &destination_domain_id_be,
        preimage.source_chancery_contract.as_ref(),
        preimage.destination_chancery_contract.as_ref(),
        &source_nonce_be,
        preimage.epoch_free_content_hash.as_ref(),
        &[preimage.retirement_reason],
        &expired_at_ts_be,
        &expired_at_slot_be,
    ]);

    h.to_bytes()
}

/// Write the canonical reclaim-digest preimage to a contiguous buffer.
/// Returns the number of bytes written (always `RECLAIM_DIGEST_PREIMAGE_LEN`).
/// Used by offline audit tooling, the test-vector verifier, and the
/// TypeScript client; on-chain handlers should call `compute_reclaim_digest`.
pub fn write_reclaim_digest_preimage(preimage: &ReclaimDigestPreimage, out: &mut [u8]) -> usize {
    assert!(
        out.len() >= RECLAIM_DIGEST_PREIMAGE_LEN,
        "write_reclaim_digest_preimage: output buffer must be >= RECLAIM_DIGEST_PREIMAGE_LEN",
    );

    let mut o: usize = 0;

    let tag_len = CHANCERY_RECLAIM_TAG.len();

    out[o..o + tag_len].copy_from_slice(CHANCERY_RECLAIM_TAG);
    o += tag_len;

    out[o..o + 2].copy_from_slice(&MESSAGE_HASH_CANON_VERSION.to_be_bytes());
    o += 2;

    out[o] = preimage.source_chain_kind;       o += 1;
    out[o] = preimage.destination_chain_kind;  o += 1;

    out[o..o + 8].copy_from_slice(&preimage.source_domain_id.to_be_bytes());      o += 8;
    out[o..o + 8].copy_from_slice(&preimage.destination_domain_id.to_be_bytes()); o += 8;

    out[o..o + 32].copy_from_slice(preimage.source_chancery_contract);      o += 32;
    out[o..o + 32].copy_from_slice(preimage.destination_chancery_contract); o += 32;

    out[o..o + 8].copy_from_slice(&preimage.source_nonce.to_be_bytes()); o += 8;

    out[o..o + 32].copy_from_slice(preimage.epoch_free_content_hash); o += 32;

    out[o] = preimage.retirement_reason; o += 1;

    out[o..o + 8].copy_from_slice(&preimage.expired_at_unix_timestamp.to_be_bytes()); o += 8;
    out[o..o + 8].copy_from_slice(&preimage.expired_at_slot_or_block.to_be_bytes()); o += 8;

    debug_assert_eq!(o, RECLAIM_DIGEST_PREIMAGE_LEN);

    o
}

// ─── Compile-time canon length check ─────────────────────────────────────────
const _: () = {
    let computed: usize =
          19  // CHANCERY_RECLAIM_TAG
        + 2  // canon_version
        + 2  // [source_chain_kind, destination_chain_kind]
        + 8  // source_domain_id
        + 8  // destination_domain_id
        + 32  // source_chancery_contract
        + 32  // destination_chancery_contract
        + 8  // source_nonce
        + 32  // epoch_free_content_hash
        + 1  // retirement_reason
        + 8  // expired_at_unix_timestamp
        + 8;  // expired_at_slot_or_block

    assert!(
        computed == RECLAIM_DIGEST_PREIMAGE_LEN,
        "RECLAIM_DIGEST_PREIMAGE_LEN drift - canon byte count mismatch",
    );
    assert!(
        CHANCERY_RECLAIM_TAG.len() == 19,
        "CHANCERY_RECLAIM_TAG length drift - wire-format breaking change",
    );
};
