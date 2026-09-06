/**
 * The runtime contract the submit path is written against.
 *
 * Extracted from the LiteSVM harness of the upstream repository so the devnet
 * RPC adapter can implement it without carrying the in-process test VM. The
 * shape is unchanged: optional members are the ones a devnet runtime does not
 * provide.
 */

import type { PublicKey } from "@solomon-labs/publickey";
import type { AddressLookupTableAccount, Transaction } from "@solomon-labs/solana-transactions";
import type { PublicKeyLike } from "@solomon-labs/types";

export interface RuntimeClock {
    slot:                bigint;
    epochStartTimestamp: bigint;
    epoch:               bigint;
    leaderScheduleEpoch: bigint;
    unixTimestamp:       bigint;
}

export interface RuntimeBanksClient {
    getLatestBlockhash(): Promise<[string, string] | undefined>;
    simulateTransaction?(transaction: Transaction): Promise<readonly string[] | undefined>;
    processTransaction(transaction: Transaction): Promise<readonly string[] | undefined>;
    getAccount(pubkey: PublicKeyLike): Promise<{
        data: Uint8Array;
        lamports: bigint;
        owner?: PublicKey;
        executable?: boolean;
    } | null>;
    getClock(): Promise<RuntimeClock>;
    getMinimumBalanceForRentExemption?(dataLength: number): Promise<bigint>;
}

export interface RuntimeLookupTableSupport {
    /** The native Address Lookup Table program id (owner of every ALT). */
    programId: PublicKey;
    /** Serialize `table` into on-chain bytes and install it at its key so the
     *  runtime can resolve v0 address-table lookups. */
    registerLookupTable(table: AddressLookupTableAccount): void;
}

export interface RuntimeHarness {
    banksClient: RuntimeBanksClient;
    setClock(partial: Partial<RuntimeClock>): void;
    /** ALT registration used by the submit path to compress oversized (v0)
     *  transactions below the legacy 1232-byte limit. Absent on runtimes (e.g.
     *  devnet) where lookup tables must be created on-chain instead. */
    lookupTables?: RuntimeLookupTableSupport;
    setAccount?(
        pubkey: PublicKeyLike,
        info: { lamports: bigint; data: Uint8Array; owner: PublicKeyLike; executable?: boolean },
    ): void;
    /** Pubkey of the BPF loader-upgradeable ProgramData account holding the
     *  chancery bytecode. `initialize_chancery` needs this. */
    programdataAddress?: PublicKey;
    /** Pubkey of the BPF loader-upgradeable program. */
    bpfLoaderUpgradeable?: PublicKey;
}
