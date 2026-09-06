import type { TesterArguments, TesterCommand, TesterMode } from "./types.js";

const U64_MAXIMUM = (1n << 64n) - 1n;

function positiveInteger(value: string, flag: string): bigint {
    if (!/^[1-9][0-9]*$/u.test(value)) throw new Error(flag + " must be a positive integer in base units");
    const amount = BigInt(value);
    if (amount > U64_MAXIMUM) throw new Error(flag + " exceeds u64");
    return amount;
}

export function parseTesterArguments(arguments_: readonly string[], command: TesterCommand): TesterArguments {
    let keypairPath: string | null = null;
    const runtimeArguments: string[] = [];
    let reuseWallet = false;
    let mode: TesterMode = "all";
    let symbol: TesterArguments["symbol"] = "all";
    let amount: bigint | undefined;
    let minimumLamports = 100_000_000n;
    let allowAirdrop = true;
    const seen = new Set<string>();
    for (let index = 0; index < arguments_.length; index++) {
        const flag = arguments_[index];
        if (index === 0 && flag === "devnet") continue;
        const allowed = flag === "--keypair"
            || (command === "wallet" && flag === "--reuse")
            || (command !== "wallet" && (flag === "--config" || flag === "--lamports" || flag === "--skip-airdrop"))
            || (command === "integration" && (flag === "--mode" || flag === "--symbol" || flag === "--amount"));
        if (!allowed) throw new Error("unknown tester " + command + " argument: " + flag);
        if (seen.has(flag)) throw new Error("repeated tester argument: " + flag);
        seen.add(flag);
        if (flag === "--reuse") { reuseWallet = true; continue; }
        if (flag === "--skip-airdrop") { allowAirdrop = false; continue; }
        const value = arguments_[++index];
        if (value === undefined || value.startsWith("--") || value.trim().length === 0) {
            throw new Error(flag + " requires a value");
        }
        switch (flag) {
            case "--keypair": keypairPath = value; break;
            case "--config": runtimeArguments.push(flag, value); break;
            case "--mode":
                if (value !== "all" && value !== "setup" && value !== "test") throw new Error("--mode must be all, setup, or test");
                mode = value;
                break;
            case "--symbol":
                if (value !== "all" && value !== "USDC" && value !== "USDT" && value !== "USDG" && value !== "PYUSD") {
                    throw new Error("--symbol must be all, USDC, USDT, USDG, or PYUSD");
                }
                symbol = value;
                break;
            case "--amount": amount = positiveInteger(value, flag); break;
            case "--lamports":
                minimumLamports = positiveInteger(value, flag);
                if (minimumLamports > BigInt(Number.MAX_SAFE_INTEGER)) throw new Error("--lamports exceeds the exact RPC integer range");
                break;
        }
    }
    if (command === "integration" && mode !== "all" && (seen.has("--lamports") || seen.has("--skip-airdrop"))) {
        throw new Error("--lamports and --skip-airdrop apply to tester:all or tester:fund");
    }
    return { keypairPath, runtimeArguments, reuseWallet, mode, symbol, amount, minimumLamports, allowAirdrop };
}
