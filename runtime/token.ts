/**
 * Test-fixture SPL Token helpers built only on @solomon-labs libraries.
 *
 * Uses @solomon-labs/solana-codec for System, SPL Token, Token-2022, ATA, and TLV-aware support.
 */

import {
    ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID_STRING,
    SYSTEM_PROGRAM_ID_STRING,
    SYSVAR_RENT_PROGRAM_ID_STRING,
} from "@solomon-labs/constants";
import { PublicKey } from "@solomon-labs/publickey";
import {
    CreateAccountInstruction,
    CreateAssociatedTokenAccountInstruction,
    InitializeMint2Instruction,
    MINT_ACCOUNT_LEN,
    MintToInstruction,
    TOKEN_2022_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    Token2022ConfidentialMintBurn,
    Token2022ConfidentialTransferMint,
    Token2022DefaultAccountState,
    Token2022MetadataPointer,
    Token2022MintCloseAuthority,
    Token2022PausableConfig,
    Token2022PermanentDelegate,
    Token2022PermissionedBurnConfig,
    Token2022TokenMetadata,
    Token2022TransferHook,
} from "@solomon-labs/solana-codec";

import type { RuntimeHarness } from "./harness.js";
import {
    generateSigner,
    submitTransaction,
    type E2ESigner,
} from "./submit.js";

export { TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID };

export const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey(ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID_STRING);
export const SYSTEM_PROGRAM_ID           = new PublicKey(SYSTEM_PROGRAM_ID_STRING);
export const SYSVAR_RENT_PUBKEY          = new PublicKey(SYSVAR_RENT_PROGRAM_ID_STRING);

const DEFAULT_MINT_RENT_LAMPORTS = 1_000_000_000n;
const TOKEN_2022_MINT_ACCOUNT_TYPE_OFFSET = 165;
const TOKEN_2022_TLV_HEADER_LEN = 4;
const TOKEN_2022_RESERVED_TOKEN_METADATA_BYTES = 512;

const ZERO_PUBLIC_KEY = new PublicKey(new Uint8Array(32));
const ZERO_BYTES_32 = new Uint8Array(32);
const ZERO_BYTES_36 = new Uint8Array(36);
const ZERO_BYTES_64 = new Uint8Array(64);

const TOKEN_2022_RESERVED_MINT_EXTENSION_BIT = {
    TRANSFER_HOOK:          1n << 0n,
    CONFIDENTIAL_TRANSFER:  1n << 1n,
    CONFIDENTIAL_MINT_BURN: 1n << 2n,
    PERMANENT_DELEGATE:    1n << 3n,
    PAUSABLE:              1n << 4n,
    METADATA_POINTER:      1n << 5n,
    TOKEN_METADATA:        1n << 6n,
    PERMISSIONED_BURN:     1n << 8n,
    MINT_CLOSE_AUTHORITY:  1n << 9n,
    DEFAULT_ACCOUNT_STATE: 1n << 10n,
    DORMANT_TRANSFER_HOOK:  1n << 21n,
} as const;

const TOKEN_2022_RESERVED_ACCOUNT_EXTENSION_BIT = {
    CONFIDENTIAL_TRANSFER_ACCOUNT: 1n << 1n,
    MEMO_TRANSFER:                 1n << 2n,
    TRANSFER_HOOK_ACCOUNT:         1n << 4n,
    PAUSABLE_ACCOUNT:              1n << 5n,
} as const;

export const TOKEN_2022_REQUIRED_RESERVED_EXTENSION_NAMES = [
    "TransferHook",
    "ConfidentialTransfer",
    "ConfidentialMintBurn",
    "PermanentDelegate",
    "Pausable",
    "MetadataPointer",
    "TokenMetadata",
    "MemoTransfer",
    "PermissionedBurn",
    "MintCloseAuthority",
    "DefaultAccountState",
    "FreezeAuthority",
] as const;

