// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          LegacyMigrationEnabledLog.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/emit/migration.rs
 */
export class LegacyMigrationEnabledLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([204, 168, 3, 221, 151, 1, 184, 220]);
    readonly discriminator = LegacyMigrationEnabledLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    riskClass!: U8;
    migrationConfig!: PublicKeyLike;
    legacyProgram!: PublicKeyLike;
    legacyMint!: PublicKeyLike;
    legacySupplySnapshot!: BU64;
    enabledBy!: PublicKeyLike;

    constructor(props: DecodableProps<LegacyMigrationEnabledLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): LegacyMigrationEnabledLog {
        return Decoder.decode(data, LegacyMigrationEnabledLog);
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
            migrationConfig: {
                type: SchemaFieldType.Address
            },
            legacyProgram: {
                type: SchemaFieldType.Address
            },
            legacyMint: {
                type: SchemaFieldType.Address
            },
            legacySupplySnapshot: {
                type: SchemaFieldType.U64
            },
            enabledBy: {
                type: SchemaFieldType.Address
            }
        };
    }
}
