/// <reference types="node" />

import { PublicKey } from "@solomon-labs/publickey";
import type { SolanaAddress } from "@solomon-labs/types";

import { keyString } from "../core/ScriptValueAssertions.js";
import { ChanceryConfig } from "../../clients/ts/src/accounts/ChanceryConfig.js";
import { PROGRAM_ID } from "../../clients/ts/src/constants.js";
import { InitializeChanceryInstruction } from "../../clients/ts/src/instructions/index.js";
import { DEFAULT_PUBLIC_KEY, SYSTEM_PROGRAM, toSolanaKey } from "../../runtime/wellKnown.js";
import { TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from "../../runtime/token.js";
import type { E2ESigner } from "../../runtime/submit.js";
import { createDevnetHarness } from "../../runtime/devnetHarness.js";
import { submitTransaction } from "../../runtime/submit.js";
import {
    chanceryConfigPda,
    closeMintAuthorityPda,
    confidentialTransferAuthorityPda,
    defaultAccountStateAuthorityPda,
    eventAuthorityPda,
    freezeAuthorityPda,
    metadataPointerAuthorityPda,
    metadataUpdateAuthorityPda,
    mintAuthorityPda,
    pauseAuthorityPda,
    pauseStatePda,
    permanentDelegateAuthorityPda,
    transferHookAuthorityPda
} from "../../runtime/pdas.js";
import {
    DRY_RUN,
    activeNetwork,
    issuedTokenMint,
    legacyTokenMint,
    loadFeePayer,
    loadSetupAuthority,
    loadUpgradeAuthority,
    preflight,
    printContext
} from "./lib/bootstrap.js";
import { updateChanceryDeploymentManifest } from "./lib/deploymentManifest.js";

const PROGRAMDATA_UPGRADE_AUTHORITY_OPTION_OFFSET = 12;
const PROGRAMDATA_UPGRADE_AUTHORITY_OFFSET = 13;
const PROGRAMDATA_MIN_LENGTH = 45;

function assertKey(label: string, actual: unknown, expected: PublicKey): void {
    const actualString = keyString(actual);
    if (actualString !== expected.toBase58()) {
        throw new Error(`${label} mismatch: got ${actualString}, expected ${expected.toBase58()}`);
    }
}

function uniqueAdditionalSigners(payer: E2ESigner, signers: E2ESigner[]): E2ESigner[] {
    const seen = new Set([payer.publicKey.toBase58()]);
    return signers.filter((signer) => {
        const key = signer.publicKey.toBase58();
        if (seen.has(key)) return false;
        seen.add(key);
        return true;
    });
}

async function main(): Promise<void> {
    preflight();
    const net = activeNetwork();
    const payer = loadFeePayer();
    const issuedMint = issuedTokenMint();
    const legacyMint = legacyTokenMint();

    printContext("initialize Chancery", {
        payer: payer.publicKey.toBase58(),
        governance: "loaded only if initialization is required",
        upgrade: "resolved from live ProgramData",
        issuedMint: issuedMint.toBase58(),
        legacyMint: legacyMint.toBase58()
    });

    const ctx = await createDevnetHarness({
        commitment: "finalized",
        requireProgramdata: true,
        rpcUrl: net.rpcUrl,
    });
    if (ctx.programdataAddress === undefined) {
        throw new Error("could not resolve the deployed Chancery ProgramData account");
    }

    // Give a clear preflight error before simulation. The on-chain instruction
    // independently performs this same Program -> ProgramData -> authority check.
    const programdataAccount = await ctx.banksClient.getAccount(ctx.programdataAddress);
    if (programdataAccount === null || programdataAccount.data.length < PROGRAMDATA_MIN_LENGTH) {
        throw new Error("deployed Chancery ProgramData account is missing or malformed");
    }
    if (programdataAccount.data[PROGRAMDATA_UPGRADE_AUTHORITY_OPTION_OFFSET] !== 1) {
        throw new Error("Chancery upgrade authority has been revoked; initialize_chancery can no longer run");
    }
    const onChainUpgradeAuthority = new PublicKey(
        programdataAccount.data.subarray(
            PROGRAMDATA_UPGRADE_AUTHORITY_OFFSET,
            PROGRAMDATA_UPGRADE_AUTHORITY_OFFSET + 32
        )
    );

    const [
        [chanceryConfigKey],
        [eventAuthorityKey],
        [pauseStateKey],
        [mintAuthorityKey],
        [freezeAuthorityKey],
        [closeMintAuthorityKey],
        [transferHookAuthorityKey],
        [confidentialTransferAuthorityKey],
        [permanentDelegateAuthorityKey],
        [pauseAuthorityKey],
        [metadataPointerAuthorityKey],
        [metadataUpdateAuthorityKey],
        [defaultAccountStateAuthorityKey]
    ] = await Promise.all([
        chanceryConfigPda(),
        eventAuthorityPda(),
        pauseStatePda(),
        mintAuthorityPda(),
        freezeAuthorityPda(),
        closeMintAuthorityPda(),
        transferHookAuthorityPda(),
        confidentialTransferAuthorityPda(),
        permanentDelegateAuthorityPda(),
        pauseAuthorityPda(),
        metadataPointerAuthorityPda(),
        metadataUpdateAuthorityPda(),
        defaultAccountStateAuthorityPda()
    ]);
    const chanceryConfig = toSolanaKey(chanceryConfigKey);
    const eventAuthority = toSolanaKey(eventAuthorityKey);
    const pauseState = toSolanaKey(pauseStateKey);
    const expectedMintAuthority = toSolanaKey(mintAuthorityKey);
    const expectedFreezeAuthority = toSolanaKey(freezeAuthorityKey);

    let initializedNow = false;
    let initializationGovernance: E2ESigner | null = null;
    const existing = await ctx.banksClient.getAccount(chanceryConfig);
    if (existing === null) {
        const governance = loadSetupAuthority();
        initializationGovernance = governance;
        const upgradeAuthority = loadUpgradeAuthority();
        assertKey("ProgramData upgrade authority", upgradeAuthority.publicKey, onChainUpgradeAuthority);
        const instruction = new InitializeChanceryInstruction({
            chanceryConfig,
            eventAuthority,
            payer: payer.publicKey,
            governanceAuthority: governance.publicKey,
            systemProgram: SYSTEM_PROGRAM,
            pauseState,
            programAccount: PROGRAM_ID,
            programdataAccount: ctx.programdataAddress,
            upgradeAuthority: upgradeAuthority.publicKey,
            issuedTokenMint: issuedMint,
            issuedTokenProgram: TOKEN_2022_PROGRAM_ID,
            legacyTokenMint: legacyMint,
            legacyTokenProgram: legacyMint.equals(DEFAULT_PUBLIC_KEY) ? DEFAULT_PUBLIC_KEY : TOKEN_PROGRAM_ID
        });
        await submitTransaction(
            ctx,
            [instruction],
            uniqueAdditionalSigners(payer, [governance, upgradeAuthority]),
            payer,
            { simulateOnly: DRY_RUN }
        );
        if (DRY_RUN) {
            console.log("initialize_chancery simulation succeeded; nothing was broadcast");
            return;
        }
        initializedNow = true;
    }

    const finalizedConfigAccount = await ctx.banksClient.getAccount(chanceryConfig);
    if (finalizedConfigAccount === null) throw new Error("finalized ChanceryConfig account was not found");
    if (finalizedConfigAccount.owner !== undefined) {
        assertKey("ChanceryConfig owner", finalizedConfigAccount.owner, PROGRAM_ID);
    }
    const decoded = ChanceryConfig.decode(finalizedConfigAccount.data, chanceryConfig);
    if ((BigInt(decoded.statusFlags) & 1n) === 0n) throw new Error("ChanceryConfig is not marked initialized");
    assertKey("issued token mint", decoded.issuedTokenMint, issuedMint);
    assertKey("issued token program", decoded.issuedTokenProgram, TOKEN_2022_PROGRAM_ID);
    assertKey("legacy token mint", decoded.legacyTokenMint, legacyMint);
    assertKey("legacy token program", decoded.legacyTokenProgram, TOKEN_PROGRAM_ID);
    assertKey("mint authority PDA", decoded.mintAuthorityPda, expectedMintAuthority);
    assertKey("freeze authority PDA", decoded.freezeAuthorityPda, expectedFreezeAuthority);
    if (initializedNow) {
        if (initializationGovernance === null) throw new Error("initialization governance signer was not retained");
        assertKey("governance authority", decoded.governanceAuthority, initializationGovernance.publicKey);
        assertKey("operations authority", decoded.operationsAuthority, initializationGovernance.publicKey);
        assertKey("emergency authority", decoded.emergencyAuthority, initializationGovernance.publicKey);
        assertKey("enforcement authority", decoded.enforcementAuthority, initializationGovernance.publicKey);
        assertKey("insurance authority", decoded.insuranceAdminAuthority, initializationGovernance.publicKey);
    }

    const finalizedPauseState = await ctx.banksClient.getAccount(pauseState);
    if (finalizedPauseState === null) throw new Error("finalized PauseState account was not found");
    if (finalizedPauseState.owner !== undefined) assertKey("PauseState owner", finalizedPauseState.owner, PROGRAM_ID);

    const clock = await ctx.banksClient.getClock();
    updateChanceryDeploymentManifest(net, {
        phase: "initialized",
        initialized: true,
        initializedAt: new Date().toISOString(),
        initializedAtSlot: clock.slot.toString(),
        addresses: {
            program: PROGRAM_ID.toBase58(),
            programdata: ctx.programdataAddress.toBase58(),
            chanceryConfig: chanceryConfig.toBase58(),
            pauseState: pauseState.toBase58(),
            issuedTokenMint: issuedMint.toBase58(),
            issuedTokenProgram: TOKEN_2022_PROGRAM_ID.toBase58(),
            legacyTokenMint: legacyMint.toBase58(),
            legacyTokenProgram: legacyMint.equals(DEFAULT_PUBLIC_KEY)
                ? DEFAULT_PUBLIC_KEY.toBase58()
                : TOKEN_PROGRAM_ID.toBase58(),
            mintAuthorityPda: expectedMintAuthority.toBase58(),
            freezeAuthorityPda: expectedFreezeAuthority.toBase58(),
            closeMintAuthorityPda: closeMintAuthorityKey.toBase58(),
            transferHookAuthorityPda: transferHookAuthorityKey.toBase58(),
            confidentialTransferAuthorityPda: confidentialTransferAuthorityKey.toBase58(),
            permanentDelegateAuthorityPda: permanentDelegateAuthorityKey.toBase58(),
            pauseAuthorityPda: pauseAuthorityKey.toBase58(),
            metadataPointerAuthorityPda: metadataPointerAuthorityKey.toBase58(),
            metadataUpdateAuthorityPda: metadataUpdateAuthorityKey.toBase58(),
            defaultAccountStateAuthorityPda: defaultAccountStateAuthorityKey.toBase58()
        },
        authorities: {
            upgrade: onChainUpgradeAuthority.toBase58(),
            governance: keyString(decoded.governanceAuthority),
            operations: keyString(decoded.operationsAuthority),
            emergency: keyString(decoded.emergencyAuthority),
            enforcement: keyString(decoded.enforcementAuthority),
            insuranceAdmin: keyString(decoded.insuranceAdminAuthority),
            feePayer: payer.publicKey.toBase58()
        },
        chanceryConfig: {
            version: Number(decoded.version),
            statusFlags: BigInt(decoded.statusFlags).toString(),
            eventSequenceNonce: BigInt(decoded.eventSequenceNonce).toString(),
            eventAuthorityBump: Number(decoded.eventAuthorityBump)
        }
    });

    console.log(
        initializedNow
            ? "Chancery initialized and finalized-state verified"
            : "Chancery already initialized; finalized-state verified"
    );
    console.log(`  ChanceryConfig ${chanceryConfig.toBase58()}`);
    console.log(`  PauseState     ${pauseState.toBase58()}`);
}

main().catch((error: unknown) => {
    console.error(error instanceof Error ? (error.stack ?? error.message) : String(error));
    process.exitCode = 1;
});
