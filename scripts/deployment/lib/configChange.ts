import { randomBytes } from "node:crypto";

import { RISK } from "../../../runtime/administration/risk.js";

import { BufferUtil } from "@solomon-labs/buffer";
import { CryptoUtil } from "@solomon-labs/crypto";
import { PublicKey } from "@solomon-labs/publickey";
import type { BI64, BU64, U8, U16 } from "@solomon-labs/types";

import {
    AcceptConfigChangeInstruction,
    ProposeConfigChangeInstruction,
} from "../../../clients/ts/src/instructions/index.js";
import { CHANGE_KIND } from "../../../clients/ts/src/constants.js";
import { pendingConfigChangePda } from "../../../runtime/pdas.js";
import { publicKeyToBytes } from "../../../runtime/publicKey.js";
import { SYSTEM_PROGRAM } from "../../../runtime/wellKnown.js";
import { submitTransaction } from "../../../runtime/submit.js";
import type { RuntimeHarness } from "../../../runtime/harness.js";

import { PendingConfigChange } from "../../../clients/ts/src/accounts/PendingConfigChange.js";
import { PROGRAM_ID } from "../../../clients/ts/src/constants.js";
import { ScheduleConfigChangeInstruction } from "../../../runtime/controller/ScheduleConfigChangeInstruction.js";
import { submitAdministration } from "../../../runtime/administration/submit.js";
import type { AdministrationSubmissionContext } from "../../../runtime/administration/types.js";

export interface ConfigChangeContext extends AdministrationSubmissionContext {
    readonly chanceryConfig: PublicKey;
    readonly eventAuthority: PublicKey;
    readonly moduleActivationState: PublicKey;
    readonly governanceAuthority: PublicKey;
    readonly operationsAuthority: PublicKey;
}

const HASH_DOMAIN_CONFIG_CHANGE = BufferUtil.fromString("chancery:pending-config-change:v1", "utf8");
const HASH_DOMAIN_CHANGE_ID = BufferUtil.fromString("chancery:pending-config-change-id:v1", "utf8");

export { RISK } from "../../../runtime/administration/risk.js";

/**
 * Minimum timelock per risk class, in seconds.
 *
 * These MUST match minimum_timelock_seconds_for_risk in
 * programs/chancery/src/modules/control/pending_change.rs. This is the devnet
 * build, where every timelocked class is 1 second; the production schedule is
 * 2h / 8h / 24h / 48h. If that file changes, change this table with it — a
 * proposal whose executable_after is below the on-chain floor is rejected at
 * propose time.
 */
const MINIMUM_TIMELOCK_SECONDS: Record<number, bigint> = {
    [RISK.RESTRICTIVE_IMMEDIATE]: 0n,
    [RISK.ROUTINE_OPS]: 0n,
    [RISK.WIDENING]: 1n,
    [RISK.HIGH_IMPACT]: 1n,
    [RISK.DANGEROUS]: 1n,
    [RISK.IRREVERSIBLE]: 1n,
};

/** Lifetime of a proposal past its executable point, in seconds. */
const PROPOSAL_LIFETIME_SECONDS = 1_000_000n;

/** How long to keep polling for the timelock to elapse before giving up. */
const TIMELOCK_POLL_TIMEOUT_MILLISECONDS = 120_000;
const TIMELOCK_POLL_INTERVAL_MILLISECONDS = 1_000;

const ASSET_CONFIG_ACCOUNT_SIZE = 280;
const ASSET_CONFIG_PAYLOAD_SIZE = 210;

function concatenateBytes(parts: readonly Uint8Array[]): Uint8Array {
    let length = 0;
    for (const part of parts) length += part.length;
    const output = new Uint8Array(length);
    let offset = 0;
    for (const part of parts) {
        output.set(part, offset);
        offset += part.length;
    }
    return output;
}

function u16BigEndian(value: number): Uint8Array {
    const output = new Uint8Array(2);
    new DataView(output.buffer).setUint16(0, value, false);
    return output;
}

function u64BigEndian(value: bigint): Uint8Array {
    const output = new Uint8Array(8);
    new DataView(output.buffer).setBigUint64(0, value, false);
    return output;
}

export function u64LittleEndian(value: bigint): Uint8Array {
    const output = new Uint8Array(8);
    new DataView(output.buffer).setBigUint64(0, value, true);
    return output;
}

