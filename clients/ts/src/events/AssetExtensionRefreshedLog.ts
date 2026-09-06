// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          AssetExtensionRefreshedLog.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/emit/asset.rs
 */
export class AssetExtensionRefreshedLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([120, 68, 221, 88, 88, 17, 192, 244]);
    readonly discriminator = AssetExtensionRefreshedLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    riskClass!: U8;
    assetConfig!: PublicKeyLike;
    assetMint!: PublicKeyLike;
    oldObservedExtensionMask!: BU64[];
    newObservedExtensionMask!: BU64[];
    refreshedBy!: PublicKeyLike;

    constructor(props: DecodableProps<AssetExtensionRefreshedLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): AssetExtensionRefreshedLog {
        return Decoder.decode(data, AssetExtensionRefreshedLog);
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
            assetConfig: {
                type: SchemaFieldType.Address
            },
            assetMint: {
                type: SchemaFieldType.Address
            },
            oldObservedExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            newObservedExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            refreshedBy: {
                type: SchemaFieldType.Address
            }
        };
    }
}
