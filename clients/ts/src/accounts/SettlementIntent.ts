// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          SettlementIntent.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/settlement/state/settlement_intent.rs
 */
export class SettlementIntent implements IsDecodable {
    static readonly discriminator = new Uint8Array([115, 116, 108, 105, 110, 116, 0, 0]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    status!: U8;
    settlementMode!: U8;
    settlementAction!: U8;
    pad0!: U8[];
    intentId!: U8[];
    principalA!: PublicKeyLike;
    principalB!: PublicKeyLike;
    executor!: PublicKeyLike;
    assetMint!: PublicKeyLike;
    issuedTokenMint!: PublicKeyLike;
    assetAmount!: BU64;
    issuedTokenAmount!: BU64;
    minimumAssetAmount!: BU64;
    minimumIssuedTokenAmount!: BU64;
    nonce!: BU64;
    validAfterUnixTimestamp!: BI64;
    expiresAtUnixTimestamp!: BI64;
    policyId!: U8[];
    intentHash!: U8[];
    pathwayId!: U8[];
    rentRefundRecipient!: PublicKeyLike;
    reserved!: U8[];

    constructor(props: DecodableProps<SettlementIntent>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): SettlementIntent {
        return Decoder.decode(data, SettlementIntent, address);
    }

    static async get(address: SolanaAddressLike): Promise<SettlementIntent> {
        return Decoder.getAccount(address, SettlementIntent);
    }

    static getSchema(): Schema {
        return {
            version: {
                type: SchemaFieldType.U16
            },
            bump: {
                type: SchemaFieldType.U8
            },
            status: {
                type: SchemaFieldType.U8
            },
            settlementMode: {
                type: SchemaFieldType.U8
            },
            settlementAction: {
                type: SchemaFieldType.U8
            },
            pad0: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U8
            },
            intentId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            principalA: {
                type: SchemaFieldType.Address
            },
            principalB: {
                type: SchemaFieldType.Address
            },
            executor: {
                type: SchemaFieldType.Address
            },
            assetMint: {
                type: SchemaFieldType.Address
            },
            issuedTokenMint: {
                type: SchemaFieldType.Address
            },
            assetAmount: {
                type: SchemaFieldType.U64
            },
            issuedTokenAmount: {
                type: SchemaFieldType.U64
            },
            minimumAssetAmount: {
                type: SchemaFieldType.U64
            },
            minimumIssuedTokenAmount: {
                type: SchemaFieldType.U64
            },
            nonce: {
                type: SchemaFieldType.U64
            },
            validAfterUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            expiresAtUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            policyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            intentHash: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            pathwayId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            rentRefundRecipient: {
                type: SchemaFieldType.Address
            },
            reserved: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            }
        };
    }
}
