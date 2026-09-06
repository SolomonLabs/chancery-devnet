import {
    createHash,
    createPrivateKey,
    createPublicKey,
    sign as cryptoSign,
    verify as cryptoVerify,
    generateKeyPairSync
} from "node:crypto";

const BASE58_ALPHABET = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
const BASE58_INDEX = new Map([...BASE58_ALPHABET].map((character, index) => [character, index]));
const BASE64_URL_ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

function base64UrlEncode(bytes) {
    let output = "";
    for (let i = 0, len = bytes.length; i < len; i += 3) {
        const first = bytes[i];
        const hasSecond = i + 1 < len;
        const hasThird = i + 2 < len;
        const second = hasSecond ? bytes[i + 1] : 0;
        const third = hasThird ? bytes[i + 2] : 0;
        output += BASE64_URL_ALPHABET[first >>> 2];
        output += BASE64_URL_ALPHABET[((first & 0x03) << 4) | (second >>> 4)];
        if (hasSecond) output += BASE64_URL_ALPHABET[((second & 0x0f) << 2) | (third >>> 6)];
        if (hasThird) output += BASE64_URL_ALPHABET[third & 0x3f];
    }
    return output;
}

function base64UrlDecode(value, sourceName) {
    if (typeof value !== "string" || value.length === 0 || !/^[A-Za-z0-9_-]+$/u.test(value)) {
        throw new Error(`${sourceName} must be base64url text`);
    }
    const standard = value.replaceAll("-", "+").replaceAll("_", "/");
    const padded = `${standard}${"=".repeat((4 - (standard.length % 4)) % 4)}`;
    const binary = globalThis.atob(padded);
    const output = new Uint8Array(binary.length);
    for (let index = 0, length = binary.length; index < length; index += 1) {
        output[index] = binary.charCodeAt(index);
    }
    return output;
}

export function decodeBase58(value, sourceName = "base58 value") {
    if (typeof value !== "string" || value.length === 0 || value.length > 256) {
        throw new Error(`${sourceName} must be non-empty bounded base58 text`);
    }
    let numeric = 0n;
    for (let i = 0, len = value.length; i < len; i++) {
        const digit = BASE58_INDEX.get(value[i]);
        if (digit === undefined) throw new Error(`${sourceName} contains a non-base58 character`);
        numeric = numeric * 58n + BigInt(digit);
    }
    const decoded = [];
    while (numeric > 0n) {
        decoded.push(Number(numeric & 0xffn));
        numeric >>= 8n;
    }
    decoded.reverse();
    let leadingZeros = 0;
    while (leadingZeros < value.length && value[leadingZeros] === "1") leadingZeros += 1;
    const output = new Uint8Array(leadingZeros + decoded.length);
    output.set(decoded, leadingZeros);
    return output;
}

export function encodeBase58(bytes, sourceName = "bytes") {
    if (!(bytes instanceof Uint8Array) || bytes.length === 0 || bytes.length > 512) {
        throw new Error(`${sourceName} must be a non-empty bounded Uint8Array`);
    }
    let numeric = 0n;
    for (let index = 0, length = bytes.length; index < length; index += 1) {
        numeric = numeric * 256n + BigInt(bytes[index]);
    }
    let encoded = "";
    while (numeric > 0n) {
        const digit = Number(numeric % 58n);
        encoded = `${BASE58_ALPHABET[digit]}${encoded}`;
        numeric /= 58n;
    }
    let leadingZeros = 0;
    while (leadingZeros < bytes.length && bytes[leadingZeros] === 0) leadingZeros += 1;
    return `${"1".repeat(leadingZeros)}${encoded}`;
}

