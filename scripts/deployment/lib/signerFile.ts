import { chmodSync, existsSync, lstatSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";

import { parseSolanaKeypairSource } from "../../core/SolanaKeypairFile.mjs";
import { generateSigner, signerFromSolanaKeypairBytes, type E2ESigner } from "./solanaSigner.js";

export function writeGeneratedSigner(path: string, signer: E2ESigner): void {
    const directory = dirname(path);
    mkdirSync(directory, { recursive: true, mode: 0o700 });
    const status = lstatSync(directory);
    if (!status.isDirectory() || status.isSymbolicLink()) {
        throw new Error("keypair directory must be a regular directory: " + directory);
    }
    if (process.platform !== "win32") chmodSync(directory, 0o700);
    const bytes = new Uint8Array(64);
    bytes.set(signer.seed, 0);
    bytes.set(signer.publicKey.toBytes(), 32);
    try {
        writeFileSync(path, JSON.stringify(Array.from(bytes)) + "\n", {
            encoding: "utf8", flag: "wx", mode: 0o600,
        });
        if (process.platform !== "win32") chmodSync(path, 0o600);
    } finally {
        bytes.fill(0);
    }
}

export function loadSignerFile(path: string): E2ESigner {
    const bytes = parseSolanaKeypairSource(readFileSync(path, "utf8"), path);
    try {
        return signerFromSolanaKeypairBytes(bytes);
    } finally {
        bytes.fill(0);
    }
}

export function loadOrGenerateSigner(path: string, label: string): E2ESigner {
    if (existsSync(path)) {
        return loadSignerFile(path);
    }
    const signer = generateSigner();
    try {
        writeGeneratedSigner(path, signer);
        console.log("generated " + label + " " + signer.publicKey.toBase58());
        return signer;
    } catch (error: unknown) {
        signer.seed.fill(0);
        throw error;
    }
}
