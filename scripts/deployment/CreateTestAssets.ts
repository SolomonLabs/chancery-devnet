import { dirname, relative, resolve } from "node:path";

import { PublicKey } from "@solomon-labs/publickey";
import {
    CreateAccountInstruction,
    InitializeMint2Instruction,
    MINT_ACCOUNT_LEN,
    TOKEN_2022_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    Token2022TransferHook,
    type EncodedInstruction,
} from "@solomon-labs/solana-codec";

import { workspaceRoot, type ChanceryCollateralMint } from "../../config/network.js";
import { faucetAuthority } from "../../runtime/controller/authority.js";
import { createDevnetHarness, requestAirdropAndConfirm } from "../../runtime/devnetHarness.js";
import {
    submitTransaction,
    type E2ESigner,
} from "../../runtime/submit.js";
import { replaceSourceAtomically } from "../core/AtomicSource.js";
import { loadOrGenerateSigner } from "./lib/signerFile.js";
import { activeNetwork, loadFeePayer, loadSetupAuthority, preflight, printContext, publicKeyFromAddress } from "./lib/bootstrap.js";
import { inspectCollateralMintAccount } from "./lib/collateralMint.js";
import { TEST_ASSET_PROFILES, type TestAssetSymbol } from "./lib/testAssetProfiles.js";

const ZERO_PUBLIC_KEY = new PublicKey(new Uint8Array(32));
const TOKEN_2022_BASE_MINT_LENGTH = 166;
const TOKEN_2022_TLV_HEADER_LENGTH = 4;
/** Token-2022 `InitializeTransferHook` discriminator. */
const TOKEN_2022_INITIALIZE_TRANSFER_HOOK = new Uint8Array([36, 0]);
const IDENTITY_PATH = resolve(workspaceRoot(), "config/networks/devnet.ts");

interface MintSpecification {
    readonly symbol: TestAssetSymbol;
    readonly role: "collateral" | "legacy-migration-source";
    readonly decimals: number;
    readonly tokenProgram: PublicKey;
    readonly tokenProgramKind: "spl-token" | "token-2022";
    readonly transferHook: "absent" | "dormant-only";
    readonly signerPath: string;
}

function workspaceRelativePath(path: string): string {
    return relative(workspaceRoot(), path).split("\\").join("/");
}

function concatenateBytes(...parts: readonly Uint8Array[]): Uint8Array {
    let length = 0;
    for (const part of parts) length += part.length;
    const output = new Uint8Array(length);
    let offset = 0;
    for (const part of parts) {
        output.set(part, offset);
        offset += part.length;
    }
    return output;
}

function verifyBaseMintFields(
    data: Uint8Array,
    mintAuthority: PublicKey,
    decimals: number,
    field: string,
): void {
    if (data.length < 82) throw new Error(`${field} mint data is too short`);
    const view = new DataView(data.buffer, data.byteOffset, data.byteLength);
    if (view.getUint32(0, true) !== 1) throw new Error(`${field} mint authority option must be Some`);
    const observedMintAuthority = new PublicKey(data.subarray(4, 36));
    if (!observedMintAuthority.equals(mintAuthority)) {
        throw new Error(
            `${field} mint authority ${observedMintAuthority.toBase58()} differs from ${mintAuthority.toBase58()}`,
        );
    }
    // Supply is deliberately not asserted zero: unlike the staging ceremony,
    // this script is re-run against mints that have already been minted from.
    if (data[44] !== decimals) throw new Error(`${field} mint decimals differ from ${decimals}`);
    if (data[45] !== 1) throw new Error(`${field} mint is not initialized`);
    if (view.getUint32(46, true) !== 0) throw new Error(`${field} freeze authority must be disabled`);
}

