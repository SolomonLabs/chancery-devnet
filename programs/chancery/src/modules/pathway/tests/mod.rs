/// modules/pathway/tests/mod.rs
///
/// Unit tests for PathwayPolicy state and guard methods.
///
/// Covers:
///   - size / layout
///   - load guards (uninit, wrong size, non-writable, double-init)
///   - assert_active: INITIALIZED flag and PATHWAY_PAUSE bit
///   - assert_kind: matching and mismatched pathway kinds
///   - assert_executor: exact designated executor or role-permitted wildcard
///   - has_fee_policy / has_limit_policy / has_evidence_policy predicates
///   - PDA determinism and uniqueness per pathway_id
///   - single-field executor ABI and pre-release layout rejection

use std::{cell::RefCell, rc::Rc};

use bytemuck::Zeroable;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::{pathway_kind, seeds, status_flag},
    error::ChanceryError,
    modules::pathway::state::pathway_policy::{
        PathwayPolicy, PATHWAY_POLICY_DISCRIMINATOR, PATHWAY_POLICY_SIZE,
    },
};

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn program_id() -> Pubkey { crate::id() }

#[allow(deprecated)]
fn make_account_info<'a>(
    key:      &'a Pubkey,
    data:     &'a mut [u8],
    lamports: &'a mut u64,
    owner:    &'a Pubkey,
    writable: bool,
) -> solana_account_info::AccountInfo<'a> {
    solana_account_info::AccountInfo {
        key,
        lamports:    Rc::new(RefCell::new(lamports)),
        data:        Rc::new(RefCell::new(data)),
        owner,
        _unused:     0,
        is_signer:   false,
        is_writable: writable,
        executable:  false,
    }
}

/// Build an initialised PathwayPolicy value (not an account).
fn make_policy(
    pathway_id:          [u8; 32],
    kind:                u8,
    designated_executor: Pubkey,
    status_flags:        u64,
) -> PathwayPolicy {
    let mut p = PathwayPolicy::zeroed();

    p.discriminator      = PATHWAY_POLICY_DISCRIMINATOR;
    p.version            = 1;
    p.pathway_kind       = kind;
    p.pathway_id         = pathway_id;
    p.asset_mint         = Pubkey::new_unique();
    p.issued_token_mint  = Pubkey::new_unique();
    p.designated_executor = designated_executor;
    p.status_flags       = status_flags;

    p
}

/// Build a standard DIRECT pathway - active, no executor constraints.
fn direct_policy() -> PathwayPolicy {
    make_policy(
        [0x01u8; 32],
        pathway_kind::DIRECT,
        Pubkey::default(),
        status_flag::INITIALIZED,
    )
}

/// Write a PathwayPolicy into an account-backed MockAccount.
fn make_policy_account(policy: &PathwayPolicy) -> (Pubkey, Vec<u8>, u64) {
    let pid          = program_id();
    let (key, bump)  = Pubkey::find_program_address(
        &[seeds::PATHWAY_POLICY, policy.pathway_id.as_ref()],
        &pid,
    );
    let mut data     = vec![0u8; PATHWAY_POLICY_SIZE];
    let lamports     = 1u64;

    {
        // direct mut access to data
        let p: &mut PathwayPolicy = bytemuck::from_bytes_mut(&mut data[..]);

        *p      = *policy;
        p.bump  = bump;
    }
    (key, data, lamports)
}

// ─── Size / layout ────────────────────────────────────────────────────────────

#[test]
fn size_matches_declared() {
    assert_eq!(core::mem::size_of::<PathwayPolicy>(), 616);
    assert_eq!(PATHWAY_POLICY_SIZE, 616);
    assert_eq!(core::mem::offset_of!(PathwayPolicy, designated_executor), 144);
    assert_eq!(core::mem::offset_of!(PathwayPolicy, source_account_policy), 176);
    assert_eq!(core::mem::offset_of!(PathwayPolicy, executor_limit_policy_id), 520);
    assert_eq!(core::mem::offset_of!(PathwayPolicy, _reserved), 552);
    assert_eq!(core::mem::size_of_val(&PathwayPolicy::zeroed()._reserved), 64);
}

