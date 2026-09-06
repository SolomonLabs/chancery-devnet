// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      enum
// File:          Module.ts
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
export enum Module {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    CORE = 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    EVENTS_CPI = 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    PERMISSIONS = 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    PATHWAY = 3,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    SETTLEMENT = 4,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    LIMITS = 5,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    EVIDENCE = 6,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    FEES = 7,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    RESERVE = 8,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    CONTROL = 9,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    MIGRATION = 10,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    ISSUED_TOKEN_CONTROL = 11,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    COMPARTMENTS = 12,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    PROVENANCE = 13,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    INSURANCE = 14,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    ENFORCEMENT = 15,
    /** Cross-chain mint/redeem. Spec §10. Active module dispatch. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    CROSS_CHAIN = 16,
}