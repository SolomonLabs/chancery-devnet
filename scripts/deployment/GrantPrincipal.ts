import { readFileSync } from "node:fs";

import { PublicKey } from "@solomon-labs/publickey";

import { ROLE, SCOPE } from "../../clients/ts/src/constants.js";
import { createDevnetHarness } from "../../runtime/devnetHarness.js";
import { pathwayPolicyPda } from "../../runtime/pdas.js";
import { DEFAULT_PUBLIC_KEY } from "../../runtime/wellKnown.js";
import { createControllerContext, createSetupContext } from "./lib/administrationContext.js";
import { DRY_RUN, activeNetwork, issuedTokenMint, loadSetupAuthority, preflight, publicKeyFromAddress } from "./lib/bootstrap.js";
import { collateralSymbol, configuredTestAsset } from "./lib/configuredTestAsset.js";
import { defaultDirectPathwayId, parsePathwayId } from "./lib/pathwayIdentity.js";
import { ensurePermission } from "./lib/permissionReadiness.js";
import { updatePermission } from "./lib/permissionUpdate.js";
import { flagValue, loadPublicCaller } from "./lib/publicCaller.js";

const ROLE_BY_NAME: Readonly<Record<string, bigint>> = {
    mint: ROLE.CAN_MINT_DIRECT,
    redeem: ROLE.CAN_REDEEM_DIRECT,
    "mint-delegated": ROLE.CAN_MINT_DELEGATED,
    "redeem-delegated": ROLE.CAN_REDEEM_DELEGATED,
};

function subjectsFromArguments(): PublicKey[] {
    const addresses = new Set<string>();
    const source = flagValue("--file");
    if (source !== null) {
        for (const line of readFileSync(source, "utf8").split("\n")) {
            const address = line.trim();
            if (address.length > 0 && !address.startsWith("#")) addresses.add(address);
        }
    }
    const valuedFlags = new Set(["--file", "--roles", "--expiry", "--keypair", "--config", "--symbol", "--pathway-id"]);
    const switches = new Set(["--global", "--setup", "--update"]);
    const arguments_ = process.argv.slice(2);
    for (let index = 0; index < arguments_.length; index++) {
        const argument = arguments_[index]!;
        if (argument === "devnet" || switches.has(argument)) continue;
        if (valuedFlags.has(argument)) { index += 1; continue; }
        if (argument.startsWith("--")) throw new Error("unknown option " + argument);
        addresses.add(argument);
    }
    if (addresses.size === 0) throw new Error("supply wallet addresses or --file");
    return Array.from(addresses, publicKeyFromAddress);
}

async function main(): Promise<void> {
    preflight();
    if (DRY_RUN) throw new Error("permission changes require committed state; --dry-run is unsupported");
    const network = activeNetwork();
    const caller = loadPublicCaller();
    const subjects = subjectsFromArguments();
    const update = process.argv.includes("--update");
    const rolesText = flagValue("--roles");
    const expiryText = flagValue("--expiry") ?? (update ? null : "0");
    if (update && rolesText === null && expiryText === null) throw new Error("specify --roles or --expiry to update");
    let roleMask: bigint | null = update && rolesText === null ? null : 0n;
    if (roleMask !== null && rolesText !== "none") {
        for (const name of (rolesText ?? "mint,redeem").split(",")) {
            const role = ROLE_BY_NAME[name.trim()];
            if (role === undefined) throw new Error("unknown settlement role " + name);
            roleMask |= role;
        }
    }
    if (expiryText !== null && !/^(0|[1-9][0-9]*)$/u.test(expiryText)) {
        throw new Error("--expiry must be an unsigned integer timestamp");
    }
    if (!update && roleMask === 0n) throw new Error("a new permission must grant at least one role");
    const expiry = expiryText === null ? null : BigInt(expiryText);
    if (expiry !== null && expiry >= (1n << 63n)) throw new Error("--expiry exceeds the signed 64-bit timestamp range");
    const harness = await createDevnetHarness({ rpcUrl: network.rpcUrl, commitment: "confirmed", requireProgramdata: false });
    const context = process.argv.includes("--setup")
        ? await createSetupContext(harness, caller, loadSetupAuthority())
        : await createControllerContext(harness, caller, publicKeyFromAddress(network.faucetProgramId));
    const globalScope = process.argv.includes("--global");
    let scopeKey = DEFAULT_PUBLIC_KEY;
    if (!globalScope) {
        const asset = configuredTestAsset(network, collateralSymbol(flagValue("--symbol") ?? "USDC"));
        const requestedId = flagValue("--pathway-id");
        const identifier = requestedId === null
            ? await defaultDirectPathwayId(asset.symbol, asset.mint, issuedTokenMint(),
                flagValue("--keypair") === null ? undefined : caller.publicKey)
            : parsePathwayId(requestedId);
        [scopeKey] = await pathwayPolicyPda(identifier);
        if (await harness.banksClient.getAccount(scopeKey) === null) throw new Error("requested pathway is not registered");
    }
    for (const subject of subjects) {
        const scope = { scopeKind: globalScope ? SCOPE.GLOBAL : SCOPE.PATHWAY, scopeKey };
        const permission = update
            ? await updatePermission(context, subject, { roleMask, expiryUnixTimestamp: expiry }, scope)
            : await ensurePermission(context, subject, roleMask!, { ...scope, expiryUnixTimestamp: expiry! });
        console.log(subject.toBase58() + " -> " + permission.toBase58());
    }
}

main().catch((error: unknown) => {
    console.error(error instanceof Error ? error.stack ?? error.message : String(error));
    process.exitCode = 1;
});
