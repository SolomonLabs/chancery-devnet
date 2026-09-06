import { PublicKey } from "@solomon-labs/publickey";
import type { BU64, U8 } from "@solomon-labs/types";

import { MintDirectInstruction, RedeemDirectInstruction } from "../../clients/ts/src/instructions/index.js";
import { createDevnetHarness } from "../../runtime/devnetHarness.js";
import {
    assetConfigPda,
    assetPauseStatePda,
    chanceryConfigPda,
    eventAuthorityPda,
    issuedTokenControlPda,
    mintAuthorityPda,
    moduleActivationStatePda,
    pathwayPolicyPda,
    pauseStatePda,
    permissionRecordPda,
    reserveAuthorityPda,
} from "../../runtime/pdas.js";
import { submitTransaction } from "../../runtime/submit.js";
import {
    createTokenAccount,
    getAssociatedTokenAddress,
    TOKEN_2022_PROGRAM_ID,
} from "../../runtime/token.js";
import { DEFAULT_PUBLIC_KEY, toSolanaKey } from "../../runtime/wellKnown.js";
import { SCOPE } from "../../clients/ts/src/constants.js";
import type { RuntimeHarness } from "../../runtime/harness.js";
import {
    DRY_RUN,
    activeNetwork,
    issuedTokenMint,
    preflight,
    printContext,
} from "./lib/bootstrap.js";
import { defaultDirectPathwayId, parsePathwayId } from "./lib/pathwayIdentity.js";
import { collateralSymbol, configuredTestAsset } from "./lib/configuredTestAsset.js";
import { loadPublicCaller } from "./lib/publicCaller.js";
import { directSettlementLimits } from "./lib/directSettlementLimits.js";

const DEFAULT_SETTLEMENT_AMOUNT = 1_000_000n;
const TOKEN_ACCOUNT_AMOUNT_OFFSET = 64;

function flagValue(flag: string): string | null {
    const index = process.argv.indexOf(flag);
    if (index < 0) return null;
    const value = process.argv[index + 1];
    if (value === undefined || value.startsWith("--")) throw new Error(`${flag} requires a value`);
    return value;
}

function settlementAmount(): bigint {
    const raw = flagValue("--amount");
    if (raw === null) return DEFAULT_SETTLEMENT_AMOUNT;
    if (!/^[1-9][0-9]*$/u.test(raw)) throw new Error("--amount must be a positive base-unit integer");
    const amount = BigInt(raw);
    if (amount > (1n << 64n) - 1n) throw new Error("--amount exceeds the unsigned 64-bit range");
    return amount;
}

async function tokenBalance(harness: RuntimeHarness, account: PublicKey): Promise<bigint> {
    const info = await harness.banksClient.getAccount(account);
    if (info === null) return 0n;
    if (info.data.length < TOKEN_ACCOUNT_AMOUNT_OFFSET + 8) {
        throw new Error(`${account.toBase58()} is not a token account`);
    }
    return new DataView(info.data.buffer, info.data.byteOffset, info.data.byteLength)
        .getBigUint64(TOKEN_ACCOUNT_AMOUNT_OFFSET, true);
}

function formatAmount(amount: bigint, decimals: number): string {
    const scale = 10n ** BigInt(decimals);
    return `${amount / scale}.${(amount % scale).toString().padStart(decimals, "0")}`;
}

