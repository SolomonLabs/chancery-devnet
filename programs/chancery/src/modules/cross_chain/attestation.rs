//! Cross-chain attestation verification.
//!
//! Quorum check for inbound messages (spec 10 §10.7). The signer set commits to a
//! Merkle root over the SORTED 20-byte Ethereum-style addresses of attestor
//! keys. Each attestation is a `(signer_address, signature, recovery_id,
//! merkle_proof)` tuple proving membership in the set and a valid
//! secp256k1 signature over the canonical message hash.
//!
//! Cryptographic primitives:
//!   - `secp256k1_recover` syscall - recovers the 64-byte uncompressed
//!     public key from `(message_hash, recovery_id, signature)`.
//!     Signatures must be low-s canonical before recovery; the syscall itself
//!     accepts high-s malleated forms.
//!   - `keccak256` syscall - used twice per attestation: once to derive
//!     the Ethereum address from the recovered pubkey (`keccak256(pubkey)[12..32]`),
//!     and repeatedly during Merkle proof verification.
//!
//! Merkle convention: sorted-pair hashing (OpenZeppelin standard). At each
//! level the smaller of `(current, sibling)` is concatenated first before
//! `keccak256`. This eliminates the need for explicit left/right index
//! tracking and is what `merkletreejs` produces by default. Off-chain trees
//! MUST be built with the same convention.
//!
//! Compute budget guidance:
//!   secp256k1_recover and Keccak dominate cost. This binary caps the inline
//!   envelope at 4 signatures with depth-2 proofs so the worst supported quorum
//!   remains inside the measured transaction and compute envelope. Callers may
//!   raise the compute budget via `ComputeBudgetProgram::SetComputeUnitLimit`.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_keccak_hasher::hashv as keccak_hashv;
use solana_program_error::ProgramError;
use solana_secp256k1_recover::secp256k1_recover;

use crate::error::ChanceryError;

/// Executability envelope for inline cross-chain attestations. Four signers
/// require at most two sibling hashes per proof and permit a 4-of-4 quorum
/// while keeping the largest reclaim instruction below the pinned 900-byte
/// instruction-data ceiling.
pub const MAX_CROSS_CHAIN_SIGNER_COUNT: u8 = 4;
pub const MAX_ATTESTATION_THRESHOLD:    u8 = 4;
pub const MAX_ATTESTATION_SIGNATURES: usize = 4;
pub const MAX_MERKLE_PROOF_DEPTH:     usize = 2;

pub const ATTESTATION_SIGNATURE_FIXED_SERIALIZED_LEN: usize = 20 + 64 + 1 + 4;
pub const MAX_ATTESTATION_SIGNATURE_SERIALIZED_LEN: usize =
    ATTESTATION_SIGNATURE_FIXED_SERIALIZED_LEN + 32 * MAX_MERKLE_PROOF_DEPTH;
pub const MAX_ATTESTATION_BUNDLE_SERIALIZED_LEN: usize =
    4 + MAX_ATTESTATION_SIGNATURES * MAX_ATTESTATION_SIGNATURE_SERIALIZED_LEN;

/// Applies before Borsh deserialization in every attested handler so a caller
/// cannot allocate or hash an inline payload outside the reviewed transaction
/// and compute envelope.
pub const MAX_ATTESTED_INSTRUCTION_DATA_LEN: usize = 900;

/// secp256k1 half-order (n/2). Signatures whose `s` exceeds this are high-s
/// (malleated) and rejected, matching OpenZeppelin's `ECDSA` low-s rule.
const SECP256K1_HALF_ORDER: [u8; 32] = [
    0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0x5D, 0x57, 0x6E, 0x73, 0x57, 0xA4, 0x50, 0x1D,
    0xDF, 0xE9, 0x2F, 0x46, 0x68, 0x1B, 0x20, 0xA0,
];

/// One attestor's signature plus their inclusion proof in the signer set
/// committed by `CrossChainSignerSet.signer_root`.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct AttestationSignature {
    /// 20-byte Ethereum-style address (last 20 of `keccak256(uncompressed_pubkey)`).
    /// Asserted equal to the address derived from the recovered pubkey on-chain;
    /// callers must supply the address for cheap pre-recovery dedup, but the
    /// authoritative source is the recovery itself.
    pub signer_address: [u8; 20],
    
    /// 64-byte secp256k1 signature: `r || s` (compact form, no `v`).
    pub signature:      [u8; 64],
    
    /// Recovery id, 0 or 1 (post-EIP-155 conventions stripped off-chain).
    pub recovery_id:    u8,
    
    /// Sibling hashes from leaf to root; sorted-pair hashing convention.
    pub merkle_proof:   Vec<[u8; 32]>,
}

