import { PublicKey } from "@solomon-labs/publickey";
import type { SolanaAddress } from "@solomon-labs/types";

interface PublicKeyBase58Export {
    toBase58(): string;
}

interface PublicKeyByteExport {
    toBytes(): Uint8Array;
}

interface PublicKeyInternalShape {
    _key?: Uint8Array;
}

function hasBase58Export(value: object): value is PublicKeyBase58Export {
    return "toBase58" in value && typeof value.toBase58 === "function";
}

function hasByteExport(value: object): value is PublicKeyByteExport {
    return "toBytes" in value && typeof value.toBytes === "function";
}

function hasInternalKey(value: object): value is PublicKeyInternalShape {
    return "_key" in value;
}

function requirePublicKeyBytes(bytes: Uint8Array): Uint8Array {
    if (bytes.length !== 32) throw new Error(`Expected 32-byte public key, got ${bytes.length}`);
    return bytes;
}

export function publicKeyToBytes(value: unknown): Uint8Array {
    if (typeof value === "string") {
        return new PublicKey(value as SolanaAddress).toBytes();
    }
    if (value instanceof Uint8Array) {
        return requirePublicKeyBytes(new Uint8Array(value));
    }
    if (value !== null && typeof value === "object") {
        const objectValue: object = value;
        if (hasByteExport(objectValue)) return requirePublicKeyBytes(new Uint8Array(objectValue.toBytes()));
        if (hasInternalKey(objectValue)) {
            const internalKey = objectValue._key;
            if (internalKey !== undefined) return requirePublicKeyBytes(new Uint8Array(internalKey));
        }
    }
    throw new Error("Unsupported public key input: expected base58, Uint8Array, or PublicKey-compatible bytes");
}

export function publicKeyToBase58(value: unknown): string {
    if (typeof value === "string") return new PublicKey(value as SolanaAddress).toBase58();
    if (value !== null && typeof value === "object" && !(value instanceof Uint8Array) && hasBase58Export(value)) {
        return value.toBase58();
    }
    return new PublicKey(publicKeyToBytes(value)).toBase58();
}
