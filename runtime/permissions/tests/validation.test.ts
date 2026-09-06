import assert from "node:assert/strict";
import test from "node:test";

import { validatePermissionCapabilities, validatePermissionCreation } from "../validation.js";
import type { PermissionCapabilities } from "../types.js";

const ACTIVE_ROLE_MASK = 7n;
const DANGEROUS_ROLE_MASK = 4n;
const NOW = 100n;
const ORDINARY: PermissionCapabilities = { roleMask: 3n, expiryUnixTimestamp: 0n };

function validate(value: PermissionCapabilities, isGlobalScope = false): void {
    validatePermissionCreation(value, ACTIVE_ROLE_MASK, DANGEROUS_ROLE_MASK, isGlobalScope, NOW);
}

test("creation requires at least one active role", () => {
    assert.throws(() => validate({ ...ORDINARY, roleMask: 0n }), /at least one role/u);
    assert.throws(() => validate({ ...ORDINARY, roleMask: 8n }), /unsupported role bits/u);
    assert.throws(() => validate({ ...ORDINARY, roleMask: 9n }), /unsupported role bits/u);
    assert.doesNotThrow(() => validate(ORDINARY));
});

test("creation checks role and expiry wire ranges before classifying capability", () => {
    assert.throws(() => validate({ ...ORDINARY, roleMask: -1n }), /u128/u);
    assert.throws(() => validate({ ...ORDINARY, roleMask: 1n << 128n }), /u128/u);
    assert.throws(() => validate({ ...ORDINARY, expiryUnixTimestamp: -1n }), /i64/u);
    assert.throws(() => validate({ ...ORDINARY, expiryUnixTimestamp: 1n << 63n }), /i64/u);
});

test("ordinary grants support global or resource scope with finite or perpetual expiry", () => {
    assert.doesNotThrow(() => validate(ORDINARY, true));
    assert.doesNotThrow(() => validate(ORDINARY, false));
    assert.doesNotThrow(() => validate({ ...ORDINARY, expiryUnixTimestamp: NOW + 1n }, true));
    assert.doesNotThrow(() => validate({ ...ORDINARY, expiryUnixTimestamp: (1n << 63n) - 1n }));
});

test("dangerous grants require both resource scope and finite expiry", () => {
    const dangerous = { roleMask: 4n, expiryUnixTimestamp: NOW + 1n };
    assert.throws(() => validate(dangerous, true), /non-global scope/u);
    assert.throws(() => validate({ ...dangerous, expiryUnixTimestamp: 0n }), /finite expiry/u);
    assert.doesNotThrow(() => validate(dangerous));
    assert.doesNotThrow(() => validate({ ...dangerous, roleMask: 7n }));
});

test("finite creation expiry must be strictly later than the observed clock", () => {
    for (const roleMask of [ORDINARY.roleMask, DANGEROUS_ROLE_MASK]) {
        assert.throws(() => validate({ roleMask, expiryUnixTimestamp: NOW - 1n }), /already expired/u);
        assert.throws(() => validate({ roleMask, expiryUnixTimestamp: NOW }), /already expired/u);
        assert.doesNotThrow(() => validate({ roleMask, expiryUnixTimestamp: NOW + 1n }));
    }
});

test("wire value validation preserves zero roles for updates", () => {
    assert.doesNotThrow(() => validatePermissionCapabilities({ roleMask: 0n, expiryUnixTimestamp: 0n }));
    assert.doesNotThrow(() => validatePermissionCapabilities({ roleMask: (1n << 128n) - 1n,
        expiryUnixTimestamp: (1n << 63n) - 1n }));
    assert.deepEqual(ORDINARY, { roleMask: 3n, expiryUnixTimestamp: 0n });
});
