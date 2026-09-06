// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          SetAssetModeWithPendingChangeInstruction.ts
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
import { Decoder, Encoder, SchemaFieldType } from "@solomon-labs/solana-codec";
import type { PublicKeyLike, U8 } from "@solomon-labs/types";

import {
    PROGRAM_ID,
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    DEFAULT_PUBLIC_KEY,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/core/instructions/set_asset_mode_with_pending_change.rs
 */
export class SetAssetModeWithPendingChangeInstruction implements IsEncodable {
    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["governanceAuthority"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.CORE, IX.core.SET_ASSET_MODE_WITH_PENDING_CHANGE]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    newMode!: U8;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    pendingConfigChange!: PublicKeyLike;
    assetConfig!: PublicKeyLike;
    governanceAuthority!: PublicKeyLike;
    rentRefundRecipient: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<SetAssetModeWithPendingChangeInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, SetAssetModeWithPendingChangeInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): SetAssetModeWithPendingChangeInstruction {
        return Decoder.decode(data, SetAssetModeWithPendingChangeInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, SetAssetModeWithPendingChangeInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, SetAssetModeWithPendingChangeInstruction);
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
            chanceryConfig: { signer: false, writable: true, relations: SetAssetModeWithPendingChangeInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            pendingConfigChange: { signer: false, writable: true },
            assetConfig: { signer: false, writable: true },
            governanceAuthority: { signer: true, writable: false },
            rentRefundRecipient: { signer: false, writable: true },
            eventProgram: { signer: false, writable: false }
        };
    }
}
