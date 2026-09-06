// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      enum
// File:          InboundMessageRetirementReason.ts
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
export enum InboundMessageRetirementReason {
    /** The message carried a nonzero expiry and `now >= expires_at`. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    EXPIRY_LAPSED = 0,
    /** A finite policy window rejects a zero or excessively distant expiry. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    EXPIRY_POLICY_TIGHTENED = 1,
    /** The message amount exceeds the current absolute per-message cap. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    PER_MESSAGE_CAP_TIGHTENED = 2,
    /** The message amount exceeds the current absolute per-day cap, so it can never fit even in an empty daily window. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    PER_DAY_CAP_TIGHTENED = 3,
    /** The authenticated message kind is not executable by this binary. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    UNSUPPORTED_MESSAGE_KIND = 4,
    /** The authenticated message carries a zero amount, which consume rejects. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    ZERO_AMOUNT = 5,
    /** The canonical u128 amount exceeds Solana's u64 token amount domain. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    AMOUNT_OUT_OF_RANGE = 6,
    /** The supported message kind cannot satisfy the corridor's mode/asset binding and therefore can never execute under the attested content. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    REMOTE_ASSET_BINDING_INVALID = 7,
    /** The amount exceeds the pathway's absolute per-transaction cap. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    PATHWAY_PER_TRANSACTION_CAP_TIGHTENED = 8,
    /** The amount exceeds one of the pathway's absolute fixed-window volume caps (hourly/daily/seven-day/thirty-day), so it can never fit even in an empty window and is terminally unexecutable. Distinct from transient window saturation, which clears with time and is deliberately not retired. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    PATHWAY_PERIOD_CAP_TIGHTENED = 9,
    /** The attested recipient's exact canonical issued-token ATA exists and is Frozen. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    RECIPIENT_ACCOUNT_FROZEN = 10,
    /** The exact canonical issued-token ATA is absent or exists as an uninitialized SPL token account. Recipients must create and initialize their ATA before initiating the source-chain transfer. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    RECIPIENT_ACCOUNT_MISSING_OR_UNINITIALIZED = 11,
}