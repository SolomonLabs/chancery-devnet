// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      constants
// File:          constants.ts
//
// Source IDL:    programs/chancery/idl/idl.json
// Output Dir:    clients/ts
//
// Introspection: disabled
// Confidence:    medium
//
// To regenerate: Run the IDL generator with the same options, or use
//                `idl-generator --use-last` to repeat the last run.
//
// ============================================================================
// @end-generated-idl
import { PublicKey } from "@solomon-labs/publickey";

export const PROGRAM_ID = new PublicKey("3doMTb5u94mzTDoBbyJXbZscNE3suuQe75ybYmirKute");

export const DEFAULT_PUBLIC_KEY = new PublicKey("11111111111111111111111111111111");

export const CHANCERY_CONFIG = new PublicKey("9vmNJfDPPB2ftSBS3qg2E47R7jPJ8mbuvK82tXbfxkP6");
export const EVENT_AUTHORITY = new PublicKey("7Aq1eLVkaP5aZmojcJg93PMb47Dtjf6pyAsqLdFhozig");
export const ISSUED_TOKEN_CONTROL = new PublicKey("35J3ja4mrZ4m81bo4Naig79983RM2JpNmFNX5pcJgpd6");
export const MIGRATION_CONFIG = new PublicKey("XRGhFj1mdTKwYJztF87cTDBZyzZn76UvaH5Uj8UKZCu");
export const MODULE_ACTIVATION_STATE = new PublicKey("CnFJmDyKb2JbwiBsbRvD2H63EmrobHaDVyyZ7hTfPunm");
export const PAUSE_STATE = new PublicKey("CAnBVzhaDFvM6sKcQGG9bK2G1RGXjy7dqkYUXSxDQ2Z5");
export const RESERVE_AUTHORITY_PDA = new PublicKey("6f7T3pGR2AwzHdGFZiupB6T7jzgDgUKqwHp6TmZFZxBs");

export const ACCOUNT_SIZES = {
    ASSET_PAUSE_STATE: 144,
    BASIC_FREEZE_RECORD: 272,
    MODULE_ACTIVATION_STATE: 128,
    PAUSE_STATE: 112,
    PENDING_CONFIG_CHANGE: 296,
    ASSET_CONFIG: 280,
    AUTHORITY_TRANSFER: 152,
    CHANCERY_CONFIG: 472,
    CROSS_CHAIN_SIGNER_SET: 168,
    OUTBOUND_RECLAIM_RECORD: 224,
    REMOTE_DOMAIN_POLICY: 336,
    REMOTE_NONCE: 168,
    EVIDENCE_POLICY: 144,
    FEE_POLICY: 200,
    ISSUED_TOKEN_CONTROL: 656,
    LIMIT_POLICY: 184,
    USAGE_WINDOW: 176,
    LEGACY_MIGRATION_CONFIG: 152,
    PATHWAY_POLICY: 616,
    PERMISSION_RECORD: 192,
    RESERVE_DESTINATION: 216,
    SETTLEMENT_INTENT: 424,
    SETTLEMENT_POLICY: 288
} as const;

export const CHAIN_KIND = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    SOLANA: 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    EVM: 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    COSMOS: 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    APTOS: 3,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    SUI: 4
} as const;

export const REMOTE_DOMAIN_MODE = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    MINT: 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    RELEASE: 1
} as const;

export const CROSS_CHAIN_MESSAGE_KIND = {
    /** Remote burn → local mint. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    INBOUND_MINT_FROM_REMOTE_BURN: 0,
    /** Local burn → remote mint. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    OUTBOUND_REDEEM_TO_REMOTE_MINT: 1,
    /** Local burn → remote release (lock-and-release model). Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    OUTBOUND_REDEEM_TO_REMOTE_RELEASE: 2
} as const;

export const CROSS_CHAIN_MESSAGE_KIND_EXTENSIONS = {
    /** Remote lock → local release. the specification Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    INBOUND_RELEASE_FROM_REMOTE_LOCK: 3,
    /** Local lock → remote release. the specification Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    OUTBOUND_LOCK_FOR_REMOTE_RELEASE: 4
} as const;

export const INBOUND_MESSAGE_RETIREMENT_REASON = {
    /** The message carried a nonzero expiry and `now >= expires_at`. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    EXPIRY_LAPSED: 0,
    /** A finite policy window rejects a zero or excessively distant expiry. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    EXPIRY_POLICY_TIGHTENED: 1,
    /** The message amount exceeds the current absolute per-message cap. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    PER_MESSAGE_CAP_TIGHTENED: 2,
    /** The message amount exceeds the current absolute per-day cap, so it can never fit even in an empty daily window. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    PER_DAY_CAP_TIGHTENED: 3,
    /** The authenticated message kind is not executable by this binary. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    UNSUPPORTED_MESSAGE_KIND: 4,
    /** The authenticated message carries a zero amount, which consume rejects. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    ZERO_AMOUNT: 5,
    /** The canonical u128 amount exceeds Solana's u64 token amount domain. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    AMOUNT_OUT_OF_RANGE: 6,
    /** The supported message kind cannot satisfy the corridor's mode/asset binding and therefore can never execute under the attested content. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    REMOTE_ASSET_BINDING_INVALID: 7,
    /** The amount exceeds the pathway's absolute per-transaction cap. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    PATHWAY_PER_TRANSACTION_CAP_TIGHTENED: 8,
    /** The amount exceeds one of the pathway's absolute fixed-window volume caps (hourly/daily/seven-day/thirty-day), so it can never fit even in an empty window and is terminally unexecutable. Distinct from transient window saturation, which clears with time and is deliberately not retired. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    PATHWAY_PERIOD_CAP_TIGHTENED: 9,
    /** The attested recipient's exact canonical issued-token ATA exists and is Frozen. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    RECIPIENT_ACCOUNT_FROZEN: 10,
    /** The exact canonical issued-token ATA is absent or exists as an uninitialized SPL token account. Recipients must create and initialize their ATA before initiating the source-chain transfer. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    RECIPIENT_ACCOUNT_MISSING_OR_UNINITIALIZED: 11
} as const;

export const REMOTE_DOMAIN_PAUSE_BIT = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    INBOUND_MESSAGES: 65536n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    OUTBOUND_MESSAGES: 131072n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    REMOTE_MINT: 262144n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    REMOTE_REDEEM: 524288n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    ATTESTATION_ACCEPTANCE: 1048576n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    DAUGHTER_CONTRACT: 2097152n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    ALL: 4128768n
} as const;

export const CROSS_CHAIN_SIGNER_SET_FLAG = {
    /** Set explicitly when the signer set is revoked outside of normal expiry - e.g. a key compromise. Verification rejects revoked sets even within their validity window. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/cross_chain.rs */
    REVOKED: 65536n
} as const;

export const SOLANA_SELF_DOMAIN_ID = {
} as const;

