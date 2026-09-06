// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          ProposeConfigChangeInstruction.ts
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
import { Decoder, Encoder, SchemaFieldType } from "@solomon-labs/solana-codec";
import type { BI64, BU64, PublicKeyLike, U16, U8 } from "@solomon-labs/types";

import { PROGRAM_ID, CHANCERY_CONFIG, EVENT_AUTHORITY, MODULE, IX } from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/control/instructions/propose_config_change.rs
 */
export class ProposeConfigChangeInstruction implements IsEncodable {
    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["governanceAuthority"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.CONTROL, IX.control.PROPOSE_CONFIG_CHANGE]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    changeKind!: U16;
    riskClass!: U8;
    targetAccount!: PublicKeyLike;
    oldValueHash!: U8[];
    newValueHash!: U8[];
    executableAfterUnixTimestamp!: BI64;
    expiresAtUnixTimestamp!: BI64;
    proposerNonce!: BU64;
    activationState!: PublicKeyLike;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    pending!: PublicKeyLike;
    payer!: PublicKeyLike;
    governanceAuthority!: PublicKeyLike;
    system: PublicKeyLike & EncodableDefault = new PublicKey(SYSTEM_PROGRAM_ID_STRING);
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<ProposeConfigChangeInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, ProposeConfigChangeInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): ProposeConfigChangeInstruction {
        return Decoder.decode(data, ProposeConfigChangeInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, ProposeConfigChangeInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, ProposeConfigChangeInstruction);
    }

    static getSchema(): Schema {
        return {
            changeKind: {
                type: SchemaFieldType.U16
            },
            riskClass: {
                type: SchemaFieldType.U8
            },
            targetAccount: {
                type: SchemaFieldType.Address
            },
            oldValueHash: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            newValueHash: {
                type: SchemaFieldType.Array,
                size: 32,
                codableType: SchemaFieldType.U8
            },
            executableAfterUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            expiresAtUnixTimestamp: {
                type: SchemaFieldType.I64
            },
            proposerNonce: {
                type: SchemaFieldType.U64
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            activationState: { signer: false, writable: false },
            chanceryConfig: { signer: false, writable: true, relations: ProposeConfigChangeInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            pending: { signer: false, writable: true },
            payer: { signer: true, writable: true },
            governanceAuthority: { signer: true, writable: false },
            system: { signer: false, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
