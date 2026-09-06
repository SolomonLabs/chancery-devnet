//! Static release gates for the spec 13 / spec 14 account-layout register.
//!
//! Spec 14's house rule requires that every consumed reserved tail is
//! replenished with fresh append headroom in the same change, and spec 13
//! makes the PDA register normative. Nothing mechanical enforced either until
//! this lint: the per-file `const _: () = assert!(size_of == SIZE)` guards
//! only pin the *total* size, so a change could consume tail bytes, shrink
//! the headroom, or move `_reserved` off the tail while keeping the total
//! constant - and every guard would still pass.
//!
//! This register pins, for every stateful account:
//!   - the declared `*_SIZE` constant against a literal recorded here;
//!   - `size_of::<T>()` against that same literal;
//!   - the state-loader header contract: `discriminator` at offset 0 and
//!     `version` at offset 8;
//!   - `_reserved` as the **trailing** region with its exact pinned array type
//!     and length (`[u8; RESERVED]` ending exactly at `SIZE`).
//!
//! It also pins every `constants::seeds` prefix byte-for-byte against the
//! spec 13 register. Seeds are append-only: adding a prefix extends this
//! table; changing one fails here first.
//!
//! Consequence: any layout or seed change must consciously edit this file in
//! the same change, alongside spec 13/14 - which is exactly the replenish
//! ritual the specs require.

use crate::{
    constants::seeds,
    modules::{
        // Out of MVP: `compartments` is not declared in `modules/mod.rs`
        // (spec 08 §8.10 reserved later module), so its state cannot be
        // linted until the module is re-enabled. Restore alongside the
        // matching pin below in the SAME change that re-declares the module.
        // compartments::state::reserve_compartment::{ReserveCompartment, RESERVE_COMPARTMENT_SIZE},
        control::state::{
            asset_pause_state::{AssetPauseState, ASSET_PAUSE_STATE_SIZE},
            basic_freeze_record::{BasicFreezeRecord, BASIC_FREEZE_RECORD_SIZE},
            module_activation_state::{ModuleActivationState, MODULE_ACTIVATION_STATE_SIZE},
            pause_state::{PauseState, PAUSE_STATE_SIZE},
            pending_config_change::{PendingConfigChange, PENDING_CONFIG_CHANGE_SIZE},
        },
        core::state::{
            asset_config::{AssetConfig, ASSET_CONFIG_RESERVED_SIZE, ASSET_CONFIG_SIZE},
            authority_transfer::{
                AuthorityTransfer, AUTHORITY_TRANSFER_RESERVED_SIZE, AUTHORITY_TRANSFER_SIZE,
            },
            chancery_config::{ChanceryConfig, CHANCERY_CONFIG_SIZE},
        },
        cross_chain::state::{
            cross_chain_signer_set::{CrossChainSignerSet, CROSS_CHAIN_SIGNER_SET_SIZE},
            outbound_reclaim_record::{OutboundReclaimRecord, OUTBOUND_RECLAIM_RECORD_SIZE},
            remote_domain_policy::{RemoteDomainPolicy, REMOTE_DOMAIN_POLICY_SIZE},
            remote_nonce::{RemoteNonce, REMOTE_NONCE_SIZE},
        },
        // Out of MVP (see note above): reserved later module, undeclared.
        // enforcement::state::enforcement_case::{EnforcementCase, ENFORCEMENT_CASE_SIZE},
        evidence::state::evidence_policy::{EvidencePolicy, EVIDENCE_POLICY_SIZE},
        fees::state::fee_policy::{FeePolicy, FEE_POLICY_SIZE},
        // Out of MVP (see note above): reserved later module, undeclared.
        // insurance::state::{
        //     insurance_claim_notice::{InsuranceClaimNotice, INSURANCE_CLAIM_NOTICE_SIZE},
        //     insurance_policy::{InsurancePolicy, INSURANCE_POLICY_SIZE},
        // },
        issuance::state::issued_token_control::{IssuedTokenControl, ISSUED_TOKEN_CONTROL_SIZE},
        limits::state::{
            limit_policy::{LimitPolicy, LIMIT_POLICY_SIZE},
            usage_window::{UsageWindow, USAGE_WINDOW_SIZE},
        },
        migration::state::legacy_migration_config::{
            LegacyMigrationConfig, LEGACY_MIGRATION_CONFIG_SIZE,
        },
        pathway::state::pathway_policy::{
            PathwayPolicy, PATHWAY_POLICY_RESERVED_SIZE, PATHWAY_POLICY_SIZE,
        },
        permissions::state::permission_record::{PermissionRecord, PERMISSION_RECORD_SIZE},
        // Out of MVP (see note above): reserved later module, undeclared.
        // provenance::state::provenance_case::{ProvenanceCase, PROVENANCE_CASE_SIZE},
        reserve::state::reserve_destination::{
            ReserveDestination, RESERVE_DESTINATION_RESERVED_SIZE, RESERVE_DESTINATION_SIZE,
        },
        settlement::state::{
            settlement_intent::{SettlementIntent, SETTLEMENT_INTENT_RESERVED_SIZE, SETTLEMENT_INTENT_SIZE},
            settlement_policy::{SettlementPolicy, SETTLEMENT_POLICY_SIZE},
        },
    },
};

