import { readFile } from "node:fs/promises";
import { resolve } from "node:path";

import { sha256Bytes } from "../core/FileSystem.mjs";
import { requireCondition, requiredRecord, requiredSha256, requiredString } from "../core/Validation.mjs";

export const issuedTokenLaunchConfigPath = "artifacts/issued-token-launch.json";
export const expectedIssuedTokenProgram = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";
export const expectedIssuedTokenDecimals = 6;

export function validateIssuedTokenLaunchConfig(config) {
    requiredRecord(config, "issued-token launch config");
    requireCondition(config.schema_version === 1, "unsupported issued-token launch config schema_version");
    requireCondition(
        config.token_program === expectedIssuedTokenProgram,
        "issued-token launch token_program must be Token-2022"
    );
    requireCondition(config.decimals === expectedIssuedTokenDecimals, "issued-token launch decimals must be 6");

    const metadata = requiredRecord(config.metadata, "issued-token metadata");
    requireCondition(requiredString(metadata.name, "metadata.name") === "USDv", "metadata.name must be USDv");
    requireCondition(requiredString(metadata.symbol, "metadata.symbol") === "USDv", "metadata.symbol must be USDv");
    const uri = requiredString(metadata.uri, "metadata.uri");
    requireCondition(
        uri === "https://soletechltd.org/assets/usdv.json",
        "metadata.uri must be the approved USDv metadata URI"
    );
    requireCondition(
        !/(?:example\.|localhost|127\.0\.0\.1|<|>)/i.test(uri),
        "metadata.uri contains a placeholder or local address"
    );
    requireCondition(/^https:\/\//.test(uri), "metadata.uri must use HTTPS");
    requiredSha256(metadata.offchain_json_sha256, "metadata.offchain_json_sha256");

    const posture = requiredRecord(config.authority_posture, "authority_posture");
    requireCondition(
        posture.mode === "chancery_pda_reserved",
        "authority_posture.mode must reserve authorities to Chancery PDAs"
    );
    requireCondition(
        posture.metadata_updates_require_chancery_upgrade === true,
        "metadata update lock-in must be explicit"
    );
    requireCondition(
        posture.extension_authorities_reserved_for_future_use === true,
        "extension authority reservation must be explicit"
    );

    const requiredExtensions = [
        "TransferHook",
        "ConfidentialTransfer",
        "PermanentDelegate",
        "Pausable",
        "MetadataPointer",
        "MintCloseAuthority",
        "DefaultAccountState",
        "TokenMetadata"
    ];
    requireCondition(Array.isArray(config.reserved_extensions), "reserved_extensions must be an array");
    for (const extension of config.reserved_extensions) {
        requiredString(extension, "reserved_extensions entry");
    }
    for (const extension of requiredExtensions) {
        requireCondition(config.reserved_extensions.includes(extension), `reserved_extensions missing ${extension}`);
    }
    return config;
}

export async function readIssuedTokenLaunchConfig(root = resolve(import.meta.dirname, "..", "..")) {
    const path = resolve(root, issuedTokenLaunchConfigPath);
    const source = await readFile(path);
    const config = validateIssuedTokenLaunchConfig(JSON.parse(source.toString("utf8")));
    return { path, source, config, configSha256: sha256Bytes(source) };
}

export const issuedTokenMetadataTimeoutMs = 15_000;
export const issuedTokenMetadataMaximumBytes = 256 * 1024;

async function readBoundedResponseBody(response, maximumBytes) {
    const contentLengthHeader = response.headers?.get?.("content-length");
    if (contentLengthHeader !== null && contentLengthHeader !== undefined) {
        requireCondition(/^\d+$/.test(contentLengthHeader), "issued-token metadata Content-Length is invalid");
        requireCondition(
            BigInt(contentLengthHeader) <= BigInt(maximumBytes),
            `issued-token metadata exceeds ${maximumBytes} bytes`
        );
    }

    if (response.body?.getReader === undefined) {
        const bytes = new Uint8Array(await response.arrayBuffer());
        requireCondition(bytes.length <= maximumBytes, `issued-token metadata exceeds ${maximumBytes} bytes`);
        return bytes;
    }

    const reader = response.body.getReader();
    const chunks = [];
    let total = 0;
    try {
        while (true) {
            const { done, value } = await reader.read();
            if (done) break;
            requireCondition(value instanceof Uint8Array, "issued-token metadata response returned a non-byte chunk");
            total += value.length;
            requireCondition(total <= maximumBytes, `issued-token metadata exceeds ${maximumBytes} bytes`);
            chunks.push(value);
        }
    } finally {
        reader.releaseLock?.();
    }
    const bytes = new Uint8Array(total);
    let offset = 0;
    for (const chunk of chunks) {
        bytes.set(chunk, offset);
        offset += chunk.length;
    }
    return bytes;
}

export async function verifyIssuedTokenOffchainMetadata(config, fetchImplementation = globalThis.fetch) {
    requireCondition(typeof fetchImplementation === "function", "fetch is unavailable for metadata verification");
    const controller = new AbortController();
    const timeout = setTimeout(
        () => controller.abort(new Error("issued-token metadata request timed out")),
        issuedTokenMetadataTimeoutMs
    );
    timeout.unref?.();
    try {
        const response = await fetchImplementation(config.metadata.uri, {
            headers: { accept: "application/json" },
            redirect: "error",
            signal: controller.signal
        });
        requireCondition(response.ok, `issued-token metadata fetch failed with HTTP ${response.status}`);
        const contentType = response.headers?.get?.("content-type") ?? "";
        requireCondition(
            /^(?:application\/(?:[a-z0-9!#$&^_.+-]+\+)?json)(?:\s*;|$)/i.test(contentType),
            `issued-token metadata Content-Type must be JSON, got ${contentType || "<missing>"}`
        );
        const bytes = await readBoundedResponseBody(response, issuedTokenMetadataMaximumBytes);
        const observedHash = sha256Bytes(bytes);
        requireCondition(
            observedHash === config.metadata.offchain_json_sha256,
            `issued-token metadata SHA-256 mismatch: expected ${config.metadata.offchain_json_sha256}, got ${observedHash}`
        );
        const document = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes));
        requireCondition(
            document !== null && typeof document === "object" && !Array.isArray(document),
            "off-chain metadata must be a JSON object"
        );
        requireCondition(
            document.name === config.metadata.name,
            "off-chain metadata name does not match launch config"
        );
        requireCondition(
            document.symbol === config.metadata.symbol,
            "off-chain metadata symbol does not match launch config"
        );
        return { observedHash, document };
    } finally {
        clearTimeout(timeout);
    }
}
