//! Rust unit tests for the cross-chain module.
//!
//! Scope: pure on-chain logic that does not require an SVM context.
//!   - `message_hash`: canonical preimage construction + SHA-256.
//!   - `attestation`: secp256k1_recover address derivation, sorted-pair
//!     Merkle proof verification.
//!   - `usage`: `scope_hash` derivation for the remote-domain usage window.
//!
//! Anything requiring transaction submission, account allocation, or
//! cross-instruction state lives in `tests/cross_chain/15_cross_chain.test.ts`
//! (litesvm-driven e2e at the repo root).

#[cfg(test)]
mod message_hash_canon {
    use crate::modules::cross_chain::message_hash::{
        compute_message_hash, write_preimage, MessageHashPreimage,
        MESSAGE_HASH_CANON_VERSION, MESSAGE_HASH_PREIMAGE_LEN,
    };

    /// Helper: build a preimage where every 32-byte field is filled with a
    /// distinct byte so any cross-field swap or wrong offset is detectable
    /// in the produced hash.
    fn distinctive_preimage<'a>(
        b1: &'a [u8; 32], b2: &'a [u8; 32], b3: &'a [u8; 32], b4: &'a [u8; 32],
        b5: &'a [u8; 32], b6: &'a [u8; 32], b7: &'a [u8; 32], b8: &'a [u8; 32],
        b9: &'a [u8; 32], b10: &'a [u8; 32], b11: &'a [u8; 32],
    ) -> MessageHashPreimage<'a> {
        MessageHashPreimage {
            message_kind:                  0x00,
            source_chain_kind:             0x01,
            destination_chain_kind:        0x00,
            source_domain_id:              1u64,
            destination_domain_id:         0u64,
            source_nonce:                  42u64,
            amount:                        1_000_000u128,
            expires_at_unix_timestamp:     1_700_000_000i64,
            source_chancery_contract:      b1,
            destination_chancery_contract: b2,
            source_domain_separator:       b3,
            destination_domain_separator:  b4,
            source_asset:                  b5,
            destination_asset:             b6,
            source_issued_token:           b7,
            destination_issued_token:      b8,
            sender:                        b9,
            recipient:                     b10,
            signer_set_id:                 b11,
        }
    }

    #[test]
    fn preimage_length_is_429() {
        assert_eq!(MESSAGE_HASH_PREIMAGE_LEN, 429);
    }

    #[test]
    fn write_preimage_produces_expected_layout() {
        let zeros   = [0u8; 32];
        let p       = MessageHashPreimage {
            message_kind:                  0,
            source_chain_kind:             0,
            destination_chain_kind:        0,
            source_domain_id:              0,
            destination_domain_id:         0,
            source_nonce:                  0,
            amount:                        0,
            expires_at_unix_timestamp:    0,
            source_chancery_contract:      &zeros,
            destination_chancery_contract: &zeros,
            source_domain_separator:       &zeros,
            destination_domain_separator:  &zeros,
            source_asset:                  &zeros,
            destination_asset:             &zeros,
            source_issued_token:           &zeros,
            destination_issued_token:      &zeros,
            sender:                        &zeros,
            recipient:                     &zeros,
            signer_set_id:                 &zeros,
        };
        let mut buf = [0u8; MESSAGE_HASH_PREIMAGE_LEN];
        let n       = write_preimage(&p, &mut buf).expect("full-size preimage buffer");

        assert_eq!(n, MESSAGE_HASH_PREIMAGE_LEN);

        // Tag at offset 0
        assert_eq!(&buf[0..23], b"CHANCERY_CROSS_CHAIN_V1");
        // canon_version = 2 BE at offset 23
        assert_eq!(&buf[23..25], &[0x00, 0x02]);
        // _pad0 at offset 28
        assert_eq!(buf[28], 0x00);

        // amount u128 BE all-zero at offset 53
        for i in 53..69 {
            assert_eq!(buf[i], 0);
        }
    }

    #[test]
    fn write_preimage_rejects_a_short_buffer_without_panicking() {
        let zeros = [0u8; 32];
        let p = MessageHashPreimage {
            message_kind:                  0,
            source_chain_kind:             0,
            destination_chain_kind:        0,
            source_domain_id:              0,
            destination_domain_id:         0,
            source_nonce:                  0,
            amount:                        0,
            expires_at_unix_timestamp:     0,
            source_chancery_contract:      &zeros,
            destination_chancery_contract: &zeros,
            source_domain_separator:       &zeros,
            destination_domain_separator:  &zeros,
            source_asset:                  &zeros,
            destination_asset:             &zeros,
            source_issued_token:           &zeros,
            destination_issued_token:      &zeros,
            sender:                        &zeros,
            recipient:                     &zeros,
            signer_set_id:                 &zeros,
        };
        let mut buf = [0u8; MESSAGE_HASH_PREIMAGE_LEN - 1];

        assert_eq!(
            write_preimage(&p, &mut buf),
            Err(solana_program_error::ProgramError::InvalidArgument),
        );
    }

    #[test]
    fn write_preimage_field_offsets_match_canon_doc() {
        // Build a preimage where each 32-byte field is filled with a unique
        // byte; assert each appears at its documented offset.
        let b1      = [0x11u8; 32];
        let b2      = [0x22u8; 32];
        let b3      = [0x33u8; 32];
        let b4      = [0x44u8; 32];
        let b5      = [0x55u8; 32];
        let b6      = [0x66u8; 32];
        let b7      = [0x77u8; 32];
        let b8      = [0x88u8; 32];
        let b9      = [0x99u8; 32];
        let b10     = [0xAAu8; 32];
        let b11     = [0xBBu8; 32];
        let p       = distinctive_preimage(&b1, &b2, &b3, &b4, &b5, &b6, &b7, &b8, &b9, &b10, &b11);

        let mut buf = [0u8; MESSAGE_HASH_PREIMAGE_LEN];

        write_preimage(&p, &mut buf).expect("full-size preimage buffer");

        // Offsets per the canon doc
        assert_eq!(&buf[ 77..109], &b1);
        assert_eq!(&buf[109..141], &b2);
        assert_eq!(&buf[141..173], &b3);
        assert_eq!(&buf[173..205], &b4);
        assert_eq!(&buf[205..237], &b5);
        assert_eq!(&buf[237..269], &b6);
        assert_eq!(&buf[269..301], &b7);
        assert_eq!(&buf[301..333], &b8);
        assert_eq!(&buf[333..365], &b9);
        assert_eq!(&buf[365..397], &b10);
        assert_eq!(&buf[397..429], &b11);
    }

    #[test]
    fn compute_message_hash_matches_explicit_preimage() {
        // The hash MUST equal sha256(write_preimage(p)) - i.e., the streaming
        // hashv path produces the same output as the contiguous-buffer path.
        let b1      = [0x11u8; 32];
        let b2      = [0x22u8; 32];
        let b3      = [0x33u8; 32];
        let b4      = [0x44u8; 32];
        let b5      = [0x55u8; 32];
        let b6      = [0x66u8; 32];
        let b7      = [0x77u8; 32];
        let b8      = [0x88u8; 32];
        let b9      = [0x99u8; 32];
        let b10     = [0xAAu8; 32];
        let b11     = [0xBBu8; 32];
        let p       = distinctive_preimage(&b1, &b2, &b3, &b4, &b5, &b6, &b7, &b8, &b9, &b10, &b11);

        let mut buf = [0u8; MESSAGE_HASH_PREIMAGE_LEN];

        write_preimage(&p, &mut buf).expect("full-size preimage buffer");

        let streaming  = compute_message_hash(&p);
        let contiguous = solana_sha256_hasher::hashv(&[&buf]).to_bytes();

        assert_eq!(streaming, contiguous);
    }

    #[test]
    fn flipping_any_field_changes_the_hash() {
        let zeros    = [0u8; 32];
        let mut base = MessageHashPreimage {
            message_kind:                  0,
            source_chain_kind:             0,
            destination_chain_kind:        0,
            source_domain_id:              0,
            destination_domain_id:         0,
            source_nonce:                  0,
            amount:                        0,
            expires_at_unix_timestamp:     0,
            source_chancery_contract:      &zeros,
            destination_chancery_contract: &zeros,
            source_domain_separator:       &zeros,
            destination_domain_separator:  &zeros,
            source_asset:                  &zeros,
            destination_asset:             &zeros,
            source_issued_token:           &zeros,
            destination_issued_token:      &zeros,
            sender:                        &zeros,
            recipient:                     &zeros,
            signer_set_id:                 &zeros,
        };
        let h0       = compute_message_hash(&base);

        base.message_kind = 1;

        let h1      = compute_message_hash(&base);

        assert_ne!(h0, h1);

        base.message_kind = 0;
        base.amount       = 1;

        let h2      = compute_message_hash(&base);

        assert_ne!(h0, h2);
        assert_ne!(h1, h2);

        base.amount       = 0;
        base.source_nonce = 1;

        let h3      = compute_message_hash(&base);

        assert_ne!(h0, h3);

        base.source_nonce = 0;

        let ones    = [1u8; 32];

        base.recipient   = &ones;

        let h4      = compute_message_hash(&base);

        assert_ne!(h0, h4);

        base.recipient     = &zeros;
        base.signer_set_id = &ones;

        let h5      = compute_message_hash(&base);

        assert_ne!(h0, h5);
        assert_ne!(h4, h5);
    }

    #[test]
    fn canon_version_is_two() {
        // Bumping canon version is a wire-format break - guard against
        // accidental change.
        assert_eq!(MESSAGE_HASH_CANON_VERSION, 2);
    }

    #[test]
    fn signer_set_id_is_bound_into_the_hash() {
        // Same message under different signer sets must hash differently.
        let b1  = [0x11u8; 32];
        let b2  = [0x22u8; 32];
        let b3  = [0x33u8; 32];
        let b4  = [0x44u8; 32];
        let b5  = [0x55u8; 32];
        let b6  = [0x66u8; 32];
        let b7  = [0x77u8; 32];
        let b8  = [0x88u8; 32];
        let b9  = [0x99u8; 32];
        let b10 = [0xAAu8; 32];

        let signer_set_a = [0xA1u8; 32];
        let signer_set_b = [0xB2u8; 32];

        let with_set_a = distinctive_preimage(
            &b1, &b2, &b3, &b4, &b5, &b6, &b7, &b8, &b9, &b10, &signer_set_a,
        );
        let with_set_b = distinctive_preimage(
            &b1, &b2, &b3, &b4, &b5, &b6, &b7, &b8, &b9, &b10, &signer_set_b,
        );

        assert_ne!(
            compute_message_hash(&with_set_a),
            compute_message_hash(&with_set_b),
            "signer_set_id must change the canonical hash",
        );
    }
}

