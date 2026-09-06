import { assertCanonicalRepositoryPath } from "./FileSystem.mjs";
import { containsControlCharacter } from "./StrictJson.mjs";

export const SHA256_PATTERN = /^[0-9a-f]{64}$/u;
export const GIT_REVISION_PATTERN = /^(?:[0-9a-f]{40}|[0-9a-f]{64})$/u;

export function requireCondition(condition, message) {
    if (!condition) throw new Error(message);
}

export function requiredRecord(value, sourceName) {
    requireCondition(
        value !== null && typeof value === "object" && !Array.isArray(value),
        `${sourceName} must be an object`
    );
    return value;
}

export function requiredString(value, sourceName, maximumLength = 4_096) {
    requireCondition(
        typeof value === "string" &&
            value.length > 0 &&
            value.length <= maximumLength &&
            value.trim() === value &&
            !containsControlCharacter(value),
        `${sourceName} must be non-empty bounded text without surrounding whitespace or control characters`
    );
    return value;
}

export function requiredCanonicalTimestamp(value, sourceName) {
    requiredString(value, sourceName, 64);
    const parsed = new Date(value);
    requireCondition(
        !Number.isNaN(parsed.valueOf()) && parsed.toISOString() === value,
        `${sourceName} must be a canonical ISO-8601 UTC timestamp`
    );
    return value;
}

export function requireTimestampNotFuture(value, sourceName, maximumFutureSkewMilliseconds = 5 * 60 * 1_000) {
    const timestamp = requiredCanonicalTimestamp(value, sourceName);
    requireCondition(
        Date.parse(timestamp) <= Date.now() + maximumFutureSkewMilliseconds,
        `${sourceName} is unreasonably in the future`
    );
    return timestamp;
}

export function requiredRevision(value, sourceName) {
    requireCondition(
        typeof value === "string" && GIT_REVISION_PATTERN.test(value),
        `${sourceName} must be an exact Git SHA-1 or SHA-256 revision`
    );
    return value;
}

export function requiredNonNegativeInteger(value, sourceName) {
    requireCondition(Number.isSafeInteger(value) && value >= 0, `${sourceName} must be a non-negative safe integer`);
    return value;
}

export function requiredSha256(value, sourceName) {
    requireCondition(typeof value === "string" && SHA256_PATTERN.test(value), `${sourceName} must be SHA-256 hex`);
    return value;
}

export function optionalSha256(value, sourceName) {
    if (value === null) return null;
    return requiredSha256(value, sourceName);
}

export function optionalRevision(value, sourceName) {
    if (value === null) return null;
    requireCondition(
        typeof value === "string" && GIT_REVISION_PATTERN.test(value),
        `${sourceName} must be an exact Git SHA-1 or SHA-256 revision`
    );
    return value;
}

export function requiredHttpsUrl(value, sourceName) {
    requiredString(value, sourceName, 2_048);
    let parsed;
    try {
        parsed = new URL(value);
    } catch (error) {
        throw new Error(`${sourceName} must be a valid URL`, { cause: error });
    }
    requireCondition(parsed.protocol === "https:", `${sourceName} must use https`);
    requireCondition(
        parsed.username.length === 0 && parsed.password.length === 0,
        `${sourceName} must not contain credentials`
    );
    return value;
}

export function optionalHttpsUrl(value, sourceName) {
    if (value === null) return null;
    return requiredHttpsUrl(value, sourceName);
}

export function requiredRepositoryPath(value, sourceName) {
    requiredString(value, sourceName, 1_024);
    return assertCanonicalRepositoryPath(value, sourceName);
}
