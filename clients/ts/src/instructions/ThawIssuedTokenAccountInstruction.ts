// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          ThawIssuedTokenAccountInstruction.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/control/instructions/thaw_issued_token_account.rs
 */
export class ThawIssuedTokenAccountInstruction implements IsEncodable {
    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["issuedTokenMint", "issuedTokenProgram", "freezeAuthorityPda"]
        },
        freezeRecord: {
            relations: ["issuedTokenAccount"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.CONTROL, IX.control.THAW_ISSUED_TOKEN_ACCOUNT]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    reasonCode!: U32;
    thawFlags!: BU64;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    activation!: PublicKeyLike;
    freezeRecord!: PublicKeyLike;
    issuedTokenAccount!: PublicKeyLike;
    issuedTokenMint!: PublicKeyLike;
    freezeAuthorityPda!: PublicKeyLike;
    issuedTokenProgram!: PublicKeyLike;
    authority!: PublicKeyLike;
    permissionRecord: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    rentRefundRecipient: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<ThawIssuedTokenAccountInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, ThawIssuedTokenAccountInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): ThawIssuedTokenAccountInstruction {
        return Decoder.decode(data, ThawIssuedTokenAccountInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, ThawIssuedTokenAccountInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, ThawIssuedTokenAccountInstruction);
    }

    static getSchema(): Schema {
        return {
            reasonCode: {
                type: SchemaFieldType.U32
            },
            thawFlags: {
                type: SchemaFieldType.U64
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            chanceryConfig: { signer: false, writable: true, relations: ThawIssuedTokenAccountInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            activation: { signer: false, writable: false },
            freezeRecord: { signer: false, writable: true, relations: ThawIssuedTokenAccountInstruction.accountRelations.freezeRecord.relations },
            issuedTokenAccount: { signer: false, writable: true },
            issuedTokenMint: { signer: false, writable: false },
            freezeAuthorityPda: { signer: false, writable: false },
            issuedTokenProgram: { signer: false, writable: false },
            authority: { signer: true, writable: false },
            permissionRecord: { signer: false, writable: false },
            rentRefundRecipient: { signer: false, writable: true },
            eventProgram: { signer: false, writable: false }
        };
    }
}
