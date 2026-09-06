import type { PublicKey } from "@solomon-labs/publickey";

export interface PermissionCapabilities {
    readonly roleMask: bigint;
    readonly expiryUnixTimestamp: bigint;
}

export interface PermissionChanges {
    readonly roleMask: bigint | null;
    readonly expiryUnixTimestamp: bigint | null;
}

export interface PermissionGenerationState extends PermissionCapabilities {
    readonly permissionFlags: bigint;
    readonly roleSchemaVersion: number;
    readonly permissionGeneration: bigint;
}

export interface PermissionDailyWindows {
    readonly counterpartyDailyUsageWindow: PublicKey;
    readonly executorDailyUsageWindow: PublicKey;
}
