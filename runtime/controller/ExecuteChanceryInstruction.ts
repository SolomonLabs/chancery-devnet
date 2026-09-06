import { PublicKey } from "@solomon-labs/publickey";
import type { EncodedInstruction, InstructionAccount, IsEncodable } from "@solomon-labs/solana-codec";

import { PROGRAM_ID } from "../../clients/ts/src/constants.js";
import { publicKeyToBase58 } from "../publicKey.js";
import type { ControllerConfiguration } from "./types.js";

export class ExecuteChanceryInstruction implements IsEncodable {
    readonly programId: PublicKey;
    protected readonly roleMask: number;
    protected readonly chanceryData: Uint8Array;
    private readonly forwardedAccounts: InstructionAccount[];

    constructor(configuration: ControllerConfiguration, caller: PublicKey, instruction: IsEncodable) {
        if (publicKeyToBase58(instruction.programId) !== PROGRAM_ID.toBase58()) {
            throw new Error("controller relay only accepts the configured Chancery program");
        }
        this.programId = configuration.programId;
        this.chanceryData = instruction.encode();
        if (this.chanceryData.length < 2) throw new Error("Chancery instruction is missing its discriminator");
        let roleMask = 0;
        const forwarded = instruction.accounts().map((account): InstructionAccount => {
            const authority = configuration.authorities.find((entry) =>
                entry.publicKey.toBase58() === publicKeyToBase58(account.pubkey));
            if (authority === undefined || !account.isSigner) return { ...account };
            roleMask |= 1 << authority.role;
            return { ...account, isSigner: false };
        });
        if (roleMask === 0 || (roleMask & ~0x1f) !== 0) {
            throw new Error("instruction must require at least one controller authority");
        }
        this.roleMask = roleMask;
        this.forwardedAccounts = [
            { pubkey: caller, isSigner: true, isWritable: false },
            { pubkey: PROGRAM_ID, isSigner: false, isWritable: false },
            ...forwarded,
        ];
    }

    encode(): Uint8Array {
        const data = new Uint8Array(2 + this.chanceryData.length);
        data.set([0x01, this.roleMask]);
        data.set(this.chanceryData, 2);
        return data;
    }

    accounts(): InstructionAccount[] {
        return this.forwardedAccounts.map((account) => ({ ...account }));
    }

    instruction(): EncodedInstruction {
        return { programId: this.programId, accounts: this.accounts(), data: this.encode() };
    }
}
