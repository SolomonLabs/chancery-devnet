// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          UpdateFeePolicyWithPendingChangeInstruction.ts
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
import type { BI64, BU64, PublicKeyLike, U32, U8 } from "@solomon-labs/types";

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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/fees/instructions/update_fee_policy_with_pending_change.rs
 */
export class UpdateFeePolicyWithPendingChangeInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        moduleActivationState: {
            seeds: [
                { kind: "const", value: [109, 111, 100, 117, 108, 101, 45, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 45, 115, 116, 97, 116, 101] }
            ]
        }
    };

    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["governanceAuthority"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.FEES, IX.fees.UPDATE_FEE_POLICY_WITH_PENDING_CHANGE]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    feePolicyId!: U8[];
    feePolicyFlags!: BU64 | null | undefined;
    flatFeeInAsset!: BU64 | null | undefined;
    flatFeeInIssuedToken!: BU64 | null | undefined;
    percentFeeBps!: U32 | null | undefined;
    feeCapAmount!: BU64 | null | undefined;
    minimumFeeAmount!: BU64 | null | undefined;
    rebateFlatAmount!: BU64 | null | undefined;
    rebateBps!: U32 | null | undefined;
    rebateCapAmount!: BU64 | null | undefined;
    netFeeFloorZero!: boolean | null | undefined;
    feeRecipientKey!: PublicKeyLike | null | undefined;
    effectiveFromUnixTimestamp!: BI64 | null | undefined;
    effectiveUntilUnixTimestamp!: BI64 | null | undefined;
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    pendingConfigChange!: PublicKeyLike;
    feePolicy!: PublicKeyLike;
    governanceAuthority!: PublicKeyLike;
    rentRefundRecipient: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<UpdateFeePolicyWithPendingChangeInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, UpdateFeePolicyWithPendingChangeInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): UpdateFeePolicyWithPendingChangeInstruction {
        return Decoder.decode(data, UpdateFeePolicyWithPendingChangeInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, UpdateFeePolicyWithPendingChangeInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, UpdateFeePolicyWithPendingChangeInstruction);
    }

    static getSchema(): Schema {
        return {
            feePolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            feePolicyFlags: {
                type: SchemaFieldType.U64,
                optional: true
            },
            flatFeeInAsset: {
                type: SchemaFieldType.U64,
                optional: true
            },
            flatFeeInIssuedToken: {
                type: SchemaFieldType.U64,
                optional: true
            },
            percentFeeBps: {
                type: SchemaFieldType.U32,
                optional: true
            },
            feeCapAmount: {
                type: SchemaFieldType.U64,
                optional: true
            },
            minimumFeeAmount: {
                type: SchemaFieldType.U64,
                optional: true
            },
            rebateFlatAmount: {
                type: SchemaFieldType.U64,
                optional: true
            },
            rebateBps: {
                type: SchemaFieldType.U32,
                optional: true
            },
            rebateCapAmount: {
                type: SchemaFieldType.U64,
                optional: true
            },
            netFeeFloorZero: {
                type: SchemaFieldType.Boolean,
                optional: true
            },
            feeRecipientKey: {
                type: SchemaFieldType.Address,
                optional: true
            },
            effectiveFromUnixTimestamp: {
                type: SchemaFieldType.I64,
                optional: true
            },
            effectiveUntilUnixTimestamp: {
                type: SchemaFieldType.I64,
                optional: true
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: UpdateFeePolicyWithPendingChangeInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true, relations: UpdateFeePolicyWithPendingChangeInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            pendingConfigChange: { signer: false, writable: true },
            feePolicy: { signer: false, writable: true },
            governanceAuthority: { signer: true, writable: false },
            rentRefundRecipient: { signer: false, writable: true },
            eventProgram: { signer: false, writable: false }
        };
    }
}
