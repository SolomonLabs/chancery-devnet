// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          LegacyMigrationConfig.ts
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
import type { DecodableProps, IsDecodable, Schema } from "@solomon-labs/solana-codec";
import { Decoder, SchemaFieldType } from "@solomon-labs/solana-codec";
import type { BU64, PublicKeyLike, SolanaAddressLike, U16, U8 } from "@solomon-labs/types";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/migration/state/legacy_migration_config.rs
 */
export class LegacyMigrationConfig implements IsDecodable {
    static readonly discriminator = new Uint8Array([108, 103, 99, 121, 109, 105, 103, 0]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    pad0!: U8[];
    migrationFlags!: BU64;
    legacyProgram!: PublicKeyLike;
    legacyMint!: PublicKeyLike;
    migrationEnabledAtSlot!: BU64;
    migratedTotal!: BU64;
    legacySupplySnapshot!: BU64;
    migratedLegacyTotal!: BU64;
    reserved!: U8[];

    constructor(props: DecodableProps<LegacyMigrationConfig>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): LegacyMigrationConfig {
        return Decoder.decode(data, LegacyMigrationConfig, address);
    }

    static async get(address: SolanaAddressLike): Promise<LegacyMigrationConfig> {
        return Decoder.getAccount(address, LegacyMigrationConfig);
    }

    static getSchema(): Schema {
        return {
            version: {
                type: SchemaFieldType.U16
            },
            bump: {
                type: SchemaFieldType.U8
            },
            pad0: {
                type: SchemaFieldType.Array,
                size: 5,
                codableType: SchemaFieldType.U8
            },
            migrationFlags: {
                type: SchemaFieldType.U64
            },
            legacyProgram: {
                type: SchemaFieldType.Address
            },
            legacyMint: {
                type: SchemaFieldType.Address
            },
            migrationEnabledAtSlot: {
                type: SchemaFieldType.U64
            },
            migratedTotal: {
                type: SchemaFieldType.U64
            },
            legacySupplySnapshot: {
                type: SchemaFieldType.U64
            },
            migratedLegacyTotal: {
                type: SchemaFieldType.U64
            },
            reserved: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            }
        };
    }
}
