// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          RegisterRemoteDomainPolicyInstruction.ts
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
    DEFAULT_PUBLIC_KEY,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/cross_chain/instructions/register_remote_domain_policy.rs
 */
export class RegisterRemoteDomainPolicyInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        moduleActivationState: {
            seeds: [
                { kind: "const", value: [109, 111, 100, 117, 108, 101, 45, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 45, 115, 116, 97, 116, 101] }
            ]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.CROSS_CHAIN, IX.cross_chain.REGISTER_REMOTE_DOMAIN_POLICY]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    remoteChainKind!: U8;
    remoteDomainId!: BU64;
    minimumAttestationThreshold!: U8;
    requiredFinalityDepth!: BU64;
    messageExpirySeconds!: BU64;
    perMessageMaximum!: BU64;
    perDayMaximum!: BU64;
    remoteDomainSeparator!: U8[];
    remoteChanceryContract!: U8[];
    remoteIssuedToken!: U8[];
    signerSetId!: U8[];
    mode!: U8;
    remoteAsset!: U8[];
    localAssetMint!: PublicKeyLike;
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    remoteDomainPolicy!: PublicKeyLike;
    payer!: PublicKeyLike;
    authority!: PublicKeyLike;
    authorityPermissionRecord!: PublicKeyLike;
    signerSet!: PublicKeyLike;
    systemProgram: PublicKeyLike & EncodableDefault = new PublicKey(SYSTEM_PROGRAM_ID_STRING);
    remoteNonceAccountIndex!: PublicKeyLike;
    remoteDailyUsageWindow: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<RegisterRemoteDomainPolicyInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, RegisterRemoteDomainPolicyInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): RegisterRemoteDomainPolicyInstruction {
        return Decoder.decode(data, RegisterRemoteDomainPolicyInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, RegisterRemoteDomainPolicyInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, RegisterRemoteDomainPolicyInstruction);
    }

    static getSchema(): Schema {
        return {
            remoteChainKind: {
                type: SchemaFieldType.U8
            },
            remoteDomainId: {
                type: SchemaFieldType.U64
            },
            minimumAttestationThreshold: {
                type: SchemaFieldType.U8
            },
            requiredFinalityDepth: {
                type: SchemaFieldType.U64
            },
            messageExpirySeconds: {
                type: SchemaFieldType.U64
            },
            perMessageMaximum: {
                type: SchemaFieldType.U64
            },
            perDayMaximum: {
                type: SchemaFieldType.U64
            },
            remoteDomainSeparator: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            remoteChanceryContract: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            remoteIssuedToken: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            signerSetId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            mode: {
                type: SchemaFieldType.U8
            },
            remoteAsset: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            localAssetMint: {
                type: SchemaFieldType.Address
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: RegisterRemoteDomainPolicyInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true },
            eventAuthority: { signer: false, writable: false },
            remoteDomainPolicy: { signer: false, writable: true },
            payer: { signer: true, writable: true },
            authority: { signer: true, writable: false },
            authorityPermissionRecord: { signer: false, writable: false },
            signerSet: { signer: false, writable: false },
            systemProgram: { signer: false, writable: false },
            remoteNonceAccountIndex: { signer: false, writable: true },
            remoteDailyUsageWindow: { signer: false, writable: true },
            eventProgram: { signer: false, writable: false }
        };
    }
}
