import { Base58Util } from "@solomon-labs/base58";
import { RpcClient } from "@solomon-labs/combined";
import { PublicKey } from "@solomon-labs/publickey";
import type { Base58, Transaction } from "@solomon-labs/solana-transactions";
import type { PublicKeyLike } from "@solomon-labs/types";

import { resolveNetwork } from "../config/network.js";
import { DEVNET_GENESIS_HASH } from "../config/devnetGenesis.js";
import { PROGRAM_ID } from "../clients/ts/src/constants.js";
import { isCanonicalChanceryEventInstruction } from "./eventProvenance.js";
import { publicKeyToBytes } from "./publicKey.js";
import { eventAuthorityPda } from "./pdas.js";
import type { RuntimeBanksClient, RuntimeClock, RuntimeHarness } from "./harness.js";

const BASE64_ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const CHANCERY_INNER_INSTRUCTION_DATA_PREFIX = "Chancery inner instruction data: ";
// Resolved lazily and only when a caller omits rpcUrl: eager module-level
// resolution pinned to "devnet" broke every non-devnet ceremony import
// (resolveNetwork reads --config from argv and asserts the file's network).
let configuredDevnetRpcUrlCache: string | null = null;
function configuredDevnetRpcUrl(): string {
    configuredDevnetRpcUrlCache ??= resolveNetwork("devnet").rpcUrl;
    return configuredDevnetRpcUrlCache;
}
const BPF_LOADER_UPGRADEABLE = new PublicKey("BPFLoaderUpgradeab1e11111111111111111111111");

interface RpcAccountInfoValue {
  data: [string, string] | string;
  executable: boolean;
  lamports: number;
  owner: string;
  rentEpoch: number;
}

interface RpcGetAccountInfoResult {
  context: { slot: number };
  value: RpcAccountInfoValue | null;
}

interface RpcSimulateTransactionResult {
  context: { slot: number };
  value: {
    err: unknown | null;
    logs?: string[] | null;
  };
}

interface RpcGetLatestBlockhashResult {
  context: { slot: number };
  value: {
    blockhash: string;
    lastValidBlockHeight: number;
  };
}

interface RpcSignatureStatus {
  confirmationStatus?: "processed" | "confirmed" | "finalized";
  confirmations?: number | null;
  err?: unknown;
  slot: number;
}

interface RpcGetSignatureStatusesResult {
  context: { slot: number };
  value: Array<RpcSignatureStatus | null>;
}

interface RpcInnerInstruction {
  accounts?: Array<number | string>;
  data?: string;
  programId?: string;
  programIdIndex?: number;
}

interface RpcInnerInstructionGroup {
  index: number;
  instructions: RpcInnerInstruction[];
}

interface RpcTransactionAccountKey {
  pubkey: string;
}

interface RpcGetTransactionResult {
  slot: number;
  transaction: {
    message?: {
      accountKeys?: Array<string | RpcTransactionAccountKey>;
    };
  };
  meta?: {
    err?: unknown;
    logMessages?: string[] | null;
    innerInstructions?: RpcInnerInstructionGroup[] | null;
  } | null;
}

interface RpcGetEpochInfoResult {
  absoluteSlot: number;
  blockHeight: number;
  epoch: number;
  slotIndex: number;
  slotsInEpoch: number;
}

export interface CreateDevnetHarnessOptions {
  rpcUrl?: string;
  secondaryRpcUrl?: string;
  commitment?: "processed" | "confirmed" | "finalized";
  requireProgramdata?: boolean;
}

function toBase64(bytes: Uint8Array): string {
  let encoded = "";
  for (let i = 0, len = bytes.length; i < len; i += 3) {
    const a = bytes[i] ?? 0;
    const b = bytes[i + 1] ?? 0;
    const c = bytes[i + 2] ?? 0;
    const triple = (a << 16) | (b << 8) | c;
    encoded += BASE64_ALPHABET[(triple >> 18) & 63];
    encoded += BASE64_ALPHABET[(triple >> 12) & 63];
    encoded += i + 1 < len ? BASE64_ALPHABET[(triple >> 6) & 63] : "=";
    encoded += i + 2 < len ? BASE64_ALPHABET[triple & 63] : "=";
  }
  return encoded;
}

