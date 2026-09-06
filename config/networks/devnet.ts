import type { ChanceryNetworkIdentity } from "../network.js";

// Program identities are stamped from the deployment keypairs.
// Test collateral is created under setup authority and handed to the faucet PDA.
export const devnetNetworkIdentity: ChanceryNetworkIdentity = {
    name: "devnet",
    cluster: "devnet",
    programId: "11111111111111111111111111111111",
    faucetProgramId: "11111111111111111111111111111111",
    usdvMint: null,
    usdcMint: null,
    legacyUsdvMint: null,
    collateralMints: [],
};
