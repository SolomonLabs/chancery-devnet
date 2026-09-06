// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          ConfigChangeProposedLog.ts
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
import type { Base64, BI64, BU64, PublicKeyLike, U16, U8 } from "@solomon-labs/types";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/emit/config_change.rs
 */
export class ConfigChangeProposedLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([57, 212, 96, 3, 9, 29, 225, 198]);
    readonly discriminator = ConfigChangeProposedLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    riskClass!: U8;
    changeId!: U8[];
    changeKind!: U16;
    targetAccount!: PublicKeyLike;
    oldValueHash!: U8[];
    newValueHash!: U8[];
    proposedBy!: PublicKeyLike;
    proposerNonce!: BU64;
    executableAfterUnixTimestamp!: BI64;
    expiresAtUnixTimestamp!: BI64;

    constructor(props: DecodableProps<ConfigChangeProposedLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): ConfigChangeProposedLog {
        return Decoder.decode(data, ConfigChangeProposedLog);
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
            changeKind: {
                type: SchemaFieldType.U16
            },
            targetAccount: {
                type: SchemaFieldType.Address
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
            proposedBy: {
                type: SchemaFieldType.Address
            },
            proposerNonce: {
                type: SchemaFieldType.U64
            },
            executableAfterUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            expiresAtUnixTimestamp: {
                type: SchemaFieldType.I64
            }
        };
    }
}
