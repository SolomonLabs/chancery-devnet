import { readFileSync } from "node:fs";

import { resolveWorkspacePath } from "../../../config/network.js";
import { replaceSourceAtomically } from "../../core/AtomicSource.js";
import { replaceNetworkConfigIssuedMintIdentity } from "../../core/NetworkIdentity.mjs";

export function recordIssuedMintIdentity(mint: string): void {
    const path = resolveWorkspacePath("config/networks/devnet.ts");
    const source = readFileSync(path, "utf8");
    const updated = replaceNetworkConfigIssuedMintIdentity(source, mint, path);
    if (updated !== source) replaceSourceAtomically(path, updated);
}
