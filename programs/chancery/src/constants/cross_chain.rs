//! Cross-chain constants. Spec §9.

// ─── Remote chain kinds (u8) ─────────────────────────────────────────────────
//
// `SOLANA` identifies the SVM/Solana wire family, not this deployment by
// itself. A remote corridor may legitimately target another SVM deployment.
// Full identity is the tuple of chain kind, domain id, contract/program id,
// and domain separator; `SOLANA_SELF_DOMAIN_ID` names only the local side.
pub mod chain_kind {
    pub const SOLANA: u8 = 0x00;
    pub const EVM:    u8 = 0x01;
    pub const COSMOS: u8 = 0x02;
    pub const APTOS:  u8 = 0x03;
    pub const SUI:    u8 = 0x04;
    // 0x05..=0xFF reserved
}

// ─── Cross-chain remote-domain mode ─────────────────────────────────
pub mod remote_domain_mode {
    pub const MINT:    u8 = 0x00;
    pub const RELEASE: u8 = 0x01;
}

// ─── Cross-chain message kinds (u8) ──────────────────────────────────────────
pub mod cross_chain_message_kind {
    /// Remote burn → local mint.
    pub const INBOUND_MINT_FROM_REMOTE_BURN:     u8 = 0x00;
    
    /// Local burn → remote mint.
    pub const OUTBOUND_REDEEM_TO_REMOTE_MINT:    u8 = 0x01;
    
    /// Local burn → remote release (lock-and-release model).
    pub const OUTBOUND_REDEEM_TO_REMOTE_RELEASE: u8 = 0x02;
}

/// to the cross-chain message kind discriminant. Kept in
/// a separate module so call sites can disambiguate v1 message kinds from
/// the lock-and-release additions while sharing the same on-wire u8
/// namespace.
pub mod cross_chain_message_kind_extensions {
    /// Remote lock → local release. the specification
    pub const INBOUND_RELEASE_FROM_REMOTE_LOCK:  u8 = 0x03;
    
    /// Local lock → remote release. the specification
    pub const OUTBOUND_LOCK_FOR_REMOTE_RELEASE:  u8 = 0x04;
}

/// Why an authenticated head message was retired without minting. The event
/// name remains `CrossChainInboundExpired` for wire compatibility, while this
/// discriminant distinguishes time expiry from policy invalidation.
pub mod inbound_message_retirement_reason {
    /// The message carried a nonzero expiry and `now >= expires_at`.
    pub const EXPIRY_LAPSED:                         u8 = 0x00;
    /// A finite policy window rejects a zero or excessively distant expiry.
    pub const EXPIRY_POLICY_TIGHTENED:               u8 = 0x01;
    /// The message amount exceeds the current absolute per-message cap.
    pub const PER_MESSAGE_CAP_TIGHTENED:             u8 = 0x02;
    /// The message amount exceeds the current absolute per-day cap, so it can
    /// never fit even in an empty daily window.
    pub const PER_DAY_CAP_TIGHTENED:                 u8 = 0x03;
    /// The authenticated message kind is not executable by this binary.
    pub const UNSUPPORTED_MESSAGE_KIND:              u8 = 0x04;
    /// The authenticated message carries a zero amount, which consume rejects.
    pub const ZERO_AMOUNT:                           u8 = 0x05;
    /// The canonical u128 amount exceeds Solana's u64 token amount domain.
    pub const AMOUNT_OUT_OF_RANGE:                   u8 = 0x06;
    /// The supported message kind cannot satisfy the corridor's mode/asset
    /// binding and therefore can never execute under the attested content.
    pub const REMOTE_ASSET_BINDING_INVALID:          u8 = 0x07;
    /// The amount exceeds the pathway's absolute per-transaction cap.
    pub const PATHWAY_PER_TRANSACTION_CAP_TIGHTENED: u8 = 0x08;
    /// The amount exceeds one of the pathway's absolute fixed-window volume caps
    /// (hourly/daily/seven-day/thirty-day), so it can never fit even in an empty
    /// window and is terminally unexecutable. Distinct from transient window
    /// saturation, which clears with time and is deliberately not retired.
    pub const PATHWAY_PERIOD_CAP_TIGHTENED:          u8 = 0x09;
    /// The attested recipient's exact canonical issued-token ATA exists and is
    /// Frozen.
    pub const RECIPIENT_ACCOUNT_FROZEN:              u8 = 0x0A;
    /// The exact canonical issued-token ATA is absent or exists as an
    /// uninitialized SPL token account. Recipients must create and initialize
    /// their ATA before initiating the source-chain transfer.
    pub const RECIPIENT_ACCOUNT_MISSING_OR_UNINITIALIZED: u8 = 0x0B;
}