/// Pin one state account against the layout register.
///
/// `$expected_size` and `$expected_reserved` are literals recorded HERE, not
/// references to the source constants - the whole point is that a source-side
/// change cannot satisfy this lint without an explicit matching edit in this
/// register.
macro_rules! pin_state_layout {
    ($t:ty, $size_const:expr, $expected_size:expr, $expected_reserved:expr) => {{
        fn assert_reserved_field_type(value: &$t) {
            let _: &[u8; $expected_reserved] = &value._reserved;
        }

        let _ = assert_reserved_field_type as fn(&$t);

        assert_eq!(
            $size_const, $expected_size,
            "{}: declared SIZE constant drifted from the layout register - update spec 13/14 and this register in the same change",
            stringify!($t),
        );
        assert_eq!(
            core::mem::size_of::<$t>(),
            $expected_size,
            "{}: struct size drifted from the layout register",
            stringify!($t),
        );
        assert_eq!(
            core::mem::offset_of!($t, discriminator),
            0,
            "{}: discriminator must open the account (state-loader contract)",
            stringify!($t),
        );
        assert_eq!(
            core::mem::offset_of!($t, version),
            8,
            "{}: state version must sit at offset 8 (state-loader contract)",
            stringify!($t),
        );
        assert_eq!(
            core::mem::offset_of!($t, _reserved),
            $expected_size - $expected_reserved,
            "{}: reserved append headroom must be the trailing {} bytes - consuming tail bytes requires replenishing headroom and updating this register in the same change (spec 14 house rule)",
            stringify!($t),
            $expected_reserved,
        );
        assert_eq!(
            core::mem::offset_of!($t, _reserved)
                + core::mem::size_of::<[u8; $expected_reserved]>(),
            $expected_size,
            "{}: reserved append headroom must end at the account boundary",
            stringify!($t),
        );
    }};
}

