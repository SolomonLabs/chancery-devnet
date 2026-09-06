import { PublicKey } from "@solomon-labs/publickey";
import type { BI64, BU64, U8 } from "@solomon-labs/types";

import { PermissionRecord } from "../../../clients/ts/src/accounts/PermissionRecord.js";
import { CHANGE_KIND, PROGRAM_ID, ROLE, SCOPE } from "../../../clients/ts/src/constants.js";
import { UpsertPermissionInstruction, UpsertPermissionWithPendingChangeInstruction } from "../../../clients/ts/src/instructions/index.js";
import { RISK } from "../../../runtime/administration/risk.js";
import { submitAdministration } from "../../../runtime/administration/submit.js";
import { permissionRecordPda } from "../../../runtime/pdas.js";
import { applyPermissionChanges, classifyPermissionChange } from "../../../runtime/permissions/changes.js";
import { effectivePermissionGeneration, nextPermissionGeneration } from "../../../runtime/permissions/generation.js";
import type { PermissionChanges } from "../../../runtime/permissions/types.js";
import { publicKeyToBase58 } from "../../../runtime/publicKey.js";
import { DEFAULT_PUBLIC_KEY, SYSTEM_PROGRAM } from "../../../runtime/wellKnown.js";
import { permissionValuePayload, proposeAndAcceptConfigChange, u64WordsFromU128 } from "./configChange.js";
import type { ConfigChangeContext } from "./configChange.js";
import type { GrantPermissionOptions } from "./permissionGrant.js";
import { permissionDailyWindows } from "./permissionWindows.js";

export async function updatePermission(context: ConfigChangeContext, subject: PublicKey,
    changes: PermissionChanges, options: Pick<GrantPermissionOptions, "scopeKind" | "scopeKey" | "grantorPermission"> = {}): Promise<PublicKey> {
    const scopeKind = options.scopeKind ?? SCOPE.GLOBAL;
    const scopeKey = options.scopeKey ?? DEFAULT_PUBLIC_KEY;
    const [address] = await permissionRecordPda(subject, scopeKind, scopeKey);
    const account = await context.harness.banksClient.getAccount(address);
    if (account === null || account.owner?.equals(PROGRAM_ID) !== true) {
        throw new Error("permission record is absent or has the wrong owner: " + address.toBase58());
    }
    const current = PermissionRecord.decode(account.data, address);
    if (publicKeyToBase58(current.subject) !== subject.toBase58() || Number(current.scopeKind) !== scopeKind
        || publicKeyToBase58(current.scopeKey) !== scopeKey.toBase58()
        || Number(current.roleSchemaVersion) !== ROLE.PERMISSION_ROLE_SCHEMA_VERSION) {
        throw new Error("permission identity or role schema differs from the requested record");
    }
    const previous = {
        roleMask: BigInt(current.roleBits[0]!) | (BigInt(current.roleBits[1]!) << 64n),
        expiryUnixTimestamp: BigInt(current.expiryUnixTimestamp),
    };
    const proposed = applyPermissionChanges(previous, changes);
    if ((proposed.roleMask & ~ROLE.ACTIVE_ROLE_MASK) !== 0n) throw new Error("permission contains unsupported role bits");
    const riskClass = classifyPermissionChange(previous, proposed, ROLE.DANGEROUS_PERMISSION_ROLE_MASK);
    if (riskClass === RISK.ROUTINE_OPS) return address;
    if (riskClass >= RISK.WIDENING && proposed.expiryUnixTimestamp !== 0n
        && proposed.expiryUnixTimestamp <= (await context.harness.banksClient.getClock()).unixTimestamp) {
        throw new Error("proposed permission has already expired");
    }
    const previousGeneration = effectivePermissionGeneration({ ...previous,
        permissionFlags: BigInt(current.permissionFlags), roleSchemaVersion: Number(current.roleSchemaVersion),
        permissionGeneration: BigInt(current.permissionGeneration),
    }, ROLE.PERMISSION_ROLE_SCHEMA_VERSION);
    const proposedGeneration = nextPermissionGeneration(previousGeneration);
    const common = {
        moduleActivationState: context.moduleActivationState, chanceryConfig: context.chanceryConfig,
        eventAuthority: context.eventAuthority, permissionRecord: address,
        grantingAuthority: context.governanceAuthority,
        grantorPermission: options.grantorPermission ?? DEFAULT_PUBLIC_KEY,
        subject, scopeKind: scopeKind as U8, scopeKey,
        roleBits: u64WordsFromU128(proposed.roleMask).map((word) => word as BU64),
        expiryUnixTimestamp: proposed.expiryUnixTimestamp as BI64,
    };
    if (riskClass >= RISK.WIDENING) {
        const identity = { subject, scopeKind, scopeKey };
        const pending = await proposeAndAcceptConfigChange({ context, changeKind: CHANGE_KIND.UPSERT_PERMISSION,
            riskClass, targetAccount: address,
            oldPayload: permissionValuePayload({ ...identity, ...previous, permissionGeneration: previousGeneration }),
            newPayload: permissionValuePayload({ ...identity, ...proposed, permissionGeneration: proposedGeneration }),
        });
        await submitAdministration(context, [new UpsertPermissionWithPendingChangeInstruction({ ...common,
            pending, payer: context.feePayer.publicKey, governanceAuthority: context.governanceAuthority,
            systemProgram: SYSTEM_PROGRAM, rentRefundRecipient: context.feePayer.publicKey,
            ...(await permissionDailyWindows(subject)),
        })]);
    } else {
        await submitAdministration(context, [new UpsertPermissionInstruction(common)]);
    }
    const after = await context.harness.banksClient.getAccount(address);
    if (after === null || after.owner?.equals(PROGRAM_ID) !== true) throw new Error("permission readback failed");
    const observed = PermissionRecord.decode(after.data, address);
    const observedRoles = BigInt(observed.roleBits[0]!) | (BigInt(observed.roleBits[1]!) << 64n);
    if (observedRoles !== proposed.roleMask || BigInt(observed.expiryUnixTimestamp) !== proposed.expiryUnixTimestamp
        || BigInt(observed.permissionGeneration) !== proposedGeneration) {
        throw new Error("permission readback differs from the submitted values");
    }
    return address;
}
