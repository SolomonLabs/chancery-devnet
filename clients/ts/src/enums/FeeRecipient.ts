// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      enum
// File:          FeeRecipient.ts
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
export enum FeeRecipient {
    /** Non-routing. No external recipient: the net fee is not routed. On mint it is not minted; on redeem it stays in the reserve. Requires `fee_recipient_key = Pubkey::default()`. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    NONE = 0,
    /** Routed to the protocol treasury token account. Requires a non-default approved `fee_recipient_key`. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    PROTOCOL_TREASURY = 1,
    /** Routed to an operator-owned wallet token account. Requires a non-default approved `fee_recipient_key`. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    OPERATOR_OWNED_WALLET = 2,
    /** Routed to a pathway-specific recipient token account. Requires a non-default approved `fee_recipient_key`. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    PATHWAY_SPECIFIC = 3,
    /** Non-routing. The net fee is retained in the collateral reserve (redeem) or left unminted (mint). Requires `fee_recipient_key = Pubkey::default()`. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    RESERVE_RETENTION = 4,
}