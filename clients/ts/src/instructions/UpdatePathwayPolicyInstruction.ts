// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          UpdatePathwayPolicyInstruction.ts
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
    MODULE_ACTIVATION_STATE,
    CHANCERY_CONFIG,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/pathway/instructions/update_pathway_policy.rs
 */
export class UpdatePathwayPolicyInstruction implements IsEncodable {
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

    static readonly discriminator = new Uint8Array([MODULE.PATHWAY, IX.pathway.UPDATE_PATHWAY_POLICY]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    pathwayId!: U8[];
    designatedExecutor!: PublicKeyLike | null | undefined;
    limitPolicyId!: U8[] | null | undefined;
    evidencePolicyId!: U8[] | null | undefined;
    feePolicyId!: U8[] | null | undefined;
    insurancePolicyId!: U8[] | null | undefined;
    forbiddenCollateralExtensionMask!: BU64[] | null | undefined;
    assetMintLimitPolicyId!: U8[] | null | undefined;
    assetRedeemLimitPolicyId!: U8[] | null | undefined;
    counterpartyLimitPolicyId!: U8[] | null | undefined;
    executorLimitPolicyId!: U8[] | null | undefined;
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    pathwayPolicy!: PublicKeyLike;
    operationsAuthority!: PublicKeyLike;
    eventAuthorityDirect!: PublicKeyLike;
    limitPolicy!: PublicKeyLike;
    evidencePolicy!: PublicKeyLike;
    feePolicy!: PublicKeyLike;
    insurancePolicy!: PublicKeyLike;
    assetMintLimitPolicy!: PublicKeyLike;
    assetRedeemLimitPolicy!: PublicKeyLike;
    counterpartyLimitPolicy!: PublicKeyLike;
    executorLimitPolicy!: PublicKeyLike;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<UpdatePathwayPolicyInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, UpdatePathwayPolicyInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): UpdatePathwayPolicyInstruction {
        return Decoder.decode(data, UpdatePathwayPolicyInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, UpdatePathwayPolicyInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, UpdatePathwayPolicyInstruction);
    }

    static getSchema(): Schema {
        return {
            pathwayId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            designatedExecutor: {
                type: SchemaFieldType.Address,
                optional: true
            },
            limitPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8,
                optional: true
            },
            evidencePolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8,
                optional: true
            },
            feePolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8,
                optional: true
            },
            insurancePolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8,
                optional: true
            },
            forbiddenCollateralExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64,
                optional: true
            },
            assetMintLimitPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8,
                optional: true
            },
            assetRedeemLimitPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8,
                optional: true
            },
            counterpartyLimitPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8,
                optional: true
            },
            executorLimitPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8,
                optional: true
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: UpdatePathwayPolicyInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true, relations: UpdatePathwayPolicyInstruction.accountRelations.chanceryConfig.relations },
            pathwayPolicy: { signer: false, writable: true },
            operationsAuthority: { signer: true, writable: false },
            eventAuthorityDirect: { signer: false, writable: false },
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
