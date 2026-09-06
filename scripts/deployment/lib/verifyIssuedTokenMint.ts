import { PublicKey } from "@solomon-labs/publickey";
import {
    TOKEN_2022_PROGRAM_ID,
    Token2022AccountType,
    Token2022ConfidentialTransferMint,
    Token2022DefaultAccountState,
    Token2022ExtensionType,
    Token2022MetadataPointer,
    Token2022MintCloseAuthority,
    Token2022PausableConfig,
    Token2022PermanentDelegate,
    Token2022TokenMetadata,
    Token2022TlvData,
    Token2022TransferHook,
} from "@solomon-labs/solana-codec";
import type { PublicKeyLike } from "@solomon-labs/types";

import { publicKeyToBase58 } from "../../core/PublicKeyUtil.js";

const ZERO_PUBLIC_KEY = new PublicKey(new Uint8Array(32));

export interface IssuedTokenMintExpectation {
    mint: PublicKey;
    mintAuthority: PublicKey;
    freezeAuthority: PublicKey;
    decimals: number;
    transferHookAuthority: PublicKey;
    confidentialTransferAuthority: PublicKey;
    permanentDelegate: PublicKey;
    pauseAuthority: PublicKey;
    metadataPointerAuthority: PublicKey;
    closeAuthority: PublicKey;
    metadataUpdateAuthority: PublicKey;
    metadata: { name: string; symbol: string; uri: string };
}

export interface IssuedTokenMintAccount {
    data: Uint8Array;
    lamports: bigint;
    owner?: PublicKey;
}

export interface VerifiedIssuedTokenMint {
    tlvData: Token2022TlvData;
}

function fail(field: string, actual: unknown, expected: unknown): never {
    throw new Error(`issued mint verification failed for ${field}: got ${String(actual)}, expected ${String(expected)}`);
}

function assertKey(field: string, actual: PublicKeyLike, expected: PublicKey): void {
    const actualString = publicKeyToBase58(actual);
    const expectedString = expected.toBase58();
    if (actualString !== expectedString) fail(field, actualString, expectedString);
}

function assertEqual(field: string, actual: unknown, expected: unknown): void {
    if (actual !== expected) fail(field, actual, expected);
}

function assertZeroBytes(field: string, bytes: Uint8Array): void {
    for (let index = 0; index < bytes.length; index++) {
        if (bytes[index] !== 0) fail(`${field}[${index}]`, bytes[index], 0);
    }
}

