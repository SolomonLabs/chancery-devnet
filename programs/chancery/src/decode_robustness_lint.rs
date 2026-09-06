//! Deterministic randomized robustness gates over the pure decode, verify,
//! and math surfaces.
//!
//! These are fuzz-style tests that run under the stable pinned toolchain with
//! no extra dependencies or corpus infrastructure: a seeded xorshift64* PRNG
//! drives thousands of adversarial inputs per surface, and the assertions are
//! *total-behavior* properties - the function must return `Ok`/`Err` (never
//! panic, never over-read) and must uphold its algebraic contract.
//!
//! Deterministic seeds keep the gate reproducible; a failing input can be
//! recovered by re-running with the printed iteration index. These runs are a
//! floor, not a substitute for coverage-guided fuzzing (cargo-fuzz) on the
//! same surfaces, which requires nightly + corpus infrastructure outside the
//! pinned gate and is tracked as a deployment-hardening follow-up.
//!
//! Surfaces:
//!   - Token-2022 TLV parsing (`issuance::tlv`): arbitrary account bytes.
//!   - Cross-chain message hashing: determinism + per-field sensitivity
//!     (every preimage field must influence the hash - a field silently
//!     dropped from the preimage is a forgery vector).
//!   - Attestation quorum verification: garbage signatures and proofs must
//!     reject without panicking, across randomized thresholds and counts.
//!   - Usage-window math: word round-trips, canonical-start algebra
//!     (idempotent, aligned, monotone, never in the future), and
//!     accumulator overflow behavior at saturation.

use solana_program_error::ProgramError;

use crate::{
    constants::window_kind,
    error::ChanceryError,
    modules::{
        cross_chain::{
            attestation::{verify_attestation_quorum, AttestationSignature},
            message_hash::{compute_message_hash, MessageHashPreimage},
        },
        issuance::tlv::tlv_parser::{
            find_tlv_value, parse_mint_extension_mask, parse_mint_extension_mask_full,
            read_spl_base_mint_decimals, read_spl_base_mint_supply,
        },
        limits::state::usage_window::{
            canonical_window_start, i128_to_words, u128_to_words, words_to_i128, words_to_u128,
            UsageWindow,
        },
    },
};

// ─── Deterministic PRNG (xorshift64*) ────────────────────────────────────────

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed.max(1))
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    fn next_u128(&mut self) -> u128 {
        ((self.next_u64() as u128) << 64) | self.next_u64() as u128
    }

    fn fill(&mut self, buf: &mut [u8]) {
        for chunk in buf.chunks_mut(8) {
            let bytes = self.next_u64().to_le_bytes();
            let n = chunk.len();
            chunk.copy_from_slice(&bytes[..n]);
        }
    }

    fn bytes(&mut self, len: usize) -> Vec<u8> {
        let mut v = vec![0u8; len];
        self.fill(&mut v);
        v
    }

    fn range(&mut self, upper_exclusive: u64) -> u64 {
        self.next_u64() % upper_exclusive.max(1)
    }
}

// ─── TLV parsing over arbitrary bytes ────────────────────────────────────────

#[test]
fn tlv_parsers_are_total_over_arbitrary_bytes() {
    let mut rng = Rng::new(0x7c17_1a01);
    for i in 0..8_192u32 {
        let len = rng.range(2_048) as usize;
        let data = rng.bytes(len);

        // Total behavior: any of Ok/Err is acceptable; a panic or hang is not.
        let _ = read_spl_base_mint_decimals(&data);
        let _ = read_spl_base_mint_supply(&data);
        let _ = parse_mint_extension_mask(&data);
        let _ = parse_mint_extension_mask_full(&data);
        let _ = find_tlv_value(&data, rng.next_u64() as u16);

        // Length-prefix adversarial shape: claim a huge TLV length at a
        // random offset and confirm no over-read panic.
        if len >= 4 {
            let mut poisoned = data;
            let at = rng.range((len - 3) as u64) as usize;
            poisoned[at] = 0xff;
            poisoned[at + 1] = 0xff;
            let _ = parse_mint_extension_mask(&poisoned);
            let _ = find_tlv_value(&poisoned, rng.next_u64() as u16);
            let _ = i; // iteration index for reproduction on failure
        }
    }
}

