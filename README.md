# chancery-devnet

Chancery's public devnet testing fork uses two separately deployed programs.
Chancery provides settlement and its existing policy state machines. The faucet
program supplies test collateral and signs allowlisted Chancery administration
instructions with distinct program-derived authorities.

The mainnet repository remains a separate reference. Program identities, test
mints, controller authorities, and the shortened timing rules belong to this
repository's devnet deployment.

## Timing and authority model

Widening, HighImpact, Dangerous, and Irreversible config changes each have a
one-second minimum timelock. RestrictiveImmediate and RoutineOps remain
immediate. Authority transfers have a one-slot minimum. Transaction confirmation
and RPC latency are additional to these floors.

The faucet/controller holds five distinct Chancery authority PDAs, derived from
`["chancery-authority", role_byte]` under its own program ID. The role bytes are
Governance 0, Operations 1, Emergency 2, Enforcement 3, and InsuranceAdmin 4.
The collateral mint authority is a separate PDA derived from
`["faucet-authority"]`.

The deployment authority hands over the four non-governance roles first and
governance last. The handoff command reads each role before acting and resumes
matching pending transfers. Loader upgrade authorities remain with their
configured deployment keys.

Public callers sign and pay for their own transactions. The controller supplies
only its selected PDA signatures during CPI. It pins the Chancery program ID,
checks the instruction allowlist, and preserves the forwarded account order.
The allowlist covers policy registration and changes, permission administration,
config proposals and acceptance, operational controls, and extension observation.
Authority-transfer proposals and arbitrary token transfers are outside that
allowlist.

This is shared public administration: testers can change shared limits or pause
state and affect one another. Per-wallet pathway IDs separate ordinary test
routes, but do not create private administration boundaries. Use test assets and
devnet-only program identities for this deployment.

## Toolchain and identities

Use Node 24.18.0 from `.nvmrc`, Yarn 4.2.2, and the repository's Rust/Solana
build toolchain. The runtime preflight accepts the declared Node minimum in
`package.json`.

Both program IDs initially contain the unstamped sentinel
`11111111111111111111111111111111`. The unified deployment creates missing keypairs,
stamps both program identities, regenerates the IDL and client, runs TypeScript
checking and Rust tests, and builds both SBF programs before provisioning.

`config/runtime/devnet.json` holds RPC settings and local keypair paths.
`config/networks/devnet.ts` holds the deployed public program and mint
identities. Keep the files under `.devnet/` private. Public testers use the
stamped public identities, their own RPC configuration, and a funded devnet wallet.

## Bring up a shared instance

Install the workspace dependencies, then run the complete build and deployment:

```sh
yarn install
yarn deploy:all
```

The command prepares deployment keys, stamps both programs, generates the client,
checks types, runs Rust tests, builds both programs, creates the four collateral
mints and legacy migration source, launches the issued token, and records its
verified address automatically. It then deploys and initializes Chancery,
initializes its controls, deploys the faucet, transfers mint and administrative
authorities, registers the four shared pathways, and runs fresh-wallet public
integration checks for every collateral asset.

The fee payer and setup authority need devnet SOL for account rent and transaction
fees. The configured airdrop settings apply during asset creation and token launch.
To generate and inspect the wallet addresses before funding them, run:

```sh
yarn deploy:prepare
```

Existing keypairs are reused. A configured program or mint whose matching keypair
is missing stops preparation. The unified flow requires configured
`issued_mint_keypair_path` and `upgrade_authority_keypair_path` values. Existing
on-chain mints and records are read and checked by their respective commands.
A failed command stops the flow immediately; rerunning uses the saved identities
and the current on-chain state.

`token:launch` records `usdvMint` in the public identity file only after
comprehensive finalized-state verification. Its persistent mint keypair also
recovers an on-chain launch interrupted before that file was written. Each
subsequent command starts a new process and loads the updated identities.

The final check creates a fresh caller wallet under `.devnet/public-integration-*/`
and funds it from the fee payer with 100,000,000 lamports (0.1 SOL) by default.
Using that wallet alone, it onboards a caller-specific pathway for each collateral,
requests one token from each faucet, and performs mint then redeem with the
existing balance-delta assertions. The funded wallet and its keypair remain
available for further testing. The funding budget is configurable:

```sh
yarn deploy:all --verification-lamports 200000000
yarn deploy:all --config config/runtime/custom.json
yarn verify:public --verification-lamports 100000000
```

The same runtime configuration is passed to every deployment and verification
command. Chancery deployment and faucet deployment verify the devnet genesis
before submitting their loader instructions. The success message is reached
only after the final public integration command exits successfully.

