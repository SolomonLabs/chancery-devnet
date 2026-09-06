import assert from "node:assert/strict";
import test from "node:test";

import { replaceNetworkConfigFaucetIdentity, replaceNetworkConfigIssuedMintIdentity } from "../NetworkIdentity.mjs";

const PROGRAM = "11111111111111111111111111111111";
const MINT = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";
const OTHER = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const source = 'programId: "' + PROGRAM + '", faucetProgramId: "' + PROGRAM + '", usdvMint: null, usdcMint: "' + OTHER + '"';

test("records a verified issued mint while preserving other identities", () => {
    assert.equal(replaceNetworkConfigIssuedMintIdentity(source, MINT, "identity"), source.replace("usdvMint: null", 'usdvMint: "' + MINT + '"'));
});

test("recording the same issued mint is idempotent", () => {
    const recorded = replaceNetworkConfigIssuedMintIdentity(source, MINT, "identity");
    assert.equal(replaceNetworkConfigIssuedMintIdentity(recorded, MINT, "identity"), recorded);
});

test("an established issued-mint identity cannot be replaced", () => {
    const recorded = replaceNetworkConfigIssuedMintIdentity(source, MINT, "identity");
    assert.throws(() => replaceNetworkConfigIssuedMintIdentity(recorded, OTHER, "identity"), /different issued mint/);
});

test("issued-mint replacement rejects missing and duplicate fields", () => {
    assert.throws(() => replaceNetworkConfigIssuedMintIdentity("usdcMint: null", MINT, "identity"), /exactly one/);
    assert.throws(() => replaceNetworkConfigIssuedMintIdentity(source + ", usdvMint: null", MINT, "identity"), /exactly one/);
});

test("issued-mint replacement validates base58 text", () => {
    assert.throws(() => replaceNetworkConfigIssuedMintIdentity(source, "invalid0", "identity"), /base58/);
});

test("faucet stamping changes only the faucet identity", () => {
    assert.equal(replaceNetworkConfigFaucetIdentity(source, MINT, "identity"), source.replace('faucetProgramId: "' + PROGRAM + '"', 'faucetProgramId: "' + MINT + '"'));
});

test("faucet stamping rejects duplicate and missing identities", () => {
    assert.throws(() => replaceNetworkConfigFaucetIdentity("programId: null", MINT, "identity"), /exactly one/);
    assert.throws(() => replaceNetworkConfigFaucetIdentity(source + ', faucetProgramId: "' + PROGRAM + '"', MINT, "identity"), /exactly one/);
});
