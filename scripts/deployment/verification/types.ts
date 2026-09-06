import type { ChanceryCollateralSymbol } from "../../../config/network.js";

export interface PublicIntegrationOptions {
    readonly mode?: "all" | "setup" | "test";
    readonly symbol?: ChanceryCollateralSymbol | "all";
    readonly amount?: bigint;
}
