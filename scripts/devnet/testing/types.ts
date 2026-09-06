import type { ChanceryCollateralSymbol } from "../../../config/network.js";

export type TesterCommand = "wallet" | "funding" | "integration";
export type TesterMode = "all" | "setup" | "test";

export interface TesterArguments {
    readonly keypairPath: string | null;
    readonly runtimeArguments: readonly string[];
    readonly reuseWallet: boolean;
    readonly mode: TesterMode;
    readonly symbol: ChanceryCollateralSymbol | "all";
    readonly amount: bigint | undefined;
    readonly minimumLamports: bigint;
    readonly allowAirdrop: boolean;
}
