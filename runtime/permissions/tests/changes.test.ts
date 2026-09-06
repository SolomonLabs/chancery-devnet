import assert from "node:assert/strict";
import test from "node:test";

import { RISK } from "../../administration/risk.js";
import { applyPermissionChanges, classifyPermissionChange } from "../changes.js";
import type { PermissionCapabilities } from "../types.js";

const CURRENT: PermissionCapabilities = { roleMask: 3n, expiryUnixTimestamp: 100n };
const DANGEROUS_ROLE_MASK = 4n;

test("omitted permission fields retain their values and explicit zero clears them", () => {
    assert.deepEqual(applyPermissionChanges(CURRENT, { roleMask: null, expiryUnixTimestamp: null }), CURRENT);
    assert.deepEqual(applyPermissionChanges(CURRENT, { roleMask: 0n, expiryUnixTimestamp: 0n }),
        { roleMask: 0n, expiryUnixTimestamp: 0n });
    assert.deepEqual(CURRENT, { roleMask: 3n, expiryUnixTimestamp: 100n });
});

test("permission values remain within the wire integer ranges", () => {
    assert.throws(() => applyPermissionChanges(CURRENT, { roleMask: -1n, expiryUnixTimestamp: null }), /u128/u);
    assert.throws(() => applyPermissionChanges(CURRENT, { roleMask: 1n << 128n, expiryUnixTimestamp: null }), /u128/u);
    assert.throws(() => applyPermissionChanges(CURRENT, { roleMask: null, expiryUnixTimestamp: -1n }), /i64/u);
    assert.throws(() => applyPermissionChanges(CURRENT, { roleMask: null, expiryUnixTimestamp: 1n << 63n }), /i64/u);
    assert.equal(applyPermissionChanges(CURRENT, { roleMask: (1n << 128n) - 1n,
        expiryUnixTimestamp: (1n << 63n) - 1n }).roleMask, (1n << 128n) - 1n);
});

test("new ordinary and dangerous grants have their corresponding timelocked risk", () => {
    assert.equal(classifyPermissionChange(null, CURRENT, DANGEROUS_ROLE_MASK), RISK.WIDENING);
    assert.equal(classifyPermissionChange(null, { ...CURRENT, roleMask: 4n }, DANGEROUS_ROLE_MASK), RISK.DANGEROUS);
});

test("unchanged values are routine and removal or shorter expiry is restrictive", () => {
    assert.equal(classifyPermissionChange(CURRENT, CURRENT, DANGEROUS_ROLE_MASK), RISK.ROUTINE_OPS);
    assert.equal(classifyPermissionChange(CURRENT, { ...CURRENT, roleMask: 1n }, DANGEROUS_ROLE_MASK), RISK.RESTRICTIVE_IMMEDIATE);
    assert.equal(classifyPermissionChange(CURRENT, { ...CURRENT, roleMask: 0n }, DANGEROUS_ROLE_MASK), RISK.RESTRICTIVE_IMMEDIATE);
    assert.equal(classifyPermissionChange(CURRENT, { ...CURRENT, expiryUnixTimestamp: 99n }, DANGEROUS_ROLE_MASK), RISK.RESTRICTIVE_IMMEDIATE);
    assert.equal(classifyPermissionChange({ ...CURRENT, expiryUnixTimestamp: 0n }, CURRENT, DANGEROUS_ROLE_MASK), RISK.RESTRICTIVE_IMMEDIATE);
});

test("ordinary role additions and expiry extensions widen even during mixed restrictive edits", () => {
    assert.equal(classifyPermissionChange({ ...CURRENT, roleMask: 1n }, CURRENT, DANGEROUS_ROLE_MASK), RISK.WIDENING);
    assert.equal(classifyPermissionChange(CURRENT, { roleMask: 8n, expiryUnixTimestamp: 99n }, DANGEROUS_ROLE_MASK), RISK.WIDENING);
    assert.equal(classifyPermissionChange(CURRENT, { roleMask: 1n, expiryUnixTimestamp: 101n }, DANGEROUS_ROLE_MASK), RISK.WIDENING);
});

test("removing a finite expiry has high-impact risk even when roles are removed", () => {
    assert.equal(classifyPermissionChange(CURRENT, { ...CURRENT, expiryUnixTimestamp: 0n }, DANGEROUS_ROLE_MASK), RISK.HIGH_IMPACT);
    assert.equal(classifyPermissionChange(CURRENT, { roleMask: 0n, expiryUnixTimestamp: 0n }, DANGEROUS_ROLE_MASK), RISK.HIGH_IMPACT);
});

test("widening a resulting dangerous grant is dangerous even when its dangerous bit is retained", () => {
    const current = { roleMask: 4n, expiryUnixTimestamp: 100n };
    assert.equal(classifyPermissionChange(current, { ...current, roleMask: 5n }, DANGEROUS_ROLE_MASK), RISK.DANGEROUS);
    assert.equal(classifyPermissionChange(current, { ...current, expiryUnixTimestamp: 101n }, DANGEROUS_ROLE_MASK), RISK.DANGEROUS);
    assert.equal(classifyPermissionChange(current, { ...current, expiryUnixTimestamp: 0n }, DANGEROUS_ROLE_MASK), RISK.DANGEROUS);
    assert.equal(classifyPermissionChange(current, { ...current, roleMask: 1n }, DANGEROUS_ROLE_MASK), RISK.WIDENING);
    assert.equal(classifyPermissionChange(current, { ...current, expiryUnixTimestamp: 99n }, DANGEROUS_ROLE_MASK), RISK.RESTRICTIVE_IMMEDIATE);
});
