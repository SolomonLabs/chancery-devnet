import { PublicKey } from "@solomon-labs/publickey";
import type { BU64, U32, U8 } from "@solomon-labs/types";

import { LimitPolicy } from "../../clients/ts/src/accounts/LimitPolicy.js";
import { CHANGE_KIND, PROGRAM_ID, SCOPE } from "../../clients/ts/src/constants.js";
import { RegisterLimitPolicyInstruction, UpdateLimitPolicyInstruction, UpdateLimitPolicyWithPendingChangeInstruction } from "../../clients/ts/src/instructions/index.js";
import { submitAdministration } from "../../runtime/administration/submit.js";
import { createDevnetHarness } from "../../runtime/devnetHarness.js";
import { applyLimitCapChanges, limitCapsWiden, limitPolicyPayload, validateLimitCaps } from "../../runtime/limits/caps.js";
import type { LimitCapChanges, LimitCaps } from "../../runtime/limits/types.js";
import { limitWindowAccounts } from "../../runtime/limits/windows.js";
import { limitPolicyPda, pathwayPolicyPda } from "../../runtime/pdas.js";
import { publicKeyToBytes } from "../../runtime/publicKey.js";
import { createControllerContext } from "../deployment/lib/administrationContext.js";
import { activeNetwork, DRY_RUN, preflight, publicKeyFromAddress } from "../deployment/lib/bootstrap.js";
import { proposeAndAcceptConfigChange, RISK } from "../deployment/lib/configChange.js";
import { parsePathwayId } from "../deployment/lib/pathwayIdentity.js";
import { flagValue, loadPublicCaller } from "../deployment/lib/publicCaller.js";

function volumeCap(flag: string): bigint | null {
    const text = flagValue(flag);
    if (text === null) return null;
    if (!/^(0|[1-9][0-9]*)$/u.test(text)) throw new Error(flag + " must be an unsigned base-unit integer");
    const value = BigInt(text);
    if (value > (1n << 64n) - 1n) throw new Error(flag + " exceeds u64");
    return value;
}

function actionCap(flag: string): number | null {
    const value = volumeCap(flag);
    if (value === null) return null;
    if (value > 0xffff_ffffn) throw new Error(flag + " exceeds u32");
    return Number(value);
}

