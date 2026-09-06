import { existsSync } from "node:fs";

import { resolveNetwork, resolveWorkspacePath } from "../../config/network.js";
import { parseDeploymentArguments } from "./flow/arguments.js";
import { assertSupportedNodeVersion } from "./lib/processEnvironment.js";
import { verifyRpcCluster } from "./lib/rpcPreflight.js";
import { loadOrGenerateSigner } from "./lib/signerFile.js";
import type { E2ESigner } from "./lib/solanaSigner.js";

const UNSTAMPED_PROGRAM_ID = "11111111111111111111111111111111";

function prepareSigner(path: string, label: string, expected: string | null = null): E2ESigner {
    if (expected !== null && expected !== UNSTAMPED_PROGRAM_ID && !existsSync(path)) {
        throw new Error(label + " keypair is missing for configured identity " + expected);
    }
    const signer = loadOrGenerateSigner(path, label);
    if (expected !== null && expected !== UNSTAMPED_PROGRAM_ID && signer.publicKey.toBase58() !== expected) {
        throw new Error(label + " keypair differs from its configured identity");
    }
    console.log(label + ": " + signer.publicKey.toBase58());
    return signer;
}

async function main(): Promise<void> {
    const options = parseDeploymentArguments(process.argv.slice(2));
    assertSupportedNodeVersion();
    const network = resolveNetwork("devnet", options.runtimeArguments);
    if (network.upgradeAuthorityKeypairPath === null) throw new Error("upgrade_authority_keypair_path must be configured");
    if (network.issuedMintKeypairPath === null) throw new Error("issued_mint_keypair_path must be configured for the unified deployment");
    const program = prepareSigner(network.programKeypairPath, "Chancery program", network.programId);
    const faucet = prepareSigner(resolveWorkspacePath(".devnet/faucet-program-keypair.json"), "faucet program", network.faucetProgramId);
    const mint = prepareSigner(network.issuedMintKeypairPath, "issued mint", network.usdvMint);
    const upgrade = prepareSigner(network.upgradeAuthorityKeypairPath, "upgrade authority");
    const feePayer = prepareSigner(network.feePayerKeypairPath, "fee payer");
    const setup = prepareSigner(network.setupAuthorityKeypairPath, "setup authority");
    const identities = [program, faucet, mint].map((signer) => signer.publicKey.toBase58());
    if (new Set(identities).size !== identities.length) throw new Error("program and issued-mint keypairs must have distinct addresses");
    for (const signer of [upgrade, feePayer, setup]) {
        if (identities.includes(signer.publicKey.toBase58())) throw new Error("deployment wallets must differ from program and mint accounts");
    }
    await verifyRpcCluster(network.rpcUrl, "devnet");
    console.log("Deployment identities prepared; fund the fee payer and setup authority for account rent and transaction fees.");
}

main().catch((error: unknown) => {
    console.error(error instanceof Error ? error.stack ?? error.message : String(error));
    process.exitCode = 1;
});
