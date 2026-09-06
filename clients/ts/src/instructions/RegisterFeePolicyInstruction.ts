// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          RegisterFeePolicyInstruction.ts
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
import type { BI64, BU64, PublicKeyLike, U32, U8 } from "@solomon-labs/types";

import {
    PROGRAM_ID,
    ACCOUNT_SIZES,
    MODULE_ACTIVATION_STATE,
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/fees/instructions/register_fee_policy.rs
 */
export class RegisterFeePolicyInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        moduleActivationState: {
            seeds: [
                { kind: "const", value: [109, 111, 100, 117, 108, 101, 45, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 45, 115, 116, 97, 116, 101] }
            ]
        },
        feePolicy: {
            seeds: [
                { kind: "const", value: [102, 101, 101, 45, 112, 111, 108, 105, 99, 121] },
                { kind: "arg", path: "feePolicyId" }
            ]
        }
    };

    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["operationsAuthority"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.FEES, IX.fees.REGISTER_FEE_POLICY]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    feePolicyId!: U8[];
    feePolicyFlags!: BU64;
    flatFeeInAsset!: BU64;
    flatFeeInIssuedToken!: BU64;
    percentFeeBps!: U32;
    feeCapAmount!: BU64;
    minimumFeeAmount!: BU64;
    rebateFlatAmount!: BU64;
    rebateBps!: U32;
    rebateCapAmount!: BU64;
    netFeeFloorZero!: boolean;
    feeRecipientPolicy!: U8;
    roundingMode!: U8;
    feeRecipientKey!: PublicKeyLike;
    effectiveFromUnixTimestamp!: BI64;
    effectiveUntilUnixTimestamp!: BI64;
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    feePolicy!: PublicKeyLike;
    payer!: PublicKeyLike;
    operationsAuthority!: PublicKeyLike;
    systemProgram: PublicKeyLike & EncodableDefault = new PublicKey(SYSTEM_PROGRAM_ID_STRING);
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<RegisterFeePolicyInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, RegisterFeePolicyInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): RegisterFeePolicyInstruction {
        return Decoder.decode(data, RegisterFeePolicyInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, RegisterFeePolicyInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, RegisterFeePolicyInstruction);
    }

    static getSchema(): Schema {
        return {
            feePolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            feePolicyFlags: {
                type: SchemaFieldType.U64
            },
            flatFeeInAsset: {
                type: SchemaFieldType.U64
            },
            flatFeeInIssuedToken: {
                type: SchemaFieldType.U64
            },
            percentFeeBps: {
                type: SchemaFieldType.U32
            },
            feeCapAmount: {
                type: SchemaFieldType.U64
            },
            minimumFeeAmount: {
                type: SchemaFieldType.U64
            },
            rebateFlatAmount: {
                type: SchemaFieldType.U64
            },
            rebateBps: {
                type: SchemaFieldType.U32
            },
            rebateCapAmount: {
                type: SchemaFieldType.U64
            },
            netFeeFloorZero: {
                type: SchemaFieldType.Boolean
            },
            feeRecipientPolicy: {
                type: SchemaFieldType.U8
            },
            roundingMode: {
                type: SchemaFieldType.U8
            },
            feeRecipientKey: {
                type: SchemaFieldType.Address
            },
            effectiveFromUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            effectiveUntilUnixTimestamp: {
                type: SchemaFieldType.I64
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: RegisterFeePolicyInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true, relations: RegisterFeePolicyInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            feePolicy: { signer: false, writable: true, pda: RegisterFeePolicyInstruction.programDerivedAccounts.feePolicy, accountSize: ACCOUNT_SIZES.FEE_POLICY, willCreate: true },
            payer: { signer: true, writable: true },
            operationsAuthority: { signer: true, writable: false },
            systemProgram: { signer: false, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