#[test]
fn state_accounts_pin_declared_layout_and_reserved_tail() {
    // ── Singletons ────────────────────────────────────────────────────────────
    pin_state_layout!(ChanceryConfig,        CHANCERY_CONFIG_SIZE,         472, 24);
    pin_state_layout!(PauseState,            PAUSE_STATE_SIZE,             112, 32);
    pin_state_layout!(ModuleActivationState, MODULE_ACTIVATION_STATE_SIZE, 128, 32);
    pin_state_layout!(IssuedTokenControl,    ISSUED_TOKEN_CONTROL_SIZE,    656, 32);
    pin_state_layout!(LegacyMigrationConfig, LEGACY_MIGRATION_CONFIG_SIZE, 152, 32);

    // ── Key-derived ───────────────────────────────────────────────────────────
    pin_state_layout!(AssetConfig,           ASSET_CONFIG_SIZE,            280, 32);
    pin_state_layout!(AssetPauseState,       ASSET_PAUSE_STATE_SIZE,       144, 32);
    pin_state_layout!(BasicFreezeRecord,     BASIC_FREEZE_RECORD_SIZE,     272, 24);
    pin_state_layout!(PermissionRecord,      PERMISSION_RECORD_SIZE,       192, 24);
    pin_state_layout!(ReserveDestination,    RESERVE_DESTINATION_SIZE,     216, 32);
    pin_state_layout!(AuthorityTransfer,     AUTHORITY_TRANSFER_SIZE,      152, 16);
    pin_state_layout!(RemoteDomainPolicy,    REMOTE_DOMAIN_POLICY_SIZE,    336, 32);
    pin_state_layout!(RemoteNonce,           REMOTE_NONCE_SIZE,            168, 32);

    // ── Identifier-keyed ──────────────────────────────────────────────────────
    pin_state_layout!(PathwayPolicy,         PATHWAY_POLICY_SIZE,          616, 64);
    pin_state_layout!(SettlementPolicy,      SETTLEMENT_POLICY_SIZE,       288, 32);
    pin_state_layout!(SettlementIntent,      SETTLEMENT_INTENT_SIZE,       424, 32);
    pin_state_layout!(LimitPolicy,           LIMIT_POLICY_SIZE,            184, 32);
    pin_state_layout!(UsageWindow,           USAGE_WINDOW_SIZE,            176, 32);
    pin_state_layout!(EvidencePolicy,        EVIDENCE_POLICY_SIZE,         144, 32);
    pin_state_layout!(FeePolicy,             FEE_POLICY_SIZE,              200, 32);
    pin_state_layout!(PendingConfigChange,   PENDING_CONFIG_CHANGE_SIZE,   296, 16);
    pin_state_layout!(CrossChainSignerSet,   CROSS_CHAIN_SIGNER_SET_SIZE,  168, 32);
    // Spec 15 §15.5: emission-identity-keyed, permanent, NEVER closeable.
    pin_state_layout!(OutboundReclaimRecord, OUTBOUND_RECLAIM_RECORD_SIZE, 224, 32);

    // ── Reserved / excluded modules (compiled; layouts stay pinned) ───────────
    // ── Out of MVP: reserved later modules (spec 08 §8.10) ────────────────
    // `compartments`, `provenance`, `insurance`, and `enforcement` are not
    // declared in `modules/mod.rs`, so their state types do not compile and
    // cannot be linted. The pinned literals are RETAINED here, commented,
    // as the layout register of record: whichever change re-declares a
    // module MUST uncomment its pin (and the matching import above) in the
    // same change, and the sizes must still hold.
    // pin_state_layout!(ReserveCompartment,    RESERVE_COMPARTMENT_SIZE,     224, 32);
    // pin_state_layout!(ProvenanceCase,        PROVENANCE_CASE_SIZE,         344, 32);
    // pin_state_layout!(InsurancePolicy,       INSURANCE_POLICY_SIZE,        280, 32);
    // pin_state_layout!(InsuranceClaimNotice,  INSURANCE_CLAIM_NOTICE_SIZE,  240, 32);
    // pin_state_layout!(EnforcementCase,       ENFORCEMENT_CASE_SIZE,        280, 32);

    // Where the source additionally declares a *_RESERVED_SIZE constant, it
    // must agree with the register.
    assert_eq!(ASSET_CONFIG_RESERVED_SIZE,        32);
    assert_eq!(AUTHORITY_TRANSFER_RESERVED_SIZE,  16);
    assert_eq!(PATHWAY_POLICY_RESERVED_SIZE,      64);
    assert_eq!(RESERVE_DESTINATION_RESERVED_SIZE, 32);
    assert_eq!(SETTLEMENT_INTENT_RESERVED_SIZE,   32);
}

