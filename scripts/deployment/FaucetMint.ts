import { Base58Util } from "@solomon-labs/base58";
import { PublicKey } from "@solomon-labs/publickey";
import type { Base58 } from "@solomon-labs/types";

import { createDevnetHarness } from "../../runtime/devnetHarness.js";
import { submitTransaction } from "../../runtime/submit.js";
import {
    createTokenAccount,
    getAssociatedTokenAddress,
    TOKEN_2022_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
} from "../../runtime/token.js";
import { activeNetwork, DRY_RUN, preflight, printContext } from "./lib/bootstrap.js";
import { loadPublicCaller } from "./lib/publicCaller.js";
import { inspectCollateralMintAccount } from "./lib/collateralMint.js";
import { testAssetProfileForSymbol, type TestAssetSymbol } from "./lib/testAssetProfiles.js";

const FAUCET_AUTHORITY_SEED = new TextEncoder().encode("faucet-authority");
const FAUCET_MINT_TEST_ASSET = 0x00;
const UNSTAMPED_PROGRAM_ID = "11111111111111111111111111111111";
const DEFAULT_WHOLE_TOKENS = 1_000n;
const TOKEN_ACCOUNT_AMOUNT_OFFSET = 64;

function flagValue(flag: string): string | null {
    const index = process.argv.indexOf(flag);
    if (index < 0) return null;
    const value = process.argv[index + 1];
    if (value === undefined || value.startsWith("--")) throw new Error(`${flag} requires a value`);
    return value;
}

function publicKeyFromAddress(address: string): PublicKey {
    return new PublicKey(Base58Util.decode(address as Base58));
}

function requestedSymbol(): TestAssetSymbol {
    const raw = flagValue("--symbol") ?? "USDC";
    // testAssetProfileForSymbol throws on anything unknown.
    testAssetProfileForSymbol(raw as TestAssetSymbol);
    return raw as TestAssetSymbol;
}

/** Build the faucet's mint_test_asset instruction. */
function mintTestAssetInstruction(args: {
    readonly faucetProgramId: PublicKey;
    readonly mint: PublicKey;
    readonly destination: PublicKey;
    readonly faucetAuthority: PublicKey;
    readonly tokenProgram: PublicKey;
    readonly amount: bigint;
}): { programId: PublicKey; accounts: { pubkey: PublicKey; isSigner: boolean; isWritable: boolean }[]; data: Uint8Array } {
    const data = new Uint8Array(9);
    data[0] = FAUCET_MINT_TEST_ASSET;
    new DataView(data.buffer).setBigUint64(1, args.amount, true);
    return {
        programId: args.faucetProgramId,
        accounts: [
            { pubkey: args.mint, isSigner: false, isWritable: true },
            { pubkey: args.destination, isSigner: false, isWritable: true },
            { pubkey: args.faucetAuthority, isSigner: false, isWritable: false },
            { pubkey: args.tokenProgram, isSigner: false, isWritable: false },
        ],
        data,
    };
}

function configuredMint(symbol: TestAssetSymbol): string {
    const network = activeNetwork();
    const profile = testAssetProfileForSymbol(symbol);
    if (profile.role === "legacy-migration-source") {
        if (network.legacyUsdvMint === null) throw new Error("legacyUsdvMint is not configured");
        return network.legacyUsdvMint;
    }
    const collateral = (network.collateralMints ?? []).find((entry) => entry.symbol === symbol);
    if (collateral === undefined) throw new Error(`${symbol} is not configured in the devnet network identity`);
    return collateral.mint;
}

async function tokenBalance(
    harness: Awaited<ReturnType<typeof createDevnetHarness>>,
    account: PublicKey,
): Promise<bigint> {
    const info = await harness.banksClient.getAccount(account);
    if (info === null) return 0n;
    if (info.data.length < 165) throw new Error("destination is not a token account");
    return new DataView(info.data.buffer, info.data.byteOffset, info.data.byteLength)
        .getBigUint64(TOKEN_ACCOUNT_AMOUNT_OFFSET, true);
}

function formatAmount(amount: bigint, decimals: number): string {
    const scale = 10n ** BigInt(decimals);
    return `${amount / scale}.${(amount % scale).toString().padStart(decimals, "0")}`;
}

