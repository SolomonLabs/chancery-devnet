// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      enum
// File:          ChangeKind.ts
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
export enum ChangeKind {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    UPDATE_LIMIT_POLICY = 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    UPDATE_FEE_POLICY = 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    UPDATE_PATHWAY_POLICY = 3,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    SET_PATHWAY_STATUS = 4,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    UPSERT_PERMISSION = 5,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    REGISTER_RESERVE_DESTINATION = 6,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    SET_RESERVE_DESTINATION_STATUS = 7,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    UPDATE_ISSUED_TOKEN_CONTROL = 8,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    ACTIVATE_ISSUED_TOKEN_MODULE = 9,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    SET_TRANSFER_HOOK_PROGRAM = 10,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    SET_PERMANENT_DELEGATE = 11,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    CONFIGURE_CONFIDENTIAL_TRANSFER = 12,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    UPDATE_REMOTE_DOMAIN_POLICY = 13,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    ROTATE_CROSS_CHAIN_SIGNER_SET = 14,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    SET_MODULE_STATUS = 15,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    UPDATE_ASSET_CONFIG = 16,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    SET_ASSET_MODE = 17,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    REGISTER_CROSS_CHAIN_SIGNER_SET = 18,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    RELAX_REMOTE_DOMAIN_PAUSE = 19,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    UPDATE_EVIDENCE_POLICY = 20,
}