// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          RotateCrossChainSignerSetInstruction.ts
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
import { Decoder, Encoder, SchemaFieldType } from "@solomon-labs/solana-codec";
import type { PublicKeyLike, U16, U8 } from "@solomon-labs/types";

import {
    PROGRAM_ID,
    ACCOUNT_SIZES,
    MODULE_ACTIVATION_STATE,
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/cross_chain/instructions/rotate_cross_chain_signer_set.rs
 */
export class RotateCrossChainSignerSetInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        moduleActivationState: {
            seeds: [
                { kind: "const", value: [109, 111, 100, 117, 108, 101, 45, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 45, 115, 116, 97, 116, 101] }
            ]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.CROSS_CHAIN, IX.cross_chain.ROTATE_CROSS_CHAIN_SIGNER_SET]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    signerSetId!: U8[];
    reasonCode!: U16;
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    signerSet!: PublicKeyLike;
    authority!: PublicKeyLike;
    authorityPermissionRecord!: PublicKeyLike;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<RotateCrossChainSignerSetInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, RotateCrossChainSignerSetInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): RotateCrossChainSignerSetInstruction {
        return Decoder.decode(data, RotateCrossChainSignerSetInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, RotateCrossChainSignerSetInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, RotateCrossChainSignerSetInstruction);
    }

    static getSchema(): Schema {
        return {
            signerSetId: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            reasonCode: {
                type: SchemaFieldType.U16
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: RotateCrossChainSignerSetInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true },
            eventAuthority: { signer: false, writable: false },
            signerSet: { signer: false, writable: true },
            authority: { signer: true, writable: false },
            authorityPermissionRecord: { signer: false, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
