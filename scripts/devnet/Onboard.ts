import { ROLE, SCOPE } from "../../clients/ts/src/constants.js";
import { createDevnetHarness } from "../../runtime/devnetHarness.js";
import { createControllerContext } from "../deployment/lib/administrationContext.js";
import { registerCollateralAsset } from "../deployment/lib/assetRegistration.js";
import { DRY_RUN, activeNetwork, issuedTokenMint, preflight, publicKeyFromAddress } from "../deployment/lib/bootstrap.js";
import { collateralSymbol, configuredTestAsset } from "../deployment/lib/configuredTestAsset.js";
import { verifyIssuedTokenReadiness } from "../deployment/lib/issuedTokenReadiness.js";
import { defaultDirectPathwayId, parsePathwayId } from "../deployment/lib/pathwayIdentity.js";
import { registerDirectPathway } from "../deployment/lib/pathwayRegistration.js";
import { ensurePermission } from "../deployment/lib/permissionReadiness.js";
import { flagValue, loadPublicCaller } from "../deployment/lib/publicCaller.js";

async function main(): Promise<void> {
    preflight();
    if (DRY_RUN) throw new Error("onboarding requires committed state changes; --dry-run is unsupported");
    const network = activeNetwork();
    const caller = loadPublicCaller();
    const asset = configuredTestAsset(network, collateralSymbol(flagValue("--symbol") ?? "USDC"));
    const issuedMint = issuedTokenMint();
    const requestedId = flagValue("--pathway-id");
    const pathwayId = requestedId === null
        ? await defaultDirectPathwayId(asset.symbol, asset.mint, issuedMint,
            flagValue("--keypair") === null ? undefined : caller.publicKey)
        : parsePathwayId(requestedId);
    const requestedLimitId = flagValue("--limit-policy-id");
    const limitPolicyId = requestedLimitId === null ? new Uint8Array(32) : parsePathwayId(requestedLimitId);
    const harness = await createDevnetHarness({ rpcUrl: network.rpcUrl, commitment: "confirmed", requireProgramdata: false });
    const faucetProgramId = publicKeyFromAddress(network.faucetProgramId);
    const context = await createControllerContext(harness, caller, faucetProgramId);
    await verifyIssuedTokenReadiness(context, issuedMint);
    await registerCollateralAsset(context, asset, faucetProgramId);
    const pathway = await registerDirectPathway(context, pathwayId, asset.mint, issuedMint, limitPolicyId);
    const permission = await ensurePermission(context, caller.publicKey, ROLE.CAN_MINT_DIRECT | ROLE.CAN_REDEEM_DIRECT,
        { scopeKind: SCOPE.PATHWAY, scopeKey: pathway });
    console.log("principal: " + caller.publicKey.toBase58());
    console.log("pathway id: " + Buffer.from(pathwayId).toString("hex"));
    console.log("pathway: " + pathway.toBase58());
    console.log("permission: " + permission.toBase58());
}

main().catch((error: unknown) => {
    console.error(error instanceof Error ? error.stack ?? error.message : String(error));
    process.exitCode = 1;
});
