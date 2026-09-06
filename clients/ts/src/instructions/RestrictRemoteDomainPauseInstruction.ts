// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          RestrictRemoteDomainPauseInstruction.ts
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
import type { BU64, PublicKeyLike, U32, U8 } from "@solomon-labs/types";

import {
    PROGRAM_ID,
    ACCOUNT_SIZES,
    MODULE_ACTIVATION_STATE,
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/cross_chain/instructions/restrict_remote_domain_pause.rs
 */
export class RestrictRemoteDomainPauseInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        moduleActivationState: {
            seeds: [
                { kind: "const", value: [109, 111, 100, 117, 108, 101, 45, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 45, 115, 116, 97, 116, 101] }
            ]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.CROSS_CHAIN, IX.cross_chain.RESTRICT_REMOTE_DOMAIN_PAUSE]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    remoteChainKind!: U8;
    remoteDomainId!: BU64;
    pauseBitsToSet!: BU64;
    reasonCode!: U32;
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    remoteDomainPolicy!: PublicKeyLike;
    authority!: PublicKeyLike;
    authorityPermissionRecord!: PublicKeyLike;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<RestrictRemoteDomainPauseInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, RestrictRemoteDomainPauseInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): RestrictRemoteDomainPauseInstruction {
        return Decoder.decode(data, RestrictRemoteDomainPauseInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, RestrictRemoteDomainPauseInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, RestrictRemoteDomainPauseInstruction);
    }

    static getSchema(): Schema {
        return {
            remoteChainKind: {
                type: SchemaFieldType.U8
            },
            remoteDomainId: {
                type: SchemaFieldType.U64
            },
            pauseBitsToSet: {
                type: SchemaFieldType.U64
            },
            reasonCode: {
                type: SchemaFieldType.U32
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: RestrictRemoteDomainPauseInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true },
            eventAuthority: { signer: false, writable: false },
            remoteDomainPolicy: { signer: false, writable: true },
            authority: { signer: true, writable: false },
            authorityPermissionRecord: { signer: false, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
