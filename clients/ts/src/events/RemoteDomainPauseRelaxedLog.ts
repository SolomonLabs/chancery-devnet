// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          RemoteDomainPauseRelaxedLog.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/emit/cross_chain.rs
 */
export class RemoteDomainPauseRelaxedLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([88, 234, 116, 210, 192, 12, 117, 122]);
    readonly discriminator = RemoteDomainPauseRelaxedLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    riskClass!: U8;
    changeId!: U8[];
    remoteDomainPolicy!: PublicKeyLike;
    oldPauseBits!: BU64;
    newPauseBits!: BU64;
    reasonCode!: U32;
    oldValueHash!: U8[];
    newValueHash!: U8[];
    proposedBy!: PublicKeyLike;
    relaxedBy!: PublicKeyLike;

    constructor(props: DecodableProps<RemoteDomainPauseRelaxedLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): RemoteDomainPauseRelaxedLog {
        return Decoder.decode(data, RemoteDomainPauseRelaxedLog);
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
            changeId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            remoteDomainPolicy: {
                type: SchemaFieldType.Address
            },
            oldPauseBits: {
                type: SchemaFieldType.U64
            },
            newPauseBits: {
                type: SchemaFieldType.U64
            },
            reasonCode: {
                type: SchemaFieldType.U32
            },
            oldValueHash: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            newValueHash: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            proposedBy: {
                type: SchemaFieldType.Address
            },
            relaxedBy: {
                type: SchemaFieldType.Address
            }
        };
    }
}
