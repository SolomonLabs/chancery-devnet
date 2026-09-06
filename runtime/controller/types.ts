import type { PublicKey } from "@solomon-labs/publickey";

export type ControllerAuthorityRole = 0 | 1 | 2 | 3 | 4;

export interface ControllerAuthority {
    readonly role: ControllerAuthorityRole;
    readonly publicKey: PublicKey;
}

export interface ControllerConfiguration {
    readonly programId: PublicKey;
    readonly authorities: readonly ControllerAuthority[];
}
