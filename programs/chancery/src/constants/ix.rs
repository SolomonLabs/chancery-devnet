//! Instruction IDs per module.
//!
//! Each submodule lists every ix dispatched by that module. Append-only
//! within a submodule - renumbering is a wire-format breaking change.

pub mod ix {
    pub mod core {
        pub const INITIALIZE_CHANCERY:                     u8 = 0x00;
        pub const REGISTER_ASSET:                          u8 = 0x01;
        pub const UPDATE_ASSET_CONFIG:                     u8 = 0x02;
        pub const SET_ASSET_MODE:                          u8 = 0x03;
        pub const PROPOSE_AUTHORITY_TRANSFER:              u8 = 0x04;
        pub const ACCEPT_AUTHORITY_TRANSFER:               u8 = 0x05;
        pub const UPDATE_ASSET_CONFIG_WITH_PENDING_CHANGE: u8 = 0x06;
        pub const SET_ASSET_MODE_WITH_PENDING_CHANGE:      u8 = 0x07;
    }

    pub mod events_cpi {
        /// No-op handler. Receives self-CPIs from emit_event; within a finalized,
        /// successfully committed transaction, the inner instruction is the
        /// evidence record. Decoders must reject failed transaction metadata.
        pub const EMIT: u8 = 0x00;
    }

    pub mod permissions {
        pub const UPSERT_PERMISSION:                     u8 = 0x00;
        pub const REVOKE_PERMISSION:                     u8 = 0x01;
        pub const UPSERT_PERMISSION_WITH_PENDING_CHANGE: u8 = 0x02;
    }

    pub mod pathway {
        pub const REGISTER_PATHWAY_POLICY:                   u8 = 0x00;
        pub const UPDATE_PATHWAY_POLICY:                     u8 = 0x01;
        pub const SET_PATHWAY_STATUS:                        u8 = 0x02;
        pub const SET_PATHWAY_STATUS_WITH_PENDING_CHANGE:    u8 = 0x03;
        pub const UPDATE_PATHWAY_POLICY_WITH_PENDING_CHANGE: u8 = 0x04;
    }

    pub mod settlement {
        pub const CREATE_SETTLEMENT_INTENT:        u8 = 0x00;
        pub const MINT_DIRECT:                     u8 = 0x01;
        pub const REDEEM_DIRECT:                   u8 = 0x02;
        pub const MINT_DELEGATED:                  u8 = 0x03;
        pub const REDEEM_DELEGATED:                u8 = 0x04;
        pub const MINT_TRILATERAL:                 u8 = 0x05;
        pub const REDEEM_TRILATERAL:               u8 = 0x06;
        pub const REGISTER_SETTLEMENT_POLICY:      u8 = 0x07;

        // ── Bounded-state terminal cleanup ──
        pub const CLOSE_EXPIRED_SETTLEMENT_INTENT: u8 = 0x08;
        pub const CANCEL_SETTLEMENT_INTENT:        u8 = 0x09;
    }

    pub mod limits {
        pub const REGISTER_LIMIT_POLICY:                   u8 = 0x00;
        pub const UPDATE_LIMIT_POLICY:                     u8 = 0x01;
        pub const UPDATE_LIMIT_POLICY_WITH_PENDING_CHANGE: u8 = 0x02;
    }

    pub mod evidence {
        pub const REGISTER_EVIDENCE_POLICY:                   u8 = 0x00;
        pub const UPDATE_EVIDENCE_POLICY:                     u8 = 0x01;
        pub const UPDATE_EVIDENCE_POLICY_WITH_PENDING_CHANGE: u8 = 0x02;
    }

    pub mod fees {
        pub const REGISTER_FEE_POLICY:                   u8 = 0x00;
        pub const UPDATE_FEE_POLICY:                     u8 = 0x01;
        pub const UPDATE_FEE_POLICY_WITH_PENDING_CHANGE: u8 = 0x02;
    }

    pub mod reserve {
        pub const WITHDRAW_RESERVE:                            u8 = 0x01;
        pub const SET_RESERVE_DESTINATION_STATUS:              u8 = 0x02;
        pub const SET_RESERVE_DESTINATION_STATUS_WITH_PENDING: u8 = 0x03;
        pub const REGISTER_RESERVE_DESTINATION_WITH_PENDING:   u8 = 0x04;
    }

    pub mod control {
        // ── Pause handlers (existing) ──
        pub const SET_GLOBAL_PAUSE:                      u8 = 0x00;
        pub const SET_ASSET_PAUSE:                       u8 = 0x01;
        pub const SET_PATHWAY_PAUSE:                     u8 = 0x02;
        pub const SET_EXECUTOR_PAUSE:                    u8 = 0x03;
        pub const SET_COUNTERPARTY_PAUSE:                u8 = 0x04;
        
        // ── Basic freeze ──
        pub const FREEZE_ISSUED_TOKEN_ACCOUNT:           u8 = 0x05;
        pub const THAW_ISSUED_TOKEN_ACCOUNT:             u8 = 0x06;
        
        // ── Pending config change ──
        pub const PROPOSE_CONFIG_CHANGE:                 u8 = 0x07;
        pub const ACCEPT_CONFIG_CHANGE:                  u8 = 0x08;
        pub const CANCEL_CONFIG_CHANGE:                  u8 = 0x09;
        
        // ── Module activation ──
        pub const INITIALIZE_MODULE_ACTIVATION_STATE:    u8 = 0x0A;
        pub const SET_MODULE_STATUS:                     u8 = 0x0B;
        pub const SET_MODULE_STATUS_WITH_PENDING_CHANGE: u8 = 0x0C;

