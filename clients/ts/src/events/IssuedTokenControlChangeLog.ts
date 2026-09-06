// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          IssuedTokenControlChangeLog.ts
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
import type { Base64, BI64, BU64, PublicKeyLike } from "@solomon-labs/types";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/emit/issued_token.rs
 */
export class IssuedTokenControlChangeLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([178, 169, 214, 97, 201, 220, 54, 13]);
    readonly discriminator = IssuedTokenControlChangeLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    issuedTokenExtensionMask!: BU64[];
    collateralExtensionMask!: BU64[];
    issuedTokenModuleMask!: BU64[];
    collateralModuleMask!: BU64[];
    hookProgramId!: PublicKeyLike;
    permanentDelegate!: PublicKeyLike;
    metadataAddress!: PublicKeyLike;
    tokenControlFlags!: BU64;
    issuedTokenMint!: PublicKeyLike;
    configuredBy!: PublicKeyLike;
    changeKind!: BU64;

    constructor(props: DecodableProps<IssuedTokenControlChangeLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): IssuedTokenControlChangeLog {
        return Decoder.decode(data, IssuedTokenControlChangeLog);
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
            issuedTokenExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            collateralExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            issuedTokenModuleMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            collateralModuleMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            hookProgramId: {
                type: SchemaFieldType.Address
            },
            permanentDelegate: {
                type: SchemaFieldType.Address
            },
            metadataAddress: {
                type: SchemaFieldType.Address
            },
            tokenControlFlags: {
                type: SchemaFieldType.U64
            },
            issuedTokenMint: {
                type: SchemaFieldType.Address
            },
            configuredBy: {
                type: SchemaFieldType.Address
            },
            changeKind: {
                type: SchemaFieldType.U64
            }
        };
    }
}
