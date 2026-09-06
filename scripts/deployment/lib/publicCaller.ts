import { readFileSync } from "node:fs";
import { homedir } from "node:os";
import { resolve } from "node:path";

import { signerFromSolanaKeypairBytes, type E2ESigner } from "../../../runtime/submit.js";
import { parseSolanaKeypairSource } from "../../core/SolanaKeypairFile.mjs";
import { loadFeePayer } from "./bootstrap.js";

export function flagValue(flag: string, arguments_: readonly string[] = process.argv.slice(2)): string | null {
    const index = arguments_.indexOf(flag);
    if (index < 0) return null;
    const value = arguments_[index + 1];
    if (value === undefined || value.startsWith("--")) throw new Error(flag + " requires a value");
    return value;
}

export function loadPublicCaller(): E2ESigner {
    const configured = flagValue("--keypair");
    if (configured === null) return loadFeePayer();
    const file = configured.startsWith("~/") ? resolve(homedir(), configured.slice(2)) : resolve(configured);
    return signerFromSolanaKeypairBytes(parseSolanaKeypairSource(readFileSync(file, "utf8"), file));
}