async function main(): Promise<void> {
    preflight();
    if (DRY_RUN) throw new Error("settlement requires committed state changes; --dry-run is unsupported");
    const network = activeNetwork();
    const principal = loadPublicCaller();
    const feePayer = principal;

    const mintOnly = process.argv.includes("--mint-only");
    const redeemOnly = process.argv.includes("--redeem-only");
    if (mintOnly && redeemOnly) throw new Error("--mint-only and --redeem-only are mutually exclusive");
    const amount = settlementAmount();

    const asset = configuredTestAsset(network, collateralSymbol(flagValue("--symbol") ?? "USDC"));
    const assetMint = asset.mint;
    const assetTokenProgram = asset.tokenProgram;
    const issuedMint = issuedTokenMint();
    const requestedPathwayId = flagValue("--pathway-id");
    const identifier = requestedPathwayId === null
        ? await defaultDirectPathwayId(asset.symbol, assetMint, issuedMint,
            flagValue("--keypair") === null ? undefined : principal.publicKey)
        : parsePathwayId(requestedPathwayId);

    const harness = await createDevnetHarness({
        commitment: "confirmed",
        requireProgramdata: false,
        rpcUrl: network.rpcUrl,
    });

    const [configKey] = await chanceryConfigPda();
    const [eventKey] = await eventAuthorityPda();
    const [pauseKey] = await pauseStatePda();
    const [activationKey] = await moduleActivationStatePda();
    const [controlKey] = await issuedTokenControlPda();
    const [mintAuthKey] = await mintAuthorityPda();
    const [reserveAuthKey] = await reserveAuthorityPda();
    const [assetConfigKey] = await assetConfigPda(assetMint);
    const [assetPauseKey] = await assetPauseStatePda(assetMint);
    const [pathwayKey] = await pathwayPolicyPda(identifier);

    const chanceryConfig = toSolanaKey(configKey);
    const eventAuthority = toSolanaKey(eventKey);
    const pauseState = toSolanaKey(pauseKey);
    const moduleActivationState = toSolanaKey(activationKey);
    const issuedTokenControl = toSolanaKey(controlKey);
    const mintAuthority = toSolanaKey(mintAuthKey);
    const reserveAuthority = toSolanaKey(reserveAuthKey);
    const assetConfig = toSolanaKey(assetConfigKey);
    const assetPauseState = toSolanaKey(assetPauseKey);
    const pathwayPolicy = toSolanaKey(pathwayKey);
    const limitAccounts = await directSettlementLimits(harness, pathwayPolicy, assetMint, issuedMint);

    const [permissionKey] = await permissionRecordPda(principal.publicKey, SCOPE.PATHWAY, pathwayPolicy);
    const permissionRecord = toSolanaKey(permissionKey);
    if (await harness.banksClient.getAccount(permissionRecord) === null) {
        throw new Error("principal has no pathway-scoped permission; run `yarn bootstrap:policies` first");
    }

    const sourceAssetTokenAccount = await getAssociatedTokenAddress(
        assetMint, principal.publicKey, false, assetTokenProgram,
    );
    const reserveAssetTokenAccount = await getAssociatedTokenAddress(
        assetMint, reserveAuthority, true, assetTokenProgram,
    );
    const destinationIssuedTokenAccount = await getAssociatedTokenAddress(
        issuedMint, principal.publicKey, false, TOKEN_2022_PROGRAM_ID,
    );

    printContext("Direct settlement round trip", {
        principal: principal.publicKey.toBase58(),
        amount: amount.toString(),
        legs: mintOnly ? "mint" : redeemOnly ? "redeem" : "mint + redeem",
    });

    for (const [address, mint, owner, program] of [
        [sourceAssetTokenAccount, assetMint, principal.publicKey, assetTokenProgram],
        [reserveAssetTokenAccount, assetMint, reserveAuthority, assetTokenProgram],
        [destinationIssuedTokenAccount, issuedMint, principal.publicKey, TOKEN_2022_PROGRAM_ID],
    ] as const) {
        if (await harness.banksClient.getAccount(address) === null) {
            await createTokenAccount(harness, feePayer, mint, owner, program);
            console.log(`created token account ${address.toBase58()}`);
        }
    }

    const pathwayId = Array.from(identifier) as U8[];

    if (!redeemOnly) {
        const collateralBefore = await tokenBalance(harness, sourceAssetTokenAccount);
        const issuedBefore = await tokenBalance(harness, destinationIssuedTokenAccount);
        if (collateralBefore < amount) {
            throw new Error(
                `principal holds ${collateralBefore} collateral base units, needs ${amount}; `
                + "top up with `yarn faucet:mint`",
            );
        }

        await submitTransaction(
            harness,
            [new MintDirectInstruction({
                moduleActivationState,
                chanceryConfig,
                eventAuthority,
                pauseState,
                assetConfig,
                assetPauseState,
                pathwayPolicy,
                permissionRecord,
                sourceAssetTokenAccount,
                reserveAssetTokenAccount,
                destinationIssuedTokenAccount,
                assetMint,
                issuedTokenMint: issuedMint,
                mintAuthorityPda: mintAuthority,
                assetTokenProgram,
                issuedTokenProgram: TOKEN_2022_PROGRAM_ID,
                principal: principal.publicKey,
                issuedTokenControl,
                feePolicy: DEFAULT_PUBLIC_KEY,
                feeRecipientTokenAccount: DEFAULT_PUBLIC_KEY,
                ...limitAccounts,
                pathwayId,
                assetAmount: amount as BU64,
                minimumIssuedTokenAmount: 0n as BU64,
            })],
            [principal],
            feePayer,
        );

        const collateralAfter = await tokenBalance(harness, sourceAssetTokenAccount);
        const issuedAfter = await tokenBalance(harness, destinationIssuedTokenAccount);
        if (collateralAfter >= collateralBefore) throw new Error("mint did not debit the collateral account");
        if (issuedAfter <= issuedBefore) throw new Error("mint did not credit the issued-token account");

        console.log("\nMINT");
        console.log(`  collateral  ${formatAmount(collateralBefore, 6)} -> ${formatAmount(collateralAfter, 6)}`
            + `  (${collateralAfter - collateralBefore})`);
        console.log(`  issued      ${formatAmount(issuedBefore, 6)} -> ${formatAmount(issuedAfter, 6)}`
            + `  (+${issuedAfter - issuedBefore})`);
    }

    if (mintOnly) {
        console.log("\n--mint-only: stopping before redeem.");
        return;
    }

    const redeemBefore = await tokenBalance(harness, destinationIssuedTokenAccount);
    const collateralBeforeRedeem = await tokenBalance(harness, sourceAssetTokenAccount);
    const redeemAmount = redeemBefore < amount ? redeemBefore : amount;
    if (redeemAmount === 0n) {
        throw new Error("principal holds no issued token to redeem");
    }

    await submitTransaction(
        harness,
        [new RedeemDirectInstruction({
            moduleActivationState,
            chanceryConfig,
            eventAuthority,
            pauseState,
            assetConfig,
            assetPauseState,
            pathwayPolicy,
            permissionRecord,
            sourceIssuedTokenAccount: destinationIssuedTokenAccount,
            reserveAssetTokenAccount,
            destinationAssetTokenAccount: sourceAssetTokenAccount,
            assetMint,
            issuedTokenMint: issuedMint,
            reserveAuthorityPda: reserveAuthority,
            assetTokenProgram,
            issuedTokenProgram: TOKEN_2022_PROGRAM_ID,
            principal: principal.publicKey,
            issuedTokenControl,
            feePolicy: DEFAULT_PUBLIC_KEY,
            feeRecipientTokenAccount: DEFAULT_PUBLIC_KEY,
            ...limitAccounts,
            pathwayId,
            issuedTokenAmount: redeemAmount as BU64,
            minimumAssetAmount: 0n as BU64,
        })],
        [principal],
        feePayer,
    );

    const redeemAfter = await tokenBalance(harness, destinationIssuedTokenAccount);
    const collateralAfterRedeem = await tokenBalance(harness, sourceAssetTokenAccount);
    if (redeemAfter >= redeemBefore) throw new Error("redeem did not burn the issued token");
    if (collateralAfterRedeem <= collateralBeforeRedeem) throw new Error("redeem did not return collateral");

    console.log("\nREDEEM");
    console.log(`  issued      ${formatAmount(redeemBefore, 6)} -> ${formatAmount(redeemAfter, 6)}`
        + `  (${redeemAfter - redeemBefore})`);
    console.log(`  collateral  ${formatAmount(collateralBeforeRedeem, 6)} -> ${formatAmount(collateralAfterRedeem, 6)}`
        + `  (+${collateralAfterRedeem - collateralBeforeRedeem})`);
    console.log("\nround trip complete.");
}

main().catch((error: unknown) => {
    console.error(error instanceof Error ? error.stack ?? error.message : String(error));
    process.exitCode = 1;
});
