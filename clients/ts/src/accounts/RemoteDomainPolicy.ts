// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          RemoteDomainPolicy.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/cross_chain/state/remote_domain_policy.rs
 */
export class RemoteDomainPolicy implements IsDecodable {
    static readonly discriminator = new Uint8Array([114, 109, 100, 109, 112, 111, 108, 121]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    remoteChainKind!: U8;
    minimumAttestationThreshold!: U8;
    pad0!: U8[];
    remoteDomainId!: BU64;
    requiredFinalityDepth!: BU64;
    messageExpirySeconds!: BU64;
    perMessageMaximum!: BU64;
    perDayMaximum!: BU64;
    statusFlags!: BU64;
    updatedAtSlot!: BU64;
    remoteDomainSeparator!: U8[];
    remoteChanceryContract!: U8[];
    remoteIssuedToken!: U8[];
    signerSetId!: U8[];
    createdBy!: PublicKeyLike;
    mode!: U8;
    padMode!: U8[];
    remoteAsset!: U8[];
    localAssetMint!: PublicKeyLike;
    reserved!: U8[];

    constructor(props: DecodableProps<RemoteDomainPolicy>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): RemoteDomainPolicy {
        return Decoder.decode(data, RemoteDomainPolicy, address);
    }

    static async get(address: SolanaAddressLike): Promise<RemoteDomainPolicy> {
        return Decoder.getAccount(address, RemoteDomainPolicy);
    }

    static getSchema(): Schema {
        return {
            version: {
                type: SchemaFieldType.U16
            },
            bump: {
                type: SchemaFieldType.U8
            },
            remoteChainKind: {
                type: SchemaFieldType.U8
            },
            minimumAttestationThreshold: {
                type: SchemaFieldType.U8
            },
            pad0: {
                type: SchemaFieldType.Array,
                size: 3,
                codableType: SchemaFieldType.U8
            },
            remoteDomainId: {
                type: SchemaFieldType.U64
            },
            requiredFinalityDepth: {
                type: SchemaFieldType.U64
            },
            messageExpirySeconds: {
                type: SchemaFieldType.U64
            },
            perMessageMaximum: {
                type: SchemaFieldType.U64
            },
            perDayMaximum: {
                type: SchemaFieldType.U64
            },
            statusFlags: {
                type: SchemaFieldType.U64
            },
            updatedAtSlot: {
                type: SchemaFieldType.U64
            },
            remoteDomainSeparator: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            remoteChanceryContract: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            remoteIssuedToken: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            signerSetId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            createdBy: {
                type: SchemaFieldType.Address
            },
            mode: {
                type: SchemaFieldType.U8
            },
            padMode: {
                type: SchemaFieldType.Array,
                size: 7,
                codableType: SchemaFieldType.U8
            },
            remoteAsset: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            localAssetMint: {
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
