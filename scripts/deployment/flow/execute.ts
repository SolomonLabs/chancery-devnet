import { spawnSync } from "node:child_process";

import { workspaceRoot } from "../../../config/network.js";
import { sanitizedProcessEnvironment } from "../lib/processEnvironment.js";
import type { DeploymentStep, DeploymentStepExecutor } from "./types.js";

export function runDeploymentStep(step: DeploymentStep): void {
    const command = step.command === "node" ? process.execPath : step.command;
    const result = spawnSync(command, [...step.arguments], {
        cwd: workspaceRoot(), stdio: "inherit", env: sanitizedProcessEnvironment(),
    });
    if (result.error !== undefined) throw new Error(step.label + " could not start", { cause: result.error });
    if (result.signal !== null) throw new Error(step.label + " terminated by " + result.signal);
    if (result.status !== 0) throw new Error(step.label + " failed with exit status " + result.status);
}

export function executeDeploymentPlan(steps: readonly DeploymentStep[],
    execute: DeploymentStepExecutor = runDeploymentStep): void {
    for (const step of steps) {
        console.log("\n" + step.label);
        execute(step);
    }
}
