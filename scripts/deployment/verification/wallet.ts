import { mkdirSync, mkdtempSync } from "node:fs";
import { resolve } from "node:path";

import { CreateAccountInstruction } from "@solomon-labs/solana-codec";

import type { RuntimeHarness } from "../../../runtime/harness.js";
import { submitTransaction } from "../../../runtime/submit.js";
import { SYSTEM_PROGRAM } from "../../../runtime/wellKnown.js";
import { writeGeneratedSigner } from "../lib/signerFile.js";
import { generateSigner, type E2ESigner } from "../lib/solanaSigner.js";

export async function createPublicVerificationWallet(harness: RuntimeHarness, payer: E2ESigner,
    directory: string, lamports: bigint): Promise<string> {
    const rentFunction = harness.banksClient.getMinimumBalanceForRentExemption;
    if (rentFunction === undefined) throw new Error("runtime cannot calculate wallet rent exemption");
    const minimumRent = await rentFunction.call(harness.banksClient, 0);
    if (lamports <= minimumRent) throw new Error("verification funding must exceed zero-data account rent: " + minimumRent);
    const payerAccount = await harness.banksClient.getAccount(payer.publicKey);
    if (payerAccount === null || payerAccount.lamports <= lamports) {
        throw new Error("fee payer must hold more than " + lamports + " lamports to fund the public integration wallet");
    }
    const tester = generateSigner();
    try {
        if (await harness.banksClient.getAccount(tester.publicKey) !== null) throw new Error("generated public integration wallet already exists");
        mkdirSync(directory, { recursive: true, mode: 0o700 });
        const walletDirectory = mkdtempSync(resolve(directory, "public-integration-"));
        const keypairPath = resolve(walletDirectory, "keypair.json");
        writeGeneratedSigner(keypairPath, tester);
        console.log("public integration wallet: " + tester.publicKey.toBase58());
        console.log("public integration keypair: " + keypairPath);
        await submitTransaction(harness, [new CreateAccountInstruction({
            from: payer.publicKey,
            newAccount: tester.publicKey,
            lamports,
            space: 0,
            owner: SYSTEM_PROGRAM,
        })], [tester], payer);
        const funded = await harness.banksClient.getAccount(tester.publicKey);
        if (funded === null || funded.owner?.equals(SYSTEM_PROGRAM) !== true || funded.lamports !== lamports) {
            throw new Error("public integration wallet funding readback differs from the requested allocation");
        }
        return keypairPath;
    } finally {
        tester.seed.fill(0);
    }
}
