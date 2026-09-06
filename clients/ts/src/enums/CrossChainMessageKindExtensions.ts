// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      enum
// File:          CrossChainMessageKindExtensions.ts
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
export enum CrossChainMessageKindExtensions {
    /** Remote lock → local release. the specification Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    INBOUND_RELEASE_FROM_REMOTE_LOCK = 3,
    /** Local lock → remote release. the specification Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    OUTBOUND_LOCK_FOR_REMOTE_RELEASE = 4,
}