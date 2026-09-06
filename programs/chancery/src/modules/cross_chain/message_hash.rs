//! Canonical message-hash construction for cross-chain mint/redeem.
//!
//! The 32-byte SHA-256 over the canonical preimage binds every economic
//! term of a cross-chain message. It is computed identically by the local
//! chancery program and the remote daughter contract; mismatch between the
//! two is cryptographically rejected on consume.
//!
//! Preimage layout (429 bytes total). Every field is fixed-width and
//! big-endian; there is no length-prefixing and no optional fields.
//!
//!   ```text
//!   offset  size  field
//!   ──────  ────  ─────
//!        0    23  CROSS_CHAIN_PROTOCOL_TAG  ("CHANCERY_CROSS_CHAIN_V1")
//!       23     2  canon_version                     u16 BE  (current = 0x0002)
//!       25     1  message_kind                      u8
//!       26     1  source_chain_kind                 u8
//!       27     1  destination_chain_kind            u8
//!       28     1  _pad0                             u8      (must be 0x00)
//!       29     8  source_domain_id                  u64 BE
//!       37     8  destination_domain_id             u64 BE
//!       45     8  source_nonce                      u64 BE
//!       53    16  amount                            u128 BE
//!       69     8  expires_at_unix_timestamp         i64 BE  (two's-complement)
//!       77    32  source_chancery_contract
//!      109    32  destination_chancery_contract
//!      141    32  source_domain_separator
//!      173    32  destination_domain_separator
//!      205    32  source_asset
//!      237    32  destination_asset
//!      269    32  source_issued_token
//!      301    32  destination_issued_token
//!      333    32  sender
//!      365    32  recipient
//!      397    32  signer_set_id
//!      429        end
//!   ```
//!
//! `signer_set_id` binds the attestation trust root, so signatures are not
//! reusable across signer sets. Both sides use `RemoteDomainPolicy.signer_set_id`.
//!
//! `intent_id` and `pathway_id` are intentionally NOT in the preimage -
//! they are local-only concepts that the remote chain has no ability to
//! compute. Local audit binding happens in the canonical evidence events
//! after the hash is verified.
//!
//! `amount` is u128 to accommodate EVM-side u256 values truncated to fit
//! within Solana's u64 token-amount domain. Local callers pass a u64
//! amount widened to u128 (high 64 bits zero); the daughter contract
//! constructs the same width on its side. Any inbound message with
//! `amount > u64::MAX` is rejected by the consume handler before the
//! hash check.
//!
//! Bumping `MESSAGE_HASH_CANON_VERSION` (or modifying field order, padding,
//! encoding, or `CROSS_CHAIN_PROTOCOL_TAG`) is a wire-format breaking
//! change requiring synchronized re-deployment of every daughter contract.

use solana_program_error::ProgramError;
use solana_sha256_hasher::hashv;

use crate::{
    constants::CROSS_CHAIN_PROTOCOL_TAG,
    error::ChanceryError,
};

/// Canonical hash version. Any change to the preimage layout requires
/// bumping this constant and re-deploying every daughter contract.
pub const MESSAGE_HASH_CANON_VERSION: u16 = 2;

/// Total preimage size in bytes. Used by tests and offline tooling for
/// sanity-checking buffer constructions; the on-chain `compute_message_hash`
/// does NOT allocate this buffer.
pub const MESSAGE_HASH_PREIMAGE_LEN:  usize = 429;

/// Borrowed view of every field that contributes to the cross-chain
/// message hash. All 32-byte fields are passed by reference to avoid
/// copying. EVM-side 20-byte addresses are left-padded with 12 zero bytes
/// on the caller side.
pub struct MessageHashPreimage<'a> {
    pub message_kind:                  u8,
    pub source_chain_kind:             u8,
    pub destination_chain_kind:        u8,

    pub source_domain_id:              u64,
    pub destination_domain_id:         u64,
    pub source_nonce:                  u64,

    pub amount:                        u128,
    pub expires_at_unix_timestamp:     i64,

    pub source_chancery_contract:      &'a [u8; 32],
    pub destination_chancery_contract: &'a [u8; 32],
    pub source_domain_separator:       &'a [u8; 32],
    pub destination_domain_separator:  &'a [u8; 32],
    pub source_asset:                  &'a [u8; 32],
    pub destination_asset:             &'a [u8; 32],
    pub source_issued_token:           &'a [u8; 32],
    pub destination_issued_token:      &'a [u8; 32],
    pub sender:                        &'a [u8; 32],
    pub recipient:                     &'a [u8; 32],

    /// Signer set authorizing the message. Binds the trust root into the hash.
    pub signer_set_id:                 &'a [u8; 32],
}

