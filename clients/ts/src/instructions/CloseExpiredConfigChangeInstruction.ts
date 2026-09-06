// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          CloseExpiredConfigChangeInstruction.ts
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
import { Decoder, Encoder } from "@solomon-labs/solana-codec";
import type { PublicKeyLike } from "@solomon-labs/types";

import {
    PROGRAM_ID,
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    DEFAULT_PUBLIC_KEY,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/control/instructions/close_expired_config_change.rs
 */
export class CloseExpiredConfigChangeInstruction implements IsEncodable {
    static readonly discriminator = new Uint8Array([MODULE.CONTROL, IX.control.CLOSE_EXPIRED_CONFIG_CHANGE]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    activationState!: PublicKeyLike;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    pending!: PublicKeyLike;
    rentRefundRecipient: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<CloseExpiredConfigChangeInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, CloseExpiredConfigChangeInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): CloseExpiredConfigChangeInstruction {
        return Decoder.decode(data, CloseExpiredConfigChangeInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, CloseExpiredConfigChangeInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, CloseExpiredConfigChangeInstruction);
    }

    static getSchema(): Schema {
        return {};
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            activationState: { signer: false, writable: false },
            chanceryConfig: { signer: false, writable: true },
            eventAuthority: { signer: false, writable: false },
            pending: { signer: false, writable: true },
            rentRefundRecipient: { signer: false, writable: true },
            eventProgram: { signer: false, writable: false }
        };
    }
}
