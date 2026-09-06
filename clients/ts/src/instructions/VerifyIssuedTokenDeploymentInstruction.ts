// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      instruction
// File:          VerifyIssuedTokenDeploymentInstruction.ts
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
import { Decoder, Encoder } from "@solomon-labs/solana-codec";
import type { PublicKeyLike } from "@solomon-labs/types";

import {
    PROGRAM_ID,
    ACCOUNT_SIZES,
    MODULE_ACTIVATION_STATE,
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    ISSUED_TOKEN_CONTROL,
    MODULE,
    IX,
} from "../constants";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/issuance/instructions/verify_issued_token_deployment.rs
 */
export class VerifyIssuedTokenDeploymentInstruction implements IsEncodable {
    static readonly programDerivedAccounts: ProgramDerivedAccountSchema = {
        moduleActivationState: {
            seeds: [
                { kind: "const", value: [109, 111, 100, 117, 108, 101, 45, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 45, 115, 116, 97, 116, 101] }
            ]
        }
    };

    static readonly accountRelations: AccountRelationsSchema = {
        chanceryConfig: {
            relations: ["governanceAuthority", "issuedTokenMint"]
        }
    };

    static readonly discriminator = new Uint8Array([MODULE.ISSUED_TOKEN_CONTROL, IX.issued_token_control.VERIFY_ISSUED_TOKEN_DEPLOYMENT]);
    programId: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    moduleActivationState: PublicKeyLike & EncodableDefault = MODULE_ACTIVATION_STATE;
    chanceryConfig: PublicKeyLike & EncodableDefault = CHANCERY_CONFIG;
    eventAuthority: PublicKeyLike & EncodableDefault = EVENT_AUTHORITY;
    assetConfig!: PublicKeyLike;
    issuedTokenControl: PublicKeyLike & EncodableDefault = ISSUED_TOKEN_CONTROL;
    issuedTokenMint!: PublicKeyLike;
    payer!: PublicKeyLike;
    governanceAuthority!: PublicKeyLike;
    eventProgram: PublicKeyLike & EncodableDefault = PROGRAM_ID;

    constructor(props: EncodableProps<VerifyIssuedTokenDeploymentInstruction>) {
        Object.assign(this, props);
    }

    encode(): Uint8Array {
        return Encoder.encode(this, VerifyIssuedTokenDeploymentInstruction);
    }

    static decode(data: EncodedInstruction | IsEncodable): VerifyIssuedTokenDeploymentInstruction {
        return Decoder.decode(data, VerifyIssuedTokenDeploymentInstruction);
    }

    accounts(): InstructionAccount[] {
        return Encoder.accounts(this, VerifyIssuedTokenDeploymentInstruction);
    }

    instruction(): EncodedInstruction {
        return Encoder.instruction(this.programId, this, VerifyIssuedTokenDeploymentInstruction);
    }

    static getSchema(): Schema {
        return {};
    }

    static getAccountsSchema(): EncodeAccountsSchemaOrNull {
        return {
            moduleActivationState: { signer: false, writable: false, pda: VerifyIssuedTokenDeploymentInstruction.programDerivedAccounts.moduleActivationState, accountSize: ACCOUNT_SIZES.MODULE_ACTIVATION_STATE },
            chanceryConfig: { signer: false, writable: true, relations: VerifyIssuedTokenDeploymentInstruction.accountRelations.chanceryConfig.relations },
            eventAuthority: { signer: false, writable: false },
            assetConfig: { signer: false, writable: false },
            issuedTokenControl: { signer: false, writable: true },
            issuedTokenMint: { signer: false, writable: false },
            payer: { signer: true, writable: false },
            governanceAuthority: { signer: true, writable: false },
            eventProgram: { signer: false, writable: false }
        };
    }
}
