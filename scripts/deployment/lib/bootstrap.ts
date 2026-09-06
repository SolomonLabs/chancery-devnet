// Shared config for the devnet bootstrap scripts. config/runtime/devnet.json is
// the sole source for RPC, addresses, keypair paths, and output paths.
//
// Flags: --dry-run previews/simulates without submission.

import { existsSync, readFileSync } from "node:fs";

import { Base58Util } from "@solomon-labs/base58";
import { PublicKey } from "@solomon-labs/publickey";
import type { Base58 } from "@solomon-labs/types";

import { generateSigner, signerFromSolanaKeypairBytes, type E2ESigner } from "./solanaSigner.js";
import { loadOrGenerateSigner } from "./signerFile.js";
import { PROGRAM_ID } from "../../../clients/ts/src/constants.js";
import { resolveNetwork, resolveWorkspacePath, type ChanceryNetwork } from "../../../config/network.js";
import { assertSupportedNodeVersion } from "./processEnvironment.js";
import { redactRpcUrl } from "./safeOutput.js";
import { parseSolanaKeypairSource } from "../../core/SolanaKeypairFile.mjs";

const NET: ChanceryNetwork = resolveNetwork("devnet");

export const DRY_RUN = process.argv.includes("--dry-run");

export function activeNetwork(): ChanceryNetwork {
    return NET;
}

export function expandLocalPath(path: string): string {
    return resolveWorkspacePath(path);
}

export function publicKeyFromAddress(address: string): PublicKey {
    return new PublicKey(Base58Util.decode(address as Base58));
}

function loadSigner(path: string, previewPubkey?: PublicKey): E2ESigner {
    const abs = expandLocalPath(path);
    if (!existsSync(abs)) {
        const preview = previewPubkey === undefined ? "" : ` (expected ${previewPubkey.toBase58()})`;
        throw new Error(
            `keypair missing at ${abs}${preview}` +
            (DRY_RUN ? "; transaction simulation still requires real signatures" : ""),
        );
    }
    return signerFromSolanaKeypairBytes(parseSolanaKeypairSource(readFileSync(abs, "utf8"), abs));
}

/** Setup/authority signer: signs init/configure/register and becomes all five slots at init. */
export function loadSetupAuthority(): E2ESigner {
    return loadSigner(NET.setupAuthorityKeypairPath);
}

/** Loader-v3 ProgramData upgrade authority; required by initialize_chancery. */
export function loadUpgradeAuthority(): E2ESigner {
    const path = NET.upgradeAuthorityKeypairPath;
    if (path === null) {
        throw new Error(
            `upgrade_authority_keypair_path is not set in ${NET.runtimeConfigPath}`,
        );
    }
    return loadSigner(path);
}

/** Fee payer for program deployment and initialization transactions. */
export function loadFeePayer(): E2ESigner {
    return loadSigner(NET.feePayerKeypairPath);
}

/** The issued-token (USDV) MINT keypair. Loaded from a path, or generated on devnet. */
export function loadIssuedMintSigner(): E2ESigner {
    const path = NET.issuedMintKeypairPath;
    if (path === null) return generateSigner();
    const expectedPubkey = NET.usdvMint === null
        ? undefined
        : publicKeyFromAddress(NET.usdvMint);
    const signer = expectedPubkey === undefined && !DRY_RUN
        ? loadOrGenerateSigner(path, "issued mint")
        : loadSigner(path, expectedPubkey);
    if (expectedPubkey !== undefined && !signer.publicKey.equals(expectedPubkey)) {
        throw new Error(`issued-mint keypair pubkey ${signer.publicKey.toBase58()} != usdvMint ${NET.usdvMint} for ${NET.name}`);
    }
    return signer;
}

/** Issued token (USDV) mint address. */
export function issuedTokenMint(): PublicKey {
    if (NET.usdvMint === null) {
        throw new Error(`usdvMint is not set in config/networks/${NET.name}.ts`);
    }
    return publicKeyFromAddress(NET.usdvMint);
}

/** Collateral (USDC) mint address. */
export function collateralMint(): PublicKey {
    if (NET.usdcMint === null) {
        throw new Error(`usdcMint is not set in config/networks/${NET.name}.ts`);
    }
    return publicKeyFromAddress(NET.usdcMint);
}

/** Legacy USDV migration source, read only from the selected network config. */
export function legacyTokenMint(): PublicKey {
    if (NET.legacyUsdvMint === null) {
        throw new Error(`legacyUsdvMint is not set in config/networks/${NET.name}.ts`);
    }
    return publicKeyFromAddress(NET.legacyUsdvMint);
}

/**
 * The scripts derive PDAs and target instructions with the generated client's
 * PROGRAM_ID. If it doesn't match the selected network's programId, the client
 * hasn't been regenerated for this deployment - everything would target the
 * wrong program. Guard it (hard fail on a real run, warn under --dry-run).
 */
export function assertProgramIdMatches(): void {
    const generated = PROGRAM_ID.toBase58();
    if (generated === NET.programId) return;
    const msg =
        `generated client PROGRAM_ID ${generated} != ${NET.name} programId ${NET.programId}. ` +
        `Run 'yarn identity' so Rust, IDL, generated client, and PDAs target it.`;
    if (DRY_RUN) console.warn(`[dry-run] warning: ${msg}`);
    else throw new Error(msg);
}

/** Run every guard. Call first in every bootstrap script. */
export function preflight(): void {
    assertSupportedNodeVersion();
    assertProgramIdMatches();
    if (NET.rpcUrl.trim().length === 0) {
        throw new Error(`rpc_url must be set in ${NET.runtimeConfigPath}`);
    }
}

/** Print the resolved network context; call at the top of every bootstrap script. */
export function printContext(title: string, extra: Record<string, string> = {}): void {
    console.log(`\n${title} - network ${NET.name} (${NET.cluster})`);
    console.log(`  rpc        ${redactRpcUrl(NET.rpcUrl)}`);
    console.log(`  programId  ${NET.programId}`);
    for (const [k, v] of Object.entries(extra)) console.log(`  ${k.padEnd(10)} ${v}`);
    if (DRY_RUN) console.log(`  [dry-run] no transactions will be submitted`);
}
