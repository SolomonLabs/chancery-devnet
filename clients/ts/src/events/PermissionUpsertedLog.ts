// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          PermissionUpsertedLog.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/emit/permission.rs
 */
export class PermissionUpsertedLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([176, 212, 110, 75, 186, 255, 121, 82]);
    readonly discriminator = PermissionUpsertedLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    riskClass!: U8;
    permissionRecord!: PublicKeyLike;
    subject!: PublicKeyLike;
    scopeKind!: U8;
    scopeKey!: PublicKeyLike;
    roleBits!: BU64[];
    permissionFlags!: BU64;
    expiryUnixTimestamp!: BI64;
    grantedBy!: PublicKeyLike;
    modifiedBy!: PublicKeyLike;
    changeId!: U8[];
    oldValueHash!: U8[];
    newValueHash!: U8[];
    changeMask!: BU64;
    approvedBy!: PublicKeyLike;

    constructor(props: DecodableProps<PermissionUpsertedLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): PermissionUpsertedLog {
        return Decoder.decode(data, PermissionUpsertedLog);
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
            permissionRecord: {
                type: SchemaFieldType.Address
            },
            subject: {
                type: SchemaFieldType.Address
            },
            scopeKind: {
                type: SchemaFieldType.U8
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
            expiryUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            grantedBy: {
                type: SchemaFieldType.Address
            },
            modifiedBy: {
                type: SchemaFieldType.Address
            },
            changeId: {
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
            approvedBy: {
                type: SchemaFieldType.Address
            }
        };
    }
}
