// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          SetAssetModeInstruction.ts
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
import type { PublicKeyLike, U8 } from "@solomon-labs/types";

import { PROGRAM_ID, CHANCERY_CONFIG, MODULE, IX } from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/core/instructions/set_asset_mode.rs
 */
export class SetAssetModeInstruction implements IsEncodable {
    static readonly discriminator = new Uint8Array([MODULE.CORE, IX.core.SET_ASSET_MODE]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    newMode!: U8;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    assetConfig!: PublicKeyLike;
    authority!: PublicKeyLike;
    eventAuthorityDirect!: PublicKeyLike;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<SetAssetModeInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, SetAssetModeInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): SetAssetModeInstruction {
        return Decoder.decode(data, SetAssetModeInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, SetAssetModeInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, SetAssetModeInstruction);
    }

    static getSchema(): Schema {
        return {
            newMode: {
                type: SchemaFieldType.U8
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            chanceryConfig: { signer: false, writable: true },
            assetConfig: { signer: false, writable: true },
            authority: { signer: true, writable: false },
            eventAuthorityDirect: { signer: false, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
