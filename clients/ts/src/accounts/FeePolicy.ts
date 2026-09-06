// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          FeePolicy.ts
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
import type {
    BI64,
    BU64,
    PublicKeyLike,
    SolanaAddressLike,
    U16,
    U32,
    U8,
} from "@solomon-labs/types";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/fees/state/fee_policy.rs
 */
export class FeePolicy implements IsDecodable {
    static readonly discriminator = new Uint8Array([102, 101, 101, 112, 111, 108, 121, 0]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    feeRecipientPolicy!: U8;
    roundingMode!: U8;
    netFeeFloorZero!: U8;
    pad0!: U8[];
    feePolicyId!: U8[];
    feePolicyFlags!: BU64;
    flatFeeInAsset!: BU64;
    flatFeeInIssuedToken!: BU64;
    percentFeeBps!: U32;
    pad1!: U8[];
    feeCapAmount!: BU64;
    minimumFeeAmount!: BU64;
    rebateFlatAmount!: BU64;
    rebateBps!: U32;
    pad2!: U8[];
    rebateCapAmount!: BU64;
    feeRecipientKey!: PublicKeyLike;
    effectiveFromUnixTimestamp!: BI64;
    effectiveUntilUnixTimestamp!: BI64;
    reserved!: U8[];

    constructor(props: DecodableProps<FeePolicy>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): FeePolicy {
        return Decoder.decode(data, FeePolicy, address);
    }

    static async get(address: SolanaAddressLike): Promise<FeePolicy> {
        return Decoder.getAccount(address, FeePolicy);
    }

    static getSchema(): Schema {
        return {
            version: {
                type: SchemaFieldType.U16
            },
            bump: {
                type: SchemaFieldType.U8
            },
            feeRecipientPolicy: {
                type: SchemaFieldType.U8
            },
            roundingMode: {
                type: SchemaFieldType.U8
            },
            netFeeFloorZero: {
                type: SchemaFieldType.U8
            },
            pad0: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U8
            },
            feePolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            feePolicyFlags: {
                type: SchemaFieldType.U64
            },
            flatFeeInAsset: {
                type: SchemaFieldType.U64
            },
            flatFeeInIssuedToken: {
                type: SchemaFieldType.U64
            },
            percentFeeBps: {
                type: SchemaFieldType.U32
            },
            pad1: {
                type: SchemaFieldType.Array,
                size: 4,
                codableType: SchemaFieldType.U8
            },
            feeCapAmount: {
                type: SchemaFieldType.U64
            },
            minimumFeeAmount: {
                type: SchemaFieldType.U64
            },
            rebateFlatAmount: {
                type: SchemaFieldType.U64
            },
            rebateBps: {
                type: SchemaFieldType.U32
            },
            pad2: {
                type: SchemaFieldType.Array,
                size: 4,
                codableType: SchemaFieldType.U8
            },
            rebateCapAmount: {
                type: SchemaFieldType.U64
            },
            feeRecipientKey: {
                type: SchemaFieldType.Address
            },
            effectiveFromUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            effectiveUntilUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            reserved: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            }
        };
    }
}