#[cfg(test)]
mod attestation_merkle {
    use crate::modules::cross_chain::attestation::{
        verify_attestation_quorum, AttestationSignature,
    };
    use crate::error::ChanceryError;
    use solana_keccak_hasher::hashv as keccak_hashv;
    use solana_program_error::ProgramError;

    fn decode_hex_32(input: &str) -> [u8; 32] {
        assert_eq!(input.len(), 64, "expected 32-byte hex");
        let mut output = [0u8; 32];
        for index in 0..32 {
            output[index] = u8::from_str_radix(&input[index * 2..index * 2 + 2], 16)
                .expect("valid hex byte");
        }
        output
    }

    /// Builds a sorted-pair Merkle root over the given 20-byte addresses,
    /// matching the on-chain verifier convention. Test-only.
    fn build_root_and_proofs(addrs: &[[u8; 20]]) -> ([u8; 32], Vec<Vec<[u8; 32]>>) {
        // Sort + dedup
        let mut sorted: Vec<[u8; 20]> = addrs.to_vec();

        sorted.sort();
        sorted.dedup();

        // Pad to next power of 2 by duplicating the last leaf
        let mut n = 1usize;

        while n < sorted.len() {
            n *= 2;
        }

        while sorted.len() < n {
            let last = *sorted.last().unwrap();
            sorted.push(last);
        }

        // Compute leaves
        let leaves: Vec<[u8; 32]> = sorted
            .iter()
            .map(|a| keccak_hashv(&[a]).to_bytes())
            .collect();

        // Build levels bottom-up
        let mut levels: Vec<Vec<[u8; 32]>> = vec![leaves.clone()];
        let mut current                    = leaves.clone();

        while current.len() > 1 {
            let mut next: Vec<[u8; 32]> = Vec::with_capacity(current.len() / 2);
            let mut i                   = 0;

            while i < current.len() {
                let l = current[i];
                let r = current[i + 1];
                let parent = if l <= r {
                    keccak_hashv(&[&l, &r]).to_bytes()
                } else {
                    keccak_hashv(&[&r, &l]).to_bytes()
                };

                next.push(parent);

                i += 2;
            }

            levels.push(next.clone());

            current = next;
        }

        let root = levels.last().unwrap()[0];

        // Generate proof for each ORIGINAL address (look up in the padded sorted leaves)
        let mut proofs: Vec<Vec<[u8; 32]>> = Vec::with_capacity(addrs.len());

        for addr in addrs.iter() {
            let idx = sorted.iter().position(|a| a == addr).expect("addr must be in set");
            let mut proof:   Vec<[u8; 32]> = Vec::new();
            let mut cursor                 = idx;

            for level in 0..(levels.len() - 1) {
                let sibling_idx = cursor ^ 1;
                proof.push(levels[level][sibling_idx]);
                cursor >>= 1;
            }

            proofs.push(proof);
        }

        (root, proofs)
    }

