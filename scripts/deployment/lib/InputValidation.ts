import { PublicKey } from "@solomon-labs/publickey";
import type { SolanaAddress } from "@solomon-labs/types";

const SHA256_PATTERN = /^[0-9a-f]{64}$/u;
const DEFAULT_PUBLIC_KEY_ADDRESS = new PublicKey(new Uint8Array(32)).toBase58();

export function assertSha256(fieldName: string, value: unknown): string {
    if (typeof value !== "string" || !SHA256_PATTERN.test(value)) {
        throw new Error(`${fieldName} must be 64 lowercase hexadecimal characters`);
    }
    return value;
}

export function assertSolanaPublicKey(fieldName: string, value: unknown, allowDefault = true): string {
    if (typeof value !== "string" || value.startsWith("<") || value.trim().length === 0) {
        throw new Error(`${fieldName} must be a final Solana public key`);
    }
    let canonicalAddress: string;
    try {
        canonicalAddress = new PublicKey(value as SolanaAddress).toBase58();
    } catch (error) {
        throw new Error(`${fieldName} is not a valid 32-byte Solana public key`, { cause: error });
    }
    if (!allowDefault && canonicalAddress === DEFAULT_PUBLIC_KEY_ADDRESS) {
        throw new Error(`${fieldName} must not be the default address`);
    }
    return canonicalAddress;
}
