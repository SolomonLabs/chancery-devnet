import { PublicKey } from "@solomon-labs/publickey";

import { LimitPolicy } from "../../../clients/ts/src/accounts/LimitPolicy.js";
import { PathwayPolicy } from "../../../clients/ts/src/accounts/PathwayPolicy.js";
import { PATHWAY_KIND, PROGRAM_ID, SCOPE } from "../../../clients/ts/src/constants.js";
import type { RuntimeHarness } from "../../../runtime/harness.js";
import type { PrimaryLimitAccounts } from "../../../runtime/limits/types.js";
import { limitWindowAccounts } from "../../../runtime/limits/windows.js";
import { limitPolicyPda } from "../../../runtime/pdas.js";
import { publicKeyToBase58, publicKeyToBytes } from "../../../runtime/publicKey.js";
import { DEFAULT_PUBLIC_KEY } from "../../../runtime/wellKnown.js";

export async function directSettlementLimits(harness: RuntimeHarness, pathway: PublicKey,
    assetMint: PublicKey, issuedMint: PublicKey): Promise<PrimaryLimitAccounts> {
    const account = await harness.banksClient.getAccount(pathway);
    if (account === null || account.owner?.equals(PROGRAM_ID) !== true) throw new Error("pathway is absent or has the wrong owner");
    const policy = PathwayPolicy.decode(account.data, pathway);
    if (Number(policy.pathwayKind) !== PATHWAY_KIND.DIRECT
        || publicKeyToBase58(policy.assetMint) !== assetMint.toBase58()
        || publicKeyToBase58(policy.issuedTokenMint) !== issuedMint.toBase58()) {
        throw new Error("pathway does not match the requested direct asset pair");
    }
    if ([policy.feePolicyId, policy.evidencePolicyId, policy.insurancePolicyId,
        policy.assetMintLimitPolicyId, policy.assetRedeemLimitPolicyId,
        policy.counterpartyLimitPolicyId, policy.executorLimitPolicyId]
        .some((identifier) => identifier.some((byte) => Number(byte) !== 0))) {
        throw new Error("settle supports the direct onboarding profile with optional primary limits; linked fee, evidence, insurance, and dimension policies require their generated instruction accounts");
    }
    if (policy.limitPolicyId.every((byte) => Number(byte) === 0)) {
        return { limitPolicy: DEFAULT_PUBLIC_KEY, hourlyUsageWindow: DEFAULT_PUBLIC_KEY,
            dailyUsageWindow: DEFAULT_PUBLIC_KEY, weeklyUsageWindow: DEFAULT_PUBLIC_KEY, monthlyUsageWindow: DEFAULT_PUBLIC_KEY };
    }
    const [limitPolicy] = await limitPolicyPda(Uint8Array.from(policy.limitPolicyId));
    const limitAccount = await harness.banksClient.getAccount(limitPolicy);
    if (limitAccount === null || limitAccount.owner?.equals(PROGRAM_ID) !== true) throw new Error("primary limit policy is absent or has the wrong owner");
    const limits = LimitPolicy.decode(limitAccount.data, limitPolicy);
    const scopeKey = new PublicKey(publicKeyToBytes(limits.scopeKey));
    if (Number(limits.scopeKind) !== SCOPE.PATHWAY || !scopeKey.equals(pathway)) throw new Error("primary limit policy scope does not match the pathway");
    return { limitPolicy, ...await limitWindowAccounts(SCOPE.PATHWAY, scopeKey, limits) };
}
