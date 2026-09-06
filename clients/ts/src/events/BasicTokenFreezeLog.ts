// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          BasicTokenFreezeLog.ts
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
export class BasicTokenFreezeLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([215, 13, 35, 175, 105, 156, 78, 202]);
    readonly discriminator = BasicTokenFreezeLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    riskClass!: U8;
    basicFreezeRecord!: PublicKeyLike;
    issuedTokenAccount!: PublicKeyLike;
    issuedTokenMint!: PublicKeyLike;
    frozenBy!: PublicKeyLike;
    reasonCode!: U32;
    statusFlags!: BU64;
    freezeFlags!: BU64;

    constructor(props: DecodableProps<BasicTokenFreezeLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): BasicTokenFreezeLog {
        return Decoder.decode(data, BasicTokenFreezeLog);
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
            frozenBy: {
                type: SchemaFieldType.Address
            },
            reasonCode: {
                type: SchemaFieldType.U32
            },
            statusFlags: {
                type: SchemaFieldType.U64
            },
            freezeFlags: {
                type: SchemaFieldType.U64
            }
        };
    }
}