function privateKeyFromSolanaKeypair(keypairBytes) {
    if (!(keypairBytes instanceof Uint8Array) || keypairBytes.length !== 64) {
        throw new Error("Solana ed25519 keypair must contain exactly 64 bytes");
    }
    const seed = keypairBytes.slice(0, 32);
    const embeddedPublicKey = keypairBytes.slice(32, 64);
    try {
        const privateKey = createPrivateKey({
            key: {
                kty: "OKP",
                crv: "Ed25519",
                d: base64UrlEncode(seed),
                x: base64UrlEncode(embeddedPublicKey)
            },
            format: "jwk"
        });
        const publicJwk = createPublicKey(privateKey).export({ format: "jwk" });
        if (typeof publicJwk.x !== "string" || publicJwk.x !== base64UrlEncode(embeddedPublicKey)) {
            throw new Error("Solana keypair public key does not match its private seed");
        }
        return { privateKey, embeddedPublicKey };
    } finally {
        seed.fill(0);
    }
}

export function generateSolanaEd25519KeypairBytes() {
    const { privateKey } = generateKeyPairSync("ed25519");
    const privateJwk = privateKey.export({ format: "jwk" });
    if (typeof privateJwk.d !== "string" || typeof privateJwk.x !== "string") {
        throw new Error("generated Ed25519 keypair cannot be exported as raw key material");
    }
    const seed = base64UrlDecode(privateJwk.d, "generated Ed25519 private seed");
    const publicKey = base64UrlDecode(privateJwk.x, "generated Ed25519 public key");
    if (seed.length !== 32 || publicKey.length !== 32) {
        throw new Error("generated Ed25519 keypair has an unexpected raw length");
    }
    const output = new Uint8Array(64);
    output.set(seed, 0);
    output.set(publicKey, 32);
    seed.fill(0);
    return output;
}

export function ed25519PublicKeyFromSolanaKeypair(keypairBytes) {
    const material = privateKeyFromSolanaKeypair(keypairBytes);
    return encodeBase58(material.embeddedPublicKey, "ed25519 public key");
}

export function signEd25519Base58(keypairBytes, message) {
    if (!(message instanceof Uint8Array)) throw new Error("ed25519 message must be a Uint8Array");
    const material = privateKeyFromSolanaKeypair(keypairBytes);
    return {
        publicKeyBase58: encodeBase58(material.embeddedPublicKey, "ed25519 public key"),
        signatureBase58: encodeBase58(
            new Uint8Array(cryptoSign(null, message, material.privateKey)),
            "ed25519 signature"
        )
    };
}

function lengthPrefix(byteLength) {
    if (!Number.isSafeInteger(byteLength) || byteLength < 0 || byteLength > 0xffff_ffff) {
        throw new Error("ceremony field exceeds the 32-bit serialization limit");
    }
    const output = new Uint8Array(4);
    new DataView(output.buffer).setUint32(0, byteLength, false);
    return output;
}

export function ceremonyDigest(domain, fields) {
    const encoder = new TextEncoder();
    const hash = createHash("sha256");
    const serializationDomain = encoder.encode("chancery:ceremony-digest:v2");
    hash.update(lengthPrefix(serializationDomain.length));
    hash.update(serializationDomain);
    const allFields = [domain, ...fields];
    hash.update(lengthPrefix(allFields.length));
    for (let i = 0, len = allFields.length; i < len; i++) {
        const encoded = encoder.encode(allFields[i]);
        hash.update(lengthPrefix(encoded.length));
        hash.update(encoded);
    }
    return new Uint8Array(hash.digest());
}

export function verifyEd25519Base58(publicKeyBase58, message, signatureBase58) {
    const rawKey = decodeBase58(publicKeyBase58, "ed25519 public key");
    const signature = decodeBase58(signatureBase58, "ed25519 signature");
    if (rawKey.length !== 32) throw new Error("ed25519 public key must decode to 32 bytes");
    if (signature.length !== 64) throw new Error("ed25519 signature must decode to 64 bytes");
    const key = createPublicKey({
        key: { kty: "OKP", crv: "Ed25519", x: base64UrlEncode(rawKey) },
        format: "jwk"
    });
    return cryptoVerify(null, message, key, signature);
}