#[test]
fn zeroed_struct_all_zero_bytes() {
    let p     = PathwayPolicy::zeroed();
    let bytes : &[u8] = bytemuck::bytes_of(&p);

    assert!(bytes.iter().all(|&b| b == 0));
    assert!(p._reserved.iter().all(|&b| b == 0));
}

// ─── Load guards ──────────────────────────────────────────────────────────────

#[test]
fn load_zero_data_returns_not_initialized() {
    let pid          = program_id();
    let key          = Pubkey::new_unique();
    let mut data     = vec![0u8; PATHWAY_POLICY_SIZE];
    let mut lamports = 1u64;
    let ai           = make_account_info(&key, &mut data, &mut lamports, &pid, false);

    assert!(matches!(
        PathwayPolicy::load(&ai),
        Err(e) if e == ProgramError::Custom(ChanceryError::NotInitialized as u32),
    ));
}

#[test]
fn load_wrong_size_returns_length_mismatch() {
    let pid          = program_id();
    let key          = Pubkey::new_unique();
    let mut data     = vec![0u8; PATHWAY_POLICY_SIZE + 4];
    let mut lamports = 1u64;
    let ai           = make_account_info(&key, &mut data, &mut lamports, &pid, false);

    assert!(matches!(
        PathwayPolicy::load(&ai),
        Err(e) if e == ProgramError::Custom(ChanceryError::AccountDataLengthMismatch as u32),
    ));
}

#[test]
fn pre_release_648_byte_layout_is_rejected() {
    let pid          = program_id();
    let key          = Pubkey::new_unique();
    let mut data     = vec![0u8; 648];
    let mut lamports = 1u64;
    let ai           = make_account_info(&key, &mut data, &mut lamports, &pid, false);

    assert!(matches!(
        PathwayPolicy::load(&ai),
        Err(e) if e == ProgramError::Custom(ChanceryError::AccountDataLengthMismatch as u32),
    ));
}

#[test]
fn load_mut_non_writable_returns_error() {
    let p   = direct_policy();
    let (key, mut data, mut lamports) = make_policy_account(&p);
    let pid = program_id();
    let ai  = make_account_info(&key, &mut data, &mut lamports, &pid, false);

    assert!(matches!(
        PathwayPolicy::load_mut(&ai),
        Err(e) if e == ProgramError::Custom(ChanceryError::AccountNotWritable as u32),
    ));
}

#[test]
fn load_uninitialized_blocks_double_init() {
    let pid          = program_id();
    let key          = Pubkey::new_unique();
    let mut data     = vec![0u8; PATHWAY_POLICY_SIZE];
    let mut lamports = 1u64;

    {
        let ai = make_account_info(&key, &mut data, &mut lamports, &pid, true);
        let mut p  = PathwayPolicy::load_uninitialized_mut(&ai).unwrap();

        p.discriminator = PATHWAY_POLICY_DISCRIMINATOR;
    }
    {
        let ai = make_account_info(&key, &mut data, &mut lamports, &pid, true);

        assert!(matches!(
            PathwayPolicy::load_uninitialized_mut(&ai),
            Err(e) if e == ProgramError::Custom(ChanceryError::AlreadyInitialized as u32),
        ));
    }
}

#[test]
fn load_reads_fields_correctly() {
    let id     = [0xAAu8; 32];
    let executor = Pubkey::new_unique();
    let p = make_policy(
        id,
        pathway_kind::DELEGATED,
        executor,
        status_flag::INITIALIZED,
    );
    let (key, mut data, mut lamports) = make_policy_account(&p);
    let pid    = program_id();
    let ai     = make_account_info(&key, &mut data, &mut lamports, &pid, false);
    let loaded = PathwayPolicy::load(&ai).unwrap();

    assert_eq!(loaded.pathway_id,       id);
    assert_eq!(loaded.pathway_kind,     pathway_kind::DELEGATED);
    assert_eq!(loaded.designated_executor, executor);
    assert_eq!(loaded.status_flags & status_flag::INITIALIZED, status_flag::INITIALIZED);
}

