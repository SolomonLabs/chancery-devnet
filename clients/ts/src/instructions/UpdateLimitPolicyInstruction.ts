// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          UpdateLimitPolicyInstruction.ts
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
import type { BU64, PublicKeyLike, U32, U8 } from "@solomon-labs/types";

import {
    PROGRAM_ID,
    ACCOUNT_SIZES,
    MODULE_ACTIVATION_STATE,
    CHANCERY_CONFIG,
    DEFAULT_PUBLIC_KEY,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/limits/instructions/update_limit_policy.rs
 */
export class UpdateLimitPolicyInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        moduleActivationState: {
            seeds: [
                { kind: "const", value: [109, 111, 100, 117, 108, 101, 45, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 45, 115, 116, 97, 116, 101] }
            ]
        }
    };

    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["operationsAuthority"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.LIMITS, IX.limits.UPDATE_LIMIT_POLICY]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    limitPolicyId!: U8[];
    perTransactionMaximum!: BU64 | null | undefined;
    perHourMaximum!: BU64 | null | undefined;
    perDayMaximum!: BU64 | null | undefined;
    perSevenDayMaximum!: BU64 | null | undefined;
    perThirtyDayMaximum!: BU64 | null | undefined;
    maximumActionsPerHour!: U32 | null | undefined;
    maximumActionsPerDay!: U32 | null | undefined;
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    limitPolicy!: PublicKeyLike;
    operationsAuthority!: PublicKeyLike;
    eventAuthorityDirect!: PublicKeyLike;
    payer!: PublicKeyLike;
    systemProgram: PublicKeyLike & EncodableDefault = new PublicKey(SYSTEM_PROGRAM_ID_STRING);
    hourlyUsageWindow: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    dailyUsageWindow: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    weeklyUsageWindow: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    monthlyUsageWindow: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<UpdateLimitPolicyInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, UpdateLimitPolicyInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): UpdateLimitPolicyInstruction {
        return Decoder.decode(data, UpdateLimitPolicyInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, UpdateLimitPolicyInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, UpdateLimitPolicyInstruction);
    }

    static getSchema(): Schema {
        return {
            limitPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            perTransactionMaximum: {
                type: SchemaFieldType.U64,
                optional: true
            },
            perHourMaximum: {
                type: SchemaFieldType.U64,
                optional: true
            },
            perDayMaximum: {
                type: SchemaFieldType.U64,
                optional: true
            },
            perSevenDayMaximum: {
                type: SchemaFieldType.U64,
                optional: true
            },
            perThirtyDayMaximum: {
                type: SchemaFieldType.U64,
                optional: true
            },
            maximumActionsPerHour: {
                type: SchemaFieldType.U32,
                optional: true
            },
            maximumActionsPerDay: {
                type: SchemaFieldType.U32,
                optional: true
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: UpdateLimitPolicyInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true, relations: UpdateLimitPolicyInstruction.accountRelations.chanceryConfig.relations },
            limitPolicy: { signer: false, writable: true },
            operationsAuthority: { signer: true, writable: false },
            eventAuthorityDirect: { signer: false, writable: false },
            payer: { signer: false, writable: false },
            systemProgram: { signer: false, writable: false },
            hourlyUsageWindow: { signer: false, writable: true },
            dailyUsageWindow: { signer: false, writable: true },
            weeklyUsageWindow: { signer: false, writable: true },
            monthlyUsageWindow: { signer: false, writable: true },
            eventProgram: { signer: false, writable: false }
        };
    }
}