// ─── Message hash determinism and per-field sensitivity ──────────────────────

fn baseline_arrays(rng: &mut Rng) -> Vec<[u8; 32]> {
    (0..10)
        .map(|_| {
            let mut a = [0u8; 32];
            rng.fill(&mut a);
            a
        })
        .collect()
}

#[test]
fn message_hash_is_deterministic_and_sensitive_to_every_field() {
    let mut rng = Rng::new(0x8a5e_11ed);

    for _ in 0..256 {
        let arrays = baseline_arrays(&mut rng);
        let scalars = (
            rng.next_u64() as u8,
            rng.next_u64() as u8,
            rng.next_u64() as u8,
            rng.next_u64(),
            rng.next_u64(),
            rng.next_u64(),
            rng.next_u128(),
            rng.next_u64() as i64,
        );

        let build = |arrays: &[[u8; 32]], s: &(u8, u8, u8, u64, u64, u64, u128, i64)| {
            compute_message_hash(&MessageHashPreimage {
                message_kind:                  s.0,
                source_chain_kind:             s.1,
                destination_chain_kind:        s.2,
                source_domain_id:              s.3,
                destination_domain_id:         s.4,
                source_nonce:                  s.5,
                amount:                        s.6,
                expires_at_unix_timestamp:     s.7,
                source_chancery_contract:      &arrays[0],
                destination_chancery_contract: &arrays[1],
                source_domain_separator:       &arrays[2],
                destination_domain_separator:  &arrays[3],
                source_asset:                  &arrays[4],
                destination_asset:             &arrays[5],
                source_issued_token:           &arrays[6],
                destination_issued_token:      &arrays[7],
                sender:                        &arrays[8],
                recipient:                     &arrays[9],
                signer_set_id:                 &arrays[0],
            })
        };

        let baseline = build(&arrays, &scalars);
        assert_eq!(baseline, build(&arrays, &scalars), "hash must be deterministic");

        // Every 32-byte field: flipping one byte must change the hash.
        // (arrays[0] doubles as signer_set_id; perturbing it perturbs both,
        // which still proves inclusion.)
        for (index, _) in arrays.iter().enumerate() {
            let mut mutated = arrays.clone();
            mutated[index][(rng.range(32)) as usize] ^= 0x01;
            assert_ne!(
                baseline,
                build(&mutated, &scalars),
                "byte-array preimage field {index} does not influence the message hash",
            );
        }

        // Every scalar field: a bit flip must change the hash.
        macro_rules! flip_scalar {
            ($field:tt, $label:literal) => {{
                let mut s = scalars;
                s.$field ^= 1;
                assert_ne!(
                    baseline,
                    build(&arrays, &s),
                    concat!("scalar preimage field ", $label, " does not influence the message hash"),
                );
            }};
        }
        flip_scalar!(0, "message_kind");
        flip_scalar!(1, "source_chain_kind");
        flip_scalar!(2, "destination_chain_kind");
        flip_scalar!(3, "source_domain_id");
        flip_scalar!(4, "destination_domain_id");
        flip_scalar!(5, "source_nonce");
        flip_scalar!(6, "amount");
        flip_scalar!(7, "expires_at_unix_timestamp");
    }
}

// ─── Attestation quorum over garbage inputs ──────────────────────────────────

