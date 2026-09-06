// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          PathwayPolicy.ts
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
import type { BU64, PublicKeyLike, SolanaAddressLike, U16, U8 } from "@solomon-labs/types";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/pathway/state/pathway_policy.rs
 */
export class PathwayPolicy implements IsDecodable {
    static readonly discriminator = new Uint8Array([112, 119, 121, 112, 111, 108, 121, 0]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    pathwayKind!: U8;
    pad0!: U8[];
    pathwayId!: U8[];
    assetMint!: PublicKeyLike;
    issuedTokenMint!: PublicKeyLike;
    allowedProgramMask!: BU64[];
    allowedInstructionMask!: BU64[];
    designatedExecutor!: PublicKeyLike;
    sourceAccountPolicy!: BU64;
    destinationAccountPolicy!: BU64;
    reserveCompartmentPolicyId!: U8[];
    limitPolicyId!: U8[];
    evidencePolicyId!: U8[];
    feePolicyId!: U8[];
    insurancePolicyId!: U8[];
    statusFlags!: BU64;
    requiredIssuedTokenModuleMask!: BU64[];
    requiredCollateralModuleMask!: BU64[];
    forbiddenIssuedTokenExtensionMask!: BU64[];
    forbiddenCollateralExtensionMask!: BU64[];
    assetMintLimitPolicyId!: U8[];
    assetRedeemLimitPolicyId!: U8[];
    counterpartyLimitPolicyId!: U8[];
    executorLimitPolicyId!: U8[];
    reserved!: U8[];

    constructor(props: DecodableProps<PathwayPolicy>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): PathwayPolicy {
        return Decoder.decode(data, PathwayPolicy, address);
    }

    static async get(address: SolanaAddressLike): Promise<PathwayPolicy> {
        return Decoder.getAccount(address, PathwayPolicy);
    }

    static getSchema(): Schema {
        return {
            version: {
                type: SchemaFieldType.U16
            },
            bump: {
                type: SchemaFieldType.U8
            },
            pathwayKind: {
                type: SchemaFieldType.U8
            },
            pad0: {
                type: SchemaFieldType.Array,
                size: 4,
                codableType: SchemaFieldType.U8
            },
            pathwayId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            assetMint: {
                type: SchemaFieldType.Address
            },
            issuedTokenMint: {
                type: SchemaFieldType.Address
            },
            allowedProgramMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            allowedInstructionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            designatedExecutor: {
                type: SchemaFieldType.Address
            },
            sourceAccountPolicy: {
                type: SchemaFieldType.U64
            },
            destinationAccountPolicy: {
                type: SchemaFieldType.U64
            },
            reserveCompartmentPolicyId: {
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
            statusFlags: {
                type: SchemaFieldType.U64
            },
            requiredIssuedTokenModuleMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            requiredCollateralModuleMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            forbiddenIssuedTokenExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            forbiddenCollateralExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            assetMintLimitPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            assetRedeemLimitPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            counterpartyLimitPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            executorLimitPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            reserved: {
                type: SchemaFieldType.Array,
                size: 64,
                codableType: SchemaFieldType.U8
            }
        };
    }
}
