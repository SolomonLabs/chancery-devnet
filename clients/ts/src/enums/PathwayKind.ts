// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      enum
// File:          PathwayKind.ts
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
export enum PathwayKind {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    DIRECT = 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    DELEGATED = 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    TRILATERAL = 2,
    /** Inbound: remote burn → local mint. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    CROSS_CHAIN_MINT = 3,
    /** Outbound: local burn → remote mint or release. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    CROSS_CHAIN_REDEEM = 4,
}