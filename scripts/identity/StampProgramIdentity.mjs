import { workspaceRoot } from "../core/FileSystem.mjs";
import { stampProgramIdentities } from "./ProgramIdentity.mjs";

async function main() {
    const arguments_ = process.argv.slice(2);
    let programKeypairPath = ".devnet/program-keypair.json";
    let faucetKeypairPath = ".devnet/faucet-program-keypair.json";
    const seen = new Set();
    for (let index = 0; index < arguments_.length; index += 2) {
        const flag = arguments_[index];
        if (flag !== "--program-keypair" && flag !== "--faucet-keypair") {
            throw new Error("unknown identity argument: " + flag);
        }
        if (seen.has(flag)) throw new Error("repeated identity argument: " + flag);
        seen.add(flag);
        const value = arguments_[index + 1];
        if (value === undefined || value.startsWith("--") || value.trim().length === 0) {
            throw new Error(flag + " requires a keypair path");
        }
        if (flag === "--program-keypair") programKeypairPath = value;
        else faucetKeypairPath = value;
    }
    const result = await stampProgramIdentities(workspaceRoot, programKeypairPath, faucetKeypairPath);
    console.log("program identity: " + result.programId);
    console.log("faucet identity: " + result.faucetProgramId);
    for (const path of result.changedPaths) console.log("stamped " + path);
}

main().catch((error) => {
    console.error(error instanceof Error ? error.stack ?? error.message : String(error));
    process.exitCode = 1;
});
