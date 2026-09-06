import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import test from "node:test";

import {
    readIssuedTokenLaunchConfig,
    validateIssuedTokenLaunchConfig,
    verifyIssuedTokenOffchainMetadata,
} from "../IssuedTokenLaunchConfig.mjs";

const CONFIGURATION_URL = new URL("../../../artifacts/issued-token-launch.json", import.meta.url);

function configuration() {
    return JSON.parse(readFileSync(CONFIGURATION_URL, "utf8"));
}

test("launch configuration reader preserves source and validated fields", async () => {
    const result = await readIssuedTokenLaunchConfig();
    assert.deepEqual(result.source, readFileSync(CONFIGURATION_URL));
    assert.deepEqual(result.config, configuration());
    assert.equal(result.configSha256, createHash("sha256").update(result.source).digest("hex"));
});

test("launch configuration validation returns the supplied configuration", () => {
    const input = configuration();
    assert.equal(validateIssuedTokenLaunchConfig(input), input);
});

test("launch configuration requires the declared schema, token program, and decimals", () => {
    assert.throws(() => validateIssuedTokenLaunchConfig({ ...configuration(), schema_version: 2 }), /schema_version/u);
    assert.throws(() => validateIssuedTokenLaunchConfig({ ...configuration(), token_program: "11111111111111111111111111111111" }), /Token-2022/u);
    assert.throws(() => validateIssuedTokenLaunchConfig({ ...configuration(), decimals: 9 }), /decimals must be 6/u);
});

test("launch metadata retains its required fields", () => {
    for (const [field, value] of [["name", "OTHER"], ["symbol", "OTHER"], ["uri", "https://other.invalid/usdv.json"], ["offchain_json_sha256", "invalid"]]) {
        const input = configuration();
        input.metadata[field] = value;
        assert.throws(() => validateIssuedTokenLaunchConfig(input));
    }
});

test("launch authority posture requires all three reservations", () => {
    for (const [field, value] of [["mode", "other"], ["metadata_updates_require_chancery_upgrade", false], ["extension_authorities_reserved_for_future_use", false]]) {
        const input = configuration();
        input.authority_posture[field] = value;
        assert.throws(() => validateIssuedTokenLaunchConfig(input));
    }
});

test("launch configuration requires every reserved extension", () => {
    for (const extension of configuration().reserved_extensions) {
        const input = configuration();
        input.reserved_extensions = input.reserved_extensions.filter((entry) => entry !== extension);
        assert.throws(() => validateIssuedTokenLaunchConfig(input), /reserved_extensions missing/u);
    }
    assert.throws(() => validateIssuedTokenLaunchConfig({ ...configuration(), reserved_extensions: null }), /must be an array/u);
});

test("reserved extension entries satisfy their declared string contract", () => {
    for (const entry of [null, 1, true, {}, [], "", " TransferHook", "TransferHook\n"]) {
        const input = configuration();
        input.reserved_extensions.push(entry);
        assert.throws(() => validateIssuedTokenLaunchConfig(input), /reserved_extensions entry/u);
    }
});

test("metadata verification checks the response against the declared launch metadata", async () => {
    const input = configuration();
    const document = { name: input.metadata.name, symbol: input.metadata.symbol };
    const source = JSON.stringify(document);
    input.metadata.offchain_json_sha256 = createHash("sha256").update(source).digest("hex");
    const result = await verifyIssuedTokenOffchainMetadata(input, async (url, options) => {
        assert.equal(url, input.metadata.uri);
        assert.equal(options.redirect, "error");
        assert.equal(options.headers.accept, "application/json");
        assert.equal(options.signal.aborted, false);
        return new Response(source, { headers: { "content-type": "application/json" } });
    });
    assert.deepEqual(result.document, document);
    assert.equal(result.observedHash, input.metadata.offchain_json_sha256);
});

test("metadata verification rejects a response whose content differs from the expected hash", async () => {
    await assert.rejects(verifyIssuedTokenOffchainMetadata(configuration(), async () => {
        return new Response("{}", { headers: { "content-type": "application/json" } });
    }), /SHA-256 mismatch/u);
});
