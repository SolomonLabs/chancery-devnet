import { deploymentScriptStep } from "../flow/plan.js";
import type { DeploymentStep } from "../flow/types.js";
import { TEST_ASSET_PROFILES } from "../lib/testAssetProfiles.js";
import type { PublicIntegrationOptions } from "./types.js";

export function publicIntegrationPlan(runtimeConfigPath: string, keypairPath: string,
    options: PublicIntegrationOptions = {}): readonly DeploymentStep[] {
    const steps: DeploymentStep[] = [];
    const mode = options.mode ?? "all";
    for (const asset of TEST_ASSET_PROFILES) {
        if (asset.role !== "collateral" || (options.symbol !== undefined && options.symbol !== "all" && options.symbol !== asset.symbol)) continue;
        const caller = ["--keypair", keypairPath, "--symbol", asset.symbol];
        const amount = options.amount ?? 10n ** BigInt(asset.decimals);
        if (amount <= 0n || amount > (1n << 64n) - 1n || (mode !== "test" && amount > 10_000n * 10n ** BigInt(asset.decimals))) {
            throw new Error("integration amount must fit u64 and the 10,000-token faucet cap for " + asset.symbol);
        }
        const amountArguments = [...caller, "--amount", amount.toString()];
        if (mode !== "test") {
            steps.push(
                deploymentScriptStep(asset.symbol + " public onboarding", "scripts/devnet/Onboard.ts", runtimeConfigPath, caller),
                deploymentScriptStep(asset.symbol + " faucet", "scripts/deployment/FaucetMint.ts", runtimeConfigPath, amountArguments),
            );
        }
        if (mode !== "setup") {
            steps.push(deploymentScriptStep(asset.symbol + " mint and redeem", "scripts/deployment/SettlementRoundTrip.ts", runtimeConfigPath, amountArguments));
        }
    }
    return steps;
}
