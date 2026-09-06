import { deploymentScriptStep } from "../../deployment/flow/plan.js";
import type { DeploymentStep } from "../../deployment/flow/types.js";
import { publicIntegrationPlan } from "../../deployment/verification/plan.js";
import type { TesterArguments } from "./types.js";

export function testerWorkflowPlan(runtimeConfigPath: string, keypairPath: string,
    options: TesterArguments): readonly DeploymentStep[] {
    const integration = publicIntegrationPlan(runtimeConfigPath, keypairPath, {
        mode: options.mode, symbol: options.symbol, amount: options.amount,
    });
    if (options.mode !== "all") return integration;
    return [
        { label: "Prepare tester wallet", command: "node", arguments: [
            "--import", "tsx", "scripts/devnet/testing/commands/CreateWallet.ts", "devnet",
            "--keypair", keypairPath, "--reuse",
        ] },
        deploymentScriptStep("Fund tester wallet", "scripts/devnet/testing/commands/FundWallet.ts", runtimeConfigPath, [
            "--keypair", keypairPath, "--lamports", options.minimumLamports.toString(),
            ...(options.allowAirdrop ? [] : ["--skip-airdrop"]),
        ]),
        ...integration,
    ];
}
