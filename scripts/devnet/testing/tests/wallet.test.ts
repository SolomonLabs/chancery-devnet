import assert from "node:assert/strict";
import { existsSync, mkdtempSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { homedir, tmpdir } from "node:os";
import { resolve } from "node:path";
import test from "node:test";

import { workspaceRoot } from "../../../../config/network.js";
import { parseSolanaKeypairSource } from "../../../core/SolanaKeypairFile.mjs";
import { loadSignerFile } from "../../../deployment/lib/signerFile.js";
import { testerKeypairPath } from "../keypairPath.js";
import { createTesterWallet, testerWalletAddress } from "../wallet.js";

test("tester paths resolve separately from operator keys and support home paths", () => {
    assert.equal(testerKeypairPath(null), resolve(workspaceRoot(), ".devnet/tester/keypair.json"));
    assert.equal(testerKeypairPath("~/tester.json"), resolve(homedir(), "tester.json"));
    assert.equal(testerKeypairPath("wallets/tester.json"), resolve("wallets/tester.json"));
});

test("tester creation writes a valid private keypair and explicit reuse preserves it", (context) => {
    const directory = mkdtempSync(resolve(tmpdir(), "chancery-tester-"));
    context.after(() => rmSync(directory, { recursive: true, force: true }));
    const path = resolve(directory, "keypair.json");
    const address = createTesterWallet(path, false);
    const source = readFileSync(path, "utf8");
    assert.equal(parseSolanaKeypairSource(source, path).length, 64);
    assert.equal(testerWalletAddress(path), address);
    assert.equal(createTesterWallet(path, true), address);
    assert.equal(readFileSync(path, "utf8"), source);
    assert.ok(source.endsWith("\n"));
    assert.equal(source.includes("\r"), false);
    if (process.platform !== "win32") assert.equal(statSync(path).mode & 0o777, 0o600);
    const signer = loadSignerFile(path);
    assert.equal(signer.publicKey.toBase58(), address);
    assert.ok(signer.seed.some((byte) => byte !== 0));
    signer.seed.fill(0);
});

test("tester:new preserves existing keypairs on refusal", (context) => {
    const directory = mkdtempSync(resolve(tmpdir(), "chancery-tester-"));
    context.after(() => rmSync(directory, { recursive: true, force: true }));
    const path = resolve(directory, "keypair.json");
    createTesterWallet(path, false);
    const source = readFileSync(path, "utf8");
    assert.throws(() => createTesterWallet(path, false), /EEXIST/);
    assert.equal(readFileSync(path, "utf8"), source);
});

test("invalid existing tester keypairs are preserved and rejected", (context) => {
    const directory = mkdtempSync(resolve(tmpdir(), "chancery-tester-"));
    context.after(() => rmSync(directory, { recursive: true, force: true }));
    const path = resolve(directory, "keypair.json");
    writeFileSync(path, "[]\n");
    assert.throws(() => createTesterWallet(path, true));
    assert.throws(() => testerWalletAddress(path));
    assert.equal(readFileSync(path, "utf8"), "[]\n");
});

test("reading a missing tester wallet does not create one", (context) => {
    const directory = mkdtempSync(resolve(tmpdir(), "chancery-tester-"));
    context.after(() => rmSync(directory, { recursive: true, force: true }));
    const path = resolve(directory, "missing.json");
    assert.throws(() => testerWalletAddress(path), /ENOENT/);
    assert.equal(existsSync(path), false);
});

test("the all workflow creates a missing tester keypair in reuse mode", (context) => {
    const directory = mkdtempSync(resolve(tmpdir(), "chancery-tester-"));
    context.after(() => rmSync(directory, { recursive: true, force: true }));
    const path = resolve(directory, "new", "keypair.json");
    const address = createTesterWallet(path, true);
    assert.equal(testerWalletAddress(path), address);
});
