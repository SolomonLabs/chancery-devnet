// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          ModuleActivationState.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/control/state/module_activation_state.rs
 */
export class ModuleActivationState implements IsDecodable {
    static readonly discriminator = new Uint8Array([109, 111, 100, 97, 99, 116, 118, 1]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    pad0!: U8[];
    moduleStatuses!: U8[];
    lastUpdatedBy!: PublicKeyLike;
    lastUpdatedAtSlot!: BU64;
    lastEventSequenceNonce!: BU64;
    reserved!: U8[];

    constructor(props: DecodableProps<ModuleActivationState>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): ModuleActivationState {
        return Decoder.decode(data, ModuleActivationState, address);
    }

    static async get(address: SolanaAddressLike): Promise<ModuleActivationState> {
        return Decoder.getAccount(address, ModuleActivationState);
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
            moduleStatuses: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            lastUpdatedBy: {
                type: SchemaFieldType.Address
            },
            lastUpdatedAtSlot: {
                type: SchemaFieldType.U64
            },
            lastEventSequenceNonce: {
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
