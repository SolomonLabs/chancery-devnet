use super::is_allowed;
use chancery::constants::{ix, module};

#[test]
fn public_administration_preserves_authority_custody_and_principal_signatures() {
    assert!(is_allowed(module::CORE, ix::core::ACCEPT_AUTHORITY_TRANSFER));
    assert!(!is_allowed(module::CORE, ix::core::PROPOSE_AUTHORITY_TRANSFER));
    assert!(!is_allowed(module::CORE, ix::core::INITIALIZE_CHANCERY));
    assert!(!is_allowed(module::SETTLEMENT, ix::settlement::MINT_DIRECT));
    assert!(!is_allowed(module::SETTLEMENT, ix::settlement::REDEEM_DIRECT));
    assert!(!is_allowed(module::RESERVE, ix::reserve::WITHDRAW_RESERVE));
    assert!(!is_allowed(module::EVENTS_CPI, ix::events_cpi::EMIT));
    assert!(!is_allowed(module::INSURANCE, ix::insurance::REGISTER_INSURANCE_POLICY));
    assert!(!is_allowed(module::CONTROL, 0xff));
    assert!(!is_allowed(0xff, 0));
}

#[test]
fn public_administration_includes_the_full_policy_change_cycle() {
    assert!(is_allowed(module::CORE, ix::core::REGISTER_ASSET));
    assert!(is_allowed(module::CONTROL, ix::control::PROPOSE_CONFIG_CHANGE));
    assert!(is_allowed(module::CONTROL, ix::control::ACCEPT_CONFIG_CHANGE));
    assert!(is_allowed(module::PERMISSIONS, ix::permissions::UPSERT_PERMISSION_WITH_PENDING_CHANGE));
    assert!(is_allowed(module::LIMITS, ix::limits::REGISTER_LIMIT_POLICY));
    assert!(is_allowed(module::LIMITS, ix::limits::UPDATE_LIMIT_POLICY_WITH_PENDING_CHANGE));
    assert!(is_allowed(module::PATHWAY, ix::pathway::REGISTER_PATHWAY_POLICY));
    assert!(is_allowed(module::PATHWAY, ix::pathway::UPDATE_PATHWAY_POLICY_WITH_PENDING_CHANGE));
}
