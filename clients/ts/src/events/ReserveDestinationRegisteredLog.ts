// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          ReserveDestinationRegisteredLog.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/emit/reserve.rs
 */
export class ReserveDestinationRegisteredLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([81, 109, 1, 83, 154, 44, 154, 101]);
    readonly discriminator = ReserveDestinationRegisteredLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    riskClass!: U8;
    changeId!: U8[];
    reserveDestination!: PublicKeyLike;
    destinationToken!: PublicKeyLike;
    purposeFlag!: BU64;
    registeredBy!: PublicKeyLike;
    withdrawalLimitPolicyId!: U8[];

    constructor(props: DecodableProps<ReserveDestinationRegisteredLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): ReserveDestinationRegisteredLog {
        return Decoder.decode(data, ReserveDestinationRegisteredLog);
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
            reserveDestination: {
                type: SchemaFieldType.Address
            },
            destinationToken: {
                type: SchemaFieldType.Address
            },
            purposeFlag: {
                type: SchemaFieldType.U64
            },
            registeredBy: {
                type: SchemaFieldType.Address
            },
            withdrawalLimitPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            }
        };
    }
}
