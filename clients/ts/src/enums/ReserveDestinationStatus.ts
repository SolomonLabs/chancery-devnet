// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      enum
// File:          ReserveDestinationStatus.ts
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
export enum ReserveDestinationStatus {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    NONE = 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    DISABLED = 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    ENABLED = 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    DEPRECATED = 3,
}