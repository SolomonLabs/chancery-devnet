// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          LimitPolicy.ts
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
import type { BU64, PublicKeyLike, SolanaAddressLike, U16, U32, U8 } from "@solomon-labs/types";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/limits/state/limit_policy.rs
 */
export class LimitPolicy implements IsDecodable {
    static readonly discriminator = new Uint8Array([108, 109, 116, 112, 111, 108, 121, 0]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    scopeKind!: U8;
    pad0!: U8[];
    limitPolicyId!: U8[];
    scopeKey!: PublicKeyLike;
    perTransactionMaximum!: BU64;
    perHourMaximum!: BU64;
    perDayMaximum!: BU64;
    perSevenDayMaximum!: BU64;
    perThirtyDayMaximum!: BU64;
    maximumActionsPerHour!: U32;
    maximumActionsPerDay!: U32;
    reservedBreachFlags!: BU64[];
    statusFlags!: BU64;
    reserved!: U8[];

    constructor(props: DecodableProps<LimitPolicy>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): LimitPolicy {
        return Decoder.decode(data, LimitPolicy, address);
    }

    static async get(address: SolanaAddressLike): Promise<LimitPolicy> {
        return Decoder.getAccount(address, LimitPolicy);
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
            limitPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            scopeKey: {
                type: SchemaFieldType.Address
            },
            perTransactionMaximum: {
                type: SchemaFieldType.U64
            },
            perHourMaximum: {
                type: SchemaFieldType.U64
            },
            perDayMaximum: {
                type: SchemaFieldType.U64
            },
            perSevenDayMaximum: {
                type: SchemaFieldType.U64
            },
            perThirtyDayMaximum: {
                type: SchemaFieldType.U64
            },
            maximumActionsPerHour: {
                type: SchemaFieldType.U32
            },
            maximumActionsPerDay: {
                type: SchemaFieldType.U32
            },
            reservedBreachFlags: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            statusFlags: {
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
