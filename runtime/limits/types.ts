import type { PublicKey } from "@solomon-labs/publickey";

export interface LimitCaps {
    readonly perTransactionMaximum: bigint;
    readonly perHourMaximum: bigint;
    readonly perDayMaximum: bigint;
    readonly perSevenDayMaximum: bigint;
    readonly perThirtyDayMaximum: bigint;
    readonly maximumActionsPerHour: number;
    readonly maximumActionsPerDay: number;
}

export interface LimitCapChanges {
    readonly perTransactionMaximum: bigint | null;
    readonly perHourMaximum: bigint | null;
    readonly perDayMaximum: bigint | null;
    readonly perSevenDayMaximum: bigint | null;
    readonly perThirtyDayMaximum: bigint | null;
    readonly maximumActionsPerHour: number | null;
    readonly maximumActionsPerDay: number | null;
}

export interface LimitWindowAccounts {
    readonly hourlyUsageWindow: PublicKey;
    readonly dailyUsageWindow: PublicKey;
    readonly weeklyUsageWindow: PublicKey;
    readonly monthlyUsageWindow: PublicKey;
}

export interface PrimaryLimitAccounts extends LimitWindowAccounts {
    readonly limitPolicy: PublicKey;
}
