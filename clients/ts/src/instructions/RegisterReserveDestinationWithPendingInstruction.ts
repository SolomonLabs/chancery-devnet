// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          RegisterReserveDestinationWithPendingInstruction.ts
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
    ProgramDerivedAccountSchema,
    Schema
} from "@solomon-labs/solana-codec";
import { Decoder, Encoder, SchemaFieldType } from "@solomon-labs/solana-codec";
import type { BU64, PublicKeyLike, U8 } from "@solomon-labs/types";

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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/reserve/instructions/register_reserve_destination_with_pending.rs
 */
export class RegisterReserveDestinationWithPendingInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        moduleActivationState: {
            seeds: [
                { kind: "const", value: [109, 111, 100, 117, 108, 101, 45, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 45, 115, 116, 97, 116, 101] }
            ]
        },
        reserveDestination: {
            seeds: [
                { kind: "const", value: [114, 101, 115, 101, 114, 118, 101, 45, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110] },
                { kind: "account", path: "assetMint" },
                { kind: "account", path: "destinationTokenAccount" }
            ]
        }
    };

    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["governanceAuthority"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.RESERVE, IX.reserve.REGISTER_RESERVE_DESTINATION_WITH_PENDING]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    destinationOwner!: PublicKeyLike;
    destinationFlags!: BU64;
    withdrawalLimitPolicyId!: U8[];
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    pending!: PublicKeyLike;
    reserveDestination!: PublicKeyLike;
    assetMint!: PublicKeyLike;
    destinationTokenAccount!: PublicKeyLike;
    payer!: PublicKeyLike;
    governanceAuthority!: PublicKeyLike;
    systemProgram: PublicKeyLike & EncodableDefault = new PublicKey(SYSTEM_PROGRAM_ID_STRING);
    withdrawalLimitPolicy!: PublicKeyLike;
    rentRefundRecipient: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<RegisterReserveDestinationWithPendingInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, RegisterReserveDestinationWithPendingInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): RegisterReserveDestinationWithPendingInstruction {
        return Decoder.decode(data, RegisterReserveDestinationWithPendingInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, RegisterReserveDestinationWithPendingInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, RegisterReserveDestinationWithPendingInstruction);
    }

    static getSchema(): Schema {
        return {
            destinationOwner: {
                type: SchemaFieldType.Address
            },
            destinationFlags: {
                type: SchemaFieldType.U64
            },
            withdrawalLimitPolicyId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: RegisterReserveDestinationWithPendingInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true, relations: RegisterReserveDestinationWithPendingInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            pending: { signer: false, writable: true },
            reserveDestination: { signer: false, writable: true, pda: RegisterReserveDestinationWithPendingInstruction.programDerivedAccounts.reserveDestination, accountSize: ACCOUNT_SIZES.RESERVE_DESTINATION, willCreate: true },
            assetMint: { signer: false, writable: false },
            destinationTokenAccount: { signer: false, writable: false },
            payer: { signer: true, writable: true },
            governanceAuthority: { signer: true, writable: false },
            systemProgram: { signer: false, writable: false },
            withdrawalLimitPolicy: { signer: false, writable: false },
            rentRefundRecipient: { signer: false, writable: true },
            eventProgram: { signer: false, writable: false }
        };
    }
}
