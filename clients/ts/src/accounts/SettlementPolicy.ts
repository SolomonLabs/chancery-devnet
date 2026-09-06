// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          SettlementPolicy.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/settlement/state/settlement_policy.rs
 */
export class SettlementPolicy implements IsDecodable {
    static readonly discriminator = new Uint8Array([115, 116, 108, 112, 111, 108, 121, 0]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    pad0!: U8[];
    policyFlags!: BU64;
    allowedSettlementModes!: U32;
    pad1!: U8[];
    policyId!: U8[];
    allowedAssetMint!: PublicKeyLike;
    allowedPrincipalA!: PublicKeyLike;
    allowedPrincipalB!: PublicKeyLike;
    designatedExecutor!: PublicKeyLike;
    maxNotional!: BU64;
    minNotional!: BU64;
    validAfterUnixTimestamp!: BI64;
    expiresAtUnixTimestamp!: BI64;
    createdBy!: PublicKeyLike;
    reserved!: U8[];

    constructor(props: DecodableProps<SettlementPolicy>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): SettlementPolicy {
        return Decoder.decode(data, SettlementPolicy, address);
    }

    static async get(address: SolanaAddressLike): Promise<SettlementPolicy> {
        return Decoder.getAccount(address, SettlementPolicy);
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
            policyFlags: {
                type: SchemaFieldType.U64
            },
            allowedSettlementModes: {
                type: SchemaFieldType.U32
            },
            pad1: {
                type: SchemaFieldType.Array,
                size: 4,
                codableType: SchemaFieldType.U8
            },
            policyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            allowedAssetMint: {
                type: SchemaFieldType.Address
            },
            allowedPrincipalA: {
                type: SchemaFieldType.Address
            },
            allowedPrincipalB: {
                type: SchemaFieldType.Address
            },
            designatedExecutor: {
                type: SchemaFieldType.Address
            },
            maxNotional: {
                type: SchemaFieldType.U64
            },
            minNotional: {
                type: SchemaFieldType.U64
            },
            validAfterUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            expiresAtUnixTimestamp: {
                type: SchemaFieldType.I64
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
