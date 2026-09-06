// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          ModuleActivationStateInitializedLog.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/emit/bootstrap.rs
 */
export class ModuleActivationStateInitializedLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([241, 55, 169, 207, 125, 204, 148, 77]);
    readonly discriminator = ModuleActivationStateInitializedLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    initializedBy!: PublicKeyLike;
    defaultStatuses!: U8[];

    constructor(props: DecodableProps<ModuleActivationStateInitializedLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): ModuleActivationStateInitializedLog {
        return Decoder.decode(data, ModuleActivationStateInitializedLog);
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
            initializedBy: {
                type: SchemaFieldType.Address
            },
            defaultStatuses: {
                type: SchemaFieldType.Vector,
                codableType: SchemaFieldType.U8
            }
        };
    }
}
