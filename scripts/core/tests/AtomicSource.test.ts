import assert from "node:assert/strict";
import { mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { resolve } from "node:path";
import test from "node:test";

import { replaceSourceAtomically } from "../AtomicSource.js";

test("atomic source replacement preserves complete content and removes its temporary file", (t) => {
    const root = mkdtempSync(resolve(tmpdir(), "chancery-source-"));
    t.after(() => rmSync(root, { recursive: true, force: true }));
    const path = resolve(root, "identity.ts");
    writeFileSync(path, "old\n");
    replaceSourceAtomically(path, "new\n");
    assert.equal(readFileSync(path, "utf8"), "new\n");
    assert.deepEqual(readdirSync(root), ["identity.ts"]);
});

test("atomic source replacement also creates a new source file", (t) => {
    const root = mkdtempSync(resolve(tmpdir(), "chancery-source-"));
    t.after(() => rmSync(root, { recursive: true, force: true }));
    const path = resolve(root, "identity.ts");
    replaceSourceAtomically(path, "created\n");
    assert.equal(readFileSync(path, "utf8"), "created\n");
});