// ─── assert_active ────────────────────────────────────────────────────────────

#[test]
fn assert_active_passes_when_initialized_and_not_paused() {
    let p = direct_policy();  // INITIALIZED, no PATHWAY_PAUSE

    assert!(p.assert_active().is_ok());
}

#[test]
fn assert_active_fails_when_not_initialized() {
    let mut p = direct_policy();

    p.status_flags &= !status_flag::INITIALIZED;

    assert!(matches!(
        p.assert_active(),
        Err(e) if e == ProgramError::Custom(ChanceryError::PathwayPolicyNotFound as u32),
    ));
}

#[test]
fn assert_active_fails_when_pathway_paused() {
    let mut p = direct_policy();

    p.status_flags |= status_flag::PATHWAY_PAUSE;

    assert!(matches!(
        p.assert_active(),
        Err(e) if e == ProgramError::Custom(ChanceryError::PathwayPaused as u32),
    ));
}

#[test]
fn assert_active_pause_takes_priority_over_initialized() {
    // Both flags set: PATHWAY_PAUSE check runs first
    let mut p = direct_policy();

    p.status_flags |= status_flag::PATHWAY_PAUSE;

    assert!(matches!(
        p.assert_active(),
        Err(e) if e == ProgramError::Custom(ChanceryError::PathwayPaused as u32),
    ));
}

// ─── assert_kind ─────────────────────────────────────────────────────────────

#[test]
fn assert_kind_direct_passes_for_direct_policy() {
    let p = make_policy(
        [1u8; 32],
        pathway_kind::DIRECT,
        Pubkey::default(),
        status_flag::INITIALIZED,
    );

    assert!(p.assert_kind(pathway_kind::DIRECT).is_ok());
}

#[test]
fn assert_kind_delegated_passes_for_delegated_policy() {
    let p = make_policy(
        [2u8; 32],
        pathway_kind::DELEGATED,
        Pubkey::default(),
        status_flag::INITIALIZED,
    );

    assert!(p.assert_kind(pathway_kind::DELEGATED).is_ok());
}

#[test]
fn assert_kind_trilateral_passes_for_trilateral_policy() {
    let p = make_policy(
        [3u8; 32],
        pathway_kind::TRILATERAL,
        Pubkey::default(),
        status_flag::INITIALIZED,
    );

    assert!(p.assert_kind(pathway_kind::TRILATERAL).is_ok());
}

#[test]
fn assert_kind_fails_when_kind_mismatches() {
    let p = make_policy(
        [1u8; 32],
        pathway_kind::DIRECT,
        Pubkey::default(),
        status_flag::INITIALIZED,
    );

    assert!(matches!(
        p.assert_kind(pathway_kind::DELEGATED),
        Err(e) if e == ProgramError::Custom(ChanceryError::PathwayKindMismatch as u32),
    ));
}

#[test]
fn assert_kind_all_three_kinds_are_distinct_values() {
    assert_ne!(pathway_kind::DIRECT,     pathway_kind::DELEGATED);
    assert_ne!(pathway_kind::DIRECT,     pathway_kind::TRILATERAL);
    assert_ne!(pathway_kind::DELEGATED,  pathway_kind::TRILATERAL);
}

// ─── assert_executor ──────────────────────────────────────────────────────────

#[test]
fn assert_executor_is_open_when_designated_executor_is_default() {
    let p = make_policy(
        [1u8; 32],
        pathway_kind::DELEGATED,
        Pubkey::default(),
        status_flag::INITIALIZED,
    );

    assert!(p.assert_executor(&Pubkey::new_unique()).is_ok());
    assert!(p.assert_executor(&Pubkey::new_unique()).is_ok());
}

#[test]
fn assert_executor_requires_exact_designated_executor() {
    let designated = Pubkey::new_unique();
    let other = Pubkey::new_unique();
    let p = make_policy(
        [1u8; 32],
        pathway_kind::DELEGATED,
        designated,
        status_flag::INITIALIZED,
    );

    assert!(p.assert_executor(&designated).is_ok());
    assert!(matches!(
        p.assert_executor(&other),
        Err(e) if e == ProgramError::Custom(ChanceryError::ExecutorNotPermitted as u32),
    ));
}

