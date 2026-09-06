// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          PendingConfigChange.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/control/state/pending_config_change.rs
 */
export class PendingConfigChange implements IsDecodable {
    static readonly discriminator = new Uint8Array([112, 101, 110, 100, 99, 102, 103, 1]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    status!: U8;
    riskClass!: U8;
    pad0!: U8[];
    changeId!: U8[];
    changeKind!: U16;
    pad1!: U8[];
    targetAccount!: PublicKeyLike;
    oldValueHash!: U8[];
    newValueHash!: U8[];
    proposedBy!: PublicKeyLike;
    proposerNonce!: BU64;
    executableAfterUnixTimestamp!: BI64;
    expiresAtUnixTimestamp!: BI64;
    proposedAtSlot!: BU64;
    acceptedAtSlot!: BU64;
    cancelledAtSlot!: BU64;
    consumedAtSlot!: BU64;
    lastEventSequenceNonce!: BU64;
    rentRefundRecipient!: PublicKeyLike;
    reserved!: U8[];

    constructor(props: DecodableProps<PendingConfigChange>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): PendingConfigChange {
        return Decoder.decode(data, PendingConfigChange, address);
    }

    static async get(address: SolanaAddressLike): Promise<PendingConfigChange> {
        return Decoder.getAccount(address, PendingConfigChange);
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
            riskClass: {
                type: SchemaFieldType.U8
            },
            pad0: {
                type: SchemaFieldType.Array,
                size: 3,
                codableType: SchemaFieldType.U8
            },
            changeId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            changeKind: {
                type: SchemaFieldType.U16
            },
            pad1: {
                type: SchemaFieldType.Array,
                size: 6,
                codableType: SchemaFieldType.U8
            },
            targetAccount: {
                type: SchemaFieldType.Address
            },
            oldValueHash: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            newValueHash: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            proposedBy: {
                type: SchemaFieldType.Address
            },
            proposerNonce: {
                type: SchemaFieldType.U64
            },
            executableAfterUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            expiresAtUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            proposedAtSlot: {
                type: SchemaFieldType.U64
            },
            acceptedAtSlot: {
                type: SchemaFieldType.U64
            },
            cancelledAtSlot: {
                type: SchemaFieldType.U64
            },
            consumedAtSlot: {
                type: SchemaFieldType.U64
            },
            lastEventSequenceNonce: {
                type: SchemaFieldType.U64
            },
            rentRefundRecipient: {
                type: SchemaFieldType.Address
            },
            reserved: {
                type: SchemaFieldType.Array,
                size: 16,
                codableType: SchemaFieldType.U8
            }
        };
    }
}
