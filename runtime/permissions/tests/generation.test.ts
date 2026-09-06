import assert from "node:assert/strict";
import test from "node:test";

import { effectivePermissionGeneration, nextPermissionGeneration } from "../generation.js";
import type { PermissionGenerationState } from "../types.js";

const TOMBSTONE: PermissionGenerationState = {
    roleMask: 0n, expiryUnixTimestamp: 0n, permissionFlags: 1n,
    roleSchemaVersion: 1, permissionGeneration: 0n,
};

test("legacy revoked tombstones remain distinct from never-created permissions", () => {
    assert.equal(effectivePermissionGeneration(TOMBSTONE, 1), 1n);
    assert.equal(nextPermissionGeneration(effectivePermissionGeneration(TOMBSTONE, 1)), 2n);
    assert.equal(effectivePermissionGeneration({ ...TOMBSTONE, permissionGeneration: 7n }, 1), 7n);
});

test("each tombstone predicate is required to normalize a zero generation", () => {
    assert.equal(effectivePermissionGeneration({ ...TOMBSTONE, roleMask: 1n }, 1), 0n);
    assert.equal(effectivePermissionGeneration({ ...TOMBSTONE, permissionFlags: 0n }, 1), 0n);
    assert.equal(effectivePermissionGeneration({ ...TOMBSTONE, permissionFlags: 3n }, 1), 0n);
    assert.equal(effectivePermissionGeneration({ ...TOMBSTONE, expiryUnixTimestamp: 1n }, 1), 0n);
    assert.equal(effectivePermissionGeneration({ ...TOMBSTONE, roleSchemaVersion: 2 }, 1), 0n);
});

test("permission generations advance without wrapping", () => {
    assert.equal(nextPermissionGeneration(0n), 1n);
    assert.equal(nextPermissionGeneration((1n << 64n) - 2n), (1n << 64n) - 1n);
    assert.throws(() => nextPermissionGeneration(-1n), /u64/u);
    assert.throws(() => nextPermissionGeneration((1n << 64n) - 1n), /u64/u);
    assert.throws(() => effectivePermissionGeneration({ ...TOMBSTONE, permissionGeneration: -1n }, 1), /u64/u);
    assert.throws(() => effectivePermissionGeneration({ ...TOMBSTONE, permissionGeneration: 1n << 64n }, 1), /u64/u);
});
