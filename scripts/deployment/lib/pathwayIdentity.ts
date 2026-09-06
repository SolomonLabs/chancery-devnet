import { CryptoUtil } from "@solomon-labs/crypto";
import { PublicKey } from "@solomon-labs/publickey";

import type { ChanceryCollateralSymbol } from "../../../config/network.js";
import { publicKeyToBytes } from "../../../runtime/publicKey.js";

export const DIRECT_PATHWAY_ID = (() => {
    const identifier = new Uint8Array(32);
    identifier.set(new TextEncoder().encode("chancery-direct-v1"));
    return identifier;
})();

export function parsePathwayId(value: string): Uint8Array {
    if (!/^[0-9a-fA-F]{64}$/u.test(value) || /^0{64}$/u.test(value)) {
        throw new Error("pathway id must be a nonzero 32-byte hexadecimal identifier");
    }
    return Uint8Array.from(value.match(/../gu)!, (byte) => Number.parseInt(byte, 16));
}

export async function defaultDirectPathwayId(symbol: ChanceryCollateralSymbol, assetMint: PublicKey,
    issuedMint: PublicKey, principal?: PublicKey): Promise<Uint8Array> {
    if (symbol === "USDC" && principal === undefined) return Uint8Array.from(DIRECT_PATHWAY_ID);
    const domain = new TextEncoder().encode("chancery-devnet:direct-pathway:v1");
    const data = new Uint8Array(domain.length + 96);
    data.set(domain);
    data.set(publicKeyToBytes(assetMint), domain.length);
    data.set(publicKeyToBytes(issuedMint), domain.length + 32);
    if (principal !== undefined) data.set(publicKeyToBytes(principal), domain.length + 64);
    return CryptoUtil.sha256(data);
}
