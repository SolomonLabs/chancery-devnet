// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          SetPathwayPauseInstruction.ts
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
import type { PublicKeyLike, U32, U8 } from "@solomon-labs/types";

import { PROGRAM_ID, CHANCERY_CONFIG, EVENT_AUTHORITY, MODULE, IX } from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/control/instructions/set_pathway_pause.rs
 */
export class SetPathwayPauseInstruction implements IsEncodable {
    static readonly discriminator = new Uint8Array([MODULE.CONTROL, IX.control.SET_PATHWAY_PAUSE]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    pathwayId!: U8[];
    isPaused!: boolean;
    reasonCode!: U32;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    pathwayPolicy!: PublicKeyLike;
    authority!: PublicKeyLike;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<SetPathwayPauseInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, SetPathwayPauseInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): SetPathwayPauseInstruction {
        return Decoder.decode(data, SetPathwayPauseInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, SetPathwayPauseInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, SetPathwayPauseInstruction);
    }

    static getSchema(): Schema {
        return {
            pathwayId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            isPaused: {
                type: SchemaFieldType.Boolean
            },
            reasonCode: {
                type: SchemaFieldType.U32
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            chanceryConfig: { signer: false, writable: true },
            eventAuthority: { signer: false, writable: false },
            pathwayPolicy: { signer: false, writable: true },
            authority: { signer: true, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
