import type { ChanceryCollateralSymbol } from "../../../config/network.js";

/**
 * The fixed shape of each devnet test asset.
 *
 * The collateral assets mirror the token-program and transfer-hook posture of
 * their mainnet counterparts so the settlement paths exercise the same
 * branches. USDV-LEGACY is not collateral: it is the migration source, which
 * predates Chancery and carries 9 decimals against the issued token's 6
 * (migrate_legacy_to_token2022 floors 9 -> 6).
 *
 * All of them are devnet mints with no value, and the setup authority holds
 * mint authority on every one, so more can be issued at any time.
 */
export type TestAssetSymbol = ChanceryCollateralSymbol | "USDV-LEGACY";

export interface TestAssetProfile {
    readonly symbol: TestAssetSymbol;
    readonly role: "collateral" | "legacy-migration-source";
    readonly decimals: number;
    readonly tokenProgramKind: "spl-token" | "token-2022";
    readonly transferHook: "absent" | "dormant-only";
}

export const TEST_ASSET_PROFILES: readonly TestAssetProfile[] = [
    {
        symbol: "USDC",
        role: "collateral",
        decimals: 6,
        tokenProgramKind: "spl-token",
        transferHook: "absent",
    },
    {
        symbol: "USDT",
        role: "collateral",
        decimals: 6,
        tokenProgramKind: "spl-token",
        transferHook: "absent",
    },
    {
        symbol: "USDG",
        role: "collateral",
        decimals: 6,
        tokenProgramKind: "token-2022",
        transferHook: "dormant-only",
    },
    {
        symbol: "PYUSD",
        role: "collateral",
        decimals: 6,
        tokenProgramKind: "token-2022",
        transferHook: "dormant-only",
    },
    {
        symbol: "USDV-LEGACY",
        role: "legacy-migration-source",
        decimals: 9,
        tokenProgramKind: "spl-token",
        transferHook: "absent",
    },
];

export function testAssetProfileForSymbol(symbol: TestAssetSymbol): TestAssetProfile {
    const profile = TEST_ASSET_PROFILES.find((entry) => entry.symbol === symbol);
    if (profile === undefined) throw new Error(`unsupported test asset symbol ${symbol}`);
    return profile;
}