export const EXTENSION_BIT = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    TRANSFER_HOOK: 1n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    CONFIDENTIAL_TRANSFER: 2n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    CONFIDENTIAL_MINT_BURN: 4n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    PERMANENT_DELEGATE: 8n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    PAUSABLE: 16n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    METADATA_POINTER: 32n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    TOKEN_METADATA: 64n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    MEMO_TRANSFER: 128n,
    /** Chancery-internal forward compat. Not a current Token-2022 extension. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    PERMISSIONED_BURN: 256n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    MINT_CLOSE_AUTHORITY: 512n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    DEFAULT_ACCOUNT_STATE: 1024n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    TRANSFER_FEE_CONFIG: 4096n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    INTEREST_BEARING_CONFIG: 8192n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    NON_TRANSFERABLE: 16384n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    SCALED_UI_AMOUNT: 32768n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    GROUP_POINTER: 65536n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    TOKEN_GROUP: 131072n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    GROUP_MEMBER_POINTER: 262144n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    TOKEN_GROUP_MEMBER: 524288n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    CONFIDENTIAL_TRANSFER_FEE_CONFIG: 1048576n,
    /** TransferHook extension present with its program id set to None/zero. This reserves future hook activation without executing a hook today. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    DORMANT_TRANSFER_HOOK: 2097152n
} as const;

export const ACCOUNT_EXTENSION_BIT = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    IMMUTABLE_OWNER: 1n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    CONFIDENTIAL_TRANSFER_ACCOUNT: 2n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    MEMO_TRANSFER: 4n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    CPI_GUARD: 8n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    TRANSFER_HOOK_ACCOUNT: 16n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    PAUSABLE_ACCOUNT: 32n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    CONFIDENTIAL_TRANSFER_FEE_AMOUNT: 64n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    TRANSFER_FEE_AMOUNT: 128n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    NON_TRANSFERABLE_ACCOUNT: 256n
} as const;

export const COLLATERAL_DEFAULT_FORBIDDEN = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    MINT_FORBIDDEN_LO: 1n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    MINT_FORBIDDEN_HI: 0n
} as const;

export const ISSUED_TOKEN_DEFAULT_FORBIDDEN = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    MINT_FORBIDDEN_LO: 61440n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    MINT_FORBIDDEN_HI: 0n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    ACCOUNT_FORBIDDEN_LO: 384n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    ACCOUNT_FORBIDDEN_HI: 0n
} as const;

export const ISSUED_TOKEN_DEPLOYMENT_FLAG = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    MINT_VERIFIED: 16n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    AUTHORITIES_VERIFIED: 32n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    EXTENSIONS_VERIFIED: 64n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    ACCOUNT_STRATEGY_VERIFIED: 128n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    READY_FOR_SETTLEMENT: 9223372036854775808n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/extensions.rs */
    ALL_PRE_REQUIRED: 240n
} as const;

export const STATUS_FLAG = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/flags.rs */
    INITIALIZED: 1n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/flags.rs */
    PATHWAY_PAUSE: 8n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/flags.rs */
    MINT_PAUSED: 16n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/flags.rs */
    REDEEM_PAUSED: 32n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/flags.rs */
    MIGRATION_ENABLED: 64n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/flags.rs */
    MODULE_ACTIVE: 128n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/flags.rs */
    COMPARTMENT_FROZEN: 256n
} as const;

export const DESTINATION_PURPOSE_FLAG = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/flags.rs */
    TREASURY: 1n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/flags.rs */
    DOWNSTREAM_CUSTODY: 2n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/flags.rs */
    OPERATIONS: 4n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/flags.rs */
    RECOVERY: 8n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/flags.rs */
    PURPOSE_MASK: 15n
} as const;

export const IX = {
    core: {
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        INITIALIZE_CHANCERY: 0,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        REGISTER_ASSET: 1,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        UPDATE_ASSET_CONFIG: 2,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        SET_ASSET_MODE: 3,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        PROPOSE_AUTHORITY_TRANSFER: 4,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        ACCEPT_AUTHORITY_TRANSFER: 5,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        UPDATE_ASSET_CONFIG_WITH_PENDING_CHANGE: 6,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        SET_ASSET_MODE_WITH_PENDING_CHANGE: 7
    },
    events_cpi: {
        /** No-op handler. Receives self-CPIs from emit_event; within a finalized, successfully committed transaction, the inner instruction is the evidence record. Decoders must reject failed transaction metadata. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        EMIT: 0
    },
    permissions: {
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        UPSERT_PERMISSION: 0,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        REVOKE_PERMISSION: 1,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        UPSERT_PERMISSION_WITH_PENDING_CHANGE: 2
    },
    pathway: {
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        REGISTER_PATHWAY_POLICY: 0,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        UPDATE_PATHWAY_POLICY: 1,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        SET_PATHWAY_STATUS: 2,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        SET_PATHWAY_STATUS_WITH_PENDING_CHANGE: 3,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        UPDATE_PATHWAY_POLICY_WITH_PENDING_CHANGE: 4
    },
    settlement: {
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        CREATE_SETTLEMENT_INTENT: 0,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        MINT_DIRECT: 1,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        REDEEM_DIRECT: 2,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        MINT_DELEGATED: 3,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        REDEEM_DELEGATED: 4,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        MINT_TRILATERAL: 5,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        REDEEM_TRILATERAL: 6,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        REGISTER_SETTLEMENT_POLICY: 7,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        CLOSE_EXPIRED_SETTLEMENT_INTENT: 8,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        CANCEL_SETTLEMENT_INTENT: 9
    },
    limits: {
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        REGISTER_LIMIT_POLICY: 0,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        UPDATE_LIMIT_POLICY: 1,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        UPDATE_LIMIT_POLICY_WITH_PENDING_CHANGE: 2
    },
    evidence: {
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        REGISTER_EVIDENCE_POLICY: 0,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        UPDATE_EVIDENCE_POLICY: 1,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        UPDATE_EVIDENCE_POLICY_WITH_PENDING_CHANGE: 2
    },
    fees: {
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        REGISTER_FEE_POLICY: 0,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        UPDATE_FEE_POLICY: 1,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        UPDATE_FEE_POLICY_WITH_PENDING_CHANGE: 2
    },
    reserve: {
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        WITHDRAW_RESERVE: 1,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        SET_RESERVE_DESTINATION_STATUS: 2,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        SET_RESERVE_DESTINATION_STATUS_WITH_PENDING: 3,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        REGISTER_RESERVE_DESTINATION_WITH_PENDING: 4
    },
    control: {
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        SET_GLOBAL_PAUSE: 0,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        SET_ASSET_PAUSE: 1,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        SET_PATHWAY_PAUSE: 2,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        SET_EXECUTOR_PAUSE: 3,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        SET_COUNTERPARTY_PAUSE: 4,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        FREEZE_ISSUED_TOKEN_ACCOUNT: 5,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        THAW_ISSUED_TOKEN_ACCOUNT: 6,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        PROPOSE_CONFIG_CHANGE: 7,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        ACCEPT_CONFIG_CHANGE: 8,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        CANCEL_CONFIG_CHANGE: 9,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        INITIALIZE_MODULE_ACTIVATION_STATE: 10,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        SET_MODULE_STATUS: 11,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        SET_MODULE_STATUS_WITH_PENDING_CHANGE: 12,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        CLOSE_EXPIRED_CONFIG_CHANGE: 13
    },
    migration: {
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        ENABLE_LEGACY_MIGRATION: 0,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        MIGRATE_LEGACY_TO_TOKEN2022: 1
    },
    issued_token_control: {
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        INITIALIZE_ISSUED_TOKEN_CONTROL: 0,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        UPDATE_ISSUED_TOKEN_CONTROL: 1,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        ACTIVATE_ISSUED_TOKEN_MODULE: 2,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        DEACTIVATE_ISSUED_TOKEN_MODULE: 3,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        SET_TRANSFER_HOOK_PROGRAM: 4,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        SET_PERMANENT_DELEGATE: 5,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        INITIALIZE_TOKEN_METADATA: 6,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        UPDATE_TOKEN_METADATA: 7,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        SET_DEFAULT_ACCOUNT_STATE: 8,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        SET_TOKEN_PAUSE_STATE: 9,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        CONFIGURE_CONFIDENTIAL_TRANSFER_MINT: 10,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        CONFIGURE_CONFIDENTIAL_TRANSFER_ACCOUNT: 11,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        REFRESH_ASSET_EXTENSION_OBSERVATION: 12,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        UPDATE_ASSET_EXTENSION_POLICY: 13,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        VERIFY_ISSUED_TOKEN_DEPLOYMENT: 14,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        REFRESH_ISSUED_TOKEN_EXTENSION_OBSERVATION: 15
    },
    compartments: {
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        CREATE_RESERVE_COMPARTMENT: 0,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        PROMOTE_COMPARTMENT_BALANCE: 1,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        FREEZE_COMPARTMENT: 2
    },
    provenance: {
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        OPEN_PROVENANCE_CASE: 0,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        APPROVE_PROVENANCE_CASE: 1,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        CLOSE_PROVENANCE_CASE: 2
    },
    insurance: {
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        REGISTER_INSURANCE_POLICY: 0,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        UPDATE_INSURANCE_POLICY: 1,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        OPEN_INSURANCE_CLAIM_NOTICE: 2,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        UPDATE_INSURANCE_CLAIM_NOTICE_STATUS: 3
    },
    enforcement: {
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        OPEN_ENFORCEMENT_CASE: 0,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        APPROVE_ENFORCEMENT_CASE: 1,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        EXECUTE_FREEZE_CASE: 2,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        EXECUTE_THAW_CASE: 3,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        EXECUTE_FORCED_BURN_CASE: 4
    },
    cross_chain: {
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        REGISTER_REMOTE_DOMAIN_POLICY: 0,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        UPDATE_REMOTE_DOMAIN_POLICY: 1,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        REGISTER_CROSS_CHAIN_SIGNER_SET: 2,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        ROTATE_CROSS_CHAIN_SIGNER_SET: 3,
        /** OR-only restriction. Emergency authority. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        RESTRICT_REMOTE_DOMAIN_PAUSE: 4,
        /** AND-NOT relaxation. Governance authority. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        RELAX_REMOTE_DOMAIN_PAUSE: 5,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        CONSUME_INBOUND_MESSAGE: 6,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        EMIT_OUTBOUND_MESSAGE: 7,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        UPDATE_REMOTE_DOMAIN_POLICY_WITH_PENDING_CHANGE: 8,
        /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        RELAX_REMOTE_DOMAIN_PAUSE_WITH_PENDING_CHANGE: 9,
        /** Permissionless authenticated terminal retirement of the strict inbound nonce head (SL-01 remedy): consume minus value effects. A message may retire after its nonzero expiry lapses or after a policy tightening makes it permanently unexecutable. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        EXPIRE_INBOUND_MESSAGE: 10,
        /** Permissionless post-retirement recovery of a burned outbound emission (spec 15 §15.5 instruction B): re-mints the canonical net principal to the original sender against a quorum-attested daughter retirement (`InboundMessageExpired` / E2) digest. Emission-time effective fees remain non-refundable. Guarded by the permanent single-shot `OutboundReclaimRecord` PDA. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/ix.rs */
        RECLAIM_EXPIRED_OUTBOUND: 11
    }
} as const;

