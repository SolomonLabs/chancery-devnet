import { resolveNetwork } from "../../config/network.js";
import { parseDeploymentArguments } from "./flow/arguments.js";
import { executeDeploymentPlan } from "./flow/execute.js";
import { deploymentPlan } from "./flow/plan.js";
import { assertSupportedNodeVersion } from "./lib/processEnvironment.js";

function main(): void {
    const options = parseDeploymentArguments(process.argv.slice(2));
    assertSupportedNodeVersion();
    const network = resolveNetwork("devnet", options.runtimeArguments);
    executeDeploymentPlan(deploymentPlan(network.runtimeConfigPath, network.programKeypairPath, options.verificationLamports));
    console.log("\nDevnet deployment and fresh-wallet public integration checks passed.");
}

try {
    main();
} catch (error) {
    console.error(error instanceof Error ? error.stack ?? error.message : String(error));
    process.exitCode = 1;
}
