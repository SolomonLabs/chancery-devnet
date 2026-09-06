// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          RegisterEvidencePolicyInstruction.ts
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
import type { BU64, PublicKeyLike, U16, U8 } from "@solomon-labs/types";

import {
    PROGRAM_ID,
    ACCOUNT_SIZES,
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/instructions/register_evidence_policy.rs
 */
export class RegisterEvidencePolicyInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        evidencePolicy: {
            seeds: [
                { kind: "const", value: [101, 118, 105, 100, 101, 110, 99, 101, 45, 112, 111, 108, 105, 99, 121] },
                { kind: "arg", path: "evidencePolicyId" }
            ]
        }
    };

    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["operationsAuthority"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.EVIDENCE, IX.evidence.REGISTER_EVIDENCE_POLICY]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    evidencePolicyId!: U8[];
    requiredFieldMask!: BU64[];
    counterpartyReportingSchemaHash!: U8[];
    allowFreeformCounterpartyFields!: boolean;
    maximumFreeformFieldCount!: U16;
    maximumFreeformValueBytes!: U16;
    retentionFlags!: BU64;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    evidencePolicy!: PublicKeyLike;
    payer!: PublicKeyLike;
    operationsAuthority!: PublicKeyLike;
    systemProgram: PublicKeyLike & EncodableDefault = new PublicKey(SYSTEM_PROGRAM_ID_STRING);
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<RegisterEvidencePolicyInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, RegisterEvidencePolicyInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): RegisterEvidencePolicyInstruction {
        return Decoder.decode(data, RegisterEvidencePolicyInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, RegisterEvidencePolicyInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, RegisterEvidencePolicyInstruction);
    }

    static getSchema(): Schema {
        return {
            evidencePolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            requiredFieldMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            counterpartyReportingSchemaHash: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            allowFreeformCounterpartyFields: {
                type: SchemaFieldType.Boolean
            },
            maximumFreeformFieldCount: {
                type: SchemaFieldType.U16
            },
            maximumFreeformValueBytes: {
                type: SchemaFieldType.U16
            },
            retentionFlags: {
                type: SchemaFieldType.U64
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            chanceryConfig: { signer: false, writable: true, relations: RegisterEvidencePolicyInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            evidencePolicy: { signer: false, writable: true, pda: RegisterEvidencePolicyInstruction.programDerivedAccounts.evidencePolicy, accountSize: ACCOUNT_SIZES.EVIDENCE_POLICY, willCreate: true },
            payer: { signer: true, writable: true },
            operationsAuthority: { signer: true, writable: false },
            systemProgram: { signer: false, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
