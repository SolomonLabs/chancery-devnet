import type { PublicKey } from "@solomon-labs/publickey";

import type { RuntimeBanksClient } from "../../../../runtime/harness.js";

export interface TesterFundingServices {
    readonly banksClient: Pick<RuntimeBanksClient, "getAccount" | "getMinimumBalanceForRentExemption">;
    readonly requestAirdrop: (recipient: PublicKey, lamports: bigint) => Promise<string>;
}

export interface TesterFundingRequest {
    readonly minimumLamports: bigint;
    readonly allowAirdrop: boolean;
}
