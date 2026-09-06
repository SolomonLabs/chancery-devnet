import { PublicKey } from "@solomon-labs/publickey";
import type { U8 } from "@solomon-labs/types";

import { PathwayPolicy } from "../../../clients/ts/src/accounts/PathwayPolicy.js";
import { PATHWAY_KIND, PROGRAM_ID } from "../../../clients/ts/src/constants.js";
import { RegisterPathwayPolicyInstruction } from "../../../clients/ts/src/instructions/RegisterPathwayPolicyInstruction.js";
import { submitAdministration } from "../../../runtime/administration/submit.js";
import { limitPolicyPda, pathwayPolicyPda } from "../../../runtime/pdas.js";
import { publicKeyToBase58 } from "../../../runtime/publicKey.js";
import { DEFAULT_PUBLIC_KEY, SYSTEM_PROGRAM } from "../../../runtime/wellKnown.js";
import type { ConfigChangeContext } from "./configChange.js";

export async function registerDirectPathway(context: ConfigChangeContext, pathwayId: Uint8Array,
    assetMint: PublicKey, issuedMint: PublicKey, limitPolicyId: Uint8Array = new Uint8Array(32)): Promise<PublicKey> {
    const [pathwayPolicy] = await pathwayPolicyPda(pathwayId);
    const existing = await context.harness.banksClient.getAccount(pathwayPolicy);
    if (existing === null) {
        const zero = Array.from(new Uint8Array(32)) as U8[];
        const limitPolicy = limitPolicyId.some((byte) => byte !== 0)
            ? (await limitPolicyPda(limitPolicyId))[0] : DEFAULT_PUBLIC_KEY;
        await submitAdministration(context, [new RegisterPathwayPolicyInstruction({
            chanceryConfig: context.chanceryConfig,
            eventAuthority: context.eventAuthority,
            pathwayPolicy,
            payer: context.feePayer.publicKey,
            operationsAuthority: context.operationsAuthority,
            systemProgram: SYSTEM_PROGRAM,
            pathwayKind: PATHWAY_KIND.DIRECT as U8,
            pathwayId: Array.from(pathwayId) as U8[],
            assetMint,
            issuedTokenMint: issuedMint,
            designatedExecutor: DEFAULT_PUBLIC_KEY,
            reserveCompartmentPolicyId: zero,
            limitPolicyId: Array.from(limitPolicyId) as U8[],
            evidencePolicyId: zero,
            feePolicyId: zero,
            insurancePolicyId: zero,
            assetMintLimitPolicyId: zero,
            assetRedeemLimitPolicyId: zero,
            counterpartyLimitPolicyId: zero,
            executorLimitPolicyId: zero,
            limitPolicy,
            evidencePolicy: DEFAULT_PUBLIC_KEY,
            feePolicy: DEFAULT_PUBLIC_KEY,
            insurancePolicy: DEFAULT_PUBLIC_KEY,
            assetMintLimitPolicy: DEFAULT_PUBLIC_KEY,
            assetRedeemLimitPolicy: DEFAULT_PUBLIC_KEY,
            counterpartyLimitPolicy: DEFAULT_PUBLIC_KEY,
            executorLimitPolicy: DEFAULT_PUBLIC_KEY,
        })]);
    }
    const account = await context.harness.banksClient.getAccount(pathwayPolicy);
    if (account === null || account.owner?.equals(PROGRAM_ID) !== true) throw new Error("pathway account is absent or has the wrong owner");
    const policy = PathwayPolicy.decode(account.data, pathwayPolicy);
    if (Number(policy.pathwayKind) !== PATHWAY_KIND.DIRECT
        || publicKeyToBase58(policy.assetMint) !== assetMint.toBase58()
        || publicKeyToBase58(policy.issuedTokenMint) !== issuedMint.toBase58()
        || policy.limitPolicyId.some((byte, index) => Number(byte) !== limitPolicyId[index])) {
        throw new Error("existing pathway does not match the requested direct-settlement policy");
    }
    if ([policy.feePolicyId, policy.evidencePolicyId, policy.insurancePolicyId, policy.reserveCompartmentPolicyId,
        policy.assetMintLimitPolicyId, policy.assetRedeemLimitPolicyId, policy.counterpartyLimitPolicyId, policy.executorLimitPolicyId]
        .some((identifier) => identifier.some((byte) => Number(byte) !== 0))) {
        throw new Error("existing pathway carries additional policies outside the requested direct profile");
    }
    return pathwayPolicy;
}
