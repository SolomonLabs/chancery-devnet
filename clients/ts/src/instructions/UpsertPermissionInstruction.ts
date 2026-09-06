// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          UpsertPermissionInstruction.ts
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
import type { BI64, BU64, PublicKeyLike, U8 } from "@solomon-labs/types";

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
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/permissions/instructions/upsert_permission.rs
 */
export class UpsertPermissionInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        moduleActivationState: {
            seeds: [
                { kind: "const", value: [109, 111, 100, 117, 108, 101, 45, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 45, 115, 116, 97, 116, 101] }
            ]
        },
        permissionRecord: {
            seeds: [
                { kind: "const", value: [112, 101, 114, 109, 105, 115, 115, 105, 111, 110] },
                { kind: "arg", path: "subject" },
                { kind: "arg", path: "scopeKind" },
                { kind: "arg", path: "scopeKey" }
            ]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.PERMISSIONS, IX.permissions.UPSERT_PERMISSION]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    subject!: PublicKeyLike;
    scopeKind!: U8;
    scopeKey!: PublicKeyLike;
    roleBits!: BU64[];
    expiryUnixTimestamp!: BI64;
    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    permissionRecord!: PublicKeyLike;
    grantingAuthority!: PublicKeyLike;
    grantorPermission: PublicKeyLike & EncodableDefault = DEFAULT_PUBLIC_KEY;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<UpsertPermissionInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, UpsertPermissionInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): UpsertPermissionInstruction {
        return Decoder.decode(data, UpsertPermissionInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, UpsertPermissionInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, UpsertPermissionInstruction);
    }

    static getSchema(): Schema {
        return {
            subject: {
                type: SchemaFieldType.Address
            },
            scopeKind: {
                type: SchemaFieldType.U8
            },
            scopeKey: {
                type: SchemaFieldType.Address
            },
            roleBits: {
                type: SchemaFieldType.Array,
                size: 2,
                codableType: SchemaFieldType.U64
            },
            expiryUnixTimestamp: {
                type: SchemaFieldType.I64
            }
        };
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: UpsertPermissionInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true },
            eventAuthority: { signer: false, writable: false },
            permissionRecord: { signer: false, writable: true, pda: UpsertPermissionInstruction.programDerivedAccounts.permissionRecord, accountSize: ACCOUNT_SIZES.PERMISSION_RECORD },
            grantingAuthority: { signer: true, writable: false },
            grantorPermission: { signer: false, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
