import type { PublicKey } from "@solomon-labs/publickey";

import { evidencePolicyPda, feePolicyPda, insurancePolicyPda, limitPolicyPda } from "../pdas.js";
import { DEFAULT_PUBLIC_KEY } from "../wellKnown.js";
import type { PathwayReferenceAccounts, PathwayReferenceIdentifiers, PolicyAddressDeriver } from "./types.js";

async function referenceAddress(identifier: readonly number[], derive: PolicyAddressDeriver): Promise<PublicKey> {
    if (identifier.length !== 32) throw new Error("pathway reference identifier must contain 32 bytes");
    if (identifier.every((byte) => byte === 0)) return DEFAULT_PUBLIC_KEY;
    return (await derive(Uint8Array.from(identifier)))[0];
}

export async function pathwayReferenceAccounts(policy: PathwayReferenceIdentifiers): Promise<PathwayReferenceAccounts> {
    const [limitPolicy, evidencePolicy, feePolicy, insurancePolicy, assetMintLimitPolicy,
        assetRedeemLimitPolicy, counterpartyLimitPolicy, executorLimitPolicy] = await Promise.all([
        referenceAddress(policy.limitPolicyId, limitPolicyPda),
        referenceAddress(policy.evidencePolicyId, evidencePolicyPda),
        referenceAddress(policy.feePolicyId, feePolicyPda),
        referenceAddress(policy.insurancePolicyId, insurancePolicyPda),
        referenceAddress(policy.assetMintLimitPolicyId, limitPolicyPda),
        referenceAddress(policy.assetRedeemLimitPolicyId, limitPolicyPda),
        referenceAddress(policy.counterpartyLimitPolicyId, limitPolicyPda),
        referenceAddress(policy.executorLimitPolicyId, limitPolicyPda),
    ]);
    return { limitPolicy, evidencePolicy, feePolicy, insurancePolicy, assetMintLimitPolicy,
        assetRedeemLimitPolicy, counterpartyLimitPolicy, executorLimitPolicy };
}