export const SETTLEMENT_MODE = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    DIRECT_PRINCIPAL: 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    DELEGATED_NON_CUSTODIAL: 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    TRILATERAL_ATOMIC: 2
} as const;

export const SETTLEMENT_ACTION = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    MINT: 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    REDEEM: 1
} as const;

export const PATHWAY_KIND = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    DIRECT: 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    DELEGATED: 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    TRILATERAL: 2,
    /** Inbound: remote burn → local mint. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    CROSS_CHAIN_MINT: 3,
    /** Outbound: local burn → remote mint or release. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    CROSS_CHAIN_REDEEM: 4
} as const;

export const COMPARTMENT_KIND = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    INTAKE_QUARANTINE: 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    CLEARED_OPERATING: 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    RESTRICTED_REVIEW: 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    RECOVERY_OR_SEIZURE: 3,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    DOWNSTREAM_CUSTODY_FEED: 4
} as const;

export const AUTHORITY_ROLE = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    GOVERNANCE: 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    OPS: 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    EMERGENCY: 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    ENFORCEMENT: 3,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    INSURANCE_ADMIN: 4
} as const;

export const WINDOW_KIND = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    HOURLY: 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    DAILY: 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    WEEKLY: 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    MONTHLY: 3
} as const;

export const FEE_RECIPIENT = {
    /** Non-routing. No external recipient: the net fee is not routed. On mint it is not minted; on redeem it stays in the reserve. Requires `fee_recipient_key = Pubkey::default()`. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    NONE: 0,
    /** Routed to the protocol treasury token account. Requires a non-default approved `fee_recipient_key`. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    PROTOCOL_TREASURY: 1,
    /** Routed to an operator-owned wallet token account. Requires a non-default approved `fee_recipient_key`. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    OPERATOR_OWNED_WALLET: 2,
    /** Routed to a pathway-specific recipient token account. Requires a non-default approved `fee_recipient_key`. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    PATHWAY_SPECIFIC: 3,
    /** Non-routing. The net fee is retained in the collateral reserve (redeem) or left unminted (mint). Requires `fee_recipient_key = Pubkey::default()`. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    RESERVE_RETENTION: 4
} as const;

export const ROUNDING = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    FLOOR: 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    CEILING: 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/kinds.rs */
    NEAREST: 2
} as const;

export const MODULE = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    CORE: 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    EVENTS_CPI: 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    PERMISSIONS: 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    PATHWAY: 3,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    SETTLEMENT: 4,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    LIMITS: 5,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    EVIDENCE: 6,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    FEES: 7,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    RESERVE: 8,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    CONTROL: 9,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    MIGRATION: 10,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    ISSUED_TOKEN_CONTROL: 11,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    COMPARTMENTS: 12,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    PROVENANCE: 13,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    INSURANCE: 14,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    ENFORCEMENT: 15,
    /** Cross-chain mint/redeem. Spec §10. Active module dispatch. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    CROSS_CHAIN: 16
} as const;

export const COMPILED_MODULES = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    IDS: [0,1,2,3,4,5,6,7,8,9,10,11,16]
} as const;

export const MODULE_STATUS = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    NONE: 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    DISABLED: 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    ADMIN_ONLY: 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    ACTIVE: 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    EMERGENCY_DISABLED: 3,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/module.rs */
    DEPRECATED: 4
} as const;

export const RATE_PRECISION_E9 = {
} as const;

export const MAX_TOKEN_DECIMALS = {
} as const;

export const MINIMUM_AUTHORITY_TRANSFER_TIMELOCK_SLOTS = {
} as const;

export const AUTHORITY_TRANSFER_ACCEPTANCE_WINDOW_SLOTS = {
} as const;

export const MAXIMUM_FREEFORM_FIELD_COUNT = {
} as const;

export const MAXIMUM_FREEFORM_VALUE_BYTES = {
} as const;

