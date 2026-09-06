// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      enum
// File:          IxEventsCpi.ts
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
export enum IxEventsCpi {
    /** No-op handler. Receives self-CPIs from emit_event; within a finalized, successfully committed transaction, the inner instruction is the evidence record. Decoders must reject failed transaction metadata. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
    EMIT = 0,
}