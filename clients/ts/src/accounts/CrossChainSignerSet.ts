// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          CrossChainSignerSet.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/cross_chain/state/cross_chain_signer_set.rs
 */
export class CrossChainSignerSet implements IsDecodable {
    static readonly discriminator = new Uint8Array([99, 99, 115, 105, 103, 115, 101, 116]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    threshold!: U8;
    signerCount!: U8;
    pad0!: U8[];
    validAfterUnixTimestamp!: BI64;
    expiresAtUnixTimestamp!: BI64;
    statusFlags!: BU64;
    signerSetId!: U8[];
    signerRoot!: U8[];
    createdBy!: PublicKeyLike;
    reserved!: U8[];

    constructor(props: DecodableProps<CrossChainSignerSet>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): CrossChainSignerSet {
        return Decoder.decode(data, CrossChainSignerSet, address);
    }

    static async get(address: SolanaAddressLike): Promise<CrossChainSignerSet> {
        return Decoder.getAccount(address, CrossChainSignerSet);
    }

    static getSchema(): Schema {
        return {
            version: {
                type: SchemaFieldType.U16
            },
            bump: {
                type: SchemaFieldType.U8
            },
            threshold: {
                type: SchemaFieldType.U8
            },
            signerCount: {
                type: SchemaFieldType.U8
            },
            pad0: {
                type: SchemaFieldType.Array,
                size: 3,
                codableType: SchemaFieldType.U8
            },
            validAfterUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            expiresAtUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            statusFlags: {
                type: SchemaFieldType.U64
            },
            signerSetId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            signerRoot: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            createdBy: {
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
