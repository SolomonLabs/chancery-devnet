// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          WithdrawReserveInstruction.ts
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
import type { BU64, PublicKeyLike } from "@solomon-labs/types";

import {
    PROGRAM_ID,
    ACCOUNT_SIZES,
    MODULE_ACTIVATION_STATE,
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    PAUSE_STATE,
    RESERVE_AUTHORITY_PDA,
    DEFAULT_PUBLIC_KEY,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/reserve/instructions/withdraw_reserve.rs
 */
export class WithdrawReserveInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        moduleActivationState: {
            seeds: [
                { kind: "const", value: [109, 111, 100, 117, 108, 101, 45, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 45, 115, 116, 97, 116, 101] }
            ]
        },
        reserveAuthorityPda: {
            seeds: [
                { kind: "const", value: [114, 101, 115, 101, 114, 118, 101, 45, 97, 117, 116, 104, 111, 114, 105, 116, 121] }
            ]
        }
    };

    static readonly accountRelations: AccountRelationsSchema = {
        assetConfig: {
            relations: ["assetMint", "assetTokenProgram"]
        },
        assetPauseState: {
            relations: ["assetMint"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.RESERVE, IX.reserve.WITHDRAW_RESERVE]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    amount!: BU64;
    minimumDestinationAmount!: BU64;
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    pauseState: PublicKeyLike & EncodableDefault = PAUSE_STATE;
    assetConfig!: PublicKeyLike;
    reserveDestination!: PublicKeyLike;
    reserveAssetTokenAccount!: PublicKeyLike;
    destinationAssetTokenAccount!: PublicKeyLike;
    assetMint!: PublicKeyLike;
    reserveAuthorityPda: PublicKeyLike & EncodableDefault = RESERVE_AUTHORITY_PDA;
    assetTokenProgram!: PublicKeyLike;
    authority!: PublicKeyLike;
    assetPauseState!: PublicKeyLike;
    withdrawalLimitPolicy!: PublicKeyLike;
    withdrawalDailyUsageWindow!: PublicKeyLike;
    permissionRecord: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<WithdrawReserveInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, WithdrawReserveInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): WithdrawReserveInstruction {
        return Decoder.decode(data, WithdrawReserveInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, WithdrawReserveInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, WithdrawReserveInstruction);
    }

    static getSchema(): Schema {
        return {
            amount: {
                type: SchemaFieldType.U64
            },
            minimumDestinationAmount: {
                type: SchemaFieldType.U64
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: WithdrawReserveInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true },
            eventAuthority: { signer: false, writable: false },
            pauseState: { signer: false, writable: false },
            assetConfig: { signer: false, writable: false, relations: WithdrawReserveInstruction.accountRelations.assetConfig.relations },
            reserveDestination: { signer: false, writable: false },
            reserveAssetTokenAccount: { signer: false, writable: true },
            destinationAssetTokenAccount: { signer: false, writable: true },
            assetMint: { signer: false, writable: false },
            reserveAuthorityPda: { signer: false, writable: false, pda: WithdrawReserveInstruction.programDerivedAccounts.reserveAuthorityPda },
            assetTokenProgram: { signer: false, writable: false },
            authority: { signer: true, writable: false },
            assetPauseState: { signer: false, writable: false, relations: WithdrawReserveInstruction.accountRelations.assetPauseState.relations },
            withdrawalLimitPolicy: { signer: false, writable: false },
            withdrawalDailyUsageWindow: { signer: false, writable: true },
            permissionRecord: { signer: false, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
