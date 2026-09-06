// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          AssetRegisteredLog.ts
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
export class AssetRegisteredLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([252, 56, 47, 89, 8, 180, 249, 254]);
    readonly discriminator = AssetRegisteredLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    riskClass!: U8;
    assetConfig!: PublicKeyLike;
    assetMint!: PublicKeyLike;
    assetTokenProgram!: PublicKeyLike;
    mode!: U8;
    isIssuedTokenMint!: boolean;
    registeredBy!: PublicKeyLike;

    constructor(props: DecodableProps<AssetRegisteredLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): AssetRegisteredLog {
        return Decoder.decode(data, AssetRegisteredLog);
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
            assetTokenProgram: {
                type: SchemaFieldType.Address
            },
            mode: {
                type: SchemaFieldType.U8
            },
            isIssuedTokenMint: {
                type: SchemaFieldType.Boolean
            },
            registeredBy: {
                type: SchemaFieldType.Address
            }
        };
    }
}
