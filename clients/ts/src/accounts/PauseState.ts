// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          PauseState.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/control/state/pause_state.rs
 */
export class PauseState implements IsDecodable {
    static readonly discriminator = new Uint8Array([112, 97, 117, 115, 101, 115, 116, 0]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    pad0!: U8[];
    globalPauseBits!: BU64;
    reasonCode!: U32;
    pad1!: U8[];
    activatedBy!: PublicKeyLike;
    activatedAtSlot!: BU64;
    expiresAtSlot!: BU64;
    reserved!: U8[];

    constructor(props: DecodableProps<PauseState>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): PauseState {
        return Decoder.decode(data, PauseState, address);
    }

    static async get(address: SolanaAddressLike): Promise<PauseState> {
        return Decoder.getAccount(address, PauseState);
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
            globalPauseBits: {
                type: SchemaFieldType.U64
            },
            reasonCode: {
                type: SchemaFieldType.U32
            },
            pad1: {
                type: SchemaFieldType.Array,
                size: 4,
                codableType: SchemaFieldType.U8
            },
            activatedBy: {
                type: SchemaFieldType.Address
            },
            activatedAtSlot: {
                type: SchemaFieldType.U64
            },
            expiresAtSlot: {
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