export const ROLE = {
    /** Persisted permission records bind their role bits to this exact schema. A future binary that changes the meaning or availability of role bits must perform an explicit state transition rather than silently giving new meaning to previously persisted data. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    PERMISSION_ROLE_SCHEMA_VERSION: 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_MINT_DIRECT: 1n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_REDEEM_DIRECT: 2n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_MINT_DELEGATED: 4n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_REDEEM_DELEGATED: 8n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_EXECUTE_SETTLEMENT: 16n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_USE_TRILATERAL_PATHWAY: 32n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_WITHDRAW_RESERVE: 64n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_CONFIGURE_ASSET: 128n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_GRANT_PERMISSION: 256n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_REVOKE_PERMISSION: 512n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_SET_GLOBAL_PAUSE: 1024n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_SET_ASSET_PAUSE: 2048n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_PROPOSE_AUTHORITY_TRANSFER: 4096n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_ACCEPT_AUTHORITY_TRANSFER: 8192n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_EXECUTE_LEGACY_MIGRATION: 16384n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_OPEN_PROVENANCE_CASE: 32768n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_APPROVE_PROVENANCE_CASE: 65536n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_PROMOTE_COMPARTMENT_BALANCE: 131072n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_FREEZE_TOKEN_ACCOUNT: 262144n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_THAW_TOKEN_ACCOUNT: 524288n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_EXECUTE_FORCED_BURN: 1048576n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_SET_PATHWAY_POLICY: 2097152n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_SET_LIMIT_POLICY: 4194304n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_SET_EVIDENCE_POLICY: 8388608n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_SET_FEE_POLICY: 16777216n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_SET_INSURANCE_POLICY: 33554432n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_SET_TRANSFER_HOOK_PROGRAM: 67108864n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_SET_PERMANENT_DELEGATE: 134217728n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_INITIALIZE_TOKEN_METADATA: 268435456n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_UPDATE_TOKEN_METADATA: 536870912n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_SET_DEFAULT_ACCOUNT_STATE: 1073741824n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_SET_TOKEN_PAUSE: 2147483648n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_CONFIGURE_CONFIDENTIAL_TRANSFER: 4294967296n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_ACTIVATE_ISSUED_TOKEN_MODULE: 8589934592n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_DEACTIVATE_ISSUED_TOKEN_MODULE: 17179869184n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_REGISTER_REMOTE_DOMAIN: 34359738368n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_ROTATE_SIGNER_SET: 68719476736n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_RESTRICT_REMOTE_DOMAIN_PAUSE: 137438953472n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_RELAX_REMOTE_DOMAIN_PAUSE: 274877906944n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_CONSUME_INBOUND_MESSAGE: 549755813888n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_EMIT_OUTBOUND_MESSAGE: 1099511627776n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_REGISTER_CROSS_CHAIN_SIGNER_SET: 2199023255552n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_SET_MODULE_STATUS: 4398046511104n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_UPDATE_ISSUED_TOKEN_CONTROL: 8796093022208n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_UPDATE_REMOTE_DOMAIN_POLICY: 17592186044416n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    RETIRED_ROLE_MASK: 35184372088832n,
    /** Every role position defined by this binary, including permanently retired positions. Bits outside this mask were never defined. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    DEFINED_ROLE_MASK: 70368744177663n,
    /** Backward-compatible name for the full defined role-position set. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    KNOWN_ROLE_MASK: 70368744177663n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_FORCE_BURN: 1048576n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    CAN_ROTATE_CROSS_CHAIN_SIGNER_SET: 68719476736n,
    /** Permission bits that a non-admin grantor may redistribute after the grant itself completes the required governance ceremony. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    NON_ADMIN_GRANTABLE_ROLE_MASK: 63n,
    /** Reserved role bits whose corresponding runtime actions are not present in this binary. Permission records may not pre-grant these bits, even to admins: a future program upgrade must deliberately remove the bit from this mask before the capability can be granted. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    INACTIVE_RESERVED_ROLE_MASK: 13228498467968n,
    /** Role bits that can be persisted and exercised by this binary. Retired positions and declared-but-inactive module roles fail closed. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    ACTIVE_ROLE_MASK: 21955873620863n,
    /** Backward-compatible alias used by mutation code and external checks. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    GRANTABLE_ROLE_MASK: 21955873620863n,
    /** Permission-redistribution and authority-adjacent capabilities that a non-admin grantor may never redistribute. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    AUTHORITY_ADJACENT_ROLE_MASK: 32576n,
    /** Roles that require a non-global scope, finite expiry, and the `Dangerous` pending-change tier when newly added. Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/role.rs */
    DANGEROUS_PERMISSION_ROLE_MASK: 33333444378432n
} as const;

export const SCOPE = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    GLOBAL: 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    ASSET: 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    PATHWAY: 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    DESTINATION: 3,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    EXECUTOR: 4,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    MIGRATION: 5,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    ENFORCEMENT: 6,
    /** Cross-chain remote domain (scope_key = sha256(chain_kind || domain_id_be)). Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    REMOTE_DOMAIN: 7,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    ISSUED_TOKEN_CONTROL: 8,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    MODULE: 9,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    CROSS_CHAIN_SIGNER_SET: 10,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    TOKEN_ACCOUNT: 11,
    /** Per-counterparty limit dimension (scope_key = Pubkey::default() on the template LimitPolicy; usage windows are keyed per counterparty key). Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/scope.rs */
    COUNTERPARTY: 12
} as const;

export const ASSET_MODE = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    ACTIVE: 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    WIND_DOWN: 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    FROZEN: 2
} as const;

export const INTENT_STATUS = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    PENDING: 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    EXECUTED: 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    CANCELLED: 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    EXPIRED: 3
} as const;

export const CASE_STATUS = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    OPEN: 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    APPROVED: 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    CLOSED: 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    REJECTED: 3
} as const;

export const BASIC_FREEZE_STATUS = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    NONE: 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    FROZEN: 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    THAWED: 2
} as const;

export const CONFIG_CHANGE_STATUS = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    NONE: 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    PROPOSED: 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    ACCEPTED: 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    CANCELLED: 3,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    CONSUMED: 4
} as const;

export const PATHWAY_STATUS = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    NONE: 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    DISABLED: 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    ACTIVE: 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    EMERGENCY_DISABLED: 3,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    DEPRECATED: 4
} as const;

export const RESERVE_DESTINATION_STATUS = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    NONE: 0,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    DISABLED: 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    ENABLED: 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    DEPRECATED: 3
} as const;

export const CHANGE_KIND = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    UPDATE_LIMIT_POLICY: 1,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    UPDATE_FEE_POLICY: 2,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    UPDATE_PATHWAY_POLICY: 3,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    SET_PATHWAY_STATUS: 4,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    UPSERT_PERMISSION: 5,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    REGISTER_RESERVE_DESTINATION: 6,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    SET_RESERVE_DESTINATION_STATUS: 7,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    UPDATE_ISSUED_TOKEN_CONTROL: 8,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    ACTIVATE_ISSUED_TOKEN_MODULE: 9,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    SET_TRANSFER_HOOK_PROGRAM: 10,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    SET_PERMANENT_DELEGATE: 11,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    CONFIGURE_CONFIDENTIAL_TRANSFER: 12,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    UPDATE_REMOTE_DOMAIN_POLICY: 13,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    ROTATE_CROSS_CHAIN_SIGNER_SET: 14,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    SET_MODULE_STATUS: 15,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    UPDATE_ASSET_CONFIG: 16,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    SET_ASSET_MODE: 17,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    REGISTER_CROSS_CHAIN_SIGNER_SET: 18,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    RELAX_REMOTE_DOMAIN_PAUSE: 19,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/constants/status.rs */
    UPDATE_EVIDENCE_POLICY: 20
} as const;

export const WIRE_MODULE_OFFSET = {
} as const;

export const WIRE_IX_OFFSET = {
} as const;

export const WIRE_ARGS_OFFSET = {
} as const;

export const WIRE_MINIMUM_LEN = {
} as const;

