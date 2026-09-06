import assert from "node:assert/strict";
import test from "node:test";

import { applyLimitCapChanges, limitCapsWiden, limitPolicyPayload, validateLimitCaps } from "../caps.js";
import type { LimitCapChanges, LimitCaps } from "../types.js";

const CAPS: LimitCaps = {
    perTransactionMaximum: 10n, perHourMaximum: 20n, perDayMaximum: 30n,
    perSevenDayMaximum: 40n, perThirtyDayMaximum: 50n,
    maximumActionsPerHour: 2, maximumActionsPerDay: 3,
};
const UNCHANGED: LimitCapChanges = {
    perTransactionMaximum: null, perHourMaximum: null, perDayMaximum: null,
    perSevenDayMaximum: null, perThirtyDayMaximum: null,
    maximumActionsPerHour: null, maximumActionsPerDay: null,
};

test("omission retains the current value while explicit zero disables the cap", () => {
    assert.deepEqual(applyLimitCapChanges(CAPS, UNCHANGED), CAPS);
    assert.equal(applyLimitCapChanges(CAPS, { ...UNCHANGED, perDayMaximum: 0n }).perDayMaximum, 0n);
    assert.equal(applyLimitCapChanges(CAPS, { ...UNCHANGED, maximumActionsPerHour: 0 }).maximumActionsPerHour, 0);
});

test("finite increases and disabling an enabled cap widen", () => {
    assert.equal(limitCapsWiden(CAPS, { ...CAPS, perTransactionMaximum: 11n }), true);
    assert.equal(limitCapsWiden(CAPS, { ...CAPS, perDayMaximum: 0n }), true);
    assert.equal(limitCapsWiden(CAPS, { ...CAPS, maximumActionsPerHour: 0 }), true);
});

test("equal values, finite decreases, and enabling a disabled cap do not widen", () => {
    assert.equal(limitCapsWiden(CAPS, CAPS), false);
    assert.equal(limitCapsWiden(CAPS, { ...CAPS, perTransactionMaximum: 9n }), false);
    assert.equal(limitCapsWiden({ ...CAPS, perDayMaximum: 0n }, CAPS), false);
    assert.equal(limitCapsWiden({ ...CAPS, maximumActionsPerHour: 0 }, CAPS), false);
});

test("mixed transitions retain the widening classification", () => {
    assert.equal(limitCapsWiden(CAPS, { ...CAPS, perTransactionMaximum: 5n, perDayMaximum: 31n }), true);
});

test("enabled volume periods must remain ordered while zero periods are skipped", () => {
    assert.doesNotThrow(() => validateLimitCaps(CAPS));
    assert.doesNotThrow(() => validateLimitCaps({ ...CAPS, perHourMaximum: 0n }));
    assert.throws(() => validateLimitCaps({ ...CAPS, perHourMaximum: 9n }), /non-decreasing/u);
    assert.throws(() => validateLimitCaps({ ...CAPS, perDayMaximum: 60n }), /non-decreasing/u);
});

test("cap storage bounds are checked before encoding", () => {
    assert.throws(() => validateLimitCaps({ ...CAPS, perTransactionMaximum: -1n }), /u64/u);
    assert.throws(() => validateLimitCaps({ ...CAPS, perThirtyDayMaximum: 1n << 64n }), /u64/u);
    assert.throws(() => validateLimitCaps({ ...CAPS, maximumActionsPerHour: 1.5 }), /u32/u);
    assert.throws(() => validateLimitCaps({ ...CAPS, maximumActionsPerDay: 0x1_0000_0000 }), /u32/u);
});

test("enabled action periods remain ordered", () => {
    assert.throws(() => validateLimitCaps({ ...CAPS, maximumActionsPerHour: 4 }), /exceeds/u);
    assert.doesNotThrow(() => validateLimitCaps({ ...CAPS, maximumActionsPerDay: 0 }));
});

test("semantic payload preserves identity and encodes each cap at its canonical offset", () => {
    const account = new Uint8Array(184);
    account[11] = 3;
    account.fill(7, 16, 48);
    account.fill(9, 48, 80);
    const payload = limitPolicyPayload(account, CAPS);
    const view = new DataView(payload.buffer);
    assert.equal(payload.length, 113);
    assert.deepEqual(payload.subarray(0, 32), new Uint8Array(32).fill(7));
    assert.equal(payload[32], 3);
    assert.deepEqual(payload.subarray(33, 65), new Uint8Array(32).fill(9));
    assert.equal(view.getBigUint64(65, true), 10n);
    assert.equal(view.getBigUint64(73, true), 20n);
    assert.equal(view.getBigUint64(81, true), 30n);
    assert.equal(view.getBigUint64(89, true), 40n);
    assert.equal(view.getBigUint64(97, true), 50n);
    assert.equal(view.getUint32(105, true), 2);
    assert.equal(view.getUint32(109, true), 3);
});

test("headers, padding, stored caps, status, and reserved storage are excluded from payload input", () => {
    const account = new Uint8Array(184);
    const expected = limitPolicyPayload(account, CAPS);
    account.fill(255, 0, 11);
    account.fill(255, 12, 16);
    account.fill(255, 80);
    assert.deepEqual(limitPolicyPayload(account, CAPS), expected);
});

test("unexpected account widths are rejected", () => {
    assert.throws(() => limitPolicyPayload(new Uint8Array(183), CAPS), /account size/u);
    assert.throws(() => limitPolicyPayload(new Uint8Array(185), CAPS), /account size/u);
});
