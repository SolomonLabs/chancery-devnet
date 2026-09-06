// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          MintDirectInstruction.ts
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
    EVENT_AUTHORITY,
    PAUSE_STATE,
    ISSUED_TOKEN_CONTROL,
    DEFAULT_PUBLIC_KEY,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/settlement/instructions/mint_direct.rs
 */
export class MintDirectInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        moduleActivationState: {
            seeds: [
                { kind: "const", value: [109, 111, 100, 117, 108, 101, 45, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 45, 115, 116, 97, 116, 101] }
            ]
        }
    };

    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["issuedTokenProgram", "mintAuthorityPda"]
        },
        assetConfig: {
            relations: ["assetMint", "assetTokenProgram"]
        },
        pathwayPolicy: {
            relations: ["assetMint"]
        },
        assetPauseState: {
            relations: ["assetMint"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.SETTLEMENT, IX.settlement.MINT_DIRECT]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    pathwayId!: U8[];
    assetAmount!: BU64;
    minimumIssuedTokenAmount!: BU64;
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    pauseState: PublicKeyLike & EncodableDefault = PAUSE_STATE;
    assetConfig!: PublicKeyLike;
    pathwayPolicy!: PublicKeyLike;
    permissionRecord!: PublicKeyLike;
    sourceAssetTokenAccount!: PublicKeyLike;
    reserveAssetTokenAccount!: PublicKeyLike;
    destinationIssuedTokenAccount!: PublicKeyLike;
    assetMint!: PublicKeyLike;
    issuedTokenMint!: PublicKeyLike;
    mintAuthorityPda!: PublicKeyLike;
    assetTokenProgram!: PublicKeyLike;
    issuedTokenProgram!: PublicKeyLike;
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
    assetLimitPolicy: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    assetDailyUsageWindow: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    counterpartyLimitPolicy: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    counterpartyDailyUsageWindow: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<MintDirectInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, MintDirectInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): MintDirectInstruction {
        return Decoder.decode(data, MintDirectInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, MintDirectInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, MintDirectInstruction);
    }

    static getSchema(): Schema {
        return {
            pathwayId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            assetAmount: {
                type: SchemaFieldType.U64
            },
            minimumIssuedTokenAmount: {
                type: SchemaFieldType.U64
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: MintDirectInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true, relations: MintDirectInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            pauseState: { signer: false, writable: false },
            assetConfig: { signer: false, writable: false, relations: MintDirectInstruction.accountRelations.assetConfig.relations },
            pathwayPolicy: { signer: false, writable: false, relations: MintDirectInstruction.accountRelations.pathwayPolicy.relations },
            permissionRecord: { signer: false, writable: false },
            sourceAssetTokenAccount: { signer: false, writable: true },
            reserveAssetTokenAccount: { signer: false, writable: true },
            destinationIssuedTokenAccount: { signer: false, writable: true },
            assetMint: { signer: false, writable: false },
            issuedTokenMint: { signer: false, writable: true },
            mintAuthorityPda: { signer: false, writable: false },
            assetTokenProgram: { signer: false, writable: false },
            issuedTokenProgram: { signer: false, writable: false },
            principal: { signer: true, writable: false },
            assetPauseState: { signer: false, writable: false, relations: MintDirectInstruction.accountRelations.assetPauseState.relations },
            issuedTokenControl: { signer: false, writable: false },
            feePolicy: { signer: false, writable: false },
            feeRecipientTokenAccount: { signer: false, writable: true },
            limitPolicy: { signer: false, writable: false },
            hourlyUsageWindow: { signer: false, writable: true },
            dailyUsageWindow: { signer: false, writable: true },
            weeklyUsageWindow: { signer: false, writable: true },
            monthlyUsageWindow: { signer: false, writable: true },
            evidencePolicy: { signer: false, writable: false },
            assetLimitPolicy: { signer: false, writable: false },
            assetDailyUsageWindow: { signer: false, writable: true },
            counterpartyLimitPolicy: { signer: false, writable: false },
            counterpartyDailyUsageWindow: { signer: false, writable: true },
            eventProgram: { signer: false, writable: false }
        };
    }
}
