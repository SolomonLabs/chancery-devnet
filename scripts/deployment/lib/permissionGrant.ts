// Grant a permission through the full timelocked cycle.
//
// Permission grants are never immediate: creating a record is Widening risk,
// or Dangerous when the mask includes a dangerous role. Both require
// propose -> accept -> upsert_permission_with_pending_change.

import { PublicKey } from "@solomon-labs/publickey";
import type { BI64, BU64, U8 } from "@solomon-labs/types";

import { UpsertPermissionWithPendingChangeInstruction } from "../../../clients/ts/src/instructions/index.js";
import { PermissionRecord } from "../../../clients/ts/src/accounts/PermissionRecord.js";
import { CHANGE_KIND, PROGRAM_ID, ROLE, SCOPE } from "../../../clients/ts/src/constants.js";
import { classifyPermissionChange } from "../../../runtime/permissions/changes.js";
import { validatePermissionCreation } from "../../../runtime/permissions/validation.js";
import { publicKeyToBase58 } from "../../../runtime/publicKey.js";
import { permissionRecordPda } from "../../../runtime/pdas.js";
import { DEFAULT_PUBLIC_KEY, SYSTEM_PROGRAM, toSolanaKey } from "../../../runtime/wellKnown.js";
import type { E2ESigner } from "../../../runtime/submit.js";
import { submitAdministration } from "../../../runtime/administration/submit.js";
import {
    proposeAndAcceptConfigChange,
    permissionValuePayload,
    u64WordsFromU128,
    type ConfigChangeContext,
} from "./configChange.js";

import { permissionDailyWindows } from "./permissionWindows.js";

export interface GrantPermissionOptions {
    readonly scopeKind?: number;
    readonly scopeKey?: PublicKey;
    readonly expiryUnixTimestamp?: bigint;
    readonly grantorPermission?: PublicKey;
}

/**
 * Create a permission record for `subject` carrying `roleMask`.
 *
 * Only handles creation: the record must not already exist. Updating an
 * existing grant classifies risk from the transition (added roles, removed
 * roles, expiry movement) rather than from the mask alone.
 */
export async function grantPermission(
    context: ConfigChangeContext,
    subject: PublicKey,
    roleMask: bigint,
    options: GrantPermissionOptions = {},
): Promise<PublicKey> {
    const scopeKind = options.scopeKind ?? SCOPE.GLOBAL;
    const scopeKey = options.scopeKey ?? DEFAULT_PUBLIC_KEY;
    const expiryUnixTimestamp = options.expiryUnixTimestamp ?? 0n;
    if (subject.equals(DEFAULT_PUBLIC_KEY)) throw new Error("permission subject must be nonzero");
    if (!Object.values(SCOPE).some((knownScope) => knownScope === scopeKind)) {
        throw new Error("permission scope is unsupported");
    }
    const proposed = { roleMask, expiryUnixTimestamp };
    validatePermissionCreation(proposed, ROLE.ACTIVE_ROLE_MASK, ROLE.DANGEROUS_PERMISSION_ROLE_MASK,
        scopeKind === SCOPE.GLOBAL, (await context.harness.banksClient.getClock()).unixTimestamp);

    const [recordKey] = await permissionRecordPda(subject, scopeKind, scopeKey);
    const permissionRecord = toSolanaKey(recordKey);

    const existing = await context.harness.banksClient.getAccount(permissionRecord);
    if (existing !== null) {
        throw new Error(
            `permission record ${permissionRecord.toBase58()} already exists; `
            + "this helper only creates grants",
        );
    }

    const riskClass = classifyPermissionChange(null, proposed, ROLE.DANGEROUS_PERMISSION_ROLE_MASK);
    const identity = { subject, scopeKind, scopeKey };
    const oldPayload = permissionValuePayload({
        ...identity,
        roleMask: 0n,
        permissionGeneration: 0n,
        expiryUnixTimestamp: 0n,
    });
    const newPayload = permissionValuePayload({
        ...identity,
        roleMask,
        permissionGeneration: 1n,
        expiryUnixTimestamp,
    });

    const pending = await proposeAndAcceptConfigChange({
        context,
        changeKind: CHANGE_KIND.UPSERT_PERMISSION,
        riskClass,
        targetAccount: permissionRecord,
        oldPayload,
        newPayload,
    });

    await submitAdministration(
        context,
        [new UpsertPermissionWithPendingChangeInstruction({
            moduleActivationState: context.moduleActivationState,
            chanceryConfig: context.chanceryConfig,
            eventAuthority: context.eventAuthority,
            pending,
            permissionRecord,
            payer: context.feePayer.publicKey,
            grantingAuthority: context.governanceAuthority,
            governanceAuthority: context.governanceAuthority,
            systemProgram: SYSTEM_PROGRAM,
            grantorPermission: options.grantorPermission ?? DEFAULT_PUBLIC_KEY,
            subject,
            scopeKind: scopeKind as U8,
            scopeKey,
            roleBits: u64WordsFromU128(roleMask).map((word) => word as BU64),
            expiryUnixTimestamp: expiryUnixTimestamp as BI64,
            ...(await permissionDailyWindows(subject)),
            rentRefundRecipient: context.feePayer.publicKey,
        })],
    );

    const account = await context.harness.banksClient.getAccount(permissionRecord);
    if (account === null || account.owner?.equals(PROGRAM_ID) !== true) {
        throw new Error("permission creation readback failed");
    }
    const observed = PermissionRecord.decode(account.data, permissionRecord);
    const observedRoles = BigInt(observed.roleBits[0]!) | (BigInt(observed.roleBits[1]!) << 64n);
    if (publicKeyToBase58(observed.subject) !== subject.toBase58()
        || Number(observed.scopeKind) !== scopeKind
        || publicKeyToBase58(observed.scopeKey) !== scopeKey.toBase58()
        || observedRoles !== roleMask || BigInt(observed.expiryUnixTimestamp) !== expiryUnixTimestamp
        || BigInt(observed.permissionGeneration) !== 1n || BigInt(observed.permissionFlags) !== 0n
        || Number(observed.roleSchemaVersion) !== ROLE.PERMISSION_ROLE_SCHEMA_VERSION) {
        throw new Error("permission creation readback differs from the submitted values");
    }
    return permissionRecord;
}

export type { E2ESigner };
