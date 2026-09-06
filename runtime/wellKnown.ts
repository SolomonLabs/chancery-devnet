/**
 * Well-known addresses and the key passthrough the deployment scripts use.
 */

import { SYSTEM_PROGRAM_ID_STRING } from "@solomon-labs/constants";
import { PublicKey } from "@solomon-labs/publickey";

export const SYSTEM_PROGRAM = new PublicKey(SYSTEM_PROGRAM_ID_STRING);
export const DEFAULT_PUBLIC_KEY = new PublicKey(new Uint8Array(32));

/** Identity passthrough - kept for source-compatibility with the legacy helper. */
export function toSolanaKey(publicKey: PublicKey): PublicKey {
    return publicKey;
}
