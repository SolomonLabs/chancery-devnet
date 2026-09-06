import assert from "node:assert/strict";
import test from "node:test";

import { parseDeploymentArguments } from "../arguments.js";

test("deployment defaults to the standard configuration and a 0.1 SOL verification wallet", () => {
    assert.deepEqual(parseDeploymentArguments(["devnet"]), { runtimeArguments: [], verificationLamports: 100_000_000n });
});

test("deployment preserves a custom configuration path and exact funding integer", () => {
    assert.deepEqual(parseDeploymentArguments(["devnet", "--config", "config/runtime/custom config.json", "--verification-lamports", "18446744073709551615"]), {
        runtimeArguments: ["--config", "config/runtime/custom config.json"], verificationLamports: (1n << 64n) - 1n,
    });
});

test("deployment rejects unsupported networks and flags", () => {
    for (const flag of ["mainnet", "--dry-run", "--skip-verification", "--unknown"]) {
        assert.throws(() => parseDeploymentArguments([flag]), /unknown deployment argument/);
    }
});

test("deployment rejects missing and repeated argument values", () => {
    assert.throws(() => parseDeploymentArguments(["--config"]), /requires a value/);
    assert.throws(() => parseDeploymentArguments(["--config", "--verification-lamports"]), /requires a value/);
    assert.throws(() => parseDeploymentArguments(["--config", "a", "--config", "b"]), /repeated/);
    assert.throws(() => parseDeploymentArguments(["--verification-lamports", "1", "--verification-lamports", "2"]), /repeated/);
});

test("deployment funding rejects zero, fractional, negative, and oversized amounts", () => {
    for (const amount of ["0", "-1", "1.5", "1e9", " 100", "18446744073709551616"]) {
        assert.throws(() => parseDeploymentArguments(["--verification-lamports", amount]));
    }
});
