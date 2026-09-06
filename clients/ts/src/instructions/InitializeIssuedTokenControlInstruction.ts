// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          InitializeIssuedTokenControlInstruction.ts
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
import type { BU64, PublicKeyLike } from "@solomon-labs/types";

import {
    PROGRAM_ID,
    ACCOUNT_SIZES,
    MODULE_ACTIVATION_STATE,
    ISSUED_TOKEN_CONTROL,
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/issuance/instructions/initialize_issued_token_control.rs
 */
export class InitializeIssuedTokenControlInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        moduleActivationState: {
            seeds: [
                { kind: "const", value: [109, 111, 100, 117, 108, 101, 45, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 45, 115, 116, 97, 116, 101] }
            ]
        },
        issuedTokenControl: {
            seeds: [
                { kind: "const", value: [105, 115, 115, 117, 101, 100, 45, 116, 111, 107, 101, 110, 45, 99, 111, 110, 116, 114, 111, 108] }
            ]
        }
    };

    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["governanceAuthority"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.ISSUED_TOKEN_CONTROL, IX.issued_token_control.INITIALIZE_ISSUED_TOKEN_CONTROL]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    issuedTokenMint!: PublicKeyLike;
    issuedTokenProgram!: PublicKeyLike;
    reservedMintExtensionMask!: BU64[];
    reservedAccountExtensionMask!: BU64[];
    maxExtensionObservationAgeSlots!: BU64;
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    issuedTokenControl: PublicKeyLike & EncodableDefault = ISSUED_TOKEN_CONTROL;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    payer!: PublicKeyLike;
    governanceAuthority!: PublicKeyLike;
    systemProgram: PublicKeyLike & EncodableDefault = new PublicKey(SYSTEM_PROGRAM_ID_STRING);
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<InitializeIssuedTokenControlInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, InitializeIssuedTokenControlInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): InitializeIssuedTokenControlInstruction {
        return Decoder.decode(data, InitializeIssuedTokenControlInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, InitializeIssuedTokenControlInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, InitializeIssuedTokenControlInstruction);
    }

    static getSchema(): Schema {
        return {
            issuedTokenMint: {
                type: SchemaFieldType.Address
            },
            issuedTokenProgram: {
                type: SchemaFieldType.Address
            },
            reservedMintExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            reservedAccountExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            maxExtensionObservationAgeSlots: {
                type: SchemaFieldType.U64
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: InitializeIssuedTokenControlInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            issuedTokenControl: { signer: false, writable: true, pda: InitializeIssuedTokenControlInstruction.programDerivedAccounts.issuedTokenControl, accountSize: ACCOUNT_SIZES.ISSUED_TOKEN_CONTROL, willCreate: true },
            chanceryConfig: { signer: false, writable: true, relations: InitializeIssuedTokenControlInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            payer: { signer: true, writable: true },
            governanceAuthority: { signer: true, writable: false },
            systemProgram: { signer: false, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