async function createMint(
    context: Awaited<ReturnType<typeof createDevnetHarness>>,
    feePayer: E2ESigner,
    mintAuthority: PublicKey,
    mintSigner: E2ESigner,
    specification: MintSpecification,
): Promise<void> {
    const rentFunction = context.banksClient.getMinimumBalanceForRentExemption;
    if (rentFunction === undefined) throw new Error("devnet runtime cannot calculate rent exemption");
    const instructions: Array<EncodedInstruction | CreateAccountInstruction | InitializeMint2Instruction> = [];
    let mintLength = MINT_ACCOUNT_LEN;
    if (specification.transferHook === "dormant-only") {
        const encodedHook = new Token2022TransferHook({
            authority: mintAuthority,
            programId: ZERO_PUBLIC_KEY,
        }).encode();
        mintLength = TOKEN_2022_BASE_MINT_LENGTH + TOKEN_2022_TLV_HEADER_LENGTH + encodedHook.length;
        instructions.push(new CreateAccountInstruction({
            from: feePayer.publicKey,
            newAccount: mintSigner.publicKey,
            lamports: await rentFunction.call(context.banksClient, mintLength),
            space: mintLength,
            owner: TOKEN_2022_PROGRAM_ID,
        }));
        instructions.push({
            programId: TOKEN_2022_PROGRAM_ID,
            accounts: [{ pubkey: mintSigner.publicKey, isSigner: false, isWritable: true }],
            data: concatenateBytes(TOKEN_2022_INITIALIZE_TRANSFER_HOOK, encodedHook),
        });
    } else {
        instructions.push(new CreateAccountInstruction({
            from: feePayer.publicKey,
            newAccount: mintSigner.publicKey,
            lamports: await rentFunction.call(context.banksClient, mintLength),
            space: mintLength,
            owner: specification.tokenProgram,
        }));
    }
    instructions.push(new InitializeMint2Instruction({
        programId: specification.tokenProgram,
        mint: mintSigner.publicKey,
        decimals: specification.decimals,
        mintAuthority,
        freezeAuthority: null,
    }));
    await submitTransaction(context, instructions, [mintSigner], feePayer);
}

async function verifyMint(
    context: Awaited<ReturnType<typeof createDevnetHarness>>,
    specification: MintSpecification,
    mintSigner: E2ESigner,
    mintAuthority: PublicKey,
): Promise<string> {
    const account = await context.banksClient.getAccount(mintSigner.publicKey);
    if (account === null) throw new Error(`${specification.symbol} mint account is absent after creation`);
    const accountOwner = account.owner;
    if (accountOwner === undefined) throw new Error(`${specification.symbol} mint account owner is missing`);
    if (!accountOwner.equals(specification.tokenProgram)) {
        throw new Error(
            `${specification.symbol} owner ${accountOwner.toBase58()} differs from ${specification.tokenProgram.toBase58()}`,
        );
    }
    const network = activeNetwork();
    const expectedFaucetAuthority = network.faucetProgramId === "11111111111111111111111111111111"
        ? null : await faucetAuthority(publicKeyFromAddress(network.faucetProgramId));
    if (account.data.length < 82) throw new Error(`${specification.symbol} mint data is too short`);
    const observedAuthority = new PublicKey(account.data.subarray(4, 36));
    const expectedAuthority = expectedFaucetAuthority?.equals(observedAuthority) === true
        ? expectedFaucetAuthority : mintAuthority;
    verifyBaseMintFields(account.data, expectedAuthority, specification.decimals, specification.symbol);

    const inspected = inspectCollateralMintAccount({ data: account.data, owner: accountOwner });
    if (inspected.decimals !== specification.decimals) {
        throw new Error(`${specification.symbol} inspected decimals differ`);
    }
    const expectedHook = specification.transferHook === "dormant-only" ? "dormant" : "absent";
    if (inspected.transferHook !== expectedHook) {
        throw new Error(`${specification.symbol} transfer hook is ${inspected.transferHook}, expected ${expectedHook}`);
    }
    const expectedNames = specification.transferHook === "dormant-only" ? "DormantTransferHook" : "";
    if (inspected.extensionNames.join(",") !== expectedNames) {
        throw new Error(
            `${specification.symbol} extensions ${inspected.extensionNames.join(",")} differ from ${expectedNames}`,
        );
    }

    const rentFunction = context.banksClient.getMinimumBalanceForRentExemption;
    if (rentFunction === undefined) throw new Error("devnet runtime cannot calculate rent exemption");
    const minimumRent = await rentFunction.call(context.banksClient, account.data.length);
    if (account.lamports < minimumRent) throw new Error(`${specification.symbol} mint is not rent exempt`);

    return mintSigner.publicKey.toBase58();
}

async function ensureMint(
    context: Awaited<ReturnType<typeof createDevnetHarness>>,
    feePayer: E2ESigner,
    mintAuthority: PublicKey,
    specification: MintSpecification,
    signer: E2ESigner,
): Promise<string> {
    const account = await context.banksClient.getAccount(signer.publicKey);
    if (account === null) {
        console.log(`creating ${specification.symbol} test mint ${signer.publicKey.toBase58()}`);
        await createMint(context, feePayer, mintAuthority, signer, specification);
    } else {
        console.log(`reusing ${specification.symbol} test mint ${signer.publicKey.toBase58()}`);
    }
    return verifyMint(context, specification, signer, mintAuthority);
}

