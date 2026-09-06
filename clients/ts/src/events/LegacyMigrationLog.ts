// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          LegacyMigrationLog.ts
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
import type { Base64, BI64, BU64, PublicKeyLike } from "@solomon-labs/types";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/emit/migration.rs
 */
export class LegacyMigrationLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([135, 170, 198, 246, 85, 34, 226, 138]);
    readonly discriminator = LegacyMigrationLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    legacyMint!: PublicKeyLike;
    issuedTokenMint!: PublicKeyLike;
    burnedLegacyAmount!: BU64;
    mintedIssuedAmount!: BU64;
    migratedBy!: PublicKeyLike;

    constructor(props: DecodableProps<LegacyMigrationLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): LegacyMigrationLog {
        return Decoder.decode(data, LegacyMigrationLog);
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
            legacyMint: {
                type: SchemaFieldType.Address
            },
            issuedTokenMint: {
                type: SchemaFieldType.Address
            },
            burnedLegacyAmount: {
                type: SchemaFieldType.U64
            },
            mintedIssuedAmount: {
                type: SchemaFieldType.U64
            },
            migratedBy: {
                type: SchemaFieldType.Address
            }
        };
    }
}
