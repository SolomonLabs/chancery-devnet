import type { DeploymentArguments } from "./types.js";

export function parseDeploymentArguments(arguments_: readonly string[]): DeploymentArguments {
    const runtimeArguments: string[] = [];
    let verificationLamports = 100_000_000n;
    const seen = new Set<string>();
    for (let index = 0; index < arguments_.length; index++) {
        const flag = arguments_[index];
        if (index === 0 && flag === "devnet") continue;
        if (flag !== "--config" && flag !== "--verification-lamports") {
            throw new Error("unknown deployment argument: " + flag);
        }
        if (seen.has(flag)) throw new Error("repeated deployment argument: " + flag);
        seen.add(flag);
        const value = arguments_[++index];
        if (value === undefined || value.startsWith("--") || value.trim().length === 0) {
            throw new Error(flag + " requires a value");
        }
        if (flag === "--config") {
            runtimeArguments.push(flag, value);
        } else {
            if (!/^[1-9][0-9]*$/u.test(value)) throw new Error("--verification-lamports must be a positive integer");
            verificationLamports = BigInt(value);
            if (verificationLamports > (1n << 64n) - 1n) throw new Error("--verification-lamports exceeds u64");
        }
    }
    return { runtimeArguments, verificationLamports };
}