export const TOKEN_2022_FREEZE_AUTHORITY_RESERVED = true;

export const TOKEN_2022_ALLOWED_RESERVED_MINT_EXTENSION_MASK = [
    TOKEN_2022_RESERVED_MINT_EXTENSION_BIT.TRANSFER_HOOK
        | TOKEN_2022_RESERVED_MINT_EXTENSION_BIT.CONFIDENTIAL_TRANSFER
        | TOKEN_2022_RESERVED_MINT_EXTENSION_BIT.CONFIDENTIAL_MINT_BURN
        | TOKEN_2022_RESERVED_MINT_EXTENSION_BIT.PERMANENT_DELEGATE
        | TOKEN_2022_RESERVED_MINT_EXTENSION_BIT.PAUSABLE
        | TOKEN_2022_RESERVED_MINT_EXTENSION_BIT.METADATA_POINTER
        | TOKEN_2022_RESERVED_MINT_EXTENSION_BIT.TOKEN_METADATA
        | TOKEN_2022_RESERVED_MINT_EXTENSION_BIT.PERMISSIONED_BURN
        | TOKEN_2022_RESERVED_MINT_EXTENSION_BIT.MINT_CLOSE_AUTHORITY
        | TOKEN_2022_RESERVED_MINT_EXTENSION_BIT.DEFAULT_ACCOUNT_STATE
        | TOKEN_2022_RESERVED_MINT_EXTENSION_BIT.DORMANT_TRANSFER_HOOK,
    0n,
] as const;

export const TOKEN_2022_ALLOWED_RESERVED_ACCOUNT_EXTENSION_MASK = [
    TOKEN_2022_RESERVED_ACCOUNT_EXTENSION_BIT.CONFIDENTIAL_TRANSFER_ACCOUNT
        | TOKEN_2022_RESERVED_ACCOUNT_EXTENSION_BIT.MEMO_TRANSFER
        | TOKEN_2022_RESERVED_ACCOUNT_EXTENSION_BIT.TRANSFER_HOOK_ACCOUNT
        | TOKEN_2022_RESERVED_ACCOUNT_EXTENSION_BIT.PAUSABLE_ACCOUNT,
    0n,
] as const;

const TOKEN_2022_EXPECTED_RESERVED_MINT_EXTENSION_MASK_LO = 2_099_071n;
const TOKEN_2022_EXPECTED_RESERVED_ACCOUNT_EXTENSION_MASK_LO = 54n;

function assertToken2022RequiredReservedExtensionSet(): void {
    if (TOKEN_2022_REQUIRED_RESERVED_EXTENSION_NAMES.length !== 12) {
        throw new Error("Token-2022 reserved extension set must contain exactly 12 required entries");
    }
    if (!TOKEN_2022_FREEZE_AUTHORITY_RESERVED) {
        throw new Error("Token-2022 FreezeAuthority must be reserved through the base mint freeze authority field");
    }
    if (TOKEN_2022_ALLOWED_RESERVED_MINT_EXTENSION_MASK[0] !== TOKEN_2022_EXPECTED_RESERVED_MINT_EXTENSION_MASK_LO) {
        throw new Error("Token-2022 mint reserved extension mask is missing a required extension or contains an unexpected bit");
    }
    if (TOKEN_2022_ALLOWED_RESERVED_MINT_EXTENSION_MASK[1] !== 0n) {
        throw new Error("Token-2022 mint reserved extension mask high word must remain empty");
    }
    if (TOKEN_2022_ALLOWED_RESERVED_ACCOUNT_EXTENSION_MASK[0] !== TOKEN_2022_EXPECTED_RESERVED_ACCOUNT_EXTENSION_MASK_LO) {
        throw new Error("Token-2022 account reserved extension mask is missing a required extension or contains an unexpected bit");
    }
    if (TOKEN_2022_ALLOWED_RESERVED_ACCOUNT_EXTENSION_MASK[1] !== 0n) {
        throw new Error("Token-2022 account reserved extension mask high word must remain empty");
    }
}

