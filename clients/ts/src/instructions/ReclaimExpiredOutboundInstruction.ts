// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          ReclaimExpiredOutboundInstruction.ts
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
import type { BI64, BU128, BU64, PublicKeyLike, U8 } from "@solomon-labs/types";

import {
    PROGRAM_ID,
    ACCOUNT_SIZES,
    MODULE_ACTIVATION_STATE,
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    PAUSE_STATE,
    ISSUED_TOKEN_CONTROL,
    MODULE,
    IX,
} from "../constants";
import { AttestationSignature } from "../types/AttestationSignature";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/cross_chain/instructions/reclaim_expired_outbound.rs
 */
export class ReclaimExpiredOutboundInstruction implements IsEncodable {
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
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.CROSS_CHAIN, IX.cross_chain.RECLAIM_EXPIRED_OUTBOUND]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    pathwayId!: U8[];
    remoteChainKind!: U8;
    remoteDomainId!: BU64;
    messageKind!: U8;
    sourceNonce!: BU64;
    amount!: BU128;
    expiresAtUnixTimestamp!: BI64;
    destinationAsset!: U8[];
    sender!: U8[];
    recipient!: U8[];
    retirementReason!: U8;
    expiredAtUnixTimestamp!: BI64;
    expiredAtSlotOrBlock!: BU64;
    providedEpochFreeContentHash!: U8[];
    providedReclaimDigest!: U8[];
    signatures!: AttestationSignature[];
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    pauseState: PublicKeyLike & EncodableDefault = PAUSE_STATE;
    assetPauseState!: PublicKeyLike;
    pathwayPolicy!: PublicKeyLike;
    remoteDomainPolicy!: PublicKeyLike;
    signerSet!: PublicKeyLike;
    remoteNonceAccountIndex!: PublicKeyLike;
    outboundReclaimRecord!: PublicKeyLike;
    senderIssuedTokenAccount!: PublicKeyLike;
    issuedTokenMint!: PublicKeyLike;
    mintAuthorityPda!: PublicKeyLike;
    issuedTokenProgram!: PublicKeyLike;
    issuedTokenControl: PublicKeyLike & EncodableDefault = ISSUED_TOKEN_CONTROL;
    payer!: PublicKeyLike;
    systemProgram: PublicKeyLike & EncodableDefault = new PublicKey(SYSTEM_PROGRAM_ID_STRING);
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<ReclaimExpiredOutboundInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, ReclaimExpiredOutboundInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): ReclaimExpiredOutboundInstruction {
        return Decoder.decode(data, ReclaimExpiredOutboundInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, ReclaimExpiredOutboundInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, ReclaimExpiredOutboundInstruction);
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
            messageKind: {
                type: SchemaFieldType.U8
            },
            sourceNonce: {
                type: SchemaFieldType.U64
            },
            amount: {
                type: SchemaFieldType.U128
            },
            expiresAtUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            destinationAsset: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            sender: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            recipient: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            retirementReason: {
                type: SchemaFieldType.U8
            },
            expiredAtUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            expiredAtSlotOrBlock: {
                type: SchemaFieldType.U64
            },
            providedEpochFreeContentHash: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            providedReclaimDigest: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            signatures: {
                type: SchemaFieldType.Vector,
                coder: AttestationSignature
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: ReclaimExpiredOutboundInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true, relations: ReclaimExpiredOutboundInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            pauseState: { signer: false, writable: false },
            assetPauseState: { signer: false, writable: false },
            pathwayPolicy: { signer: false, writable: false },
            remoteDomainPolicy: { signer: false, writable: false },
            signerSet: { signer: false, writable: false },
            remoteNonceAccountIndex: { signer: false, writable: false },
            outboundReclaimRecord: { signer: false, writable: true },
            senderIssuedTokenAccount: { signer: false, writable: true },
            issuedTokenMint: { signer: false, writable: true },
            mintAuthorityPda: { signer: false, writable: false },
            issuedTokenProgram: { signer: false, writable: false },
            issuedTokenControl: { signer: false, writable: false },
            payer: { signer: true, writable: true },
            systemProgram: { signer: false, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
