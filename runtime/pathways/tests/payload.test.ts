import assert from "node:assert/strict";
import test from "node:test";

import { pathwayPolicyPayload } from "../payload.js";

test("pathway payload retains its semantic prefix and excludes reserved storage", () => {
    const account = new Uint8Array(616).fill(17);
    const expected = account.slice(0, 552);
    const payload = pathwayPolicyPayload(account);
    assert.equal(payload.length, 552);
    assert.deepEqual(payload, expected);
    account.fill(255, 552);
    assert.deepEqual(pathwayPolicyPayload(account), payload);
});

test("only the operational pathway pause bit is normalized", () => {
    const account = new Uint8Array(616);
    const view = new DataView(account.buffer);
    view.setBigUint64(352, 0xffff_ffff_ffff_ffffn, true);
    const payload = pathwayPolicyPayload(account);
    assert.equal(new DataView(payload.buffer).getBigUint64(352, true), 0xffff_ffff_ffff_fff7n);
    assert.equal(view.getBigUint64(352, true), 0xffff_ffff_ffff_ffffn);
    view.setBigUint64(352, 8n, true);
    assert.deepEqual(pathwayPolicyPayload(account), pathwayPolicyPayload(new Uint8Array(616)));
});

test("rebinding changes exactly the primary-limit identifier bytes", () => {
    const account = new Uint8Array(616).fill(1);
    const identifier = new Uint8Array(32).fill(7);
    const expected = pathwayPolicyPayload(account);
    expected.set(identifier, 224);
    assert.deepEqual(pathwayPolicyPayload(account, identifier), expected);
    assert.deepEqual(account.subarray(224, 256), new Uint8Array(32).fill(1));
    assert.deepEqual(identifier, new Uint8Array(32).fill(7));
});

test("null retains a reference while explicit zero unlinks it", () => {
    const account = new Uint8Array(616);
    account.fill(3, 224, 256);
    assert.deepEqual(pathwayPolicyPayload(account, null).subarray(224, 256), new Uint8Array(32).fill(3));
    assert.deepEqual(pathwayPolicyPayload(account, new Uint8Array(32)).subarray(224, 256), new Uint8Array(32));
});

test("sliced account buffers preserve their offsets", () => {
    const storage = new Uint8Array(648).fill(5);
    const account = storage.subarray(16, 632);
    const payload = pathwayPolicyPayload(account, new Uint8Array(32).fill(9));
    assert.deepEqual(payload.subarray(224, 256), new Uint8Array(32).fill(9));
    assert.deepEqual(storage.subarray(0, 16), new Uint8Array(16).fill(5));
    assert.deepEqual(storage.subarray(632), new Uint8Array(16).fill(5));
});

test("incorrect account and identifier widths are rejected", () => {
    assert.throws(() => pathwayPolicyPayload(new Uint8Array(615)), /account size/u);
    assert.throws(() => pathwayPolicyPayload(new Uint8Array(617)), /account size/u);
    assert.throws(() => pathwayPolicyPayload(new Uint8Array(552)), /account size/u);
    assert.throws(() => pathwayPolicyPayload(new Uint8Array(616), new Uint8Array(31)), /32 bytes/u);
    assert.throws(() => pathwayPolicyPayload(new Uint8Array(616), new Uint8Array(33)), /32 bytes/u);
});
