/**
 * pdas.ts
 *
 * PDA derivation helpers built on `@solomon-labs/publickey`.
 *
 * Every function here is a thin composition over `PublicKey.findProgramAddress`.
 * The seed prefixes mirror the Rust source of truth at
 * `programs/chancery/src/constants.rs::seeds` - append-only; renaming a seed
 * is an account-layout breaking change.
 *
 * The IDL exposes seed metadata only on instructions that declare a PDA, so
 * authority-only and helper PDAs (`mint-authority`, `freeze-authority`, `reserve-authority`,
 * `event-authority`, `usage-window`, `remote-nonce`,
 * `settlement-policy`) are not auto-discoverable from the IDL alone. Once the
 * generator emits per-class `seed`/`pda` static mappings the bodies below
 * reduce to one-line passthroughs onto `<Class>.pda(...)`.
 *
 * No hand-rolled byte manipulation beyond integer encoding and fixed scope-hash
 * preimages which the IDL cannot represent.
 */

import { BufferUtil } from "@solomon-labs/buffer";
import { CryptoUtil } from "@solomon-labs/crypto";
import { PublicKey } from "@solomon-labs/publickey";

import { PROGRAM_ID } from "clients/ts/src/constants";

// ─── Seed prefixes (mirror programs/chancery/src/constants.rs::seeds) ────────

const REMOTE_DOMAIN_USAGE_WINDOW_SCOPE_KIND = 0x07;

const SEED = {
    CHANCERY_CONFIG:        BufferUtil.fromString("chancery-config",        "utf8"),
    ASSET_CONFIG:           BufferUtil.fromString("asset-config",           "utf8"),
    PERMISSION:             BufferUtil.fromString("permission",             "utf8"),
    PATHWAY_POLICY:         BufferUtil.fromString("pathway-policy",         "utf8"),
    SETTLEMENT_POLICY:      BufferUtil.fromString("settlement-policy",      "utf8"),
    SETTLEMENT_INTENT:      BufferUtil.fromString("settlement-intent",      "utf8"),
    LIMIT_POLICY:           BufferUtil.fromString("limit-policy",           "utf8"),
    USAGE_WINDOW:           BufferUtil.fromString("usage-window",           "utf8"),
    EVIDENCE_POLICY:        BufferUtil.fromString("evidence-policy",        "utf8"),
    FEE_POLICY:             BufferUtil.fromString("fee-policy",             "utf8"),
    INSURANCE_POLICY:       BufferUtil.fromString("insurance-policy",       "utf8"),
    RESERVE_DESTINATION:    BufferUtil.fromString("reserve-destination",    "utf8"),
    PAUSE_STATE:            BufferUtil.fromString("pause-state",            "utf8"),
    ASSET_PAUSE:            BufferUtil.fromString("asset-pause",            "utf8"),
    AUTHORITY_TRANSFER:     BufferUtil.fromString("authority-transfer",     "utf8"),
    LEGACY_MIGRATION:       BufferUtil.fromString("legacy-migration",       "utf8"),
    MODULE_ACTIVATION_STATE: BufferUtil.fromString("module-activation-state", "utf8"),
    PENDING_CONFIG_CHANGE:   BufferUtil.fromString("pending-config-change",   "utf8"),
    MINT_AUTHORITY:         BufferUtil.fromString("mint-authority",         "utf8"),
    FREEZE_AUTHORITY:       BufferUtil.fromString("freeze-authority",       "utf8"),
    RESERVE_AUTHORITY:      BufferUtil.fromString("reserve-authority",      "utf8"),
    CLOSE_MINT_AUTHORITY:   BufferUtil.fromString("close-mint-authority",   "utf8"),
    TRANSFER_HOOK_AUTHORITY: BufferUtil.fromString("transfer-hook-authority", "utf8"),
    PERMANENT_DELEGATE_AUTHORITY: BufferUtil.fromString("permanent-delegate-authority", "utf8"),
    METADATA_POINTER_AUTHORITY: BufferUtil.fromString("metadata-pointer-authority", "utf8"),
    METADATA_UPDATE_AUTHORITY: BufferUtil.fromString("metadata-update-authority", "utf8"),
    PAUSE_AUTHORITY:        BufferUtil.fromString("pause-authority",        "utf8"),
    CONFIDENTIAL_TRANSFER_AUTHORITY: BufferUtil.fromString("confidential-transfer-authority", "utf8"),
    DEFAULT_ACCOUNT_STATE_AUTHORITY: BufferUtil.fromString("default-account-state-authority", "utf8"),
    EVENT_AUTHORITY:        BufferUtil.fromString("event-authority",        "utf8"),
    REMOTE_DOMAIN_POLICY:   BufferUtil.fromString("remote-domain-policy",   "utf8"),
    CROSS_CHAIN_SIGNER_SET: BufferUtil.fromString("cross-chain-signer-set", "utf8"),
    REMOTE_NONCE:           BufferUtil.fromString("remote-nonce",           "utf8"),
    OUTBOUND_RECLAIM_RECORD: BufferUtil.fromString("outbound-reclaim",      "utf8"),
    ISSUED_TOKEN_CONTROL:   BufferUtil.fromString("issued-token-control",   "utf8"),
    BASIC_FREEZE_RECORD:    BufferUtil.fromString("basic-freeze-record",    "utf8"),
} as const;

