/// <reference types="node" />

import { createHash } from "node:crypto";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";

import { PublicKey } from "@solomon-labs/combined";
import type { SolanaAddress } from "@solomon-labs/types";
import {
    CreateAccountInstruction,
    InitializeMint2Instruction,
    TOKEN_2022_PROGRAM_ID,
    Token2022ConfidentialTransferMint,
    Token2022DefaultAccountState,
    Token2022MetadataPointer,
    Token2022MintCloseAuthority,
    Token2022PermanentDelegate,
    Token2022TransferHook,
    type EncodedInstruction,
} from "@solomon-labs/combined";
import { toSolanaKey } from "../../runtime/wellKnown.js";
import { createDevnetHarness, requestAirdropAndConfirm } from "../../runtime/devnetHarness.js";
import { submitTransaction, type E2ESigner } from "../../runtime/submit.js";
import {
    closeMintAuthorityPda,
    confidentialTransferAuthorityPda,
    freezeAuthorityPda,
    metadataPointerAuthorityPda,
    metadataUpdateAuthorityPda,
    mintAuthorityPda,
    pauseAuthorityPda,
    permanentDelegateAuthorityPda,
    transferHookAuthorityPda,
} from "../../runtime/pdas.js";
import { activeNetwork, loadIssuedMintSigner, loadSetupAuthority, preflight, printContext } from "./lib/bootstrap.js";
import { verifyIssuedTokenMintAccount } from "./lib/verifyIssuedTokenMint.js";
import { redactRpcUrl } from "./lib/safeOutput.js";
import { readIssuedTokenLaunchConfig } from "../release/IssuedTokenLaunchConfig.mjs";
import { recordIssuedMintIdentity } from "./lib/issuedMintIdentity.js";

const TOKEN_2022_PROGRAM_ID_STRING = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";

// Token-2022 extended-mint layout: 165-byte base + 1 account-type byte, then TLV entries (4-byte header + value).
const BASE = 166;
const TLV_HEADER = 4;

function cat(...parts: Uint8Array[]): Uint8Array {
    const out = new Uint8Array(parts.reduce((n, p) => n + p.length, 0));
    let o = 0; for (const p of parts) { out.set(p, o); o += p.length; }
    return out;
}
const u8s = (...xs: number[]): Uint8Array => new Uint8Array(xs);
const ZERO32 = new Uint8Array(32);

// On-chain TokenMetadata is sourced from the machine-readable, release-gated
// issued-token launch configuration. Metadata authority is intentionally a
// Chancery PDA; changing these values later requires a reviewed program upgrade.
// SPL Token-Metadata Interface `Initialize` discriminator. Derived from the
// canonical namespace (sha256(...)[..8]) rather than a magic-number literal, so
// it's self-verifying against the interface spec. The codec ships no builder for
// this instruction (only a Token2022TokenMetadata *state* encoder), so - like the
// other extension inits and InitializeMint2 in this file - it's assembled by hand.
const TOKEN_METADATA_INIT_DISC = new Uint8Array(
    createHash("sha256").update("spl_token_metadata_interface:initialize_account").digest().subarray(0, 8),
);

/** Borsh string: 4-byte LE length prefix + UTF-8 bytes. */
function borshString(s: string): Uint8Array {
    const bytes = new TextEncoder().encode(s);
    const out = new Uint8Array(4 + bytes.length);
    new DataView(out.buffer).setUint32(0, bytes.length, true);
    out.set(bytes, 4);
    return out;
}

interface ExtensionAuthorities {
    transferHook:         PublicKey;
    confidentialTransfer: PublicKey;
    permanentDelegate:    PublicKey;
    pause:                PublicKey;
    metadataPointer:      PublicKey;
    closeMint:            PublicKey;
}

interface Token2022StateCodec {
    encode(): Uint8Array;
}

type Token2022StateConstructor<T extends Token2022StateCodec> = new (properties: T) => T;
type Token2022StateProperties<T extends Token2022StateCodec> = Omit<T, keyof Token2022StateCodec>;