assertToken2022RequiredReservedExtensionSet();

function token2022TlvEntryLen(dataLength: number): number {
    return TOKEN_2022_TLV_HEADER_LEN + dataLength;
}

function token2022RequiredMintExtensionsReservedLen(): number {
    const mintExtensionPayloadLengths = [
        new Token2022TransferHook({
            authority: ZERO_PUBLIC_KEY,
            programId: ZERO_PUBLIC_KEY,
        }).encode().length,
        new Token2022ConfidentialTransferMint({
            authority: ZERO_PUBLIC_KEY,
            autoApproveNewAccounts: false,
            auditorElgamalPubkey: ZERO_BYTES_32,
        }).encode().length,
        new Token2022ConfidentialMintBurn({
            confidentialSupply: ZERO_BYTES_64,
            decryptableSupply: ZERO_BYTES_36,
            supplyElgamalPubkey: ZERO_BYTES_32,
            pendingBurn: ZERO_BYTES_64,
        }).encode().length,
        new Token2022PermanentDelegate({ delegate: ZERO_PUBLIC_KEY }).encode().length,
        new Token2022PausableConfig({
            authority: ZERO_PUBLIC_KEY,
            paused: false,
        }).encode().length,
        new Token2022MetadataPointer({
            authority: ZERO_PUBLIC_KEY,
            metadataAddress: ZERO_PUBLIC_KEY,
        }).encode().length,
        Math.max(
            TOKEN_2022_RESERVED_TOKEN_METADATA_BYTES,
            new Token2022TokenMetadata({
                updateAuthority: ZERO_PUBLIC_KEY,
                mint: ZERO_PUBLIC_KEY,
                name: "",
                symbol: "",
                uri: "",
                additionalMetadata: [],
            }).encode().length,
        ),
        new Token2022PermissionedBurnConfig({ authority: ZERO_PUBLIC_KEY }).encode().length,
        new Token2022MintCloseAuthority({ closeAuthority: ZERO_PUBLIC_KEY }).encode().length,
        new Token2022DefaultAccountState({ state: 1 }).encode().length,
    ];

    let accountLen = TOKEN_2022_MINT_ACCOUNT_TYPE_OFFSET + 1;
    for (let i = 0, len = mintExtensionPayloadLengths.length; i < len; i++) {
        accountLen += token2022TlvEntryLen(mintExtensionPayloadLengths[i]);
    }
    return accountLen;
}

export const TOKEN_2022_REQUIRED_EXTENSIONS_RESERVED_MINT_ACCOUNT_LEN = token2022RequiredMintExtensionsReservedLen();
export const TOKEN_2022_ALL_EXTENSIONS_RESERVED_MINT_ACCOUNT_LEN = TOKEN_2022_REQUIRED_EXTENSIONS_RESERVED_MINT_ACCOUNT_LEN;

async function rentForMint(ctx: RuntimeHarness, mintAccountLen: number = MINT_ACCOUNT_LEN): Promise<bigint> {
    const rentFunction = ctx.banksClient.getMinimumBalanceForRentExemption;
    if (rentFunction !== undefined) {
        return rentFunction.call(ctx.banksClient, mintAccountLen);
    }
    return DEFAULT_MINT_RENT_LAMPORTS;
}

function isToken2022(tokenProgram: PublicKey): boolean {
    return tokenProgram.toBase58() === TOKEN_2022_PROGRAM_ID.toBase58();
}

export async function getAssociatedTokenAddress(
    mint:                PublicKey,
    owner:               PublicKey,
    _allowOwnerOffCurve: boolean,
    tokenProgram:        PublicKey,
): Promise<PublicKey> {
    void _allowOwnerOffCurve;
    if (isToken2022(tokenProgram)) {
        return PublicKey.findAssociatedTokenAddressToken2022(owner, mint);
    }
    return PublicKey.findAssociatedTokenAddress(owner, mint);
}

