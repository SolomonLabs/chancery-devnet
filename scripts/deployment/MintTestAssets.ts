// Mint devnet test collateral into wallets.
//
// Allocations are top-ups, not deposits: each names a target balance, and the
// script mints only the deficit needed to reach it. Re-running with the same
// plan is a no-op once every wallet is at target, so a plan can be kept around
// and replayed whenever a test run drains a balance.
//
// The recipient's associated token account is created if it does not exist.
// Every mint is verified to still carry the setup authority as its mint
// authority before anything is minted, and every balance is read back after.

import { resolve } from "node:path";

import { PublicKey } from "@solomon-labs/publickey";
import {
    CreateAssociatedTokenAccountInstruction,
    MintToInstruction,
    TOKEN_2022_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    type IsEncodable,
} from "@solomon-labs/solana-codec";

import { createDevnetHarness } from "../../runtime/devnetHarness.js";
import { submitTransaction } from "../../runtime/submit.js";
import { getAssociatedTokenAddress } from "../../runtime/token.js";
import {
    activeNetwork,
    loadFeePayer,
    loadSetupAuthority,
    preflight,
    printContext,
    publicKeyFromAddress,
} from "./lib/bootstrap.js";
import { inspectCollateralMintAccount } from "./lib/collateralMint.js";
import { testAssetProfileForSymbol } from "./lib/testAssetProfiles.js";
import { loadTestAssetMintPlan, type TestAssetAllocation } from "./lib/testAssetMintPlan.js";

const DEFAULT_PLAN_PATH = ".devnet/asset-mint-plan.json";

interface TokenAccountState {
    readonly mint: PublicKey;
    readonly owner: PublicKey;
    readonly amount: bigint;
}

interface PreparedAllocation {
    readonly allocation: TestAssetAllocation;
    readonly mint: PublicKey;
    readonly tokenProgram: PublicKey;
    readonly decimals: number;
    readonly recipientOwner: PublicKey;
    readonly recipientTokenAccount: PublicKey;
}

function flagValue(flag: string): string | null {
    const index = process.argv.indexOf(flag);
    if (index < 0) return null;
    const value = process.argv[index + 1];
    if (value === undefined || value.startsWith("--")) throw new Error(`${flag} requires a value`);
    return value;
}

function publicKeyFromData(data: Uint8Array, offset: number, label: string): PublicKey {
    if (data.length < offset + 32) {
        throw new Error(`${label} is too short to contain a public key at offset ${offset}`);
    }
    return new PublicKey(data.subarray(offset, offset + 32));
}

function decodeTokenAccount(data: Uint8Array, label: string): TokenAccountState {
    if (data.length < 72) throw new Error(`${label} has invalid token-account length ${data.length}`);
    return {
        mint: publicKeyFromData(data, 0, label),
        owner: publicKeyFromData(data, 32, label),
        amount: new DataView(data.buffer, data.byteOffset, data.byteLength).getBigUint64(64, true),
    };
}

function mintAuthorityFromData(data: Uint8Array, label: string): PublicKey | null {
    if (data.length < 36) throw new Error(`${label} has invalid mint length ${data.length}`);
    const option = new DataView(data.buffer, data.byteOffset, data.byteLength).getUint32(0, true);
    if (option === 0) return null;
    if (option !== 1) throw new Error(`${label} has invalid mint-authority option ${option}`);
    return publicKeyFromData(data, 4, label);
}

function formatBaseUnits(amount: bigint, decimals: number): string {
    const scale = 10n ** BigInt(decimals);
    const whole = amount / scale;
    const fraction = (amount % scale).toString().padStart(decimals, "0");
    return `${whole}.${fraction}`;
}

/**
 * Resolve the configured mint for an allocation. Collateral comes from
 * `collateralMints`; the migration source is not collateral and is carried on
 * `legacyUsdvMint` instead.
 */
function configuredMintAddress(allocation: TestAssetAllocation): string {
    const network = activeNetwork();
    const profile = testAssetProfileForSymbol(allocation.symbol);
    if (profile.role === "legacy-migration-source") {
        if (network.legacyUsdvMint === null) {
            throw new Error(
                "legacyUsdvMint is not configured in the devnet network identity; run yarn assets:create",
            );
        }
        return network.legacyUsdvMint;
    }
    const collateral = (network.collateralMints ?? []).find((entry) => entry.symbol === allocation.symbol);
    if (collateral === undefined) {
        throw new Error(
            `${allocation.symbol} is not configured in the devnet network identity; run yarn assets:create`,
        );
    }
    if (
        collateral.tokenProgram !== profile.tokenProgramKind
        || collateral.transferHook !== profile.transferHook
    ) {
        throw new Error(`${allocation.symbol} network identity does not match its fixed test-asset profile`);
    }
    return collateral.mint;
}

async function prepareAllocation(allocation: TestAssetAllocation): Promise<PreparedAllocation> {
    const profile = testAssetProfileForSymbol(allocation.symbol);
    const mint = publicKeyFromAddress(configuredMintAddress(allocation));
    const tokenProgram = profile.tokenProgramKind === "token-2022" ? TOKEN_2022_PROGRAM_ID : TOKEN_PROGRAM_ID;
    const recipientOwner = publicKeyFromAddress(allocation.recipient_owner);
    const recipientTokenAccount = await getAssociatedTokenAddress(mint, recipientOwner, true, tokenProgram);
    return {
        allocation,
        mint,
        tokenProgram,
        decimals: profile.decimals,
        recipientOwner,
        recipientTokenAccount,
    };
}