/**
 * The current Solomon declarations type Token-2022 state constructors as
 * accepting a complete instance (`T`), even though their runtime constructors
 * accept only the data fields and apply them with `Object.assign`. Keep the
 * declaration workaround isolated here while preserving field-level checking.
 */
function encodeToken2022State<T extends Token2022StateCodec>(
    State: Token2022StateConstructor<T>,
    properties: Token2022StateProperties<T>,
): Uint8Array {
    return new State(properties as unknown as T).encode();
}

/**
 * The mint-level extensions installed on the issued token. Each reserved
 * extension's authority/delegate is a Chancery **PDA** (issue-72), NOT the
 * deployer - `verify_issued_token_deployment` now rejects any that isn't. A PDA
 * authority is the genuinely inert state: only the Chancery program could ever
 * sign for it, and there is no instruction that does. ConfidentialTransfer is
 * installed but OFF (`auto_approve = false`, no auditor) so it's dormant yet
 * enableable later by governance. Discriminants are Token-2022 instruction bytes.
 */
function mintExtensions(auth: ExtensionAuthorities, mint: PublicKey): Array<{ name: string; bit: number; valueLen: number; data: Uint8Array }> {
    const transferHook = encodeToken2022State(Token2022TransferHook, {
        authority: auth.transferHook,
        programId: new PublicKey(ZERO32),
    });
    const confidentialTransfer = encodeToken2022State(Token2022ConfidentialTransferMint, {
        authority: auth.confidentialTransfer,
        autoApproveNewAccounts: false,
        auditorElgamalPubkey: ZERO32,
    });
    const permanentDelegate = encodeToken2022State(Token2022PermanentDelegate, {
        delegate: auth.permanentDelegate,
    });
    const metadataPointer = encodeToken2022State(Token2022MetadataPointer, {
        authority: auth.metadataPointer,
        metadataAddress: mint,
    });
    const closeMint = encodeToken2022State(Token2022MintCloseAuthority, {
        closeAuthority: auth.closeMint,
    });
    const defaultAccountState = encodeToken2022State(Token2022DefaultAccountState, { state: 1 });

    return [
        { name: "TransferHook",         bit: 0,  valueLen: transferHook.length, data: cat(u8s(36, 0), transferHook) },
        { name: "ConfidentialTransfer", bit: 1,  valueLen: confidentialTransfer.length, data: cat(u8s(27, 0), confidentialTransfer) },
        { name: "PermanentDelegate",    bit: 3,  valueLen: permanentDelegate.length, data: cat(u8s(35), permanentDelegate) },
        // InitializePausableConfig always starts unpaused and takes only its authority.
        { name: "Pausable",             bit: 4,  valueLen: 33, data: cat(u8s(44, 0), auth.pause.toBytes()) },
        { name: "MetadataPointer",      bit: 5,  valueLen: metadataPointer.length, data: cat(u8s(39, 0), metadataPointer) },
        { name: "MintCloseAuthority",   bit: 9,  valueLen: closeMint.length, data: cat(u8s(25, 1), closeMint) },
        { name: "DefaultAccountState",  bit: 10, valueLen: defaultAccountState.length, data: cat(u8s(28, 0), defaultAccountState) },
    ];
}

async function maybeAirdrop(
    authority: E2ESigner,
    rpcUrl: string,
    secondaryRpcUrl: string | undefined,
    settings: { enabled: boolean; required: boolean; lamports: bigint },
): Promise<void> {
    if (!settings.enabled) return;
    try {
        await requestAirdropAndConfirm(authority.publicKey, settings.lamports, {
            requireProgramdata: false,
            rpcUrl,
            secondaryRpcUrl,
            commitment: "finalized",
        });
    } catch (error) {
        if (settings.required) throw error;
    }
}

function outputPath(): string {
    return activeNetwork().issuedMintManifestPath;
}

