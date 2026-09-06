// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      test
// File:          ChanceryProgram.types.generated.test.ts
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
import { assert, describe, it } from "@solomon-labs/testing";

import { AttestationSignature } from "../src/types/AttestationSignature";

describe("ChanceryProgram type classes auto generated tests", () => {
    describe("types", () => {
        it("has schema for AttestationSignature", () => {
            const schema = AttestationSignature.getSchema();
            assert(schema.signerAddress);
            assert(schema.signature);
            assert(schema.recoveryId);
            assert(schema.merkleProof);
        });

    });
});
