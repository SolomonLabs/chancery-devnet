import { PublicKey } from "@solomon-labs/publickey";
import type { BU64, U8 } from "@solomon-labs/types";

import { AssetConfig } from "../../../clients/ts/src/accounts/AssetConfig.js";
import { ASSET_MODE, CHANGE_KIND, PROGRAM_ID } from "../../../clients/ts/src/constants.js";
import { RegisterAssetInstruction, SetAssetModeWithPendingChangeInstruction } from "../../../clients/ts/src/instructions/index.js";
import { submitAdministration } from "../../../runtime/administration/submit.js";
import { faucetAuthority } from "../../../runtime/controller/authority.js";
import { assetConfigPda } from "../../../runtime/pdas.js";
import { publicKeyToBase58 } from "../../../runtime/publicKey.js";
import { SYSTEM_PROGRAM } from "../../../runtime/wellKnown.js";
import { inspectCollateralMintAccount, type InspectedCollateralMint } from "./collateralMint.js";
import type { ConfiguredTestAsset } from "./configuredTestAsset.js";
import { proposeAndAcceptConfigChange, readAccountBytes, RISK, type ConfigChangeContext } from "./configChange.js";

const ASSET_MODE_OFFSET = 12;
const RATE_E9 = 1_000_000_000n;

async function registerAsset(context: ConfigChangeContext, mint: PublicKey,
    inspection: InspectedCollateralMint, issued: boolean): Promise<PublicKey> {
    const [address] = await assetConfigPda(mint);
    const existing = await context.harness.banksClient.getAccount(address);
    if (existing === null) {
        await submitAdministration(context, [new RegisterAssetInstruction({
            chanceryConfig: context.chanceryConfig,
            eventAuthority: context.eventAuthority,
            assetConfig: address,
            assetMint: mint,
            payer: context.feePayer.publicKey,
            operationsAuthority: context.operationsAuthority,
            systemProgram: SYSTEM_PROGRAM,
            assetTokenProgram: inspection.tokenProgram,
            mode: ASSET_MODE.WIND_DOWN as U8,
            approvedExtensionMask: inspection.extensionMask.map((word) => word as BU64),
            observedExtensionMaskHint: inspection.extensionMask.map((word) => word as BU64),
            depositRateE9: RATE_E9 as BU64,
            redeemRateE9: RATE_E9 as BU64,
            minimumDepositAmount: (issued ? 1n : 1_000n) as BU64,
            minimumRedeemAmount: (issued ? 1n : 1_000n) as BU64,
            maximumSingleSettlementAmount: (issued ? (1n << 64n) - 1n : 100_000_000n) as BU64,
            maxExtensionObservationAgeSlots: 0n as BU64,
        })]);
    }
    const account = await context.harness.banksClient.getAccount(address);
    if (account === null || account.owner?.equals(PROGRAM_ID) !== true) throw new Error("asset configuration is absent or has the wrong owner");
    const configuration = AssetConfig.decode(account.data, address);
    if (publicKeyToBase58(configuration.assetMint) !== mint.toBase58()
        || publicKeyToBase58(configuration.assetTokenProgram) !== inspection.tokenProgram.toBase58()
        || Number(configuration.decimals) !== inspection.decimals) {
        throw new Error("registered asset identity differs from the configured test mint");
    }
    for (let index = 0; index < 2; index++) {
        if ((BigInt(configuration.approvedExtensionMask[index]!) & inspection.extensionMask[index]!) !== inspection.extensionMask[index]) {
            throw new Error("registered asset extension approval does not cover the mint");
        }
    }
    return address;
}

export async function registerCollateralAsset(context: ConfigChangeContext, asset: ConfiguredTestAsset,
    faucetProgramId: PublicKey): Promise<PublicKey> {
    const account = await context.harness.banksClient.getAccount(asset.mint);
    if (account === null) throw new Error(asset.symbol + " mint is absent");
    const inspection = inspectCollateralMintAccount(account);
    const expectedHook = asset.profile.transferHook === "dormant-only" ? "dormant" : "absent";
    if (!inspection.tokenProgram.equals(asset.tokenProgram) || inspection.decimals !== asset.profile.decimals
        || inspection.transferHook !== expectedHook) throw new Error(asset.symbol + " mint differs from its test profile");
    const expectedAuthority = await faucetAuthority(faucetProgramId);
    const view = new DataView(account.data.buffer, account.data.byteOffset, account.data.byteLength);
    if (view.getUint32(0, true) !== 1 || !new PublicKey(account.data.subarray(4, 36)).equals(expectedAuthority)) {
        throw new Error(asset.symbol + " is not faucet-backed; run yarn faucet:deploy before registering it");
    }
    const address = await registerAsset(context, asset.mint, inspection, false);
    const current = await readAccountBytes(context.harness, address);
    if (current[ASSET_MODE_OFFSET] !== ASSET_MODE.ACTIVE) {
        const proposed = Uint8Array.from(current);
        proposed[ASSET_MODE_OFFSET] = ASSET_MODE.ACTIVE;
        const pending = await proposeAndAcceptConfigChange({ context, changeKind: CHANGE_KIND.SET_ASSET_MODE,
            riskClass: RISK.WIDENING, targetAccount: address, oldPayload: current, newPayload: proposed });
        await submitAdministration(context, [new SetAssetModeWithPendingChangeInstruction({
            chanceryConfig: context.chanceryConfig,
            eventAuthority: context.eventAuthority,
            pendingConfigChange: pending,
            assetConfig: address,
            governanceAuthority: context.governanceAuthority,
            newMode: ASSET_MODE.ACTIVE as U8,
            rentRefundRecipient: context.feePayer.publicKey,
        })]);
    }
    return address;
}

export async function registerIssuedAsset(context: ConfigChangeContext, mint: PublicKey): Promise<PublicKey> {
    const account = await context.harness.banksClient.getAccount(mint);
    if (account === null) throw new Error("issued-token mint is absent");
    return registerAsset(context, mint, inspectCollateralMintAccount(account), true);
}
