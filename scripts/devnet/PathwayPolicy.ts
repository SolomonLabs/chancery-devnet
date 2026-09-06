import type { U8 } from "@solomon-labs/types";

import { LimitPolicy } from "../../clients/ts/src/accounts/LimitPolicy.js";
import { PathwayPolicy } from "../../clients/ts/src/accounts/PathwayPolicy.js";
import { CHANGE_KIND, PROGRAM_ID, SCOPE } from "../../clients/ts/src/constants.js";
import { UpdatePathwayPolicyWithPendingChangeInstruction } from "../../clients/ts/src/instructions/UpdatePathwayPolicyWithPendingChangeInstruction.js";
import { RISK } from "../../runtime/administration/risk.js";
import { submitAdministration } from "../../runtime/administration/submit.js";
import { createDevnetHarness } from "../../runtime/devnetHarness.js";
import { pathwayPolicyPayload } from "../../runtime/pathways/payload.js";
import { pathwayReferenceAccounts } from "../../runtime/pathways/references.js";
import { pathwayPolicyPda } from "../../runtime/pdas.js";
import { publicKeyToBase58 } from "../../runtime/publicKey.js";
import { createControllerContext } from "../deployment/lib/administrationContext.js";
import { activeNetwork, DRY_RUN, preflight, publicKeyFromAddress } from "../deployment/lib/bootstrap.js";
import { proposeAndAcceptConfigChange } from "../deployment/lib/configChange.js";
import { parsePathwayId } from "../deployment/lib/pathwayIdentity.js";
import { flagValue, loadPublicCaller } from "../deployment/lib/publicCaller.js";

async function main(): Promise<void> {
    preflight();
    if (DRY_RUN) throw new Error("pathway updates require committed state; --dry-run is unsupported");
    const pathwayText = flagValue("--pathway-id");
    const limitText = flagValue("--limit-policy-id");
    if (pathwayText === null || limitText === null) throw new Error("specify --pathway-id and --limit-policy-id; use none to unlink the primary limit");
    const identifier = parsePathwayId(pathwayText);
    const limitIdentifier = limitText === "none" ? new Uint8Array(32) : parsePathwayId(limitText);
    const network = activeNetwork();
    const caller = loadPublicCaller();
    const harness = await createDevnetHarness({ rpcUrl: network.rpcUrl, commitment: "confirmed", requireProgramdata: false });
    const context = await createControllerContext(harness, caller, publicKeyFromAddress(network.faucetProgramId));
    const [address] = await pathwayPolicyPda(identifier);
    const account = await harness.banksClient.getAccount(address);
    if (account === null || account.owner?.equals(PROGRAM_ID) !== true) throw new Error("pathway is absent or has the wrong owner");
    const current = PathwayPolicy.decode(account.data, address);
    if (current.pathwayId.some((byte, index) => Number(byte) !== identifier[index])) throw new Error("pathway identifier differs from the requested policy");
    const oldPayload = pathwayPolicyPayload(account.data);
    const newPayload = pathwayPolicyPayload(account.data, limitIdentifier);
    if (oldPayload.every((byte, index) => byte === newPayload[index])) {
        console.log("pathway already has the requested primary limit: " + address.toBase58());
        return;
    }
    const limitPolicyId = Array.from(limitIdentifier) as U8[];
    const references = await pathwayReferenceAccounts({ ...current, limitPolicyId });
    if (limitIdentifier.some((byte) => byte !== 0)) {
        const limitAccount = await harness.banksClient.getAccount(references.limitPolicy);
        if (limitAccount === null || limitAccount.owner?.equals(PROGRAM_ID) !== true) throw new Error("limit policy is absent or has the wrong owner");
        const limit = LimitPolicy.decode(limitAccount.data, references.limitPolicy);
        if (Number(limit.scopeKind) !== SCOPE.PATHWAY || publicKeyToBase58(limit.scopeKey) !== address.toBase58()
            || limit.limitPolicyId.some((byte, index) => Number(byte) !== limitIdentifier[index])) {
            throw new Error("limit policy identity or scope differs from the selected pathway");
        }
    }
    const pendingConfigChange = await proposeAndAcceptConfigChange({ context, changeKind: CHANGE_KIND.UPDATE_PATHWAY_POLICY,
        riskClass: RISK.WIDENING, targetAccount: address, oldPayload, newPayload,
    });
    await submitAdministration(context, [new UpdatePathwayPolicyWithPendingChangeInstruction({
        moduleActivationState: context.moduleActivationState, chanceryConfig: context.chanceryConfig,
        eventAuthority: context.eventAuthority, pathwayPolicy: address, governanceAuthority: context.governanceAuthority,
        pendingConfigChange, rentRefundRecipient: caller.publicKey, pathwayId: Array.from(identifier) as U8[],
        designatedExecutor: null, limitPolicyId, evidencePolicyId: null, feePolicyId: null, insurancePolicyId: null,
        forbiddenCollateralExtensionMask: null, assetMintLimitPolicyId: null, assetRedeemLimitPolicyId: null,
        counterpartyLimitPolicyId: null, executorLimitPolicyId: null, ...references,
    })]);
    const after = await harness.banksClient.getAccount(address);
    if (after === null || after.owner?.equals(PROGRAM_ID) !== true) throw new Error("pathway readback failed");
    if (pathwayPolicyPayload(after.data).some((byte, index) => byte !== newPayload[index])) {
        throw new Error("pathway readback differs from the submitted values");
    }
    console.log("pathway: " + address.toBase58());
    console.log("primary limit: " + (limitText === "none" ? "none" : references.limitPolicy.toBase58()));
}

main().catch((error: unknown) => {
    console.error(error instanceof Error ? error.stack ?? error.message : String(error));
    process.exitCode = 1;
});
