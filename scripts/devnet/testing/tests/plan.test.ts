import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import test from "node:test";

import { workspaceRoot } from "../../../../config/network.js";
import { executeDeploymentPlan } from "../../../deployment/flow/execute.js";
import { parseTesterArguments } from "../arguments.js";
import { testerWorkflowPlan } from "../plan.js";

const runtime = "/workspace/config/runtime/devnet.json";
const keypair = "/workspace/.devnet/alice/keypair.json";

test("the complete tester workflow creates, funds, onboards, faucets, and settles in order", () => {
    const steps = testerWorkflowPlan(runtime, keypair, parseTesterArguments([], "integration"));
    assert.equal(steps.length, 14);
    assert.deepEqual(steps.slice(0, 2).map((step) => step.label), ["Prepare tester wallet", "Fund tester wallet"]);
    assert.ok(steps[0].arguments.includes("--reuse"));
    assert.deepEqual(steps.slice(2).map((step) => step.label), ["USDC", "USDT", "USDG", "PYUSD"].flatMap((symbol) => [
        symbol + " public onboarding", symbol + " faucet", symbol + " mint and redeem",
    ]));
    for (const step of steps) {
        assert.equal(step.command, "node");
        assert.equal(step.arguments[step.arguments.indexOf("--keypair") + 1], keypair);
        assert.equal(step.arguments.includes("--setup"), false);
        assert.ok(existsSync(resolve(workspaceRoot(), step.arguments[2])));
    }
    for (const step of steps.slice(1)) assert.equal(step.arguments[step.arguments.indexOf("--config") + 1], runtime);
});

test("setup grants and faucets only the selected collateral", () => {
    const options = parseTesterArguments(["--mode", "setup", "--symbol", "PYUSD", "--amount", "7000000"], "integration");
    const steps = testerWorkflowPlan(runtime, keypair, options);
    assert.deepEqual(steps.map((step) => step.label), ["PYUSD public onboarding", "PYUSD faucet"]);
    assert.equal(steps[1].arguments[steps[1].arguments.indexOf("--amount") + 1], "7000000");
});

test("test mode reuses the existing wallet and settles without granting or faucet calls", () => {
    const options = parseTesterArguments(["--mode", "test", "--symbol", "USDT"], "integration");
    const steps = testerWorkflowPlan(runtime, keypair, options);
    assert.equal(steps.length, 1);
    assert.equal(steps[0].label, "USDT mint and redeem");
    assert.equal(steps[0].arguments[2], "scripts/deployment/SettlementRoundTrip.ts");
});

test("all mode propagates a SOL target and skips the SOL faucet explicitly", () => {
    const options = parseTesterArguments(["--skip-airdrop", "--lamports", "250000000", "--symbol", "USDC"], "integration");
    const steps = testerWorkflowPlan(runtime, keypair, options);
    assert.equal(steps.length, 5);
    assert.equal(steps[1].arguments[steps[1].arguments.indexOf("--lamports") + 1], "250000000");
    assert.ok(steps[1].arguments.includes("--skip-airdrop"));
    assert.equal(steps[3].label, "USDC faucet");
});

test("oversized faucet requests fail before the workflow creates or funds a wallet", () => {
    const options = parseTesterArguments(["--amount", "10000000001"], "integration");
    assert.throws(() => testerWorkflowPlan(runtime, keypair, options), /faucet cap/);
});

test("a tester funding failure prevents onboarding and token issuance", () => {
    const steps = testerWorkflowPlan(runtime, keypair, parseTesterArguments([], "integration"));
    const observed: string[] = [];
    const failure = new Error("airdrop unavailable");
    assert.throws(() => executeDeploymentPlan(steps, (step) => {
        observed.push(step.label);
        if (step.label === "Fund tester wallet") throw failure;
    }), (error: unknown) => error === failure);
    assert.deepEqual(observed, ["Prepare tester wallet", "Fund tester wallet"]);
});

test("a settlement failure stops subsequent assets", () => {
    const steps = testerWorkflowPlan(runtime, keypair, parseTesterArguments([], "integration"));
    const observed: string[] = [];
    const failure = new Error("pathway paused");
    assert.throws(() => executeDeploymentPlan(steps, (step) => {
        observed.push(step.label);
        if (step.label === "USDC mint and redeem") throw failure;
    }), (error: unknown) => error === failure);
    assert.equal(observed.length, 5);
    assert.equal(observed.includes("USDT public onboarding"), false);
});

test("every tester package command references a real first-party script", () => {
    const packageJson = JSON.parse(readFileSync(resolve(workspaceRoot(), "package.json"), "utf8")) as { scripts: Record<string, string> };
    for (const command of ["tester:new", "tester:fund", "tester:setup", "tester:test", "tester:all"]) {
        const script = packageJson.scripts[command];
        assert.ok(script.startsWith("node --import tsx scripts/devnet/testing/commands/"));
        assert.ok(existsSync(resolve(workspaceRoot(), script.split(" ")[3])));
    }
});

test("settlement-only mode accepts amounts above the faucet cap", () => {
    const options = parseTesterArguments(["--mode", "test", "--symbol", "USDC", "--amount", "10000000001"], "integration");
    const steps = testerWorkflowPlan(runtime, keypair, options);
    assert.equal(steps.length, 1);
    assert.equal(steps[0].arguments[steps[0].arguments.indexOf("--amount") + 1], "10000000001");
});
