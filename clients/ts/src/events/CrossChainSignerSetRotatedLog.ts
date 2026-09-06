// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          CrossChainSignerSetRotatedLog.ts
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
import type { Base64, BI64, BU64, PublicKeyLike, U16, U8 } from "@solomon-labs/types";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/emit/cross_chain.rs
 */
export class CrossChainSignerSetRotatedLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([32, 51, 94, 2, 196, 96, 232, 215]);
    readonly discriminator = CrossChainSignerSetRotatedLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    crossChainSignerSet!: PublicKeyLike;
    signerSetId!: U8[];
    reasonCode!: U16;
    rotatedBy!: PublicKeyLike;

    constructor(props: DecodableProps<CrossChainSignerSetRotatedLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): CrossChainSignerSetRotatedLog {
        return Decoder.decode(data, CrossChainSignerSetRotatedLog);
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
            crossChainSignerSet: {
                type: SchemaFieldType.Address
            },
            signerSetId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            reasonCode: {
                type: SchemaFieldType.U16
            },
            rotatedBy: {
                type: SchemaFieldType.Address
            }
        };
    }
}