function fromBase64(text: string): Uint8Array {
  const clean = text.replace(/[\r\n\t ]/g, "");
  if (clean.length % 4 !== 0) {
    throw new Error("base64 length must be divisible by 4");
  }
  let padding = 0;
  if (clean.endsWith("==")) {
    padding = 2;
  } else if (clean.endsWith("=")) {
    padding = 1;
  }
  const out = new Uint8Array((clean.length / 4) * 3 - padding);
  let outIndex = 0;
  for (let i = 0, len = clean.length; i < len; i += 4) {
    const a = BASE64_ALPHABET.indexOf(clean[i] ?? "");
    const b = BASE64_ALPHABET.indexOf(clean[i + 1] ?? "");
    const cChar = clean[i + 2] ?? "=";
    const dChar = clean[i + 3] ?? "=";
    const c = cChar === "=" ? 0 : BASE64_ALPHABET.indexOf(cChar);
    const d = dChar === "=" ? 0 : BASE64_ALPHABET.indexOf(dChar);
    if (a < 0 || b < 0 || (cChar !== "=" && c < 0) || (dChar !== "=" && d < 0)) {
      throw new Error("invalid base64 character");
    }
    const triple = (a << 18) | (b << 12) | (c << 6) | d;
    if (outIndex < out.length) {
      out[outIndex++] = (triple >> 16) & 0xff;
    }
    if (outIndex < out.length) {
      out[outIndex++] = (triple >> 8) & 0xff;
    }
    if (outIndex < out.length) {
      out[outIndex++] = triple & 0xff;
    }
  }
  return out;
}

function publicKeyString(publicKey: PublicKeyLike): string {
  return new PublicKey(publicKeyToBytes(publicKey)).toBase58();
}

function accountKeyString(value: string | RpcTransactionAccountKey | undefined): string | undefined {
  if (typeof value === "string") {
    return value;
  }
  return value?.pubkey;
}

function appendInnerInstructionDataLines(
  logs: string[],
  transaction: RpcGetTransactionResult,
  expectedEventAuthority: PublicKey,
): string[] {
  const lines = [...logs];
  const messageAccountKeys = transaction.transaction.message?.accountKeys ?? [];
  const innerInstructions = transaction.meta?.innerInstructions ?? [];
  for (let outerIndex = 0, outerLen = innerInstructions.length; outerIndex < outerLen; outerIndex++) {
    const innerGroup = innerInstructions[outerIndex];
    if (innerGroup === undefined) {
      continue;
    }
    const instructions = innerGroup.instructions;
    for (let innerIndex = 0, innerLen = instructions.length; innerIndex < innerLen; innerIndex++) {
      const innerInstruction = instructions[innerIndex];
      const dataString = innerInstruction?.data;
      if (dataString === undefined || dataString.length === 0) {
        continue;
      }
      const data = Base58Util.decode(dataString as Base58);
      const programId = innerInstruction.programId
        ?? accountKeyString(
          innerInstruction.programIdIndex === undefined
            ? undefined
            : messageAccountKeys[innerInstruction.programIdIndex],
        );
      if (programId === undefined) {
        continue;
      }

      const accountPublicKeys: string[] = [];
      const rawAccounts = innerInstruction.accounts ?? [];
      for (let accountIndex = 0, accountLen = rawAccounts.length; accountIndex < accountLen; accountIndex++) {
        const rawAccount = rawAccounts[accountIndex];
        if (typeof rawAccount === "string") {
          accountPublicKeys.push(rawAccount);
        } else if (typeof rawAccount === "number") {
          const accountPublicKey = accountKeyString(messageAccountKeys[rawAccount]);
          if (accountPublicKey !== undefined) {
            accountPublicKeys.push(accountPublicKey);
          }
        }
      }

      if (!isCanonicalChanceryEventInstruction({
        programId,
        accountPublicKeys,
        data,
      }, expectedEventAuthority)) {
        continue;
      }
      lines.push(`${CHANCERY_INNER_INSTRUCTION_DATA_PREFIX}${toBase64(data)}`);
    }
  }
  return lines;
}

function sleep(milliseconds: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, milliseconds));
}

