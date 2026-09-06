// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      enum
// File:          ModuleStatus.ts
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
export enum ModuleStatus {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    NONE = 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    DISABLED = 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    ADMIN_ONLY = 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    ACTIVE = 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    EMERGENCY_DISABLED = 3,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    DEPRECATED = 4,
}