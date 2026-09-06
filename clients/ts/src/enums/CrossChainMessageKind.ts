// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      enum
// File:          CrossChainMessageKind.ts
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
export enum CrossChainMessageKind {
    /** Remote burn → local mint. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    INBOUND_MINT_FROM_REMOTE_BURN = 0,
    /** Local burn → remote mint. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    OUTBOUND_REDEEM_TO_REMOTE_MINT = 1,
    /** Local burn → remote release (lock-and-release model). Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    OUTBOUND_REDEEM_TO_REMOTE_RELEASE = 2,
}