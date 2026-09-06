// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          PauseStateChangeLog.ts
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
import type { Base64, BI64, BU64, PublicKeyLike, U32, U8 } from "@solomon-labs/types";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/emit/pause.rs
 */
export class PauseStateChangeLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([163, 114, 45, 90, 110, 217, 74, 240]);
    readonly discriminator = PauseStateChangeLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    previousPauseBits!: BU64;
    changedPauseBits!: BU64;
    effectivePauseBits!: BU64;
    isClear!: boolean;
    expiresAtSlot!: BU64;
    reasonCode!: U32;
    actingAuthority!: PublicKeyLike;
    activatedBy!: PublicKeyLike;
    scopeKind!: U8;
    scopeKey!: PublicKeyLike;
    subject!: PublicKeyLike;

    constructor(props: DecodableProps<PauseStateChangeLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): PauseStateChangeLog {
        return Decoder.decode(data, PauseStateChangeLog);
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
            previousPauseBits: {
                type: SchemaFieldType.U64
            },
            changedPauseBits: {
                type: SchemaFieldType.U64
            },
            effectivePauseBits: {
                type: SchemaFieldType.U64
            },
            isClear: {
                type: SchemaFieldType.Boolean
            },
            expiresAtSlot: {
                type: SchemaFieldType.U64
            },
            reasonCode: {
                type: SchemaFieldType.U32
            },
            actingAuthority: {
                type: SchemaFieldType.Address
            },
            activatedBy: {
                type: SchemaFieldType.Address
            },
            scopeKind: {
                type: SchemaFieldType.U8
            },
            scopeKey: {
                type: SchemaFieldType.Address
            },
            subject: {
                type: SchemaFieldType.Address
            }
        };
    }
}