export const getAssociatedTokenAddressSync = getAssociatedTokenAddress;

export async function createTestMint(
    ctx:             RuntimeHarness,
    payer:           E2ESigner,
    mintAuthority:   PublicKey,
    freezeAuthority: PublicKey | null,
    decimals:        number,
    tokenProgram:    PublicKey,
): Promise<PublicKey> {
    const mintSigner = generateSigner();
    const lamports   = await rentForMint(ctx, MINT_ACCOUNT_LEN);

    await submitTransaction(ctx, [
        new CreateAccountInstruction({
            from:       payer.publicKey,
            newAccount: mintSigner.publicKey,
            lamports,
            space:      MINT_ACCOUNT_LEN,
            owner:      tokenProgram,
        }),
        new InitializeMint2Instruction({
            programId:              tokenProgram,
            mint:                   mintSigner.publicKey,
            decimals,
            mintAuthority,
            freezeAuthority,
        }),
    ], [mintSigner], payer);

    return mintSigner.publicKey;
}

export async function createToken2022MintWithAllExtensionsReserved(
    ctx:             RuntimeHarness,
    payer:           E2ESigner,
    mintAuthority:   PublicKey,
    freezeAuthority: PublicKey | null,
    decimals:        number,
): Promise<PublicKey> {
    const mintSigner = generateSigner();
    const lamports   = await rentForMint(ctx, TOKEN_2022_ALL_EXTENSIONS_RESERVED_MINT_ACCOUNT_LEN);

    await submitTransaction(ctx, [
        new CreateAccountInstruction({
            from:       payer.publicKey,
            newAccount: mintSigner.publicKey,
            lamports,
            space:      TOKEN_2022_ALL_EXTENSIONS_RESERVED_MINT_ACCOUNT_LEN,
            owner:      TOKEN_2022_PROGRAM_ID,
        }),
        new InitializeMint2Instruction({
            programId:              TOKEN_2022_PROGRAM_ID,
            mint:                   mintSigner.publicKey,
            decimals,
            mintAuthority,
            freezeAuthority,
        }),
    ], [mintSigner], payer);

    return mintSigner.publicKey;
}

/**
 * Create a Token-2022 mint carrying an ACTIVE TransferFeeConfig extension, so a
 * `transfer_checked` into a reserve withholds the fee (issue-73 regression).
 * Instructions are hand-built: InitializeTransferFeeConfig (TokenInstruction 26,
 * sub-instruction 0, 1-byte COption tags) then InitializeMint2 (20) - the codec's
 * InitializeMint2 mis-encodes the freeze COption, so we build it by hand with no
 * freeze authority. Account is sized exactly for the one extension TLV.
 */
export async function createToken2022MintWithTransferFee(
    ctx:                    RuntimeHarness,
    payer:                  E2ESigner,
    mintAuthority:          PublicKey,
    decimals:               number,
    transferFeeBasisPoints: number,
    maximumFee:             bigint,
): Promise<PublicKey> {
    // Token-2022 extended-mint layout: 165 base + 1 account-type byte + TLV
    // entries (4-byte header + value). TransferFeeConfig value is 108 bytes.
    const space    = 166 + 4 + 108;
    const mintSigner = generateSigner();
    const lamports = await rentForMint(ctx, space);

    // InitializeTransferFeeConfig: [26, 0, cfgAuth=None(0), withdrawAuth=None(0), bps u16 LE, maxFee u64 LE]
    const feeData = new Uint8Array(14);
    feeData[0] = 26;
    feeData[1] = 0;
    feeData[2] = 0;
    feeData[3] = 0;
    const feeView = new DataView(feeData.buffer);
    feeView.setUint16(4, transferFeeBasisPoints, true);
    feeView.setBigUint64(6, maximumFee, true);

    // InitializeMint2: [20, decimals, mintAuthority(32), freezeOption=None(0)]
    const mintData = new Uint8Array(35);
    mintData[0] = 20;
    mintData[1] = decimals;
    mintData.set(mintAuthority.toBytes(), 2);
    mintData[34] = 0;

    await submitTransaction(ctx, [
        new CreateAccountInstruction({
            from:       payer.publicKey,
            newAccount: mintSigner.publicKey,
            lamports,
            space,
            owner:      TOKEN_2022_PROGRAM_ID,
        }),
        { programId: TOKEN_2022_PROGRAM_ID, accounts: [{ pubkey: mintSigner.publicKey, isSigner: false, isWritable: true }], data: feeData },
        { programId: TOKEN_2022_PROGRAM_ID, accounts: [{ pubkey: mintSigner.publicKey, isSigner: false, isWritable: true }], data: mintData },
    ], [mintSigner], payer);

    return mintSigner.publicKey;
}

