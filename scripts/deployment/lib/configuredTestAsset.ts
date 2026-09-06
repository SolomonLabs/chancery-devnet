import { PublicKey } from "@solomon-labs/publickey";

import type { ChanceryCollateralSymbol, ChanceryNetwork } from "../../../config/network.js";
import { TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from "../../../runtime/token.js";
import { publicKeyFromAddress } from "./bootstrap.js";
import { testAssetProfileForSymbol, type TestAssetProfile } from "./testAssetProfiles.js";

export interface ConfiguredTestAsset {
    readonly symbol: ChanceryCollateralSymbol;
    readonly mint: PublicKey;
    readonly tokenProgram: PublicKey;
    readonly profile: TestAssetProfile;
}

export function collateralSymbol(value: string): ChanceryCollateralSymbol {
    switch (value) {
        case "USDC": case "USDT": case "USDG": case "PYUSD": return value;
        default: throw new Error("unsupported collateral symbol " + value);
    }
}

export function configuredTestAsset(network: ChanceryNetwork, symbol: ChanceryCollateralSymbol): ConfiguredTestAsset {
    const configured = network.collateralMints?.find((entry) => entry.symbol === symbol);
    if (configured === undefined) throw new Error(symbol + " is not configured; run yarn assets:create");
    const profile = testAssetProfileForSymbol(symbol);
    if (configured.tokenProgram !== profile.tokenProgramKind || configured.transferHook !== profile.transferHook) {
        throw new Error(symbol + " configuration differs from its test-asset profile");
    }
    return { symbol, mint: publicKeyFromAddress(configured.mint), profile,
        tokenProgram: profile.tokenProgramKind === "spl-token" ? TOKEN_PROGRAM_ID : TOKEN_2022_PROGRAM_ID };
}
