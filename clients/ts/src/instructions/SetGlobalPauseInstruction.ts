// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          SetGlobalPauseInstruction.ts
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
import type { BU64, PublicKeyLike, U32 } from "@solomon-labs/types";

import {
    PROGRAM_ID,
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    PAUSE_STATE,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/control/instructions/set_global_pause.rs
 */
export class SetGlobalPauseInstruction implements IsEncodable {
    static readonly discriminator = new Uint8Array([MODULE.CONTROL, IX.control.SET_GLOBAL_PAUSE]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    pauseBits!: BU64;
    isClear!: boolean;
    reasonCode!: U32;
    expiresAtSlot!: BU64;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    pauseState: PublicKeyLike & EncodableDefault = PAUSE_STATE;
    authority!: PublicKeyLike;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<SetGlobalPauseInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, SetGlobalPauseInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): SetGlobalPauseInstruction {
        return Decoder.decode(data, SetGlobalPauseInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, SetGlobalPauseInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, SetGlobalPauseInstruction);
    }

    static getSchema(): Schema {
        return {
            pauseBits: {
                type: SchemaFieldType.U64
            },
            isClear: {
                type: SchemaFieldType.Boolean
            },
            reasonCode: {
                type: SchemaFieldType.U32
            },
            expiresAtSlot: {
                type: SchemaFieldType.U64
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            chanceryConfig: { signer: false, writable: true },
            eventAuthority: { signer: false, writable: false },
            pauseState: { signer: false, writable: true },
            authority: { signer: true, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
