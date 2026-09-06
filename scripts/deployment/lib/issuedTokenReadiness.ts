import { PublicKey } from "@solomon-labs/publickey";

import { VerifyIssuedTokenDeploymentInstruction } from "../../../clients/ts/src/instructions/VerifyIssuedTokenDeploymentInstruction.js";
import { submitAdministration } from "../../../runtime/administration/submit.js";
import { issuedTokenControlPda } from "../../../runtime/pdas.js";
import { registerIssuedAsset } from "./assetRegistration.js";
import type { ConfigChangeContext } from "./configChange.js";

export async function verifyIssuedTokenReadiness(context: ConfigChangeContext, issuedMint: PublicKey): Promise<void> {
    const assetConfig = await registerIssuedAsset(context, issuedMint);
    const [issuedTokenControl] = await issuedTokenControlPda();
    await submitAdministration(context, [new VerifyIssuedTokenDeploymentInstruction({
        moduleActivationState: context.moduleActivationState,
        chanceryConfig: context.chanceryConfig,
        eventAuthority: context.eventAuthority,
        assetConfig,
        issuedTokenControl,
        issuedTokenMint: issuedMint,
        payer: context.feePayer.publicKey,
        governanceAuthority: context.governanceAuthority,
    })]);
}
