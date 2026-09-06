// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          ChanceryConfig.ts
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
import type { BU64, PublicKeyLike, SolanaAddressLike, U16, U32, U8 } from "@solomon-labs/types";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/core/state/chancery_config.rs
 */
export class ChanceryConfig implements IsDecodable {
    static readonly discriminator = new Uint8Array([118, 97, 117, 108, 116, 99, 102, 103]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    pad0!: U8;
    pad1!: U8[];
    statusFlags!: BU64;
    governanceAuthority!: PublicKeyLike;
    operationsAuthority!: PublicKeyLike;
    emergencyAuthority!: PublicKeyLike;
    enforcementAuthority!: PublicKeyLike;
    insuranceAdminAuthority!: PublicKeyLike;
    issuedTokenMint!: PublicKeyLike;
    issuedTokenProgram!: PublicKeyLike;
    legacyTokenMint!: PublicKeyLike;
    legacyTokenProgram!: PublicKeyLike;
    mintAuthorityPda!: PublicKeyLike;
    freezeAuthorityPda!: PublicKeyLike;
    eventSequenceNonce!: BU64;
    domainSeparator!: U8[];
    eventAuthorityBump!: U8;
    mintAuthorityBump!: U8;
    reserveAuthorityBump!: U8;
    authorityBumpCacheVersion!: U8;
    padAuthorityBumps!: U8[];
    totalRemoteDomainsRegistered!: U32;
    totalSignerSetsRegistered!: U32;
    allocatedPermissionRecordSlots!: U32;
    totalReserveDestinationsRegistered!: U32;
    totalLimitPoliciesRegistered!: U32;
    totalPathwayPoliciesRegistered!: U32;
    reserved!: U8[];

    constructor(props: DecodableProps<ChanceryConfig>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): ChanceryConfig {
        return Decoder.decode(data, ChanceryConfig, address);
    }

    static async get(address: SolanaAddressLike): Promise<ChanceryConfig> {
        return Decoder.getAccount(address, ChanceryConfig);
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
                type: SchemaFieldType.U8
            },
            pad1: {
                type: SchemaFieldType.Array,
                size: 4,
                codableType: SchemaFieldType.U8
            },
            statusFlags: {
                type: SchemaFieldType.U64
            },
            governanceAuthority: {
                type: SchemaFieldType.Address
            },
            operationsAuthority: {
                type: SchemaFieldType.Address
            },
            emergencyAuthority: {
                type: SchemaFieldType.Address
            },
            enforcementAuthority: {
                type: SchemaFieldType.Address
            },
            insuranceAdminAuthority: {
                type: SchemaFieldType.Address
            },
            issuedTokenMint: {
                type: SchemaFieldType.Address
            },
            issuedTokenProgram: {
                type: SchemaFieldType.Address
            },
            legacyTokenMint: {
                type: SchemaFieldType.Address
            },
            legacyTokenProgram: {
                type: SchemaFieldType.Address
            },
            mintAuthorityPda: {
                type: SchemaFieldType.Address
            },
            freezeAuthorityPda: {
                type: SchemaFieldType.Address
            },
            eventSequenceNonce: {
                type: SchemaFieldType.U64
            },
            domainSeparator: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            eventAuthorityBump: {
                type: SchemaFieldType.U8
            },
            mintAuthorityBump: {
                type: SchemaFieldType.U8
            },
            reserveAuthorityBump: {
                type: SchemaFieldType.U8
            },
            authorityBumpCacheVersion: {
                type: SchemaFieldType.U8
            },
            padAuthorityBumps: {
                type: SchemaFieldType.Array,
                size: 4,
                codableType: SchemaFieldType.U8
            },
            totalRemoteDomainsRegistered: {
                type: SchemaFieldType.U32
            },
            totalSignerSetsRegistered: {
                type: SchemaFieldType.U32
            },
            allocatedPermissionRecordSlots: {
                type: SchemaFieldType.U32
            },
            totalReserveDestinationsRegistered: {
                type: SchemaFieldType.U32
            },
            totalLimitPoliciesRegistered: {
                type: SchemaFieldType.U32
            },
            totalPathwayPoliciesRegistered: {
                type: SchemaFieldType.U32
            },
            reserved: {
                type: SchemaFieldType.Array,
                size: 24,
                codableType: SchemaFieldType.U8
            }
        };
    }
}
