// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      account
// File:          IssuedTokenControl.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/issuance/state/issued_token_control.rs
 */
export class IssuedTokenControl implements IsDecodable {
    static readonly discriminator = new Uint8Array([105, 116, 99, 116, 114, 108, 48, 1]);
    address!: PublicKeyLike;
    version!: U16;
    bump!: U8;
    pad0!: U8[];
    issuedTokenMint!: PublicKeyLike;
    issuedTokenProgram!: PublicKeyLike;
    reservedMintExtensionMask!: BU64[];
    activeMintExtensionMask!: BU64[];
    reservedAccountExtensionMask!: BU64[];
    activeAccountExtensionMask!: BU64[];
    controlFlags!: BU64;
    mintAuthorityPda!: PublicKeyLike;
    freezeAuthorityPda!: PublicKeyLike;
    closeMintAuthorityPda!: PublicKeyLike;
    transferHookAuthorityPda!: PublicKeyLike;
    permanentDelegateAuthorityPda!: PublicKeyLike;
    metadataPointerAuthorityPda!: PublicKeyLike;
    metadataUpdateAuthorityPda!: PublicKeyLike;
    pauseAuthorityPda!: PublicKeyLike;
    confidentialTransferAuthorityPda!: PublicKeyLike;
    defaultAccountStateAuthorityPda!: PublicKeyLike;
    hookProgramId!: PublicKeyLike;
    permanentDelegate!: PublicKeyLike;
    metadataAddress!: PublicKeyLike;
    lastConfiguredAtSlot!: BU64;
    configuredBy!: PublicKeyLike;
    extensionObservedAtSlot!: BU64;
    maxExtensionObservationAgeSlots!: BU64;
    reserved!: U8[];

    constructor(props: DecodableProps<IssuedTokenControl>, address: PublicKeyLike) {
        Object.assign(this, props);
        this.address = address;
    }

    static decode(data: string | Uint8Array | Buffer | null, address: SolanaAddressLike): IssuedTokenControl {
        return Decoder.decode(data, IssuedTokenControl, address);
    }

    static async get(address: SolanaAddressLike): Promise<IssuedTokenControl> {
        return Decoder.getAccount(address, IssuedTokenControl);
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
            issuedTokenMint: {
                type: SchemaFieldType.Address
            },
            issuedTokenProgram: {
                type: SchemaFieldType.Address
            },
            reservedMintExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            activeMintExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            reservedAccountExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            activeAccountExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            controlFlags: {
                type: SchemaFieldType.U64
            },
            mintAuthorityPda: {
                type: SchemaFieldType.Address
            },
            freezeAuthorityPda: {
                type: SchemaFieldType.Address
            },
            closeMintAuthorityPda: {
                type: SchemaFieldType.Address
            },
            transferHookAuthorityPda: {
                type: SchemaFieldType.Address
            },
            permanentDelegateAuthorityPda: {
                type: SchemaFieldType.Address
            },
            metadataPointerAuthorityPda: {
                type: SchemaFieldType.Address
            },
            metadataUpdateAuthorityPda: {
                type: SchemaFieldType.Address
            },
            pauseAuthorityPda: {
                type: SchemaFieldType.Address
            },
            confidentialTransferAuthorityPda: {
                type: SchemaFieldType.Address
            },
            defaultAccountStateAuthorityPda: {
                type: SchemaFieldType.Address
            },
            hookProgramId: {
                type: SchemaFieldType.Address
            },
            permanentDelegate: {
                type: SchemaFieldType.Address
            },
            metadataAddress: {
                type: SchemaFieldType.Address
            },
            lastConfiguredAtSlot: {
                type: SchemaFieldType.U64
            },
            configuredBy: {
                type: SchemaFieldType.Address
            },
            extensionObservedAtSlot: {
                type: SchemaFieldType.U64
            },
            maxExtensionObservationAgeSlots: {
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
