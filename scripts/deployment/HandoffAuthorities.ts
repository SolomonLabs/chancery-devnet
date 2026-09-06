import { PublicKey } from "@solomon-labs/publickey";
import type { BU64, U8 } from "@solomon-labs/types";

import { AuthorityTransfer } from "../../clients/ts/src/accounts/AuthorityTransfer.js";
import { ChanceryConfig } from "../../clients/ts/src/accounts/ChanceryConfig.js";
import { AUTHORITY_ROLE, PROGRAM_ID } from "../../clients/ts/src/constants.js";
import { AcceptAuthorityTransferInstruction, ProposeAuthorityTransferInstruction } from "../../clients/ts/src/instructions/index.js";
import { controllerAuthority, controllerConfiguration } from "../../runtime/controller/authority.js";
import { ExecuteChanceryInstruction } from "../../runtime/controller/ExecuteChanceryInstruction.js";
import type { ControllerAuthorityRole } from "../../runtime/controller/types.js";
import { createDevnetHarness } from "../../runtime/devnetHarness.js";
import type { RuntimeHarness } from "../../runtime/harness.js";
import { authorityTransferPda, chanceryConfigPda, eventAuthorityPda } from "../../runtime/pdas.js";
import { publicKeyToBase58 } from "../../runtime/publicKey.js";
import { submitTransaction } from "../../runtime/submit.js";
import { SYSTEM_PROGRAM } from "../../runtime/wellKnown.js";
import { DRY_RUN, activeNetwork, loadFeePayer, loadSetupAuthority, preflight, publicKeyFromAddress } from "./lib/bootstrap.js";

const ROLE_FIELDS = ["governanceAuthority", "operationsAuthority", "emergencyAuthority", "enforcementAuthority", "insuranceAdminAuthority"] as const;
const HANDOFF_ORDER: readonly ControllerAuthorityRole[] = [
    AUTHORITY_ROLE.OPS, AUTHORITY_ROLE.EMERGENCY, AUTHORITY_ROLE.ENFORCEMENT,
    AUTHORITY_ROLE.INSURANCE_ADMIN, AUTHORITY_ROLE.GOVERNANCE,
];

async function readConfiguration(harness: RuntimeHarness, address: PublicKey): Promise<ChanceryConfig> {
    const account = await harness.banksClient.getAccount(address);
    if (account === null || account.owner?.equals(PROGRAM_ID) !== true) {
        throw new Error("Chancery configuration is absent or has the wrong owner");
    }
    return ChanceryConfig.decode(account.data, address);
}

async function readTransfer(harness: RuntimeHarness, address: PublicKey): Promise<AuthorityTransfer | null> {
    const account = await harness.banksClient.getAccount(address);
    if (account === null || account.data.length === 0) return null;
    if (account.owner?.equals(PROGRAM_ID) !== true) throw new Error("authority transfer has the wrong owner");
    if (account.data.every((byte) => byte === 0)) return null;
    return AuthorityTransfer.decode(account.data, address);
}

async function main(): Promise<void> {
    preflight();
    if (DRY_RUN) throw new Error("authority handoff requires confirmed state changes; --dry-run is unsupported");
    const network = activeNetwork();
    const feePayer = loadFeePayer();
    const controller = await controllerConfiguration(publicKeyFromAddress(network.faucetProgramId));
    const harness = await createDevnetHarness({ rpcUrl: network.rpcUrl, commitment: "confirmed", requireProgramdata: false });
    const program = await harness.banksClient.getAccount(controller.programId);
    if (program?.executable !== true) throw new Error("deploy the faucet/controller before transferring authorities");
    const [[configurationAddress], [eventAuthority]] = await Promise.all([chanceryConfigPda(), eventAuthorityPda()]);

    for (const role of HANDOFF_ORDER) {
        const expected = controllerAuthority(controller, role);
        const configuration = await readConfiguration(harness, configurationAddress);
        const current = publicKeyToBase58(configuration[ROLE_FIELDS[role]]);
        if (current === expected.toBase58()) {
            console.log(ROLE_FIELDS[role] + " already belongs to the controller");
            continue;
        }
        const setup = loadSetupAuthority();
        if (publicKeyToBase58(configuration.governanceAuthority) !== setup.publicKey.toBase58()
            || current !== setup.publicKey.toBase58()) {
            throw new Error("handoff requires the configured setup authority for governance and the remaining role");
        }
        const [transferAddress] = await authorityTransferPda(role);
        let transfer = await readTransfer(harness, transferAddress);
        const clock = await harness.banksClient.getClock();
        if (transfer !== null && BigInt(transfer.expiresAtSlot) >= clock.slot) {
            if (Number(transfer.roleKind) !== role
                || publicKeyToBase58(transfer.proposedAuthority) !== expected.toBase58()
                || publicKeyToBase58(transfer.oldAuthority) !== current
                || publicKeyToBase58(transfer.proposingGovernance) !== setup.publicKey.toBase58()) {
                throw new Error("a different live authority transfer already occupies " + transferAddress.toBase58());
            }
        } else {
            await submitTransaction(harness, [new ProposeAuthorityTransferInstruction({
                chanceryConfig: configurationAddress,
                eventAuthority,
                authorityTransfer: transferAddress,
                payer: feePayer.publicKey,
                governanceAuthority: setup.publicKey,
                systemProgram: SYSTEM_PROGRAM,
                roleKind: role as U8,
                proposedAuthority: expected,
                timelockSlots: 1n as BU64,
            })], [setup], feePayer);
            transfer = await readTransfer(harness, transferAddress);
        }
        if (transfer === null) throw new Error("authority transfer was not created");
        const deadline = Date.now() + 120_000;
        while ((await harness.banksClient.getClock()).slot < BigInt(transfer.executableAfterSlot)) {
            if (Date.now() >= deadline) throw new Error("authority-transfer timelock has not elapsed");
            await new Promise((resolve) => setTimeout(resolve, 500));
        }
        await submitTransaction(harness, [new ExecuteChanceryInstruction(controller, feePayer.publicKey,
            new AcceptAuthorityTransferInstruction({
                chanceryConfig: configurationAddress,
                eventAuthority,
                authorityTransfer: transferAddress,
                newAuthority: expected,
                roleKind: role as U8,
            }))], [], feePayer);
        const after = await readConfiguration(harness, configurationAddress);
        if (publicKeyToBase58(after[ROLE_FIELDS[role]]) !== expected.toBase58()) {
            throw new Error("authority handoff did not produce the expected configuration");
        }
        console.log(ROLE_FIELDS[role] + " -> " + expected.toBase58());
    }
}

main().catch((error: unknown) => {
    console.error(error instanceof Error ? error.stack ?? error.message : String(error));
    process.exitCode = 1;
});
