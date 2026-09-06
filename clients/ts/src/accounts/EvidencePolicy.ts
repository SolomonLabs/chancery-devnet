// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          EvidencePolicy.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/state/evidence_policy.rs
 */
export class EvidencePolicy implements IsDecodable {
    static readonly discriminator = new Uint8Array([101, 118, 100, 112, 111, 108, 121, 0]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    allowFreeformCounterpartyFields!: U8;
    pad0!: U8[];
    evidencePolicyId!: U8[];
    requiredFieldMask!: BU64[];
    counterpartyReportingSchemaHash!: U8[];
    maximumFreeformFieldCount!: U16;
    maximumFreeformValueBytes!: U16;
    pad1!: U8[];
    retentionFlags!: BU64;
    reserved!: U8[];

    constructor(props: DecodableProps<EvidencePolicy>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): EvidencePolicy {
        return Decoder.decode(data, EvidencePolicy, address);
    }

    static async get(address: SolanaAddressLike): Promise<EvidencePolicy> {
        return Decoder.getAccount(address, EvidencePolicy);
    }

    static getSchema(): Schema {
        return {
            version: {
                type: SchemaFieldType.U16
            },
            bump: {
                type: SchemaFieldType.U8
            },
            allowFreeformCounterpartyFields: {
                type: SchemaFieldType.U8
            },
            pad0: {
                type: SchemaFieldType.Array,
                size: 4,
                codableType: SchemaFieldType.U8
            },
            evidencePolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            requiredFieldMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            counterpartyReportingSchemaHash: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            maximumFreeformFieldCount: {
                type: SchemaFieldType.U16
            },
            maximumFreeformValueBytes: {
                type: SchemaFieldType.U16
            },
            pad1: {
                type: SchemaFieldType.Array,
                size: 4,
                codableType: SchemaFieldType.U8
            },
            retentionFlags: {
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
