// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          EmitOutboundMessageInstruction.ts
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
import type { BI64, BU64, PublicKeyLike, U8 } from "@solomon-labs/types";

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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/cross_chain/instructions/emit_outbound_message.rs
 */
export class EmitOutboundMessageInstruction implements IsEncodable {
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
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.CROSS_CHAIN, IX.cross_chain.EMIT_OUTBOUND_MESSAGE]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    pathwayId!: U8[];
    remoteChainKind!: U8;
    remoteDomainId!: BU64;
    amount!: BU64;
    minimumRemoteAmount!: BU64;
    recipient!: U8[];
    destinationAsset!: U8[];
    messageKind!: U8;
    expiresAtUnixTimestamp!: BI64;
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    pauseState: PublicKeyLike & EncodableDefault = PAUSE_STATE;
    assetPauseState!: PublicKeyLike;
    pathwayPolicy!: PublicKeyLike;
    remoteDomainPolicy!: PublicKeyLike;
    signerSet!: PublicKeyLike;
    remoteNonceAccountIndex!: PublicKeyLike;
    sourceIssuedTokenAccount!: PublicKeyLike;
    issuedTokenMint!: PublicKeyLike;
    issuedTokenProgram!: PublicKeyLike;
    sender!: PublicKeyLike;
    issuedTokenControl: PublicKeyLike & EncodableDefault = ISSUED_TOKEN_CONTROL;
    senderPermissionRecord!: PublicKeyLike;
    assetConfig!: PublicKeyLike;
    feePolicy: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    feeRecipientTokenAccount: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    mintAuthorityPda: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    limitPolicy: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    hourlyUsageWindow: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    dailyUsageWindow: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    remoteDomainDailyUsageWindow: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    weeklyUsageWindow: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    monthlyUsageWindow: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<EmitOutboundMessageInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, EmitOutboundMessageInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): EmitOutboundMessageInstruction {
        return Decoder.decode(data, EmitOutboundMessageInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, EmitOutboundMessageInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, EmitOutboundMessageInstruction);
    }

    static getSchema(): Schema {
        return {
            pathwayId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            remoteChainKind: {
                type: SchemaFieldType.U8
            },
            remoteDomainId: {
                type: SchemaFieldType.U64
            },
            amount: {
                type: SchemaFieldType.U64
            },
            minimumRemoteAmount: {
                type: SchemaFieldType.U64
            },
            recipient: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            destinationAsset: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            messageKind: {
                type: SchemaFieldType.U8
            },
            expiresAtUnixTimestamp: {
                type: SchemaFieldType.I64
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: EmitOutboundMessageInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true, relations: EmitOutboundMessageInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            pauseState: { signer: false, writable: false },
            assetPauseState: { signer: false, writable: false },
            pathwayPolicy: { signer: false, writable: false },
            remoteDomainPolicy: { signer: false, writable: false },
            signerSet: { signer: false, writable: false },
            remoteNonceAccountIndex: { signer: false, writable: true },
            sourceIssuedTokenAccount: { signer: false, writable: true },
            issuedTokenMint: { signer: false, writable: true },
            issuedTokenProgram: { signer: false, writable: false },
            sender: { signer: true, writable: false },
            issuedTokenControl: { signer: false, writable: false },
            senderPermissionRecord: { signer: false, writable: false },
            assetConfig: { signer: false, writable: false },
            feePolicy: { signer: false, writable: false },
            feeRecipientTokenAccount: { signer: false, writable: true },
            mintAuthorityPda: { signer: false, writable: false },
            limitPolicy: { signer: false, writable: false },
            hourlyUsageWindow: { signer: false, writable: true },
            dailyUsageWindow: { signer: false, writable: true },
            remoteDomainDailyUsageWindow: { signer: false, writable: true },
            weeklyUsageWindow: { signer: false, writable: true },
            monthlyUsageWindow: { signer: false, writable: true },
            eventProgram: { signer: false, writable: false }
        };
    }
}