/// Compute the canonical 32-byte cross-chain message hash via SHA-256.
/// MUST produce identical output for identical inputs across every
/// chancery deployment and every daughter contract.
pub fn compute_message_hash(preimage: &MessageHashPreimage) -> [u8; 32] {
    // Encode scalar fields onto the stack. Held alive for the duration
    // of the hashv call so the slice references remain valid.
    let canon_version_be         = MESSAGE_HASH_CANON_VERSION.to_be_bytes();
    let scalar_block: [u8; 4]    = [
        preimage.message_kind,
        preimage.source_chain_kind,
        preimage.destination_chain_kind,
        0u8,  // _pad0 - must be zero to match canon
    ];
    let source_domain_id_be      = preimage.source_domain_id.to_be_bytes();
    let destination_domain_id_be = preimage.destination_domain_id.to_be_bytes();
    let source_nonce_be          = preimage.source_nonce.to_be_bytes();
    let amount_be                = preimage.amount.to_be_bytes();
    let expires_at_be            = preimage.expires_at_unix_timestamp.to_be_bytes();

    let h = hashv(&[
        CROSS_CHAIN_PROTOCOL_TAG,
        &canon_version_be,
        &scalar_block,
        &source_domain_id_be,
        &destination_domain_id_be,
        &source_nonce_be,
        &amount_be,
        &expires_at_be,
        preimage.source_chancery_contract.as_ref(),
        preimage.destination_chancery_contract.as_ref(),
        preimage.source_domain_separator.as_ref(),
        preimage.destination_domain_separator.as_ref(),
        preimage.source_asset.as_ref(),
        preimage.destination_asset.as_ref(),
        preimage.source_issued_token.as_ref(),
        preimage.destination_issued_token.as_ref(),
        preimage.sender.as_ref(),
        preimage.recipient.as_ref(),
        preimage.signer_set_id.as_ref(),
    ]);

    h.to_bytes()
}

// ─── Epoch-free content hash (spec 15 §15.4) ─────────────────────────────────

/// The fixed `signer_set_id` substituted into the canonical preimage when
/// computing the epoch-free content hash: 32 zero bytes.
pub const EPOCH_FREE_SIGNER_SET_ID: [u8; 32] = [0u8; 32];

/// Compute the epoch-free content hash of a message: the canonical message
/// hash of §13.6 with `signer_set_id` fixed to 32 zero bytes.
///
/// The wire message hash embeds the attesting epoch's `signer_set_id`, so it
/// changes across signer rotations and cannot serve as the stable identity of
/// an emission. This hash is deterministic across rotations, binds every
/// economic field, and is computable by the emitting side at emission time,
/// by the expiring side at expiry, and by any auditor offline. The reclaim
/// verifier (spec 15 §15.5) binds to THIS hash, not the epoch-dependent one.
///
/// The `signer_set_id` on the supplied preimage is ignored.
pub fn compute_epoch_free_content_hash(preimage: &MessageHashPreimage) -> [u8; 32] {
    let epoch_free = MessageHashPreimage {
        message_kind:                  preimage.message_kind,
        source_chain_kind:             preimage.source_chain_kind,
        destination_chain_kind:        preimage.destination_chain_kind,
        source_domain_id:              preimage.source_domain_id,
        destination_domain_id:         preimage.destination_domain_id,
        source_nonce:                  preimage.source_nonce,
        amount:                        preimage.amount,
        expires_at_unix_timestamp:     preimage.expires_at_unix_timestamp,
        source_chancery_contract:      preimage.source_chancery_contract,
        destination_chancery_contract: preimage.destination_chancery_contract,
        source_domain_separator:       preimage.source_domain_separator,
        destination_domain_separator:  preimage.destination_domain_separator,
        source_asset:                  preimage.source_asset,
        destination_asset:             preimage.destination_asset,
        source_issued_token:           preimage.source_issued_token,
        destination_issued_token:      preimage.destination_issued_token,
        sender:                        preimage.sender,
        recipient:                     preimage.recipient,
        signer_set_id:                 &EPOCH_FREE_SIGNER_SET_ID,
    };

    compute_message_hash(&epoch_free)
}

/// Compute the hash and assert it equals a claimed value. Returns
/// `MessageHashMismatch` on disagreement. Used in the inbound consume
/// path: the handler is given a claimed hash plus the preimage fields,
/// and must prove they correspond before retiring the source nonce.
pub fn assert_message_hash_matches(
    preimage    : &MessageHashPreimage,
    claimed_hash: &[u8; 32],
) -> Result<(), ProgramError> {
    let computed = compute_message_hash(preimage);

    if &computed != claimed_hash {
        return Err(ChanceryError::MessageHashMismatch.into());
    }

    Ok(())
}

