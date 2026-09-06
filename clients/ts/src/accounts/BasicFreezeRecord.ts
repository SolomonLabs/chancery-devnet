// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          BasicFreezeRecord.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/control/state/basic_freeze_record.rs
 */
export class BasicFreezeRecord implements IsDecodable {
    static readonly discriminator = new Uint8Array([98, 102, 114, 101, 101, 122, 101, 1]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    status!: U8;
    reasonCode!: U32;
    issuedTokenAccount!: PublicKeyLike;
    issuedTokenMint!: PublicKeyLike;
    freezeAuthorityPda!: PublicKeyLike;
    frozenBy!: PublicKeyLike;
    thawedBy!: PublicKeyLike;
    frozenAtSlot!: BU64;
    thawedAtSlot!: BU64;
    lastEventSequenceNonce!: BU64;
    freezeFlags!: BU64;
    thawFlags!: BU64;
    rentRefundRecipient!: PublicKeyLike;
    reserved!: U8[];

    constructor(props: DecodableProps<BasicFreezeRecord>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): BasicFreezeRecord {
        return Decoder.decode(data, BasicFreezeRecord, address);
    }

    static async get(address: SolanaAddressLike): Promise<BasicFreezeRecord> {
        return Decoder.getAccount(address, BasicFreezeRecord);
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
            reasonCode: {
                type: SchemaFieldType.U32
            },
            issuedTokenAccount: {
                type: SchemaFieldType.Address
            },
            issuedTokenMint: {
                type: SchemaFieldType.Address
            },
            freezeAuthorityPda: {
                type: SchemaFieldType.Address
            },
            frozenBy: {
                type: SchemaFieldType.Address
            },
            thawedBy: {
                type: SchemaFieldType.Address
            },
            frozenAtSlot: {
                type: SchemaFieldType.U64
            },
            thawedAtSlot: {
                type: SchemaFieldType.U64
            },
            lastEventSequenceNonce: {
                type: SchemaFieldType.U64
            },
            freezeFlags: {
                type: SchemaFieldType.U64
            },
            thawFlags: {
                type: SchemaFieldType.U64
            },
            rentRefundRecipient: {
                type: SchemaFieldType.Address
            },
            reserved: {
                type: SchemaFieldType.Array,
                size: 24,
                codableType: SchemaFieldType.U8
            }
        };
    }
}
