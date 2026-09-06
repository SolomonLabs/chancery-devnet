# Public testing

Use the repository stamped for the shared devnet deployment, including its generated client and `config/networks/devnet.ts`. Install Node according to `.nvmrc` and the Yarn version declared in `package.json`, then run `yarn install`. RPC settings are read from `config/runtime/devnet.json`; `--config` selects another runtime configuration inside the repository. Network operations check the devnet genesis hash.

## Complete workflow

```sh
yarn tester:all
```

This creates or reuses `.devnet/tester/keypair.json`, tops its SOL balance up to 100,000,000 lamports, and processes USDC, USDT, USDG, and PYUSD in that order. For each asset it registers the caller's direct pathway, ensures the caller has pathway-scoped direct mint and redeem roles, faucets one whole collateral token, then runs a mint/redeem round trip with balance assertions. The tester pays transaction fees and account rent with its own devnet SOL.

Select a particular wallet, collateral, and base-unit amount:

```sh
yarn tester:all --keypair .devnet/alice/keypair.json --symbol USDC --amount 1000000
```

`--symbol` accepts `USDC`, `USDT`, `USDG`, `PYUSD`, or `all`. The default is `all`. `--amount` supplies the faucet and settlement quantity for every selected asset. The default is one whole token per asset. Faucet calls support up to 10,000 whole tokens; settlement quantities also remain subject to the deployed asset and pathway limits.

`--lamports` sets the minimum SOL balance used by the funding stage. It is an integer lamport quantity: 100,000,000 lamports is 0.1 SOL. Only the deficit is requested from the devnet SOL faucet. An already sufficiently funded wallet proceeds immediately.

```sh
yarn tester:all --keypair .devnet/alice/keypair.json --lamports 200000000
```

## Individual stages

```sh
yarn tester:new --keypair .devnet/alice/keypair.json
yarn tester:fund --keypair .devnet/alice/keypair.json
yarn tester:setup --keypair .devnet/alice/keypair.json --symbol USDC
yarn tester:test --keypair .devnet/alice/keypair.json --symbol USDC
```

`tester:new` generates a local Solana keypair and prints its public address. Creation uses exclusive file access and preserves an existing keypair. `--reuse` explicitly loads an existing keypair or creates a missing one. The complete workflow selects reuse automatically. Each keypair is checked against its embedded public key.

`tester:fund` checks the wallet's SOL balance and requests a single confirmed airdrop for the deficit. A failed airdrop stops the command. Fund the printed public address through another devnet SOL source, then use the balance-check mode:

```sh
yarn tester:fund --keypair .devnet/alice/keypair.json --skip-airdrop
yarn tester:all --keypair .devnet/alice/keypair.json --skip-airdrop
```

`--skip-airdrop` checks that the wallet meets the selected SOL target. Collateral faucets still run in the complete workflow.

`tester:setup` uses an existing funded wallet to register its routes, ensure its direct mint/redeem grants, and faucet the selected collateral. Repeating setup adds another requested collateral allocation.

`tester:test` uses the existing wallet, permissions, pathways, and balances to run settlement only. A new wallet therefore starts with `tester:setup` or `tester:all`. Reuse the same `--symbol` and `--amount` when separating setup from testing.

## Individual integration calls

The underlying commands accept the same explicit tester keypair and derive the same caller-specific pathway:

```sh
yarn onboard --keypair .devnet/alice/keypair.json --symbol USDC
yarn faucet:mint --keypair .devnet/alice/keypair.json --symbol USDC --amount 1000000
yarn settle:mint --keypair .devnet/alice/keypair.json --symbol USDC --amount 1000000
yarn settle:redeem --keypair .devnet/alice/keypair.json --symbol USDC --amount 1000000
```

Onboarding prints the principal address, pathway ID, pathway account, and permission account. These identify the caller's route for an external integration. Explicit `--pathway-id` selection remains available on the underlying onboarding, permission, pathway, and settlement commands.

The supplied round-trip profile is direct settlement with optional primary limits. Fee, evidence, insurance, and dimension-policy profiles use their corresponding generated instruction accounts.

## Repeated and concurrent testing

Different keypair paths create different testers and caller-specific default pathways. The controller provides shared public administration, so policy, permission, and pause changes are visible across the deployment.

Each command stops at its first failed stage. Confirmed transactions remain committed, and the error identifies the failing stage. Re-running `tester:all` preserves the wallet and tops up SOL as needed; its collateral faucet calls add further balances. Use `tester:test` to repeat settlement against existing state.

## Local script tests

```sh
yarn test:testers
yarn test:deployment
```

These tests exercise argument parsing, keypair persistence, funding decisions and readback, generated command paths, caller propagation, asset selection, and failure propagation. `tester:test` executes settlement against the shared devnet deployment.
