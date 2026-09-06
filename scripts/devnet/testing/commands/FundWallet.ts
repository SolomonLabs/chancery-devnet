import { resolveNetwork } from "../../../../config/network.js";
import { createDevnetHarness, requestAirdropAndConfirm } from "../../../../runtime/devnetHarness.js";
import { assertSupportedNodeVersion } from "../../../deployment/lib/processEnvironment.js";
import { loadSignerFile } from "../../../deployment/lib/signerFile.js";
import { parseTesterArguments } from "../arguments.js";
import { fundTesterWallet } from "../funding/fund.js";
import { testerKeypairPath } from "../keypairPath.js";

async function main(): Promise<void> {
    const options = parseTesterArguments(process.argv.slice(2), "funding");
    assertSupportedNodeVersion();
    const network = resolveNetwork("devnet", options.runtimeArguments);
    const keypairPath = testerKeypairPath(options.keypairPath);
    const signer = loadSignerFile(keypairPath);
    const wallet = signer.publicKey;
    signer.seed.fill(0);
    const rpcOptions = { rpcUrl: network.rpcUrl, commitment: "confirmed", requireProgramdata: false } as const;
    const harness = await createDevnetHarness(rpcOptions);
    const balance = await fundTesterWallet({
        banksClient: harness.banksClient,
        requestAirdrop: async (recipient, lamports) => {
            const signature = await requestAirdropAndConfirm(recipient, lamports, rpcOptions);
            console.log("airdrop signature: " + signature);
            return signature;
        },
    }, wallet, options);
    console.log("tester wallet: " + wallet.toBase58());
    console.log("tester lamports: " + balance);
}

main().catch((error: unknown) => {
    console.error(error instanceof Error ? error.stack ?? error.message : String(error));
    process.exitCode = 1;
});
