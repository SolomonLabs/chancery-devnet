// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          UsageWindowInitializedLog.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/emit/limit.rs
 */
export class UsageWindowInitializedLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([253, 117, 176, 12, 212, 44, 27, 248]);
    readonly discriminator = UsageWindowInitializedLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    riskClass!: U8;
    usageWindow!: PublicKeyLike;
    scopeHash!: U8[];
    windowKind!: U8;
    windowStartUnixTimestamp!: BI64;
    rentRefundRecipient!: PublicKeyLike;
    createdBy!: PublicKeyLike;

    constructor(props: DecodableProps<UsageWindowInitializedLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): UsageWindowInitializedLog {
        return Decoder.decode(data, UsageWindowInitializedLog);
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
            usageWindow: {
                type: SchemaFieldType.Address
            },
            scopeHash: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            windowKind: {
                type: SchemaFieldType.U8
            },
            windowStartUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            rentRefundRecipient: {
                type: SchemaFieldType.Address
            },
            createdBy: {
                type: SchemaFieldType.Address
            }
        };
    }
}