// ─── Integer-to-seed-bytes encoders ──────────────────────────────────────────

function u8Bytes(value: number): Uint8Array {
    return new Uint8Array([value & 0xff]);
}

function u64BeBytes(value: bigint): Uint8Array {
    const buf = new Uint8Array(8);
    new DataView(buf.buffer).setBigUint64(0, value, false);
    return buf;
}

function findPda(seeds: Uint8Array[]): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddress(seeds, PROGRAM_ID);
}

// ─── Singleton PDAs ──────────────────────────────────────────────────────────

export function chanceryConfigPda(): Promise<[PublicKey, number]> {
    return findPda([SEED.CHANCERY_CONFIG]);
}

export function pauseStatePda(): Promise<[PublicKey, number]> {
    return findPda([SEED.PAUSE_STATE]);
}

export function legacyMigrationConfigPda(): Promise<[PublicKey, number]> {
    return findPda([SEED.LEGACY_MIGRATION]);
}

export function moduleActivationStatePda(): Promise<[PublicKey, number]> {
    return findPda([SEED.MODULE_ACTIVATION_STATE]);
}

export function pendingConfigChangePda(changeId: Uint8Array): Promise<[PublicKey, number]> {
    return findPda([SEED.PENDING_CONFIG_CHANGE, changeId]);
}

export function mintAuthorityPda(): Promise<[PublicKey, number]> {
    return findPda([SEED.MINT_AUTHORITY]);
}

export function freezeAuthorityPda(): Promise<[PublicKey, number]> {
    return findPda([SEED.FREEZE_AUTHORITY]);
}

export function reserveAuthorityPda(): Promise<[PublicKey, number]> {
    return findPda([SEED.RESERVE_AUTHORITY]);
}

export function closeMintAuthorityPda(): Promise<[PublicKey, number]> {
    return findPda([SEED.CLOSE_MINT_AUTHORITY]);
}

export function transferHookAuthorityPda(): Promise<[PublicKey, number]> {
    return findPda([SEED.TRANSFER_HOOK_AUTHORITY]);
}

export function permanentDelegateAuthorityPda(): Promise<[PublicKey, number]> {
    return findPda([SEED.PERMANENT_DELEGATE_AUTHORITY]);
}

export function metadataPointerAuthorityPda(): Promise<[PublicKey, number]> {
    return findPda([SEED.METADATA_POINTER_AUTHORITY]);
}

export function metadataUpdateAuthorityPda(): Promise<[PublicKey, number]> {
    return findPda([SEED.METADATA_UPDATE_AUTHORITY]);
}

export function pauseAuthorityPda(): Promise<[PublicKey, number]> {
    return findPda([SEED.PAUSE_AUTHORITY]);
}

export function confidentialTransferAuthorityPda(): Promise<[PublicKey, number]> {
    return findPda([SEED.CONFIDENTIAL_TRANSFER_AUTHORITY]);
}

export function defaultAccountStateAuthorityPda(): Promise<[PublicKey, number]> {
    return findPda([SEED.DEFAULT_ACCOUNT_STATE_AUTHORITY]);
}

export function eventAuthorityPda(): Promise<[PublicKey, number]> {
    return findPda([SEED.EVENT_AUTHORITY]);
}

export function issuedTokenControlPda(): Promise<[PublicKey, number]> {
    return findPda([SEED.ISSUED_TOKEN_CONTROL]);
}

export function basicFreezeRecordPda(issuedTokenAccount: PublicKey): Promise<[PublicKey, number]> {
    return findPda([SEED.BASIC_FREEZE_RECORD, issuedTokenAccount.toBytes()]);
}

// ─── Mint-keyed PDAs ─────────────────────────────────────────────────────────

export function assetConfigPda(assetMint: PublicKey): Promise<[PublicKey, number]> {
    return findPda([SEED.ASSET_CONFIG, assetMint.toBytes()]);
}

export function assetPauseStatePda(assetMint: PublicKey): Promise<[PublicKey, number]> {
    return findPda([SEED.ASSET_PAUSE, assetMint.toBytes()]);
}

