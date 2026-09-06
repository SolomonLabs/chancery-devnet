// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          FreezeIssuedTokenAccountInstruction.ts
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
import type { BU64, PublicKeyLike, U32 } from "@solomon-labs/types";

import {
    PROGRAM_ID,
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    DEFAULT_PUBLIC_KEY,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/control/instructions/freeze_issued_token_account.rs
 */
export class FreezeIssuedTokenAccountInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        freezeRecord: {
            seeds: [
                { kind: "const", value: [98, 97, 115, 105, 99, 45, 102, 114, 101, 101, 122, 101, 45, 114, 101, 99, 111, 114, 100] },
                { kind: "account", path: "issuedTokenAccount" }
            ]
        }
    };

    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["issuedTokenMint", "issuedTokenProgram", "freezeAuthorityPda"]
        },
        freezeRecord: {
            relations: ["issuedTokenAccount"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.CONTROL, IX.control.FREEZE_ISSUED_TOKEN_ACCOUNT]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    reasonCode!: U32;
    freezeFlags!: BU64;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    activation!: PublicKeyLike;
    freezeRecord!: PublicKeyLike;
    issuedTokenAccount!: PublicKeyLike;
    issuedTokenMint!: PublicKeyLike;
    freezeAuthorityPda!: PublicKeyLike;
    issuedTokenProgram!: PublicKeyLike;
    authority!: PublicKeyLike;
    payer!: PublicKeyLike;
    systemProgram: PublicKeyLike & EncodableDefault = new PublicKey(SYSTEM_PROGRAM_ID_STRING);
    permissionRecord: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<FreezeIssuedTokenAccountInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, FreezeIssuedTokenAccountInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): FreezeIssuedTokenAccountInstruction {
        return Decoder.decode(data, FreezeIssuedTokenAccountInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, FreezeIssuedTokenAccountInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, FreezeIssuedTokenAccountInstruction);
    }

    static getSchema(): Schema {
        return {
            reasonCode: {
                type: SchemaFieldType.U32
            },
            freezeFlags: {
                type: SchemaFieldType.U64
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            chanceryConfig: { signer: false, writable: true, relations: FreezeIssuedTokenAccountInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            activation: { signer: false, writable: false },
            freezeRecord: { signer: false, writable: true, pda: FreezeIssuedTokenAccountInstruction.programDerivedAccounts.freezeRecord, relations: FreezeIssuedTokenAccountInstruction.accountRelations.freezeRecord.relations },
            issuedTokenAccount: { signer: false, writable: true },
            issuedTokenMint: { signer: false, writable: false },
            freezeAuthorityPda: { signer: false, writable: false },
            issuedTokenProgram: { signer: false, writable: false },
            authority: { signer: true, writable: false },
            payer: { signer: true, writable: true },
            systemProgram: { signer: false, writable: false },
            permissionRecord: { signer: false, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
