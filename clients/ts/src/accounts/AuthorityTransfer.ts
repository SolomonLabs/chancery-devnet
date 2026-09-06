// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          AuthorityTransfer.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/core/state/authority_transfer.rs
 */
export class AuthorityTransfer implements IsDecodable {
    static readonly discriminator = new Uint8Array([97, 117, 116, 104, 120, 102, 114, 0]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    roleKind!: U8;
    pad0!: U8[];
    oldAuthority!: PublicKeyLike;
    proposedAuthority!: PublicKeyLike;
    proposedAtSlot!: BU64;
    executableAfterSlot!: BU64;
    proposingGovernance!: PublicKeyLike;
    expiresAtSlot!: BU64;
    reserved!: U8[];

    constructor(props: DecodableProps<AuthorityTransfer>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): AuthorityTransfer {
        return Decoder.decode(data, AuthorityTransfer, address);
    }

    static async get(address: SolanaAddressLike): Promise<AuthorityTransfer> {
        return Decoder.getAccount(address, AuthorityTransfer);
    }

    static getSchema(): Schema {
        return {
            version: {
                type: SchemaFieldType.U16
            },
            bump: {
                type: SchemaFieldType.U8
            },
            roleKind: {
                type: SchemaFieldType.U8
            },
            pad0: {
                type: SchemaFieldType.Array,
                size: 4,
                codableType: SchemaFieldType.U8
            },
            oldAuthority: {
                type: SchemaFieldType.Address
            },
            proposedAuthority: {
                type: SchemaFieldType.Address
            },
            proposedAtSlot: {
                type: SchemaFieldType.U64
            },
            executableAfterSlot: {
                type: SchemaFieldType.U64
            },
            proposingGovernance: {
                type: SchemaFieldType.Address
            },
            expiresAtSlot: {
                type: SchemaFieldType.U64
            },
            reserved: {
                type: SchemaFieldType.Array,
                size: 16,
                codableType: SchemaFieldType.U8
            }
        };
    }
}
