import type { PublicKey } from "@solomon-labs/publickey";

import { SYSTEM_PROGRAM } from "../../../../runtime/wellKnown.js";
import type { TesterFundingRequest, TesterFundingServices } from "./types.js";

async function walletBalance(services: TesterFundingServices, wallet: PublicKey): Promise<bigint> {
    const account = await services.banksClient.getAccount(wallet);
    if (account === null) return 0n;
    if (account.owner?.equals(SYSTEM_PROGRAM) !== true || account.executable === true || account.data.length !== 0) {
        throw new Error("tester wallet must be a system-owned, zero-data account: " + wallet.toBase58());
    }
    return account.lamports;
}

export async function fundTesterWallet(services: TesterFundingServices, wallet: PublicKey,
    request: TesterFundingRequest): Promise<bigint> {
    const { minimumLamports, allowAirdrop } = request;
    if (minimumLamports <= 0n || minimumLamports > BigInt(Number.MAX_SAFE_INTEGER)) {
        throw new Error("tester funding must be a positive, exactly representable RPC lamport amount");
    }
    const before = await walletBalance(services, wallet);
    if (before >= minimumLamports) return before;
    if (!allowAirdrop) {
        throw new Error("tester wallet " + wallet.toBase58() + " holds " + before + " lamports; requires at least " + minimumLamports);
    }
    const minimumRent = services.banksClient.getMinimumBalanceForRentExemption;
    if (minimumRent === undefined) throw new Error("runtime cannot calculate tester wallet rent exemption");
    const rent = await minimumRent.call(services.banksClient, 0);
    if (minimumLamports <= rent) throw new Error("tester funding must exceed zero-data account rent: " + rent);
    try {
        await services.requestAirdrop(wallet, minimumLamports - before);
    } catch (error: unknown) {
        throw new Error("devnet SOL airdrop failed for " + wallet.toBase58()
            + "; fund this wallet and rerun with --skip-airdrop", { cause: error });
    }
    const after = await walletBalance(services, wallet);
    if (after < minimumLamports) {
        throw new Error("tester funding readback holds " + after + " lamports; requires at least " + minimumLamports);
    }
    return after;
}
