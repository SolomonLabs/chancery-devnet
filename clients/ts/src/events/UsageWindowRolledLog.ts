// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          UsageWindowRolledLog.ts
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
import type { Base64, BI128, BI64, BU128, BU64, PublicKeyLike, U32, U8 } from "@solomon-labs/types";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/emit/limit.rs
 */
export class UsageWindowRolledLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([175, 151, 129, 201, 3, 184, 200, 19]);
    readonly discriminator = UsageWindowRolledLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    usageWindow!: PublicKeyLike;
    scopeHash!: U8[];
    windowKind!: U8;
    previousWindowStartUnixTimestamp!: BI64;
    newWindowStartUnixTimestamp!: BI64;
    finalGrossIn!: BU128;
    finalGrossOutputAmount!: BU128;
    finalNetFlow!: BI128;
    finalActionCount!: U32;

    constructor(props: DecodableProps<UsageWindowRolledLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): UsageWindowRolledLog {
        return Decoder.decode(data, UsageWindowRolledLog);
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
            previousWindowStartUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            newWindowStartUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            finalGrossIn: {
                type: SchemaFieldType.U128
            },
            finalGrossOutputAmount: {
                type: SchemaFieldType.U128
            },
            finalNetFlow: {
                type: SchemaFieldType.I128
            },
            finalActionCount: {
                type: SchemaFieldType.U32
            }
        };
    }
}
