// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          OutboundReclaimRecord.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/cross_chain/state/outbound_reclaim_record.rs
 */
export class OutboundReclaimRecord implements IsDecodable {
    static readonly discriminator = new Uint8Array([111, 117, 116, 114, 101, 99, 108, 109]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    messageKind!: U8;
    remoteChainKind!: U8;
    retirementReason!: U8;
    pad0!: U8[];
    remoteDomainId!: BU64;
    sourceNonce!: BU64;
    amountWords!: BU64[];
    reclaimedAtSlot!: BU64;
    reclaimedAtUnixTimestamp!: BI64;
    epochFreeContentHash!: U8[];
    reclaimDigest!: U8[];
    sender!: U8[];
    attestingSignerSetId!: U8[];
    reserved!: U8[];

    constructor(props: DecodableProps<OutboundReclaimRecord>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): OutboundReclaimRecord {
        return Decoder.decode(data, OutboundReclaimRecord, address);
    }

    static async get(address: SolanaAddressLike): Promise<OutboundReclaimRecord> {
        return Decoder.getAccount(address, OutboundReclaimRecord);
    }

    static getSchema(): Schema {
        return {
            version: {
                type: SchemaFieldType.U16
            },
            bump: {
                type: SchemaFieldType.U8
            },
            messageKind: {
                type: SchemaFieldType.U8
            },
            remoteChainKind: {
                type: SchemaFieldType.U8
            },
            retirementReason: {
                type: SchemaFieldType.U8
            },
            pad0: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U8
            },
            remoteDomainId: {
                type: SchemaFieldType.U64
            },
            sourceNonce: {
                type: SchemaFieldType.U64
            },
            amountWords: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            reclaimedAtSlot: {
                type: SchemaFieldType.U64
            },
            reclaimedAtUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            epochFreeContentHash: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            reclaimDigest: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            sender: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            attestingSignerSetId: {
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