function identitySource(
    programId: string,
    collateralMints: readonly ChanceryCollateralMint[],
    legacyUsdvMint: string,
): string {
    const entries = collateralMints.map((entry) => [
        "        {",
        `            symbol: ${JSON.stringify(entry.symbol)},`,
        `            mint: ${JSON.stringify(entry.mint)},`,
        `            tokenProgram: ${JSON.stringify(entry.tokenProgram)},`,
        `            transferHook: ${JSON.stringify(entry.transferHook)},`,
        "        },",
    ].join("\n"));
    const network = activeNetwork();
    const usdc = collateralMints.find((entry) => entry.symbol === "USDC");
    return [
        'import type { ChanceryNetworkIdentity } from "../network.js";',
        "",
        "// Devnet collateral uses the configured SPL Token or Token-2022 profile.",
        "export const devnetNetworkIdentity: ChanceryNetworkIdentity = {",
        '    name: "devnet",',
        '    cluster: "devnet",',
        `    programId: ${JSON.stringify(programId)},`,
        `    faucetProgramId: ${JSON.stringify(network.faucetProgramId)},`,
        `    usdvMint: ${JSON.stringify(network.usdvMint)},`,
        `    usdcMint: ${JSON.stringify(usdc?.mint ?? null)},`,
        `    legacyUsdvMint: ${JSON.stringify(legacyUsdvMint)},`,
        "    collateralMints: [",
        ...entries,
        "    ],",
        "};",
        "",
    ].join("\n");
}

async function main(): Promise<void> {
    preflight();
    const network = activeNetwork();
    const feePayer = loadFeePayer();
    const setupAuthority = loadSetupAuthority();
    const keypairDirectory = resolve(dirname(network.chanceryManifestPath), "test-assets");

    const specifications: MintSpecification[] = TEST_ASSET_PROFILES.map((profile) => ({
        symbol: profile.symbol,
        role: profile.role,
        decimals: profile.decimals,
        tokenProgram: profile.tokenProgramKind === "token-2022" ? TOKEN_2022_PROGRAM_ID : TOKEN_PROGRAM_ID,
        tokenProgramKind: profile.tokenProgramKind,
        transferHook: profile.transferHook,
        signerPath: resolve(keypairDirectory, `${profile.symbol.toLowerCase()}-mint-keypair.json`),
    }));

    printContext("Create devnet test assets", {
        assets: specifications.map((entry) => entry.symbol).join(", "),
        mintAuthority: setupAuthority.publicKey.toBase58(),
        keypairs: workspaceRelativePath(keypairDirectory),
    });

    const context = await createDevnetHarness({
        commitment: "finalized",
        requireProgramdata: false,
        rpcUrl: network.rpcUrl,
    });

    if (network.devnetAirdrop.enabled) {
        try {
            await requestAirdropAndConfirm(feePayer.publicKey, network.devnetAirdrop.lamports, {
                rpcUrl: network.rpcUrl,
                commitment: "finalized",
                requireProgramdata: false,
            });
        } catch (error) {
            if (network.devnetAirdrop.required) throw error;
            console.log(`airdrop unavailable, continuing with the existing fee-payer balance: ${String(error)}`);
        }
    }

    const collateral: ChanceryCollateralMint[] = [];
    let legacyUsdvMint: string | null = null;
    for (const specification of specifications) {
        const signer = loadOrGenerateSigner(specification.signerPath, `${specification.symbol} mint`);
        const mint = await ensureMint(context, feePayer, setupAuthority.publicKey, specification, signer);
        if (specification.role === "legacy-migration-source") {
            legacyUsdvMint = mint;
            continue;
        }
        if (specification.symbol === "USDV-LEGACY") {
            throw new Error("USDV-LEGACY cannot be recorded as a collateral asset");
        }
        collateral.push({
            symbol: specification.symbol,
            mint,
            tokenProgram: specification.tokenProgramKind,
            transferHook: specification.transferHook,
        });
    }
    if (legacyUsdvMint === null) throw new Error("the legacy migration source mint was not created");

    replaceSourceAtomically(IDENTITY_PATH, identitySource(network.programId, collateral, legacyUsdvMint));
    console.log(`\nwrote ${collateral.length + 1} test mints to ${workspaceRelativePath(IDENTITY_PATH)}`);
    for (const entry of collateral) {
        console.log(`  ${entry.symbol.padEnd(12)} ${entry.mint}  ${entry.tokenProgram}  hook:${entry.transferHook}`);
    }
    console.log(`  ${"USDV-LEGACY".padEnd(12)} ${legacyUsdvMint}  spl-token  migration source`);
    console.log("\nUse assets:mint before faucet handoff; use faucet:mint after handoff.");
}

main().catch((error: unknown) => {
    console.error(error instanceof Error ? error.stack ?? error.message : String(error));
    process.exitCode = 1;
});
