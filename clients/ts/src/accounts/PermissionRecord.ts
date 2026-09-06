// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          PermissionRecord.ts
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
import type { BI64, BU64, PublicKeyLike, SolanaAddressLike, U16, U8 } from "@solomon-labs/types";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/permissions/state/permission_record.rs
 */
export class PermissionRecord implements IsDecodable {
    static readonly discriminator = new Uint8Array([112, 101, 114, 109, 114, 101, 99, 0]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    scopeKind!: U8;
    pad0!: U8[];
    subject!: PublicKeyLike;
    scopeKey!: PublicKeyLike;
    roleBits!: BU64[];
    permissionFlags!: BU64;
    issuedAtUnixTimestamp!: BI64;
    expiryUnixTimestamp!: BI64;
    grantedBy!: PublicKeyLike;
    roleSchemaVersion!: U16;
    pad1!: U8[];
    permissionGeneration!: BU64;
    reserved!: U8[];

    constructor(props: DecodableProps<PermissionRecord>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): PermissionRecord {
        return Decoder.decode(data, PermissionRecord, address);
    }

    static async get(address: SolanaAddressLike): Promise<PermissionRecord> {
        return Decoder.getAccount(address, PermissionRecord);
    }

    static getSchema(): Schema {
        return {
            version: {
                type: SchemaFieldType.U16
            },
            bump: {
                type: SchemaFieldType.U8
            },
            scopeKind: {
                type: SchemaFieldType.U8
            },
            pad0: {
                type: SchemaFieldType.Array,
                size: 4,
                codableType: SchemaFieldType.U8
            },
            subject: {
                type: SchemaFieldType.Address
            },
            scopeKey: {
                type: SchemaFieldType.Address
            },
            roleBits: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            permissionFlags: {
                type: SchemaFieldType.U64
            },
            issuedAtUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            expiryUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            grantedBy: {
                type: SchemaFieldType.Address
            },
            roleSchemaVersion: {
                type: SchemaFieldType.U16
            },
            pad1: {
                type: SchemaFieldType.Array,
                size: 6,
                codableType: SchemaFieldType.U8
            },
            permissionGeneration: {
                type: SchemaFieldType.U64
            },
            reserved: {
                type: SchemaFieldType.Array,
                size: 24,
                codableType: SchemaFieldType.U8
            }
        };
    }
}
