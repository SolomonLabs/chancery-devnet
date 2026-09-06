// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          RegisterPathwayPolicyInstruction.ts
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
import type { PublicKeyLike, U8 } from "@solomon-labs/types";

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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/pathway/instructions/register_pathway_policy.rs
 */
export class RegisterPathwayPolicyInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        moduleActivationState: {
            seeds: [
                { kind: "const", value: [109, 111, 100, 117, 108, 101, 45, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 45, 115, 116, 97, 116, 101] }
            ]
        },
        pathwayPolicy: {
            seeds: [
                { kind: "const", value: [112, 97, 116, 104, 119, 97, 121, 45, 112, 111, 108, 105, 99, 121] },
                { kind: "arg", path: "pathwayId" }
            ]
        }
    };

    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["operationsAuthority"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.PATHWAY, IX.pathway.REGISTER_PATHWAY_POLICY]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    pathwayId!: U8[];
    pathwayKind!: U8;
    assetMint!: PublicKeyLike;
    issuedTokenMint!: PublicKeyLike;
    designatedExecutor!: PublicKeyLike;
    reserveCompartmentPolicyId!: U8[];
    limitPolicyId!: U8[];
    evidencePolicyId!: U8[];
    feePolicyId!: U8[];
    insurancePolicyId!: U8[];
    assetMintLimitPolicyId!: U8[];
    assetRedeemLimitPolicyId!: U8[];
    counterpartyLimitPolicyId!: U8[];
    executorLimitPolicyId!: U8[];
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    pathwayPolicy!: PublicKeyLike;
    payer!: PublicKeyLike;
    operationsAuthority!: PublicKeyLike;
    systemProgram: PublicKeyLike & EncodableDefault = new PublicKey(SYSTEM_PROGRAM_ID_STRING);
    limitPolicy!: PublicKeyLike;
    evidencePolicy!: PublicKeyLike;
    feePolicy!: PublicKeyLike;
    insurancePolicy!: PublicKeyLike;
    assetMintLimitPolicy!: PublicKeyLike;
    assetRedeemLimitPolicy!: PublicKeyLike;
    counterpartyLimitPolicy!: PublicKeyLike;
    executorLimitPolicy!: PublicKeyLike;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<RegisterPathwayPolicyInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, RegisterPathwayPolicyInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): RegisterPathwayPolicyInstruction {
        return Decoder.decode(data, RegisterPathwayPolicyInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, RegisterPathwayPolicyInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, RegisterPathwayPolicyInstruction);
    }

    static getSchema(): Schema {
        return {
            pathwayId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            pathwayKind: {
                type: SchemaFieldType.U8
            },
            assetMint: {
                type: SchemaFieldType.Address
            },
            issuedTokenMint: {
                type: SchemaFieldType.Address
            },
            designatedExecutor: {
                type: SchemaFieldType.Address
            },
            reserveCompartmentPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            limitPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            evidencePolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            feePolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            insurancePolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            assetMintLimitPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            assetRedeemLimitPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            counterpartyLimitPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            executorLimitPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: RegisterPathwayPolicyInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true, relations: RegisterPathwayPolicyInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            pathwayPolicy: { signer: false, writable: true, pda: RegisterPathwayPolicyInstruction.programDerivedAccounts.pathwayPolicy, accountSize: ACCOUNT_SIZES.PATHWAY_POLICY, willCreate: true },
            payer: { signer: true, writable: true },
            operationsAuthority: { signer: true, writable: false },
            systemProgram: { signer: false, writable: false },
            limitPolicy: { signer: false, writable: false },
            evidencePolicy: { signer: false, writable: false },
            feePolicy: { signer: false, writable: false },
            insurancePolicy: { signer: false, writable: false },
            assetMintLimitPolicy: { signer: false, writable: false },
            assetRedeemLimitPolicy: { signer: false, writable: false },
            counterpartyLimitPolicy: { signer: false, writable: false },
            executorLimitPolicy: { signer: false, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
