import assert from "node:assert/strict";
import test from "node:test";

import { executeDeploymentPlan, runDeploymentStep } from "../execute.js";
import type { DeploymentStep } from "../types.js";

const steps: readonly DeploymentStep[] = [
    { label: "first", command: "node", arguments: [] },
    { label: "second", command: "node", arguments: [] },
    { label: "third", command: "node", arguments: [] },
];

test("deployment executes each step exactly once in order", () => {
    const observed: string[] = [];
    executeDeploymentPlan(steps, (step) => { observed.push(step.label); });
    assert.deepEqual(observed, ["first", "second", "third"]);
});

test("a deployment failure stops later steps and is propagated unchanged", () => {
    const observed: string[] = [];
    const failure = new Error("build failed");
    assert.throws(() => executeDeploymentPlan(steps, (step) => {
        observed.push(step.label);
        if (step.label === "second") throw failure;
    }), (error: unknown) => error === failure);
    assert.deepEqual(observed, ["first", "second"]);
});

test("a successful child process is accepted", () => {
    assert.doesNotThrow(() => runDeploymentStep({ label: "success", command: "node", arguments: ["-e", "process.exit(0)"] }));
});

test("a nonzero child exit is reported with its stage and exit status", () => {
    assert.throws(() => runDeploymentStep({ label: "failed child", command: "node", arguments: ["-e", "process.exit(7)"] }), /failed child failed with exit status 7/);
});
