import type { IsEncodable } from "@solomon-labs/solana-codec";

import { ExecuteChanceryInstruction } from "../controller/ExecuteChanceryInstruction.js";
import { submitTransaction } from "../submit.js";
import type { AdministrationSubmissionContext } from "./types.js";

export async function submitAdministration(context: AdministrationSubmissionContext, instructions: IsEncodable[]): Promise<readonly string[]> {
    const authorization = context.authorization;
    if (authorization.kind === "keypair") {
        return submitTransaction(context.harness, instructions, authorization.signers, context.feePayer);
    }
    const wrapped = instructions.map((instruction) => new ExecuteChanceryInstruction(
        authorization.controller, context.feePayer.publicKey, instruction));
    return submitTransaction(context.harness, wrapped, [], context.feePayer);
}