    #[test]
    fn keccak_sorted_pair_known_answer_matches_typescript_vector() {
        let addresses = [
            [0x11u8; 20],
            [0x22u8; 20],
            [0x33u8; 20],
            [0x44u8; 20],
        ];
        let expected_leaves = [
            decode_hex_32("e2c07404b8c1df4c46226425cac68c28d27a766bbddce62309f36724839b22c0"),
            decode_hex_32("2ab0a4443bbea3fbe4d0e1503d11ff1367842fb0c8b28a5c8550f27599a40751"),
            decode_hex_32("37d95e0aa71e34defa88b4c43498bc8b90207e31ad0ef4aa6f5bea78bd25a1ab"),
            decode_hex_32("4cfa6af4bfa0111fd5e7625d43e84cd2d40629cf6008219d2c0e30ed48abf8b6"),
        ];
        let expected_first_level = [
            decode_hex_32("4beda981c9d34f2dd099131be6049a1d87676d227e63f4a409ee629043314b4f"),
            decode_hex_32("0aafebc39b02f78812dd98aa2d43138e57bf2e2129476469fcffb7c1d572f346"),
        ];
        let expected_root =
            decode_hex_32("8ea0e3a5b1bcc3d21d094be4a529068bb97ef23671d5a18bc24c5ae11cffdbf7");

        for index in 0..addresses.len() {
            assert_eq!(
                keccak_hashv(&[&addresses[index]]).to_bytes(),
                expected_leaves[index],
                "leaf {index} drift",
            );
        }

        let first_parent = keccak_hashv(&[&expected_leaves[1], &expected_leaves[0]]).to_bytes();
        let second_parent = keccak_hashv(&[&expected_leaves[2], &expected_leaves[3]]).to_bytes();
        assert_eq!(first_parent, expected_first_level[0]);
        assert_eq!(second_parent, expected_first_level[1]);

        let root = if first_parent <= second_parent {
            keccak_hashv(&[&first_parent, &second_parent]).to_bytes()
        } else {
            keccak_hashv(&[&second_parent, &first_parent]).to_bytes()
        };
        assert_eq!(root, expected_root);

        let (built_root, proofs) = build_root_and_proofs(&addresses);
        assert_eq!(built_root, expected_root);
        assert_eq!(proofs[0], vec![expected_leaves[1], expected_first_level[1]]);
    }

    #[test]
    fn merkle_proof_round_trip_4_addresses() {
        let a1             = [0x11u8; 20];
        let a2             = [0x22u8; 20];
        let a3             = [0x33u8; 20];
        let a4             = [0x44u8; 20];
        let addrs          = [a1, a2, a3, a4];
        let (root, proofs) = build_root_and_proofs(&addrs);

        // Manually walk each proof against the leaf and check it equals root
        for (i, addr) in addrs.iter().enumerate() {
            let mut current = keccak_hashv(&[addr]).to_bytes();

            for sibling in proofs[i].iter() {
                current = if current.as_slice() <= sibling.as_slice() {
                    keccak_hashv(&[&current, sibling]).to_bytes()
                } else {
                    keccak_hashv(&[sibling, &current]).to_bytes()
                };
            }

            assert_eq!(current, root, "proof for addr {i} failed");
        }
    }

    #[test]
    fn quorum_below_threshold_rejected() {
        // Construct a valid four-signer envelope with threshold=3 and no
        // signatures, so the failure is specifically quorum insufficiency.
        let dummy_msg_hash = [0u8; 32];
        let dummy_root     = [0u8; 32];
        let sigs: Vec<AttestationSignature> = vec![];

        let result = verify_attestation_quorum(
            &dummy_msg_hash,
            &dummy_root,
            3,  // threshold
            4,  // signer_count
            1,  // minimum_attestation_threshold
            &sigs,
        );

        match result {
            Err(e) => {
                assert_eq!(
                    e,
                    ProgramError::from(ChanceryError::AttestationThresholdNotMet),
                );
            }
            Ok(_) => panic!("expected AttestationThresholdNotMet"),
        }
    }

    #[test]
    fn quorum_threshold_below_minimum_floor_rejected() {
        // Even if the caller supplies "enough" sigs, the policy floor must be respected.
        let dummy_msg_hash = [0u8; 32];
        let dummy_root     = [0u8; 32];
        let sigs: Vec<AttestationSignature> = vec![];  // length irrelevant
        let result         = verify_attestation_quorum(
            &dummy_msg_hash,
            &dummy_root,
            2,  // threshold
            4,  // signer_count
            3,  // minimum_attestation_threshold (> threshold → reject)
            &sigs,
        );

        assert!(result.is_err());
    }
}

#[cfg(test)]
mod usage_scope_hash {
    use crate::modules::cross_chain::usage::{
        remote_domain_usage_window_scope_hash, REMOTE_DOMAIN_USAGE_SCOPE_KIND,
    };
    use solana_pubkey::Pubkey;
    use solana_sha256_hasher::hashv;

    #[test]
    fn scope_hash_matches_documented_construction() {
        // scope_hash = sha256([scope::REMOTE_DOMAIN] || policy_pda_bytes)
        let policy_pda = Pubkey::new_unique();
        let computed   = remote_domain_usage_window_scope_hash(&policy_pda);
        let expected   = hashv(&[
            &[REMOTE_DOMAIN_USAGE_SCOPE_KIND],
            policy_pda.as_ref(),
        ]).to_bytes();

        assert_eq!(computed, expected);
    }

    #[test]
    fn scope_hash_distinguishes_different_policies() {
        let p1 = Pubkey::new_unique();
        let p2 = Pubkey::new_unique();

        assert_ne!(
            remote_domain_usage_window_scope_hash(&p1),
            remote_domain_usage_window_scope_hash(&p2),
        );
    }

    #[test]
    fn scope_kind_uses_the_canonical_remote_domain_namespace() {
        assert_eq!(
            REMOTE_DOMAIN_USAGE_SCOPE_KIND,
            crate::constants::scope::REMOTE_DOMAIN,
        );
        assert_eq!(REMOTE_DOMAIN_USAGE_SCOPE_KIND, 0x07);
    }
}

