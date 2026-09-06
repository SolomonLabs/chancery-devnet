// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          UsageWindow.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/limits/state/usage_window.rs
 */
export class UsageWindow implements IsDecodable {
    static readonly discriminator = new Uint8Array([117, 115, 103, 119, 110, 100, 119, 0]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    windowKind!: U8;
    pad0!: U8[];
    scopeHash!: U8[];
    windowStartUnixTimestamp!: BI64;
    grossIn!: BU64[];
    grossOutputAmount!: BU64[];
    netFlow!: BU64[];
    actionCount!: U32;
    pad1!: U8[];
    rentRefundRecipient!: PublicKeyLike;
    reserved!: U8[];

    constructor(props: DecodableProps<UsageWindow>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): UsageWindow {
        return Decoder.decode(data, UsageWindow, address);
    }

    static async get(address: SolanaAddressLike): Promise<UsageWindow> {
        return Decoder.getAccount(address, UsageWindow);
    }

    static getSchema(): Schema {
        return {
            version: {
                type: SchemaFieldType.U16
            },
            bump: {
                type: SchemaFieldType.U8
            },
            windowKind: {
                type: SchemaFieldType.U8
            },
            pad0: {
                type: SchemaFieldType.Array,
                size: 4,
                codableType: SchemaFieldType.U8
            },
            scopeHash: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            windowStartUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            grossIn: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            grossOutputAmount: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            netFlow: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            actionCount: {
                type: SchemaFieldType.U32
            },
            pad1: {
                type: SchemaFieldType.Array,
                size: 4,
                codableType: SchemaFieldType.U8
            },
            rentRefundRecipient: {
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
