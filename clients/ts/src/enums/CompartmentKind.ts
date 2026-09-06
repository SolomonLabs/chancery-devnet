// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      enum
// File:          CompartmentKind.ts
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
export enum CompartmentKind {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    INTAKE_QUARANTINE = 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    CLEARED_OPERATING = 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    RESTRICTED_REVIEW = 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    RECOVERY_OR_SEIZURE = 3,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    DOWNSTREAM_CUSTODY_FEED = 4,
}