// ─── RemoteNonce: the exactly-once replay primitive ───────────────────────────
//
// With the per-message `CrossChainMessage` account removed pre-deployment,
// strict nonce equality + atomic bump is the entire inbound replay boundary
// and monotonic `take_outbound` the entire outbound uniqueness guarantee.
// These tests pin those semantics.
mod remote_nonce_replay {
    use bytemuck::Zeroable;
    use solana_program_error::ProgramError;

    use crate::{
        error::ChanceryError,
        modules::cross_chain::state::remote_nonce::RemoteNonce,
    };

    fn nonce_at(inbound: u64, outbound: u64) -> RemoteNonce {
        let mut n = RemoteNonce::zeroed();
        n.next_inbound_nonce  = inbound;
        n.next_outbound_nonce = outbound;
        n
    }

    #[test]
    fn inbound_accepts_exact_expected_nonce() {
        let n = nonce_at(7, 0);

        assert!(n.assert_inbound_nonce(7).is_ok());
    }

    #[test]
    fn inbound_rejects_stale_nonce_as_replay() {
        // A replay re-presents an already-retired source_nonce.
        let n = nonce_at(8, 0);

        assert!(matches!(
            n.assert_inbound_nonce(7),
            Err(e) if e == ProgramError::Custom(ChanceryError::RemoteNonceMismatch as u32),
        ));
    }

    #[test]
    fn inbound_rejects_future_nonce_as_gap() {
        let n = nonce_at(7, 0);

        assert!(matches!(
            n.assert_inbound_nonce(9),
            Err(e) if e == ProgramError::Custom(ChanceryError::RemoteNonceMismatch as u32),
        ));
    }

    #[test]
    fn bump_retires_the_consumed_nonce() {
        let mut n = nonce_at(7, 0);

        assert!(n.assert_inbound_nonce(7).is_ok());
        n.bump_inbound().unwrap();

        // Exactly-once: the same message can never pass the check again.
        assert!(n.assert_inbound_nonce(7).is_err());
        assert!(n.assert_inbound_nonce(8).is_ok());
    }

    #[test]
    fn bump_inbound_overflow_fails_closed() {
        let mut n = nonce_at(u64::MAX, 0);

        assert!(matches!(
            n.bump_inbound(),
            Err(e) if e == ProgramError::Custom(ChanceryError::ArithmeticOverflow as u32),
        ));
        assert_eq!(n.next_inbound_nonce, u64::MAX);
    }

    #[test]
    fn take_outbound_returns_current_then_increments() {
        let mut n = nonce_at(0, 4);

        assert_eq!(n.take_outbound().unwrap(), 4);
        assert_eq!(n.next_outbound_nonce, 5);
        assert_eq!(n.take_outbound().unwrap(), 5);
    }

    #[test]
    fn take_outbound_overflow_fails_closed() {
        let mut n = nonce_at(0, u64::MAX);

        assert!(matches!(
            n.take_outbound(),
            Err(e) if e == ProgramError::Custom(ChanceryError::ArithmeticOverflow as u32),
        ));
        assert_eq!(n.next_outbound_nonce, u64::MAX);
    }
}

// ─── expire_inbound_message: terminal retirement classification ─────────────
mod expire_inbound_message_gate {
    use crate::{
        constants::{cross_chain_message_kind, inbound_message_retirement_reason},
        error::ChanceryError,
        modules::cross_chain::instructions::expire_inbound_message::classify_message_retirement,
    };

    fn classify(
        message_kind: u8,
        expires_at: i64,
        amount: u128,
        now: i64,
        expiry_window: u64,
        per_message: u64,
        per_day: u64,
    ) -> Result<u8, ChanceryError> {
        classify_message_retirement(
            message_kind,
            expires_at,
            amount,
            now,
            expiry_window,
            per_message,
            per_day,
            true,
            0,
            [0u64; 4],
        )
    }

    fn supported_kind() -> u8 {
        cross_chain_message_kind::INBOUND_MINT_FROM_REMOTE_BURN
    }

    #[test]
    fn live_message_allowed_by_current_policy_is_not_retirable() {
        assert_eq!(
            classify(supported_kind(), 2_000, 100, 1_000, 3_600, 1_000, 10_000),
            Err(ChanceryError::MessageNotExpired),
        );
        assert_eq!(
            classify(supported_kind(), 0, 100, i64::MAX, 0, 0, 0),
            Err(ChanceryError::MessageNotExpired),
        );
    }

    #[test]
    fn elapsed_expiry_is_retirable_at_the_boundary() {
        assert_eq!(
            classify(supported_kind(), 1_000, 100, 1_000, 0, 0, 0),
            Ok(inbound_message_retirement_reason::EXPIRY_LAPSED),
        );
        assert_eq!(
            classify(supported_kind(), 1_000, 100, 1_001, 0, 0, 0),
            Ok(inbound_message_retirement_reason::EXPIRY_LAPSED),
        );
    }

    #[test]
    fn finite_policy_window_retires_an_indefinite_head_message() {
        assert_eq!(
            classify(supported_kind(), 0, 100, 1_000, 3_600, 0, 0),
            Ok(inbound_message_retirement_reason::EXPIRY_POLICY_TIGHTENED),
        );
    }

    #[test]
    fn tightened_absolute_caps_retire_only_messages_that_can_never_fit() {
        assert_eq!(
            classify(supported_kind(), 2_000, 1_001, 1_000, 3_600, 1_000, 10_000),
            Ok(inbound_message_retirement_reason::PER_MESSAGE_CAP_TIGHTENED),
        );
        assert_eq!(
            classify(supported_kind(), 2_000, 10_001, 1_000, 3_600, 0, 10_000),
            Ok(inbound_message_retirement_reason::PER_DAY_CAP_TIGHTENED),
        );
        assert_eq!(
            classify(supported_kind(), 2_000, 10_000, 1_000, 3_600, 0, 10_000),
            Err(ChanceryError::MessageNotExpired),
        );
    }

    #[test]
    fn invalid_remote_asset_binding_and_pathway_cap_are_retirable() {
        assert_eq!(
            classify_message_retirement(
                supported_kind(),
                0,
                100,
                1_000,
                0,
                0,
                0,
                false,
                0,
                [0u64; 4],
            ),
            Ok(inbound_message_retirement_reason::REMOTE_ASSET_BINDING_INVALID),
        );
        assert_eq!(
            classify_message_retirement(
                supported_kind(),
                0,
                101,
                1_000,
                0,
                0,
                0,
                true,
                100,
                [0u64; 4],
            ),
            Ok(
                inbound_message_retirement_reason::PATHWAY_PER_TRANSACTION_CAP_TIGHTENED,
            ),
        );
        assert_eq!(
            classify_message_retirement(
                supported_kind(),
                0,
                100,
                1_000,
                0,
                0,
                0,
                true,
                100,
                [0u64; 4],
            ),
            Err(ChanceryError::MessageNotExpired),
        );
    }

