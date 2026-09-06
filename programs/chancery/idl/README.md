# Generated IDL

`idl.json` is the canonical checked-in IDL and is generated directly by
`cargo xtask idl` using the rebuilt Pentecost Rust-IDL generator.

The generator is responsible for:

- discovering helper-forwarded account indices;
- selecting only active module account, type, and event schemas;
- resolving exact event names from their Rust trait implementations;
- adding the read-only self-CPI `event_program` account where required; and
- emitting deterministic repository-relative metadata.

Chancery does not rewrite the generated IDL. Do not edit `idl.json` directly.
`cargo xtask idl-check` regenerates a temporary IDL and fails on byte drift.
