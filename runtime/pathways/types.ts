import type { PublicKey } from "@solomon-labs/publickey";

import type { PathwayPolicy } from "../../clients/ts/src/accounts/PathwayPolicy.js";

export type PolicyAddressDeriver = (identifier: Uint8Array) => Promise<[PublicKey, number]>;

export type PathwayReferenceIdentifiers = Pick<PathwayPolicy,
    "limitPolicyId" | "evidencePolicyId" | "feePolicyId" | "insurancePolicyId"
    | "assetMintLimitPolicyId" | "assetRedeemLimitPolicyId" | "counterpartyLimitPolicyId" | "executorLimitPolicyId">;

export interface PathwayReferenceAccounts {
    readonly limitPolicy: PublicKey;
    readonly evidencePolicy: PublicKey;
    readonly feePolicy: PublicKey;
    readonly insurancePolicy: PublicKey;
    readonly assetMintLimitPolicy: PublicKey;
    readonly assetRedeemLimitPolicy: PublicKey;
    readonly counterpartyLimitPolicy: PublicKey;
    readonly executorLimitPolicy: PublicKey;
}
