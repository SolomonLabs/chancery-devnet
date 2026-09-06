// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          CreateSettlementIntentInstruction.ts
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
import type { BI64, BU64, PublicKeyLike, U8 } from "@solomon-labs/types";

import {
    PROGRAM_ID,
    ACCOUNT_SIZES,
    MODULE_ACTIVATION_STATE,
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    DEFAULT_PUBLIC_KEY,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/settlement/instructions/create_settlement_intent.rs
 */
export class CreateSettlementIntentInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        moduleActivationState: {
            seeds: [
                { kind: "const", value: [109, 111, 100, 117, 108, 101, 45, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 45, 115, 116, 97, 116, 101] }
            ]
        },
        settlementIntent: {
            seeds: [
                { kind: "const", value: [115, 101, 116, 116, 108, 101, 109, 101, 110, 116, 45, 105, 110, 116, 101, 110, 116] },
                { kind: "arg", path: "intentId" }
            ]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.SETTLEMENT, IX.settlement.CREATE_SETTLEMENT_INTENT]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    intentId!: U8[];
    pathwayId!: U8[];
    settlementMode!: U8;
    settlementAction!: U8;
    principalA!: PublicKeyLike;
    principalB!: PublicKeyLike;
    executor!: PublicKeyLike;
    assetMint!: PublicKeyLike;
    issuedTokenMint!: PublicKeyLike;
    assetAmount!: BU64;
    issuedTokenAmount!: BU64;
    minimumAssetAmount!: BU64;
    minimumIssuedTokenAmount!: BU64;
    nonce!: BU64;
    validAfterUnixTimestamp!: BI64;
    expiresAtUnixTimestamp!: BI64;
    policyId!: U8[];
    intentHash!: U8[];
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    settlementIntent!: PublicKeyLike;
    payer!: PublicKeyLike;
    creator!: PublicKeyLike;
    systemProgram: PublicKeyLike & EncodableDefault = new PublicKey(SYSTEM_PROGRAM_ID_STRING);
    pathwayPolicy!: PublicKeyLike;
    principalAPermissionRecord!: PublicKeyLike;
    principalBPermissionRecord!: PublicKeyLike;
    executorPermissionRecord!: PublicKeyLike;
    settlementPolicy: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    feePolicy: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<CreateSettlementIntentInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, CreateSettlementIntentInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): CreateSettlementIntentInstruction {
        return Decoder.decode(data, CreateSettlementIntentInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, CreateSettlementIntentInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, CreateSettlementIntentInstruction);
    }

    static getSchema(): Schema {
        return {
            intentId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            pathwayId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            settlementMode: {
                type: SchemaFieldType.U8
            },
            settlementAction: {
                type: SchemaFieldType.U8
            },
            principalA: {
                type: SchemaFieldType.Address
            },
            principalB: {
                type: SchemaFieldType.Address
            },
            executor: {
                type: SchemaFieldType.Address
            },
            assetMint: {
                type: SchemaFieldType.Address
            },
            issuedTokenMint: {
                type: SchemaFieldType.Address
            },
            assetAmount: {
                type: SchemaFieldType.U64
            },
            issuedTokenAmount: {
                type: SchemaFieldType.U64
            },
            minimumAssetAmount: {
                type: SchemaFieldType.U64
            },
            minimumIssuedTokenAmount: {
                type: SchemaFieldType.U64
            },
            nonce: {
                type: SchemaFieldType.U64
            },
            validAfterUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            expiresAtUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            policyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            intentHash: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: CreateSettlementIntentInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true },
            eventAuthority: { signer: false, writable: false },
            settlementIntent: { signer: false, writable: true, pda: CreateSettlementIntentInstruction.programDerivedAccounts.settlementIntent, accountSize: ACCOUNT_SIZES.SETTLEMENT_INTENT, willCreate: true },
            payer: { signer: true, writable: true },
            creator: { signer: true, writable: false },
            systemProgram: { signer: false, writable: false },
            pathwayPolicy: { signer: false, writable: false },
            principalAPermissionRecord: { signer: false, writable: false },
            principalBPermissionRecord: { signer: false, writable: false },
            executorPermissionRecord: { signer: false, writable: false },
            settlementPolicy: { signer: false, writable: false },
            feePolicy: { signer: false, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
