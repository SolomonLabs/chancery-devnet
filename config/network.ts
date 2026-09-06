import { createHash } from "node:crypto";
import { existsSync, readFileSync, realpathSync } from "node:fs";
import { dirname, isAbsolute, relative, resolve } from "node:path";

import { devnetNetworkIdentity } from "./networks/devnet.js";
import { assertExactObjectKeys, canonicalJson, parseStrictJson } from "./strictJson.js";

export type ChanceryNetworkName = "devnet";
export type ChanceryCluster = "devnet";
export type ChanceryCollateralSymbol = "USDC" | "USDT" | "USDG" | "PYUSD";

export interface ChanceryCollateralMint {
    readonly symbol: ChanceryCollateralSymbol;
    readonly mint: string;
    readonly tokenProgram: "spl-token" | "token-2022";
    readonly transferHook: "absent" | "dormant-only";
}

export interface ChanceryNetworkIdentity {
    readonly name: ChanceryNetworkName;
    readonly cluster: ChanceryCluster;
    readonly programId: string;
    readonly faucetProgramId: string;
    readonly usdvMint: string | null;
    readonly usdcMint: string | null;
    readonly legacyUsdvMint: string | null;
    readonly collateralMints?: readonly ChanceryCollateralMint[];
}

export interface ChanceryDevnetAirdropConfig {
    readonly enabled: boolean;
    readonly required: boolean;
    readonly lamports: bigint;
}

export interface ChanceryNetwork extends ChanceryNetworkIdentity {
    readonly runtimeConfigPath: string;
    readonly runtimeConfigSha256: string;
    readonly rpcUrl: string;
    readonly rpcProviderId: string;
    readonly programKeypairPath: string;
    readonly upgradeAuthorityKeypairPath: string | null;
    readonly feePayerKeypairPath: string;
    readonly setupAuthorityKeypairPath: string;
    readonly issuedMintKeypairPath: string | null;
    readonly chanceryManifestPath: string;
    readonly issuedMintManifestPath: string;
    readonly legacyUsdvMintManifestPath: string | null;
    readonly maxExtensionObservationAgeSlots: bigint;
    readonly devnetAirdrop: ChanceryDevnetAirdropConfig;
}

interface RuntimeConfigFile {
    readonly schema_version: 2;
    readonly network: ChanceryNetworkName;
    readonly rpc_url: string;
    readonly rpc_provider_id: string;
    readonly program_keypair_path: string;
    readonly upgrade_authority_keypair_path: string | null;
    readonly fee_payer_keypair_path: string;
    readonly setup_authority_keypair_path: string;
    readonly issued_mint_keypair_path: string | null;
    readonly chancery_manifest_path: string;
    readonly issued_mint_manifest_path: string;
    readonly legacy_usdv_mint_manifest_path: string | null;
    readonly max_extension_observation_age_slots: string;
    readonly devnet_airdrop: {
        readonly enabled: boolean;
        readonly required: boolean;
        readonly lamports: string;
    };
}

const WORKSPACE_ROOT = resolve(import.meta.dirname, "..");

const RUNTIME_KEYS = [
    "schema_version",
    "network",
    "rpc_url",
    "rpc_provider_id",
    "program_keypair_path",
    "upgrade_authority_keypair_path",
    "fee_payer_keypair_path",
    "setup_authority_keypair_path",
    "issued_mint_keypair_path",
    "chancery_manifest_path",
    "issued_mint_manifest_path",
    "legacy_usdv_mint_manifest_path",
    "max_extension_observation_age_slots",
    "devnet_airdrop",
] as const;

function requireString(value: unknown, field: string): string {
    if (typeof value !== "string" || value.trim().length === 0) {
        throw new Error(`${field} must be a non-empty string`);
    }
    return value;
}

function requireRpcProviderId(value: unknown, field: string): string {
    const providerId = requireString(value, field);
    if (providerId !== providerId.trim() || !/^[A-Za-z0-9][A-Za-z0-9._:-]{0,127}$/u.test(providerId)) {
        throw new Error(`${field} must be a stable provider/operator identifier`);
    }
    return providerId;
}

function assertRpcIdentity(value: string, field: string): void {
    let parsed: URL;
    try {
        parsed = new URL(value);
    } catch (error) {
        throw new Error(`${field} must be an absolute RPC URL`, { cause: error });
    }
    const hostname = parsed.hostname.toLowerCase();
    const loopbackHostname = hostname === "localhost" || hostname === "127.0.0.1" ||
        hostname === "::1" || hostname === "[::1]";
    if (parsed.protocol !== "https:" && !(parsed.protocol === "http:" && loopbackHostname)) {
        throw new Error(`${field} must use HTTPS; plaintext HTTP is permitted only for loopback testing`);
    }
}

function optionalPath(value: unknown, field: string): string | null {
    if (value === null) return null;
    return resolveWorkspacePath(requireString(value, field));
}

