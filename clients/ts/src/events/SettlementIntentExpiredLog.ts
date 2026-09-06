// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          SettlementIntentExpiredLog.ts
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
import type { DecodableProps, IsLogDecodable, Schema } from "@solomon-labs/solana-codec";
import { Decoder, SchemaFieldType } from "@solomon-labs/solana-codec";
import type { Base64, BI64, BU64, PublicKeyLike, U8 } from "@solomon-labs/types";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/emit/settlement.rs
 */
export class SettlementIntentExpiredLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([7, 134, 34, 178, 227, 33, 208, 200]);
    readonly discriminator = SettlementIntentExpiredLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    settlementIntent!: PublicKeyLike;
    intentId!: U8[];
    pathwayId!: U8[];
    settlementMode!: U8;
    settlementAction!: U8;
    principalA!: PublicKeyLike;
    principalB!: PublicKeyLike;
    executor!: PublicKeyLike;
    assetMint!: PublicKeyLike;
    issuedTokenMint!: PublicKeyLike;
    assetAmount!: BU64;
    issuedTokenAmount!: BU64;
    minimumAssetAmount!: BU64;
    minimumIssuedTokenAmount!: BU64;
    nonce!: BU64;
    validAfterUnixTimestamp!: BI64;
    expiresAtUnixTimestamp!: BI64;
    policyId!: U8[];
    intentHash!: U8[];
    rentRefundRecipient!: PublicKeyLike;

    constructor(props: DecodableProps<SettlementIntentExpiredLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): SettlementIntentExpiredLog {
        return Decoder.decode(data, SettlementIntentExpiredLog);
    }

    static getSchema(): Schema {
        return {
            sequenceNonce: {
                type: SchemaFieldType.U64
            },
            chancery: {
                type: SchemaFieldType.Address
            },
            slot: {
                type: SchemaFieldType.U64
            },
            unixTimestamp: {
                type: SchemaFieldType.I64
            },
            settlementIntent: {
                type: SchemaFieldType.Address
            },
            intentId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            pathwayId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            settlementMode: {
                type: SchemaFieldType.U8
            },
            settlementAction: {
                type: SchemaFieldType.U8
            },
            principalA: {
                type: SchemaFieldType.Address
            },
            principalB: {
                type: SchemaFieldType.Address
            },
            executor: {
                type: SchemaFieldType.Address
            },
            assetMint: {
                type: SchemaFieldType.Address
            },
            issuedTokenMint: {
                type: SchemaFieldType.Address
            },
            assetAmount: {
                type: SchemaFieldType.U64
            },
            issuedTokenAmount: {
                type: SchemaFieldType.U64
            },
            minimumAssetAmount: {
                type: SchemaFieldType.U64
            },
            minimumIssuedTokenAmount: {
                type: SchemaFieldType.U64
            },
            nonce: {
                type: SchemaFieldType.U64
            },
            validAfterUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            expiresAtUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            policyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            intentHash: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            rentRefundRecipient: {
                type: SchemaFieldType.Address
            }
        };
    }
}