/// Verify that `signatures` constitutes a valid attestation quorum over
/// `message_hash` from a signer set committed by `signer_root`.
///
/// Order of checks (cheap → expensive):
///   1. Configuration sanity: `threshold >= minimum_attestation_threshold`.
///   2. Bounds: `threshold <= signatures.len() <= signer_count`.
///   3. Dedup: fixed-stack sort over the bounded claimed signer addresses.
///   4. For each signature: recover pubkey → derive address → assert claim
///      matches recovery → verify Merkle proof against `signer_root`.
pub fn verify_attestation_quorum(
    message_hash:                  &[u8; 32],
    signer_root:                   &[u8; 32],
    threshold:                     u8,
    signer_count:                  u8,
    minimum_attestation_threshold: u8,
    signatures:                    &[AttestationSignature],
) -> Result<(), ProgramError> {
    // ── 1. Threshold floor ────────────────────────────────────────────────────
    if signer_count == 0
        || signer_count > MAX_CROSS_CHAIN_SIGNER_COUNT
        || threshold == 0
        || threshold > MAX_ATTESTATION_THRESHOLD
        || threshold > signer_count
        || signatures.len() > MAX_ATTESTATION_SIGNATURES
    {
        return Err(ChanceryError::AttestationResourceLimitExceeded.into());
    }

    if threshold < minimum_attestation_threshold {
        return Err(ChanceryError::AttestationThresholdNotMet.into());
    }

    // ── 2. Bounds ─────────────────────────────────────────────────────────────
    let signature_count = signatures.len();

    if signature_count < threshold as usize {
        return Err(ChanceryError::AttestationThresholdNotMet.into());
    }

    if signature_count > signer_count as usize {
        return Err(ChanceryError::InvalidSignerSetParameters.into());
    }

    // ── 3. Dedup on claimed addresses (cheap before any crypto) ──────────────
    // Copy into a fixed stack array and sort; no heap allocation and no
    // quadratic comparison path as signer limits evolve.
    let mut sorted_addresses = [[0u8; 20]; MAX_ATTESTATION_SIGNATURES];
    let mut signature_index = 0usize;
    while signature_index < signature_count {
        sorted_addresses[signature_index] = signatures[signature_index].signer_address;
        signature_index += 1;
    }
    sorted_addresses[..signature_count].sort_unstable();
    let mut address_index = 1usize;
    while address_index < signature_count {
        if sorted_addresses[address_index - 1] == sorted_addresses[address_index] {
            return Err(ChanceryError::AttestationDuplicateSigner.into());
        }
        address_index += 1;
    }

    // ── 4. Per-signature crypto ──────────────────────────────────────────────
    for attestation_signature in signatures.iter() {
        if attestation_signature.merkle_proof.len() > MAX_MERKLE_PROOF_DEPTH {
            return Err(ChanceryError::AttestationResourceLimitExceeded.into());
        }


        if attestation_signature.recovery_id > 1 {
            return Err(ChanceryError::AttestationSignatureInvalid.into());
        }

        // 4a. Enforce Ethereum/OpenZeppelin-compatible low-s canonical form.
        //     secp256k1_recover accepts high-s malleated forms; reject s > n/2.
        if attestation_signature.signature[32..64] > SECP256K1_HALF_ORDER[..] {
            return Err(ChanceryError::AttestationSignatureNonCanonical.into());
        }

        // 4b. Recover the secp256k1 pubkey from the signature.
        let recovered_public_key = secp256k1_recover(
            message_hash,
            attestation_signature.recovery_id,
            &attestation_signature.signature,
        )
        .map_err(|_| ChanceryError::AttestationSignatureInvalid)?;
        let public_key_bytes = recovered_public_key.to_bytes();

        // 4c. Derive the 20-byte Ethereum address: keccak256(pubkey)[12..32].
        let public_key_hash                      = keccak_hashv(&[&public_key_bytes]);
        let mut derived_signer_address: [u8; 20] = [0u8; 20];

        derived_signer_address.copy_from_slice(&public_key_hash.to_bytes()[12..32]);

        // 4d. Assert the recovered address matches the caller's claim.
        if derived_signer_address != attestation_signature.signer_address {
            return Err(ChanceryError::AttestationSignatureInvalid.into());
        }

        // 4e. Verify Merkle inclusion in the signer set (sorted-pair convention).
        //     Leaf = keccak256(20-byte address).
        let leaf_hash                   = keccak_hashv(&[&attestation_signature.signer_address]);
        let mut current_hash: [u8; 32]  = leaf_hash.to_bytes();

        for sibling_hash in attestation_signature.merkle_proof.iter() {
            // Lexicographic ordering on the 32-byte hashes - concat smaller-first.
            current_hash = if current_hash.as_slice() <= sibling_hash.as_slice() {
                keccak_hashv(&[&current_hash, sibling_hash]).to_bytes()
            } else {
                keccak_hashv(&[sibling_hash, &current_hash]).to_bytes()
            };
        }

        if &current_hash != signer_root {
            return Err(ChanceryError::AttestationSignerNotInSet.into());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn high_s_quorum(s: [u8; 32]) -> Result<(), ProgramError> {
        let mut signature = [0u8; 64];
        signature[31] = 1; // r = 1; the s-check fires before recovery is reached
        signature[32..].copy_from_slice(&s);

        let sigs = [AttestationSignature {
            signer_address: [0u8; 20],
            signature,
            recovery_id:    0,
            merkle_proof:   Vec::new(),
        }];

        verify_attestation_quorum(&[0u8; 32], &[0u8; 32], 1, 1, 1, &sigs)
    }

    #[test]
    fn high_s_signature_rejected() {
        // s = n - 1, the largest high-s value.
        let s = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE,
            0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B,
            0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36, 0x41, 0x40,
        ];
        assert_eq!(high_s_quorum(s), Err(ChanceryError::AttestationSignatureNonCanonical.into()));
    }

    #[test]
    fn s_just_above_half_order_rejected() {
        // n/2 + 1, the smallest high-s value - proves the boundary is exclusive.
        let mut s = SECP256K1_HALF_ORDER;
        s[31] += 1;
        assert_eq!(high_s_quorum(s), Err(ChanceryError::AttestationSignatureNonCanonical.into()));
    }

    #[test]
    fn half_order_is_floor_of_secp256k1_curve_order() {
        // secp256k1 group order `n` (SEC 2; identical in Bitcoin/Ethereum).
        let n: [u8; 32] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE,
            0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B,
            0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36, 0x41, 0x41,
        ];

        // floor(n / 2) via a big-endian right-shift by one bit.
        let mut half  = [0u8; 32];
        let mut carry = 0u8;
        for i in 0..32 {
            half[i] = (carry << 7) | (n[i] >> 1);
            carry   = n[i] & 1;
        }

        assert_eq!(half, SECP256K1_HALF_ORDER);
    }

    #[test]
    fn maximum_bundle_and_instruction_data_are_pinned() {
        assert_eq!(MAX_ATTESTATION_SIGNATURE_SERIALIZED_LEN, 153);
        assert_eq!(MAX_ATTESTATION_BUNDLE_SERIALIZED_LEN, 616);
        assert!(MAX_ATTESTATION_BUNDLE_SERIALIZED_LEN < MAX_ATTESTED_INSTRUCTION_DATA_LEN);
    }

    #[test]
    fn signer_threshold_and_proof_bounds_fail_before_crypto() {
        let signatures: Vec<AttestationSignature> = (0..MAX_ATTESTATION_SIGNATURES)
            .map(|index| AttestationSignature {
                signer_address: [index as u8; 20],
                signature: [0u8; 64],
                recovery_id: 0,
                merkle_proof: vec![[0u8; 32]; MAX_MERKLE_PROOF_DEPTH + 1],
            })
            .collect();

        assert_eq!(
            verify_attestation_quorum(
                &[0u8; 32],
                &[0u8; 32],
                MAX_ATTESTATION_THRESHOLD,
                MAX_CROSS_CHAIN_SIGNER_COUNT,
                1,
                &signatures,
            ),
            Err(ChanceryError::AttestationResourceLimitExceeded.into()),
        );

        assert_eq!(
            verify_attestation_quorum(&[0u8; 32], &[0u8; 32], 5, 5, 1, &[]),
            Err(ChanceryError::AttestationResourceLimitExceeded.into()),
        );
    }
}
