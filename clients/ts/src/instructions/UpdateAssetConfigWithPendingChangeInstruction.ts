// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          UpdateAssetConfigWithPendingChangeInstruction.ts
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
import type { BU64, PublicKeyLike } from "@solomon-labs/types";

import {
    PROGRAM_ID,
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    DEFAULT_PUBLIC_KEY,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/core/instructions/update_asset_config_with_pending_change.rs
 */
export class UpdateAssetConfigWithPendingChangeInstruction implements IsEncodable {
    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["governanceAuthority"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.CORE, IX.core.UPDATE_ASSET_CONFIG_WITH_PENDING_CHANGE]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    approvedExtensionMask!: BU64[] | null | undefined;
    observedExtensionMask!: BU64[] | null | undefined;
    depositRateE9!: BU64 | null | undefined;
    redeemRateE9!: BU64 | null | undefined;
    minimumDepositAmount!: BU64 | null | undefined;
    minimumRedeemAmount!: BU64 | null | undefined;
    maximumSingleSettlementAmount!: BU64 | null | undefined;
    forbiddenExtensionMask!: BU64[] | null | undefined;
    requiredModuleMask!: BU64[] | null | undefined;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    pendingConfigChange!: PublicKeyLike;
    assetConfig!: PublicKeyLike;
    governanceAuthority!: PublicKeyLike;
    rentRefundRecipient: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<UpdateAssetConfigWithPendingChangeInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, UpdateAssetConfigWithPendingChangeInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): UpdateAssetConfigWithPendingChangeInstruction {
        return Decoder.decode(data, UpdateAssetConfigWithPendingChangeInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, UpdateAssetConfigWithPendingChangeInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, UpdateAssetConfigWithPendingChangeInstruction);
    }

    static getSchema(): Schema {
        return {
            approvedExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64,
                optional: true
            },
            observedExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64,
                optional: true
            },
            depositRateE9: {
                type: SchemaFieldType.U64,
                optional: true
            },
            redeemRateE9: {
                type: SchemaFieldType.U64,
                optional: true
            },
            minimumDepositAmount: {
                type: SchemaFieldType.U64,
                optional: true
            },
            minimumRedeemAmount: {
                type: SchemaFieldType.U64,
                optional: true
            },
            maximumSingleSettlementAmount: {
                type: SchemaFieldType.U64,
                optional: true
            },
            forbiddenExtensionMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64,
                optional: true
            },
            requiredModuleMask: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64,
                optional: true
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            chanceryConfig: { signer: false, writable: true, relations: UpdateAssetConfigWithPendingChangeInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            pendingConfigChange: { signer: false, writable: true },
            assetConfig: { signer: false, writable: true },
            governanceAuthority: { signer: true, writable: false },
            rentRefundRecipient: { signer: false, writable: true },
            eventProgram: { signer: false, writable: false }
        };
    }
}
