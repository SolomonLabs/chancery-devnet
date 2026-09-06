// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          RemoteNonce.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/cross_chain/state/remote_nonce.rs
 */
export class RemoteNonce implements IsDecodable {
    static readonly discriminator = new Uint8Array([114, 109, 116, 110, 111, 110, 99, 101]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    pad0!: U8[];
    remoteDomainId!: BU64;
    nextInboundNonce!: BU64;
    nextOutboundNonce!: BU64;
    scopeKey!: U8[];
    lastConsumedMessageHash!: U8[];
    lastEmittedMessageHash!: U8[];
    reserved!: U8[];

    constructor(props: DecodableProps<RemoteNonce>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): RemoteNonce {
        return Decoder.decode(data, RemoteNonce, address);
    }

    static async get(address: SolanaAddressLike): Promise<RemoteNonce> {
        return Decoder.getAccount(address, RemoteNonce);
    }

    static getSchema(): Schema {
        return {
            version: {
                type: SchemaFieldType.U16
            },
            bump: {
                type: SchemaFieldType.U8
            },
            pad0: {
                type: SchemaFieldType.Array,
                size: 5,
                codableType: SchemaFieldType.U8
            },
            remoteDomainId: {
                type: SchemaFieldType.U64
            },
            nextInboundNonce: {
                type: SchemaFieldType.U64
            },
            nextOutboundNonce: {
                type: SchemaFieldType.U64
            },
            scopeKey: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            lastConsumedMessageHash: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            lastEmittedMessageHash: {
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
