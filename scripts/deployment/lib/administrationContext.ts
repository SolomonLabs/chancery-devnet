import { PublicKey } from "@solomon-labs/publickey";

import { ChanceryConfig } from "../../../clients/ts/src/accounts/ChanceryConfig.js";
import { AUTHORITY_ROLE, PROGRAM_ID } from "../../../clients/ts/src/constants.js";
import { controllerAuthority, controllerConfiguration } from "../../../runtime/controller/authority.js";
import type { RuntimeHarness } from "../../../runtime/harness.js";
import { chanceryConfigPda, eventAuthorityPda, moduleActivationStatePda } from "../../../runtime/pdas.js";
import { publicKeyToBase58 } from "../../../runtime/publicKey.js";
import type { E2ESigner } from "../../../runtime/submit.js";
import type { AdministrationAuthorization } from "../../../runtime/administration/types.js";
import type { ConfigChangeContext } from "./configChange.js";

async function createContext(harness: RuntimeHarness, feePayer: E2ESigner,
    authorization: AdministrationAuthorization, governanceAuthority: PublicKey,
    operationsAuthority: PublicKey): Promise<ConfigChangeContext> {
    const [[chanceryConfig], [eventAuthority], [moduleActivationState]] = await Promise.all([
        chanceryConfigPda(), eventAuthorityPda(), moduleActivationStatePda(),
    ]);
    const account = await harness.banksClient.getAccount(chanceryConfig);
    if (account === null || account.owner?.equals(PROGRAM_ID) !== true) {
        throw new Error("Chancery configuration is absent or has the wrong owner");
    }
    const configuration = ChanceryConfig.decode(account.data, chanceryConfig);
    if (publicKeyToBase58(configuration.governanceAuthority) !== governanceAuthority.toBase58()
        || publicKeyToBase58(configuration.operationsAuthority) !== operationsAuthority.toBase58()) {
        throw new Error("selected governance and operations authorities do not match Chancery configuration");
    }
    return { harness, feePayer, authorization, chanceryConfig, eventAuthority,
        moduleActivationState, governanceAuthority, operationsAuthority };
}

export async function createSetupContext(harness: RuntimeHarness, feePayer: E2ESigner,
    setupAuthority: E2ESigner): Promise<ConfigChangeContext> {
    return createContext(harness, feePayer, { kind: "keypair", signers: [setupAuthority] },
        setupAuthority.publicKey, setupAuthority.publicKey);
}

export async function createControllerContext(harness: RuntimeHarness, caller: E2ESigner,
    programId: PublicKey): Promise<ConfigChangeContext> {
    const controller = await controllerConfiguration(programId);
    return createContext(harness, caller, { kind: "controller", controller },
        controllerAuthority(controller, AUTHORITY_ROLE.GOVERNANCE),
        controllerAuthority(controller, AUTHORITY_ROLE.OPS));
}
