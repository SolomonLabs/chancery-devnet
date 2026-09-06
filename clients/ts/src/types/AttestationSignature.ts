// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      type
// File:          AttestationSignature.ts
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
import type { DecodableProps, IsDecodable, Schema } from "@solomon-labs/solana-codec";
import { Decoder, SchemaFieldType } from "@solomon-labs/solana-codec";
import type { U8 } from "@solomon-labs/types";

/**
 * Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/cross_chain/attestation.rs
 */
export class AttestationSignature implements IsDecodable {
    signerAddress!: U8[];
    signature!: U8[];
    recoveryId!: U8;
    merkleProof!: U8[][];

    constructor(props: DecodableProps<AttestationSignature>) {
        Object.assign(this, props);
    }

    static decode(data: string | Uint8Array | Buffer | null): AttestationSignature {
        return Decoder.decode(data, AttestationSignature);
    }

    static getSchema(): Schema {
        return {
            signerAddress: {
                type: SchemaFieldType.Array,
                size: 20,
                codableType: SchemaFieldType.U8
            },
            signature: {
                type: SchemaFieldType.Array,
                size: 64,
                codableType: SchemaFieldType.U8
            },
            recoveryId: {
                type: SchemaFieldType.U8
            },
            merkleProof: {
                type: SchemaFieldType.Vector,
                codableType: SchemaFieldType.Address
            }
        };
    }
}
