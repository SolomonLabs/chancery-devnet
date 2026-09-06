
import { PublicKey } from "@solomon-labs/publickey";
import type { BU64, PublicKeyLike } from "@solomon-labs/types";

import { bigintValue } from "../core/ScriptValueAssertions.js";
import { ChanceryConfig } from "../../clients/ts/src/accounts/ChanceryConfig.js";
import { IssuedTokenControl } from "../../clients/ts/src/accounts/IssuedTokenControl.js";
import { ModuleActivationState } from "../../clients/ts/src/accounts/ModuleActivationState.js";
import { MODULE, MODULE_STATUS } from "../../clients/ts/src/constants.js";
import {
    InitializeIssuedTokenControlInstruction,
    InitializeModuleActivationStateInstruction,
} from "../../clients/ts/src/instructions/index.js";
import { SYSTEM_PROGRAM, toSolanaKey } from "../../runtime/wellKnown.js";
import {
    TOKEN_2022_ALLOWED_RESERVED_ACCOUNT_EXTENSION_MASK,
    TOKEN_2022_ALLOWED_RESERVED_MINT_EXTENSION_MASK,
    TOKEN_2022_PROGRAM_ID,
} from "../../runtime/token.js";
import { createDevnetHarness } from "../../runtime/devnetHarness.js";
import { submitTransaction, type E2ESigner } from "../../runtime/submit.js";
import { publicKeyToBytes } from "../core/PublicKeyUtil.js";
import {
    chanceryConfigPda,
    eventAuthorityPda,
    issuedTokenControlPda,
    moduleActivationStatePda,
} from "../../runtime/pdas.js";
import {
    DRY_RUN,
    activeNetwork,
    issuedTokenMint as configuredIssuedTokenMint,
    loadFeePayer,
    loadSetupAuthority,
    preflight,
    printContext,
} from "./lib/bootstrap.js";

function configuredObservationAgeSlots(isProduction: boolean): bigint {
    const value = activeNetwork().maxExtensionObservationAgeSlots;
    // Approved trusted-mint launch posture: observations of the five
    // provider-controlled mints never expire (age 0), matching the bootstrap
    // plan, runtime config, and the whole evidence chain. A nonzero mainnet
    // age would silently reintroduce a mandatory always-on refresh runner.
    if (isProduction && value !== 0n) {
        throw new Error("mainnet requires max_extension_observation_age_slots = 0 (approved non-expiring trusted-mint posture)");
    }
    return value;
}

function publicKeyString(value: unknown): string {
    if (value !== null && typeof value === "object" && "toBase58" in value) {
        return String((value as { toBase58(): unknown }).toBase58());
    }
    return new PublicKey(publicKeyToBytes(value as PublicKeyLike)).toBase58();
}

function governanceSigner(onChainAddress: string): E2ESigner {
    const setup = loadSetupAuthority();
    if (setup.publicKey.toBase58() === onChainAddress) return setup;
    throw new Error(`on-chain governance ${onChainAddress} is neither the configured setup signer nor the approved final governance address`);
}

function verifyActivation(account: { data: Uint8Array }, address: PublicKey, production: boolean): void {
    const decoded = ModuleActivationState.decode(account.data, address);
    if (production && decoded.moduleStatuses[MODULE.CROSS_CHAIN] !== MODULE_STATUS.DISABLED) {
        throw new Error("cross-chain module must remain DISABLED at production launch");
    }
}

function verifyIssuedControl(
    account: { data: Uint8Array },
    address: PublicKey,
    issuedMint: PublicKey,
    maxAge: bigint,
): void {
    const decoded = IssuedTokenControl.decode(account.data, address);
    if (publicKeyString(decoded.issuedTokenMint) !== issuedMint.toBase58()) {
        throw new Error("IssuedTokenControl mint differs from selected network configuration");
    }
    if (publicKeyString(decoded.issuedTokenProgram) !== TOKEN_2022_PROGRAM_ID.toBase58()) {
        throw new Error("IssuedTokenControl token program is not Token-2022");
    }
    const expectedMint = TOKEN_2022_ALLOWED_RESERVED_MINT_EXTENSION_MASK;
    const expectedAccount = TOKEN_2022_ALLOWED_RESERVED_ACCOUNT_EXTENSION_MASK;
    for (let index = 0; index < 2; index++) {
        if (bigintValue(decoded.reservedMintExtensionMask[index]) !== expectedMint[index]) {
            throw new Error(`IssuedTokenControl reserved mint extension mask[${index}] differs from the launch policy`);
        }
        if (bigintValue(decoded.reservedAccountExtensionMask[index]) !== expectedAccount[index]) {
            throw new Error(`IssuedTokenControl reserved account extension mask[${index}] differs from the launch policy`);
        }
    }
    if (bigintValue(decoded.maxExtensionObservationAgeSlots) !== maxAge) {
        throw new Error("IssuedTokenControl observation age differs from the approved configuration");
    }
}

