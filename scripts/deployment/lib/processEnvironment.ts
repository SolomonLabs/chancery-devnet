import { readFileSync } from "node:fs";

const PLATFORM_ENVIRONMENT_KEYS = [
    "PATH",
    "SYSTEMROOT",
    "WINDIR",
    "COMSPEC",
    "PATHEXT",
    "TMP",
    "TEMP",
    "TMPDIR",
    "HOME",
    "USERPROFILE",
    "APPDATA",
    "LOCALAPPDATA",
    "SSL_CERT_FILE",
    "SSL_CERT_DIR",
] as const;

interface PackageJsonShape {
    engines?: {
        node?: unknown;
    };
}

const absoluteNodeVersionPattern = /^(?:0|[1-9]\d*)\.(?:0|[1-9]\d*)\.(?:0|[1-9]\d*)$/;
const nodeSourceShaPattern = /^(?:[0-9a-f]{40}|[0-9a-f]{64})$/i;

export function assertSupportedNodeVersion(): void {
    const packageJson = JSON.parse(
        readFileSync(new URL("../../../package.json", import.meta.url), "utf8")
    ) as PackageJsonShape;

    const declaredValue = packageJson.engines?.node;
    const declared = typeof declaredValue === "string" ? declaredValue.trim() : "";

    if (absoluteNodeVersionPattern.test(declared)) {
        if (process.versions.node !== declared) {
            throw new Error(
                `Node ${process.versions.node} is unsupported: package.json engines.node pins ${declared}; ` +
                    "install the .nvmrc toolchain before running release or deployment commands"
            );
        }

        return;
    }

    const minimumVersion = /^>=(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$/u.exec(declared);
    if (minimumVersion !== null) {
        const required = minimumVersion.slice(1).map(Number);
        const actual = process.versions.node.split(".").map(Number);
        for (let index = 0; index < 3; index++) {
            if (actual[index]! > required[index]!) return;
            if (actual[index]! < required[index]!) {
                throw new Error("Node " + process.versions.node + " is unsupported; package.json requires " + declared);
            }
        }
        return;
    }

    if (nodeSourceShaPattern.test(declared)) {
        const actualNodeSourceSha = process.env.NODE_SOURCE_SHA?.trim();

        if (actualNodeSourceSha === undefined || !nodeSourceShaPattern.test(actualNodeSourceSha)) {
            throw new Error(
                `package.json engines.node pins source SHA ${declared}, but NODE_SOURCE_SHA was not provided`
            );
        }

        if (actualNodeSourceSha.toLowerCase() !== declared.toLowerCase()) {
            throw new Error(
                `Node source SHA ${actualNodeSourceSha} is unsupported: ` +
                    `package.json engines.node pins ${declared}`
            );
        }

        return;
    }

    throw new Error(
        `package.json engines.node must be an exact version, a >= minimum version, or a full SHA-1/SHA-256 hash, got ${String(
            declaredValue
        )}`
    );
}

export function sanitizedProcessEnvironment(
    extra: Readonly<Record<string, string | undefined>> = {},
): NodeJS.ProcessEnv {
    const output: NodeJS.ProcessEnv = {
        TZ: "UTC",
        LANG: "C.UTF-8",
        LC_ALL: "C.UTF-8",
    };
    for (let index = 0, length = PLATFORM_ENVIRONMENT_KEYS.length; index < length; index += 1) {
        const key = PLATFORM_ENVIRONMENT_KEYS[index];
        const value = process.env[key];
        if (value !== undefined && value.length > 0) output[key] = value;
    }
    for (const [key, value] of Object.entries(extra)) {
        if (value !== undefined) output[key] = value;
    }
    return output;
}

export function isolatedToolProcessEnvironment(
    extra: Readonly<Record<string, string | undefined>> = {},
): NodeJS.ProcessEnv {
    const output: NodeJS.ProcessEnv = {
        TZ: "UTC",
        LANG: "C.UTF-8",
        LC_ALL: "C.UTF-8",
    };
    for (const key of ["PATH", "SYSTEMROOT", "WINDIR", "COMSPEC", "PATHEXT", "TMP", "TEMP", "TMPDIR", "SSL_CERT_FILE", "SSL_CERT_DIR"] as const) {
        const value = process.env[key];
        if (value !== undefined && value.length > 0) output[key] = value;
    }
    for (const [key, value] of Object.entries(extra)) {
        if (value !== undefined) output[key] = value;
    }
    return output;
}