export async function createTokenAccount(
    ctx:          RuntimeHarness,
    payer:        E2ESigner,
    mint:         PublicKey,
    owner:        PublicKey,
    tokenProgram: PublicKey,
): Promise<PublicKey> {
    const associatedToken = await getAssociatedTokenAddress(mint, owner, false, tokenProgram);
    await submitTransaction(ctx, [new CreateAssociatedTokenAccountInstruction({
        payer: payer.publicKey,
        associatedToken,
        owner,
        mint,
        tokenProgram,
    })], [], payer);
    return associatedToken;
}

export async function mintTokensTo(
    ctx:           RuntimeHarness,
    payer:         E2ESigner,
    mintAuthority: E2ESigner,
    mint:          PublicKey,
    destination:   PublicKey,
    amount:        bigint,
    tokenProgram:  PublicKey,
): Promise<void> {
    await submitTransaction(ctx, [new MintToInstruction({
        programId: tokenProgram,
        mint,
        destination,
        authority: mintAuthority.publicKey,
        amount,
    })], [mintAuthority], payer);
}


export async function revokeMintAuthority(
    ctx:              RuntimeHarness,
    payer:            E2ESigner,
    currentAuthority: E2ESigner,
    mint:             PublicKey,
    tokenProgram:     PublicKey,
): Promise<void> {
    await submitTransaction(ctx, [{
        programId: tokenProgram,
        accounts: [
            { pubkey: mint, isSigner: false, isWritable: true },
            { pubkey: currentAuthority.publicKey, isSigner: true, isWritable: false },
        ],
        // SPL Token SetAuthority, AuthorityType::MintTokens, COption::None.
        data: new Uint8Array([6, 0, 0]),
    }], [currentAuthority], payer);
}

export async function deriveReserveTokenAccount(
    assetMint:         PublicKey,
    reserveAuthority: PublicKey,
    tokenProgram:     PublicKey,
): Promise<PublicKey> {
    return getAssociatedTokenAddress(assetMint, reserveAuthority, true, tokenProgram);
}

export function createAssociatedTokenAccountInstruction(
    payer:        PublicKey,
    ata:          PublicKey,
    owner:        PublicKey,
    mint:         PublicKey,
    tokenProgram: PublicKey,
): CreateAssociatedTokenAccountInstruction {
    return new CreateAssociatedTokenAccountInstruction({
        payer,
        associatedToken: ata,
        owner,
        mint,
        tokenProgram,
    });
}

export function createMintToInstruction(
    mint:          PublicKey,
    destination:   PublicKey,
    authority:     PublicKey,
    amount:        bigint,
    _multiSigners: readonly unknown[],
    tokenProgram:  PublicKey,
): MintToInstruction {
    void _multiSigners;
    return new MintToInstruction({
        programId: tokenProgram,
        mint,
        destination,
        authority,
        amount,
    });
}
