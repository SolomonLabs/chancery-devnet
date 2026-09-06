// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          RegisterSettlementPolicyInstruction.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/settlement/instructions/register_settlement_policy.rs
 */
export class RegisterSettlementPolicyInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        moduleActivationState: {
            seeds: [
                { kind: "const", value: [109, 111, 100, 117, 108, 101, 45, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 45, 115, 116, 97, 116, 101] }
            ]
        },
        settlementPolicy: {
            seeds: [
                { kind: "const", value: [115, 101, 116, 116, 108, 101, 109, 101, 110, 116, 45, 112, 111, 108, 105, 99, 121] },
                { kind: "arg", path: "policyId" }
            ]
        }
    };

    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["operationsAuthority"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.SETTLEMENT, IX.settlement.REGISTER_SETTLEMENT_POLICY]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    policyId!: U8[];
    policyFlags!: BU64;
    allowedSettlementModes!: U32;
    allowedAssetMint!: PublicKeyLike;
    allowedPrincipalA!: PublicKeyLike;
    allowedPrincipalB!: PublicKeyLike;
    designatedExecutor!: PublicKeyLike;
    maxNotional!: BU64;
    minNotional!: BU64;
    validAfterUnixTimestamp!: BI64;
    expiresAtUnixTimestamp!: BI64;
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    settlementPolicy!: PublicKeyLike;
    payer!: PublicKeyLike;
    operationsAuthority!: PublicKeyLike;
    systemProgram: PublicKeyLike & EncodableDefault = new PublicKey(SYSTEM_PROGRAM_ID_STRING);
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<RegisterSettlementPolicyInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, RegisterSettlementPolicyInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): RegisterSettlementPolicyInstruction {
        return Decoder.decode(data, RegisterSettlementPolicyInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, RegisterSettlementPolicyInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, RegisterSettlementPolicyInstruction);
    }

    static getSchema(): Schema {
        return {
            policyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            policyFlags: {
                type: SchemaFieldType.U64
            },
            allowedSettlementModes: {
                type: SchemaFieldType.U32
            },
            allowedAssetMint: {
                type: SchemaFieldType.Address
            },
            allowedPrincipalA: {
                type: SchemaFieldType.Address
            },
            allowedPrincipalB: {
                type: SchemaFieldType.Address
            },
            designatedExecutor: {
                type: SchemaFieldType.Address
            },
            maxNotional: {
                type: SchemaFieldType.U64
            },
            minNotional: {
                type: SchemaFieldType.U64
            },
            validAfterUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            expiresAtUnixTimestamp: {
                type: SchemaFieldType.I64
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: RegisterSettlementPolicyInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true, relations: RegisterSettlementPolicyInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            settlementPolicy: { signer: false, writable: true, pda: RegisterSettlementPolicyInstruction.programDerivedAccounts.settlementPolicy, accountSize: ACCOUNT_SIZES.SETTLEMENT_POLICY, willCreate: true },
            payer: { signer: true, writable: true },
            operationsAuthority: { signer: true, writable: false },
            systemProgram: { signer: false, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
