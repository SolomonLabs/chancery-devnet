import { PublicKey } from "@solomon-labs/publickey";

import { AUTHORITY_ROLE } from "../../clients/ts/src/constants.js";
import type { ControllerAuthority, ControllerAuthorityRole, ControllerConfiguration } from "./types.js";

const AUTHORITY_SEED = new TextEncoder().encode("chancery-authority");
const FAUCET_AUTHORITY_SEED = new TextEncoder().encode("faucet-authority");

export const CONTROLLER_ROLES: readonly ControllerAuthorityRole[] = [
    AUTHORITY_ROLE.GOVERNANCE,
    AUTHORITY_ROLE.OPS,
    AUTHORITY_ROLE.EMERGENCY,
    AUTHORITY_ROLE.ENFORCEMENT,
    AUTHORITY_ROLE.INSURANCE_ADMIN,
];

export async function controllerConfiguration(programId: PublicKey): Promise<ControllerConfiguration> {
    if (programId.toBase58() === "11111111111111111111111111111111") {
        throw new Error("controller program identity is unstamped; run yarn identity");
    }
    const authorities = await Promise.all(CONTROLLER_ROLES.map(async (role): Promise<ControllerAuthority> => {
        const [publicKey] = await PublicKey.findProgramAddress([AUTHORITY_SEED, new Uint8Array([role])], programId);
        return { role, publicKey };
    }));
    return { programId, authorities };
}

export function controllerAuthority(configuration: ControllerConfiguration, role: ControllerAuthorityRole): PublicKey {
    const authority = configuration.authorities.find((entry) => entry.role === role);
    if (authority === undefined) throw new Error("controller has no authority for role " + role);
    return authority.publicKey;
}

export async function faucetAuthority(programId: PublicKey): Promise<PublicKey> {
    const [authority] = await PublicKey.findProgramAddress([FAUCET_AUTHORITY_SEED], programId);
    return authority;
}
