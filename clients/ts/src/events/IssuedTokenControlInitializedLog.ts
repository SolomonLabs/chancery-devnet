// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          IssuedTokenControlInitializedLog.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/emit/issued_token.rs
 */
export class IssuedTokenControlInitializedLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([149, 181, 131, 155, 94, 12, 52, 140]);
    readonly discriminator = IssuedTokenControlInitializedLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    riskClass!: U8;
    issuedTokenControl!: PublicKeyLike;
    reservedMintExtensionMask!: BU64[];
    reservedAccountExtensionMask!: BU64[];
    maxExtensionObservationAgeSlots!: BU64;
    initializedBy!: PublicKeyLike;

    constructor(props: DecodableProps<IssuedTokenControlInitializedLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): IssuedTokenControlInitializedLog {
        return Decoder.decode(data, IssuedTokenControlInitializedLog);
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
            reservedMintExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            reservedAccountExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            maxExtensionObservationAgeSlots: {
                type: SchemaFieldType.U64
            },
            initializedBy: {
                type: SchemaFieldType.Address
            }
        };
    }
}
