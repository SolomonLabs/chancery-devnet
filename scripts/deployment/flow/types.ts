export interface DeploymentStep {
    readonly label: string;
    readonly command: "node" | "cargo" | "solana" | "solana-keygen";
    readonly arguments: readonly string[];
}

export interface DeploymentArguments {
    readonly runtimeArguments: readonly string[];
    readonly verificationLamports: bigint;
}

export type DeploymentStepExecutor = (step: DeploymentStep) => void;
