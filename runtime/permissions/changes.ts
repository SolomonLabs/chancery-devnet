import { RISK } from "../administration/risk.js";
import type { ConfigChangeRisk } from "../administration/types.js";
import type { PermissionCapabilities, PermissionChanges } from "./types.js";
import { validatePermissionCapabilities } from "./validation.js";

export function applyPermissionChanges(current: PermissionCapabilities, changes: PermissionChanges): PermissionCapabilities {
    const proposed = {
        roleMask: changes.roleMask ?? current.roleMask,
        expiryUnixTimestamp: changes.expiryUnixTimestamp ?? current.expiryUnixTimestamp,
    };
    validatePermissionCapabilities(proposed);
    return proposed;
}

export function classifyPermissionChange(current: PermissionCapabilities | null,
    proposed: PermissionCapabilities, dangerousRoleMask: bigint): ConfigChangeRisk {
    const dangerous = (proposed.roleMask & dangerousRoleMask) !== 0n;
    if (current === null) return dangerous ? RISK.DANGEROUS : RISK.WIDENING;
    const addedRoles = proposed.roleMask & ~current.roleMask;
    const expiryRemoved = current.expiryUnixTimestamp !== 0n && proposed.expiryUnixTimestamp === 0n;
    const expiryExtended = current.expiryUnixTimestamp !== 0n
        && proposed.expiryUnixTimestamp > current.expiryUnixTimestamp;
    if (addedRoles !== 0n || expiryRemoved || expiryExtended) {
        if (dangerous) return RISK.DANGEROUS;
        return expiryRemoved ? RISK.HIGH_IMPACT : RISK.WIDENING;
    }
    if (current.roleMask !== proposed.roleMask || current.expiryUnixTimestamp !== proposed.expiryUnixTimestamp) {
        return RISK.RESTRICTIVE_IMMEDIATE;
    }
    return RISK.ROUTINE_OPS;
}
