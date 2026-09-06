import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import { Base58Util } from "@solomon-labs/base58";
import { PublicKey } from "@solomon-labs/publickey";
import type { Base58 } from "@solomon-labs/types";

import { assertExactObjectKeys, parseStrictJson } from "../../../config/strictJson.js";
import { TEST_ASSET_PROFILES, type TestAssetSymbol } from "./testAssetProfiles.js";

export interface TestAssetAllocation {
    readonly symbol: TestAssetSymbol;
    readonly recipient_owner: string;
    readonly minimum_balance_base_units: string;
}

export interface TestAssetMintPlan {
    readonly schema_version: 1;
    readonly network: "devnet";
    readonly allocations: readonly TestAssetAllocation[];
}

export interface LoadedTestAssetMintPlan {
    readonly path: string;
    readonly plan: TestAssetMintPlan;
}

function requirePublicKey(value: unknown, field: string): string {
    if (typeof value !== "string" || value.trim().length === 0) {
        throw new Error(`${field} must be a non-empty public key`);
    }
    const publicKey = new PublicKey(Base58Util.decode(value as Base58));
    if (publicKey.toBase58() !== value) throw new Error(`${field} must be canonical base58 public-key text`);
    return value;
}

function requireBalance(value: unknown, field: string): string {
    if (typeof value !== "string" || !/^[1-9][0-9]*$/u.test(value)) {
        throw new Error(`${field} must be a positive base-unit integer string`);
    }
    return value;
}

function parseAllocation(value: unknown, field: string): TestAssetAllocation {
    assertExactObjectKeys(value, field, [
        "symbol",
        "recipient_owner",
        "minimum_balance_base_units",
    ]);
    const symbol = TEST_ASSET_PROFILES.find((profile) => profile.symbol === value.symbol)?.symbol;
    if (symbol === undefined) {
        throw new Error(
            `${field}.symbol must be one of ${TEST_ASSET_PROFILES.map((profile) => profile.symbol).join(", ")}`,
        );
    }
    return {
        symbol,
        recipient_owner: requirePublicKey(value.recipient_owner, `${field}.recipient_owner`),
        minimum_balance_base_units: requireBalance(
            value.minimum_balance_base_units,
            `${field}.minimum_balance_base_units`,
        ),
    };
}

/**
 * Unlike the staging plan this is derived from, a devnet plan is not required
 * to allocate every configured symbol. Topping up a single asset for a single
 * wallet is the common case here.
 */
export function parseTestAssetMintPlan(value: unknown): TestAssetMintPlan {
    assertExactObjectKeys(value, "test asset mint plan", [
        "schema_version",
        "network",
        "allocations",
    ]);
    if (value.schema_version !== 1) throw new Error("test asset mint plan schema_version must be 1");
    if (value.network !== "devnet") throw new Error("test asset mint plan network must be devnet");
    if (!Array.isArray(value.allocations) || value.allocations.length === 0) {
        throw new Error("test asset mint plan allocations must be a non-empty array");
    }
    const allocations: TestAssetAllocation[] = [];
    const allocationKeys = new Set<string>();
    for (let index = 0, length = value.allocations.length; index < length; index += 1) {
        const allocation = parseAllocation(value.allocations[index], `allocations[${index}]`);
        const key = `${allocation.symbol}:${allocation.recipient_owner}`;
        if (allocationKeys.has(key)) throw new Error(`duplicate test asset allocation ${key}`);
        allocationKeys.add(key);
        allocations.push(allocation);
    }
    return {
        schema_version: 1,
        network: "devnet",
        allocations,
    };
}

export function loadTestAssetMintPlan(path: string): LoadedTestAssetMintPlan {
    const absolutePath = resolve(path);
    const source = readFileSync(absolutePath, "utf8");
    const plan = parseTestAssetMintPlan(parseStrictJson(source, absolutePath, {
        maxBytes: 1024 * 1024,
        maxDepth: 12,
    }));
    return { path: absolutePath, plan };
}
