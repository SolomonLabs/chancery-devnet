import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { spawnSync } from "node:child_process";

import { sanitizedProcessEnvironment } from "./processEnvironment.js";
import { redactCommandOutput, redactedCommand } from "./safeOutput.js";

export interface LiveProgramInfo {
    programId: string;
    programdataAddress: string;
    upgradeAuthority: string | null;
    raw: Record<string, unknown>;
}

function capture(command: string, args: string[]): string {
    console.log(`\n$ ${redactedCommand(command, args)}`);
    const result = spawnSync(command, args, { encoding: "utf8", env: sanitizedProcessEnvironment() });
    if (result.error) throw result.error;
    if (result.status !== 0) throw new Error(`${command} failed: ${redactCommandOutput((result.stderr ?? result.stdout ?? "").trim(), args)}`);
    return result.stdout.trim();
}

export function readLiveProgramInfo(rpcUrl: string, programId: string): LiveProgramInfo {
    const source = capture("solana", ["program", "show", "--url", rpcUrl, programId, "--output", "json"]);
    let raw: Record<string, unknown>;
    try {
        raw = JSON.parse(source) as Record<string, unknown>;
    } catch (error) {
        throw new Error("solana program show returned invalid JSON", { cause: error });
    }
    const programdataAddress = String(raw.programdataAddress ?? raw.programDataAddress ?? "");
    if (programdataAddress.length === 0) throw new Error("live program readback omitted ProgramData address");
    const authorityValue = raw.authority ?? raw.upgradeAuthority ?? raw.upgradeAuthorityAddress ?? null;
    // `solana program show` reports a revoked upgrade authority as the literal
    // string "none" rather than omitting the field, so a length check alone lets
    // it through as if it were an address. Immutable programs are the norm on
    // mainnet - Squads v4 is one - and were never exercised on devnet.
    const authorityText = authorityValue === null || authorityValue === undefined ? "" : String(authorityValue).trim();
    const upgradeAuthority = authorityText.length === 0 || authorityText.toLowerCase() === "none"
        ? null
        : authorityText;
    return { programId, programdataAddress, upgradeAuthority, raw };
}

export function readLiveProgramInfoFromBoth(
    primaryRpcUrl: string,
    secondaryRpcUrl: string,
    programId: string,
): LiveProgramInfo {
    const primary = readLiveProgramInfo(primaryRpcUrl, programId);
    const secondary = readLiveProgramInfo(secondaryRpcUrl, programId);
    if (primary.programdataAddress !== secondary.programdataAddress) {
        throw new Error(
            `RPC providers disagree on ProgramData address: ${primary.programdataAddress} != ${secondary.programdataAddress}`,
        );
    }
    if (primary.upgradeAuthority !== secondary.upgradeAuthority) {
        throw new Error(
            `RPC providers disagree on upgrade authority: ${primary.upgradeAuthority ?? "<revoked>"} != `
            + `${secondary.upgradeAuthority ?? "<revoked>"}`,
        );
    }
    return primary;
}

/**
 * Trailing-zero-stripped SHA-256 - solana-verify's executable-hash semantics.
 * A ProgramData account keeps its original allocation across upgrades, so a
 * raw dump of a smaller ELF carries zero-padding and can never equal the raw
 * file hash. Fresh deploys size the account exactly, which is why every
 * rehearsal passed the raw comparison until the first differently-sized
 * mainnet redeploy (finding #8).
 */
export function normalizedElfSha256(bytes: Uint8Array): string {
    let end = bytes.length;
    while (end > 0 && bytes[end - 1] === 0) end -= 1;
    return createHash("sha256").update(bytes.subarray(0, end)).digest("hex");
}

export function normalizedElfSha256FromFile(path: string): string {
    return normalizedElfSha256(readFileSync(path));
}

/** Normalized live-ELF hash; raw hashLiveProgramElf below is retained for
 * Squads policy hashes recorded pre-normalization in the frozen plan. */
export function hashLiveProgramElfNormalized(rpcUrl: string, programId: string): string {
    const directory = mkdtempSync(join(tmpdir(), "chancery-live-program-"));
    const output = join(directory, "chancery-live.so");
    try {
        const args = ["program", "dump", "--url", rpcUrl, programId, output];
        console.log(`\n$ ${redactedCommand("solana", args)}`);
        const result = spawnSync("solana", args, { stdio: "inherit", env: sanitizedProcessEnvironment() });
        if (result.error) throw result.error;
        if (result.status !== 0) throw new Error("solana program dump failed");
        return normalizedElfSha256(readFileSync(output));
    } finally {
        rmSync(directory, { recursive: true, force: true });
    }
}

export function hashLiveProgramElf(rpcUrl: string, programId: string): string {
    const directory = mkdtempSync(join(tmpdir(), "chancery-live-program-"));
    const output = join(directory, "chancery-live.so");
    try {
        const args = ["program", "dump", "--url", rpcUrl, programId, output];
        console.log(`\n$ ${redactedCommand("solana", args)}`);
        const result = spawnSync("solana", args, { stdio: "inherit", env: sanitizedProcessEnvironment() });
        if (result.error) throw result.error;
        if (result.status !== 0) throw new Error("solana program dump failed");
        return createHash("sha256").update(readFileSync(output)).digest("hex");
    } finally {
        rmSync(directory, { recursive: true, force: true });
    }
}

export function verifyLiveProgramArtifact(args: {
    rpcUrl: string;
    secondaryRpcUrl?: string;
    programId: string;
    expectedElfSha256: string;
    expectedUpgradeAuthority: string;
}): LiveProgramInfo {
    const primaryInfo = readLiveProgramInfo(args.rpcUrl, args.programId);
    if (primaryInfo.upgradeAuthority !== args.expectedUpgradeAuthority) {
        throw new Error(
            `live upgrade authority ${primaryInfo.upgradeAuthority ?? "<revoked>"} != expected ${args.expectedUpgradeAuthority}`,
        );
    }
    // Both sides normalized: expectedElfSha256 must be a trailing-zero-
    // stripped hash (normalizedElfSha256FromFile of the reviewed artifact).
    const primaryHash = hashLiveProgramElfNormalized(args.rpcUrl, args.programId);
    if (primaryHash !== args.expectedElfSha256) {
        throw new Error(`live program ELF hash ${primaryHash} != reviewed release ELF ${args.expectedElfSha256}`);
    }
    if (args.secondaryRpcUrl !== undefined) {
        const secondaryInfo = readLiveProgramInfo(args.secondaryRpcUrl, args.programId);
        if (secondaryInfo.programdataAddress !== primaryInfo.programdataAddress) {
            throw new Error(
                `RPC providers disagree on ProgramData address: ${primaryInfo.programdataAddress} != ${secondaryInfo.programdataAddress}`,
            );
        }
        if (secondaryInfo.upgradeAuthority !== primaryInfo.upgradeAuthority) {
            throw new Error(
                `RPC providers disagree on upgrade authority: ${primaryInfo.upgradeAuthority ?? "<revoked>"} != `
                + `${secondaryInfo.upgradeAuthority ?? "<revoked>"}`,
            );
        }
        const secondaryHash = hashLiveProgramElfNormalized(args.secondaryRpcUrl, args.programId);
        if (secondaryHash !== primaryHash) {
            throw new Error(`RPC providers disagree on live program ELF hash: ${primaryHash} != ${secondaryHash}`);
        }
    }
    return primaryInfo;
}
