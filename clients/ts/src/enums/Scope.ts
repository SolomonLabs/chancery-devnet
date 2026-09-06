// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      enum
// File:          Scope.ts
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
export enum Scope {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    GLOBAL = 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    ASSET = 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    PATHWAY = 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    DESTINATION = 3,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    EXECUTOR = 4,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    MIGRATION = 5,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    ENFORCEMENT = 6,
    /** Cross-chain remote domain (scope_key = sha256(chain_kind || domain_id_be)). Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    REMOTE_DOMAIN = 7,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    ISSUED_TOKEN_CONTROL = 8,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    MODULE = 9,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    CROSS_CHAIN_SIGNER_SET = 10,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    TOKEN_ACCOUNT = 11,
    /** Per-counterparty limit dimension (scope_key = Pubkey::default() on the template LimitPolicy; usage windows are keyed per counterparty key). Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    COUNTERPARTY = 12,
}