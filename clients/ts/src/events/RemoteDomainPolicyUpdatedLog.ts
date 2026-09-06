// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          RemoteDomainPolicyUpdatedLog.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/emit/cross_chain.rs
 */
export class RemoteDomainPolicyUpdatedLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([70, 33, 0, 213, 50, 191, 83, 101]);
    readonly discriminator = RemoteDomainPolicyUpdatedLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    riskClass!: U8;
    changeId!: U8[];
    remoteDomainPolicy!: PublicKeyLike;
    remoteChainKind!: U8;
    remoteDomainId!: BU64;
    signerSetId!: U8[];
    oldValueHash!: U8[];
    newValueHash!: U8[];
    changeMask!: BU64;
    proposedBy!: PublicKeyLike;
    updatedBy!: PublicKeyLike;

    constructor(props: DecodableProps<RemoteDomainPolicyUpdatedLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): RemoteDomainPolicyUpdatedLog {
        return Decoder.decode(data, RemoteDomainPolicyUpdatedLog);
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
            remoteChainKind: {
                type: SchemaFieldType.U8
            },
            remoteDomainId: {
                type: SchemaFieldType.U64
            },
            signerSetId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
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
            changeMask: {
                type: SchemaFieldType.U64
            },
            proposedBy: {
                type: SchemaFieldType.Address
            },
            updatedBy: {
                type: SchemaFieldType.Address
            }
        };
    }
}
