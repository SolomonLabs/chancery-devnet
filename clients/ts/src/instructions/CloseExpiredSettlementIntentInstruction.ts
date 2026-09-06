// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          CloseExpiredSettlementIntentInstruction.ts
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

import {
    PROGRAM_ID,
    ACCOUNT_SIZES,
    MODULE_ACTIVATION_STATE,
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    DEFAULT_PUBLIC_KEY,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/settlement/instructions/close_expired_settlement_intent.rs
 */
export class CloseExpiredSettlementIntentInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        moduleActivationState: {
            seeds: [
                { kind: "const", value: [109, 111, 100, 117, 108, 101, 45, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 45, 115, 116, 97, 116, 101] }
            ]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.SETTLEMENT, IX.settlement.CLOSE_EXPIRED_SETTLEMENT_INTENT]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    settlementIntent!: PublicKeyLike;
    rentRefundRecipient: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<CloseExpiredSettlementIntentInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, CloseExpiredSettlementIntentInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): CloseExpiredSettlementIntentInstruction {
        return Decoder.decode(data, CloseExpiredSettlementIntentInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, CloseExpiredSettlementIntentInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, CloseExpiredSettlementIntentInstruction);
    }

    static getSchema(): Schema {
        return {};
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: CloseExpiredSettlementIntentInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true },
            eventAuthority: { signer: false, writable: false },
            settlementIntent: { signer: false, writable: true },
            rentRefundRecipient: { signer: false, writable: true },
            eventProgram: { signer: false, writable: false }
        };
    }
}
