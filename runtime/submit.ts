/**
 * Sign and submit transactions against LiteSVM using Solomon libraries only.
 */

import { Base58Util } from "@solomon-labs/base58";
import { CryptoUtil } from "@solomon-labs/crypto";
import { PublicKey } from "@solomon-labs/publickey";
import type {
  IsEncodable,
  EncodedInstruction,
  InstructionAccount,
  SolanaAddress,
  Base58,
  SolanaBlockhash,
} from "@solomon-labs/solana-codec";
import { Transaction } from "@solomon-labs/solana-transactions";
import type { PublicKeyLike } from "@solomon-labs/types";

import { publicKeyToBase58 } from "./publicKey.js";
import { makeLookupTableAccount } from "./lookupTable.js";
import type { RuntimeHarness } from "./harness.js";
import {
  generateSigner,
  signerFromSeed,
  signerFromSolanaKeypairBytes,
  type E2ESigner,
  type SolanaSeedSigner,
} from "../scripts/deployment/lib/solanaSigner.js";

export { generateSigner, signerFromSeed, signerFromSolanaKeypairBytes };
export type E2ESeedSigner = SolanaSeedSigner;
export type { E2ESigner };

export type SubmitTransactionContext =
  Pick<RuntimeHarness, "banksClient" | "lookupTables">;

/** Maximum serialized size of a legacy Solana transaction. */
const MAX_LEGACY_TRANSACTION_BYTES = 1232;

/**
 * Collect the unique non-signer account keys across `instructions`, skipping
 * anything in `excluded` (fee payer, signers, and instruction program ids -
 * all of which must remain static keys in a v0 message).
 */
function collectLookupTableAddresses(
  instructions: readonly IsEncodable[],
  excluded: ReadonlySet<string>,
): PublicKey[] {
  const seen = new Set<string>();
  const addresses: PublicKey[] = [];
  for (let i = 0, len = instructions.length; i < len; i++) {
    const instruction = instructions[i];
    if (instruction === undefined) {
      continue;
    }
    const accounts = instruction.accounts();
    for (let j = 0, accountsLen = accounts.length; j < accountsLen; j++) {
      const account = accounts[j];
      if (account === undefined || account.isSigner) {
        continue;
      }
      const base58 = publicKeyToBase58(account.pubkey);
      if (excluded.has(base58) || seen.has(base58)) {
        continue;
      }
      seen.add(base58);
      addresses.push(new PublicKey(base58 as SolanaAddress));
    }
  }
  return addresses;
}

export async function signGeneratedTransaction(
  transaction: Transaction,
  ...signers: SolanaSeedSigner[]
): Promise<void> {
  for (
    let signerIndex = 0, len = signers.length;
    signerIndex < len;
    signerIndex++
  ) {
    const signer = signers[signerIndex];
    const signature = await CryptoUtil.signEd25519(
      transaction.serializeMessage(),
      {
        format: "raw-seed",
        key: signer.seed,
      },
    );
    transaction.addSignature(signer.publicKey, signature);
  }
}

export const signSolomonTransaction = signGeneratedTransaction;

class EncodedInstructionAdapter implements IsEncodable {
  readonly programId: PublicKeyLike;
  private readonly encodedInstruction: EncodedInstruction;

  constructor(encodedInstruction: EncodedInstruction) {
    const data =
      typeof encodedInstruction.data === "string"
        ? Base58Util.decode(encodedInstruction.data as Base58)
        : encodedInstruction.data;
    this.programId = encodedInstruction.programId;
    this.encodedInstruction = {
      programId: encodedInstruction.programId,
      accounts: encodedInstruction.accounts,
      data,
    };
  }

  encode(): Uint8Array {
    return this.encodedInstruction.data;
  }

  accounts(): InstructionAccount[] {
    return this.encodedInstruction.accounts;
  }

  instruction(): EncodedInstruction {
    return this.encodedInstruction;
  }
}

function isEncodableInstruction(value: unknown): value is IsEncodable {
  return (
    value !== null &&
    typeof value === "object" &&
    "instruction" in value &&
    typeof (value as { instruction?: unknown }).instruction === "function"
  );
}

function isEncodedInstructionLike(
  value: unknown,
): value is EncodedInstruction {
  return (
    value !== null &&
    typeof value === "object" &&
    "programId" in value &&
    "accounts" in value &&
    "data" in value
  );
}

function normalizeInstruction(
  instruction: IsEncodable | EncodedInstruction,
): IsEncodable {
  if (isEncodableInstruction(instruction)) {
    return instruction;
  }
  if (isEncodedInstructionLike(instruction)) {
    return new EncodedInstructionAdapter(instruction);
  }
  throw new Error("Unsupported instruction shape");
}

