import { CryptoUtil } from "@solomon-labs/crypto";
import type { PublicKey } from "@solomon-labs/publickey";

import { usageWindowPda } from "../pdas.js";
import { DEFAULT_PUBLIC_KEY } from "../wellKnown.js";
import type { LimitCaps, LimitWindowAccounts } from "./types.js";

export async function limitWindowAccounts(scopeKind: number, scopeKey: PublicKey, caps: LimitCaps): Promise<LimitWindowAccounts> {
    const identity = new Uint8Array(33);
    identity[0] = scopeKind;
    identity.set(scopeKey.toBytes(), 1);
    const hash = await CryptoUtil.sha256(identity);
    return {
        hourlyUsageWindow: caps.perHourMaximum !== 0n || caps.maximumActionsPerHour !== 0
            ? (await usageWindowPda(hash, 0))[0] : DEFAULT_PUBLIC_KEY,
        dailyUsageWindow: caps.perDayMaximum !== 0n || caps.maximumActionsPerDay !== 0
            ? (await usageWindowPda(hash, 1))[0] : DEFAULT_PUBLIC_KEY,
        weeklyUsageWindow: caps.perSevenDayMaximum !== 0n ? (await usageWindowPda(hash, 2))[0] : DEFAULT_PUBLIC_KEY,
        monthlyUsageWindow: caps.perThirtyDayMaximum !== 0n ? (await usageWindowPda(hash, 3))[0] : DEFAULT_PUBLIC_KEY,
    };
}
