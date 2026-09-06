// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          RegisterAssetInstruction.ts
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
import { SYSTEM_PROGRAM_ID_STRING } from "@solomon-labs/constants";
import { PublicKey } from "@solomon-labs/publickey";
import type {
    AccountRelationsSchema,
    EncodableDefault,
    EncodableProps,
    EncodeAccountsSchemaOrNull,
    EncodedInstruction,
    InstructionAccount,
    IsEncodable,
    ProgramDerivedAccountSchema,
    Schema
} from "@solomon-labs/solana-codec";
import { Decoder, Encoder, SchemaFieldType } from "@solomon-labs/solana-codec";
import type { BU64, PublicKeyLike, U8 } from "@solomon-labs/types";

import {
    PROGRAM_ID,
    ACCOUNT_SIZES,
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/core/instructions/register_asset.rs
 */
export class RegisterAssetInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        assetConfig: {
            seeds: [
                { kind: "const", value: [97, 115, 115, 101, 116, 45, 99, 111, 110, 102, 105, 103] },
                { kind: "account", path: "assetMint" }
            ]
        }
    };

    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["operationsAuthority"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.CORE, IX.core.REGISTER_ASSET]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    mode!: U8;
    approvedExtensionMask!: BU64[];
    observedExtensionMaskHint!: BU64[] | null | undefined;
    depositRateE9!: BU64;
    redeemRateE9!: BU64;
    minimumDepositAmount!: BU64;
    minimumRedeemAmount!: BU64;
    maximumSingleSettlementAmount!: BU64;
    maxExtensionObservationAgeSlots!: BU64;
    assetTokenProgram!: PublicKeyLike;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    assetConfig!: PublicKeyLike;
    assetMint!: PublicKeyLike;
    payer!: PublicKeyLike;
    operationsAuthority!: PublicKeyLike;
    systemProgram: PublicKeyLike & EncodableDefault = new PublicKey(SYSTEM_PROGRAM_ID_STRING);
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<RegisterAssetInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, RegisterAssetInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): RegisterAssetInstruction {
        return Decoder.decode(data, RegisterAssetInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, RegisterAssetInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, RegisterAssetInstruction);
    }

    static getSchema(): Schema {
        return {
            mode: {
                type: SchemaFieldType.U8
            },
            approvedExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            observedExtensionMaskHint: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64,
                optional: true
            },
            depositRateE9: {
                type: SchemaFieldType.U64
            },
            redeemRateE9: {
                type: SchemaFieldType.U64
            },
            minimumDepositAmount: {
                type: SchemaFieldType.U64
            },
            minimumRedeemAmount: {
                type: SchemaFieldType.U64
            },
            maximumSingleSettlementAmount: {
                type: SchemaFieldType.U64
            },
            maxExtensionObservationAgeSlots: {
                type: SchemaFieldType.U64
            },
            assetTokenProgram: {
                type: SchemaFieldType.Address
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            chanceryConfig: { signer: false, writable: true, relations: RegisterAssetInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            assetConfig: { signer: false, writable: true, pda: RegisterAssetInstruction.programDerivedAccounts.assetConfig, accountSize: ACCOUNT_SIZES.ASSET_CONFIG, willCreate: true },
            assetMint: { signer: false, writable: false },
            payer: { signer: true, writable: true },
            operationsAuthority: { signer: true, writable: false },
            systemProgram: { signer: false, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
