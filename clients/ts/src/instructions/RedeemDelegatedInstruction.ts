// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          RedeemDelegatedInstruction.ts
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
import type { PublicKeyLike, U8 } from "@solomon-labs/types";

import {
    PROGRAM_ID,
    ACCOUNT_SIZES,
    MODULE_ACTIVATION_STATE,
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    PAUSE_STATE,
    RESERVE_AUTHORITY_PDA,
    ISSUED_TOKEN_CONTROL,
    DEFAULT_PUBLIC_KEY,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/settlement/instructions/redeem_delegated.rs
 */
export class RedeemDelegatedInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        moduleActivationState: {
            seeds: [
                { kind: "const", value: [109, 111, 100, 117, 108, 101, 45, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 45, 115, 116, 97, 116, 101] }
            ]
        }
    };

    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["issuedTokenProgram"]
        },
        assetConfig: {
            relations: ["assetMint", "assetTokenProgram"]
        },
        pathwayPolicy: {
            relations: ["assetMint"]
        },
        intent: {
            relations: ["assetMint", "issuedTokenMint", "executor"]
        },
        assetPauseState: {
            relations: ["assetMint"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.SETTLEMENT, IX.settlement.REDEEM_DELEGATED]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    intentId!: U8[];
    pathwayId!: U8[];
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    pauseState: PublicKeyLike & EncodableDefault = PAUSE_STATE;
    assetConfig!: PublicKeyLike;
    pathwayPolicy!: PublicKeyLike;
    intent!: PublicKeyLike;
    principalPermissionRecord!: PublicKeyLike;
    executorPermissionRecord!: PublicKeyLike;
    sourceIssuedTokenAccount!: PublicKeyLike;
    reserveAssetTokenAccount!: PublicKeyLike;
    destinationAssetTokenAccount!: PublicKeyLike;
    assetMint!: PublicKeyLike;
    issuedTokenMint!: PublicKeyLike;
    reserveAuthorityPda: PublicKeyLike & EncodableDefault = RESERVE_AUTHORITY_PDA;
    assetTokenProgram!: PublicKeyLike;
    issuedTokenProgram!: PublicKeyLike;
    executor!: PublicKeyLike;
    principal!: PublicKeyLike;
    assetPauseState!: PublicKeyLike;
    issuedTokenControl: PublicKeyLike & EncodableDefault = ISSUED_TOKEN_CONTROL;
    feePolicy: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    feeRecipientTokenAccount: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    limitPolicy: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    hourlyUsageWindow: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    dailyUsageWindow: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    weeklyUsageWindow: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    monthlyUsageWindow: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    evidencePolicy: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    settlementPolicy: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    assetLimitPolicy: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    assetDailyUsageWindow: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    counterpartyLimitPolicy: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    counterpartyDailyUsageWindow: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    executorLimitPolicy: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    executorDailyUsageWindow: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    rentRefundRecipient: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<RedeemDelegatedInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, RedeemDelegatedInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): RedeemDelegatedInstruction {
        return Decoder.decode(data, RedeemDelegatedInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, RedeemDelegatedInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, RedeemDelegatedInstruction);
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
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: RedeemDelegatedInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true, relations: RedeemDelegatedInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            pauseState: { signer: false, writable: false },
            assetConfig: { signer: false, writable: false, relations: RedeemDelegatedInstruction.accountRelations.assetConfig.relations },
            pathwayPolicy: { signer: false, writable: false, relations: RedeemDelegatedInstruction.accountRelations.pathwayPolicy.relations },
            intent: { signer: false, writable: true, relations: RedeemDelegatedInstruction.accountRelations.intent.relations },
            principalPermissionRecord: { signer: false, writable: false },
            executorPermissionRecord: { signer: false, writable: false },
            sourceIssuedTokenAccount: { signer: false, writable: true },
            reserveAssetTokenAccount: { signer: false, writable: true },
            destinationAssetTokenAccount: { signer: false, writable: true },
            assetMint: { signer: false, writable: false },
            issuedTokenMint: { signer: false, writable: true },
            reserveAuthorityPda: { signer: false, writable: false },
            assetTokenProgram: { signer: false, writable: false },
            issuedTokenProgram: { signer: false, writable: false },
            executor: { signer: true, writable: false },
            principal: { signer: false, writable: false },
            assetPauseState: { signer: false, writable: false, relations: RedeemDelegatedInstruction.accountRelations.assetPauseState.relations },
            issuedTokenControl: { signer: false, writable: false },
            feePolicy: { signer: false, writable: false },
            feeRecipientTokenAccount: { signer: false, writable: true },
            limitPolicy: { signer: false, writable: false },
            hourlyUsageWindow: { signer: false, writable: true },
            dailyUsageWindow: { signer: false, writable: true },
            weeklyUsageWindow: { signer: false, writable: true },
            monthlyUsageWindow: { signer: false, writable: true },
            evidencePolicy: { signer: false, writable: false },
            settlementPolicy: { signer: false, writable: false },
            assetLimitPolicy: { signer: false, writable: false },
            assetDailyUsageWindow: { signer: false, writable: true },
            counterpartyLimitPolicy: { signer: false, writable: false },
            counterpartyDailyUsageWindow: { signer: false, writable: true },
            executorLimitPolicy: { signer: false, writable: false },
            executorDailyUsageWindow: { signer: false, writable: true },
            rentRefundRecipient: { signer: false, writable: true },
            eventProgram: { signer: false, writable: false }
        };
    }
}