async function main(): Promise<void> {
    preflight();
    if (DRY_RUN) throw new Error("limit changes require committed state; --dry-run is unsupported");
    const create = process.argv.includes("--create");
    if (create === process.argv.includes("--update")) throw new Error("select either --create or --update");
    const identifierText = flagValue("--limit-policy-id");
    if (identifierText === null) throw new Error("--limit-policy-id requires a nonzero 32-byte hexadecimal identifier");
    const identifier = parsePathwayId(identifierText);
    const changes: LimitCapChanges = {
        perTransactionMaximum: volumeCap("--per-transaction"), perHourMaximum: volumeCap("--per-hour"),
        perDayMaximum: volumeCap("--per-day"), perSevenDayMaximum: volumeCap("--per-seven-day"),
        perThirtyDayMaximum: volumeCap("--per-thirty-day"), maximumActionsPerHour: actionCap("--actions-per-hour"),
        maximumActionsPerDay: actionCap("--actions-per-day"),
    };
    if (!create && [changes.perTransactionMaximum, changes.perHourMaximum, changes.perDayMaximum,
        changes.perSevenDayMaximum, changes.perThirtyDayMaximum, changes.maximumActionsPerHour,
        changes.maximumActionsPerDay].every((value) => value === null)) throw new Error("specify at least one cap to update");
    const network = activeNetwork();
    const caller = loadPublicCaller();
    const harness = await createDevnetHarness({ rpcUrl: network.rpcUrl, commitment: "confirmed", requireProgramdata: false });
    const context = await createControllerContext(harness, caller, publicKeyFromAddress(network.faucetProgramId));
    const [address] = await limitPolicyPda(identifier);
    const existing = await harness.banksClient.getAccount(address);
    if (create ? existing !== null : existing === null) throw new Error(create ? "limit policy already exists" : "limit policy is absent");
    if (existing !== null && existing.owner?.equals(PROGRAM_ID) !== true) throw new Error("limit policy has the wrong owner");
    const current = existing === null ? null : LimitPolicy.decode(existing.data, address);
    if (current !== null && Number(current.scopeKind) !== SCOPE.PATHWAY) throw new Error("this command requires a pathway-scoped limit policy");
    const pathwayText = flagValue("--pathway-id");
    if (create && pathwayText === null) throw new Error("--pathway-id is required when creating a pathway limit policy");
    const scopeKey = current === null
        ? (await pathwayPolicyPda(parsePathwayId(pathwayText!)))[0]
        : new PublicKey(publicKeyToBytes(current.scopeKey));
    if (pathwayText !== null && !(await pathwayPolicyPda(parsePathwayId(pathwayText)))[0].equals(scopeKey)) {
        throw new Error("--pathway-id differs from the existing policy scope");
    }
    const previous: LimitCaps = current === null ? {
        perTransactionMaximum: 0n, perHourMaximum: 0n, perDayMaximum: 0n,
        perSevenDayMaximum: 0n, perThirtyDayMaximum: 0n, maximumActionsPerHour: 0, maximumActionsPerDay: 0,
    } : current;
    const proposed = applyLimitCapChanges(previous, changes);
    validateLimitCaps(proposed);
    const windows = await limitWindowAccounts(SCOPE.PATHWAY, scopeKey, proposed);
    const common = {
        moduleActivationState: context.moduleActivationState, chanceryConfig: context.chanceryConfig,
        limitPolicy: address, limitPolicyId: Array.from(identifier) as U8[], payer: caller.publicKey, ...windows,
    };
    if (create) {
        await submitAdministration(context, [new RegisterLimitPolicyInstruction({ ...common,
            scopeKind: SCOPE.PATHWAY as U8, scopeKey, eventAuthority: context.eventAuthority,
            operationsAuthority: context.operationsAuthority,
            perTransactionMaximum: proposed.perTransactionMaximum as BU64, perHourMaximum: proposed.perHourMaximum as BU64,
            perDayMaximum: proposed.perDayMaximum as BU64, perSevenDayMaximum: proposed.perSevenDayMaximum as BU64,
            perThirtyDayMaximum: proposed.perThirtyDayMaximum as BU64, maximumActionsPerHour: proposed.maximumActionsPerHour as U32,
            maximumActionsPerDay: proposed.maximumActionsPerDay as U32,
        })]);
    } else {
        const update = { ...common,
            perTransactionMaximum: changes.perTransactionMaximum as BU64 | null,
            perHourMaximum: changes.perHourMaximum as BU64 | null, perDayMaximum: changes.perDayMaximum as BU64 | null,
            perSevenDayMaximum: changes.perSevenDayMaximum as BU64 | null, perThirtyDayMaximum: changes.perThirtyDayMaximum as BU64 | null,
            maximumActionsPerHour: changes.maximumActionsPerHour as U32 | null, maximumActionsPerDay: changes.maximumActionsPerDay as U32 | null,
        };
        if (limitCapsWiden(previous, proposed)) {
            const pendingConfigChange = await proposeAndAcceptConfigChange({ context, changeKind: CHANGE_KIND.UPDATE_LIMIT_POLICY,
                riskClass: RISK.WIDENING, targetAccount: address,
                oldPayload: limitPolicyPayload(existing!.data, previous), newPayload: limitPolicyPayload(existing!.data, proposed),
            });
            await submitAdministration(context, [new UpdateLimitPolicyWithPendingChangeInstruction({ ...update,
                eventAuthority: context.eventAuthority, governanceAuthority: context.governanceAuthority,
                pendingConfigChange, rentRefundRecipient: caller.publicKey,
            })]);
        } else {
            await submitAdministration(context, [new UpdateLimitPolicyInstruction({ ...update,
                eventAuthorityDirect: context.eventAuthority, operationsAuthority: context.operationsAuthority,
            })]);
        }
    }
    const after = await harness.banksClient.getAccount(address);
    if (after === null || after.owner?.equals(PROGRAM_ID) !== true) throw new Error("limit policy readback failed");
    const observed = LimitPolicy.decode(after.data, address);
    const expected = limitPolicyPayload(after.data, proposed);
    if (limitPolicyPayload(after.data, observed).some((byte, index) => byte !== expected[index])) throw new Error("limit cap readback differs from the submitted values");
    console.log("limit policy: " + address.toBase58());
}

main().catch((error: unknown) => {
    console.error(error instanceof Error ? error.stack ?? error.message : String(error));
    process.exitCode = 1;
});
