// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          SettlementIntentCreatedLog.ts
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
export class SettlementIntentCreatedLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([134, 252, 95, 72, 170, 26, 178, 210]);
    readonly discriminator = SettlementIntentCreatedLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    riskClass!: U8;
    settlementIntent!: PublicKeyLike;
    intentId!: U8[];
    pathwayId!: U8[];
    pathwayPolicy!: PublicKeyLike;
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
    createdBy!: PublicKeyLike;

    constructor(props: DecodableProps<SettlementIntentCreatedLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): SettlementIntentCreatedLog {
        return Decoder.decode(data, SettlementIntentCreatedLog);
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
            riskClass: {
                type: SchemaFieldType.U8
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
            pathwayPolicy: {
                type: SchemaFieldType.Address
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
            createdBy: {
                type: SchemaFieldType.Address
            }
        };
    }
}