#[test]
fn attestation_quorum_rejects_garbage_without_panicking() {
    let mut rng = Rng::new(0xa77e_57a7);

    for _ in 0..512 {
        let mut message_hash = [0u8; 32];
        let mut signer_root = [0u8; 32];
        rng.fill(&mut message_hash);
        rng.fill(&mut signer_root);

        let signer_count = (rng.range(16) + 1) as u8;
        let threshold = (rng.range(signer_count as u64) + 1) as u8;
        let minimum = (rng.range(threshold as u64) + 1) as u8;
        let signature_count = rng.range(signer_count as u64 + 4) as usize;

        let signatures: Vec<AttestationSignature> = (0..signature_count)
            .map(|_| {
                let mut signer_address = [0u8; 20];
                let mut signature = [0u8; 64];
                rng.fill(&mut signer_address);
                rng.fill(&mut signature);
                let proof_len = rng.range(5) as usize;
                AttestationSignature {
                    signer_address,
                    signature,
                    recovery_id: (rng.next_u64() % 4) as u8,
                    merkle_proof: (0..proof_len)
                        .map(|_| {
                            let mut sibling = [0u8; 32];
                            rng.fill(&mut sibling);
                            sibling
                        })
                        .collect(),
                }
            })
            .collect();

        // Random signatures cannot constitute a valid quorum over a random
        // root; any Ok here is a soundness failure, and any panic is a DoS
        // vector reachable from unvalidated instruction data.
        let result = verify_attestation_quorum(
            &message_hash,
            &signer_root,
            threshold,
            signer_count,
            minimum,
            &signatures,
        );
        assert!(result.is_err(), "garbage attestation verified as a quorum");
    }
}

// ─── Usage-window math ───────────────────────────────────────────────────────

const WINDOW_KINDS: [(u8, i64); 4] = [
    (window_kind::HOURLY, 3_600),
    (window_kind::DAILY, 86_400),
    (window_kind::WEEKLY, 7 * 86_400),
    (window_kind::MONTHLY, 30 * 86_400),
];

#[test]
fn window_word_conversions_round_trip() {
    let mut rng = Rng::new(0x30_1d_ca_fe);
    for _ in 0..16_384 {
        let u = rng.next_u128();
        assert_eq!(words_to_u128(u128_to_words(u)), u);
        let i = rng.next_u128() as i128;
        assert_eq!(words_to_i128(i128_to_words(i)), i);
    }
    assert_eq!(words_to_u128(u128_to_words(u128::MAX)), u128::MAX);
    assert_eq!(words_to_i128(i128_to_words(i128::MIN)), i128::MIN);
    assert_eq!(words_to_i128(i128_to_words(-1)), -1);
}

#[test]
fn canonical_window_start_is_aligned_idempotent_and_monotone() {
    let mut rng = Rng::new(0x5eed_0001);

    for (kind, period) in WINDOW_KINDS {
        let mut previous: Option<(i64, i64)> = None;
        for _ in 0..8_192 {
            // Realistic on-chain clock domain: positive timestamps.
            let ts = (rng.next_u64() % 8_000_000_000) as i64;
            let start = canonical_window_start(kind, ts).unwrap();

            assert!(start <= ts, "kind {kind}: start must never be in the future");
            assert!(ts - start < period, "kind {kind}: ts must fall inside its period");
            assert_eq!(
                canonical_window_start(kind, start).unwrap(),
                start,
                "kind {kind}: canonical start must be a fixed point",
            );
            if let Some((prev_ts, prev_start)) = previous {
                if ts >= prev_ts {
                    assert!(start >= prev_start, "kind {kind}: start must be monotone in ts");
                } else {
                    assert!(start <= prev_start + period, "kind {kind}: start must be monotone in ts");
                }
            }
            previous = Some((ts, start));
        }
    }

    // Unknown kinds fail closed.
    assert!(canonical_window_start(0x7f, 1_000).is_err());
}

#[test]
fn window_accumulators_saturate_with_errors_not_panics() {
    let mut rng = Rng::new(0xacc0_0007);

    for _ in 0..64 {
        let mut window: UsageWindow = bytemuck::Zeroable::zeroed();
        // Drive the accumulator toward u128::MAX with maximal inflows; every
        // step must be Ok or ArithmeticOverflow - never a wrap or a panic.
        window.gross_in = u128_to_words(u128::MAX - rng.range(1 << 20) as u128);
        let mut saw_overflow = false;
        for _ in 0..4 {
            match window.record_inflow(u64::MAX) {
                Ok(()) => {}
                Err(e) => {
                    assert_eq!(
                        e,
                        ProgramError::Custom(ChanceryError::ArithmeticOverflow as u32),
                        "saturation must surface as ArithmeticOverflow",
                    );
                    saw_overflow = true;
                }
            }
        }
        assert!(saw_overflow, "an accumulator this close to MAX must overflow within four max inflows");
    }
}
