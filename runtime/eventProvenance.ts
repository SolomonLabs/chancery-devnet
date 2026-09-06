import { PublicKey } from "@solomon-labs/publickey";
import type { PublicKeyLike } from "@solomon-labs/types";

import { IX, MODULE, PROGRAM_ID } from "../clients/ts/src/constants.js";
import { publicKeyToBytes } from "./publicKey.js";

export interface ChanceryEventInstructionCandidate {
    readonly programId: PublicKeyLike | string;
    readonly accountPublicKeys?: readonly (PublicKeyLike | string)[];
    readonly data: Uint8Array;
}

function publicKeyString(value: PublicKeyLike | string): string {
    return typeof value === "string"
        ? value
        : new PublicKey(publicKeyToBytes(value)).toBase58();
}

export function isCanonicalChanceryEventInstruction(
    candidate: ChanceryEventInstructionCandidate,
    expectedEventAuthority?: PublicKeyLike | string,
): boolean {
    if (publicKeyString(candidate.programId) !== PROGRAM_ID.toBase58()) {
        return false;
    }

    if (candidate.data.length < 10
        || candidate.data[0] !== MODULE.EVENTS_CPI
        || candidate.data[1] !== IX.events_cpi.EMIT
    ) {
        return false;
    }

    if (expectedEventAuthority === undefined) {
        return true;
    }

    const firstAccountPublicKey = candidate.accountPublicKeys?.[0];
    return firstAccountPublicKey !== undefined
        && publicKeyString(firstAccountPublicKey) === publicKeyString(expectedEventAuthority);
}