    #[test]
    fn corridor_caps_take_precedence_over_binding_and_pathway_reasons() {
        assert_eq!(
            classify_message_retirement(
                supported_kind(),
                0,
                101,
                1_000,
                0,
                100,
                0,
                false,
                50,
                [0u64; 4],
            ),
            Ok(inbound_message_retirement_reason::PER_MESSAGE_CAP_TIGHTENED),
        );
    }

    #[test]
    fn future_expiry_outside_the_current_window_is_policy_retirable() {
        // The exact upper boundary remains consumable and therefore cannot be
        // retired through the policy-invalidated path.
        assert_eq!(
            classify(supported_kind(), 1_100, 100, 1_000, 100, 1_000, 10_000),
            Err(ChanceryError::MessageNotExpired),
        );

        assert_eq!(
            classify(supported_kind(), 1_101, 100, 1_000, 100, 1_000, 10_000),
            Ok(inbound_message_retirement_reason::EXPIRY_POLICY_TIGHTENED),
        );

        // The signed-time maximum must remain classifiable without overflow
        // instead of pinning the strict nonce head indefinitely.
        assert_eq!(
            classify(supported_kind(), i64::MAX, 100, 1_000, 100, 1_000, 10_000),
            Ok(inbound_message_retirement_reason::EXPIRY_POLICY_TIGHTENED),
        );
    }

    #[test]
    fn permanently_unexecutable_zero_expiry_heads_are_retirable() {
        assert_eq!(
            classify(0xFF, 0, 100, 1_000, 0, 0, 0),
            Ok(inbound_message_retirement_reason::UNSUPPORTED_MESSAGE_KIND),
        );
        assert_eq!(
            classify(supported_kind(), 0, 0, 1_000, 0, 0, 0),
            Ok(inbound_message_retirement_reason::ZERO_AMOUNT),
        );
        assert_eq!(
            classify(
                supported_kind(),
                0,
                u64::MAX as u128 + 1,
                1_000,
                0,
                0,
                0,
            ),
            Ok(inbound_message_retirement_reason::AMOUNT_OUT_OF_RANGE),
        );
    }
}

// ─── Spec 15 §15.4: epoch-free content hash ──────────────────────────────────
mod epoch_free_content_hash {
    use crate::modules::cross_chain::message_hash::{
        compute_epoch_free_content_hash, compute_message_hash, MessageHashPreimage,
        EPOCH_FREE_SIGNER_SET_ID,
    };

    fn preimage_with_signer_set<'a>(
        fixed:         &'a [u8; 32],
        signer_set_id: &'a [u8; 32],
    ) -> MessageHashPreimage<'a> {
        MessageHashPreimage {
            message_kind:                  0x01,
            source_chain_kind:             0x00,
            destination_chain_kind:        0x01,
            source_domain_id:              0u64,
            destination_domain_id:         7u64,
            source_nonce:                  9u64,
            amount:                        1_234u128,
            expires_at_unix_timestamp:     1_700_000_000i64,
            source_chancery_contract:      fixed,
            destination_chancery_contract: fixed,
            source_domain_separator:       fixed,
            destination_domain_separator:  fixed,
            source_asset:                  fixed,
            destination_asset:             fixed,
            source_issued_token:           fixed,
            destination_issued_token:      fixed,
            sender:                        fixed,
            recipient:                     fixed,
            signer_set_id,
        }
    }

    #[test]
    fn stable_across_signer_set_rotation() {
        // The rotation-stable identity: two epochs (different signer_set_id)
        // over the same economic content MUST hash identically.
        let fixed  = [0x11u8; 32];
        let epoch1 = [0xAAu8; 32];
        let epoch2 = [0xBBu8; 32];

        let h1 = compute_epoch_free_content_hash(&preimage_with_signer_set(&fixed, &epoch1));
        let h2 = compute_epoch_free_content_hash(&preimage_with_signer_set(&fixed, &epoch2));

        assert_eq!(h1, h2, "epoch-free hash must not depend on signer_set_id");
    }

    #[test]
    fn equals_canonical_hash_under_zeroed_signer_set() {
        // §15.4 definition: the canonical message hash with signer_set_id
        // fixed to 32 zero bytes.
        let fixed = [0x22u8; 32];
        let epoch = [0xCCu8; 32];

        let epoch_free = compute_epoch_free_content_hash(&preimage_with_signer_set(&fixed, &epoch));
        let canonical_zeroed =
            compute_message_hash(&preimage_with_signer_set(&fixed, &EPOCH_FREE_SIGNER_SET_ID));

        assert_eq!(epoch_free, canonical_zeroed);
    }

    #[test]
    fn differs_from_wire_hash_for_nonzero_epoch() {
        // The wire hash embeds the epoch; for any nonzero signer_set_id the
        // two hashes MUST differ (SHA-256 over differing preimages).
        let fixed = [0x33u8; 32];
        let epoch = [0x01u8; 32];
        let p     = preimage_with_signer_set(&fixed, &epoch);

        assert_ne!(compute_epoch_free_content_hash(&p), compute_message_hash(&p));
    }

    #[test]
    fn binds_every_economic_field() {
        // Flipping the amount changes the epoch-free identity: a forged
        // refund amount cannot reuse an honest emission's reclaim record slot
        // or its quorum-signed digest.
        let fixed = [0x44u8; 32];
        let epoch = [0x55u8; 32];

        let base   = preimage_with_signer_set(&fixed, &epoch);
        let mut alt = preimage_with_signer_set(&fixed, &epoch);
        alt.amount += 1;

        assert_ne!(
            compute_epoch_free_content_hash(&base),
            compute_epoch_free_content_hash(&alt),
        );
    }
}

// ─── Spec 15 §15.5: reclaim digest R ─────────────────────────────────────────
mod reclaim_digest_canon {
    use crate::{
        constants::{
            inbound_message_retirement_reason, CHANCERY_RECLAIM_TAG,
            CROSS_CHAIN_PROTOCOL_TAG,
        },
        modules::cross_chain::reclaim_digest::{
            compute_reclaim_digest, write_reclaim_digest_preimage, ReclaimDigestPreimage,
            RECLAIM_DIGEST_PREIMAGE_LEN,
        },
    };

