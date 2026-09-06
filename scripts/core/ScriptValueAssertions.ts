import { publicKeyToBase58 } from "./PublicKeyUtil.js";

const HEX = "0123456789abcdef";

export function keyString(value: unknown): string {
    return publicKeyToBase58(value);
}

export function bigintValue(value: unknown): bigint {
    return typeof value === "bigint" ? value : BigInt(String(value));
}

export function assertEqual(label: string, actual: unknown, expected: unknown): void {
    if (actual !== expected) throw new Error(`${label}: got ${String(actual)}, expected ${String(expected)}`);
}

export function bytesHex(value: ArrayLike<number>): string {
    let output = "";
    for (let index = 0, length = value.length; index < length; index += 1) {
        const byte = value[index];
        if (!Number.isInteger(byte) || byte < 0 || byte > 255) {
            throw new Error(`bytesHex input at ${index} is not an unsigned byte`);
        }
        output += HEX[(byte >>> 4) & 0x0f] + HEX[byte & 0x0f];
    }
    return output;
}
