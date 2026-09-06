// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          ReserveDestination.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/reserve/state/reserve_destination.rs
 */
export class ReserveDestination implements IsDecodable {
    static readonly discriminator = new Uint8Array([114, 115, 118, 100, 115, 116, 0, 0]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    status!: U8;
    pad0!: U8[];
    assetMint!: PublicKeyLike;
    destinationTokenAccount!: PublicKeyLike;
    destinationOwner!: PublicKeyLike;
    destinationFlags!: BU64;
    approvedBy!: PublicKeyLike;
    withdrawalLimitPolicyId!: U8[];
    reserved!: U8[];

    constructor(props: DecodableProps<ReserveDestination>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): ReserveDestination {
        return Decoder.decode(data, ReserveDestination, address);
    }

    static async get(address: SolanaAddressLike): Promise<ReserveDestination> {
        return Decoder.getAccount(address, ReserveDestination);
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
            pad0: {
                type: SchemaFieldType.Array,
                size: 4,
                codableType: SchemaFieldType.U8
            },
            assetMint: {
                type: SchemaFieldType.Address
            },
            destinationTokenAccount: {
                type: SchemaFieldType.Address
            },
            destinationOwner: {
                type: SchemaFieldType.Address
            },
            destinationFlags: {
                type: SchemaFieldType.U64
            },
            approvedBy: {
                type: SchemaFieldType.Address
            },
            withdrawalLimitPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            reserved: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            }
        };
    }
}