    fn digest_preimage<'a>(
        source_contract:         &'a [u8; 32],
        destination_contract:    &'a [u8; 32],
        epoch_free_content_hash: &'a [u8; 32],
    ) -> ReclaimDigestPreimage<'a> {
        ReclaimDigestPreimage {
            source_chain_kind:             0x00,
            destination_chain_kind:        0x01,
            source_domain_id:              0u64,
            destination_domain_id:         7u64,
            source_chancery_contract:      source_contract,
            destination_chancery_contract: destination_contract,
            source_nonce:                  9u64,
            epoch_free_content_hash,
            retirement_reason:             inbound_message_retirement_reason::EXPIRY_LAPSED,
            expired_at_unix_timestamp:     1_700_000_100i64,
            expired_at_slot_or_block:      42_000u64,
        }
    }

    #[test]
    fn tag_differs_from_the_message_family() {
        // §15.5: RECLAIM_TAG MUST differ from the message tag so the two
        // attestation families are mutually unreplayable.
        assert_ne!(CHANCERY_RECLAIM_TAG, CROSS_CHAIN_PROTOCOL_TAG);
        assert_eq!(CHANCERY_RECLAIM_TAG, b"CHANCERY_RECLAIM_V1");
    }

    #[test]
    fn preimage_length_is_160() {
        assert_eq!(RECLAIM_DIGEST_PREIMAGE_LEN, 160);
    }

    #[test]
    fn write_preimage_agrees_with_hashv_construction() {
        use solana_sha256_hasher::hashv;

        let a = [0xA1u8; 32];
        let b = [0xB2u8; 32];
        let c = [0xC3u8; 32];
        let p = digest_preimage(&a, &b, &c);

        let mut buf = [0u8; RECLAIM_DIGEST_PREIMAGE_LEN];
        let written = write_reclaim_digest_preimage(&p, &mut buf);

        assert_eq!(written, RECLAIM_DIGEST_PREIMAGE_LEN);
        assert_eq!(hashv(&[&buf]).to_bytes(), compute_reclaim_digest(&p));
    }

    #[test]
    fn digest_is_deterministic_and_field_sensitive() {
        let a = [0x01u8; 32];
        let b = [0x02u8; 32];
        let c = [0x03u8; 32];

        let base = compute_reclaim_digest(&digest_preimage(&a, &b, &c));
        assert_eq!(base, compute_reclaim_digest(&digest_preimage(&a, &b, &c)));

        // Corridor identity fields prevent cross-corridor replay.
        let mut other_corridor = digest_preimage(&a, &b, &c);
        other_corridor.destination_domain_id = 8;
        assert_ne!(base, compute_reclaim_digest(&other_corridor));

        // source_nonce prevents intra-corridor replay.
        let mut other_nonce = digest_preimage(&a, &b, &c);
        other_nonce.source_nonce = 10;
        assert_ne!(base, compute_reclaim_digest(&other_nonce));

        // The expiry observation is bound: a forged expiry time is a
        // different authorization.
        let mut other_expiry = digest_preimage(&a, &b, &c);
        other_expiry.expired_at_unix_timestamp += 1;
        assert_ne!(base, compute_reclaim_digest(&other_expiry));


        let mut other_reason = digest_preimage(&a, &b, &c);
        other_reason.retirement_reason =
            inbound_message_retirement_reason::PER_MESSAGE_CAP_TIGHTENED;
        assert_ne!(base, compute_reclaim_digest(&other_reason));
    }

    #[test]
    fn digest_layout_offsets_are_pinned() {
        // Byte-for-byte pin of the §15.5 preimage layout. Any drift here is
        // a wire-format breaking change against the daughter contract.
        let a = [0xAAu8; 32];
        let b = [0xBBu8; 32];
        let c = [0xCCu8; 32];
        let p = digest_preimage(&a, &b, &c);

        let mut buf = [0u8; RECLAIM_DIGEST_PREIMAGE_LEN];
        write_reclaim_digest_preimage(&p, &mut buf);

        assert_eq!(&buf[0..19], b"CHANCERY_RECLAIM_V1");
        assert_eq!(&buf[19..21], &[0x00, 0x02]);              // canon_version 2 BE
        assert_eq!(buf[21], 0x00);                             // source_chain_kind
        assert_eq!(buf[22], 0x01);                             // destination_chain_kind
        assert_eq!(&buf[23..31], &0u64.to_be_bytes());         // source_domain_id
        assert_eq!(&buf[31..39], &7u64.to_be_bytes());         // destination_domain_id
        assert_eq!(&buf[39..71], &a);                          // source_chancery_contract
        assert_eq!(&buf[71..103], &b);                         // destination_chancery_contract
        assert_eq!(&buf[103..111], &9u64.to_be_bytes());       // source_nonce
        assert_eq!(&buf[111..143], &c);                        // epoch_free_content_hash
        assert_eq!(buf[143], inbound_message_retirement_reason::EXPIRY_LAPSED);
        assert_eq!(&buf[144..152], &1_700_000_100i64.to_be_bytes());
        assert_eq!(&buf[152..160], &42_000u64.to_be_bytes());
    }
}

// ─── Spec 15 §15.5: reclaim_expired_outbound pure gates ──────────────────────
mod reclaim_expired_outbound_gates {
    use solana_pubkey::Pubkey;

    use crate::{
        constants::{
            cross_chain_message_kind, cross_chain_message_kind_extensions,
            inbound_message_retirement_reason, programs, token_program,
        },
        error::ChanceryError,
        modules::cross_chain::instructions::reclaim_expired_outbound::{
            assert_canonical_sender_issued_token_account,
            assert_reclaim_retirement_condition, assert_reclaimable_outbound_kind,
            assert_source_nonce_emitted, narrow_net_reclaim_amount,
        },
    };

    #[test]
    fn only_outbound_burn_kinds_are_reclaimable() {
        assert!(assert_reclaimable_outbound_kind(
            cross_chain_message_kind::OUTBOUND_REDEEM_TO_REMOTE_MINT
        ).is_ok());
        assert!(assert_reclaimable_outbound_kind(
            cross_chain_message_kind::OUTBOUND_REDEEM_TO_REMOTE_RELEASE
        ).is_ok());

        // Inbound kinds select the daughter-side refund shape, never this one.
        assert_eq!(
            assert_reclaimable_outbound_kind(
                cross_chain_message_kind::INBOUND_MINT_FROM_REMOTE_BURN
            ),
            Err(ChanceryError::ReclaimMessageKindNotOutboundBurn),
        );
        assert_eq!(
            assert_reclaimable_outbound_kind(
                cross_chain_message_kind_extensions::INBOUND_RELEASE_FROM_REMOTE_LOCK
            ),
            Err(ChanceryError::ReclaimMessageKindNotOutboundBurn),
        );

        // The lock kind moved value by locking, not burning: re-minting it
        // would fabricate supply on top of the still-locked collateral.
        assert_eq!(
            assert_reclaimable_outbound_kind(
                cross_chain_message_kind_extensions::OUTBOUND_LOCK_FOR_REMOTE_RELEASE
            ),
            Err(ChanceryError::ReclaimMessageKindNotOutboundBurn),
        );
    }

