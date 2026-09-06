import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

import { redactRpcUrl } from "./safeOutput.js";
import { DEVNET_GENESIS_HASH } from "../../../config/devnetGenesis.js";

const GENESIS_HASHES = {
    devnet: DEVNET_GENESIS_HASH,
    "mainnet-beta": "5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d",
} as const;

const LOCAL_GENESIS_PIN_PATH = ".local-fixtures/genesis-pin.json";

for (const [cluster, hash] of Object.entries(GENESIS_HASHES)) {
    if (hash.length < 43 || hash.length > 44) {
        throw new Error(`${cluster} genesis pin must be a full base58 32-byte hash, got ${hash.length} characters`);
    }
}

/// Localnet has no public genesis to hard-code: the first start of a local
/// ledger records its genesis hash (scripts/local/start-validator.sh), and
/// every later start and every `local` ceremony preflight verifies against
/// that recorded pin. Same identity-vs-cluster separation as staging: the pin
/// is the cluster check; it never substitutes for an identity check.
function expectedGenesisHash(expectedCluster: "devnet" | "mainnet-beta" | "localnet"): string {
    if (expectedCluster !== "localnet") return GENESIS_HASHES[expectedCluster];
    const pinPath = resolve(LOCAL_GENESIS_PIN_PATH);
    if (!existsSync(pinPath)) {
        throw new Error(
            `local genesis pin is missing: ${pinPath}; start the rig once with scripts/local/start-validator.sh to record it`,
        );
    }
    const pin = JSON.parse(readFileSync(pinPath, "utf8")) as { genesis_hash?: unknown };
    if (typeof pin.genesis_hash !== "string" || pin.genesis_hash.trim().length === 0) {
        throw new Error(`${pinPath} does not contain a recorded genesis_hash`);
    }
    return pin.genesis_hash;
}

interface JsonRpcResponse<T> {
    jsonrpc?: string;
    id?: number;
    result?: T;
    error?: { code?: number; message?: string };
}

async function rpc<T>(rpcUrl: string, method: string, params: unknown[] = []): Promise<T> {
    let response: Response;
    try {
        response = await fetch(rpcUrl, {
            method: "POST",
            headers: { "content-type": "application/json" },
            body: JSON.stringify({ jsonrpc: "2.0", id: 1, method, params }),
        });
    } catch (error) {
        throw new Error(`RPC ${method} request failed at ${redactRpcUrl(rpcUrl)}`, { cause: error });
    }
    if (!response.ok) throw new Error(`RPC ${method} returned HTTP ${response.status} at ${redactRpcUrl(rpcUrl)}`);
    const payload = await response.json() as JsonRpcResponse<T>;
    if (payload.error !== undefined) {
        throw new Error(`RPC ${method} failed (${payload.error.code ?? "unknown"}): ${payload.error.message ?? "unknown error"}`);
    }
    if (!("result" in payload)) throw new Error(`RPC ${method} returned no result`);
    return payload.result as T;
}

export interface RpcClusterEvidence {
    genesisHash: string;
    finalizedSlot: bigint;
    finalizedBlockTime: number;
    blockAgeSeconds: number;
    health: string;
}

export async function verifyRpcCluster(
    rpcUrl: string,
    expectedCluster: "devnet" | "mainnet-beta" | "localnet",
    maxFinalizedBlockAgeSeconds = 900,
): Promise<RpcClusterEvidence> {
    if (rpcUrl.trim().length === 0) throw new Error("RPC URL is unset");
    const [genesisHash, health, slotValue] = await Promise.all([
        rpc<string>(rpcUrl, "getGenesisHash"),
        rpc<string>(rpcUrl, "getHealth"),
        rpc<number>(rpcUrl, "getSlot", [{ commitment: "finalized" }]),
    ]);
    const expectedGenesis = expectedGenesisHash(expectedCluster);
    if (genesisHash !== expectedGenesis) {
        throw new Error(`RPC cluster mismatch: expected ${expectedCluster} genesis ${expectedGenesis}, got ${genesisHash}`);
    }
    if (health !== "ok") throw new Error(`RPC health check failed: ${health}`);
    if (!Number.isSafeInteger(slotValue) || slotValue <= 0) throw new Error(`RPC returned invalid finalized slot ${String(slotValue)}`);
    const blockTime = await rpc<number | null>(rpcUrl, "getBlockTime", [slotValue]);
    if (blockTime === null || !Number.isSafeInteger(blockTime)) throw new Error("RPC returned no finalized block time");
    const age = Math.max(0, Math.floor(Date.now() / 1000) - blockTime);
    if (age > maxFinalizedBlockAgeSeconds) {
        throw new Error(`RPC finalized block is stale by ${age}s (maximum ${maxFinalizedBlockAgeSeconds}s)`);
    }
    return {
        genesisHash,
        finalizedSlot: BigInt(slotValue),
        finalizedBlockTime: blockTime,
        blockAgeSeconds: age,
        health,
    };
}

export async function rpcBalance(rpcUrl: string, address: string): Promise<bigint> {
    const result = await rpc<{ value: number }>(rpcUrl, "getBalance", [address, { commitment: "finalized" }]);
    if (!Number.isSafeInteger(result.value) || result.value < 0) throw new Error(`invalid balance for ${address}`);
    return BigInt(result.value);
}

export async function rpcAccountExists(rpcUrl: string, address: string): Promise<boolean> {
    const result = await rpc<{ value: unknown | null }>(rpcUrl, "getAccountInfo", [
        address,
        { commitment: "finalized", encoding: "base64" },
    ]);
    return result.value !== null;
}