#[test]
fn assert_executor_default_pubkey_is_the_zero_sentinel() {
    assert_eq!(Pubkey::default(), Pubkey::new_from_array([0u8; 32]));
}

// ─── has_*_policy predicates ──────────────────────────────────────────────────

#[test]
fn has_fee_policy_false_when_id_all_zeros() {
    let p = direct_policy();

    assert!(!p.has_fee_policy(), "zero fee_policy_id must mean no fee policy");
}

#[test]
fn has_fee_policy_true_when_id_nonzero() {
    let mut p = direct_policy();
    p.fee_policy_id = [0x01u8; 32];

    assert!(p.has_fee_policy());
}

#[test]
fn has_limit_policy_false_when_id_all_zeros() {
    let p = direct_policy();

    assert!(!p.has_limit_policy());
}

#[test]
fn has_limit_policy_true_when_id_nonzero() {
    let mut p = direct_policy();

    p.limit_policy_id = [0xFFu8; 32];

    assert!(p.has_limit_policy());
}

#[test]
fn has_evidence_policy_false_when_id_all_zeros() {
    let p = direct_policy();

    assert!(!p.has_evidence_policy());
}

#[test]
fn has_evidence_policy_true_when_id_nonzero() {
    let mut p = direct_policy();

    p.evidence_policy_id = [0x42u8; 32];

    assert!(p.has_evidence_policy());
}

#[test]
fn all_policies_can_be_active_simultaneously() {
    let mut p = direct_policy();

    p.fee_policy_id      = [0x01u8; 32];
    p.limit_policy_id    = [0x02u8; 32];
    p.evidence_policy_id = [0x03u8; 32];

    assert!(p.has_fee_policy());
    assert!(p.has_limit_policy());
    assert!(p.has_evidence_policy());
}

#[test]
fn insurance_policy_zero_id_means_not_active() {
    let p = direct_policy();

    assert_eq!(p.insurance_policy_id, [0u8; 32],
        "default insurance_policy_id must be zero");
}

// ─── Extension mask guards ────────────────────────────────────────────────────

#[test]
fn collateral_extensions_allowed_when_no_forbidden() {
    let mut p = direct_policy();

    p.forbidden_collateral_extension_mask = [0u64; 2];

    assert!(p.assert_collateral_extensions_allowed([0xFF, 0xFF]).is_ok());
}

#[test]
fn collateral_extensions_blocked_when_forbidden_bit_observed() {
    let mut p = direct_policy();

    p.forbidden_collateral_extension_mask = [1u64, 0u64];  // bit 0 forbidden

    assert!(matches!(
        p.assert_collateral_extensions_allowed([1u64, 0u64]),
        Err(e) if e == ProgramError::Custom(ChanceryError::ForbiddenExtension as u32),
    ));
}

#[test]
fn collateral_extensions_pass_when_observed_does_not_overlap_forbidden() {
    let mut p = direct_policy();

    p.forbidden_collateral_extension_mask = [0b10u64, 0u64];  // bit 1 forbidden

    assert!(p.assert_collateral_extensions_allowed([0b01u64, 0u64]).is_ok());  // bit 0 only
}

#[test]
fn collateral_extensions_forbidden_high_word() {
    let mut p = direct_policy();

    p.forbidden_collateral_extension_mask = [0u64, 1u64];  // high-word bit 0 forbidden

    assert!(matches!(
        p.assert_collateral_extensions_allowed([0u64, 1u64]),
        Err(e) if e == ProgramError::Custom(ChanceryError::ForbiddenExtension as u32),
    ));
}

#[test]
fn issued_token_extensions_allowed_when_no_forbidden() {
    let mut p = direct_policy();

    p.forbidden_issued_token_extension_mask = [0u64; 2];

    assert!(p.assert_issued_token_extensions_allowed([0xFF, 0xFF]).is_ok());
}

