import assert from "node:assert/strict";
import { generateKeyPairSync } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, resolve } from "node:path";
import test from "node:test";

import { ed25519PublicKeyFromSolanaKeypair } from "../../core/Ed25519.mjs";
import { stampProgramIdentities } from "../ProgramIdentity.mjs";

function keypair() {
    const pair = generateKeyPairSync("ed25519");
    const seed = pair.privateKey.export({ type: "pkcs8", format: "der" }).subarray(-32);
    const publicKey = pair.publicKey.export({ type: "spki", format: "der" }).subarray(-32);
    const bytes = new Uint8Array(Buffer.concat([seed, publicKey]));
    return { source: JSON.stringify(Array.from(bytes)), address: ed25519PublicKeyFromSolanaKeypair(bytes) };
}

function fixture(t) {
    const root = mkdtempSync(resolve(tmpdir(), "chancery-identity-"));
    t.after(() => rmSync(root, { recursive: true, force: true }));
    const program = keypair();
    const faucet = keypair();
    const write = (file, source) => {
        const full = resolve(root, file);
        mkdirSync(dirname(full), { recursive: true });
        writeFileSync(full, source);
    };
    write("keys/program.json", program.source);
    write("keys/faucet.json", faucet.source);
    write("programs/chancery/src/lib.rs", 'declare_id!("' + program.address + '");\n');
    write("programs/faucet/src/lib.rs", 'declare_id!("11111111111111111111111111111111");\n');
    write("config/networks/devnet.ts", 'programId: "' + program.address + '", faucetProgramId: "11111111111111111111111111111111", usdvMint: null\n');
    write("clients/ts/src/constants.ts", 'export const PROGRAM_ID = new PublicKey("' + program.address + '");\n');
    write("programs/chancery/idl/idl.json", JSON.stringify({ address: program.address, metadata: { deployments: { devnet: program.address } } }, null, 4) + "\n");
    return { root, program, faucet, write };
}

test("stamps the faucet when Chancery already matches", async (t) => {
    const state = fixture(t);
    const result = await stampProgramIdentities(state.root, "keys/program.json", "keys/faucet.json");
    assert.equal(result.programId, state.program.address);
    assert.equal(result.faucetProgramId, state.faucet.address);
    assert.deepEqual(result.changedPaths, ["programs/faucet/src/lib.rs", "config/networks/devnet.ts"]);
    assert.ok(readFileSync(resolve(state.root, "programs/faucet/src/lib.rs"), "utf8").includes(state.faucet.address));
});

test("stamping both identities is idempotent", async (t) => {
    const state = fixture(t);
    await stampProgramIdentities(state.root, "keys/program.json", "keys/faucet.json");
    assert.deepEqual((await stampProgramIdentities(state.root, "keys/program.json", "keys/faucet.json")).changedPaths, []);
});

test("a replacement faucet keypair is stamped independently", async (t) => {
    const state = fixture(t);
    await stampProgramIdentities(state.root, "keys/program.json", "keys/faucet.json");
    const replacement = keypair();
    state.write("keys/faucet.json", replacement.source);
    const result = await stampProgramIdentities(state.root, "keys/program.json", "keys/faucet.json");
    assert.equal(result.faucetProgramId, replacement.address);
    assert.equal(result.programId, state.program.address);
    assert.ok(result.changedPaths.includes("programs/faucet/src/lib.rs"));
});

test("validates all source targets before writing", async (t) => {
    const state = fixture(t);
    const before = readFileSync(resolve(state.root, "programs/faucet/src/lib.rs"), "utf8");
    state.write("clients/ts/src/constants.ts", "export const unrelated = 1;\n");
    await assert.rejects(stampProgramIdentities(state.root, "keys/program.json", "keys/faucet.json"), /missing/);
    assert.equal(readFileSync(resolve(state.root, "programs/faucet/src/lib.rs"), "utf8"), before);
});

test("rejects identical Chancery and faucet identities", async (t) => {
    const state = fixture(t);
    state.write("keys/faucet.json", state.program.source);
    await assert.rejects(stampProgramIdentities(state.root, "keys/program.json", "keys/faucet.json"), /must differ/);
});