The individual `assets:create`, `token:launch`, `deploy`, `init`, `configure`,
`faucet:deploy`, `authorities:handoff`, and `bootstrap:policies` commands remain
available. `faucet:deploy --skip-deploy` checks an existing executable controller
and transfers test-mint authorities. Policy bootstrap verifies the issued token,
registers and activates all four collateral assets, creates their shared direct
pathways, and grants the configured fee payer direct mint/redeem permissions.

The shared USDC route retains its `chancery-direct-v1` identity. Shared routes
for the other assets are derived from the collateral and issued mint. Commands
print their pathway IDs. Public administration remains shared across testers.

## Test assets and faucet

USDC and USDT use SPL Token with six decimals. USDG and PYUSD use Token-2022
with six decimals and a dormant transfer hook. USDV-LEGACY is a separate
nine-decimal SPL Token migration source.

The faucet accepts SPL Token and Token-2022 mints whose mint authority it holds.
Each call is capped at 10,000 whole tokens, scaled by the mint's decimals and
checked against the unsigned 64-bit amount range. There is no cumulative faucet
quota. Collateral registered through the controller must be faucet-backed.
The canonical issued mint is registered for Chancery control checks and is
issued through settlement rather than the collateral faucet.

`assets:mint` supports setup-authority top-ups before mint handoff.
`faucet:mint` supplies balances after handoff and checks the requested balance
increase on readback:

```sh
yarn faucet:mint --keypair ~/.config/solana/id.json --symbol USDC --amount 5000000000
yarn faucet:mint --keypair ~/.config/solana/id.json --symbol USDG
```

`--amount` is in base units. Its default is 1,000 whole tokens. `--to` selects
another destination wallet; otherwise the caller receives the tokens.

## Public tester scripts

Run `yarn tester:all` against the deployed instance to create or reuse a tester
wallet, top up devnet SOL, register direct mint/redeem permissions and pathways,
faucet each collateral asset, and execute mint/redeem round trips.

`tester:new`, `tester:fund`, `tester:setup`, and `tester:test` expose the individual
stages. See [Public testing](docs/public-testing.md) for wallet selection,
single-asset runs, pre-funded wallets, and separate settlement legs.

## Public onboarding and settlement

A tester can create a route and grant their own direct settlement roles using
only their wallet:

```sh
yarn onboard --keypair ~/.config/solana/id.json --symbol USDC
yarn faucet:mint --keypair ~/.config/solana/id.json --symbol USDC
yarn settle --keypair ~/.config/solana/id.json --symbol USDC --amount 1000000
```

`onboard` and `pathway:add` run the same onboarding command. With an explicit
`--keypair`, its default route ID is derived from the caller, collateral mint,
and issued mint. Without that flag, both onboarding and settlement use the
shared route and the configured fee payer. `--pathway-id` selects an explicit
nonzero 32-byte hexadecimal ID in either command.

The onboarding profile is a direct mint/redeem pathway with optional primary
limits. Settlement resolves its enabled hourly, daily, seven-day, and thirty-day
usage accounts. Linked fee, evidence, insurance, and dimension policies require
their additional generated instruction accounts; the settlement command rejects
those profiles explicitly.

`settle:mint` and `settle:redeem` run one leg. The collateral symbol must match
the selected pathway. The initial collateral asset cap is 100,000,000 base units
per settlement, independent of any additional pathway limit.

`grant:principal` creates missing permission records for supplied wallet
addresses, or checks that existing records provide the requested active roles
and expiry. It accepts `--file`, `--roles`, `--expiry`, `--global`,
`--symbol`, and `--pathway-id`. Its default authority is the controller;
`--setup` selects the setup signer before handoff. It does not silently replace
mismatching existing permissions.

## Permission updates

Change an existing grant with an explicit role set, expiry, or both:

```sh
yarn permission:update --keypair ~/.config/solana/id.json --symbol USDC --roles mint WALLET_ADDRESS
yarn permission:update --keypair ~/.config/solana/id.json --symbol USDC --expiry 0 WALLET_ADDRESS
yarn permission:update --keypair ~/.config/solana/id.json --symbol USDC --roles none WALLET_ADDRESS
```

`--roles` replaces the complete role set. Names are `mint`, `redeem`,
`mint-delegated`, and `redeem-delegated`, separated by commas. `none`
clears every role on an existing record. An omitted role set or expiry retains
its current value. Explicit `--expiry 0` makes the grant perpetual.

Grant and update commands use the same default route selection as onboarding:
an explicit caller keypair selects that caller's route; the configured fee payer
selects the shared route. `--pathway-id` selects a particular route and
`--global` selects global permission scope.

