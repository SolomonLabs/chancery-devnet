import type { PermissionGenerationState } from "./types.js";

export function effectivePermissionGeneration(record: PermissionGenerationState, roleSchemaVersion: number): bigint {
    if (record.permissionGeneration < 0n || record.permissionGeneration >= (1n << 64n)) {
        throw new Error("permission generation is outside the u64 range");
    }
    if (record.permissionGeneration === 0n && record.roleMask === 0n && record.permissionFlags === 1n
        && record.expiryUnixTimestamp === 0n && record.roleSchemaVersion === roleSchemaVersion) return 1n;
    return record.permissionGeneration;
}

export function nextPermissionGeneration(current: bigint): bigint {
    if (current < 0n || current >= (1n << 64n) - 1n) {
        throw new Error("permission generation cannot advance within the u64 range");
    }
    return current + 1n;
}
