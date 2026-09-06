// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      enum
// File:          IxCore.ts
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
export enum IxCore {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    INITIALIZE_CHANCERY = 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    REGISTER_ASSET = 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    UPDATE_ASSET_CONFIG = 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    SET_ASSET_MODE = 3,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    PROPOSE_AUTHORITY_TRANSFER = 4,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    ACCEPT_AUTHORITY_TRANSFER = 5,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    UPDATE_ASSET_CONFIG_WITH_PENDING_CHANGE = 6,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    SET_ASSET_MODE_WITH_PENDING_CHANGE = 7,
}