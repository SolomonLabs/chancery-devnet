import { loadOrGenerateSigner, loadSignerFile, writeGeneratedSigner } from "../../deployment/lib/signerFile.js";
import { generateSigner } from "../../deployment/lib/solanaSigner.js";

export function createTesterWallet(path: string, reuse: boolean): string {
    const signer = reuse ? loadOrGenerateSigner(path, "tester") : generateSigner();
    try {
        if (!reuse) writeGeneratedSigner(path, signer);
        return signer.publicKey.toBase58();
    } finally {
        signer.seed.fill(0);
    }
}

export function testerWalletAddress(path: string): string {
    const signer = loadSignerFile(path);
    try {
        return signer.publicKey.toBase58();
    } finally {
        signer.seed.fill(0);
    }
}