async function main(): Promise<void> {
    preflight();
    const network = activeNetwork();
    const production = false;
    const issuedTokenMint = configuredIssuedTokenMint();
    const maxExtensionObservationAgeSlots = configuredObservationAgeSlots(production);
    const feePayer = loadFeePayer();

    const context = await createDevnetHarness({
        commitment: "finalized",
        requireProgramdata: false,
        rpcUrl: network.rpcUrl,
    });
    const [[configKey], [eventKey], [activationKey], [controlKey]] = await Promise.all([
        chanceryConfigPda(), eventAuthorityPda(), moduleActivationStatePda(), issuedTokenControlPda(),
    ]);
    const chanceryConfig = toSolanaKey(configKey);
    const eventAuthority = toSolanaKey(eventKey);
    const moduleActivationState = toSolanaKey(activationKey);
    const issuedTokenControl = toSolanaKey(controlKey);
    const configAccount = await context.banksClient.getAccount(chanceryConfig);
    if (configAccount === null) throw new Error("ChanceryConfig is not initialized; run initialization first");
    const decodedConfig = ChanceryConfig.decode(configAccount.data, chanceryConfig);
    if (publicKeyString(decodedConfig.issuedTokenMint) !== issuedTokenMint.toBase58()) {
        throw new Error("configured issued-token mint does not match ChanceryConfig");
    }

    let activationAccount = await context.banksClient.getAccount(moduleActivationState);
    let controlAccount = await context.banksClient.getAccount(issuedTokenControl);
    printContext("configure Chancery", {
        governance: publicKeyString(decodedConfig.governanceAuthority),
        issuedMint: issuedTokenMint.toBase58(),
        observationAgeSlots: maxExtensionObservationAgeSlots.toString(),
    });

    if (activationAccount !== null) verifyActivation(activationAccount, moduleActivationState, production);
    if (controlAccount !== null) verifyIssuedControl(controlAccount, issuedTokenControl, issuedTokenMint, maxExtensionObservationAgeSlots);
    if (activationAccount !== null && controlAccount !== null) {
        console.log("ModuleActivationState and IssuedTokenControl already exist and match the approved configuration.");
        return;
    }

    const governance = governanceSigner(publicKeyString(decodedConfig.governanceAuthority));
    if (activationAccount === null) {
        await submitTransaction(context, [new InitializeModuleActivationStateInstruction({
            activation: moduleActivationState,
            chanceryConfig,
            eventAuthority,
            payer: feePayer.publicKey,
            governanceAuthority: governance.publicKey,
            system: SYSTEM_PROGRAM,
        })], [governance], feePayer, { simulateOnly: DRY_RUN });
        if (DRY_RUN) {
            console.log("ModuleActivationState initialization simulation succeeded; IssuedTokenControl requires the committed activation write.");
            return;
        }
        activationAccount = await context.banksClient.getAccount(moduleActivationState);
        if (activationAccount === null) throw new Error("ModuleActivationState was not created");
        verifyActivation(activationAccount, moduleActivationState, production);
    }

    if (controlAccount === null) {
        await submitTransaction(context, [new InitializeIssuedTokenControlInstruction({
            issuedTokenControl,
            chanceryConfig,
            eventAuthority,
            payer: feePayer.publicKey,
            governanceAuthority: governance.publicKey,
            issuedTokenMint,
            issuedTokenProgram: TOKEN_2022_PROGRAM_ID,
            reservedMintExtensionMask: [
                TOKEN_2022_ALLOWED_RESERVED_MINT_EXTENSION_MASK[0] as BU64,
                TOKEN_2022_ALLOWED_RESERVED_MINT_EXTENSION_MASK[1] as BU64,
            ],
            reservedAccountExtensionMask: [
                TOKEN_2022_ALLOWED_RESERVED_ACCOUNT_EXTENSION_MASK[0] as BU64,
                TOKEN_2022_ALLOWED_RESERVED_ACCOUNT_EXTENSION_MASK[1] as BU64,
            ],
            maxExtensionObservationAgeSlots: maxExtensionObservationAgeSlots as BU64,
        })], [governance], feePayer, { simulateOnly: DRY_RUN });
        if (DRY_RUN) {
            console.log("IssuedTokenControl initialization simulation succeeded.");
            return;
        }
        controlAccount = await context.banksClient.getAccount(issuedTokenControl);
        if (controlAccount === null) throw new Error("IssuedTokenControl was not created");
        verifyIssuedControl(controlAccount, issuedTokenControl, issuedTokenMint, maxExtensionObservationAgeSlots);
    }
    console.log("Configuration complete. Settlement remains unavailable until assets and separate economic records are reviewed and installed.");
}

main().catch((error: unknown) => {
    console.error(error instanceof Error ? error.stack ?? error.message : String(error));
    process.exitCode = 1;
});
