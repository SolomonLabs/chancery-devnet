// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          InitializeModuleActivationStateInstruction.ts
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
    Schema
} from "@solomon-labs/solana-codec";
import { Decoder, Encoder } from "@solomon-labs/solana-codec";
import type { PublicKeyLike } from "@solomon-labs/types";

import { PROGRAM_ID, CHANCERY_CONFIG, EVENT_AUTHORITY, MODULE, IX } from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/control/instructions/initialize_module_activation_state.rs
 */
export class InitializeModuleActivationStateInstruction implements IsEncodable {
    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["governanceAuthority"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.CONTROL, IX.control.INITIALIZE_MODULE_ACTIVATION_STATE]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    activation!: PublicKeyLike;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    payer!: PublicKeyLike;
    governanceAuthority!: PublicKeyLike;
    system: PublicKeyLike & EncodableDefault = new PublicKey(SYSTEM_PROGRAM_ID_STRING);
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<InitializeModuleActivationStateInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, InitializeModuleActivationStateInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): InitializeModuleActivationStateInstruction {
        return Decoder.decode(data, InitializeModuleActivationStateInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, InitializeModuleActivationStateInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, InitializeModuleActivationStateInstruction);
    }

    static getSchema(): Schema {
        return {};
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            activation: { signer: false, writable: true },
            chanceryConfig: { signer: false, writable: true, relations: InitializeModuleActivationStateInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            payer: { signer: true, writable: true },
            governanceAuthority: { signer: true, writable: false },
            system: { signer: false, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