Role additions and expiry extensions use the appropriate pending-change tier.
Removing a finite expiry uses HighImpact for ordinary roles. Pure role removals
and expiry shortening use the direct restrictive instruction. The resulting
permission retains its operational pause flags. Each changed record advances
its permission generation; unchanged values leave the record as it stands.
Creation rejects empty grants, unsupported roles, invalid integer ranges, and
expired values before submitting a proposal. Dangerous grants require a
non-global scope and finite expiry. Creation and update check committed values
on readback.

## Pathway limits and changes

Create a pathway-scoped limit before registering a new route that references it.
Both IDs must be nonzero 32-byte hexadecimal values. For example:

```sh
PATHWAY_ID=1111111111111111111111111111111111111111111111111111111111111111
LIMIT_POLICY_ID=2222222222222222222222222222222222222222222222222222222222222222

yarn limit:add --keypair ~/.config/solana/id.json --limit-policy-id "$LIMIT_POLICY_ID" --pathway-id "$PATHWAY_ID" --per-transaction 10000000 --per-hour 100000000 --per-day 200000000
yarn onboard --keypair ~/.config/solana/id.json --symbol USDC --pathway-id "$PATHWAY_ID" --limit-policy-id "$LIMIT_POLICY_ID"
yarn settle --keypair ~/.config/solana/id.json --symbol USDC --pathway-id "$PATHWAY_ID" --amount 1000000
yarn limit:update --keypair ~/.config/solana/id.json --limit-policy-id "$LIMIT_POLICY_ID" --per-transaction 20000000
```

Volume flags are `--per-transaction`, `--per-hour`, `--per-day`,
`--per-seven-day`, and `--per-thirty-day`. Action flags are
`--actions-per-hour` and `--actions-per-day`. Volumes use base units.
Enabled volume caps must be non-decreasing across periods; enabled hourly
and daily action caps follow the same ordering rule.

On creation, omitted caps are disabled. On update, omitted caps keep their
current values, while explicit zero disables that cap. Finite increases and
disabling enabled caps use the Widening pending-change flow. Restrictive changes
use the direct operations instruction. Usage windows track gross flow, so a
mint followed by a redemption consumes capacity on both legs.

Attach or replace the primary limit on an existing pathway, or unlink it:

```sh
yarn pathway:update --keypair ~/.config/solana/id.json --pathway-id "$PATHWAY_ID" --limit-policy-id "$LIMIT_POLICY_ID"
yarn pathway:update --keypair ~/.config/solana/id.json --pathway-id "$PATHWAY_ID" --limit-policy-id none
```

A linked limit must already exist with the selected pathway as its scope.
`none` removes the primary-limit reference. The command preserves other
pathway fields, submits reference changes through the Widening pending-change
flow, and verifies the resulting semantic payload. Existing usage history
remains associated with its limit policy.

Controller scheduling derives proposal timestamps from the on-chain execution
clock, then the client reads the resulting pending account, waits, accepts,
and submits the target change. This avoids proposing an already-stale one-second
timestamp from an RPC observation.

## Controller integration

`runtime/controller/authority.ts` derives the role PDAs.
`ExecuteChanceryInstruction` wraps an existing generated Chancery instruction,
marks selected controller authorities as CPI signers, and retains caller or
payer signer requirements. `submitAdministration` submits these wrappers for a
controller authorization or submits directly for an explicit keypair authorization.

The faucet instruction formats are:

- `0x00 | amount:u64-le`: mint test collateral; accounts are mint, destination,
  faucet authority, and token program.
- `0x01 | role-mask:u8 | chancery-instruction`: execute an allowlisted instruction.
- `0x02 | governance-mask:u8 | delay:u64-le | lifetime:u64-le | proposal`:
  schedule a Chancery config proposal using the execution clock.

Both administrative formats receive caller signer, executable Chancery program,
then the generated Chancery accounts in their original order. The scheduling
variant preserves the proposal hashes and nonce while replacing its two timestamps.

`scripts/deployment/lib/configChange.ts` handles semantic hashes, proposal
identity, scheduling, acceptance, and the pending-change account. Additional
allowlisted fee, evidence, cross-chain, or operational workflows can use their
existing generated instruction types through this controller interface.

## Checks

After stamping program identities and installing the selected toolchain:

```sh
yarn typecheck
yarn test:limits
yarn test:permissions
yarn test:pathways
cargo test -p chancery-devnet-faucet
yarn test:rust
yarn build:sbf
yarn build:faucet
```

The pure limit tests cover omitted values, explicit zero, transition risk,
storage bounds, period ordering, and the canonical semantic payload. Permission
tests cover creation invariants, transition risk, omitted values, explicit zero,
and generation bounds. Pathway tests cover primary-limit rebinding and semantic
payload normalization. The faucet
Rust tests cover token/program boundaries, authority masks, the instruction
allowlist, and execution-clock proposal scheduling. Live devnet onboarding and
settlement exercise the complete cross-program workflow.
