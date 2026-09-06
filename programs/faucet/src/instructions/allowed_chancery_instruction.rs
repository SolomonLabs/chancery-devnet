use chancery::constants::{ix, module};

pub(super) fn is_allowed(module_id: u8, instruction_id: u8) -> bool {
    matches!((module_id, instruction_id),
        (module::CORE, ix::core::REGISTER_ASSET
            | ix::core::UPDATE_ASSET_CONFIG
            | ix::core::SET_ASSET_MODE
            | ix::core::ACCEPT_AUTHORITY_TRANSFER
            | ix::core::UPDATE_ASSET_CONFIG_WITH_PENDING_CHANGE
            | ix::core::SET_ASSET_MODE_WITH_PENDING_CHANGE)
        | (module::PERMISSIONS, ix::permissions::UPSERT_PERMISSION
            | ix::permissions::REVOKE_PERMISSION
            | ix::permissions::UPSERT_PERMISSION_WITH_PENDING_CHANGE)
        | (module::PATHWAY, ix::pathway::REGISTER_PATHWAY_POLICY
            | ix::pathway::UPDATE_PATHWAY_POLICY
            | ix::pathway::SET_PATHWAY_STATUS
            | ix::pathway::SET_PATHWAY_STATUS_WITH_PENDING_CHANGE
            | ix::pathway::UPDATE_PATHWAY_POLICY_WITH_PENDING_CHANGE)
        | (module::SETTLEMENT, ix::settlement::REGISTER_SETTLEMENT_POLICY)
        | (module::LIMITS, ix::limits::REGISTER_LIMIT_POLICY
            | ix::limits::UPDATE_LIMIT_POLICY
            | ix::limits::UPDATE_LIMIT_POLICY_WITH_PENDING_CHANGE)
        | (module::EVIDENCE, ix::evidence::REGISTER_EVIDENCE_POLICY
            | ix::evidence::UPDATE_EVIDENCE_POLICY
            | ix::evidence::UPDATE_EVIDENCE_POLICY_WITH_PENDING_CHANGE)
        | (module::FEES, ix::fees::REGISTER_FEE_POLICY
            | ix::fees::UPDATE_FEE_POLICY
            | ix::fees::UPDATE_FEE_POLICY_WITH_PENDING_CHANGE)
        | (module::RESERVE, ix::reserve::SET_RESERVE_DESTINATION_STATUS
            | ix::reserve::SET_RESERVE_DESTINATION_STATUS_WITH_PENDING
            | ix::reserve::REGISTER_RESERVE_DESTINATION_WITH_PENDING)
        | (module::CONTROL, ix::control::SET_GLOBAL_PAUSE
            | ix::control::SET_ASSET_PAUSE
            | ix::control::SET_PATHWAY_PAUSE
            | ix::control::SET_EXECUTOR_PAUSE
            | ix::control::SET_COUNTERPARTY_PAUSE
            | ix::control::FREEZE_ISSUED_TOKEN_ACCOUNT
            | ix::control::THAW_ISSUED_TOKEN_ACCOUNT
            | ix::control::PROPOSE_CONFIG_CHANGE
            | ix::control::ACCEPT_CONFIG_CHANGE
            | ix::control::CANCEL_CONFIG_CHANGE
            | ix::control::INITIALIZE_MODULE_ACTIVATION_STATE
            | ix::control::SET_MODULE_STATUS
            | ix::control::SET_MODULE_STATUS_WITH_PENDING_CHANGE
            | ix::control::CLOSE_EXPIRED_CONFIG_CHANGE)
        | (module::MIGRATION, ix::migration::ENABLE_LEGACY_MIGRATION)
        | (module::ISSUED_TOKEN_CONTROL, ix::issued_token_control::INITIALIZE_ISSUED_TOKEN_CONTROL
            | ix::issued_token_control::REFRESH_ASSET_EXTENSION_OBSERVATION
            | ix::issued_token_control::UPDATE_ASSET_EXTENSION_POLICY
            | ix::issued_token_control::VERIFY_ISSUED_TOKEN_DEPLOYMENT
            | ix::issued_token_control::REFRESH_ISSUED_TOKEN_EXTENSION_OBSERVATION)
        | (module::CROSS_CHAIN, ix::cross_chain::REGISTER_REMOTE_DOMAIN_POLICY
            | ix::cross_chain::UPDATE_REMOTE_DOMAIN_POLICY
            | ix::cross_chain::REGISTER_CROSS_CHAIN_SIGNER_SET
            | ix::cross_chain::ROTATE_CROSS_CHAIN_SIGNER_SET
            | ix::cross_chain::RESTRICT_REMOTE_DOMAIN_PAUSE
            | ix::cross_chain::RELAX_REMOTE_DOMAIN_PAUSE
            | ix::cross_chain::UPDATE_REMOTE_DOMAIN_POLICY_WITH_PENDING_CHANGE
            | ix::cross_chain::RELAX_REMOTE_DOMAIN_PAUSE_WITH_PENDING_CHANGE)
    )
}

#[cfg(test)]
#[path = "../tests/allowed_chancery_instruction.rs"]
mod tests;
