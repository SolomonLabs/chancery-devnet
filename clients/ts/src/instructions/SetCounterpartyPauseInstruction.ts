// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          SetCounterpartyPauseInstruction.ts
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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/control/instructions/set_counterparty_pause.rs
 */
export class SetCounterpartyPauseInstruction implements IsEncodable {
    static readonly discriminator = new Uint8Array([MODULE.CONTROL, IX.control.SET_COUNTERPARTY_PAUSE]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    subject!: PublicKeyLike;
    scopeKind!: U8;
    scopeKey!: PublicKeyLike;
    isPaused!: boolean;
    reasonCode!: U32;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    permissionRecord!: PublicKeyLike;
    authority!: PublicKeyLike;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<SetCounterpartyPauseInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, SetCounterpartyPauseInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): SetCounterpartyPauseInstruction {
        return Decoder.decode(data, SetCounterpartyPauseInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, SetCounterpartyPauseInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, SetCounterpartyPauseInstruction);
    }

    static getSchema(): Schema {
        return {
            subject: {
                type: SchemaFieldType.Address
            },
            scopeKind: {
                type: SchemaFieldType.U8
            },
            scopeKey: {
                type: SchemaFieldType.Address
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
            permissionRecord: { signer: false, writable: true },
            authority: { signer: true, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