async function pollSignature(
  rpc: RpcClient,
  signature: string,
  commitment: "processed" | "confirmed" | "finalized",
): Promise<void> {
  for (let attempt = 0; attempt < 90; attempt++) {
    const statusResult = await rpc.request<RpcGetSignatureStatusesResult>("getSignatureStatuses", [
      [signature],
      { searchTransactionHistory: true },
    ]);
    const status = statusResult.value[0];
    if (status?.err !== undefined && status.err !== null) {
      throw new Error(`transaction ${signature} failed: ${JSON.stringify(status.err)}`);
    }
    if (status?.confirmationStatus === "finalized") {
      return;
    }
    if (commitment !== "finalized" && status?.confirmationStatus === "confirmed") {
      return;
    }
    if (commitment === "processed" && status !== null) {
      return;
    }
    await sleep(1_000);
  }
  throw new Error(`transaction ${signature} was not confirmed before timeout`);
}

async function fetchTransactionLogs(
  rpc: RpcClient,
  signature: string,
  commitment: "processed" | "confirmed" | "finalized",
): Promise<readonly string[]> {
  for (let attempt = 0; attempt < 30; attempt++) {
    const transaction = await rpc.request<RpcGetTransactionResult | null>("getTransaction", [
      signature,
      {
        commitment,
        encoding: "json",
        maxSupportedTransactionVersion: 0,
      },
    ]);
    if (transaction !== null) {
      const logs = transaction.meta?.logMessages ?? [];
      const [expectedEventAuthority] = await eventAuthorityPda();
      return appendInnerInstructionDataLines(logs, transaction, expectedEventAuthority);
    }
    await sleep(1_000);
  }
  return [];
}

async function readProgramdataAddress(banksClient: RuntimeBanksClient): Promise<PublicKey> {
  const account = await banksClient.getAccount(PROGRAM_ID);
  if (account === null) {
    throw new Error(`deployed Chancery program account not found: ${PROGRAM_ID.toBase58()}`);
  }
  if (account.data.length < 36) {
    throw new Error(`deployed program account data too short: ${account.data.length}`);
  }
  return new PublicKey(account.data.subarray(4, 36));
}

function equalBytes(left: Uint8Array, right: Uint8Array): boolean {
  if (left.length !== right.length) return false;
  for (let index = 0, length = left.length; index < length; index += 1) {
    if (left[index] !== right[index]) return false;
  }
  return true;
}

async function getRpcAccount(
  rpc: RpcClient,
  pubkey: PublicKeyLike,
  commitment: "processed" | "confirmed" | "finalized",
): Promise<{ data: Uint8Array; lamports: bigint; owner: PublicKey; executable: boolean } | null> {
  const result = await rpc.request<RpcGetAccountInfoResult>("getAccountInfo", [
    publicKeyString(pubkey),
    { commitment, encoding: "base64" },
  ]);
  if (result.value === null) return null;
  // A zero-lamport account does not exist: the runtime purges it at the end of
  // the transaction that drains it. Providers differ on how they report the
  // moment after closure - some return null immediately, others serve a
  // zero-lamport snapshot until they reap it. Both describe the same state, so
  // normalise here rather than letting the difference surface as a disagreement
  // between independent RPCs. Ceremony changes are applied through PDAs that
  // close on success, so this is the common case, not an edge case.
  if (BigInt(result.value.lamports) === 0n) return null;
  const accountData = result.value.data;
  const dataString = Array.isArray(accountData) ? accountData[0] : accountData;
  return {
    data: fromBase64(dataString),
    lamports: BigInt(result.value.lamports),
    executable: result.value.executable,
    owner: new PublicKey(Base58Util.decode(result.value.owner as Base58)),
  };
}

function assertMatchingAccounts(
  address: string,
  primary: { data: Uint8Array; lamports: bigint; owner: PublicKey; executable: boolean } | null,
  secondary: { data: Uint8Array; lamports: bigint; owner: PublicKey; executable: boolean } | null,
): void {
  if (primary === null || secondary === null) {
    if (primary !== secondary) throw new Error(`independent RPCs disagree on account existence for ${address}`);
    return;
  }
  if (primary.lamports !== secondary.lamports) {
    throw new Error(`independent RPCs disagree on lamports for ${address}`);
  }
  if (primary.executable !== secondary.executable) {
    throw new Error(`independent RPCs disagree on executable state for ${address}`);
  }
  if (!primary.owner.equals(secondary.owner)) {
    throw new Error(`independent RPCs disagree on owner for ${address}`);
  }
  if (!equalBytes(primary.data, secondary.data)) {
    throw new Error(`independent RPCs disagree on account data for ${address}`);
  }
}

