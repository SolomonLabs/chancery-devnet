// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          UpdateEvidencePolicyWithPendingChangeInstruction.ts
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
    Schema
} from "@solomon-labs/solana-codec";
import { Decoder, Encoder, SchemaFieldType } from "@solomon-labs/solana-codec";
import type { BU64, PublicKeyLike, U16, U8 } from "@solomon-labs/types";

import { PROGRAM_ID, CHANCERY_CONFIG, EVENT_AUTHORITY, MODULE, IX } from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/instructions/update_evidence_policy_with_pending_change.rs
 */
export class UpdateEvidencePolicyWithPendingChangeInstruction implements IsEncodable {
    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["governanceAuthority"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.EVIDENCE, IX.evidence.UPDATE_EVIDENCE_POLICY_WITH_PENDING_CHANGE]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    evidencePolicyId!: U8[];
    requiredFieldMask!: BU64[] | null | undefined;
    counterpartyReportingSchemaHash!: U8[] | null | undefined;
    allowFreeformCounterpartyFields!: boolean | null | undefined;
    maximumFreeformFieldCount!: U16 | null | undefined;
    maximumFreeformValueBytes!: U16 | null | undefined;
    retentionFlags!: BU64 | null | undefined;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    pendingConfigChange!: PublicKeyLike;
    evidencePolicy!: PublicKeyLike;
    governanceAuthority!: PublicKeyLike;
    rentRefundRecipient!: PublicKeyLike;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<UpdateEvidencePolicyWithPendingChangeInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, UpdateEvidencePolicyWithPendingChangeInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): UpdateEvidencePolicyWithPendingChangeInstruction {
        return Decoder.decode(data, UpdateEvidencePolicyWithPendingChangeInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, UpdateEvidencePolicyWithPendingChangeInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, UpdateEvidencePolicyWithPendingChangeInstruction);
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
                codableType: SchemaFieldType.U64,
                optional: true
            },
            counterpartyReportingSchemaHash: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8,
                optional: true
            },
            allowFreeformCounterpartyFields: {
                type: SchemaFieldType.Boolean,
                optional: true
            },
            maximumFreeformFieldCount: {
                type: SchemaFieldType.U16,
                optional: true
            },
            maximumFreeformValueBytes: {
                type: SchemaFieldType.U16,
                optional: true
            },
            retentionFlags: {
                type: SchemaFieldType.U64,
                optional: true
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            chanceryConfig: { signer: false, writable: true, relations: UpdateEvidencePolicyWithPendingChangeInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            pendingConfigChange: { signer: false, writable: true },
            evidencePolicy: { signer: false, writable: true },
            governanceAuthority: { signer: true, writable: false },
            rentRefundRecipient: { signer: false, writable: true },
            eventProgram: { signer: false, writable: false }
        };
    }
}