#[test]
fn issued_token_extensions_blocked_when_forbidden_bit_observed() {
    let mut p = direct_policy();

    p.forbidden_issued_token_extension_mask = [0b100u64, 0u64];  // bit 2 forbidden

    assert!(matches!(
        p.assert_issued_token_extensions_allowed([0b100u64, 0u64]),
        Err(e) if e == ProgramError::Custom(ChanceryError::ForbiddenExtension as u32),
    ));
}

#[test]
fn issued_token_extensions_pass_when_no_overlap() {
    let mut p = direct_policy();

    p.forbidden_issued_token_extension_mask = [0b100u64, 0u64];  // bit 2 forbidden

    assert!(p.assert_issued_token_extensions_allowed([0b011u64, 0u64]).is_ok());  // bits 0,1 only
}

// ─── PDA derivation ───────────────────────────────────────────────────────────

#[test]
fn pda_is_deterministic() {
    let pid      = program_id();
    let id       = [0xBBu8; 32];
    let (k1, b1) = PathwayPolicy::pda(&id, &pid);
    let (k2, b2) = PathwayPolicy::pda(&id, &pid);

    assert_eq!(k1, k2);
    assert_eq!(b1, b2);
}

#[test]
fn different_pathway_ids_produce_different_pdas() {
    let pid     = program_id();
    let (k1, _) = PathwayPolicy::pda(&[0x01u8; 32], &pid);
    let (k2, _) = PathwayPolicy::pda(&[0x02u8; 32], &pid);

    assert_ne!(k1, k2);
}

#[test]
fn pda_changes_when_single_byte_of_pathway_id_differs() {
    let pid      = program_id();
    let mut id_a = [0u8; 32];
    let mut id_b = [0u8; 32];

    id_a[0] = 0x01;
    id_b[0] = 0x02;

    let (ka, _) = PathwayPolicy::pda(&id_a, &pid);
    let (kb, _) = PathwayPolicy::pda(&id_b, &pid);

    assert_ne!(ka, kb);
}

#[test]
fn ten_distinct_pathway_ids_produce_ten_distinct_pdas() {
    let pid = program_id();
    let keys: Vec<Pubkey> = (0u8..10)
        .map(|i| {
            let mut id = [0u8; 32];
            id[31] = i;
            PathwayPolicy::pda(&id, &pid).0
        })
        .collect();
    let unique: std::collections::HashSet<_> = keys.iter().collect();

    assert_eq!(unique.len(), 10, "10 distinct pathway IDs must produce 10 distinct PDAs");
}

// ─── Runtime support for dimension policies ──────────────────────────────────

#[test]
fn direct_pathway_rejects_executor_dimension_policy() {
    let mut p = direct_policy();
    p.executor_limit_policy_id = [1u8; 32];

    assert!(matches!(
        p.assert_runtime_supported(),
        Err(e) if e == ProgramError::Custom(ChanceryError::PathwayDependencyMissing as u32),
    ));
}

#[test]
fn delegated_and_trilateral_pathways_accept_executor_dimension_policy() {
    for kind in [pathway_kind::DELEGATED, pathway_kind::TRILATERAL] {
        let mut p = make_policy(
            [kind; 32],
            kind,
            Pubkey::default(),
            status_flag::INITIALIZED,
        );
        p.executor_limit_policy_id = [2u8; 32];
        if kind == pathway_kind::DELEGATED {
            p.counterparty_limit_policy_id = [3u8; 32];
        }

        assert!(p.assert_runtime_supported().is_ok());
    }
}

#[test]
fn delegated_pathway_requires_counterparty_dimension_policy() {
    let p = make_policy(
        [pathway_kind::DELEGATED; 32],
        pathway_kind::DELEGATED,
        Pubkey::default(),
        status_flag::INITIALIZED,
    );

    assert!(matches!(
        p.assert_runtime_supported(),
        Err(e) if e == ProgramError::Custom(ChanceryError::PathwayDependencyMissing as u32),
    ));
}

#[test]
fn trilateral_pathway_keeps_counterparty_dimension_optional() {
    let p = make_policy(
        [pathway_kind::TRILATERAL; 32],
        pathway_kind::TRILATERAL,
        Pubkey::default(),
        status_flag::INITIALIZED,
    );

    assert!(p.assert_runtime_supported().is_ok());
}

