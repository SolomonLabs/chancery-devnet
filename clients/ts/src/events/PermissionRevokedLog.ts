// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          PermissionRevokedLog.ts
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
export class PermissionRevokedLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([190, 222, 94, 140, 253, 138, 31, 172]);
    readonly discriminator = PermissionRevokedLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    riskClass!: U8;
    permissionRecord!: PublicKeyLike;
    subject!: PublicKeyLike;
    scopeKind!: U8;
    scopeKey!: PublicKeyLike;
    revokedBy!: PublicKeyLike;

    constructor(props: DecodableProps<PermissionRevokedLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): PermissionRevokedLog {
        return Decoder.decode(data, PermissionRevokedLog);
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
            revokedBy: {
                type: SchemaFieldType.Address
            }
        };
    }
}