async function main(): Promise<void> {
    preflight();
    const network = activeNetwork();

    const planPath = resolve(flagValue("--plan") ?? DEFAULT_PLAN_PATH);
    const loadedPlan = loadTestAssetMintPlan(planPath);
    const feePayer = loadFeePayer();
    const setupAuthority = loadSetupAuthority();

    const prepared: PreparedAllocation[] = [];
    for (const allocation of loadedPlan.plan.allocations) {
        prepared.push(await prepareAllocation(allocation));
    }

    printContext("Mint devnet test collateral", {
        plan: planPath,
        allocations: prepared.length.toString(),
        authority: setupAuthority.publicKey.toBase58(),
    });

    const harness = await createDevnetHarness({
        commitment: "finalized",
        requireProgramdata: false,
        rpcUrl: network.rpcUrl,
    });

    for (const entry of prepared) {
        const symbol: string = entry.allocation.symbol;

        const mintAccount = await harness.banksClient.getAccount(entry.mint);
        if (mintAccount === null) throw new Error(`${symbol} mint ${entry.mint.toBase58()} is absent`);
        if (mintAccount.owner === undefined || !mintAccount.owner.equals(entry.tokenProgram)) {
            throw new Error(`${symbol} mint is owned by an unexpected token program`);
        }
        const inspected = inspectCollateralMintAccount({ data: mintAccount.data, owner: entry.tokenProgram });
        if (inspected.decimals !== entry.decimals) {
            throw new Error(`${symbol} mint decimals ${inspected.decimals} differ from the profile ${entry.decimals}`);
        }
        const liveMintAuthority = mintAuthorityFromData(mintAccount.data, `${symbol} mint`);
        if (liveMintAuthority === null || !liveMintAuthority.equals(setupAuthority.publicKey)) {
            throw new Error(
                `${symbol} mint authority ${liveMintAuthority?.toBase58() ?? "none"} does not equal `
                + `setup authority ${setupAuthority.publicKey.toBase58()}`,
            );
        }

        const beforeAccount = await harness.banksClient.getAccount(entry.recipientTokenAccount);
        let createRecipientAccount = false;
        let amountBefore = 0n;
        if (beforeAccount === null) {
            createRecipientAccount = true;
        } else {
            if (beforeAccount.owner === undefined || !beforeAccount.owner.equals(entry.tokenProgram)) {
                throw new Error(`${symbol} recipient token account is owned by an unexpected program`);
            }
            const decoded = decodeTokenAccount(beforeAccount.data, `${symbol} recipient token account`);
            if (!decoded.mint.equals(entry.mint)) throw new Error(`${symbol} recipient account mint mismatch`);
            if (!decoded.owner.equals(entry.recipientOwner)) {
                throw new Error(`${symbol} recipient account owner mismatch`);
            }
            amountBefore = decoded.amount;
        }

        const targetAmount = BigInt(entry.allocation.minimum_balance_base_units);
        const mintDeficit = amountBefore < targetAmount ? targetAmount - amountBefore : 0n;
        const instructions: IsEncodable[] = [];
        if (createRecipientAccount) {
            instructions.push(new CreateAssociatedTokenAccountInstruction({
                payer: feePayer.publicKey,
                associatedToken: entry.recipientTokenAccount,
                owner: entry.recipientOwner,
                mint: entry.mint,
                tokenProgram: entry.tokenProgram,
            }));
        }
        if (mintDeficit > 0n) {
            instructions.push(new MintToInstruction({
                programId: entry.tokenProgram,
                mint: entry.mint,
                destination: entry.recipientTokenAccount,
                authority: setupAuthority.publicKey,
                amount: mintDeficit,
            }));
        }

        if (instructions.length === 0) {
            console.log(
                `  ${symbol.padEnd(6)} ${entry.recipientOwner.toBase58()} already at `
                + `${formatBaseUnits(amountBefore, entry.decimals)}; nothing to mint`,
            );
            continue;
        }

        await submitTransaction(
            harness,
            instructions,
            mintDeficit > 0n ? [setupAuthority] : [],
            feePayer,
        );

        const afterAccount = await harness.banksClient.getAccount(entry.recipientTokenAccount);
        if (afterAccount === null) throw new Error(`${symbol} recipient token account is absent after minting`);
        if (afterAccount.owner === undefined || !afterAccount.owner.equals(entry.tokenProgram)) {
            throw new Error(`${symbol} recipient token account is owned by an unexpected program after minting`);
        }
        const after = decodeTokenAccount(afterAccount.data, `${symbol} recipient token account`);
        if (!after.mint.equals(entry.mint)) throw new Error(`${symbol} minted account mint mismatch`);
        if (!after.owner.equals(entry.recipientOwner)) throw new Error(`${symbol} minted account owner mismatch`);
        if (after.amount < targetAmount) {
            throw new Error(`${symbol} balance ${after.amount} is below the requested ${targetAmount}`);
        }

        console.log(
            `  ${symbol.padEnd(6)} ${entry.recipientOwner.toBase58()} `
            + `${formatBaseUnits(amountBefore, entry.decimals)} -> ${formatBaseUnits(after.amount, entry.decimals)}`
            + `${createRecipientAccount ? " (account created)" : ""}`,
        );
    }

    console.log("\ntest collateral minted and read back.");
}

main().catch((error: unknown) => {
    console.error(error instanceof Error ? error.stack ?? error.message : String(error));
    process.exitCode = 1;
});
