// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      enum
// File:          IxControl.ts
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
export enum IxControl {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    SET_GLOBAL_PAUSE = 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    SET_ASSET_PAUSE = 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    SET_PATHWAY_PAUSE = 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    SET_EXECUTOR_PAUSE = 3,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    SET_COUNTERPARTY_PAUSE = 4,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    FREEZE_ISSUED_TOKEN_ACCOUNT = 5,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    THAW_ISSUED_TOKEN_ACCOUNT = 6,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    PROPOSE_CONFIG_CHANGE = 7,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    ACCEPT_CONFIG_CHANGE = 8,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    CANCEL_CONFIG_CHANGE = 9,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    INITIALIZE_MODULE_ACTIVATION_STATE = 10,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    SET_MODULE_STATUS = 11,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    SET_MODULE_STATUS_WITH_PENDING_CHANGE = 12,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    CLOSE_EXPIRED_CONFIG_CHANGE = 13,
}