    #[test]
    fn reclaim_condition_matches_the_authenticated_retirement_reason() {
        assert_eq!(
            assert_reclaim_retirement_condition(
                inbound_message_retirement_reason::EXPIRY_LAPSED,
                0,
                i64::MAX,
            ),
            Err(ChanceryError::MessageNotExpired),
        );
        assert_eq!(
            assert_reclaim_retirement_condition(
                inbound_message_retirement_reason::EXPIRY_LAPSED,
                1_000,
                999,
            ),
            Err(ChanceryError::MessageNotExpired),
        );
        assert!(assert_reclaim_retirement_condition(
            inbound_message_retirement_reason::EXPIRY_LAPSED,
            1_000,
            1_000,
        ).is_ok());

        assert!(assert_reclaim_retirement_condition(
            inbound_message_retirement_reason::EXPIRY_POLICY_TIGHTENED,
            0,
            0,
        ).is_ok());
        assert!(assert_reclaim_retirement_condition(
            inbound_message_retirement_reason::EXPIRY_POLICY_TIGHTENED,
            i64::MAX,
            0,
        ).is_ok());
        assert!(assert_reclaim_retirement_condition(
            inbound_message_retirement_reason::PER_MESSAGE_CAP_TIGHTENED,
            i64::MAX,
            0,
        ).is_ok());
        assert!(assert_reclaim_retirement_condition(
            inbound_message_retirement_reason::UNSUPPORTED_MESSAGE_KIND,
            0,
            0,
        ).is_ok());
        assert!(assert_reclaim_retirement_condition(
            inbound_message_retirement_reason::REMOTE_ASSET_BINDING_INVALID,
            0,
            0,
        ).is_ok());
        assert!(assert_reclaim_retirement_condition(
            inbound_message_retirement_reason::PATHWAY_PER_TRANSACTION_CAP_TIGHTENED,
            0,
            0,
        ).is_ok());
        assert!(assert_reclaim_retirement_condition(
            inbound_message_retirement_reason::PATHWAY_PERIOD_CAP_TIGHTENED,
            0,
            0,
        ).is_ok());
        assert!(assert_reclaim_retirement_condition(
            inbound_message_retirement_reason::RECIPIENT_ACCOUNT_FROZEN,
            0,
            0,
        ).is_ok());
        assert!(assert_reclaim_retirement_condition(
            inbound_message_retirement_reason::RECIPIENT_ACCOUNT_MISSING_OR_UNINITIALIZED,
            0,
            0,
        ).is_ok());
        assert_eq!(
            assert_reclaim_retirement_condition(0xFF, 0, 0),
            Err(ChanceryError::InvalidInboundRetirementReason),
        );
    }

    #[test]
    fn net_principal_amount_narrowing_is_checked() {
        assert_eq!(narrow_net_reclaim_amount(0), Ok(0));
        assert_eq!(narrow_net_reclaim_amount(u64::MAX as u128), Ok(u64::MAX));
        assert_eq!(
            narrow_net_reclaim_amount(u64::MAX as u128 + 1),
            Err(ChanceryError::ReclaimAmountExceedsTokenDomain),
        );
        assert_eq!(
            narrow_net_reclaim_amount(u128::MAX),
            Err(ChanceryError::ReclaimAmountExceedsTokenDomain),
        );
    }

    #[test]
    fn plausibility_floor_is_strictly_below_next_outbound() {
        // next_outbound_nonce is the NEXT nonce to be committed; nonce 0 has
        // been emitted iff next > 0.
        assert_eq!(
            assert_source_nonce_emitted(0, 0),
            Err(ChanceryError::ReclaimSourceNonceNotEmitted),
        );
        assert!(assert_source_nonce_emitted(0, 1).is_ok());
        assert!(assert_source_nonce_emitted(41, 42).is_ok());
        assert_eq!(
            assert_source_nonce_emitted(42, 42),
            Err(ChanceryError::ReclaimSourceNonceNotEmitted),
        );
        assert_eq!(
            assert_source_nonce_emitted(43, 42),
            Err(ChanceryError::ReclaimSourceNonceNotEmitted),
        );
    }

    #[test]
    fn refund_target_is_the_attested_senders_canonical_associated_token_account() {
        let sender = Pubkey::new_from_array([0x11; 32]);
        let issued_token_mint = Pubkey::new_from_array([0x22; 32]);

        let (canonical_token_2022_account, _) = Pubkey::find_program_address(
            &[
                sender.as_ref(),
                token_program::TOKEN_2022.as_ref(),
                issued_token_mint.as_ref(),
            ],
            &programs::ASSOCIATED_TOKEN_PROGRAM_ID,
        );

        assert!(assert_canonical_sender_issued_token_account(
            &canonical_token_2022_account,
            &sender,
            &token_program::TOKEN_2022,
            &issued_token_mint,
        )
        .is_ok());

        let (classic_token_account, _) = Pubkey::find_program_address(
            &[
                sender.as_ref(),
                token_program::SPL_TOKEN.as_ref(),
                issued_token_mint.as_ref(),
            ],
            &programs::ASSOCIATED_TOKEN_PROGRAM_ID,
        );

        assert_eq!(
            assert_canonical_sender_issued_token_account(
                &classic_token_account,
                &sender,
                &token_program::TOKEN_2022,
                &issued_token_mint,
            ),
            Err(ChanceryError::AccountKeyMismatch),
        );

        let mut alternate_account_bytes = canonical_token_2022_account.to_bytes();
        alternate_account_bytes[0] ^= 0x80;
        let alternate_sender_owned_account = Pubkey::new_from_array(alternate_account_bytes);

        assert_eq!(
            assert_canonical_sender_issued_token_account(
                &alternate_sender_owned_account,
                &sender,
                &token_program::TOKEN_2022,
                &issued_token_mint,
            ),
            Err(ChanceryError::AccountKeyMismatch),
        );
    }
}

// ─── Spec 15 §15.5: OutboundReclaimRecord layout + derivation ────────────────
mod outbound_reclaim_record_layout {
    use solana_pubkey::Pubkey;

    use crate::{
        constants::seeds,
        modules::cross_chain::state::outbound_reclaim_record::{
            OutboundReclaimRecord, OUTBOUND_RECLAIM_RECORD_SIZE,
        },
    };

    #[test]
    fn size_matches_declared_constant() {
        assert_eq!(
            core::mem::size_of::<OutboundReclaimRecord>(),
            OUTBOUND_RECLAIM_RECORD_SIZE,
        );
        assert_eq!(OUTBOUND_RECLAIM_RECORD_SIZE, 224);
    }

