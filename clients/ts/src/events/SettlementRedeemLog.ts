// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      event
// File:          SettlementRedeemLog.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/emit/settlement.rs
 */
export class SettlementRedeemLog implements IsLogDecodable {
    static readonly discriminator = new Uint8Array([146, 88, 3, 220, 199, 30, 109, 16]);
    readonly discriminator = SettlementRedeemLog.discriminator;

    sequenceNonce!: BU64;
    chancery!: PublicKeyLike;
    slot!: BU64;
    unixTimestamp!: BI64;
    pathwayId!: U8[];
    settlementMode!: U8;
    intentId!: U8[];
    settlementPolicyId!: U8[];
    limitPolicyId!: U8[];
    evidencePolicyId!: U8[];
    feePolicyId!: U8[];
    insurancePolicyId!: U8[];
    principalA!: PublicKeyLike;
    principalB!: PublicKeyLike;
    executor!: PublicKeyLike;
    assetMint!: PublicKeyLike;
    issuedTokenMint!: PublicKeyLike;
    assetTokenProgram!: PublicKeyLike;
    issuedTokenProgram!: PublicKeyLike;
    sourceAccount!: PublicKeyLike;
    destinationAccount!: PublicKeyLike;
    feeRecipientOwner!: PublicKeyLike;
    feeRecipientTokenAccount!: PublicKeyLike;
    reserveCompartmentId!: U8[];
    grossAmountIn!: BU64;
    grossAmountOut!: BU64;
    feeOutputAmount!: BU64;
    rebateAmount!: BU64;
    netOutputAmount!: BU64;
    statusFlags!: BU64;
    reasonCode!: U32;
    actualFeeRecipientAmount!: BU64;

    constructor(props: DecodableProps<SettlementRedeemLog>) {
        Object.assign(this, props);
    }

    static decode(data: string | Base64 | Uint8Array | Buffer | null): SettlementRedeemLog {
        return Decoder.decode(data, SettlementRedeemLog);
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
            pathwayId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            settlementMode: {
                type: SchemaFieldType.U8
            },
            intentId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            settlementPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            limitPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            evidencePolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            feePolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            insurancePolicyId: {
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
            assetTokenProgram: {
                type: SchemaFieldType.Address
            },
            issuedTokenProgram: {
                type: SchemaFieldType.Address
            },
            sourceAccount: {
                type: SchemaFieldType.Address
            },
            destinationAccount: {
                type: SchemaFieldType.Address
            },
            feeRecipientOwner: {
                type: SchemaFieldType.Address
            },
            feeRecipientTokenAccount: {
                type: SchemaFieldType.Address
            },
            reserveCompartmentId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            grossAmountIn: {
                type: SchemaFieldType.U64
            },
            grossAmountOut: {
                type: SchemaFieldType.U64
            },
            feeOutputAmount: {
                type: SchemaFieldType.U64
            },
            rebateAmount: {
                type: SchemaFieldType.U64
            },
            netOutputAmount: {
                type: SchemaFieldType.U64
            },
            statusFlags: {
                type: SchemaFieldType.U64
            },
            reasonCode: {
                type: SchemaFieldType.U32
            },
            actualFeeRecipientAmount: {
                type: SchemaFieldType.U64
            }
        };
    }
}
