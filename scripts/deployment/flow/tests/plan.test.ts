import assert from "node:assert/strict";
import test from "node:test";

import { deploymentPlan } from "../plan.js";

const configuration = "/workspace/config/runtime/custom config.json";
const programKeypair = "/workspace/private/program.json";

test("deployment orders configuration writes, builds, provisioning, and verification", () => {
    const labels = deploymentPlan(configuration, programKeypair, 100_000_000n).map((step) => step.label);
    assert.deepEqual(labels, [
        "Rust toolchain", "Solana CLI", "Solana keygen", "SBF toolchain",
        "Prepare deployment identities", "Stamp both program identities", "Generate IDL and client",
        "Typecheck deployment client", "Rust tests", "Build Chancery", "Build faucet controller",
        "Create test assets", "Launch and record issued mint", "Verify issued mint", "Deploy Chancery",
        "Initialize Chancery", "Configure Chancery", "Deploy faucet and hand over test mints",
        "Hand over Chancery authorities", "Bootstrap collateral and pathways", "Verify public integration",
    ]);
});

test("each deployment command reloads the selected runtime configuration in its own process", () => {
    const steps = deploymentPlan(configuration, programKeypair, 100_000_000n);
    const scripts = steps.filter((step) => step.arguments[0] === "--import");
    assert.equal(scripts.length, 11);
    for (const step of scripts) {
        assert.equal(step.command, "node");
        assert.deepEqual(step.arguments.slice(0, 2), ["--import", "tsx"]);
        assert.deepEqual(step.arguments.slice(3, 6), ["devnet", "--config", configuration]);
    }
});

test("identity stamping uses the configured program keypair", () => {
    const step = deploymentPlan(configuration, programKeypair, 100_000_000n).find((entry) => entry.label === "Stamp both program identities");
    assert.deepEqual(step?.arguments, ["scripts/identity/StampProgramIdentity.mjs", "--program-keypair", programKeypair]);
});

test("final public verification receives the requested funding budget", () => {
    const steps = deploymentPlan(configuration, programKeypair, 150_000_000n);
    assert.deepEqual(steps[steps.length - 1]?.arguments.slice(-2), ["--verification-lamports", "150000000"]);
});
