// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          ReserveWithdrawalLog.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/emit/reserve.rs
 */
export class ReserveWithdrawalLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([33, 234, 153, 99, 56, 19, 63, 214]);
    readonly discriminator = ReserveWithdrawalLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    assetMint!: PublicKeyLike;
    destinationTokenAccount!: PublicKeyLike;
    amount!: BU64;
    initiatedBy!: PublicKeyLike;
    actualDestinationAmount!: BU64;

    constructor(props: DecodableProps<ReserveWithdrawalLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): ReserveWithdrawalLog {
        return Decoder.decode(data, ReserveWithdrawalLog);
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
            assetMint: {
                type: SchemaFieldType.Address
            },
            destinationTokenAccount: {
                type: SchemaFieldType.Address
            },
            amount: {
                type: SchemaFieldType.U64
            },
            initiatedBy: {
                type: SchemaFieldType.Address
            },
            actualDestinationAmount: {
                type: SchemaFieldType.U64
            }
        };
    }
}