/** Decode and validate every base-mint field and every extension installed by the launch script. */
export function verifyIssuedTokenMintAccount(
    account: IssuedTokenMintAccount,
    expected: IssuedTokenMintExpectation,
    minimumRentLamports: bigint,
    options: { requireZeroSupply?: boolean } = {},
): VerifiedIssuedTokenMint {
    if (account.owner === undefined) fail("owner", "missing from RPC response", TOKEN_2022_PROGRAM_ID.toBase58());
    assertKey("owner", account.owner, TOKEN_2022_PROGRAM_ID);
    if (account.lamports < minimumRentLamports) {
        fail("rent-exempt lamports", account.lamports, `>= ${minimumRentLamports}`);
    }

    // Extended mints use the 82-byte base Mint layout, 83 bytes of padding up
    // to Account::LEN (165), one account-type byte, then TLV. Decode explicitly:
    // the bundled TokenMint2022 codec currently assumes the account-type byte
    // immediately follows byte 82 and therefore cannot safely verify this layout.
    if (account.data.length < 166) fail("data length", account.data.length, ">= 166");
    const view = new DataView(account.data.buffer, account.data.byteOffset, account.data.byteLength);
    assertEqual("mint authority option", view.getUint32(0, true), 1);
    assertKey("mint authority", new PublicKey(account.data.subarray(4, 36)), expected.mintAuthority);
    const supply = view.getBigUint64(36, true);
    if (options.requireZeroSupply === true) assertEqual("supply", supply, 0n);
    assertEqual("decimals", account.data[44], expected.decimals);
    assertEqual("is initialized", account.data[45], 1);
    assertEqual("freeze authority option", view.getUint32(46, true), 1);
    assertKey("freeze authority", new PublicKey(account.data.subarray(50, 82)), expected.freezeAuthority);
    assertZeroBytes("extended mint padding", account.data.subarray(82, 165));
    assertEqual("account type", account.data[165], Token2022AccountType.Mint);

    const tlvData = Token2022TlvData.decode(account.data.subarray(166));

    const expectedTypes = new Set<number>([
        Token2022ExtensionType.TransferHook,
        Token2022ExtensionType.ConfidentialTransferMint,
        Token2022ExtensionType.PermanentDelegate,
        Token2022ExtensionType.Pausable,
        Token2022ExtensionType.MetadataPointer,
        Token2022ExtensionType.MintCloseAuthority,
        Token2022ExtensionType.DefaultAccountState,
        Token2022ExtensionType.TokenMetadata,
    ]);
    const entries = new Map<number, (typeof tlvData.entries)[number]>();
    for (const entry of tlvData.entries) {
        if (entries.has(entry.extensionType)) fail("duplicate extension", entry.extensionTypeName, "one entry");
        entries.set(entry.extensionType, entry);
    }
    const actualTypes = [...entries.keys()].sort((a, b) => a - b);
    const wantedTypes = [...expectedTypes].sort((a, b) => a - b);
    assertEqual("extension count", actualTypes.length, wantedTypes.length);
    assertEqual("extension types", actualTypes.join(","), wantedTypes.join(","));

    const decoded = <T>(type: Token2022ExtensionType, ctor: abstract new (...args: never[]) => T): T => {
        const entry = entries.get(type);
        if (entry === undefined) fail("extension", type, "present");
        if (!(entry.decoded instanceof ctor)) fail(`extension ${entry.extensionTypeName}`, "undecodable", ctor.name);
        return entry.decoded;
    };

    const hook = decoded(Token2022ExtensionType.TransferHook, Token2022TransferHook);
    assertKey("transfer hook authority", hook.authority, expected.transferHookAuthority);
    assertKey("transfer hook program", hook.programId, ZERO_PUBLIC_KEY);

    const confidential = decoded(Token2022ExtensionType.ConfidentialTransferMint, Token2022ConfidentialTransferMint);
    assertKey("confidential transfer authority", confidential.authority, expected.confidentialTransferAuthority);
    assertEqual("confidential auto-approve", confidential.autoApproveNewAccounts, false);
    assertZeroBytes("confidential auditor key", confidential.auditorElgamalPubkey);

    const permanent = decoded(Token2022ExtensionType.PermanentDelegate, Token2022PermanentDelegate);
    assertKey("permanent delegate", permanent.delegate, expected.permanentDelegate);

    const pausable = decoded(Token2022ExtensionType.Pausable, Token2022PausableConfig);
    assertKey("pause authority", pausable.authority, expected.pauseAuthority);
    assertEqual("paused", pausable.paused, false);

    const pointer = decoded(Token2022ExtensionType.MetadataPointer, Token2022MetadataPointer);
    assertKey("metadata pointer authority", pointer.authority, expected.metadataPointerAuthority);
    assertKey("metadata pointer address", pointer.metadataAddress, expected.mint);

    const close = decoded(Token2022ExtensionType.MintCloseAuthority, Token2022MintCloseAuthority);
    assertKey("mint close authority", close.closeAuthority, expected.closeAuthority);

    const defaultState = decoded(Token2022ExtensionType.DefaultAccountState, Token2022DefaultAccountState);
    assertEqual("default account state", defaultState.state, 1);

    const metadata = decoded(Token2022ExtensionType.TokenMetadata, Token2022TokenMetadata);
    assertKey("metadata update authority", metadata.updateAuthority, expected.metadataUpdateAuthority);
    assertKey("metadata mint", metadata.mint, expected.mint);
    assertEqual("metadata name", metadata.name, expected.metadata.name);
    assertEqual("metadata symbol", metadata.symbol, expected.metadata.symbol);
    assertEqual("metadata URI", metadata.uri, expected.metadata.uri);
    assertEqual("additional metadata count", metadata.additionalMetadata.length, 0);

    return { tlvData };
}
