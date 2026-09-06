import assert from "node:assert/strict";
import test from "node:test";

import { publicIntegrationPlan } from "../plan.js";

const runtime = "/workspace/config/runtime/devnet.json";
const keypair = "/workspace/.devnet/public-integration-random/keypair.json";

test("public verification covers onboarding, faucet, and round-trip settlement for each collateral", () => {
    const steps = publicIntegrationPlan(runtime, keypair);
    assert.equal(steps.length, 12);
    for (const [index, symbol] of ["USDC", "USDT", "USDG", "PYUSD"].entries()) {
        const group = steps.slice(index * 3, index * 3 + 3);
        assert.deepEqual(group.map((step) => step.label), [symbol + " public onboarding", symbol + " faucet", symbol + " mint and redeem"]);
        assert.deepEqual(group.map((step) => step.arguments[2]), ["scripts/devnet/Onboard.ts", "scripts/deployment/FaucetMint.ts", "scripts/deployment/SettlementRoundTrip.ts"]);
    }
});

test("every public integration command uses only the fresh caller keypair", () => {
    for (const step of publicIntegrationPlan(runtime, keypair)) {
        const index = step.arguments.indexOf("--keypair");
        assert.equal(step.arguments[index + 1], keypair);
        assert.equal(step.arguments[step.arguments.indexOf("--config") + 1], runtime);
        assert.equal(step.arguments.includes("--setup"), false);
        assert.equal(step.arguments.includes("--global"), false);
    }
});

test("faucet and settlement request the same one-token amount", () => {
    const steps = publicIntegrationPlan(runtime, keypair);
    for (const step of steps.filter((entry) => !entry.label.endsWith("public onboarding"))) {
        assert.equal(step.arguments[step.arguments.indexOf("--amount") + 1], "1000000");
    }
});
