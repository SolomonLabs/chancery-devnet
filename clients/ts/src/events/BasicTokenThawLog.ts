// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          BasicTokenThawLog.ts
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
import type { Base64, BI64, BU64, PublicKeyLike, U32, U8 } from "@solomon-labs/types";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/emit/issued_token.rs
 */
export class BasicTokenThawLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([104, 71, 193, 140, 108, 247, 30, 125]);
    readonly discriminator = BasicTokenThawLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    riskClass!: U8;
    basicFreezeRecord!: PublicKeyLike;
    issuedTokenAccount!: PublicKeyLike;
    issuedTokenMint!: PublicKeyLike;
    thawedBy!: PublicKeyLike;
    reasonCode!: U32;
    statusFlags!: BU64;
    thawFlags!: BU64;

    constructor(props: DecodableProps<BasicTokenThawLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): BasicTokenThawLog {
        return Decoder.decode(data, BasicTokenThawLog);
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
            basicFreezeRecord: {
                type: SchemaFieldType.Address
            },
            issuedTokenAccount: {
                type: SchemaFieldType.Address
            },
            issuedTokenMint: {
                type: SchemaFieldType.Address
            },
            thawedBy: {
                type: SchemaFieldType.Address
            },
            reasonCode: {
                type: SchemaFieldType.U32
            },
            statusFlags: {
                type: SchemaFieldType.U64
            },
            thawFlags: {
                type: SchemaFieldType.U64
            }
        };
    }
}
