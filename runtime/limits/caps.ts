import type { LimitCapChanges, LimitCaps } from "./types.js";

export function applyLimitCapChanges(current: LimitCaps, changes: LimitCapChanges): LimitCaps {
    return {
        perTransactionMaximum: changes.perTransactionMaximum ?? current.perTransactionMaximum,
        perHourMaximum: changes.perHourMaximum ?? current.perHourMaximum,
        perDayMaximum: changes.perDayMaximum ?? current.perDayMaximum,
        perSevenDayMaximum: changes.perSevenDayMaximum ?? current.perSevenDayMaximum,
        perThirtyDayMaximum: changes.perThirtyDayMaximum ?? current.perThirtyDayMaximum,
        maximumActionsPerHour: changes.maximumActionsPerHour ?? current.maximumActionsPerHour,
        maximumActionsPerDay: changes.maximumActionsPerDay ?? current.maximumActionsPerDay,
    };
}

export function validateLimitCaps(caps: LimitCaps): void {
    let previous = 0n;
    for (const cap of [caps.perTransactionMaximum, caps.perHourMaximum, caps.perDayMaximum,
        caps.perSevenDayMaximum, caps.perThirtyDayMaximum]) {
        if (cap < 0n || cap > (1n << 64n) - 1n) throw new Error("volume cap is outside the u64 range");
        if (cap === 0n) continue;
        if (cap < previous) throw new Error("enabled volume caps must be non-decreasing across periods");
        previous = cap;
    }
    for (const cap of [caps.maximumActionsPerHour, caps.maximumActionsPerDay]) {
        if (!Number.isInteger(cap) || cap < 0 || cap > 0xffff_ffff) throw new Error("action cap is outside the u32 range");
    }
    if (caps.maximumActionsPerHour !== 0 && caps.maximumActionsPerDay !== 0
        && caps.maximumActionsPerHour > caps.maximumActionsPerDay) {
        throw new Error("hourly action cap exceeds daily action cap");
    }
}

export function limitCapsWiden(current: LimitCaps, proposed: LimitCaps): boolean {
    const pairs = [
        [current.perTransactionMaximum, proposed.perTransactionMaximum],
        [current.perHourMaximum, proposed.perHourMaximum],
        [current.perDayMaximum, proposed.perDayMaximum],
        [current.perSevenDayMaximum, proposed.perSevenDayMaximum],
        [current.perThirtyDayMaximum, proposed.perThirtyDayMaximum],
        [BigInt(current.maximumActionsPerHour), BigInt(proposed.maximumActionsPerHour)],
        [BigInt(current.maximumActionsPerDay), BigInt(proposed.maximumActionsPerDay)],
    ] as const;
    return pairs.some(([oldValue, newValue]) => oldValue !== 0n && (newValue === 0n || newValue > oldValue));
}

export function limitPolicyPayload(account: Uint8Array, caps: LimitCaps): Uint8Array {
    if (account.length !== 184) throw new Error("unexpected LimitPolicy account size");
    validateLimitCaps(caps);
    const payload = new Uint8Array(113);
    payload.set(account.subarray(16, 48), 0);
    payload[32] = account[11]!;
    payload.set(account.subarray(48, 80), 33);
    const view = new DataView(payload.buffer);
    view.setBigUint64(65, caps.perTransactionMaximum, true);
    view.setBigUint64(73, caps.perHourMaximum, true);
    view.setBigUint64(81, caps.perDayMaximum, true);
    view.setBigUint64(89, caps.perSevenDayMaximum, true);
    view.setBigUint64(97, caps.perThirtyDayMaximum, true);
    view.setUint32(105, caps.maximumActionsPerHour, true);
    view.setUint32(109, caps.maximumActionsPerDay, true);
    return payload;
}
