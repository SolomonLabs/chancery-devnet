//! PDA seed prefixes.
//!
//! These string literals are baked into PDA derivations on-chain. Renaming a
//! seed is an account-layout breaking change - every PDA derived under the
//! old seed becomes unreachable. Append-only.

pub mod seeds {
    pub const CHANCERY_CONFIG:                 &[u8] = b"chancery-config";
    pub const ASSET_CONFIG:                    &[u8] = b"asset-config";
    pub const PERMISSION:                      &[u8] = b"permission";
    pub const PATHWAY_POLICY:                  &[u8] = b"pathway-policy";
    pub const SETTLEMENT_POLICY:               &[u8] = b"settlement-policy";
    pub const SETTLEMENT_INTENT:               &[u8] = b"settlement-intent";
    pub const LIMIT_POLICY:                    &[u8] = b"limit-policy";
    pub const USAGE_WINDOW:                    &[u8] = b"usage-window";
    pub const EVIDENCE_POLICY:                 &[u8] = b"evidence-policy";
    pub const FEE_POLICY:                      &[u8] = b"fee-policy";
    pub const INSURANCE_POLICY:                &[u8] = b"insurance-policy";
    pub const RESERVE_COMPARTMENT:             &[u8] = b"reserve-compartment";
    pub const PROVENANCE_CASE:                 &[u8] = b"provenance-case";
    pub const RESERVE_DESTINATION:             &[u8] = b"reserve-destination";
    pub const PAUSE_STATE:                     &[u8] = b"pause-state";
    pub const ASSET_PAUSE:                     &[u8] = b"asset-pause";
    pub const AUTHORITY_TRANSFER:              &[u8] = b"authority-transfer";
    pub const LEGACY_MIGRATION:                &[u8] = b"legacy-migration";
    pub const INSURANCE_CLAIM_NOTICE:          &[u8] = b"insurance-claim-notice";
    pub const REPORT_FIELD:                    &[u8] = b"report-field";
    pub const ENFORCEMENT_CASE:                &[u8] = b"enforcement-case";
    pub const MINT_AUTHORITY:                  &[u8] = b"mint-authority";
    pub const FREEZE_AUTHORITY:                &[u8] = b"freeze-authority";
    pub const RESERVE_AUTHORITY:               &[u8] = b"reserve-authority";
    pub const ISSUED_TOKEN_CONTROL:            &[u8] = b"issued-token-control";
    pub const CLOSE_MINT_AUTHORITY:            &[u8] = b"close-mint-authority";
    pub const TRANSFER_HOOK_AUTHORITY:         &[u8] = b"transfer-hook-authority";
    pub const PERMANENT_DELEGATE_AUTHORITY:    &[u8] = b"permanent-delegate-authority";
    pub const METADATA_POINTER_AUTHORITY:      &[u8] = b"metadata-pointer-authority";
    pub const METADATA_UPDATE_AUTHORITY:       &[u8] = b"metadata-update-authority";
    pub const PAUSE_AUTHORITY:                 &[u8] = b"pause-authority";
    pub const CONFIDENTIAL_TRANSFER_AUTHORITY: &[u8] = b"confidential-transfer-authority";
    pub const DEFAULT_ACCOUNT_STATE_AUTHORITY: &[u8] = b"default-account-state-authority";

    // ── Cross-chain (later module) ──
    pub const REMOTE_DOMAIN_POLICY:            &[u8] = b"remote-domain-policy";
    pub const CROSS_CHAIN_SIGNER_SET:          &[u8] = b"cross-chain-signer-set";
    pub const REMOTE_NONCE:                    &[u8] = b"remote-nonce";

    /// Spec 15 §15.5 (H-01 remediated): permanent single-shot double-reclaim
    /// guard for terminally retired outbound emissions, keyed by EMISSION IDENTITY -
    /// [prefix, remote_chain_kind, remote_domain_id BE, source_nonce BE] -
    /// enforcing reclaim_count(corridor, nonce) <= 1 across every content
    /// claim. NEVER closeable - closure would re-enable the reclaim.
    pub const OUTBOUND_RECLAIM_RECORD:         &[u8] = b"outbound-reclaim";

    // ── the specification, 10, 11 ──
    pub const BASIC_FREEZE_RECORD:             &[u8] = b"basic-freeze-record";
    pub const PENDING_CONFIG_CHANGE:           &[u8] = b"pending-config-change";
    pub const MODULE_ACTIVATION_STATE:         &[u8] = b"module-activation-state";

    /// PDA that signs every event CPI emit. Seed is sole element. Bump is
    /// cached on `ChanceryConfig.event_authority_bump` to avoid re-derivation
    /// in every emitting handler.
    pub const EVENT_AUTHORITY:                 &[u8] = b"event-authority";
}
