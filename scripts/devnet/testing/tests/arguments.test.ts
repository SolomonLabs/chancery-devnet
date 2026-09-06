import assert from "node:assert/strict";
import test from "node:test";

import { parseTesterArguments } from "../arguments.js";

test("tester defaults select all collateral and a separate reusable workflow wallet", () => {
    assert.deepEqual(parseTesterArguments(["devnet"], "integration"), {
        keypairPath: null, runtimeArguments: [], reuseWallet: false, mode: "all", symbol: "all",
        amount: undefined, minimumLamports: 100_000_000n, allowAirdrop: true,
    });
});

test("tester options preserve exact bigint amounts and paths with spaces", () => {
    const options = parseTesterArguments(["devnet", "--keypair", "wallets/my wallet.json", "--config", "config/runtime/custom.json",
        "--symbol", "USDG", "--amount", "9007199254740993", "--lamports", "9007199254740991", "--skip-airdrop"], "integration");
    assert.equal(options.keypairPath, "wallets/my wallet.json");
    assert.deepEqual(options.runtimeArguments, ["--config", "config/runtime/custom.json"]);
    assert.equal(options.symbol, "USDG");
    assert.equal(options.amount, 9_007_199_254_740_993n);
    assert.equal(options.minimumLamports, 9_007_199_254_740_991n);
    assert.equal(options.allowAirdrop, false);
});

test("wallet creation permits explicit reuse without reading deployment configuration", () => {
    assert.equal(parseTesterArguments(["devnet", "--reuse", "--keypair", "tester.json"], "wallet").reuseWallet, true);
    assert.throws(() => parseTesterArguments(["--config", "runtime.json"], "wallet"), /unknown/);
    assert.throws(() => parseTesterArguments(["--amount", "1"], "wallet"), /unknown/);
});

test("funding accepts a pre-funded wallet check", () => {
    const options = parseTesterArguments(["devnet", "--skip-airdrop", "--lamports", "200000000"], "funding");
    assert.equal(options.minimumLamports, 200_000_000n);
    assert.equal(options.allowAirdrop, false);
    assert.throws(() => parseTesterArguments(["--symbol", "USDC"], "funding"), /unknown/);
});

test("setup and test modes reject funding options that they would not execute", () => {
    for (const mode of ["setup", "test"]) {
        assert.equal(parseTesterArguments(["--mode", mode], "integration").mode, mode);
        assert.throws(() => parseTesterArguments(["--mode", mode, "--lamports", "1"], "integration"), /apply to tester:all/);
        assert.throws(() => parseTesterArguments(["--skip-airdrop", "--mode", mode], "integration"), /apply to tester:all/);
    }
});

test("amount inputs reject signs, decimals, exponents, whitespace, zero, and overflow", () => {
    for (const value of ["0", "-1", "+1", "1.5", "1e6", " 1", "1 ", "01", "18446744073709551616"]) {
        assert.throws(() => parseTesterArguments(["--amount", value], "integration"));
    }
    assert.equal(parseTesterArguments(["--amount", "18446744073709551615"], "integration").amount, (1n << 64n) - 1n);
});

test("airdrop amounts reject numbers that the RPC cannot represent exactly", () => {
    assert.throws(() => parseTesterArguments(["--lamports", "9007199254740992"], "funding"), /exact RPC integer range/);
    assert.throws(() => parseTesterArguments(["--lamports", "0"], "funding"), /positive integer/);
});

test("duplicate flags and missing values are rejected", () => {
    assert.throws(() => parseTesterArguments(["--keypair", "a", "--keypair", "b"], "wallet"), /repeated/);
    assert.throws(() => parseTesterArguments(["--reuse", "--reuse"], "wallet"), /repeated/);
    assert.throws(() => parseTesterArguments(["--skip-airdrop", "--skip-airdrop"], "funding"), /repeated/);
    for (const arguments_ of [["--keypair"], ["--keypair", ""], ["--amount", "--symbol", "USDC"], ["--config", "  "]]) {
        assert.throws(() => parseTesterArguments(arguments_, "integration"), /requires a value/);
    }
});

test("unsupported networks, collateral, modes, and authority flags fail", () => {
    for (const arguments_ of [["mainnet"], ["devnet", "devnet"], ["--symbol", "USDV-LEGACY"], ["--symbol", "usdc"],
        ["--mode", "mint"], ["--setup"], ["--global"], ["--reuse"]]) {
        assert.throws(() => parseTesterArguments(arguments_, "integration"));
    }
});
