// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          EnableLegacyMigrationInstruction.ts
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
import type { PublicKeyLike } from "@solomon-labs/types";

import {
    PROGRAM_ID,
    ACCOUNT_SIZES,
    MODULE_ACTIVATION_STATE,
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    MIGRATION_CONFIG,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/migration/instructions/enable_legacy_migration.rs
 */
export class EnableLegacyMigrationInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        moduleActivationState: {
            seeds: [
                { kind: "const", value: [109, 111, 100, 117, 108, 101, 45, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 45, 115, 116, 97, 116, 101] }
            ]
        },
        migrationConfig: {
            seeds: [
                { kind: "const", value: [108, 101, 103, 97, 99, 121, 45, 109, 105, 103, 114, 97, 116, 105, 111, 110] }
            ]
        }
    };

    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["governanceAuthority"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.MIGRATION, IX.migration.ENABLE_LEGACY_MIGRATION]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    legacyProgram!: PublicKeyLike;
    legacyMint!: PublicKeyLike;
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    migrationConfig: PublicKeyLike & EncodableDefault = MIGRATION_CONFIG;
    payer!: PublicKeyLike;
    governanceAuthority!: PublicKeyLike;
    systemProgram: PublicKeyLike & EncodableDefault = new PublicKey(SYSTEM_PROGRAM_ID_STRING);
    legacyMintAccount!: PublicKeyLike;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<EnableLegacyMigrationInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, EnableLegacyMigrationInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): EnableLegacyMigrationInstruction {
        return Decoder.decode(data, EnableLegacyMigrationInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, EnableLegacyMigrationInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, EnableLegacyMigrationInstruction);
    }

    static getSchema(): Schema {
        return {
            legacyProgram: {
                type: SchemaFieldType.Address
            },
            legacyMint: {
                type: SchemaFieldType.Address
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: EnableLegacyMigrationInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true, relations: EnableLegacyMigrationInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            migrationConfig: { signer: false, writable: true, pda: EnableLegacyMigrationInstruction.programDerivedAccounts.migrationConfig },
            payer: { signer: true, writable: true },
            governanceAuthority: { signer: true, writable: false },
            systemProgram: { signer: false, writable: false },
            legacyMintAccount: { signer: false, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
