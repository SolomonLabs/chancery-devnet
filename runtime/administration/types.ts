import type { RISK } from "./risk.js";
import type { RuntimeHarness } from "../harness.js";
import type { E2ESigner } from "../submit.js";
import type { ControllerConfiguration } from "../controller/types.js";

export type ConfigChangeRisk = (typeof RISK)[keyof typeof RISK];

export type AdministrationAuthorization =
    | { readonly kind: "keypair"; readonly signers: E2ESigner[] }
    | { readonly kind: "controller"; readonly controller: ControllerConfiguration };

export interface AdministrationSubmissionContext {
    readonly harness: RuntimeHarness;
    readonly feePayer: E2ESigner;
    readonly authorization: AdministrationAuthorization;
}
