// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          ProposeAuthorityTransferInstruction.ts
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
import type { BU64, PublicKeyLike, U8 } from "@solomon-labs/types";

import {
    PROGRAM_ID,
    ACCOUNT_SIZES,
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/core/instructions/propose_authority_transfer.rs
 */
export class ProposeAuthorityTransferInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        authorityTransfer: {
            seeds: [
                { kind: "const", value: [97, 117, 116, 104, 111, 114, 105, 116, 121, 45, 116, 114, 97, 110, 115, 102, 101, 114] },
                { kind: "arg", path: "roleKind" }
            ]
        }
    };

    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["governanceAuthority"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.CORE, IX.core.PROPOSE_AUTHORITY_TRANSFER]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    roleKind!: U8;
    proposedAuthority!: PublicKeyLike;
    timelockSlots!: BU64;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    authorityTransfer!: PublicKeyLike;
    payer!: PublicKeyLike;
    governanceAuthority!: PublicKeyLike;
    systemProgram: PublicKeyLike & EncodableDefault = new PublicKey(SYSTEM_PROGRAM_ID_STRING);
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<ProposeAuthorityTransferInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, ProposeAuthorityTransferInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): ProposeAuthorityTransferInstruction {
        return Decoder.decode(data, ProposeAuthorityTransferInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, ProposeAuthorityTransferInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, ProposeAuthorityTransferInstruction);
    }

    static getSchema(): Schema {
        return {
            roleKind: {
                type: SchemaFieldType.U8
            },
            proposedAuthority: {
                type: SchemaFieldType.Address
            },
            timelockSlots: {
                type: SchemaFieldType.U64
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            chanceryConfig: { signer: false, writable: true, relations: ProposeAuthorityTransferInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            authorityTransfer: { signer: false, writable: true, pda: ProposeAuthorityTransferInstruction.programDerivedAccounts.authorityTransfer, accountSize: ACCOUNT_SIZES.AUTHORITY_TRANSFER, willCreate: true },
            payer: { signer: true, writable: true },
            governanceAuthority: { signer: true, writable: false },
            systemProgram: { signer: false, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