#[test]
fn cross_chain_pathways_reject_unenforced_dimension_references() {
    for kind in [pathway_kind::CROSS_CHAIN_MINT, pathway_kind::CROSS_CHAIN_REDEEM] {
        let mut p = make_policy(
            [kind; 32],
            kind,
            Pubkey::default(),
            status_flag::INITIALIZED,
        );
        p.counterparty_limit_policy_id = [3u8; 32];

        assert!(matches!(
            p.assert_runtime_supported(),
            Err(e) if e == ProgramError::Custom(ChanceryError::PathwayDependencyMissing as u32),
        ));
    }
}

#[test]
fn cross_chain_pathways_reject_unenforced_evidence_policy_references() {
    for kind in [pathway_kind::CROSS_CHAIN_MINT, pathway_kind::CROSS_CHAIN_REDEEM] {
        let mut p = make_policy(
            [kind; 32],
            kind,
            Pubkey::default(),
            status_flag::INITIALIZED,
        );
        p.evidence_policy_id = [4u8; 32];

        assert!(matches!(
            p.assert_runtime_supported(),
            Err(e) if e == ProgramError::Custom(ChanceryError::PathwayDependencyMissing as u32),
        ));
    }
}

// ─── pathway_kind constants ───────────────────────────────────────────────────

#[test]
fn pathway_kind_constants_match_settlement_mode_constants() {
    use crate::constants::settlement_mode;

    // pathway_kind and settlement_mode share the same ordinal values
    // so handlers can use either for matching
    assert_eq!(pathway_kind::DIRECT     as u8, settlement_mode::DIRECT_PRINCIPAL);
    assert_eq!(pathway_kind::DELEGATED  as u8, settlement_mode::DELEGATED_NON_CUSTODIAL);
    assert_eq!(pathway_kind::TRILATERAL as u8, settlement_mode::TRILATERAL_ATOMIC);
}

// ─── register_pathway_policy canonical mint (#49 / #50) ──────────────────────

