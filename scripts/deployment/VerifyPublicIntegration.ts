import { resolveWorkspacePath } from "../../config/network.js";
import { createDevnetHarness } from "../../runtime/devnetHarness.js";
import { parseDeploymentArguments } from "./flow/arguments.js";
import { executeDeploymentPlan } from "./flow/execute.js";
import { createControllerContext } from "./lib/administrationContext.js";
import { activeNetwork, loadFeePayer, preflight, publicKeyFromAddress } from "./lib/bootstrap.js";
import { publicIntegrationPlan } from "./verification/plan.js";
import { createPublicVerificationWallet } from "./verification/wallet.js";

async function main(): Promise<void> {
    const options = parseDeploymentArguments(process.argv.slice(2));
    preflight();
    const network = activeNetwork();
    const payer = loadFeePayer();
    const harness = await createDevnetHarness({ rpcUrl: network.rpcUrl, commitment: "finalized", requireProgramdata: true });
    const controller = publicKeyFromAddress(network.faucetProgramId);
    const controllerAccount = await harness.banksClient.getAccount(controller);
    if (controllerAccount?.executable !== true) throw new Error("faucet controller is not executable");
    await createControllerContext(harness, payer, controller);
    const keypairPath = await createPublicVerificationWallet(harness, payer,
        resolveWorkspacePath(".devnet"), options.verificationLamports);
    executeDeploymentPlan(publicIntegrationPlan(network.runtimeConfigPath, keypairPath));
    console.log("\nFresh-wallet onboarding, faucet, mint, and redeem passed for all four collateral assets.");
}

main().catch((error: unknown) => {
    console.error(error instanceof Error ? error.stack ?? error.message : String(error));
    process.exitCode = 1;
});