    #[test]
    fn pda_is_derived_from_the_emission_identity() {
        // H-01: the guard is keyed by (corridor, nonce), NOT by content -
        // every content claim for one emission maps to ONE record, so a
        // quorum signing divergent content objects cannot mint twice.
        let program_id = crate::id();

        let (pda_a1, bump_a1) = OutboundReclaimRecord::pda(0x01, 7, 9, &program_id);
        let (pda_a2, bump_a2) = OutboundReclaimRecord::pda(0x01, 7, 9, &program_id);

        assert_eq!(pda_a1, pda_a2);
        assert_eq!(bump_a1, bump_a2);

        // Distinct nonces, domains, and chain kinds map to distinct records.
        let (pda_other_nonce, _)  = OutboundReclaimRecord::pda(0x01, 7, 10, &program_id);
        let (pda_other_domain, _) = OutboundReclaimRecord::pda(0x01, 8, 9, &program_id);
        let (pda_other_chain, _)  = OutboundReclaimRecord::pda(0x02, 7, 9, &program_id);
        assert_ne!(pda_a1, pda_other_nonce);
        assert_ne!(pda_a1, pda_other_domain);
        assert_ne!(pda_a1, pda_other_chain);

        let (expected, _) = Pubkey::find_program_address(
            &[
                seeds::OUTBOUND_RECLAIM_RECORD,
                &[0x01],
                &7u64.to_be_bytes(),
                &9u64.to_be_bytes(),
            ],
            &program_id,
        );
        assert_eq!(pda_a1, expected);
    }
}

// ─── H-02: corridor identity immutability at the update boundary ─────────────
mod remote_domain_identity_freeze {
    use crate::{
        constants::remote_domain_mode,
        error::ChanceryError,
        modules::cross_chain::{
            change_detection::{assert_remote_domain_policy_value_valid, RemoteDomainPolicyValue},
            instructions::update_remote_domain_policy::{
                proposed_remote_domain_policy_value, UpdateRemoteDomainPolicyArgs,
            },
        },
    };

    fn current_release_value() -> RemoteDomainPolicyValue {
        RemoteDomainPolicyValue {
            remote_chain_kind:             0x01,
            minimum_attestation_threshold: 2,
            remote_domain_id:              7,
            required_finality_depth:       3,
            message_expiry_seconds:        3_600,
            per_message_maximum:           1_000_000,
            per_day_maximum:               10_000_000,
            pause_bits:                    0,
            remote_domain_separator:       [0x11; 32],
            remote_chancery_contract:      [0x22; 32],
            remote_issued_token:           [0x33; 32],
            signer_set_id:                 [0x44; 32],
            mode:                          remote_domain_mode::RELEASE,
            remote_asset:                  [0x55; 32],
            local_asset_mint:              [0x66; 32],
        }
    }

    fn no_op_args() -> UpdateRemoteDomainPolicyArgs {
        UpdateRemoteDomainPolicyArgs {
            remote_chain_kind:             0x01,
            remote_domain_id:              7,
            minimum_attestation_threshold: None,
            required_finality_depth:       None,
            message_expiry_seconds:        None,
            per_message_maximum:           None,
            per_day_maximum:               None,
            remote_domain_separator:       None,
            remote_chancery_contract:      None,
            remote_issued_token:           None,
            signer_set_id:                 None,
            mode:                          None,
            remote_asset:                  None,
        }
    }

    #[test]
    fn identity_and_mode_fields_are_frozen() {
        // Every field that feeds historical message-hash reconstruction or
        // historical asset binding must be immutable in place: mutating it
        // would strand the strict inbound nonce head unconsumable AND
        // unexpirable, and break the epoch-free hash reclaims depend on.
        let current = current_release_value();

        let mut separator = no_op_args();
        separator.remote_domain_separator = Some([0xAA; 32]);
        assert_eq!(
            proposed_remote_domain_policy_value(&current, &separator),
            Err(ChanceryError::ImmutableFieldChange),
        );

        let mut contract = no_op_args();
        contract.remote_chancery_contract = Some([0xBB; 32]);
        assert_eq!(
            proposed_remote_domain_policy_value(&current, &contract),
            Err(ChanceryError::ImmutableFieldChange),
        );

        let mut issued = no_op_args();
        issued.remote_issued_token = Some([0xCC; 32]);
        assert_eq!(
            proposed_remote_domain_policy_value(&current, &issued),
            Err(ChanceryError::ImmutableFieldChange),
        );

        let mut mode = no_op_args();
        mode.mode = Some(remote_domain_mode::MINT);
        mode.remote_asset = Some([0xDD; 32]);
        assert_eq!(
            proposed_remote_domain_policy_value(&current, &mode),
            Err(ChanceryError::ImmutableFieldChange),
        );

        let mut asset = no_op_args();
        asset.remote_asset = Some([0xEE; 32]);
        assert_eq!(
            proposed_remote_domain_policy_value(&current, &asset),
            Err(ChanceryError::ImmutableFieldChange),
        );
    }

    #[test]
    fn idempotent_identity_echoes_and_operational_updates_pass() {
        let current = current_release_value();
        assert_eq!(assert_remote_domain_policy_value_valid(&current), Ok(()));

        // Echoing each current identity field is a tolerated no-op for idempotent tooling.
        let mut echo = no_op_args();
        echo.remote_domain_separator  = Some(current.remote_domain_separator);
        echo.remote_chancery_contract = Some(current.remote_chancery_contract);
        echo.remote_issued_token      = Some(current.remote_issued_token);
        echo.mode                     = Some(current.mode);
        echo.remote_asset             = Some(current.remote_asset);
        let echoed = proposed_remote_domain_policy_value(&current, &echo).unwrap();
        assert_eq!(echoed.remote_domain_separator, current.remote_domain_separator);
        assert_eq!(echoed.mode, current.mode);
        assert_eq!(echoed.remote_asset, current.remote_asset);

        // Operational fields (outside the epoch-free preimage) stay mutable;
        // signer rotation stays legal (§15.4 exists precisely for it).
        let mut operational = no_op_args();
        operational.minimum_attestation_threshold = Some(3);
        operational.message_expiry_seconds        = Some(7_200);
        operational.signer_set_id                 = Some([0x99; 32]);
        let updated = proposed_remote_domain_policy_value(&current, &operational).unwrap();
        assert_eq!(updated.minimum_attestation_threshold, 3);
        assert_eq!(updated.message_expiry_seconds, 7_200);
        assert_eq!(updated.signer_set_id, [0x99; 32]);
        assert_eq!(updated.remote_domain_separator, current.remote_domain_separator);
        assert_eq!(updated.mode, current.mode);
        assert_eq!(updated.remote_asset, current.remote_asset);
    }
}
