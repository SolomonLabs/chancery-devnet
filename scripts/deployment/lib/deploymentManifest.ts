import { existsSync, mkdirSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";

import type { ChanceryNetwork } from "../../../config/network.js";
import { redactRpcUrl } from "./safeOutput.js";

export type ChanceryDeploymentManifest = Record<string, unknown> & {
    schemaVersion: 1;
    network: string;
    cluster: string;
    programId: string;
    rpcEndpoint: string;
    phase: "deployed" | "initialized";
    updatedAt: string;
};

/**
 * Merge and atomically write the output-only deployment manifest. Network config
 * and on-chain RPC reads remain authoritative; this file is never trusted as input.
 */
export function updateChanceryDeploymentManifest(
    net: ChanceryNetwork,
    patch: Record<string, unknown>
): ChanceryDeploymentManifest {
    const path = resolve(net.chanceryManifestPath);
    let previous: Record<string, unknown> = {};
    if (existsSync(path)) {
        const parsed = JSON.parse(readFileSync(path, "utf8")) as Record<string, unknown>;
        if (parsed.programId === net.programId && parsed.network === net.name) previous = parsed;
    }

    // Remove legacy raw-RPC fields before merging so an older local manifest cannot
    // preserve a credential-bearing endpoint after this safe schema is written.
    delete previous.rpcUrl;
    delete previous.rpc_url;
    delete previous.rpc;

    const manifest = {
        ...previous,
        schemaVersion: 1 as const,
        network: net.name,
        cluster: net.cluster,
        programId: net.programId,
        rpcEndpoint: redactRpcUrl(net.rpcUrl),
        ...patch,
        updatedAt: new Date().toISOString()
    } as ChanceryDeploymentManifest;

    // Callers may only add evidence and state fields. Never allow a patch to
    // smuggle a credential-bearing endpoint back into the output manifest or
    // replace the canonical redacted endpoint.
    delete manifest.rpcUrl;
    delete manifest.rpc_url;
    delete manifest.rpc;
    manifest.rpcEndpoint = redactRpcUrl(net.rpcUrl);

    mkdirSync(dirname(path), { recursive: true });
    const temporaryPath = `${path}.tmp`;
    writeFileSync(temporaryPath, `${JSON.stringify(manifest, null, 2)}\n`);
    renameSync(temporaryPath, path);
    console.log(`  manifest           : ${path}`);
    return manifest;
}
