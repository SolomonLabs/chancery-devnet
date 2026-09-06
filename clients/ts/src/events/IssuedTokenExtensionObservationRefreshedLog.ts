// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          IssuedTokenExtensionObservationRefreshedLog.ts
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
export class IssuedTokenExtensionObservationRefreshedLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([237, 242, 47, 38, 81, 207, 244, 147]);
    readonly discriminator = IssuedTokenExtensionObservationRefreshedLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    riskClass!: U8;
    issuedTokenControl!: PublicKeyLike;
    observedExtensionMask!: BU64[];
    controlFlags!: BU64;
    verificationSucceeded!: boolean;
    regressionErrorCode!: U32;
    refreshedBy!: PublicKeyLike;

    constructor(props: DecodableProps<IssuedTokenExtensionObservationRefreshedLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): IssuedTokenExtensionObservationRefreshedLog {
        return Decoder.decode(data, IssuedTokenExtensionObservationRefreshedLog);
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
            issuedTokenControl: {
                type: SchemaFieldType.Address
            },
            observedExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            controlFlags: {
                type: SchemaFieldType.U64
            },
            verificationSucceeded: {
                type: SchemaFieldType.Boolean
            },
            regressionErrorCode: {
                type: SchemaFieldType.U32
            },
            refreshedBy: {
                type: SchemaFieldType.Address
            }
        };
    }
}
