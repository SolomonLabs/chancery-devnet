import { CryptoUtil } from "@solomon-labs/crypto";
import { PublicKey } from "@solomon-labs/publickey";

import { usageWindowPda } from "../../../runtime/pdas.js";
import { publicKeyToBytes } from "../../../runtime/publicKey.js";
import type { PermissionDailyWindows } from "../../../runtime/permissions/types.js";

const SCOPE_KIND_COUNTERPARTY = 0x0c;
const SCOPE_KIND_EXECUTOR = 0x04;
const USAGE_WINDOW_KIND_DAILY = 0x01;

async function dimensionDailyWindow(scopeKind: number, subject: PublicKey): Promise<PublicKey> {
    const subjectBytes = publicKeyToBytes(subject);
    const preimage = new Uint8Array(1 + subjectBytes.length);
    preimage[0] = scopeKind;
    preimage.set(subjectBytes, 1);
    const scopeHash = await CryptoUtil.sha256(preimage);
    const [address] = await usageWindowPda(scopeHash, USAGE_WINDOW_KIND_DAILY);
    return address;
}

export async function permissionDailyWindows(subject: PublicKey): Promise<PermissionDailyWindows> {
    const [counterpartyDailyUsageWindow, executorDailyUsageWindow] = await Promise.all([
        dimensionDailyWindow(SCOPE_KIND_COUNTERPARTY, subject),
        dimensionDailyWindow(SCOPE_KIND_EXECUTOR, subject),
    ]);
    return { counterpartyDailyUsageWindow, executorDailyUsageWindow };
}
