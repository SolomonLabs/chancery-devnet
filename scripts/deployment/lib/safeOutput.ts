/** Utilities for logging commands without exposing RPC credentials or signer paths. */

const URL_FLAG_NAMES = new Set(["--url", "--rpc"]);
const PATH_FLAG_NAMES = new Set([
    "--keypair",
    "--payer",
    "--program-id",
    "--upgrade-authority",
    "--program-keypair",
    "--mint-keypair",
]);

export function redactRpcUrl(value: string): string {
    const trimmed = value.trim();
    if (trimmed.length === 0) return "<unset>";
    try {
        const url = new URL(trimmed);
        const hasSensitivePath = url.pathname !== "" && url.pathname !== "/";
        const hasSensitiveQuery = url.search.length > 0;
        const hasCredentials = url.username.length > 0 || url.password.length > 0;
        const suffix = hasSensitivePath || hasSensitiveQuery || hasCredentials ? "/<redacted>" : "";
        return `${url.protocol}//${url.host}${suffix}`;
    } catch {
        return "<redacted-rpc-url>";
    }
}

export function redactPath(value: string): string {
    const normalized = value.replaceAll("\\", "/");
    const parts = normalized.split("/").filter((part) => part.length > 0);
    return parts.length === 0 ? "<unset>" : `<local>/${parts.at(-1)}`;
}

export function redactedCommand(
    command: string,
    args: readonly string[],
    positionalPathIndices: readonly number[] = [],
): string {
    const positionalPaths = new Set(positionalPathIndices);
    const output: string[] = [command];
    for (let index = 0; index < args.length; index++) {
        const value = args[index]!;
        if (positionalPaths.has(index)) {
            output.push(redactPath(value));
            continue;
        }
        output.push(value);
        if (URL_FLAG_NAMES.has(value) && index + 1 < args.length) {
            output.push(redactRpcUrl(args[++index]!));
            continue;
        }
        if (PATH_FLAG_NAMES.has(value) && index + 1 < args.length) {
            output.push(redactPath(args[++index]!));
            continue;
        }
        if (/^https?:\/\//i.test(value)) output[output.length - 1] = redactRpcUrl(value);
    }
    return output.join(" ");
}

export function redactCommandOutput(
    value: string,
    args: readonly string[] = [],
    positionalPathIndices: readonly number[] = [],
): string {
    let output = value;
    for (let index = 0; index < args.length - 1; index++) {
        const flag = args[index]!;
        const secret = args[index + 1]!;
        if (URL_FLAG_NAMES.has(flag)) output = output.replaceAll(secret, redactRpcUrl(secret));
        if (PATH_FLAG_NAMES.has(flag)) output = output.replaceAll(secret, redactPath(secret));
    }
    for (const index of positionalPathIndices) {
        const secret = args[index];
        if (typeof secret === "string" && secret.length > 0) {
            output = output.replaceAll(secret, redactPath(secret));
        }
    }
    return output.replace(/https?:\/\/[^\s'"`]+/gi, (match) => redactRpcUrl(match));
}
