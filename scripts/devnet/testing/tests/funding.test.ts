import assert from "node:assert/strict";
import test from "node:test";

import type { RuntimeBanksClient } from "../../../../runtime/harness.js";
import { SYSTEM_PROGRAM } from "../../../../runtime/wellKnown.js";
import { generateSigner } from "../../../deployment/lib/solanaSigner.js";
import { fundTesterWallet } from "../funding/fund.js";
import type { TesterFundingServices } from "../funding/types.js";

type WalletAccount = Awaited<ReturnType<RuntimeBanksClient["getAccount"]>>;
interface FundingFixture {
    readonly services: TesterFundingServices;
    readonly requested: bigint[];
    readonly reads: () => number;
}

const signer = generateSigner();
const wallet = signer.publicKey;
signer.seed.fill(0);
const request = { minimumLamports: 100_000_000n, allowAirdrop: true };

function account(lamports: bigint): NonNullable<WalletAccount> {
    return { data: new Uint8Array(), owner: SYSTEM_PROGRAM, lamports, executable: false };
}

function fixture(before: WalletAccount, after: WalletAccount = account(request.minimumLamports)): FundingFixture {
    const requested: bigint[] = [];
    let reads = 0;
    return {
        requested,
        reads: () => reads,
        services: {
            banksClient: {
                getAccount: async (publicKey) => {
                    assert.equal(publicKey, wallet);
                    return reads++ === 0 ? before : after;
                },
                getMinimumBalanceForRentExemption: async (length) => { assert.equal(length, 0); return 890_880n; },
            },
            requestAirdrop: async (recipient, lamports) => {
                assert.equal(recipient, wallet);
                requested.push(lamports);
                return "confirmed-airdrop";
            },
        },
    };
}

test("an absent tester wallet receives the requested SOL target and is read back", async () => {
    const scenario = fixture(null);
    assert.equal(await fundTesterWallet(scenario.services, wallet, request), request.minimumLamports);
    assert.deepEqual(scenario.requested, [request.minimumLamports]);
    assert.equal(scenario.reads(), 2);
});

test("funding tops up only the exact deficit", async () => {
    const scenario = fixture(account(40_000_001n));
    await fundTesterWallet(scenario.services, wallet, request);
    assert.deepEqual(scenario.requested, [59_999_999n]);
});

test("a sufficiently funded wallet performs no airdrop or second read", async () => {
    const scenario = fixture(account(120_000_000n));
    assert.equal(await fundTesterWallet(scenario.services, wallet, { ...request, allowAirdrop: false }), 120_000_000n);
    assert.deepEqual(scenario.requested, []);
    assert.equal(scenario.reads(), 1);
});

test("skip-airdrop rejects insufficient SOL without requesting funding", async () => {
    const scenario = fixture(null);
    await assert.rejects(() => fundTesterWallet(scenario.services, wallet, { ...request, allowAirdrop: false }), /requires at least/);
    assert.deepEqual(scenario.requested, []);
});

test("confirmed airdrop with insufficient balance fails readback", async () => {
    const scenario = fixture(null, account(99_999_999n));
    await assert.rejects(() => fundTesterWallet(scenario.services, wallet, request), /funding readback/);
    assert.equal(scenario.requested.length, 1);
});

test("concurrent extra funding is accepted when the requested target is met", async () => {
    const scenario = fixture(null, account(110_000_000n));
    assert.equal(await fundTesterWallet(scenario.services, wallet, request), 110_000_000n);
});

test("an airdrop failure is preserved as the cause and is not retried", async () => {
    const scenario = fixture(null);
    const failure = new Error("RPC rate limit");
    let requests = 0;
    const services: TesterFundingServices = { ...scenario.services, requestAirdrop: async () => { requests++; throw failure; } };
    await assert.rejects(() => fundTesterWallet(services, wallet, request), (error: unknown) => {
        assert.ok(error instanceof Error);
        assert.equal(error.cause, failure);
        assert.match(error.message, /--skip-airdrop/);
        return true;
    });
    assert.equal(requests, 1);
    assert.equal(scenario.reads(), 1);
});

test("data-bearing, executable, wrong-owner, and unknown-owner accounts are rejected", async () => {
    for (const invalid of [
        { ...account(1n), data: new Uint8Array(1) },
        { ...account(1n), executable: true },
        { ...account(1n), owner: wallet },
        { data: new Uint8Array(), lamports: 1n },
    ]) {
        const scenario = fixture(invalid);
        await assert.rejects(() => fundTesterWallet(scenario.services, wallet, request), /system-owned, zero-data/);
        assert.deepEqual(scenario.requested, []);
    }
});

test("wallet ownership is rechecked after an airdrop", async () => {
    const scenario = fixture(null, { ...account(100_000_000n), owner: wallet });
    await assert.rejects(() => fundTesterWallet(scenario.services, wallet, request), /system-owned, zero-data/);
});

test("rent-inadequate funding is rejected before requesting SOL", async () => {
    const scenario = fixture(null);
    await assert.rejects(() => fundTesterWallet(scenario.services, wallet, { ...request, minimumLamports: 890_880n }), /exceed zero-data account rent/);
    assert.deepEqual(scenario.requested, []);
});

test("a runtime without rent support fails before requesting SOL", async () => {
    const scenario = fixture(null);
    const services: TesterFundingServices = { ...scenario.services, banksClient: { getAccount: scenario.services.banksClient.getAccount } };
    await assert.rejects(() => fundTesterWallet(services, wallet, request), /cannot calculate/);
    assert.deepEqual(scenario.requested, []);
});

test("invalid RPC lamport quantities are rejected before reading accounts", async () => {
    for (const minimumLamports of [0n, -1n, 9_007_199_254_740_992n]) {
        const scenario = fixture(null);
        await assert.rejects(() => fundTesterWallet(scenario.services, wallet, { ...request, minimumLamports }), /exactly representable/);
        assert.equal(scenario.reads(), 0);
    }
});
