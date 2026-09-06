import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { resolve } from "node:path";

import { Base58Util } from "@solomon-labs/base58";
import { PublicKey } from "@solomon-labs/publickey";
import type { Base58 } from "@solomon-labs/types";

import { createDevnetHarness } from "../../runtime/devnetHarness.js";
import { submitTransaction } from "../../runtime/submit.js";
import { TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from "../../runtime/token.js";
import { activeNetwork, DRY_RUN, loadFeePayer, loadSetupAuthority, preflight, printContext } from "./lib/bootstrap.js";
import { inspectCollateralMintAccount } from "./lib/collateralMint.js";
import { sanitizedProcessEnvironment } from "./lib/processEnvironment.js";
import { redactedCommand, redactPath, redactRpcUrl } from "./lib/safeOutput.js";
import { TEST_ASSET_PROFILES } from "./lib/testAssetProfiles.js";

const FAUCET_ARTIFACT_PATH = "target/deploy/chancery_devnet_faucet.so";
const FAUCET_KEYPAIR_PATH = ".devnet/faucet-program-keypair.json";
const FAUCET_AUTHORITY_SEED = new TextEncoder().encode("faucet-authority");
const UNSTAMPED_PROGRAM_ID = "11111111111111111111111111111111";

/** SPL Token / Token-2022 `SetAuthority`, AuthorityType::MintTokens = 0. */
const TOKEN_SET_AUTHORITY = 6;
const AUTHORITY_TYPE_MINT_TOKENS = 0;

function run(command: string, args: string[]): void {
    console.log(`\n$ ${redactedCommand(command, args)}`);
    const result = spawnSync(command, args, { stdio: "inherit", env: sanitizedProcessEnvironment() });
    if (result.error) throw result.error;
    if (result.status !== 0) throw new Error(`${command} failed with exit status ${result.status ?? 1}`);
}

function publicKeyFromAddress(address: string): PublicKey {
    return new PublicKey(Base58Util.decode(address as Base58));
}

/** Build `SetAuthority` for the mint-tokens authority. */
function setMintAuthorityInstruction(
    tokenProgram: PublicKey,
    mint: PublicKey,
    currentAuthority: PublicKey,
    newAuthority: PublicKey,
): { programId: PublicKey; accounts: { pubkey: PublicKey; isSigner: boolean; isWritable: boolean }[]; data: Uint8Array } {
    const data = new Uint8Array(1 + 1 + 1 + 32);
    data[0] = TOKEN_SET_AUTHORITY;
    data[1] = AUTHORITY_TYPE_MINT_TOKENS;
    data[2] = 1; // COption::Some
    data.set(newAuthority.toBytes(), 3);
    return {
        programId: tokenProgram,
        accounts: [
            { pubkey: mint, isSigner: false, isWritable: true },
            { pubkey: currentAuthority, isSigner: true, isWritable: false },
        ],
        data,
    };
}

async function main(): Promise<void> {
    preflight();
    if (DRY_RUN) throw new Error("faucet deployment and handoff require committed state; --dry-run is unsupported");
    const network = activeNetwork();
    const feePayer = loadFeePayer();
    const setupAuthority = loadSetupAuthority();
    const skipDeploy = process.argv.includes("--skip-deploy");

    if (network.faucetProgramId === UNSTAMPED_PROGRAM_ID) {
        throw new Error(
            "faucet identity is unstamped; run "
            + "`solana-keygen new -o .devnet/faucet-program-keypair.json` then `yarn identity`",
        );
    }

    const faucetProgramId = publicKeyFromAddress(network.faucetProgramId);
    const [faucetAuthority] = await PublicKey.findProgramAddress([FAUCET_AUTHORITY_SEED], faucetProgramId);

    printContext("Deploy devnet faucet", {
        rpc: redactRpcUrl(network.rpcUrl),
        faucetProgram: faucetProgramId.toBase58(),
        faucetAuthority: faucetAuthority.toBase58(),
        artifact: FAUCET_ARTIFACT_PATH,
    });

    const harness = await createDevnetHarness({
        commitment: "confirmed", requireProgramdata: false, rpcUrl: network.rpcUrl,
    });

    if (!skipDeploy) {
        if (!existsSync(FAUCET_ARTIFACT_PATH)) {
            throw new Error(`missing ${FAUCET_ARTIFACT_PATH}; run \`cargo xtask build faucet\``);
        }
        const keypairPath = resolve(FAUCET_KEYPAIR_PATH);
        if (!existsSync(keypairPath)) throw new Error(`faucet keypair missing: ${redactPath(keypairPath)}`);
        run("solana", [
            "program", "deploy",
            "--url", network.rpcUrl,
            "--keypair", network.feePayerKeypairPath,
            "--program-id", keypairPath,
            "--upgrade-authority", network.upgradeAuthorityKeypairPath ?? keypairPath,
            FAUCET_ARTIFACT_PATH,
        ]);
    }

    const deployed = await harness.banksClient.getAccount(faucetProgramId);
    if (deployed?.executable !== true) throw new Error("the configured faucet program is not executable");

    const configured = network.collateralMints ?? [];
    const targets: { symbol: string; mint: PublicKey; tokenProgram: PublicKey }[] = [];
    for (const profile of TEST_ASSET_PROFILES) {
        const tokenProgram = profile.tokenProgramKind === "token-2022" ? TOKEN_2022_PROGRAM_ID : TOKEN_PROGRAM_ID;
        if (profile.role === "legacy-migration-source") {
            if (network.legacyUsdvMint === null) continue;
            targets.push({
                symbol: profile.symbol,
                mint: publicKeyFromAddress(network.legacyUsdvMint),
                tokenProgram,
            });
            continue;
        }
        const collateral = configured.find((entry) => entry.symbol === profile.symbol);
        if (collateral === undefined) continue;
        targets.push({ symbol: profile.symbol, mint: publicKeyFromAddress(collateral.mint), tokenProgram });
    }
    if (targets.length === 0) {
        throw new Error("no test assets are configured; run `yarn assets:create` first");
    }

    console.log(`\nhanding mint authority to ${faucetAuthority.toBase58()}`);
    let handed = 0;
    for (const target of targets) {
        const account = await harness.banksClient.getAccount(target.mint);
        if (account === null) throw new Error(`${target.symbol} mint ${target.mint.toBase58()} is absent`);

        if (account.owner?.equals(target.tokenProgram) !== true) throw new Error(`${target.symbol} mint has the wrong token program`);
        const inspected = inspectCollateralMintAccount({ data: account.data, owner: target.tokenProgram });
        if (10_000n * 10n ** BigInt(inspected.decimals) > (1n << 64n) - 1n) {
            throw new Error(`${target.symbol} decimals exceed the faucet capacity`);
        }
        const view = new DataView(account.data.buffer, account.data.byteOffset, account.data.byteLength);
        if (view.getUint32(0, true) !== 1) throw new Error(`${target.symbol} mint authority is already revoked`);
        const currentAuthority = new PublicKey(account.data.subarray(4, 36));
        if (currentAuthority.equals(faucetAuthority)) {
            console.log(`  ${target.symbol.padEnd(12)} already held by the faucet`);
            continue;
        }
        if (!currentAuthority.equals(setupAuthority.publicKey)) {
            throw new Error(
                `${target.symbol} mint authority ${currentAuthority.toBase58()} is neither the setup `
                + "authority nor the faucet; refusing to guess",
            );
        }

        await submitTransaction(
            harness,
            [setMintAuthorityInstruction(target.tokenProgram, target.mint, setupAuthority.publicKey, faucetAuthority)],
            [setupAuthority],
            feePayer,
        );

        const after = await harness.banksClient.getAccount(target.mint);
        if (after === null || after.owner?.equals(target.tokenProgram) !== true || after.data.length < 82) {
            throw new Error(`${target.symbol} mint readback failed after the handover`);
        }
        const afterView = new DataView(after.data.buffer, after.data.byteOffset, after.data.byteLength);
        if (afterView.getUint32(0, true) !== 1) throw new Error(`${target.symbol} mint authority was revoked`);
        const observed = new PublicKey(after.data.subarray(4, 36));
        if (!observed.equals(faucetAuthority)) {
            throw new Error(`${target.symbol} mint authority is ${observed.toBase58()} after the handover`);
        }
        console.log(`  ${target.symbol.padEnd(12)} ${target.mint.toBase58()} -> faucet`);
        handed += 1;
    }

    console.log(`\n${handed} mint authorities handed over, ${targets.length - handed} already held.`);
    console.log("`yarn assets:mint` no longer works. Anyone can now run `yarn faucet:mint`.");
}

main().catch((error: unknown) => {
    console.error(error instanceof Error ? error.stack ?? error.message : String(error));
    process.exitCode = 1;
});
