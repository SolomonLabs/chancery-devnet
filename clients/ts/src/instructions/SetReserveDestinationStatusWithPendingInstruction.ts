// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          SetReserveDestinationStatusWithPendingInstruction.ts
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
    ProgramDerivedAccountSchema,
    Schema
} from "@solomon-labs/solana-codec";
import { Decoder, Encoder, SchemaFieldType } from "@solomon-labs/solana-codec";
import type { PublicKeyLike, U8 } from "@solomon-labs/types";

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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/reserve/instructions/set_reserve_destination_status_with_pending.rs
 */
export class SetReserveDestinationStatusWithPendingInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        moduleActivationState: {
            seeds: [
                { kind: "const", value: [109, 111, 100, 117, 108, 101, 45, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 45, 115, 116, 97, 116, 101] }
            ]
        }
    };

    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["governanceAuthority"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.RESERVE, IX.reserve.SET_RESERVE_DESTINATION_STATUS_WITH_PENDING]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    newStatus!: U8;
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    pending!: PublicKeyLike;
    reserveDestination!: PublicKeyLike;
    governanceAuthority!: PublicKeyLike;
    rentRefundRecipient: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<SetReserveDestinationStatusWithPendingInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, SetReserveDestinationStatusWithPendingInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): SetReserveDestinationStatusWithPendingInstruction {
        return Decoder.decode(data, SetReserveDestinationStatusWithPendingInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, SetReserveDestinationStatusWithPendingInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, SetReserveDestinationStatusWithPendingInstruction);
    }

    static getSchema(): Schema {
        return {
            newStatus: {
                type: SchemaFieldType.U8
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: SetReserveDestinationStatusWithPendingInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true, relations: SetReserveDestinationStatusWithPendingInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            pending: { signer: false, writable: true },
            reserveDestination: { signer: false, writable: true },
            governanceAuthority: { signer: true, writable: false },
            rentRefundRecipient: { signer: false, writable: true },
            eventProgram: { signer: false, writable: false }
        };
    }
}
