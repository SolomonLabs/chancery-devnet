# Vendored bundle outputs

This directory contains prebuilt JavaScript and TypeScript declaration outputs used as local workspace packages. Every vendored package is authored inside the Solomon Labs monorepo: there is no third-party publisher, so upstream source archives, package licence texts, and component SBOMs do not apply.

`vendor/PROVENANCE.json` is the canonical inventory. It binds the sha256 of each retained output and package manifest; the canonical form of every file is its Git index blob, and the audit assessed-tree digest covers these bytes.

## Regeneration

Regenerate only from the exact monorepo revision under review. Never regenerate from a moving branch.

Copy the generated outputs into this directory and update the sha256 values in `vendor/PROVENANCE.json` to match the retained bytes. Preserve the declaration-file bytes exactly; `.gitattributes` marks the two checksum-bound declaration outputs `-text` so Git does not rewrite their line endings.

Stage all source and metadata changes before regenerating audit evidence:

```bash
git add --all
yarn write:provenance:stage
yarn validate:provenance --source index
git commit
yarn validate:provenance --source head
```

The audit writer generates evidence from staged Git blobs; `vendor/PROVENANCE.json` is an authored inventory and must be updated in the same commit as any vendored byte change. Do not regenerate `audits/PROVENANCE.json` first or manually edit hashes afterward.
