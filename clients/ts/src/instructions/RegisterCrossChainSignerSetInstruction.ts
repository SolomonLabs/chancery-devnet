// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          RegisterCrossChainSignerSetInstruction.ts
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
import type { BI64, PublicKeyLike, U8 } from "@solomon-labs/types";

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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/cross_chain/instructions/register_cross_chain_signer_set.rs
 */
export class RegisterCrossChainSignerSetInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        moduleActivationState: {
            seeds: [
                { kind: "const", value: [109, 111, 100, 117, 108, 101, 45, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 45, 115, 116, 97, 116, 101] }
            ]
        },
        signerSet: {
            seeds: [
                { kind: "const", value: [99, 114, 111, 115, 115, 45, 99, 104, 97, 105, 110, 45, 115, 105, 103, 110, 101, 114, 45, 115, 101, 116] },
                { kind: "arg", path: "signerSetId" }
            ]
        }
    };

    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["governanceAuthority"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.CROSS_CHAIN, IX.cross_chain.REGISTER_CROSS_CHAIN_SIGNER_SET]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    signerSetId!: U8[];
    threshold!: U8;
    signerCount!: U8;
    signerRoot!: U8[];
    validAfterUnixTimestamp!: BI64;
    expiresAtUnixTimestamp!: BI64;
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    pending!: PublicKeyLike;
    signerSet!: PublicKeyLike;
    payer!: PublicKeyLike;
    registeringAuthority!: PublicKeyLike;
    governanceAuthority!: PublicKeyLike;
    authorityPermissionRecord!: PublicKeyLike;
    systemProgram: PublicKeyLike & EncodableDefault = new PublicKey(SYSTEM_PROGRAM_ID_STRING);
    rentRefundRecipient: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<RegisterCrossChainSignerSetInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, RegisterCrossChainSignerSetInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): RegisterCrossChainSignerSetInstruction {
        return Decoder.decode(data, RegisterCrossChainSignerSetInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, RegisterCrossChainSignerSetInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, RegisterCrossChainSignerSetInstruction);
    }

    static getSchema(): Schema {
        return {
            signerSetId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            threshold: {
                type: SchemaFieldType.U8
            },
            signerCount: {
                type: SchemaFieldType.U8
            },
            signerRoot: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            validAfterUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            expiresAtUnixTimestamp: {
                type: SchemaFieldType.I64
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: RegisterCrossChainSignerSetInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true, relations: RegisterCrossChainSignerSetInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            pending: { signer: false, writable: true },
            signerSet: { signer: false, writable: true, pda: RegisterCrossChainSignerSetInstruction.programDerivedAccounts.signerSet },
            payer: { signer: true, writable: true },
            registeringAuthority: { signer: true, writable: false },
            governanceAuthority: { signer: true, writable: false },
            authorityPermissionRecord: { signer: false, writable: false },
            systemProgram: { signer: false, writable: false },
            rentRefundRecipient: { signer: false, writable: true },
            eventProgram: { signer: false, writable: false }
        };
    }
}
