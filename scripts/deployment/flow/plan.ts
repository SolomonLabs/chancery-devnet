import type { DeploymentStep } from "./types.js";

export function deploymentScriptStep(label: string, script: string, runtimeConfigPath: string,
    arguments_: readonly string[] = []): DeploymentStep {
    return { label, command: "node", arguments: ["--import", "tsx", script, "devnet", "--config", runtimeConfigPath, ...arguments_] };
}

export function deploymentPlan(runtimeConfigPath: string, programKeypairPath: string,
    verificationLamports: bigint): readonly DeploymentStep[] {
    const script = (label: string, name: string, arguments_: readonly string[] = []) =>
        deploymentScriptStep(label, "scripts/deployment/" + name, runtimeConfigPath, arguments_);
    return [
        { label: "Rust toolchain", command: "cargo", arguments: ["--version"] },
        { label: "Solana CLI", command: "solana", arguments: ["--version"] },
        { label: "Solana keygen", command: "solana-keygen", arguments: ["--version"] },
        { label: "SBF toolchain", command: "cargo", arguments: ["build-sbf", "--help"] },
        script("Prepare deployment identities", "PrepareDeployment.ts"),
        { label: "Stamp both program identities", command: "node", arguments: [
            "scripts/identity/StampProgramIdentity.mjs", "--program-keypair", programKeypairPath,
        ] },
        { label: "Generate IDL and client", command: "cargo", arguments: ["xtask", "codegen"] },
        { label: "Typecheck deployment client", command: "node", arguments: ["node_modules/typescript/bin/tsc", "--noEmit"] },
        { label: "Rust tests", command: "cargo", arguments: ["xtask", "test"] },
        { label: "Build Chancery", command: "cargo", arguments: ["xtask", "build"] },
        { label: "Build faucet controller", command: "cargo", arguments: ["xtask", "build", "faucet"] },
        script("Create test assets", "CreateTestAssets.ts"),
        script("Launch and record issued mint", "LaunchIssuedToken.ts"),
        script("Verify issued mint", "LaunchIssuedToken.ts", ["--verify-only"]),
        script("Deploy Chancery", "Deploy.ts"),
        script("Initialize Chancery", "InitializeChancery.ts"),
        script("Configure Chancery", "ConfigureChancery.ts"),
        script("Deploy faucet and hand over test mints", "DeployFaucet.ts"),
        script("Hand over Chancery authorities", "HandoffAuthorities.ts"),
        script("Bootstrap collateral and pathways", "BootstrapPolicies.ts"),
        script("Verify public integration", "VerifyPublicIntegration.ts", ["--verification-lamports", verificationLamports.toString()]),
    ];
}
