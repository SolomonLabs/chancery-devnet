import { assertSupportedNodeVersion } from "../../../deployment/lib/processEnvironment.js";
import { parseTesterArguments } from "../arguments.js";
import { testerKeypairPath } from "../keypairPath.js";
import { createTesterWallet } from "../wallet.js";

function main(): void {
    const options = parseTesterArguments(process.argv.slice(2), "wallet");
    assertSupportedNodeVersion();
    const keypairPath = testerKeypairPath(options.keypairPath);
    console.log("tester wallet: " + createTesterWallet(keypairPath, options.reuseWallet));
    console.log("tester keypair: " + keypairPath);
}

try {
    main();
} catch (error: unknown) {
    console.error(error instanceof Error ? error.stack ?? error.message : String(error));
    process.exitCode = 1;
}
