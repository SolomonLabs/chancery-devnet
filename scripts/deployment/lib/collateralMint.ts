import { PublicKey } from "@solomon-labs/publickey";

import { TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solomon-labs/solana-codec";

export const COLLATERAL_EXTENSION_BIT = {
    TransferHook: 1n << 0n,
    ConfidentialTransfer: 1n << 1n,
    ConfidentialMintBurn: 1n << 2n,
    PermanentDelegate: 1n << 3n,
    Pausable: 1n << 4n,
    MetadataPointer: 1n << 5n,
    TokenMetadata: 1n << 6n,
    PermissionedBurn: 1n << 8n,
    MintCloseAuthority: 1n << 9n,
    DefaultAccountState: 1n << 10n,
    TransferFeeConfig: 1n << 12n,
    InterestBearingConfig: 1n << 13n,
    NonTransferable: 1n << 14n,
    ScaledUiAmount: 1n << 15n,
    GroupPointer: 1n << 16n,
    TokenGroup: 1n << 17n,
    GroupMemberPointer: 1n << 18n,
    TokenGroupMember: 1n << 19n,
    ConfidentialTransferFeeConfig: 1n << 20n,
    DormantTransferHook: 1n << 21n,
} as const;

const TAGS = {
    0x0001: ["TransferFeeConfig", COLLATERAL_EXTENSION_BIT.TransferFeeConfig],
    0x0003: ["MintCloseAuthority", COLLATERAL_EXTENSION_BIT.MintCloseAuthority],
    0x0004: ["ConfidentialTransfer", COLLATERAL_EXTENSION_BIT.ConfidentialTransfer],
    0x0006: ["DefaultAccountState", COLLATERAL_EXTENSION_BIT.DefaultAccountState],
    0x0009: ["NonTransferable", COLLATERAL_EXTENSION_BIT.NonTransferable],
    0x000a: ["InterestBearingConfig", COLLATERAL_EXTENSION_BIT.InterestBearingConfig],
    0x000c: ["PermanentDelegate", COLLATERAL_EXTENSION_BIT.PermanentDelegate],
    0x0010: ["ConfidentialTransferFeeConfig", COLLATERAL_EXTENSION_BIT.ConfidentialTransferFeeConfig],
    0x0012: ["MetadataPointer", COLLATERAL_EXTENSION_BIT.MetadataPointer],
    0x0013: ["TokenMetadata", COLLATERAL_EXTENSION_BIT.TokenMetadata],
    0x0014: ["GroupPointer", COLLATERAL_EXTENSION_BIT.GroupPointer],
    0x0015: ["TokenGroup", COLLATERAL_EXTENSION_BIT.TokenGroup],
    0x0016: ["GroupMemberPointer", COLLATERAL_EXTENSION_BIT.GroupMemberPointer],
    0x0017: ["TokenGroupMember", COLLATERAL_EXTENSION_BIT.TokenGroupMember],
    0x0018: ["ConfidentialMintBurn", COLLATERAL_EXTENSION_BIT.ConfidentialMintBurn],
    0x0019: ["ScaledUiAmount", COLLATERAL_EXTENSION_BIT.ScaledUiAmount],
    0x001a: ["Pausable", COLLATERAL_EXTENSION_BIT.Pausable],
    0x001c: ["PermissionedBurn", COLLATERAL_EXTENSION_BIT.PermissionedBurn],
} as const;
const ACCOUNT_ONLY_TAGS = new Set([0x0002, 0x0005, 0x0007, 0x0008, 0x000b, 0x000d, 0x000f, 0x0011, 0x001b]);
const ZERO = new Uint8Array(32);

export interface MintAccountLike {
    data: Uint8Array;
    owner?: PublicKey;
}

export interface InspectedCollateralMint {
    decimals: number;
    tokenProgram: PublicKey;
    extensionMask: [bigint, bigint];
    extensionNames: string[];
    transferHook: "absent" | "dormant" | "active";
}

function allZero(value: Uint8Array): boolean {
    if (value.length !== ZERO.length) return false;
    for (let index = 0; index < value.length; index++) if (value[index] !== 0) return false;
    return true;
}

export function inspectCollateralMintAccount(account: MintAccountLike): InspectedCollateralMint {
    if (account.owner === undefined) throw new Error("mint RPC response omitted owner");
    if (account.data.length < 82) throw new Error(`mint account is too short: ${account.data.length}`);
    if (account.data[45] !== 1) throw new Error("mint is not initialized");
    const decimals = account.data[44] ?? 0;
    if (account.owner.equals(TOKEN_PROGRAM_ID)) {
        if (account.data.length !== 82) throw new Error(`classic SPL mint must be 82 bytes, got ${account.data.length}`);
        return { decimals, tokenProgram: TOKEN_PROGRAM_ID, extensionMask: [0n, 0n], extensionNames: [], transferHook: "absent" };
    }
    if (!account.owner.equals(TOKEN_2022_PROGRAM_ID)) {
        throw new Error(`mint owner ${account.owner.toBase58()} is not SPL Token or Token-2022`);
    }
    if (account.data.length < 166 || account.data[165] !== 1) {
        throw new Error("Token-2022 mint has an invalid extended-mint layout");
    }

    let cursor = 166;
    let mask = 0n;
    let hook: "absent" | "dormant" | "active" = "absent";
    const names: string[] = [];
    while (cursor + 4 <= account.data.length) {
        const tag = account.data[cursor]! | (account.data[cursor + 1]! << 8);
        const length = account.data[cursor + 2]! | (account.data[cursor + 3]! << 8);
        if (tag === 0) break;
        const start = cursor + 4;
        const end = start + length;
        if (end > account.data.length) throw new Error(`Token-2022 TLV tag ${tag} overruns the mint account`);
        if (tag === 0x000e) {
            if (length !== 64) throw new Error("TransferHook extension must contain two 32-byte optional pubkeys");
            hook = allZero(account.data.subarray(start + 32, end)) ? "dormant" : "active";
            const name = hook === "dormant" ? "DormantTransferHook" : "TransferHook";
            mask |= COLLATERAL_EXTENSION_BIT[name];
            names.push(name);
        } else if (ACCOUNT_ONLY_TAGS.has(tag)) {
            // Mirror the on-chain parser: account-only extensions do not set mint bits.
        } else {
            const mapped = TAGS[tag as keyof typeof TAGS];
            if (mapped === undefined) throw new Error(`unknown Token-2022 mint extension tag ${tag}`);
            mask |= mapped[1];
            names.push(mapped[0]);
        }
        cursor = end;
    }
    names.sort();
    return { decimals, tokenProgram: TOKEN_2022_PROGRAM_ID, extensionMask: [mask, 0n], extensionNames: names, transferHook: hook };
}