export const ACCOUNT_SIZE = {
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/compartments/state/reserve_compartment.rs */
    ReserveCompartment: 224n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/control/state/asset_pause_state.rs */
    AssetPauseState: 144n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/control/state/basic_freeze_record.rs */
    BasicFreezeRecord: 272n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/control/state/module_activation_state.rs */
    ModuleActivationState: 128n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/control/state/pause_state.rs */
    PauseState: 112n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/control/state/pending_config_change.rs */
    PendingConfigChange: 296n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/core/state/asset_config.rs */
    AssetConfig: 280n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/core/state/authority_transfer.rs */
    AuthorityTransfer: 152n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/core/state/chancery_config.rs */
    ChanceryConfig: 472n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/cross_chain/state/cross_chain_signer_set.rs */
    CrossChainSignerSet: 168n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/cross_chain/state/outbound_reclaim_record.rs */
    OutboundReclaimRecord: 224n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/cross_chain/state/remote_domain_policy.rs */
    RemoteDomainPolicy: 336n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/cross_chain/state/remote_nonce.rs */
    RemoteNonce: 168n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/enforcement/state/enforcement_case.rs */
    EnforcementCase: 280n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/evidence/state/evidence_policy.rs */
    EvidencePolicy: 144n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/fees/state/fee_policy.rs */
    FeePolicy: 200n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/insurance/state/insurance_claim_notice.rs */
    InsuranceClaimNotice: 240n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/insurance/state/insurance_policy.rs */
    InsurancePolicy: 280n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/issuance/state/issued_token_control.rs */
    IssuedTokenControl: 656n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/limits/state/limit_policy.rs */
    LimitPolicy: 184n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/limits/state/usage_window.rs */
    UsageWindow: 176n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/migration/state/legacy_migration_config.rs */
    LegacyMigrationConfig: 152n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/pathway/state/pathway_policy.rs */
    PathwayPolicy: 616n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/permissions/state/permission_record.rs */
    PermissionRecord: 192n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/provenance/state/provenance_case.rs */
    ProvenanceCase: 344n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/reserve/state/reserve_destination.rs */
    ReserveDestination: 216n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/settlement/state/settlement_intent.rs */
    SettlementIntent: 424n,
    /** Source: https://github.com/SolomonLabs/chancery/blob/main/programs/chancery/src/modules/settlement/state/settlement_policy.rs */
    SettlementPolicy: 288n
} as const;