export function i64LittleEndian(value: bigint): Uint8Array {
    const output = new Uint8Array(8);
    new DataView(output.buffer).setBigInt64(0, value, true);
    return output;
}

/** Split a u128 role mask into the two little-endian u64 words the program stores. */
export function u64WordsFromU128(value: bigint): readonly [bigint, bigint] {
    const mask = (1n << 64n) - 1n;
    return [value & mask, (value >> 64n) & mask];
}

/**
 * The canonical semantic encoding of an AssetConfig, matching
 * AssetConfig::config_change_payload. Runtime observation fields
 * (observed_extension_mask, extension_observed_at_slot), the account header,
 * alignment padding, and reserved headroom are all excluded, so later use of
 * reserved storage cannot invalidate a pinned change.
 */
function canonicalAssetConfigPayload(payload: Uint8Array): Uint8Array {
    if (payload.length === ASSET_CONFIG_PAYLOAD_SIZE) return payload;
    if (payload.length !== ASSET_CONFIG_ACCOUNT_SIZE) {
        throw new Error(
            `AssetConfig config-change payload must be ${ASSET_CONFIG_ACCOUNT_SIZE} raw bytes `
            + `or ${ASSET_CONFIG_PAYLOAD_SIZE} canonical bytes, got ${payload.length}`,
        );
    }
    return concatenateBytes([
        payload.slice(11, 13),
        payload.slice(16, 136),
        payload.slice(152, 232),
        payload.slice(240, 248),
    ]);
}

function canonicalPayload(changeKind: number, payload: Uint8Array): Uint8Array {
    if (changeKind === CHANGE_KIND.SET_ASSET_MODE || changeKind === CHANGE_KIND.UPDATE_ASSET_CONFIG) {
        return canonicalAssetConfigPayload(payload);
    }
    // UPSERT_PERMISSION and the rest hash their payload as supplied.
    return payload;
}

export async function computeConfigChangeHash(
    changeKind: number,
    targetAccount: PublicKey,
    riskClass: number,
    payload: Uint8Array,
): Promise<Uint8Array> {
    return CryptoUtil.sha256(concatenateBytes([
        HASH_DOMAIN_CONFIG_CHANGE,
        u16BigEndian(changeKind),
        publicKeyToBytes(targetAccount),
        new Uint8Array([riskClass & 0xff]),
        canonicalPayload(changeKind, payload),
    ]));
}

export async function computeChangeId(
    changeKind: number,
    targetAccount: PublicKey,
    newValueHash: Uint8Array,
    proposer: PublicKey,
    proposerNonce: bigint,
): Promise<Uint8Array> {
    return CryptoUtil.sha256(concatenateBytes([
        HASH_DOMAIN_CHANGE_ID,
        u16BigEndian(changeKind),
        publicKeyToBytes(targetAccount),
        newValueHash,
        publicKeyToBytes(proposer),
        u64BigEndian(proposerNonce),
    ]));
}

export async function readAccountBytes(
    harness: RuntimeHarness,
    account: PublicKey,
): Promise<Uint8Array> {
    const data = await harness.banksClient.getAccount(account);
    if (data === null) throw new Error(`account ${account.toBase58()} not found`);
    return Uint8Array.from(data.data);
}

/**
 * Wait until the on-chain clock has passed `executableAfterUnixTimestamp`.
 *
 * Devnet's clock cannot be moved, so this polls. With the devnet floor of one
 * second this normally returns after a slot or two.
 */
async function waitForTimelock(
    harness: RuntimeHarness,
    executableAfterUnixTimestamp: bigint,
): Promise<void> {
    const deadline = Date.now() + TIMELOCK_POLL_TIMEOUT_MILLISECONDS;
    for (;;) {
        const clock = await harness.banksClient.getClock();
        if (clock.unixTimestamp >= executableAfterUnixTimestamp) return;
        if (Date.now() > deadline) {
            throw new Error(
                `timed out waiting for the config-change timelock to elapse `
                + `(on-chain ${clock.unixTimestamp}, executable after ${executableAfterUnixTimestamp})`,
            );
        }
        await new Promise((resolve) => setTimeout(resolve, TIMELOCK_POLL_INTERVAL_MILLISECONDS));
    }
}


/**
 * Propose a change, wait out its timelock, and accept it. Returns the
 * PendingConfigChange PDA for the caller to pass to the target handler's
 * `_with_pending_change` variant.
 */
