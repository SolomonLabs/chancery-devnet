// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          AcceptConfigChangeInstruction.ts
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
import { Decoder, Encoder } from "@solomon-labs/solana-codec";
import type { PublicKeyLike } from "@solomon-labs/types";

import { PROGRAM_ID, CHANCERY_CONFIG, EVENT_AUTHORITY, MODULE, IX } from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/control/instructions/accept_config_change.rs
 */
export class AcceptConfigChangeInstruction implements IsEncodable {
    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["governanceAuthority"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.CONTROL, IX.control.ACCEPT_CONFIG_CHANGE]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    activationState!: PublicKeyLike;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    pending!: PublicKeyLike;
    governanceAuthority!: PublicKeyLike;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<AcceptConfigChangeInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, AcceptConfigChangeInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): AcceptConfigChangeInstruction {
        return Decoder.decode(data, AcceptConfigChangeInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, AcceptConfigChangeInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, AcceptConfigChangeInstruction);
    }

    static getSchema(): Schema {
        return {};
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            activationState: { signer: false, writable: false },
            chanceryConfig: { signer: false, writable: true, relations: AcceptConfigChangeInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            pending: { signer: false, writable: true },
            governanceAuthority: { signer: true, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