async function sendExactWire(
  rpc: RpcClient,
  wireBase64: string,
  commitment: "processed" | "confirmed" | "finalized",
): Promise<string> {
  const signature = await rpc.sendTransaction(wireBase64, {
    encoding: "base64",
    skipPreflight: false,
    preflightCommitment: commitment,
    maxRetries: 3,
  });
  if (signature === null) throw new Error("RPC sendTransaction returned no transaction signature");
  return signature;
}

export async function createDevnetHarness(options: CreateDevnetHarnessOptions = {}): Promise<RuntimeHarness> {
  const rpcUrl = options.rpcUrl ?? configuredDevnetRpcUrl();
  const commitment = options.commitment ?? "confirmed";
  const rpc = new RpcClient(
    rpcUrl as ConstructorParameters<typeof RpcClient>[0],
  );
  const secondaryRpc = options.secondaryRpcUrl === undefined
    ? undefined
    : new RpcClient(options.secondaryRpcUrl as ConstructorParameters<typeof RpcClient>[0]);

  const genesis = await rpc.request<string>("getGenesisHash", []);
  if (genesis !== DEVNET_GENESIS_HASH) throw new Error("RPC endpoint is not Solana devnet");
  if (secondaryRpc !== undefined) {
    const secondaryGenesis = await secondaryRpc.request<string>("getGenesisHash", []);
    if (secondaryGenesis !== DEVNET_GENESIS_HASH) throw new Error("secondary RPC endpoint is not Solana devnet");
  }

  const banksClient: RuntimeBanksClient = {
    async getLatestBlockhash() {
      const result = await rpc.request<RpcGetLatestBlockhashResult>("getLatestBlockhash", [
        { commitment },
      ]);
      if (secondaryRpc !== undefined) {
        const validity = await secondaryRpc.request<{ context: { slot: number }; value: boolean }>("isBlockhashValid", [
          result.value.blockhash,
          { commitment },
        ]);
        if (!validity.value) throw new Error("secondary RPC rejected the primary RPC recent blockhash");
      }
      return [result.value.blockhash, commitment];
    },
    async processTransaction(transaction: Transaction) {
      const raw = transaction.serialize(false);
      const wire = raw instanceof Uint8Array ? raw : new Uint8Array(raw);
      const wireBase64 = toBase64(wire);
      const primarySignature = await sendExactWire(rpc, wireBase64, commitment);
      if (secondaryRpc !== undefined) {
        let secondarySignature: string;
        try {
          secondarySignature = await sendExactWire(secondaryRpc, wireBase64, commitment);
        } catch (error: unknown) {
          const message = error instanceof Error ? error.message : String(error);
          if (!message.toLowerCase().includes("already processed")) throw error;
          secondarySignature = primarySignature;
        }
        if (secondarySignature !== primarySignature) {
          throw new Error("independent RPCs returned different signatures for identical transaction bytes");
        }
        await Promise.all([
          pollSignature(rpc, primarySignature, commitment),
          pollSignature(secondaryRpc, primarySignature, commitment),
        ]);
      } else {
        await pollSignature(rpc, primarySignature, commitment);
      }
      return fetchTransactionLogs(rpc, primarySignature, commitment);
    },
    async simulateTransaction(transaction: Transaction) {
      const raw = transaction.serialize(false);
      const wire = raw instanceof Uint8Array ? raw : new Uint8Array(raw);
      const result = await rpc.request<RpcSimulateTransactionResult>("simulateTransaction", [
        toBase64(wire),
        {
          commitment,
          encoding: "base64",
          replaceRecentBlockhash: false,
          sigVerify: true,
        },
      ]);
      const logs = result.value.logs ?? [];
      if (result.value.err !== null) {
        const suffix = logs.length > 0 ? `\n${logs.join("\n")}` : "";
        throw new Error(`transaction simulation failed: ${JSON.stringify(result.value.err)}${suffix}`);
      }
      if (secondaryRpc !== undefined) {
        const secondaryResult = await secondaryRpc.request<RpcSimulateTransactionResult>("simulateTransaction", [
          toBase64(wire),
          {
            commitment,
            encoding: "base64",
            replaceRecentBlockhash: false,
            sigVerify: true,
          },
        ]);
        const secondaryLogs = secondaryResult.value.logs ?? [];
        if (secondaryResult.value.err !== null) {
          const suffix = secondaryLogs.length > 0 ? `\n${secondaryLogs.join("\n")}` : "";
          throw new Error(`secondary RPC transaction simulation failed: ${JSON.stringify(secondaryResult.value.err)}${suffix}`);
        }
      }
      return logs;
    },
    async getAccount(pubkey: PublicKeyLike) {
      const primary = await getRpcAccount(rpc, pubkey, commitment);
      if (secondaryRpc !== undefined) {
        const secondary = await getRpcAccount(secondaryRpc, pubkey, commitment);
        assertMatchingAccounts(publicKeyString(pubkey), primary, secondary);
      }
      return primary;
    },
    async getClock(): Promise<RuntimeClock> {
      const [slot, epochInfo] = await Promise.all([
        rpc.request<number>("getSlot", [{ commitment }]),
        rpc.request<RpcGetEpochInfoResult>("getEpochInfo", [{ commitment }]),
      ]);
      const blockTime = await rpc.request<number | null>("getBlockTime", [slot]);
      let conservativeSlot = slot;
      let conservativeTimestamp = blockTime ?? Math.floor(Date.now() / 1000);
      if (secondaryRpc !== undefined) {
        const secondarySlot = await secondaryRpc.request<number>("getSlot", [{ commitment }]);
        const slotDifference = Math.abs(slot - secondarySlot);
        if (slotDifference > 512) {
          throw new Error(`independent RPC finalized slots differ by ${slotDifference}, exceeding the ceremony limit`);
        }
        conservativeSlot = Math.min(slot, secondarySlot);
        const secondaryBlockTime = await secondaryRpc.request<number | null>("getBlockTime", [conservativeSlot]);
        if (secondaryBlockTime !== null) conservativeTimestamp = Math.min(conservativeTimestamp, secondaryBlockTime);
      }
      return {
        slot: BigInt(conservativeSlot),
        epochStartTimestamp: 0n,
        epoch: BigInt(epochInfo.epoch),
        leaderScheduleEpoch: BigInt(epochInfo.epoch),
        unixTimestamp: BigInt(conservativeTimestamp),
      };
    },
    async getMinimumBalanceForRentExemption(dataLength: number) {
      const lamports = await rpc.request<number>("getMinimumBalanceForRentExemption", [dataLength, { commitment }]);
      if (secondaryRpc !== undefined) {
        const secondaryLamports = await secondaryRpc.request<number>("getMinimumBalanceForRentExemption", [
          dataLength,
          { commitment },
        ]);
        if (secondaryLamports !== lamports) {
          throw new Error(`independent RPCs disagree on rent exemption for ${dataLength} bytes`);
        }
      }
      return BigInt(lamports);
    },
  };

  const programdataAddress = options.requireProgramdata === false
    ? undefined
    : await readProgramdataAddress(banksClient);

  return {
    banksClient,
    setClock() {
      throw new Error("devnet runtime cannot mutate the on-chain clock; use LiteSVM for time-travel tests");
    },
    programdataAddress,
    bpfLoaderUpgradeable: BPF_LOADER_UPGRADEABLE,
  };
}

export async function requestAirdropAndConfirm(
  publicKey: PublicKeyLike,
  lamports: bigint,
  options: CreateDevnetHarnessOptions = {},
): Promise<string> {
  const rpcUrl = options.rpcUrl ?? configuredDevnetRpcUrl();
  const commitment = options.commitment ?? "confirmed";
  const rpc = new RpcClient(rpcUrl as ConstructorParameters<typeof RpcClient>[0]);
  const signature = await rpc.request<string>("requestAirdrop", [publicKeyString(publicKey), Number(lamports)]);
  const secondaryRpc = options.secondaryRpcUrl === undefined
    ? null
    : new RpcClient(options.secondaryRpcUrl as ConstructorParameters<typeof RpcClient>[0]);
  await Promise.all([
    pollSignature(rpc, signature, commitment),
    secondaryRpc === null ? Promise.resolve() : pollSignature(secondaryRpc, signature, commitment),
  ]);
  return signature;
}
