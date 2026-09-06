// Chancery devnet deployment: loader-v3 deploy of the built ELF with local
// keypairs, then a read-back of the on-chain program against the local
// artifact hash and the expected upgrade authority.
//
// There is no multisig, no ceremony gate, and no release-evidence chain. The
// upgrade authority is whatever keypair `upgrade_authority_keypair_path`
// names, and it stays that way after deployment.

import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

import { resolveNetwork } from "../../config/network.js";
import { updateChanceryDeploymentManifest } from "./lib/deploymentManifest.js";
import { normalizedElfSha256FromFile, verifyLiveProgramArtifact } from "./lib/liveProgram.js";
import { rpcAccountExists, verifyRpcCluster } from "./lib/rpcPreflight.js";
import { sanitizedProcessEnvironment } from "./lib/processEnvironment.js";
import { redactCommandOutput, redactedCommand, redactPath, redactRpcUrl } from "./lib/safeOutput.js";

const SO_PATH = "target/deploy/chancery.so";
const UNSTAMPED_PROGRAM_ID = "11111111111111111111111111111111";

function run(command: string, args: string[]): void {
    console.log(`\n$ ${redactedCommand(command, args)}`);
    const result = spawnSync(command, args, { stdio: "inherit", env: sanitizedProcessEnvironment() });
    if (result.error) throw result.error;
    if (result.status !== 0) throw new Error(`${command} failed with exit status ${result.status ?? 1}`);
}

function capture(command: string, args: string[], redactPathIndices: readonly number[] = []): string {
    console.log(`\n$ ${redactedCommand(command, args, redactPathIndices)}`);
    const result = spawnSync(command, args, { encoding: "utf8", env: sanitizedProcessEnvironment() });
    if (result.error) throw result.error;
    if (result.status !== 0) {
        throw new Error(
            `${command} failed: ${redactCommandOutput((result.stderr ?? result.stdout ?? "").trim(), args, redactPathIndices)}`,
        );
    }
    return result.stdout.trim();
}

function keypairPubkey(path: string): string {
    return capture("solana-keygen", ["pubkey", path], [1]);
}

function fail(message: string): never {
    throw new Error(message);
}

function sha256File(path: string): string {
    return createHash("sha256").update(readFileSync(path)).digest("hex");
}

function declaredProgramId(): string | null {
    try {
        return readFileSync("programs/chancery/src/lib.rs", "utf8").match(/declare_id!\("([^"]+)"\)/)?.[1] ?? null;
    } catch {
        return null;
    }
}

function generatedClientProgramId(): string | null {
    try {
        return readFileSync("clients/ts/src/constants.ts", "utf8")
            .match(/PROGRAM_ID\s*=\s*new PublicKey\("([^"]+)"\)/)?.[1] ?? null;
    } catch {
        return null;
    }
}

export async function deployDevnet(argv: readonly string[]): Promise<void> {
    const dryRun = argv.includes("--dry-run");
    const networkArg = argv.find((value) => !value.startsWith("--"));
    if (networkArg !== undefined && networkArg !== "devnet") {
        fail("usage: Deploy.ts [devnet] [--dry-run] [--config <path>]");
    }
    const net = resolveNetwork("devnet", argv);

    if (net.programId === UNSTAMPED_PROGRAM_ID) {
        fail("program identity is unstamped; run `yarn identity` before deploying");
    }

    const programKeypair = net.programKeypairPath;
    const feePayer = net.feePayerKeypairPath;
    const setupAuthority = net.setupAuthorityKeypairPath;
    if (net.upgradeAuthorityKeypairPath === null) fail("upgrade_authority_keypair_path is unset for devnet");
    const upgradeAuthority = net.upgradeAuthorityKeypairPath;

    console.log("Chancery devnet deploy plan");
    console.log("  network            :", `${net.name} (${net.cluster})`);
    console.log("  rpc                :", redactRpcUrl(net.rpcUrl));
    console.log("  programId          :", net.programId);
    console.log("  program keypair    :", redactPath(programKeypair));
    console.log("  upgrade authority  :", redactPath(upgradeAuthority));
    console.log("  fee payer          :", redactPath(feePayer));
    console.log("  setup authority    :", redactPath(setupAuthority));
    console.log("  artifact           :", SO_PATH);
    console.log("  issued mint        :", net.usdvMint ?? "<unset>");

    for (const [label, path] of [
        ["program", programKeypair],
        ["fee payer", feePayer],
        ["setup authority", setupAuthority],
        ["upgrade authority", upgradeAuthority],
    ] as const) {
        if (!existsSync(path)) fail(`${label} keypair missing: ${redactPath(path)}`);
    }
    if (keypairPubkey(programKeypair) !== net.programId) {
        fail("program keypair does not match the configured devnet programId");
    }
    if (declaredProgramId() !== net.programId || generatedClientProgramId() !== net.programId) {
        fail(`source/generated identity does not match ${net.programId}; run \`yarn identity\``);
    }

    if (dryRun) {
        console.log("\nDry run completed; identity and keypairs check out, no network writes were performed.");
        return;
    }

    if (!existsSync(SO_PATH)) fail(`missing build artifact ${SO_PATH}; run \`yarn build:sbf\``);

    await verifyRpcCluster(net.rpcUrl, "devnet");
    const programExisted = await rpcAccountExists(net.rpcUrl, net.programId);
    console.log(`\n  program account    : ${programExisted ? "exists (upgrade)" : "absent (first deploy)"}`);

    run("solana", [
        "program", "deploy",
        "--url", net.rpcUrl,
        "--keypair", feePayer,
        "--program-id", programKeypair,
        "--upgrade-authority", upgradeAuthority,
        SO_PATH,
    ]);

    const artifactSha256 = sha256File(SO_PATH);
    const initialUpgradeAddress = keypairPubkey(upgradeAuthority);
    const live = verifyLiveProgramArtifact({
        rpcUrl: net.rpcUrl,
        programId: net.programId,
        expectedElfSha256: normalizedElfSha256FromFile(SO_PATH),
        expectedUpgradeAuthority: initialUpgradeAddress,
    });

    updateChanceryDeploymentManifest(net, {
        phase: "deployed",
        deployedAt: new Date().toISOString(),
        loader: "bpf-loader-upgradeable-v3",
        artifact: {
            path: resolve(SO_PATH),
            sha256: artifactSha256,
        },
        addresses: {
            program: net.programId,
            programdata: live.programdataAddress,
            issuedTokenMint: net.usdvMint,
        },
        authorities: {
            upgrade: live.upgradeAuthority,
            feePayer: keypairPubkey(feePayer),
            setup: keypairPubkey(setupAuthority),
        },
        program: live.raw,
        initialized: false,
        operationalStatus: "deployed; settlement not activated",
    });
    console.log(`\nDeployed and read back ${net.programId} on ${net.cluster}.`);
}

async function main(): Promise<void> {
    await deployDevnet(process.argv.slice(2));
}

const invokedPath = process.argv[1];
if (invokedPath !== undefined && pathToFileURL(resolve(invokedPath)).href === import.meta.url) {
    main().catch((error: unknown) => {
        console.error("deploy failed:", error instanceof Error ? error.stack ?? error.message : String(error));
        process.exitCode = 1;
    });
}