export async function submitTransaction(
  ctx: SubmitTransactionContext,
  instructions: Array<IsEncodable | EncodedInstruction>,
  signers: SolanaSeedSigner[],
  payer: E2ESeedSigner,
  options: { simulateOnly?: boolean; onSignature?: (signature: string) => void } = {},
): Promise<readonly string[]> {
  const blockhashTuple = await ctx.banksClient.getLatestBlockhash();
  const blockhash = blockhashTuple?.[0];
  if (!blockhash) {
    throw new Error("Could not get blockhash");
  }

  const normalizedInstructions: IsEncodable[] = [];
  for (
    let instructionIndex = 0, len = instructions.length;
    instructionIndex < len;
    instructionIndex++
  ) {
    normalizedInstructions.push(
      normalizeInstruction(instructions[instructionIndex]),
    );
  }

  const allSigners: SolanaSeedSigner[] = [payer];
  for (
    let signerIndex = 0, len = signers.length;
    signerIndex < len;
    signerIndex++
  ) {
    const signer = signers[signerIndex];
    if (signer !== payer) {
      allSigners.push(signer);
    }
  }

  // Build the legacy transaction and measure it. Small transactions keep the
  // legacy path unchanged; only oversized ones (settlement carries up to 40
  // accounts) switch to a v0 transaction with an ALT so the wire fits under the
  // 1232-byte legacy limit. The switch never touches transactions that already
  // fit, so it cannot regress the passing suite.
  let transaction = new Transaction({
    feePayer: payer.publicKey,
    recentBlockhash: blockhash as SolanaBlockhash,
    instructions: normalizedInstructions,
  });

  // The transaction library throws (rather than returning bytes) when a legacy
  // transaction exceeds the size limit, so a throw means "oversized" too.
  let legacyTooLarge: boolean;
  try {
    legacyTooLarge = transaction.serialize(false).length > MAX_LEGACY_TRANSACTION_BYTES;
  } catch {
    legacyTooLarge = true;
  }

  if (legacyTooLarge && ctx.lookupTables !== undefined) {
    const excluded = new Set<string>();
    for (
      let signerIndex = 0, len = allSigners.length;
      signerIndex < len;
      signerIndex++
    ) {
      excluded.add(publicKeyToBase58(allSigners[signerIndex].publicKey));
    }
    for (
      let instructionIndex = 0, len = normalizedInstructions.length;
      instructionIndex < len;
      instructionIndex++
    ) {
      const instruction = normalizedInstructions[instructionIndex];
      if (instruction !== undefined) {
        excluded.add(publicKeyToBase58(instruction.instruction().programId));
      }
    }

    const addresses = collectLookupTableAddresses(normalizedInstructions, excluded);
    if (addresses.length > 0) {
      const table = makeLookupTableAccount(
        generateSigner().publicKey,
        addresses,
        payer.publicKey,
      );
      ctx.lookupTables.registerLookupTable(table);
      // Registration may advance the slot to activate the table; refresh the
      // blockhash so the versioned transaction references a live one.
      const refreshedTuple = await ctx.banksClient.getLatestBlockhash();
      const refreshedBlockhash = refreshedTuple?.[0] ?? blockhash;
      transaction = new Transaction({
        feePayer: payer.publicKey,
        recentBlockhash: refreshedBlockhash as SolanaBlockhash,
        instructions: normalizedInstructions,
      });
      transaction
        .enableLookupTables()
        .setLookupTableMode("external")
        .addLookupTableAccounts(table);
    }
  }

  await signGeneratedTransaction(transaction, ...allSigners);
  if (options.onSignature !== undefined) {
    const firstSignature = (transaction as unknown as { signatures?: ReadonlyArray<Uint8Array | string | { signature?: Uint8Array | null } | null> }).signatures?.[0];
    // Depending on the transaction backend, signatures[0] is raw bytes, an
    // object wrapping raw bytes, or an already-base58 string.
    if (typeof firstSignature === "string" && firstSignature.length > 0) {
      options.onSignature(firstSignature);
    } else {
      const raw = firstSignature instanceof Uint8Array
        ? firstSignature
        : firstSignature && typeof firstSignature === "object" && "signature" in firstSignature && firstSignature.signature instanceof Uint8Array
          ? firstSignature.signature
          : null;
      if (raw !== null) {
        options.onSignature(Base58Util.encode(raw));
      }
    }
  }

  // Simulate the exact signed transaction before sending whenever the backing
  // runtime supports it. Simulation is read-only and catches instruction,
  // account-layout, compute, and signature failures without changing state.
  const simulationLogs = await ctx.banksClient.simulateTransaction?.(transaction);
  if (options.simulateOnly === true) {
    if (ctx.banksClient.simulateTransaction === undefined) {
      throw new Error("simulate-only requested, but the runtime does not support transaction simulation");
    }
    return simulationLogs ? [...simulationLogs] : [];
  }

  const logs = await ctx.banksClient.processTransaction(transaction);
  return logs ? [...logs] : [];
}