#[test]
fn register_pathway_rejects_issued_token_mint_not_in_chancery_config() {
    use crate::modules::{
        core::state::chancery_config::{
            ChanceryConfig, CHANCERY_CONFIG_DISCRIMINATOR, CHANCERY_CONFIG_SIZE,
        },
        pathway::instructions::register_pathway_policy::handle,
    };

    let program_id   = program_id();
    let governance   = Pubkey::new_unique();
    let ops          = Pubkey::new_unique();
    let canonical    = Pubkey::new_unique();
    let wrong        = Pubkey::new_unique();
    let payer_key    = Pubkey::new_unique();
    let pathway_key  = Pubkey::new_unique();
    let system_key   = Pubkey::new_unique();

    let (cfg_key, cfg_bump) = Pubkey::find_program_address(&[seeds::CHANCERY_CONFIG], &program_id);

    let mut cfg_data = vec![0u8; CHANCERY_CONFIG_SIZE];
    {
        let cfg: &mut ChanceryConfig = bytemuck::from_bytes_mut(&mut cfg_data[..]);
        cfg.discriminator        = CHANCERY_CONFIG_DISCRIMINATOR;
        cfg.version              = 1;
        cfg.bump                 = cfg_bump;
        cfg.status_flags         = status_flag::INITIALIZED;
        cfg.governance_authority = governance;
        cfg.operations_authority = ops;
        cfg.issued_token_mint    = canonical;
    }

    let mut cfg_lamports     = 1u64;
    let mut payer_lamports   = 1u64;
    let mut ops_lamports     = 1u64;
    let mut pathway_lamports = 1u64;
    let mut sys_lamports     = 0u64;
    let mut payer_data       = vec![0u8];
    let mut ops_data         = vec![0u8];
    let mut pathway_data     = vec![0u8; PATHWAY_POLICY_SIZE];
    let mut sys_data         = vec![];

    let evt_key          = Pubkey::new_unique();
    let mut evt_lamports = 1u64;
    let mut evt_data     = vec![0u8];

    let cfg_ai      = make_account_info(&cfg_key, &mut cfg_data, &mut cfg_lamports, &program_id, true);
    let evt_ai      = make_account_info(&evt_key, &mut evt_data, &mut evt_lamports, &program_id, false);
    let pathway_ai  = make_account_info(&pathway_key, &mut pathway_data, &mut pathway_lamports, &program_id, true);
    let mut payer_ai = make_account_info(&payer_key, &mut payer_data, &mut payer_lamports, &program_id, true);
    payer_ai.is_signer = true;
    let mut ops_ai = make_account_info(&ops, &mut ops_data, &mut ops_lamports, &program_id, true);
    ops_ai.is_signer = true;
    let sys_ai = make_account_info(&system_key, &mut sys_data, &mut sys_lamports, &program_id, false);

    let pathway_id = [0x01u8; 32];
    let asset_mint = Pubkey::new_unique();
    let mut args_data = Vec::new();
    args_data.extend_from_slice(&pathway_id);
    args_data.push(pathway_kind::DIRECT);
    args_data.extend_from_slice(asset_mint.as_ref());
    args_data.extend_from_slice(wrong.as_ref());
    args_data.extend_from_slice(Pubkey::default().as_ref());
    // reserve_compartment / limit / evidence / fee / insurance policy ids,
    // then the four RB-04 dimension policy ids (asset mint / asset redeem /
    // counterparty / executor) - all zero.
    for _ in 0..9 {
        args_data.extend_from_slice(&[0u8; 32]);
    }
    assert_eq!(args_data.len(), 417, "single-executor registration ABI length");
    // Registration has a fixed account family for every optional pathway
    // reference. This test must include the eight zero-key sentinels so the
    // handler reaches the issued-token-mint validation rather than failing
    // early with MissingAccount.
    let default_key = Pubkey::default();
    let mut reference_lamports_0 = 0u64;
    let mut reference_lamports_1 = 0u64;
    let mut reference_lamports_2 = 0u64;
    let mut reference_lamports_3 = 0u64;
    let mut reference_lamports_4 = 0u64;
    let mut reference_lamports_5 = 0u64;
    let mut reference_lamports_6 = 0u64;
    let mut reference_lamports_7 = 0u64;
    let mut reference_data_0 = vec![];
    let mut reference_data_1 = vec![];
    let mut reference_data_2 = vec![];
    let mut reference_data_3 = vec![];
    let mut reference_data_4 = vec![];
    let mut reference_data_5 = vec![];
    let mut reference_data_6 = vec![];
    let mut reference_data_7 = vec![];
    let reference_0 = make_account_info(&default_key, &mut reference_data_0, &mut reference_lamports_0, &program_id, false);
    let reference_1 = make_account_info(&default_key, &mut reference_data_1, &mut reference_lamports_1, &program_id, false);
    let reference_2 = make_account_info(&default_key, &mut reference_data_2, &mut reference_lamports_2, &program_id, false);
    let reference_3 = make_account_info(&default_key, &mut reference_data_3, &mut reference_lamports_3, &program_id, false);
    let reference_4 = make_account_info(&default_key, &mut reference_data_4, &mut reference_lamports_4, &program_id, false);
    let reference_5 = make_account_info(&default_key, &mut reference_data_5, &mut reference_lamports_5, &program_id, false);
    let reference_6 = make_account_info(&default_key, &mut reference_data_6, &mut reference_lamports_6, &program_id, false);
    let reference_7 = make_account_info(&default_key, &mut reference_data_7, &mut reference_lamports_7, &program_id, false);

    let accounts = [
        cfg_ai,
        evt_ai,
        pathway_ai,
        payer_ai,
        ops_ai,
        sys_ai,
        reference_0,
        reference_1,
        reference_2,
        reference_3,
        reference_4,
        reference_5,
        reference_6,
        reference_7,
    ];

    assert!(matches!(
        handle(&accounts, &args_data),
        Err(e) if e == ProgramError::Custom(ChanceryError::AccountKeyMismatch as u32),
    ));
}
