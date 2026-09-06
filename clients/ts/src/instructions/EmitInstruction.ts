// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          EmitInstruction.ts
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
import { Decoder, Encoder } from "@solomon-labs/solana-codec";
import type { PublicKeyLike } from "@solomon-labs/types";

import { PROGRAM_ID, EVENT_AUTHORITY, MODULE, IX } from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/events_cpi/instructions/emit.rs
 */
export class EmitInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        eventAuthority: {
            seeds: [
                { kind: "const", value: [101, 118, 101, 110, 116, 45, 97, 117, 116, 104, 111, 114, 105, 116, 121] }
            ]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.EVENTS_CPI, IX.events_cpi.EMIT]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<EmitInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, EmitInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): EmitInstruction {
        return Decoder.decode(data, EmitInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, EmitInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, EmitInstruction);
    }

    static getSchema(): Schema {
        return {};
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            eventAuthority: { signer: true, writable: false, pda: EmitInstruction.programDerivedAccounts.eventAuthority },
            eventProgram: { signer: false, writable: false }
        };
    }
}
