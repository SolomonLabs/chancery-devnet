// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          InitializeChanceryInstruction.ts
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
import type { PublicKeyLike } from "@solomon-labs/types";

import {
    PROGRAM_ID,
    ACCOUNT_SIZES,
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    PAUSE_STATE,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/core/instructions/initialize_chancery.rs
 */
export class InitializeChanceryInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        chanceryConfig: {
            seeds: [
                { kind: "const", value: [99, 104, 97, 110, 99, 101, 114, 121, 45, 99, 111, 110, 102, 105, 103] }
            ]
        },
        eventAuthority: {
            seeds: [
                { kind: "const", value: [101, 118, 101, 110, 116, 45, 97, 117, 116, 104, 111, 114, 105, 116, 121] }
            ]
        },
        pauseState: {
            seeds: [
                { kind: "const", value: [112, 97, 117, 115, 101, 45, 115, 116, 97, 116, 101] }
            ]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.CORE, IX.core.INITIALIZE_CHANCERY]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    issuedTokenMint!: PublicKeyLike;
    issuedTokenProgram!: PublicKeyLike;
    legacyTokenMint!: PublicKeyLike;
    legacyTokenProgram!: PublicKeyLike;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    payer!: PublicKeyLike;
    governanceAuthority!: PublicKeyLike;
    systemProgram: PublicKeyLike & EncodableDefault = new PublicKey(SYSTEM_PROGRAM_ID_STRING);
    pauseState: PublicKeyLike & EncodableDefault = PAUSE_STATE;
    programAccount!: PublicKeyLike;
    programdataAccount!: PublicKeyLike;
    upgradeAuthority!: PublicKeyLike;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<InitializeChanceryInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, InitializeChanceryInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): InitializeChanceryInstruction {
        return Decoder.decode(data, InitializeChanceryInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, InitializeChanceryInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, InitializeChanceryInstruction);
    }

    static getSchema(): Schema {
        return {
            issuedTokenMint: {
                type: SchemaFieldType.Address
            },
            issuedTokenProgram: {
                type: SchemaFieldType.Address
            },
            legacyTokenMint: {
                type: SchemaFieldType.Address
            },
            legacyTokenProgram: {
                type: SchemaFieldType.Address
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            chanceryConfig: { signer: false, writable: true, pda: InitializeChanceryInstruction.programDerivedAccounts.chanceryConfig, accountSize: ACCOUNT_SIZES.CHANCERY_CONFIG, willCreate: true },
            eventAuthority: { signer: false, writable: false, pda: InitializeChanceryInstruction.programDerivedAccounts.eventAuthority },
            payer: { signer: true, writable: true },
            governanceAuthority: { signer: true, writable: false },
            systemProgram: { signer: false, writable: false },
            pauseState: { signer: false, writable: true, pda: InitializeChanceryInstruction.programDerivedAccounts.pauseState, accountSize: ACCOUNT_SIZES.PAUSE_STATE, willCreate: true },
            programAccount: { signer: false, writable: false },
            programdataAccount: { signer: false, writable: false },
            upgradeAuthority: { signer: true, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
