// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          AcceptAuthorityTransferInstruction.ts
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
    Schema
} from "@solomon-labs/solana-codec";
import { Decoder, Encoder, SchemaFieldType } from "@solomon-labs/solana-codec";
import type { PublicKeyLike, U8 } from "@solomon-labs/types";

import { PROGRAM_ID, CHANCERY_CONFIG, EVENT_AUTHORITY, MODULE, IX } from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/core/instructions/accept_authority_transfer.rs
 */
export class AcceptAuthorityTransferInstruction implements IsEncodable {
    static readonly discriminator = new Uint8Array([MODULE.CORE, IX.core.ACCEPT_AUTHORITY_TRANSFER]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    roleKind!: U8;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    authorityTransfer!: PublicKeyLike;
    newAuthority!: PublicKeyLike;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<AcceptAuthorityTransferInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, AcceptAuthorityTransferInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): AcceptAuthorityTransferInstruction {
        return Decoder.decode(data, AcceptAuthorityTransferInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, AcceptAuthorityTransferInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, AcceptAuthorityTransferInstruction);
    }

    static getSchema(): Schema {
        return {
            roleKind: {
                type: SchemaFieldType.U8
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            chanceryConfig: { signer: false, writable: true },
            eventAuthority: { signer: false, writable: false },
            authorityTransfer: { signer: false, writable: true },
            newAuthority: { signer: true, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
