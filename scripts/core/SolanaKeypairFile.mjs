import { parseStrictJson } from "./StrictJson.mjs";

const SOLANA_KEYPAIR_BYTES = 64;
const MAX_KEYPAIR_FILE_BYTES = 4 * 1024;

export function parseSolanaKeypairSource(source, sourceName) {
    const value = parseStrictJson(source, sourceName, { maxBytes: MAX_KEYPAIR_FILE_BYTES, maxDepth: 4 });
    if (!Array.isArray(value) || value.length !== SOLANA_KEYPAIR_BYTES) {
        throw new Error(`${sourceName} must contain exactly ${SOLANA_KEYPAIR_BYTES} byte values`);
    }
    const output = new Uint8Array(SOLANA_KEYPAIR_BYTES);
    for (let index = 0; index < SOLANA_KEYPAIR_BYTES; index += 1) {
        const byte = value[index];
        if (!Number.isInteger(byte) || byte < 0 || byte > 255) {
            throw new Error(`${sourceName}[${index}] must be an integer from 0 through 255`);
        }
        output[index] = byte;
    }
    return output;
}
