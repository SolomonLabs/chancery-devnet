import { createPrivateKey, createPublicKey, randomBytes } from "node:crypto";

import { PublicKey } from "@solomon-labs/publickey";

export interface SolanaSeedSigner {
    publicKey: PublicKey;
    seed: Uint8Array;
}

export type E2ESigner = SolanaSeedSigner;

function ed25519SeedToPkcs8Der(seed: Uint8Array): Uint8Array {
    const output = new Uint8Array(48);
    output[0] = 0x30;
    output[1] = 0x2e;
    output[2] = 0x02;
    output[3] = 0x01;
    output[4] = 0x00;
    output[5] = 0x30;
    output[6] = 0x05;
    output[7] = 0x06;
    output[8] = 0x03;
    output[9] = 0x2b;
    output[10] = 0x65;
    output[11] = 0x70;
    output[12] = 0x04;
    output[13] = 0x22;
    output[14] = 0x04;
    output[15] = 0x20;
    output.set(seed, 16);
    return output;
}

function publicKeyFromSeed(seed: Uint8Array): PublicKey {
    const privateKey = createPrivateKey({
        key: ed25519SeedToPkcs8Der(seed) as unknown as string,
        format: "der",
        type: "pkcs8",
    });
    const publicKey = createPublicKey(privateKey);
    const encodedPublicKey = new Uint8Array(publicKey.export({ format: "der", type: "spki" }));
    return new PublicKey(encodedPublicKey.subarray(encodedPublicKey.length - 32));
}

export function signerFromSeed(seed: Uint8Array): SolanaSeedSigner {
    if (seed.length !== 32) {
        throw new Error(`signerFromSeed requires 32 bytes, got ${seed.length}`);
    }
    const copiedSeed = new Uint8Array(seed);
    return { seed: copiedSeed, publicKey: publicKeyFromSeed(copiedSeed) };
}

export function signerFromSolanaKeypairBytes(keypairBytes: Uint8Array): SolanaSeedSigner {
    if (keypairBytes.length !== 64) {
        throw new Error(`Solana keypair must contain exactly 64 bytes, got ${keypairBytes.length}`);
    }
    const signer = signerFromSeed(keypairBytes.subarray(0, 32));
    const expectedPublicKey = keypairBytes.subarray(32, 64);
    const actualPublicKey = signer.publicKey.toBytes();
    for (let byteIndex = 0, len = expectedPublicKey.length; byteIndex < len; byteIndex++) {
        if (expectedPublicKey[byteIndex] !== actualPublicKey[byteIndex]) {
            signer.seed.fill(0);
            throw new Error("Solana keypair public key does not match private seed");
        }
    }
    return signer;
}

export function generateSigner(): SolanaSeedSigner {
    const seed = new Uint8Array(randomBytes(32));
    try {
        return signerFromSeed(seed);
    } finally {
        seed.fill(0);
    }
}
