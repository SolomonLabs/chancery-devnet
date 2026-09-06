// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          CrossChainInboundConsumedLog.ts
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
import type { Base64, BI64, BU128, BU64, PublicKeyLike, U16, U8 } from "@solomon-labs/types";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/emit/cross_chain.rs
 */
export class CrossChainInboundConsumedLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([21, 199, 194, 205, 28, 199, 190, 238]);
    readonly discriminator = CrossChainInboundConsumedLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    messageHash!: U8[];
    canonVersion!: U16;
    messageKind!: U8;
    sourceChainKind!: U8;
    destinationChainKind!: U8;
    sourceDomainId!: BU64;
    destinationDomainId!: BU64;
    sourceNonce!: BU64;
    amount!: BU128;
    expiresAtUnixTimestamp!: BI64;
    sourceChanceryContract!: U8[];
    destinationChanceryContract!: U8[];
    sourceDomainSeparator!: U8[];
    destinationDomainSeparator!: U8[];
    sourceAsset!: U8[];
    destinationAsset!: U8[];
    sourceIssuedToken!: U8[];
    destinationIssuedToken!: U8[];
    sender!: U8[];
    recipient!: U8[];
    signerSetId!: U8[];
    pathwayId!: U8[];
    recipientTokenAccount!: PublicKeyLike;
    consumedBy!: PublicKeyLike;
    attestationCount!: U8;

    constructor(props: DecodableProps<CrossChainInboundConsumedLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): CrossChainInboundConsumedLog {
        return Decoder.decode(data, CrossChainInboundConsumedLog);
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
            messageHash: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            canonVersion: {
                type: SchemaFieldType.U16
            },
            messageKind: {
                type: SchemaFieldType.U8
            },
            sourceChainKind: {
                type: SchemaFieldType.U8
            },
            destinationChainKind: {
                type: SchemaFieldType.U8
            },
            sourceDomainId: {
                type: SchemaFieldType.U64
            },
            destinationDomainId: {
                type: SchemaFieldType.U64
            },
            sourceNonce: {
                type: SchemaFieldType.U64
            },
            amount: {
                type: SchemaFieldType.U128
            },
            expiresAtUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            sourceChanceryContract: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            destinationChanceryContract: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            sourceDomainSeparator: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            destinationDomainSeparator: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            sourceAsset: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            destinationAsset: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            sourceIssuedToken: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            destinationIssuedToken: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            sender: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            recipient: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            signerSetId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            pathwayId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            recipientTokenAccount: {
                type: SchemaFieldType.Address
            },
            consumedBy: {
                type: SchemaFieldType.Address
            },
            attestationCount: {
                type: SchemaFieldType.U8
            }
        };
    }
}
