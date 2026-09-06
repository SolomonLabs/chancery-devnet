import { ROLE, SCOPE } from "../../clients/ts/src/constants.js";
import { createDevnetHarness } from "../../runtime/devnetHarness.js";
import { createControllerContext } from "./lib/administrationContext.js";
import { registerCollateralAsset } from "./lib/assetRegistration.js";
import { DRY_RUN, activeNetwork, issuedTokenMint, preflight, publicKeyFromAddress } from "./lib/bootstrap.js";
import { collateralSymbol, configuredTestAsset } from "./lib/configuredTestAsset.js";
import { verifyIssuedTokenReadiness } from "./lib/issuedTokenReadiness.js";
import { defaultDirectPathwayId } from "./lib/pathwayIdentity.js";
import { registerDirectPathway } from "./lib/pathwayRegistration.js";
import { ensurePermission } from "./lib/permissionReadiness.js";
import { loadPublicCaller } from "./lib/publicCaller.js";

async function main(): Promise<void> {
    preflight();
    if (DRY_RUN) throw new Error("policy bootstrap requires committed state changes; --dry-run is unsupported");
    const network = activeNetwork();
    const caller = loadPublicCaller();
    const issuedMint = issuedTokenMint();
    const faucetProgramId = publicKeyFromAddress(network.faucetProgramId);
    const harness = await createDevnetHarness({ rpcUrl: network.rpcUrl, commitment: "confirmed", requireProgramdata: false });
    const context = await createControllerContext(harness, caller, faucetProgramId);
    await verifyIssuedTokenReadiness(context, issuedMint);
    for (const symbol of ["USDC", "USDT", "USDG", "PYUSD"]) {
        const asset = configuredTestAsset(network, collateralSymbol(symbol));
        await registerCollateralAsset(context, asset, faucetProgramId);
        const pathwayId = await defaultDirectPathwayId(asset.symbol, asset.mint, issuedMint);
        const pathway = await registerDirectPathway(context, pathwayId, asset.mint, issuedMint);
        await ensurePermission(context, caller.publicKey, ROLE.CAN_MINT_DIRECT | ROLE.CAN_REDEEM_DIRECT,
            { scopeKind: SCOPE.PATHWAY, scopeKey: pathway });
        console.log(symbol + " direct pathway: " + pathway.toBase58());
    }
}

main().catch((error: unknown) => {
    console.error(error instanceof Error ? error.stack ?? error.message : String(error));
    process.exitCode = 1;
});