export const IDL_CONSTANTS = {
    "chain_kind.SOLANA": 0,
    "chain_kind.EVM": 1,
    "chain_kind.COSMOS": 2,
    "chain_kind.APTOS": 3,
    "chain_kind.SUI": 4,
    "remote_domain_mode.MINT": 0,
    "remote_domain_mode.RELEASE": 1,
    "cross_chain_message_kind.INBOUND_MINT_FROM_REMOTE_BURN": 0,
    "cross_chain_message_kind.OUTBOUND_REDEEM_TO_REMOTE_MINT": 1,
    "cross_chain_message_kind.OUTBOUND_REDEEM_TO_REMOTE_RELEASE": 2,
    "cross_chain_message_kind_extensions.INBOUND_RELEASE_FROM_REMOTE_LOCK": 3,
    "cross_chain_message_kind_extensions.OUTBOUND_LOCK_FOR_REMOTE_RELEASE": 4,
    "inbound_message_retirement_reason.EXPIRY_LAPSED": 0,
    "inbound_message_retirement_reason.EXPIRY_POLICY_TIGHTENED": 1,
    "inbound_message_retirement_reason.PER_MESSAGE_CAP_TIGHTENED": 2,
    "inbound_message_retirement_reason.PER_DAY_CAP_TIGHTENED": 3,
    "inbound_message_retirement_reason.UNSUPPORTED_MESSAGE_KIND": 4,
    "inbound_message_retirement_reason.ZERO_AMOUNT": 5,
    "inbound_message_retirement_reason.AMOUNT_OUT_OF_RANGE": 6,
    "inbound_message_retirement_reason.REMOTE_ASSET_BINDING_INVALID": 7,
    "inbound_message_retirement_reason.PATHWAY_PER_TRANSACTION_CAP_TIGHTENED": 8,
    "inbound_message_retirement_reason.PATHWAY_PERIOD_CAP_TIGHTENED": 9,
    "inbound_message_retirement_reason.RECIPIENT_ACCOUNT_FROZEN": 10,
    "inbound_message_retirement_reason.RECIPIENT_ACCOUNT_MISSING_OR_UNINITIALIZED": 11,
    "remote_domain_pause_bit.INBOUND_MESSAGES": 65536n,
    "remote_domain_pause_bit.OUTBOUND_MESSAGES": 131072n,
    "remote_domain_pause_bit.REMOTE_MINT": 262144n,
    "remote_domain_pause_bit.REMOTE_REDEEM": 524288n,
    "remote_domain_pause_bit.ATTESTATION_ACCEPTANCE": 1048576n,
    "remote_domain_pause_bit.DAUGHTER_CONTRACT": 2097152n,
    "remote_domain_pause_bit.ALL": 4128768n,
    "cross_chain_signer_set_flag.REVOKED": 65536n,
    "SOLANA_SELF_DOMAIN_ID": 0n,
    "extension_bit.TRANSFER_HOOK": 1n,
    "extension_bit.CONFIDENTIAL_TRANSFER": 2n,
    "extension_bit.CONFIDENTIAL_MINT_BURN": 4n,
    "extension_bit.PERMANENT_DELEGATE": 8n,
    "extension_bit.PAUSABLE": 16n,
    "extension_bit.METADATA_POINTER": 32n,
    "extension_bit.TOKEN_METADATA": 64n,
    "extension_bit.MEMO_TRANSFER": 128n,
    "extension_bit.PERMISSIONED_BURN": 256n,
    "extension_bit.MINT_CLOSE_AUTHORITY": 512n,
    "extension_bit.DEFAULT_ACCOUNT_STATE": 1024n,
    "extension_bit.TRANSFER_FEE_CONFIG": 4096n,
    "extension_bit.INTEREST_BEARING_CONFIG": 8192n,
    "extension_bit.NON_TRANSFERABLE": 16384n,
    "extension_bit.SCALED_UI_AMOUNT": 32768n,
    "extension_bit.GROUP_POINTER": 65536n,
    "extension_bit.TOKEN_GROUP": 131072n,
    "extension_bit.GROUP_MEMBER_POINTER": 262144n,
    "extension_bit.TOKEN_GROUP_MEMBER": 524288n,
    "extension_bit.CONFIDENTIAL_TRANSFER_FEE_CONFIG": 1048576n,
    "extension_bit.DORMANT_TRANSFER_HOOK": 2097152n,
    "account_extension_bit.IMMUTABLE_OWNER": 1n,
    "account_extension_bit.CONFIDENTIAL_TRANSFER_ACCOUNT": 2n,
    "account_extension_bit.MEMO_TRANSFER": 4n,
    "account_extension_bit.CPI_GUARD": 8n,
    "account_extension_bit.TRANSFER_HOOK_ACCOUNT": 16n,
    "account_extension_bit.PAUSABLE_ACCOUNT": 32n,
    "account_extension_bit.CONFIDENTIAL_TRANSFER_FEE_AMOUNT": 64n,
    "account_extension_bit.TRANSFER_FEE_AMOUNT": 128n,
    "account_extension_bit.NON_TRANSFERABLE_ACCOUNT": 256n,
    "collateral_default_forbidden.MINT_FORBIDDEN_LO": 1n,
    "collateral_default_forbidden.MINT_FORBIDDEN_HI": 0n,
    "issued_token_default_forbidden.MINT_FORBIDDEN_LO": 61440n,
    "issued_token_default_forbidden.MINT_FORBIDDEN_HI": 0n,
    "issued_token_default_forbidden.ACCOUNT_FORBIDDEN_LO": 384n,
    "issued_token_default_forbidden.ACCOUNT_FORBIDDEN_HI": 0n,
    "issued_token_deployment_flag.MINT_VERIFIED": 16n,
    "issued_token_deployment_flag.AUTHORITIES_VERIFIED": 32n,
    "issued_token_deployment_flag.EXTENSIONS_VERIFIED": 64n,
    "issued_token_deployment_flag.ACCOUNT_STRATEGY_VERIFIED": 128n,
    "issued_token_deployment_flag.READY_FOR_SETTLEMENT": 9223372036854775808n,
    "issued_token_deployment_flag.ALL_PRE_REQUIRED": 240n,
    "status_flag.INITIALIZED": 1n,
    "status_flag.PATHWAY_PAUSE": 8n,
    "status_flag.MINT_PAUSED": 16n,
    "status_flag.REDEEM_PAUSED": 32n,
    "status_flag.MIGRATION_ENABLED": 64n,
    "status_flag.MODULE_ACTIVE": 128n,
    "status_flag.COMPARTMENT_FROZEN": 256n,
    "destination_purpose_flag.TREASURY": 1n,
    "destination_purpose_flag.DOWNSTREAM_CUSTODY": 2n,
    "destination_purpose_flag.OPERATIONS": 4n,
    "destination_purpose_flag.RECOVERY": 8n,
    "destination_purpose_flag.PURPOSE_MASK": 15n,
    "ix.core.INITIALIZE_CHANCERY": 0,
    "ix.core.REGISTER_ASSET": 1,
    "ix.core.UPDATE_ASSET_CONFIG": 2,
    "ix.core.SET_ASSET_MODE": 3,
    "ix.core.PROPOSE_AUTHORITY_TRANSFER": 4,
    "ix.core.ACCEPT_AUTHORITY_TRANSFER": 5,
    "ix.core.UPDATE_ASSET_CONFIG_WITH_PENDING_CHANGE": 6,
    "ix.core.SET_ASSET_MODE_WITH_PENDING_CHANGE": 7,
    "ix.events_cpi.EMIT": 0,
    "ix.permissions.UPSERT_PERMISSION": 0,
    "ix.permissions.REVOKE_PERMISSION": 1,
    "ix.permissions.UPSERT_PERMISSION_WITH_PENDING_CHANGE": 2,
    "ix.pathway.REGISTER_PATHWAY_POLICY": 0,
    "ix.pathway.UPDATE_PATHWAY_POLICY": 1,
    "ix.pathway.SET_PATHWAY_STATUS": 2,
    "ix.pathway.SET_PATHWAY_STATUS_WITH_PENDING_CHANGE": 3,
    "ix.pathway.UPDATE_PATHWAY_POLICY_WITH_PENDING_CHANGE": 4,
    "ix.settlement.CREATE_SETTLEMENT_INTENT": 0,
    "ix.settlement.MINT_DIRECT": 1,
    "ix.settlement.REDEEM_DIRECT": 2,
    "ix.settlement.MINT_DELEGATED": 3,
    "ix.settlement.REDEEM_DELEGATED": 4,
    "ix.settlement.MINT_TRILATERAL": 5,
    "ix.settlement.REDEEM_TRILATERAL": 6,
    "ix.settlement.REGISTER_SETTLEMENT_POLICY": 7,
    "ix.settlement.CLOSE_EXPIRED_SETTLEMENT_INTENT": 8,
    "ix.settlement.CANCEL_SETTLEMENT_INTENT": 9,
    "ix.limits.REGISTER_LIMIT_POLICY": 0,
    "ix.limits.UPDATE_LIMIT_POLICY": 1,
    "ix.limits.UPDATE_LIMIT_POLICY_WITH_PENDING_CHANGE": 2,
    "ix.evidence.REGISTER_EVIDENCE_POLICY": 0,
    "ix.evidence.UPDATE_EVIDENCE_POLICY": 1,
    "ix.evidence.UPDATE_EVIDENCE_POLICY_WITH_PENDING_CHANGE": 2,
    "ix.fees.REGISTER_FEE_POLICY": 0,
    "ix.fees.UPDATE_FEE_POLICY": 1,
    "ix.fees.UPDATE_FEE_POLICY_WITH_PENDING_CHANGE": 2,
    "ix.reserve.WITHDRAW_RESERVE": 1,
    "ix.reserve.SET_RESERVE_DESTINATION_STATUS": 2,
    "ix.reserve.SET_RESERVE_DESTINATION_STATUS_WITH_PENDING": 3,
    "ix.reserve.REGISTER_RESERVE_DESTINATION_WITH_PENDING": 4,
    "ix.control.SET_GLOBAL_PAUSE": 0,
    "ix.control.SET_ASSET_PAUSE": 1,
    "ix.control.SET_PATHWAY_PAUSE": 2,
    "ix.control.SET_EXECUTOR_PAUSE": 3,
    "ix.control.SET_COUNTERPARTY_PAUSE": 4,
    "ix.control.FREEZE_ISSUED_TOKEN_ACCOUNT": 5,
    "ix.control.THAW_ISSUED_TOKEN_ACCOUNT": 6,
    "ix.control.PROPOSE_CONFIG_CHANGE": 7,
    "ix.control.ACCEPT_CONFIG_CHANGE": 8,
    "ix.control.CANCEL_CONFIG_CHANGE": 9,
    "ix.control.INITIALIZE_MODULE_ACTIVATION_STATE": 10,
    "ix.control.SET_MODULE_STATUS": 11,
    "ix.control.SET_MODULE_STATUS_WITH_PENDING_CHANGE": 12,
    "ix.control.CLOSE_EXPIRED_CONFIG_CHANGE": 13,
    "ix.migration.ENABLE_LEGACY_MIGRATION": 0,
    "ix.migration.MIGRATE_LEGACY_TO_TOKEN2022": 1,
    "ix.issued_token_control.INITIALIZE_ISSUED_TOKEN_CONTROL": 0,
    "ix.issued_token_control.UPDATE_ISSUED_TOKEN_CONTROL": 1,
    "ix.issued_token_control.ACTIVATE_ISSUED_TOKEN_MODULE": 2,
    "ix.issued_token_control.DEACTIVATE_ISSUED_TOKEN_MODULE": 3,
    "ix.issued_token_control.SET_TRANSFER_HOOK_PROGRAM": 4,
    "ix.issued_token_control.SET_PERMANENT_DELEGATE": 5,
    "ix.issued_token_control.INITIALIZE_TOKEN_METADATA": 6,
    "ix.issued_token_control.UPDATE_TOKEN_METADATA": 7,
    "ix.issued_token_control.SET_DEFAULT_ACCOUNT_STATE": 8,
    "ix.issued_token_control.SET_TOKEN_PAUSE_STATE": 9,
    "ix.issued_token_control.CONFIGURE_CONFIDENTIAL_TRANSFER_MINT": 10,
    "ix.issued_token_control.CONFIGURE_CONFIDENTIAL_TRANSFER_ACCOUNT": 11,
    "ix.issued_token_control.REFRESH_ASSET_EXTENSION_OBSERVATION": 12,
    "ix.issued_token_control.UPDATE_ASSET_EXTENSION_POLICY": 13,
    "ix.issued_token_control.VERIFY_ISSUED_TOKEN_DEPLOYMENT": 14,
    "ix.issued_token_control.REFRESH_ISSUED_TOKEN_EXTENSION_OBSERVATION": 15,
    "ix.compartments.CREATE_RESERVE_COMPARTMENT": 0,
    "ix.compartments.PROMOTE_COMPARTMENT_BALANCE": 1,
    "ix.compartments.FREEZE_COMPARTMENT": 2,
    "ix.provenance.OPEN_PROVENANCE_CASE": 0,
    "ix.provenance.APPROVE_PROVENANCE_CASE": 1,
    "ix.provenance.CLOSE_PROVENANCE_CASE": 2,
    "ix.insurance.REGISTER_INSURANCE_POLICY": 0,
    "ix.insurance.UPDATE_INSURANCE_POLICY": 1,
    "ix.insurance.OPEN_INSURANCE_CLAIM_NOTICE": 2,
    "ix.insurance.UPDATE_INSURANCE_CLAIM_NOTICE_STATUS": 3,
    "ix.enforcement.OPEN_ENFORCEMENT_CASE": 0,
    "ix.enforcement.APPROVE_ENFORCEMENT_CASE": 1,
    "ix.enforcement.EXECUTE_FREEZE_CASE": 2,
    "ix.enforcement.EXECUTE_THAW_CASE": 3,
    "ix.enforcement.EXECUTE_FORCED_BURN_CASE": 4,
    "ix.cross_chain.REGISTER_REMOTE_DOMAIN_POLICY": 0,
    "ix.cross_chain.UPDATE_REMOTE_DOMAIN_POLICY": 1,
    "ix.cross_chain.REGISTER_CROSS_CHAIN_SIGNER_SET": 2,
    "ix.cross_chain.ROTATE_CROSS_CHAIN_SIGNER_SET": 3,
    "ix.cross_chain.RESTRICT_REMOTE_DOMAIN_PAUSE": 4,
    "ix.cross_chain.RELAX_REMOTE_DOMAIN_PAUSE": 5,
    "ix.cross_chain.CONSUME_INBOUND_MESSAGE": 6,
    "ix.cross_chain.EMIT_OUTBOUND_MESSAGE": 7,
    "ix.cross_chain.UPDATE_REMOTE_DOMAIN_POLICY_WITH_PENDING_CHANGE": 8,
    "ix.cross_chain.RELAX_REMOTE_DOMAIN_PAUSE_WITH_PENDING_CHANGE": 9,
    "ix.cross_chain.EXPIRE_INBOUND_MESSAGE": 10,
    "ix.cross_chain.RECLAIM_EXPIRED_OUTBOUND": 11,
    "settlement_mode.DIRECT_PRINCIPAL": 0,
    "settlement_mode.DELEGATED_NON_CUSTODIAL": 1,
    "settlement_mode.TRILATERAL_ATOMIC": 2,
    "settlement_action.MINT": 0,
    "settlement_action.REDEEM": 1,
    "pathway_kind.DIRECT": 0,
    "pathway_kind.DELEGATED": 1,
    "pathway_kind.TRILATERAL": 2,
    "pathway_kind.CROSS_CHAIN_MINT": 3,
    "pathway_kind.CROSS_CHAIN_REDEEM": 4,
    "compartment_kind.INTAKE_QUARANTINE": 0,
    "compartment_kind.CLEARED_OPERATING": 1,
    "compartment_kind.RESTRICTED_REVIEW": 2,
    "compartment_kind.RECOVERY_OR_SEIZURE": 3,
    "compartment_kind.DOWNSTREAM_CUSTODY_FEED": 4,
    "authority_role.GOVERNANCE": 0,
    "authority_role.OPS": 1,
    "authority_role.EMERGENCY": 2,
    "authority_role.ENFORCEMENT": 3,
    "authority_role.INSURANCE_ADMIN": 4,
    "window_kind.HOURLY": 0,
    "window_kind.DAILY": 1,
    "window_kind.WEEKLY": 2,
    "window_kind.MONTHLY": 3,
    "fee_recipient.NONE": 0,
    "fee_recipient.PROTOCOL_TREASURY": 1,
    "fee_recipient.OPERATOR_OWNED_WALLET": 2,
    "fee_recipient.PATHWAY_SPECIFIC": 3,
    "fee_recipient.RESERVE_RETENTION": 4,
    "rounding.FLOOR": 0,
    "rounding.CEILING": 1,
    "rounding.NEAREST": 2,
    "module.CORE": 0,
    "module.EVENTS_CPI": 1,
    "module.PERMISSIONS": 2,
    "module.PATHWAY": 3,
    "module.SETTLEMENT": 4,
    "module.LIMITS": 5,
    "module.EVIDENCE": 6,
    "module.FEES": 7,
    "module.RESERVE": 8,
    "module.CONTROL": 9,
    "module.MIGRATION": 10,
    "module.ISSUED_TOKEN_CONTROL": 11,
    "module.COMPARTMENTS": 12,
    "module.PROVENANCE": 13,
    "module.INSURANCE": 14,
    "module.ENFORCEMENT": 15,
    "module.CROSS_CHAIN": 16,
    "compiled_modules.IDS": [0,1,2,3,4,5,6,7,8,9,10,11,16],
    "module_status.NONE": 0,
    "module_status.DISABLED": 0,
    "module_status.ADMIN_ONLY": 1,
    "module_status.ACTIVE": 2,
    "module_status.EMERGENCY_DISABLED": 3,
    "module_status.DEPRECATED": 4,
    "RATE_PRECISION_E9": 1000000000n,
    "MAX_TOKEN_DECIMALS": 18,
    "MINIMUM_AUTHORITY_TRANSFER_TIMELOCK_SLOTS": 1n,
    "AUTHORITY_TRANSFER_ACCEPTANCE_WINDOW_SLOTS": 8640000n,
    "MAXIMUM_FREEFORM_FIELD_COUNT": 32,
    "MAXIMUM_FREEFORM_VALUE_BYTES": 64,
    "role.PERMISSION_ROLE_SCHEMA_VERSION": 1,
    "role.CAN_MINT_DIRECT": 1n,
    "role.CAN_REDEEM_DIRECT": 2n,
    "role.CAN_MINT_DELEGATED": 4n,
    "role.CAN_REDEEM_DELEGATED": 8n,
    "role.CAN_EXECUTE_SETTLEMENT": 16n,
    "role.CAN_USE_TRILATERAL_PATHWAY": 32n,
    "role.CAN_WITHDRAW_RESERVE": 64n,
    "role.CAN_CONFIGURE_ASSET": 128n,
    "role.CAN_GRANT_PERMISSION": 256n,
    "role.CAN_REVOKE_PERMISSION": 512n,
    "role.CAN_SET_GLOBAL_PAUSE": 1024n,
    "role.CAN_SET_ASSET_PAUSE": 2048n,
    "role.CAN_PROPOSE_AUTHORITY_TRANSFER": 4096n,
    "role.CAN_ACCEPT_AUTHORITY_TRANSFER": 8192n,
    "role.CAN_EXECUTE_LEGACY_MIGRATION": 16384n,
    "role.CAN_OPEN_PROVENANCE_CASE": 32768n,
    "role.CAN_APPROVE_PROVENANCE_CASE": 65536n,
    "role.CAN_PROMOTE_COMPARTMENT_BALANCE": 131072n,
    "role.CAN_FREEZE_TOKEN_ACCOUNT": 262144n,
    "role.CAN_THAW_TOKEN_ACCOUNT": 524288n,
    "role.CAN_EXECUTE_FORCED_BURN": 1048576n,
    "role.CAN_SET_PATHWAY_POLICY": 2097152n,
    "role.CAN_SET_LIMIT_POLICY": 4194304n,
    "role.CAN_SET_EVIDENCE_POLICY": 8388608n,
    "role.CAN_SET_FEE_POLICY": 16777216n,
    "role.CAN_SET_INSURANCE_POLICY": 33554432n,
    "role.CAN_SET_TRANSFER_HOOK_PROGRAM": 67108864n,
    "role.CAN_SET_PERMANENT_DELEGATE": 134217728n,
    "role.CAN_INITIALIZE_TOKEN_METADATA": 268435456n,
    "role.CAN_UPDATE_TOKEN_METADATA": 536870912n,
    "role.CAN_SET_DEFAULT_ACCOUNT_STATE": 1073741824n,
    "role.CAN_SET_TOKEN_PAUSE": 2147483648n,
    "role.CAN_CONFIGURE_CONFIDENTIAL_TRANSFER": 4294967296n,
    "role.CAN_ACTIVATE_ISSUED_TOKEN_MODULE": 8589934592n,
    "role.CAN_DEACTIVATE_ISSUED_TOKEN_MODULE": 17179869184n,
    "role.CAN_REGISTER_REMOTE_DOMAIN": 34359738368n,
    "role.CAN_ROTATE_SIGNER_SET": 68719476736n,
    "role.CAN_RESTRICT_REMOTE_DOMAIN_PAUSE": 137438953472n,
    "role.CAN_RELAX_REMOTE_DOMAIN_PAUSE": 274877906944n,
    "role.CAN_CONSUME_INBOUND_MESSAGE": 549755813888n,
    "role.CAN_EMIT_OUTBOUND_MESSAGE": 1099511627776n,
    "role.CAN_REGISTER_CROSS_CHAIN_SIGNER_SET": 2199023255552n,
    "role.CAN_SET_MODULE_STATUS": 4398046511104n,
    "role.CAN_UPDATE_ISSUED_TOKEN_CONTROL": 8796093022208n,
    "role.CAN_UPDATE_REMOTE_DOMAIN_POLICY": 17592186044416n,
    "role.RETIRED_ROLE_MASK": 35184372088832n,
    "role.DEFINED_ROLE_MASK": 70368744177663n,
    "role.KNOWN_ROLE_MASK": 70368744177663n,
    "role.CAN_FORCE_BURN": 1048576n,
    "role.CAN_ROTATE_CROSS_CHAIN_SIGNER_SET": 68719476736n,
    "role.NON_ADMIN_GRANTABLE_ROLE_MASK": 63n,
    "role.INACTIVE_RESERVED_ROLE_MASK": 13228498467968n,
    "role.ACTIVE_ROLE_MASK": 21955873620863n,
    "role.GRANTABLE_ROLE_MASK": 21955873620863n,
    "role.AUTHORITY_ADJACENT_ROLE_MASK": 32576n,
    "role.DANGEROUS_PERMISSION_ROLE_MASK": 33333444378432n,
    "scope.GLOBAL": 0,
    "scope.ASSET": 1,
    "scope.PATHWAY": 2,
    "scope.DESTINATION": 3,
    "scope.EXECUTOR": 4,
    "scope.MIGRATION": 5,
    "scope.ENFORCEMENT": 6,
    "scope.REMOTE_DOMAIN": 7,
    "scope.ISSUED_TOKEN_CONTROL": 8,
    "scope.MODULE": 9,
    "scope.CROSS_CHAIN_SIGNER_SET": 10,
    "scope.TOKEN_ACCOUNT": 11,
    "scope.COUNTERPARTY": 12,
    "asset_mode.ACTIVE": 0,
    "asset_mode.WIND_DOWN": 1,
    "asset_mode.FROZEN": 2,
    "intent_status.PENDING": 0,
    "intent_status.EXECUTED": 1,
    "intent_status.CANCELLED": 2,
    "intent_status.EXPIRED": 3,
    "case_status.OPEN": 0,
    "case_status.APPROVED": 1,
    "case_status.CLOSED": 2,
    "case_status.REJECTED": 3,
    "basic_freeze_status.NONE": 0,
    "basic_freeze_status.FROZEN": 1,
    "basic_freeze_status.THAWED": 2,
    "config_change_status.NONE": 0,
    "config_change_status.PROPOSED": 1,
    "config_change_status.ACCEPTED": 2,
    "config_change_status.CANCELLED": 3,
    "config_change_status.CONSUMED": 4,
    "pathway_status.NONE": 0,
    "pathway_status.DISABLED": 1,
    "pathway_status.ACTIVE": 2,
    "pathway_status.EMERGENCY_DISABLED": 3,
    "pathway_status.DEPRECATED": 4,
    "reserve_destination_status.NONE": 0,
    "reserve_destination_status.DISABLED": 1,
    "reserve_destination_status.ENABLED": 2,
    "reserve_destination_status.DEPRECATED": 3,
    "change_kind.UPDATE_LIMIT_POLICY": 1,
    "change_kind.UPDATE_FEE_POLICY": 2,
    "change_kind.UPDATE_PATHWAY_POLICY": 3,
    "change_kind.SET_PATHWAY_STATUS": 4,
    "change_kind.UPSERT_PERMISSION": 5,
    "change_kind.REGISTER_RESERVE_DESTINATION": 6,
    "change_kind.SET_RESERVE_DESTINATION_STATUS": 7,
    "change_kind.UPDATE_ISSUED_TOKEN_CONTROL": 8,
    "change_kind.ACTIVATE_ISSUED_TOKEN_MODULE": 9,
    "change_kind.SET_TRANSFER_HOOK_PROGRAM": 10,
    "change_kind.SET_PERMANENT_DELEGATE": 11,
    "change_kind.CONFIGURE_CONFIDENTIAL_TRANSFER": 12,
    "change_kind.UPDATE_REMOTE_DOMAIN_POLICY": 13,
    "change_kind.ROTATE_CROSS_CHAIN_SIGNER_SET": 14,
    "change_kind.SET_MODULE_STATUS": 15,
    "change_kind.UPDATE_ASSET_CONFIG": 16,
    "change_kind.SET_ASSET_MODE": 17,
    "change_kind.REGISTER_CROSS_CHAIN_SIGNER_SET": 18,
    "change_kind.RELAX_REMOTE_DOMAIN_PAUSE": 19,
    "change_kind.UPDATE_EVIDENCE_POLICY": 20,
    "WIRE_MODULE_OFFSET": 0n,
    "WIRE_IX_OFFSET": 1n,
    "WIRE_ARGS_OFFSET": 2n,
    "WIRE_MINIMUM_LEN": 2n,
    "account_size.ReserveCompartment": 224n,
    "account_size.AssetPauseState": 144n,
    "account_size.BasicFreezeRecord": 272n,
    "account_size.ModuleActivationState": 128n,
    "account_size.PauseState": 112n,
    "account_size.PendingConfigChange": 296n,
    "account_size.AssetConfig": 280n,
    "account_size.AuthorityTransfer": 152n,
    "account_size.ChanceryConfig": 472n,
    "account_size.CrossChainSignerSet": 168n,
    "account_size.OutboundReclaimRecord": 224n,
    "account_size.RemoteDomainPolicy": 336n,
    "account_size.RemoteNonce": 168n,
    "account_size.EnforcementCase": 280n,
    "account_size.EvidencePolicy": 144n,
    "account_size.FeePolicy": 200n,
    "account_size.InsuranceClaimNotice": 240n,
    "account_size.InsurancePolicy": 280n,
    "account_size.IssuedTokenControl": 656n,
    "account_size.LimitPolicy": 184n,
    "account_size.UsageWindow": 176n,
    "account_size.LegacyMigrationConfig": 152n,
    "account_size.PathwayPolicy": 616n,
    "account_size.PermissionRecord": 192n,
    "account_size.ProvenanceCase": 344n,
    "account_size.ReserveDestination": 216n,
    "account_size.SettlementIntent": 424n,
    "account_size.SettlementPolicy": 288n
} as const;