// ─── 32-byte ID PDAs ─────────────────────────────────────────────────────────

export function pathwayPolicyPda(pathwayId: Uint8Array): Promise<[PublicKey, number]> {
    return findPda([SEED.PATHWAY_POLICY, pathwayId]);
}

export function settlementIntentPda(intentId: Uint8Array): Promise<[PublicKey, number]> {
    return findPda([SEED.SETTLEMENT_INTENT, intentId]);
}

export function settlementPolicyPda(policyId: Uint8Array): Promise<[PublicKey, number]> {
    return findPda([SEED.SETTLEMENT_POLICY, policyId]);
}

export function limitPolicyPda(limitPolicyId: Uint8Array): Promise<[PublicKey, number]> {
    return findPda([SEED.LIMIT_POLICY, limitPolicyId]);
}

export function evidencePolicyPda(evidencePolicyId: Uint8Array): Promise<[PublicKey, number]> {
    return findPda([SEED.EVIDENCE_POLICY, evidencePolicyId]);
}

export function feePolicyPda(feePolicyId: Uint8Array): Promise<[PublicKey, number]> {
    return findPda([SEED.FEE_POLICY, feePolicyId]);
}

export function insurancePolicyPda(insurancePolicyId: Uint8Array): Promise<[PublicKey, number]> {
    return findPda([SEED.INSURANCE_POLICY, insurancePolicyId]);
}

export function crossChainSignerSetPda(signerSetId: Uint8Array): Promise<[PublicKey, number]> {
    return findPda([SEED.CROSS_CHAIN_SIGNER_SET, signerSetId]);
}

/**
 * Spec 15 §15.5 (H-01 remediated): permanent single-shot reclaim guard keyed
 * by EMISSION IDENTITY - reclaim_count(corridor, nonce) <= 1 across every
 * content claim.
 */
export function outboundReclaimRecordPda(
    remoteChainKind: number,
    remoteDomainId:  bigint,
    sourceNonce:     bigint,
): Promise<[PublicKey, number]> {
    return findPda([
        SEED.OUTBOUND_RECLAIM_RECORD,
        u8Bytes(remoteChainKind),
        u64BeBytes(remoteDomainId),
        u64BeBytes(sourceNonce),
    ]);
}


// ─── Composite PDAs ──────────────────────────────────────────────────────────

export function authorityTransferPda(roleKind: number): Promise<[PublicKey, number]> {
    return findPda([SEED.AUTHORITY_TRANSFER, u8Bytes(roleKind)]);
}

export function permissionRecordPda(
    subject:   PublicKey,
    scopeKind: number,
    scopeKey:  PublicKey,
): Promise<[PublicKey, number]> {
    return findPda([
        SEED.PERMISSION,
        subject.toBytes(),
        u8Bytes(scopeKind),
        scopeKey.toBytes(),
    ]);
}

export function reserveDestinationPda(
    assetMint:               PublicKey,
    destinationTokenAccount: PublicKey,
): Promise<[PublicKey, number]> {
    return findPda([
        SEED.RESERVE_DESTINATION,
        assetMint.toBytes(),
        destinationTokenAccount.toBytes(),
    ]);
}

export function remoteDomainPolicyPda(
    remoteChainKind: number,
    remoteDomainId:  bigint,
): Promise<[PublicKey, number]> {
    return findPda([
        SEED.REMOTE_DOMAIN_POLICY,
        u8Bytes(remoteChainKind),
        u64BeBytes(remoteDomainId),
    ]);
}

export async function remoteDomainUsageWindowScopeHash(
    remoteDomainPolicy: PublicKey,
): Promise<Uint8Array> {
    const remoteDomainPolicyBytes = remoteDomainPolicy.toBytes();
    const preimage = new Uint8Array(1 + remoteDomainPolicyBytes.length);
    preimage[0] = REMOTE_DOMAIN_USAGE_WINDOW_SCOPE_KIND;
    preimage.set(remoteDomainPolicyBytes, 1);
    return CryptoUtil.sha256(preimage);
}

export function remoteNoncePda(
    remoteChainKind: number,
    remoteDomainId:  bigint,
    scopeKey:        PublicKey,
): Promise<[PublicKey, number]> {
    return findPda([
        SEED.REMOTE_NONCE,
        u8Bytes(remoteChainKind),
        u64BeBytes(remoteDomainId),
        scopeKey.toBytes(),
    ]);
}

export function usageWindowPda(
    scopeHash:  Uint8Array,
    windowKind: number,
): Promise<[PublicKey, number]> {
    return findPda([
        SEED.USAGE_WINDOW,
        scopeHash,
        u8Bytes(windowKind),
    ]);
}
