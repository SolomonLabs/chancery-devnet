import { homedir } from "node:os";
import { resolve } from "node:path";

import { workspaceRoot } from "../../../config/network.js";

export function testerKeypairPath(configured: string | null): string {
    if (configured === null) return resolve(workspaceRoot(), ".devnet", "tester", "keypair.json");
    if (configured.startsWith("~/")) return resolve(homedir(), configured.slice(2));
    return resolve(configured);
}