async function launchIssuedTokenMint(): Promise<void> {
    const { config: launchConfig, configSha256 } = await readIssuedTokenLaunchConfig();
    // Devnet does not fetch the hosted metadata document: it is a production
    // asset this workspace neither controls nor serves, and a devnet launch
    // must not depend on it being reachable. Every on-chain metadata assertion
    // (name, symbol, URI, update lock-in) still applies below, and the pinned
    // config hash is recorded as the expectation.
    console.log(
        "devnet: skipping hosted issued-token metadata fetch; "
        + `recorded expectation ${launchConfig.metadata.offchain_json_sha256}`,
    );
    const offchainMetadataSha256 = launchConfig.metadata.offchain_json_sha256;
    const METADATA_NAME = launchConfig.metadata.name;
    const METADATA_SYMBOL = launchConfig.metadata.symbol;
    const METADATA_URI = launchConfig.metadata.uri;
    const SYMBOL = launchConfig.metadata.symbol;
    const DECIMALS = launchConfig.decimals;

    // Gate first: require the generated client to target this programId.
    preflight();
    printContext("launch issued token mint");

    const net = activeNetwork();
    const dryRun = process.argv.includes("--dry-run");
    const verifyOnly = process.argv.includes("--verify-only");
    const requireZeroSupply = process.argv.includes("--require-zero-supply");
    const authority = verifyOnly ? null : loadSetupAuthority();

    if (!dryRun && !verifyOnly) {
        if (authority === null) throw new Error("internal error: launch authority was not loaded");
        await maybeAirdrop(
            authority,
            net.rpcUrl,
            undefined,
            net.devnetAirdrop,
        );
    }

    // The harness submits REAL transactions via RPC. Point it at THIS network's
    // endpoint from config/networks/<net>.ts (not the baked-in devnet default),
    // and on mainnet require the Chancery program to actually be deployed at
    // PROGRAM_ID before we bind a new mint to its authority PDAs.
    // Use finalized reads and confirmation for launch. `submitTransaction` also
    // simulates the exact signed transaction before broadcasting it.
    const ctx = await createDevnetHarness({
        commitment: "finalized",
        // The mint is intentionally created before the program is deployed.
        // Its PDA authorities are deterministic from the precommitted program ID.
        requireProgramdata: false,
        rpcUrl: net.rpcUrl,
    });
    const [mintAuthorityKey] = await mintAuthorityPda();
    const [freezeAuthorityKey] = await freezeAuthorityPda();
    const mintAuthority = toSolanaKey(mintAuthorityKey);
    const freezeAuthority = toSolanaKey(freezeAuthorityKey);

    // issue-72: each reserved extension's authority is its Chancery PDA, not the deployer.
    const [
        [transferHookAuth],
        [confidentialAuth],
        [permanentDelegateAuth],
        [pauseAuth],
        [metadataAuth],
        [closeAuth],
        [metadataUpdateKey],
    ] = await Promise.all([
        transferHookAuthorityPda(),
        confidentialTransferAuthorityPda(),
        permanentDelegateAuthorityPda(),
        pauseAuthorityPda(),
        metadataPointerAuthorityPda(),
        closeMintAuthorityPda(),
        metadataUpdateAuthorityPda(),
    ]);
    const metadataUpdate = toSolanaKey(metadataUpdateKey);

    const mintSigner = verifyOnly || (!dryRun && net.issuedMintKeypairPath === null && net.usdvMint !== null)
        ? null : loadIssuedMintSigner();
    // The persistent mint keypair recovers a confirmed launch even when output writing was interrupted.
    const candidate = net.usdvMint ?? mintSigner?.publicKey.toBase58() ?? null;
    if (!dryRun && candidate !== null) {
        const existingMint = new PublicKey(candidate as SolanaAddress);
        const existingAccount = await ctx.banksClient.getAccount(existingMint);
        if (existingAccount !== null) {
            const rentFunction = ctx.banksClient.getMinimumBalanceForRentExemption;
            if (rentFunction === undefined) throw new Error("runtime cannot calculate rent exemption");
            const minimumRent = await rentFunction.call(ctx.banksClient, existingAccount.data.length);
            verifyIssuedTokenMintAccount(existingAccount, {
                mint: existingMint,
                mintAuthority,
                freezeAuthority,
                decimals: DECIMALS,
                transferHookAuthority: toSolanaKey(transferHookAuth),
                confidentialTransferAuthority: toSolanaKey(confidentialAuth),
                permanentDelegate: toSolanaKey(permanentDelegateAuth),
                pauseAuthority: toSolanaKey(pauseAuth),
                metadataPointerAuthority: toSolanaKey(metadataAuth),
                closeAuthority: toSolanaKey(closeAuth),
                metadataUpdateAuthority: metadataUpdate,
                metadata: { name: METADATA_NAME, symbol: METADATA_SYMBOL, uri: METADATA_URI },
            }, minimumRent, { requireZeroSupply });
            if (!verifyOnly) recordIssuedMintIdentity(candidate);
            console.log("issued mint " + candidate + " passed finalized-state verification");
            return;
        }
    }
    if (verifyOnly) {
        throw new Error("the configured issued Token-2022 mint is absent on " + net.name);
    }
    if (mintSigner === null) throw new Error("the configured issued mint is absent and its keypair path is unset");
    if (authority === null) throw new Error("issued-mint launch authority was not loaded");

    const exts = mintExtensions({
        transferHook:         transferHookAuth,
        confidentialTransfer: confidentialAuth,
        permanentDelegate:    permanentDelegateAuth,
        pause:                pauseAuth,
        metadataPointer:      metadataAuth,
        closeMint:            closeAuth,
    }, mintSigner.publicKey);
    const space = BASE + exts.reduce((n, e) => n + TLV_HEADER + e.valueLen, 0);
    // TokenMetadata is created after InitializeMint2 and therefore is not in
    // `exts`, but it is still an installed mint extension (Chancery bit 6).
    const observedMask = exts.reduce((m, e) => m | (1 << e.bit), 0) | (1 << 6);

    const extInstructions: EncodedInstruction[] = exts.map((e) => ({
        programId: TOKEN_2022_PROGRAM_ID,
        accounts: [{ pubkey: mintSigner.publicKey, isSigner: false, isWritable: true }],
        data: e.data,
    }));

    // The mint authority is TEMPORARILY the deployer: the token-metadata Initialize
    // below requires the mint authority to SIGN, but our final authority is a PDA -
    // we hand it to the PDA via SetAuthority at the end.
    const initializeMint2 = new InitializeMint2Instruction({
        programId: TOKEN_2022_PROGRAM_ID,
        mint: mintSigner.publicKey,
        decimals: DECIMALS,
        mintAuthority: authority.publicKey,
        freezeAuthority,
    });

    // TokenMetadata Initialize (SPL Token-Metadata Interface), self-hosted on the
    // mint (MetadataPointer points at the mint). Accounts: [metadata(=mint, w),
    // updateAuthority, mint, mintAuthority(signer)]. Reallocs the account for the
    // variable-length strings - the 12M lamports above cover the extra rent.
    const metadataInit = {
        programId: TOKEN_2022_PROGRAM_ID,
        accounts: [
            { pubkey: mintSigner.publicKey, isSigner: false, isWritable: true },
            { pubkey: metadataUpdate,       isSigner: false, isWritable: false },
            { pubkey: mintSigner.publicKey, isSigner: false, isWritable: false },
            { pubkey: authority.publicKey,  isSigner: true,  isWritable: false },
        ],
        data: cat(TOKEN_METADATA_INIT_DISC, borshString(METADATA_NAME), borshString(METADATA_SYMBOL), borshString(METADATA_URI)),
    };

    // Hand the mint authority to the Chancery PDA (SetAuthority, AuthorityType MintTokens=0).
    const setMintAuthority = {
        programId: TOKEN_2022_PROGRAM_ID,
        accounts: [
            { pubkey: mintSigner.publicKey, isSigner: false, isWritable: true },
            { pubkey: authority.publicKey,  isSigner: true,  isWritable: false },
        ],
        data: cat(u8s(6, 0, 1), mintAuthority.toBytes()),
    };

    await submitTransaction(ctx, [
        new CreateAccountInstruction({
            from: authority.publicKey,
            newAccount: mintSigner.publicKey,
            lamports: 12_000_000n,
            space,
            owner: TOKEN_2022_PROGRAM_ID,
        }),
        ...extInstructions,
        initializeMint2,
        metadataInit,
        setMintAuthority,
    ], [mintSigner], authority, { simulateOnly: dryRun });

    if (dryRun) {
        console.log("issued mint transaction simulation succeeded; --dry-run was set, so nothing was broadcast");
        return;
    }

    const account = await ctx.banksClient.getAccount(mintSigner.publicKey);
    if (account === null) {
        throw new Error("issued mint verification failed: finalized account was not found");
    }
    const rentFunction = ctx.banksClient.getMinimumBalanceForRentExemption;
    if (rentFunction === undefined) {
        throw new Error("issued mint verification failed: runtime cannot calculate rent exemption");
    }
    const minimumRentLamports = await rentFunction.call(ctx.banksClient, account.data.length);
    const decodedMint = verifyIssuedTokenMintAccount(account, {
        mint: mintSigner.publicKey,
        mintAuthority,
        freezeAuthority,
        decimals: DECIMALS,
        transferHookAuthority: toSolanaKey(transferHookAuth),
        confidentialTransferAuthority: toSolanaKey(confidentialAuth),
        permanentDelegate: toSolanaKey(permanentDelegateAuth),
        pauseAuthority: toSolanaKey(pauseAuth),
        metadataPointerAuthority: toSolanaKey(metadataAuth),
        closeAuthority: toSolanaKey(closeAuth),
        metadataUpdateAuthority: metadataUpdate,
        metadata: { name: METADATA_NAME, symbol: METADATA_SYMBOL, uri: METADATA_URI },
    }, minimumRentLamports, { requireZeroSupply: true });

    const manifest = {
        cluster: net.cluster,
        symbol: SYMBOL,
        rpcEndpoint: redactRpcUrl(net.rpcUrl),
        mint: mintSigner.publicKey.toBase58(),
        mintAuthority: mintAuthority.toBase58(),
        freezeAuthority: freezeAuthority.toBase58(),
        // issue-72: the Chancery PDAs each reserved extension's authority is bound
        // to. verify_issued_token_deployment requires the on-chain values to match.
        extensionAuthorities: {
            transferHook:         transferHookAuth.toBase58(),
            confidentialTransfer: confidentialAuth.toBase58(),
            permanentDelegate:    permanentDelegateAuth.toBase58(),
            pause:                pauseAuth.toBase58(),
            metadataPointer:      metadataAuth.toBase58(),
            closeMint:            closeAuth.toBase58(),
            metadataUpdate:       metadataUpdate.toBase58(),
        },
        // Self-hosted TokenMetadata written onto the mint (metadataAddress = mint).
        metadata: {
            name: METADATA_NAME,
            symbol: METADATA_SYMBOL,
            uri: METADATA_URI,
            updateAuthority: metadataUpdate.toBase58(),
            offchainJsonSha256: offchainMetadataSha256,
        },
        launchConfigurationSha256: configSha256,
        metadataUpdatePath: "requires-chancery-upgrade",
        tokenProgram: TOKEN_2022_PROGRAM_ID_STRING,
        decimals: DECIMALS,
        accountDataLength: account.data.length,
        accountLamports: account.lamports.toString(),
        decodedExtensionTypes: decodedMint.tlvData.entries.map((entry) => entry.extensionTypeName),
        observedMintExtensionMask: [observedMask, 0],
        installedExtensions: [...exts.map((e) => e.name), "TokenMetadata"],
    };

    const path = outputPath();
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, `${JSON.stringify(manifest, null, 2)}\n`);
    recordIssuedMintIdentity(mintSigner.publicKey.toBase58());
    console.log(JSON.stringify(manifest, null, 2));
    console.log(`\nmanifest written to ${path}`);
}

launchIssuedTokenMint().catch((error: unknown) => {
    console.error(error instanceof Error ? error.stack ?? error.message : String(error));
    process.exitCode = 1;
});
