// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          SetAssetPauseInstruction.ts
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
import type { BU64, PublicKeyLike, U32 } from "@solomon-labs/types";

import {
    PROGRAM_ID,
    ACCOUNT_SIZES,
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/control/instructions/set_asset_pause.rs
 */
export class SetAssetPauseInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        assetPauseState: {
            seeds: [
                { kind: "const", value: [97, 115, 115, 101, 116, 45, 112, 97, 117, 115, 101] },
                { kind: "account", path: "assetMint" }
            ]
        }
    };

    static readonly accountRelations: AccountRelationsSchema = {
        assetPauseState: {
            relations: ["assetMint"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.CONTROL, IX.control.SET_ASSET_PAUSE]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    pauseBits!: BU64;
    isClear!: boolean;
    reasonCode!: U32;
    expiresAtSlot!: BU64;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    assetPauseState!: PublicKeyLike;
    assetMint!: PublicKeyLike;
    payer!: PublicKeyLike;
    authority!: PublicKeyLike;
    systemProgram: PublicKeyLike & EncodableDefault = new PublicKey(SYSTEM_PROGRAM_ID_STRING);
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<SetAssetPauseInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, SetAssetPauseInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): SetAssetPauseInstruction {
        return Decoder.decode(data, SetAssetPauseInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, SetAssetPauseInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, SetAssetPauseInstruction);
    }

    static getSchema(): Schema {
        return {
            pauseBits: {
                type: SchemaFieldType.U64
            },
            isClear: {
                type: SchemaFieldType.Boolean
            },
            reasonCode: {
                type: SchemaFieldType.U32
            },
            expiresAtSlot: {
                type: SchemaFieldType.U64
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            chanceryConfig: { signer: false, writable: true },
            eventAuthority: { signer: false, writable: false },
            assetPauseState: { signer: false, writable: true, pda: SetAssetPauseInstruction.programDerivedAccounts.assetPauseState, relations: SetAssetPauseInstruction.accountRelations.assetPauseState.relations, accountSize: ACCOUNT_SIZES.ASSET_PAUSE_STATE, willCreate: true },
            assetMint: { signer: false, writable: false },
            payer: { signer: true, writable: true },
            authority: { signer: true, writable: false },
            systemProgram: { signer: false, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
