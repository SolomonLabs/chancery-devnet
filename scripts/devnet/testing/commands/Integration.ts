import { executeDeploymentPlan } from "../../../deployment/flow/execute.js";
import { activeNetwork, issuedTokenMint, preflight } from "../../../deployment/lib/bootstrap.js";
import { configuredTestAsset } from "../../../deployment/lib/configuredTestAsset.js";
import { parseTesterArguments } from "../arguments.js";
import { testerKeypairPath } from "../keypairPath.js";
import { testerWorkflowPlan } from "../plan.js";
import { testerWalletAddress } from "../wallet.js";

function main(): void {
    const options = parseTesterArguments(process.argv.slice(2), "integration");
    preflight();
    const network = activeNetwork();
    issuedTokenMint();
    const symbols = options.symbol === "all" ? ["USDC", "USDT", "USDG", "PYUSD"] as const : [options.symbol];
    for (const symbol of symbols) configuredTestAsset(network, symbol);
    const keypairPath = testerKeypairPath(options.keypairPath);
    const steps = testerWorkflowPlan(network.runtimeConfigPath, keypairPath, options);
    if (options.mode !== "all") console.log("tester wallet: " + testerWalletAddress(keypairPath));
    executeDeploymentPlan(steps);
    console.log("Tester " + options.mode + " completed for " + symbols.join(", ") + ".");
}

try {
    main();
} catch (error: unknown) {
    console.error(error instanceof Error ? error.stack ?? error.message : String(error));
    process.exitCode = 1;
}
