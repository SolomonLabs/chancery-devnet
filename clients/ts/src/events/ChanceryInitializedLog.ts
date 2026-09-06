// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          ChanceryInitializedLog.ts
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
export class ChanceryInitializedLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([69, 139, 68, 223, 251, 54, 166, 230]);
    readonly discriminator = ChanceryInitializedLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    governanceAuthority!: PublicKeyLike;
    operationsAuthority!: PublicKeyLike;
    emergencyAuthority!: PublicKeyLike;
    enforcementAuthority!: PublicKeyLike;
    insuranceAdminAuthority!: PublicKeyLike;
    issuedTokenMint!: PublicKeyLike;
    issuedTokenProgram!: PublicKeyLike;
    legacyTokenMint!: PublicKeyLike;
    legacyTokenProgram!: PublicKeyLike;
    mintAuthorityPda!: PublicKeyLike;
    freezeAuthorityPda!: PublicKeyLike;
    domainSeparator!: U8[];

    constructor(props: DecodableProps<ChanceryInitializedLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): ChanceryInitializedLog {
        return Decoder.decode(data, ChanceryInitializedLog);
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
            governanceAuthority: {
                type: SchemaFieldType.Address
            },
            operationsAuthority: {
                type: SchemaFieldType.Address
            },
            emergencyAuthority: {
                type: SchemaFieldType.Address
            },
            enforcementAuthority: {
                type: SchemaFieldType.Address
            },
            insuranceAdminAuthority: {
                type: SchemaFieldType.Address
            },
            issuedTokenMint: {
                type: SchemaFieldType.Address
            },
            issuedTokenProgram: {
                type: SchemaFieldType.Address
            },
            legacyTokenMint: {
                type: SchemaFieldType.Address
            },
            legacyTokenProgram: {
                type: SchemaFieldType.Address
            },
            mintAuthorityPda: {
                type: SchemaFieldType.Address
            },
            freezeAuthorityPda: {
                type: SchemaFieldType.Address
            },
            domainSeparator: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            }
        };
    }
}
