import assert from "node:assert/strict";
import test from "node:test";

import { parseSolanaKeypairSource } from "../SolanaKeypairFile.mjs";

const KEYPAIR = Array.from({ length: 64 }, (_, index) => index);
const SOURCE_NAME = "keypair.json";

function parse(value) {
    return parseSolanaKeypairSource(JSON.stringify(value), SOURCE_NAME);
}

test("keypair parser preserves all 64 byte values", () => {
    const values = [...KEYPAIR];
    values[63] = 255;
    assert.deepEqual(parse(values), Uint8Array.from(values));
});

test("keypair parser requires an array of exactly 64 entries", () => {
    for (const value of [null, {}, "keypair", KEYPAIR.slice(1), [...KEYPAIR, 64]]) {
        assert.throws(() => parse(value), /keypair.json must contain exactly 64 byte values/u);
    }
});

test("keypair parser rejects values before typed-array conversion", () => {
    for (const value of [-1, 256, 1.5, null, true, "1", {}, []]) {
        const values = [...KEYPAIR];
        values[17] = value;
        assert.throws(() => parse(values), /keypair.json\[17\] must be an integer from 0 through 255/u);
    }
});

test("keypair parser rejects malformed or trailing JSON", () => {
    assert.throws(() => parseSolanaKeypairSource("[", SOURCE_NAME), /keypair.json/u);
    assert.throws(() => parseSolanaKeypairSource(JSON.stringify(KEYPAIR) + " null", SOURCE_NAME), /trailing content/u);
});

test("keypair parser enforces the source byte limit", () => {
    assert.throws(() => parseSolanaKeypairSource(" ".repeat(4097) + JSON.stringify(KEYPAIR), SOURCE_NAME), /exceeds 4096 bytes/u);
});

test("each keypair parse returns independently owned bytes", () => {
    const first = parse(KEYPAIR);
    const second = parse(KEYPAIR);
    first.fill(0);
    assert.deepEqual(second, Uint8Array.from(KEYPAIR));
});
