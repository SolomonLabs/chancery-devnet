import assert from "node:assert/strict";
import { mkdtempSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { resolve } from "node:path";
import test from "node:test";

import { parseSolanaKeypairSource } from "../../../core/SolanaKeypairFile.mjs";
import { loadOrGenerateSigner, writeGeneratedSigner } from "../signerFile.js";
import { generateSigner, signerFromSolanaKeypairBytes } from "../solanaSigner.js";

test("generated deployment keypairs are persisted and reused", (t) => {
    const directory = mkdtempSync(resolve(tmpdir(), "chancery-signer-"));
    t.after(() => rmSync(directory, { recursive: true, force: true }));
    const path = resolve(directory, "mint.json");
    const first = loadOrGenerateSigner(path, "mint");
    const contents = readFileSync(path, "utf8");
    const second = loadOrGenerateSigner(path, "mint");
    assert.equal(first.publicKey.toBase58(), second.publicKey.toBase58());
    assert.equal(readFileSync(path, "utf8"), contents);
    assert.equal(signerFromSolanaKeypairBytes(parseSolanaKeypairSource(contents, path)).publicKey.toBase58(), first.publicKey.toBase58());
    assert.ok(contents.endsWith("\n"));
    if (process.platform !== "win32") {
        assert.equal(statSync(path).mode & 0o777, 0o600);
        assert.equal(statSync(directory).mode & 0o777, 0o700);
    }
});

test("an invalid existing keypair is rejected and preserved", (t) => {
    const directory = mkdtempSync(resolve(tmpdir(), "chancery-signer-"));
    t.after(() => rmSync(directory, { recursive: true, force: true }));
    const path = resolve(directory, "mint.json");
    writeFileSync(path, "[]\n");
    assert.throws(() => loadOrGenerateSigner(path, "mint"));
    assert.equal(readFileSync(path, "utf8"), "[]\n");
});

test("writing a generated signer refuses to overwrite an existing file", (t) => {
    const directory = mkdtempSync(resolve(tmpdir(), "chancery-signer-"));
    t.after(() => rmSync(directory, { recursive: true, force: true }));
    const path = resolve(directory, "mint.json");
    const signer = generateSigner();
    writeGeneratedSigner(path, signer);
    const contents = readFileSync(path, "utf8");
    assert.throws(() => writeGeneratedSigner(path, generateSigner()), /EEXIST/);
    assert.equal(readFileSync(path, "utf8"), contents);
});
