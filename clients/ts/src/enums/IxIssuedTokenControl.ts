// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      enum
// File:          IxIssuedTokenControl.ts
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
export enum IxIssuedTokenControl {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    INITIALIZE_ISSUED_TOKEN_CONTROL = 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    UPDATE_ISSUED_TOKEN_CONTROL = 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    ACTIVATE_ISSUED_TOKEN_MODULE = 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    DEACTIVATE_ISSUED_TOKEN_MODULE = 3,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    SET_TRANSFER_HOOK_PROGRAM = 4,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    SET_PERMANENT_DELEGATE = 5,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    INITIALIZE_TOKEN_METADATA = 6,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    UPDATE_TOKEN_METADATA = 7,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    SET_DEFAULT_ACCOUNT_STATE = 8,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    SET_TOKEN_PAUSE_STATE = 9,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    CONFIGURE_CONFIDENTIAL_TRANSFER_MINT = 10,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    CONFIGURE_CONFIDENTIAL_TRANSFER_ACCOUNT = 11,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    REFRESH_ASSET_EXTENSION_OBSERVATION = 12,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    UPDATE_ASSET_EXTENSION_POLICY = 13,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    VERIFY_ISSUED_TOKEN_DEPLOYMENT = 14,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    REFRESH_ISSUED_TOKEN_EXTENSION_OBSERVATION = 15,
}