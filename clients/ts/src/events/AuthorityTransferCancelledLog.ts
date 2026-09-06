// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          AuthorityTransferCancelledLog.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/emit/authority.rs
 */
export class AuthorityTransferCancelledLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([31, 228, 187, 148, 20, 99, 237, 48]);
    readonly discriminator = AuthorityTransferCancelledLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    roleKind!: U8;
    cancelledAuthority!: PublicKeyLike;
    proposedAtSlot!: BU64;

    constructor(props: DecodableProps<AuthorityTransferCancelledLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): AuthorityTransferCancelledLog {
        return Decoder.decode(data, AuthorityTransferCancelledLog);
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
            roleKind: {
                type: SchemaFieldType.U8
            },
            cancelledAuthority: {
                type: SchemaFieldType.Address
            },
            proposedAtSlot: {
                type: SchemaFieldType.U64
            }
        };
    }
}