#[test]
fn seed_prefixes_are_pinned_to_the_spec_13_register() {
    // Append-only: adding a prefix extends this table; changing one is an
    // address-space breaking change and fails here first.
    let register: &[(&[u8], &[u8])] = &[
        (seeds::CHANCERY_CONFIG,                 b"chancery-config"),
        (seeds::ASSET_CONFIG,                    b"asset-config"),
        (seeds::PERMISSION,                      b"permission"),
        (seeds::PATHWAY_POLICY,                  b"pathway-policy"),
        (seeds::SETTLEMENT_POLICY,               b"settlement-policy"),
        (seeds::SETTLEMENT_INTENT,               b"settlement-intent"),
        (seeds::LIMIT_POLICY,                    b"limit-policy"),
        (seeds::USAGE_WINDOW,                    b"usage-window"),
        (seeds::EVIDENCE_POLICY,                 b"evidence-policy"),
        (seeds::FEE_POLICY,                      b"fee-policy"),
        (seeds::INSURANCE_POLICY,                b"insurance-policy"),
        (seeds::RESERVE_COMPARTMENT,             b"reserve-compartment"),
        (seeds::PROVENANCE_CASE,                 b"provenance-case"),
        (seeds::RESERVE_DESTINATION,             b"reserve-destination"),
        (seeds::PAUSE_STATE,                     b"pause-state"),
        (seeds::ASSET_PAUSE,                     b"asset-pause"),
        (seeds::AUTHORITY_TRANSFER,              b"authority-transfer"),
        (seeds::LEGACY_MIGRATION,                b"legacy-migration"),
        (seeds::INSURANCE_CLAIM_NOTICE,          b"insurance-claim-notice"),
        (seeds::REPORT_FIELD,                    b"report-field"),
        (seeds::ENFORCEMENT_CASE,                b"enforcement-case"),
        (seeds::MINT_AUTHORITY,                  b"mint-authority"),
        (seeds::FREEZE_AUTHORITY,                b"freeze-authority"),
        (seeds::RESERVE_AUTHORITY,               b"reserve-authority"),
        (seeds::ISSUED_TOKEN_CONTROL,            b"issued-token-control"),
        (seeds::CLOSE_MINT_AUTHORITY,            b"close-mint-authority"),
        (seeds::TRANSFER_HOOK_AUTHORITY,         b"transfer-hook-authority"),
        (seeds::PERMANENT_DELEGATE_AUTHORITY,    b"permanent-delegate-authority"),
        (seeds::METADATA_POINTER_AUTHORITY,      b"metadata-pointer-authority"),
        (seeds::METADATA_UPDATE_AUTHORITY,       b"metadata-update-authority"),
        (seeds::PAUSE_AUTHORITY,                 b"pause-authority"),
        (seeds::CONFIDENTIAL_TRANSFER_AUTHORITY, b"confidential-transfer-authority"),
        (seeds::DEFAULT_ACCOUNT_STATE_AUTHORITY, b"default-account-state-authority"),
        (seeds::REMOTE_DOMAIN_POLICY,            b"remote-domain-policy"),
        (seeds::CROSS_CHAIN_SIGNER_SET,          b"cross-chain-signer-set"),
        (seeds::REMOTE_NONCE,                    b"remote-nonce"),
        (seeds::OUTBOUND_RECLAIM_RECORD,         b"outbound-reclaim"),
        (seeds::BASIC_FREEZE_RECORD,             b"basic-freeze-record"),
        (seeds::PENDING_CONFIG_CHANGE,           b"pending-config-change"),
        (seeds::MODULE_ACTIVATION_STATE,         b"module-activation-state"),
        (seeds::EVENT_AUTHORITY,                 b"event-authority"),
    ];

    for (actual, expected) in register {
        assert_eq!(
            actual, expected,
            "seed prefix drifted from the spec 13 register - seed changes are address-space breaking",
        );
    }
}
