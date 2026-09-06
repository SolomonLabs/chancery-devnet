import type { ChanceryNetworkIdentity } from "../network.js";

// Devnet collateral uses the configured SPL Token or Token-2022 profile.
export const devnetNetworkIdentity: ChanceryNetworkIdentity = {
    name: "devnet",
    cluster: "devnet",
    programId: "3doMTb5u94mzTDoBbyJXbZscNE3suuQe75ybYmirKute",
    faucetProgramId: "6JgE46kwvTqPpY7s2DDjFDvSqWdmfm9puoDBRqHubn85",
    usdvMint: "FEA2m8pnzXLXdh3irYhJPKgmLhK23sdSgB1KjmXqyVqz",
    usdcMint: "A3wGiSTtQLaEBLnYMSDUUgrSzs3b9gQpyah1sVowHFce",
    legacyUsdvMint: "J2EMwPmXnG4g7hzoSe5VFo1yiwg23VJT3UaQmW8AadAR",
    collateralMints: [
        {
            symbol: "USDC",
            mint: "A3wGiSTtQLaEBLnYMSDUUgrSzs3b9gQpyah1sVowHFce",
            tokenProgram: "spl-token",
            transferHook: "absent",
        },
        {
            symbol: "USDT",
            mint: "G4WrQJD2nu61VuCVv3wpZt4FB92Hb3UJPMU4kMdvoTz7",
            tokenProgram: "spl-token",
            transferHook: "absent",
        },
        {
            symbol: "USDG",
            mint: "4y7HzfopqgJp87S6uyuMaMy3TB6ME1oqQkFpZZoRvtnj",
            tokenProgram: "token-2022",
            transferHook: "dormant-only",
        },
        {
            symbol: "PYUSD",
            mint: "BLCaBUQ591XkddMF7HZVwpFayutrW5wfdYBxkvL3w2r8",
            tokenProgram: "token-2022",
            transferHook: "dormant-only",
        },
    ],
};
