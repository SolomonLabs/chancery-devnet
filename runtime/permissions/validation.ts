import type { PermissionCapabilities } from "./types.js";

export function validatePermissionCapabilities(value: PermissionCapabilities): void {
    if (value.roleMask < 0n || value.roleMask >= (1n << 128n)) {
        throw new Error("permission role mask is outside the u128 range");
    }
    if (value.expiryUnixTimestamp < 0n || value.expiryUnixTimestamp >= (1n << 63n)) {
        throw new Error("permission expiry is outside the nonnegative i64 range");
    }
}

export function validatePermissionCreation(value: PermissionCapabilities, activeRoleMask: bigint,
    dangerousRoleMask: bigint, isGlobalScope: boolean, nowUnixTimestamp: bigint): void {
    validatePermissionCapabilities(value);
    if (value.roleMask === 0n) throw new Error("a new permission must grant at least one role");
    if ((value.roleMask & ~activeRoleMask) !== 0n) throw new Error("permission contains unsupported role bits");
    if ((value.roleMask & dangerousRoleMask) !== 0n) {
        if (isGlobalScope) throw new Error("dangerous permissions require a non-global scope");
        if (value.expiryUnixTimestamp === 0n) throw new Error("dangerous permissions require a finite expiry");
    }
    if (value.expiryUnixTimestamp !== 0n && value.expiryUnixTimestamp <= nowUnixTimestamp) {
        throw new Error("proposed permission has already expired");
    }
}