export async function proposeAndAcceptConfigChange(args: {
    readonly context: ConfigChangeContext;
    readonly changeKind: number;
    readonly riskClass: number;
    readonly targetAccount: PublicKey;
    readonly oldPayload: Uint8Array;
    readonly newPayload: Uint8Array;
    readonly proposerNonce?: bigint;
}): Promise<PublicKey> {
    const { context, changeKind, riskClass, targetAccount, oldPayload, newPayload } = args;
    const minimumTimelock = MINIMUM_TIMELOCK_SECONDS[riskClass];
    if (minimumTimelock === undefined || minimumTimelock <= 0n) {
        throw new Error("a pending change requires a supported timelocked risk class");
    }
    const oldHash = await computeConfigChangeHash(changeKind, targetAccount, riskClass, oldPayload);
    const newHash = await computeConfigChangeHash(changeKind, targetAccount, riskClass, newPayload);
    const nonce = args.proposerNonce ?? randomBytes(8).readBigUInt64LE();
    const changeId = await computeChangeId(changeKind, targetAccount, newHash, context.governanceAuthority, nonce);
    const [pendingConfigChange] = await pendingConfigChangePda(changeId);
    const clock = await context.harness.banksClient.getClock();
    // Direct signatures need submission headroom; controller scheduling uses the execution clock.
    const executableAfterUnixTimestamp = clock.unixTimestamp + minimumTimelock
        + (context.authorization.kind === "keypair" ? 30n : 0n);
    const proposal = new ProposeConfigChangeInstruction({
        activationState: context.moduleActivationState,
        chanceryConfig: context.chanceryConfig,
        eventAuthority: context.eventAuthority,
        pending: pendingConfigChange,
        payer: context.feePayer.publicKey,
        governanceAuthority: context.governanceAuthority,
        system: SYSTEM_PROGRAM,
        changeKind: changeKind as U16,
        riskClass: riskClass as U8,
        targetAccount,
        oldValueHash: Array.from(oldHash) as U8[],
        newValueHash: Array.from(newHash) as U8[],
        executableAfterUnixTimestamp: executableAfterUnixTimestamp as BI64,
        expiresAtUnixTimestamp: (executableAfterUnixTimestamp + PROPOSAL_LIFETIME_SECONDS) as BI64,
        proposerNonce: nonce as BU64,
    });
    if (context.authorization.kind === "controller") {
        await submitTransaction(context.harness, [new ScheduleConfigChangeInstruction(
            context.authorization.controller, context.feePayer.publicKey, proposal,
            minimumTimelock, PROPOSAL_LIFETIME_SECONDS,
        )], [], context.feePayer);
    } else {
        await submitAdministration(context, [proposal]);
    }
    const account = await context.harness.banksClient.getAccount(pendingConfigChange);
    if (account === null || account.owner?.equals(PROGRAM_ID) !== true) {
        throw new Error("pending config-change account is absent or has the wrong owner");
    }
    const pending = PendingConfigChange.decode(account.data, pendingConfigChange);
    await waitForTimelock(context.harness, BigInt(pending.executableAfterUnixTimestamp));
    await submitAdministration(context, [new AcceptConfigChangeInstruction({
        activationState: context.moduleActivationState,
        chanceryConfig: context.chanceryConfig,
        eventAuthority: context.eventAuthority,
        pending: pendingConfigChange,
        governanceAuthority: context.governanceAuthority,
    })]);
    return pendingConfigChange;
}

/**
 * The canonical value payload for a permission record, matching
 * PermissionRecord's config-change encoding.
 */
export function permissionValuePayload(args: {
    readonly subject: PublicKey;
    readonly scopeKind: number;
    readonly scopeKey: PublicKey;
    readonly roleMask: bigint;
    readonly permissionGeneration: bigint;
    readonly expiryUnixTimestamp: bigint;
}): Uint8Array {
    const maximumUnsigned64 = (1n << 64n) - 1n;
    if (args.permissionGeneration < 0n || args.permissionGeneration > maximumUnsigned64) {
        throw new Error("permission generation is outside the u64 range");
    }
    const roleWords = u64WordsFromU128(args.roleMask);
    return concatenateBytes([
        publicKeyToBytes(args.subject),
        new Uint8Array([args.scopeKind & 0xff]),
        publicKeyToBytes(args.scopeKey),
        u64LittleEndian(roleWords[0]),
        u64LittleEndian(roleWords[1]),
        u64LittleEndian(args.permissionGeneration),
        i64LittleEndian(args.expiryUnixTimestamp),
    ]);
}
