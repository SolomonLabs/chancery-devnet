// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      enum
// File:          IxCrossChain.ts
//
// Source IDL:    programs/chancery/idl/idl.json
// Output Dir:    clients/ts
//
// Introspection: disabled
// Confidence:    medium
//
// To regenerate: Run the IDL generator with the same options, or use
//                `idl-generator --use-last` to repeat the last run.
//
// ============================================================================
// @end-generated-idl
export enum IxCrossChain {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    REGISTER_REMOTE_DOMAIN_POLICY = 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    UPDATE_REMOTE_DOMAIN_POLICY = 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    REGISTER_CROSS_CHAIN_SIGNER_SET = 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    ROTATE_CROSS_CHAIN_SIGNER_SET = 3,
    /** OR-only restriction. Emergency authority. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    RESTRICT_REMOTE_DOMAIN_PAUSE = 4,
    /** AND-NOT relaxation. Governance authority. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    RELAX_REMOTE_DOMAIN_PAUSE = 5,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    CONSUME_INBOUND_MESSAGE = 6,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    EMIT_OUTBOUND_MESSAGE = 7,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    UPDATE_REMOTE_DOMAIN_POLICY_WITH_PENDING_CHANGE = 8,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    RELAX_REMOTE_DOMAIN_PAUSE_WITH_PENDING_CHANGE = 9,
    /** Permissionless authenticated terminal retirement of the strict inbound nonce head (SL-01 remedy): consume minus value effects. A message may retire after its nonzero expiry lapses or after a policy tightening makes it permanently unexecutable. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    EXPIRE_INBOUND_MESSAGE = 10,
    /** Permissionless post-retirement recovery of a burned outbound emission (spec 15 §15.5 instruction B): re-mints the canonical net principal to the original sender against a quorum-attested daughter retirement (`InboundMessageExpired` / E2) digest. Emission-time effective fees remain non-refundable. Guarded by the permanent single-shot `OutboundReclaimRecord` PDA. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    RECLAIM_EXPIRED_OUTBOUND = 11,
}