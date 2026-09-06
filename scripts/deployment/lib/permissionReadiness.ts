import { PublicKey } from "@solomon-labs/publickey";

import { PermissionRecord } from "../../../clients/ts/src/accounts/PermissionRecord.js";
import { PROGRAM_ID, ROLE, SCOPE } from "../../../clients/ts/src/constants.js";
import { permissionRecordPda } from "../../../runtime/pdas.js";
import { publicKeyToBase58 } from "../../../runtime/publicKey.js";
import { DEFAULT_PUBLIC_KEY } from "../../../runtime/wellKnown.js";
import type { ConfigChangeContext } from "./configChange.js";
import { grantPermission, type GrantPermissionOptions } from "./permissionGrant.js";

export async function ensurePermission(context: ConfigChangeContext, subject: PublicKey,
    roleMask: bigint, options: GrantPermissionOptions = {}): Promise<PublicKey> {
    const scopeKind = options.scopeKind ?? SCOPE.GLOBAL;
    const scopeKey = options.scopeKey ?? DEFAULT_PUBLIC_KEY;
    const expiry = options.expiryUnixTimestamp ?? 0n;
    const [address] = await permissionRecordPda(subject, scopeKind, scopeKey);
    const account = await context.harness.banksClient.getAccount(address);
    if (account === null) return grantPermission(context, subject, roleMask, options);
    if (account.owner?.equals(PROGRAM_ID) !== true) throw new Error("permission record has the wrong owner");
    const permission = PermissionRecord.decode(account.data, address);
    const roles = BigInt(permission.roleBits[0]!) | (BigInt(permission.roleBits[1]!) << 64n);
    if (publicKeyToBase58(permission.subject) !== subject.toBase58()
        || Number(permission.scopeKind) !== scopeKind
        || publicKeyToBase58(permission.scopeKey) !== scopeKey.toBase58()
        || Number(permission.roleSchemaVersion) !== ROLE.PERMISSION_ROLE_SCHEMA_VERSION
        || (roles & ~ROLE.ACTIVE_ROLE_MASK) !== 0n
        || (roles & roleMask) !== roleMask
        || (BigInt(permission.permissionFlags) & 1n) !== 0n
        || BigInt(permission.expiryUnixTimestamp) !== expiry) {
        throw new Error("existing permission does not provide the requested active grant: " + address.toBase58());
    }
    if (expiry !== 0n && (await context.harness.banksClient.getClock()).unixTimestamp >= expiry) {
        throw new Error("permission is expired: " + address.toBase58());
    }
    return address;
}