/// Write the canonical preimage to a contiguous buffer. Returns the number
/// of bytes written (always `MESSAGE_HASH_PREIMAGE_LEN`) or
/// `ProgramError::InvalidArgument` when the output buffer is too short. Used
/// by offline audit tooling, the test-vector verifier, and the TypeScript
/// client to reconstruct the preimage for inspection.
///
/// On-chain handlers should NOT call this function - `compute_message_hash`
/// hashes via `hashv` over slice fragments without allocating the buffer.
pub fn write_preimage(
    preimage: &MessageHashPreimage,
    out:      &mut [u8],
) -> Result<usize, ProgramError> {
    if out.len() < MESSAGE_HASH_PREIMAGE_LEN {
        return Err(ProgramError::InvalidArgument);
    }

    let mut o: usize = 0;

    let tag_len = CROSS_CHAIN_PROTOCOL_TAG.len();

    out[o..o + tag_len].copy_from_slice(CROSS_CHAIN_PROTOCOL_TAG);
    o += tag_len;

    out[o..o + 2].copy_from_slice(&MESSAGE_HASH_CANON_VERSION.to_be_bytes());
    o += 2;

    out[o] = preimage.message_kind;            o += 1;
    out[o] = preimage.source_chain_kind;       o += 1;
    out[o] = preimage.destination_chain_kind;  o += 1;
    out[o] = 0u8;                              o += 1;  // _pad0

    out[o..o + 8].copy_from_slice(&preimage.source_domain_id.to_be_bytes());      o += 8;
    out[o..o + 8].copy_from_slice(&preimage.destination_domain_id.to_be_bytes()); o += 8;
    out[o..o + 8].copy_from_slice(&preimage.source_nonce.to_be_bytes());          o += 8;

    out[o..o + 16].copy_from_slice(&preimage.amount.to_be_bytes()); o += 16;
    out[o..o + 8].copy_from_slice(&preimage.expires_at_unix_timestamp.to_be_bytes()); o += 8;

    out[o..o + 32].copy_from_slice(preimage.source_chancery_contract);      o += 32;
    out[o..o + 32].copy_from_slice(preimage.destination_chancery_contract); o += 32;
    out[o..o + 32].copy_from_slice(preimage.source_domain_separator);       o += 32;
    out[o..o + 32].copy_from_slice(preimage.destination_domain_separator);  o += 32;
    out[o..o + 32].copy_from_slice(preimage.source_asset);                  o += 32;
    out[o..o + 32].copy_from_slice(preimage.destination_asset);             o += 32;
    out[o..o + 32].copy_from_slice(preimage.source_issued_token);           o += 32;
    out[o..o + 32].copy_from_slice(preimage.destination_issued_token);      o += 32;
    out[o..o + 32].copy_from_slice(preimage.sender);                        o += 32;
    out[o..o + 32].copy_from_slice(preimage.recipient);                     o += 32;
    out[o..o + 32].copy_from_slice(preimage.signer_set_id);                 o += 32;

    if o != MESSAGE_HASH_PREIMAGE_LEN {
        return Err(ProgramError::InvalidInstructionData);
    }

    Ok(o)
}

// ─── Compile-time canon length check ─────────────────────────────────────────
//
// The preimage length is asserted via constant arithmetic so that any drift
// in the byte counts above fails the build. Update both the offset table
// and `MESSAGE_HASH_PREIMAGE_LEN` together if the canon is ever bumped.
const _: () = {
    let computed: usize =
          23  // CROSS_CHAIN_PROTOCOL_TAG
        + 2  // canon_version
        + 4  // [message_kind, source_chain_kind, destination_chain_kind, _pad0]
        + 8  // source_domain_id
        + 8  // destination_domain_id
        + 8  // source_nonce
        + 16  // amount
        + 8  // expires_at_unix_timestamp
        + 32 * 11;  // eleven 32-byte fields

    assert!(
        computed == MESSAGE_HASH_PREIMAGE_LEN,
        "MESSAGE_HASH_PREIMAGE_LEN drift - canon byte count mismatch",
    );
    assert!(
        CROSS_CHAIN_PROTOCOL_TAG.len() == 23,
        "CROSS_CHAIN_PROTOCOL_TAG length drift - bump MESSAGE_HASH_CANON_VERSION",
    );
};