        // ── Bounded-state terminal cleanup ──
        pub const CLOSE_EXPIRED_CONFIG_CHANGE:           u8 = 0x0D;
    }

    pub mod migration {
        pub const ENABLE_LEGACY_MIGRATION:     u8 = 0x00;
        pub const MIGRATE_LEGACY_TO_TOKEN2022: u8 = 0x01;
    }

    pub mod issued_token_control {
        pub const INITIALIZE_ISSUED_TOKEN_CONTROL:            u8 = 0x00;
        pub const UPDATE_ISSUED_TOKEN_CONTROL:                u8 = 0x01;
        pub const ACTIVATE_ISSUED_TOKEN_MODULE:               u8 = 0x02;
        pub const DEACTIVATE_ISSUED_TOKEN_MODULE:             u8 = 0x03;
        pub const SET_TRANSFER_HOOK_PROGRAM:                  u8 = 0x04;
        pub const SET_PERMANENT_DELEGATE:                     u8 = 0x05;
        pub const INITIALIZE_TOKEN_METADATA:                  u8 = 0x06;
        pub const UPDATE_TOKEN_METADATA:                      u8 = 0x07;
        pub const SET_DEFAULT_ACCOUNT_STATE:                  u8 = 0x08;
        pub const SET_TOKEN_PAUSE_STATE:                      u8 = 0x09;
        pub const CONFIGURE_CONFIDENTIAL_TRANSFER_MINT:       u8 = 0x0A;
        pub const CONFIGURE_CONFIDENTIAL_TRANSFER_ACCOUNT:    u8 = 0x0B;
        pub const REFRESH_ASSET_EXTENSION_OBSERVATION:        u8 = 0x0C;
        pub const UPDATE_ASSET_EXTENSION_POLICY:              u8 = 0x0D;
        pub const VERIFY_ISSUED_TOKEN_DEPLOYMENT:             u8 = 0x0E;
        pub const REFRESH_ISSUED_TOKEN_EXTENSION_OBSERVATION: u8 = 0x0F;
    }

    pub mod compartments {
        pub const CREATE_RESERVE_COMPARTMENT:  u8 = 0x00;
        pub const PROMOTE_COMPARTMENT_BALANCE: u8 = 0x01;
        pub const FREEZE_COMPARTMENT:          u8 = 0x02;
    }

    pub mod provenance {
        pub const OPEN_PROVENANCE_CASE:    u8 = 0x00;
        pub const APPROVE_PROVENANCE_CASE: u8 = 0x01;
        pub const CLOSE_PROVENANCE_CASE:   u8 = 0x02;
    }

    pub mod insurance {
        pub const REGISTER_INSURANCE_POLICY:            u8 = 0x00;
        pub const UPDATE_INSURANCE_POLICY:              u8 = 0x01;
        pub const OPEN_INSURANCE_CLAIM_NOTICE:          u8 = 0x02;
        pub const UPDATE_INSURANCE_CLAIM_NOTICE_STATUS: u8 = 0x03;
    }

    pub mod enforcement {
        pub const OPEN_ENFORCEMENT_CASE:    u8 = 0x00;
        pub const APPROVE_ENFORCEMENT_CASE: u8 = 0x01;
        pub const EXECUTE_FREEZE_CASE:      u8 = 0x02;
        pub const EXECUTE_THAW_CASE:        u8 = 0x03;
        pub const EXECUTE_FORCED_BURN_CASE: u8 = 0x04;
    }

    pub mod cross_chain {
        pub const REGISTER_REMOTE_DOMAIN_POLICY:   u8 = 0x00;
        pub const UPDATE_REMOTE_DOMAIN_POLICY:     u8 = 0x01;
        pub const REGISTER_CROSS_CHAIN_SIGNER_SET: u8 = 0x02;
        pub const ROTATE_CROSS_CHAIN_SIGNER_SET:   u8 = 0x03;
        
        /// OR-only restriction. Emergency authority.
        pub const RESTRICT_REMOTE_DOMAIN_PAUSE:    u8 = 0x04;
        
        /// AND-NOT relaxation. Governance authority.
        pub const RELAX_REMOTE_DOMAIN_PAUSE:       u8 = 0x05;
        pub const CONSUME_INBOUND_MESSAGE:         u8 = 0x06;
        pub const EMIT_OUTBOUND_MESSAGE:           u8 = 0x07;
        pub const UPDATE_REMOTE_DOMAIN_POLICY_WITH_PENDING_CHANGE: u8 = 0x08;
        pub const RELAX_REMOTE_DOMAIN_PAUSE_WITH_PENDING_CHANGE:   u8 = 0x09;

        /// Permissionless authenticated terminal retirement of the strict
        /// inbound nonce head (SL-01 remedy): consume minus value effects. A
        /// message may retire after its nonzero expiry lapses or after a policy
        /// tightening makes it permanently unexecutable.
        pub const EXPIRE_INBOUND_MESSAGE:          u8 = 0x0A;

        /// Permissionless post-retirement recovery of a burned outbound emission
        /// (spec 15 §15.5 instruction B): re-mints the canonical net principal
        /// to the original sender against a quorum-attested daughter retirement
        /// (`InboundMessageExpired` / E2) digest. Emission-time effective fees
        /// remain non-refundable. Guarded by the permanent single-shot
        /// `OutboundReclaimRecord` PDA.
        pub const RECLAIM_EXPIRED_OUTBOUND:        u8 = 0x0B;
    }
}
