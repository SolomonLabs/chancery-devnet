// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          CrossChainSignerSetRegisteredLog.ts
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
export class CrossChainSignerSetRegisteredLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([126, 32, 84, 198, 76, 100, 232, 78]);
    readonly discriminator = CrossChainSignerSetRegisteredLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    riskClass!: U8;
    changeId!: U8[];
    crossChainSignerSet!: PublicKeyLike;
    signerSetId!: U8[];
    signerRoot!: U8[];
    signerCount!: U8;
    threshold!: U8;
    validAfterUnixTimestamp!: BI64;
    expiresAtUnixTimestamp!: BI64;
    oldValueHash!: U8[];
    newValueHash!: U8[];
    proposedBy!: PublicKeyLike;
    registeredBy!: PublicKeyLike;

    constructor(props: DecodableProps<CrossChainSignerSetRegisteredLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): CrossChainSignerSetRegisteredLog {
        return Decoder.decode(data, CrossChainSignerSetRegisteredLog);
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
            crossChainSignerSet: {
                type: SchemaFieldType.Address
            },
            signerSetId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            signerRoot: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            signerCount: {
                type: SchemaFieldType.U8
            },
            threshold: {
                type: SchemaFieldType.U8
            },
            validAfterUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            expiresAtUnixTimestamp: {
                type: SchemaFieldType.I64
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
            registeredBy: {
                type: SchemaFieldType.Address
            }
        };
    }
}