async function main(): Promise<void> {
    preflight();
    if (DRY_RUN) throw new Error("faucet minting requires committed state; --dry-run is unsupported");
    const network = activeNetwork();
    if (network.faucetProgramId === UNSTAMPED_PROGRAM_ID) {
        throw new Error("faucet program id is unstamped in config/networks/devnet.ts");
    }

    const symbol = requestedSymbol();
    const profile = testAssetProfileForSymbol(symbol);
    const caller = loadPublicCaller();
    const destinationOwner = flagValue("--to") === null
        ? caller.publicKey
        : publicKeyFromAddress(flagValue("--to") as string);

    const rawAmount = flagValue("--amount");
    if (rawAmount !== null && !/^[1-9][0-9]*$/u.test(rawAmount)) {
        throw new Error("--amount must be a positive base-unit integer");
    }
    const amount = rawAmount === null
        ? DEFAULT_WHOLE_TOKENS * 10n ** BigInt(profile.decimals)
        : BigInt(rawAmount);

    if (amount <= 0n || amount > (1n << 64n) - 1n || amount > 10_000n * 10n ** BigInt(profile.decimals)) {
        throw new Error("--amount must fit u64 and be at most 10,000 whole tokens per faucet call");
    }
    const faucetProgramId = publicKeyFromAddress(network.faucetProgramId);
    const [faucetAuthority] = await PublicKey.findProgramAddress([FAUCET_AUTHORITY_SEED], faucetProgramId);
    const mint = publicKeyFromAddress(configuredMint(symbol));
    const tokenProgram = profile.tokenProgramKind === "token-2022" ? TOKEN_2022_PROGRAM_ID : TOKEN_PROGRAM_ID;

    const harness = await createDevnetHarness({
        commitment: "confirmed",
        requireProgramdata: false,
        rpcUrl: network.rpcUrl,
    });

    const programAccount = await harness.banksClient.getAccount(faucetProgramId);
    if (programAccount?.executable !== true) throw new Error("configured faucet program is not executable");
    const mintAccount = await harness.banksClient.getAccount(mint);
    if (mintAccount === null || mintAccount.owner?.equals(tokenProgram) !== true) throw new Error("mint is absent or has the wrong token program");
    const inspected = inspectCollateralMintAccount({ data: mintAccount.data, owner: tokenProgram });
    if (inspected.decimals !== profile.decimals || inspected.transferHook !== (profile.transferHook === "dormant-only" ? "dormant" : "absent")) {
        throw new Error("mint does not match its configured test profile");
    }
    const mintView = new DataView(mintAccount.data.buffer, mintAccount.data.byteOffset, mintAccount.data.byteLength);
    if (mintView.getUint32(0, true) !== 1 || !new PublicKey(mintAccount.data.subarray(4, 36)).equals(faucetAuthority)) {
        throw new Error("mint authority has not been handed to the faucet");
    }
    const destination = await getAssociatedTokenAddress(mint, destinationOwner, false, tokenProgram);

    printContext("Faucet mint", {
        symbol,
        mint: mint.toBase58(),
        destination: destination.toBase58(),
        amount: `${formatAmount(amount, profile.decimals)} (${amount} base units)`,
        caller: caller.publicKey.toBase58(),
    });

    if (await harness.banksClient.getAccount(destination) === null) {
        await createTokenAccount(harness, caller, mint, destinationOwner, tokenProgram);
        console.log(`created destination token account ${destination.toBase58()}`);
    }

    const before = await tokenBalance(harness, destination);
    await submitTransaction(
        harness,
        [mintTestAssetInstruction({ faucetProgramId, mint, destination, faucetAuthority, tokenProgram, amount })],
        [],
        caller,
    );
    const after = await tokenBalance(harness, destination);
    if (after - before !== amount) throw new Error("faucet destination balance change differs from the requested amount");

    console.log(
        `\n${symbol}  ${formatAmount(before, profile.decimals)} -> ${formatAmount(after, profile.decimals)}`
        + `  (+${formatAmount(after - before, profile.decimals)})`,
    );
}

main().catch((error: unknown) => {
    console.error(error instanceof Error ? error.stack ?? error.message : String(error));
    process.exitCode = 1;
});
