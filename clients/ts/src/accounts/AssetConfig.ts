// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          AssetConfig.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/core/state/asset_config.rs
 */
export class AssetConfig implements IsDecodable {
    static readonly discriminator = new Uint8Array([97, 115, 115, 116, 99, 102, 103, 0]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    decimals!: U8;
    mode!: U8;
    pad0!: U8[];
    assetFlags!: BU64;
    assetMint!: PublicKeyLike;
    assetTokenProgram!: PublicKeyLike;
    primaryReserveCompartmentId!: U8[];
    approvedExtensionMask!: BU64[];
    observedExtensionMask!: BU64[];
    depositRateE9!: BU64;
    redeemRateE9!: BU64;
    minimumDepositAmount!: BU64;
    minimumRedeemAmount!: BU64;
    maximumSingleSettlementAmount!: BU64;
    statusFlags!: BU64;
    forbiddenExtensionMask!: BU64[];
    requiredModuleMask!: BU64[];
    extensionObservedAtSlot!: BU64;
    maxExtensionObservationAgeSlots!: BU64;
    reserved!: U8[];

    constructor(props: DecodableProps<AssetConfig>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): AssetConfig {
        return Decoder.decode(data, AssetConfig, address);
    }

    static async get(address: SolanaAddressLike): Promise<AssetConfig> {
        return Decoder.getAccount(address, AssetConfig);
    }

    static getSchema(): Schema {
        return {
            version: {
                type: SchemaFieldType.U16
            },
            bump: {
                type: SchemaFieldType.U8
            },
            decimals: {
                type: SchemaFieldType.U8
            },
            mode: {
                type: SchemaFieldType.U8
            },
            pad0: {
                type: SchemaFieldType.Array,
                size: 3,
                codableType: SchemaFieldType.U8
            },
            assetFlags: {
                type: SchemaFieldType.U64
            },
            assetMint: {
                type: SchemaFieldType.Address
            },
            assetTokenProgram: {
                type: SchemaFieldType.Address
            },
            primaryReserveCompartmentId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            approvedExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            observedExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            depositRateE9: {
                type: SchemaFieldType.U64
            },
            redeemRateE9: {
                type: SchemaFieldType.U64
            },
            minimumDepositAmount: {
                type: SchemaFieldType.U64
            },
            minimumRedeemAmount: {
                type: SchemaFieldType.U64
            },
            maximumSingleSettlementAmount: {
                type: SchemaFieldType.U64
            },
            statusFlags: {
                type: SchemaFieldType.U64
            },
            forbiddenExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            requiredModuleMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            extensionObservedAtSlot: {
                type: SchemaFieldType.U64
            },
            maxExtensionObservationAgeSlots: {
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