// Cross-chain message PDA status constants retired pre-deployment with the
// per-message `CrossChainMessage` account: strict `RemoteNonce` equality is
// the exactly-once primitive, and the canonical consumption/emission record
// is the evidence event stream.

// ─── Remote-domain pause bits (u64) ──────────────────────────────────────────
//
// Bits stored in `RemoteDomainPolicy.status_flags` alongside
// `status_flag::INITIALIZED` (bit 0).
//
// Pause bits start at bit 16 to leave room for future status_flag::* values
// in bits 1-15 without textual collision in the same u64.
//
// Emergency authority may move toward restriction (bitwise-OR new bits).
// Relaxation requires governance authority (bitwise-AND-NOT).
//
// Operation matrix:
//   INBOUND_MESSAGES       -> consume inbound
//   OUTBOUND_MESSAGES      -> emit outbound
//   REMOTE_MINT            -> consume inbound mint/release corridor
//   REMOTE_REDEEM          -> emit outbound redeem corridor
//   ATTESTATION_ACCEPTANCE -> every attestation-consuming path: inbound
//                             consume, authenticated inbound retirement, and
//                             outbound reclaim
//   DAUGHTER_CONTRACT      -> both directions; disables interaction with the
//                             configured daughter contract as a whole
pub mod remote_domain_pause_bit {
    pub const INBOUND_MESSAGES:       u64 = 1 << 16;
    pub const OUTBOUND_MESSAGES:      u64 = 1 << 17;
    pub const REMOTE_MINT:            u64 = 1 << 18;
    pub const REMOTE_REDEEM:          u64 = 1 << 19;
    pub const ATTESTATION_ACCEPTANCE: u64 = 1 << 20;
    pub const DAUGHTER_CONTRACT:      u64 = 1 << 21;
    pub const ALL: u64 =
          INBOUND_MESSAGES
        | OUTBOUND_MESSAGES
        | REMOTE_MINT
        | REMOTE_REDEEM
        | ATTESTATION_ACCEPTANCE
        | DAUGHTER_CONTRACT;
}

// ─── Signer set status flag bits (u64) ───────────────────────────────────────
//
// Bits stored in `CrossChainSignerSet.status_flags` alongside
// `status_flag::INITIALIZED` (bit 0). Bits start at 16 to mirror the
// pause-bit convention.
pub mod cross_chain_signer_set_flag {
    /// Set explicitly when the signer set is revoked outside of normal
    /// expiry - e.g. a key compromise. Verification rejects revoked sets
    /// even within their validity window.
    pub const REVOKED: u64 = 1 << 16;
}

// ─── Protocol tag (preimage prefix) ──────────────────────────────────────────
//
// Domain-separation prefix for the cross-chain message hash preimage.
// Spec 10 §10.5. Mirrored exactly in the EVM daughter contract.
// A bump here is a wire-format breaking change.
pub const CROSS_CHAIN_PROTOCOL_TAG: &[u8] = b"CHANCERY_CROSS_CHAIN_V1";

// ─── Reclaim tag (attestation-digest preimage prefix) ────────────────────────
//
// Domain-separation prefix for the post-retirement reclaim attestation digest R
// (spec 15 §15.5). A NEW tag in the §13.6 family: it MUST differ from
// `CROSS_CHAIN_PROTOCOL_TAG` so no message attestation is replayable as a
// reclaim authorization or vice versa. Mirrored exactly in the EVM daughter
// contract's `reclaimExpiredEmission` verifier. This preproduction tag names
// the sole canonical 160-byte layout; any edit must update both sides before
// corridor activation. No legacy decoder is retained.
pub const CHANCERY_RECLAIM_TAG: &[u8] = b"CHANCERY_RECLAIM_V1";

// ─── This-chancery self-identity ─────────────────────────────────────────────
//
// The `source_domain_id` field stamped into every outbound message hash and
// expected on every inbound message hash. For a single Solana chancery
// deployment this is `0`; per-chain disambiguation is already provided by
// the 32-byte `domain_separator` on `ChanceryConfig`. Multi-chancery
// deployments on the same chain (e.g. mainnet vs testnet handled by separate
// programs) MAY use the same value because their `domain_separator` differs.
//
// If a future deployment needs to vary this per-chancery, promote it to a
// `self_domain_id` field on `ChanceryConfig` (taking 8 bytes from
// `_reserved`) and update both `emit_outbound_message` and
// `consume_inbound_message` to read it from config.
pub const SOLANA_SELF_DOMAIN_ID: u64 = 0;
