// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          SetModuleStatusInstruction.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/control/instructions/set_module_status.rs
 */
export class SetModuleStatusInstruction implements IsEncodable {
    static readonly discriminator = new Uint8Array([MODULE.CONTROL, IX.control.SET_MODULE_STATUS]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    moduleId!: U8;
    newStatus!: U8;
    activation!: PublicKeyLike;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    authority!: PublicKeyLike;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<SetModuleStatusInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, SetModuleStatusInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): SetModuleStatusInstruction {
        return Decoder.decode(data, SetModuleStatusInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, SetModuleStatusInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, SetModuleStatusInstruction);
    }

    static getSchema(): Schema {
        return {
            moduleId: {
                type: SchemaFieldType.U8
            },
            newStatus: {
                type: SchemaFieldType.U8
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            activation: { signer: false, writable: true },
            chanceryConfig: { signer: false, writable: true },
            eventAuthority: { signer: false, writable: false },
            authority: { signer: true, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
