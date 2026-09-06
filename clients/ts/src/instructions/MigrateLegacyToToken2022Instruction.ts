// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          MigrateLegacyToToken2022Instruction.ts
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
    MIGRATION_CONFIG,
    ISSUED_TOKEN_CONTROL,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/migration/instructions/migrate_legacy_to_token2022.rs
 */
export class MigrateLegacyToToken2022Instruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        moduleActivationState: {
            seeds: [
                { kind: "const", value: [109, 111, 100, 117, 108, 101, 45, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 45, 115, 116, 97, 116, 101] }
            ]
        }
    };

    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["issuedTokenMint", "legacyTokenProgram", "issuedTokenProgram"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.MIGRATION, IX.migration.MIGRATE_LEGACY_TO_TOKEN2022]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    amount!: BU64;
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    pauseState: PublicKeyLike & EncodableDefault = PAUSE_STATE;
    migrationConfig: PublicKeyLike & EncodableDefault = MIGRATION_CONFIG;
    sourceLegacyTokenAccount!: PublicKeyLike;
    destinationIssuedTokenAccount!: PublicKeyLike;
    legacyMint!: PublicKeyLike;
    issuedTokenMint!: PublicKeyLike;
    mintAuthorityPda!: PublicKeyLike;
    legacyTokenProgram!: PublicKeyLike;
    issuedTokenProgram!: PublicKeyLike;
    holder!: PublicKeyLike;
    permissionRecord!: PublicKeyLike;
    issuedTokenControl: PublicKeyLike & EncodableDefault = ISSUED_TOKEN_CONTROL;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<MigrateLegacyToToken2022Instruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, MigrateLegacyToToken2022Instruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): MigrateLegacyToToken2022Instruction {
        return Decoder.decode(data, MigrateLegacyToToken2022Instruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, MigrateLegacyToToken2022Instruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, MigrateLegacyToToken2022Instruction);
    }

    static getSchema(): Schema {
        return {
            amount: {
                type: SchemaFieldType.U64
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: MigrateLegacyToToken2022Instruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true, relations: MigrateLegacyToToken2022Instruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            pauseState: { signer: false, writable: false },
            migrationConfig: { signer: false, writable: true },
            sourceLegacyTokenAccount: { signer: false, writable: true },
            destinationIssuedTokenAccount: { signer: false, writable: true },
            legacyMint: { signer: false, writable: true },
            issuedTokenMint: { signer: false, writable: true },
            mintAuthorityPda: { signer: false, writable: false },
            legacyTokenProgram: { signer: false, writable: false },
            issuedTokenProgram: { signer: false, writable: false },
            holder: { signer: true, writable: false },
            permissionRecord: { signer: false, writable: false },
            issuedTokenControl: { signer: false, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