export function resolveWorkspacePath(path: string): string {
    const absolutePath = resolve(WORKSPACE_ROOT, path);
    const relativePath = relative(WORKSPACE_ROOT, absolutePath);
    if (relativePath.startsWith("..") || isAbsolute(relativePath)) {
        throw new Error(`configured path escapes the workspace: ${path}`);
    }
    let existingAncestor = absolutePath;
    while (!existsSync(existingAncestor)) {
        const parent = dirname(existingAncestor);
        if (parent === existingAncestor) throw new Error(`configured path has no existing ancestor: ${path}`);
        existingAncestor = parent;
    }
    const realAncestor = realpathSync(existingAncestor);
    const realRelativePath = relative(realpathSync(WORKSPACE_ROOT), realAncestor);
    if (realRelativePath.startsWith("..") || isAbsolute(realRelativePath)) {
        throw new Error(`configured path resolves outside the workspace: ${path}`);
    }
    if (realAncestor !== existingAncestor) {
        throw new Error(`configured path must not pass through a symlink: ${path}`);
    }
    return absolutePath;
}

function requireUnsignedBigInt(value: unknown, field: string): bigint {
    if (typeof value !== "string" || !/^\d+$/.test(value)) {
        throw new Error(`${field} must be an unsigned integer string`);
    }
    return BigInt(value);
}

function runtimeConfigPathFromArguments(args: readonly string[]): string {
    const configIndex = args.indexOf("--config");
    if (configIndex !== -1) {
        const value = args[configIndex + 1];
        if (value === undefined || value.startsWith("--")) {
            throw new Error("--config requires a file path");
        }
        return resolveWorkspacePath(value);
    }
    return resolveWorkspacePath("config/runtime/devnet.json");
}

function parseRuntimeConfig(path: string, source: string): RuntimeConfigFile {
    const parsed = parseStrictJson(source, path);
    assertExactObjectKeys(parsed, path, RUNTIME_KEYS);
    if (parsed.schema_version !== 2) throw new Error(`${path}: schema_version must be 2`);
    if (parsed.network !== "devnet") throw new Error(`${path}: network must be devnet`);
    assertRpcIdentity(requireString(parsed.rpc_url, `${path}.rpc_url`), `${path}.rpc_url`);
    requireRpcProviderId(parsed.rpc_provider_id, `${path}.rpc_provider_id`);
    for (const field of [
        "program_keypair_path",
        "fee_payer_keypair_path",
        "setup_authority_keypair_path",
        "chancery_manifest_path",
        "issued_mint_manifest_path",
    ] as const) {
        requireString(parsed[field], `${path}.${field}`);
    }
    for (const field of [
        "upgrade_authority_keypair_path",
        "issued_mint_keypair_path",
        "legacy_usdv_mint_manifest_path",
    ] as const) {
        if (parsed[field] !== null) requireString(parsed[field], `${path}.${field}`);
    }
    requireUnsignedBigInt(
        parsed.max_extension_observation_age_slots,
        `${path}.max_extension_observation_age_slots`,
    );
    assertExactObjectKeys(parsed.devnet_airdrop, `${path}.devnet_airdrop`, ["enabled", "required", "lamports"]);
    if (typeof parsed.devnet_airdrop.enabled !== "boolean" || typeof parsed.devnet_airdrop.required !== "boolean") {
        throw new Error(`${path}.devnet_airdrop enabled/required must be boolean`);
    }
    requireUnsignedBigInt(parsed.devnet_airdrop.lamports, `${path}.devnet_airdrop.lamports`);
    return parsed as unknown as RuntimeConfigFile;
}

export function resolveNetwork(
    value?: string,
    args: readonly string[] = process.argv.slice(2),
): ChanceryNetwork {
    const selectedNetwork = value ?? "devnet";
    if (selectedNetwork !== "devnet") {
        throw new Error(`this workspace deploys devnet only; got ${selectedNetwork}`);
    }
    const runtimeConfigPath = runtimeConfigPathFromArguments(args);
    if (!existsSync(runtimeConfigPath)) {
        throw new Error(`runtime config missing: ${runtimeConfigPath}; restore config/runtime/devnet.json`);
    }
    const source = readFileSync(runtimeConfigPath, "utf8");
    const runtime = parseRuntimeConfig(runtimeConfigPath, source);
    return Object.freeze({
        ...devnetNetworkIdentity,
        runtimeConfigPath,
        runtimeConfigSha256: createHash("sha256").update(canonicalJson(runtime)).digest("hex"),
        rpcUrl: runtime.rpc_url,
        rpcProviderId: runtime.rpc_provider_id,
        programKeypairPath: resolveWorkspacePath(runtime.program_keypair_path),
        upgradeAuthorityKeypairPath: optionalPath(runtime.upgrade_authority_keypair_path, "upgrade_authority_keypair_path"),
        feePayerKeypairPath: resolveWorkspacePath(runtime.fee_payer_keypair_path),
        setupAuthorityKeypairPath: resolveWorkspacePath(runtime.setup_authority_keypair_path),
        issuedMintKeypairPath: optionalPath(runtime.issued_mint_keypair_path, "issued_mint_keypair_path"),
        chanceryManifestPath: resolveWorkspacePath(runtime.chancery_manifest_path),
        issuedMintManifestPath: resolveWorkspacePath(runtime.issued_mint_manifest_path),
        legacyUsdvMintManifestPath: optionalPath(runtime.legacy_usdv_mint_manifest_path, "legacy_usdv_mint_manifest_path"),
        maxExtensionObservationAgeSlots: BigInt(runtime.max_extension_observation_age_slots),
        devnetAirdrop: Object.freeze({
            enabled: runtime.devnet_airdrop.enabled,
            required: runtime.devnet_airdrop.required,
            lamports: BigInt(runtime.devnet_airdrop.lamports),
        }),
    });
}

export function workspaceRoot(): string {
    return WORKSPACE_ROOT;
}
