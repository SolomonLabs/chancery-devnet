/// register_pathway_policy
///
/// Creates a PathwayPolicyPda. Every settlement pathway must have one.
///
/// Accounts:
///   0  chancery_config            writable  PDA
///   1  event_authority            readable  PDA [b"event-authority"]
///   2  pathway_policy             writable  PDA [b"pathway-policy", pathway_id]
///   3  payer                      signer
///   4  operations_authority       signer    must be chancery_config.operations_authority
///   5  system_program
///   6  limit_policy                    optional  referenced pathway policy
///   7  evidence_policy                 optional  referenced evidence policy
///   8  fee_policy                      optional  referenced fee policy
///   9  insurance_policy                optional  referenced insurance policy
///  10  asset_mint_limit_policy         optional  referenced ASSET policy
///  11  asset_redeem_limit_policy       optional  referenced ASSET policy
///  12  counterparty_limit_policy       optional  referenced COUNTERPARTY template
///  13  executor_limit_policy           optional  referenced EXECUTOR template

use borsh::BorshDeserialize;
use bytemuck::Zeroable;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sha256_hasher::hashv;
use solana_sysvar::Sysvar;

use crate::{
    constants::{pathway_kind, seeds, status_flag},
    error::ChanceryError,
    modules::{
        core::{
            instructions::create_pda_account,
            state::chancery_config::ChanceryConfig,
        },
        evidence::emit::{emit_pathway_policy_registered, PathwayPolicyRegistered},
        pathway::{
            reference_validation::{validate_pathway_policy_references, PathwayPolicyReferenceAccounts},
            state::pathway_policy::{
                PathwayPolicy, PATHWAY_POLICY_DISCRIMINATOR, PATHWAY_POLICY_SIZE,
            },
        },
    },
};

const CHANCERY_CONFIG:           usize = 0;
const EVENT_AUTHORITY:           usize = 1;
const PATHWAY_POLICY:            usize = 2;
const PAYER:                     usize = 3;
const OPERATIONS_AUTHORITY:      usize = 4;
const SYSTEM_PROGRAM:            usize = 5;
const LIMIT_POLICY:              usize = 6;
const EVIDENCE_POLICY:           usize = 7;
const FEE_POLICY:                usize = 8;
const INSURANCE_POLICY:          usize = 9;
const ASSET_MINT_LIMIT_POLICY:   usize = 10;
const ASSET_REDEEM_LIMIT_POLICY: usize = 11;
const COUNTERPARTY_LIMIT_POLICY: usize = 12;
const EXECUTOR_LIMIT_POLICY:     usize = 13;
const REQUIRED_ACCOUNT_COUNT:    usize = 14;

#[derive(BorshDeserialize)]
pub struct RegisterPathwayPolicyArgs {
    pub pathway_id:                    [u8; 32],
    pub pathway_kind:                  u8,
    pub asset_mint:                    Pubkey,
    pub issued_token_mint:             Pubkey,
    pub designated_executor:           Pubkey,
    pub reserve_compartment_policy_id: [u8; 32],
    pub limit_policy_id:               [u8; 32],
    pub evidence_policy_id:            [u8; 32],
    pub fee_policy_id:                 [u8; 32],
    pub insurance_policy_id:           [u8; 32],
    pub asset_mint_limit_policy_id:    [u8; 32],
    pub asset_redeem_limit_policy_id:  [u8; 32],
    pub counterparty_limit_policy_id:  [u8; 32],
    pub executor_limit_policy_id:      [u8; 32],
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info      = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];
    let pathway_policy_account_info       = &accounts[PATHWAY_POLICY];
    let payer_account_info                = &accounts[PAYER];
    let operations_authority_account_info = &accounts[OPERATIONS_AUTHORITY];
    let system_program_account_info       = &accounts[SYSTEM_PROGRAM];

    if !payer_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !operations_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !pathway_policy_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;


    if operations_authority_account_info.key != &chancery_config.operations_authority {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    let args = RegisterPathwayPolicyArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    if &args.issued_token_mint != &chancery_config.issued_token_mint {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    // Cross-chain corridors must be unique per (kind, asset_mint, issued_token_mint)
    // so a relayer cannot pick which pathway's policies apply at consume/emit time.
    // Pinning pathway_id to the canonical hash makes the PDA address itself the gate:
    // a second registration for the same triple hits an already-initialized PDA.
    if matches!(
        args.pathway_kind,
        pathway_kind::CROSS_CHAIN_MINT | pathway_kind::CROSS_CHAIN_REDEEM,
    ) {
        let canonical_pathway_id = hashv(&[
            &[args.pathway_kind],
            args.asset_mint.as_ref(),
            args.issued_token_mint.as_ref(),
        ]).to_bytes();

        if args.pathway_id != canonical_pathway_id {
            return Err(ChanceryError::CrossChainPathwayIdNotCanonical.into());
        }
    }

    let program_id = crate::id();
    let (expected_key, bump) = Pubkey::find_program_address(
        &[seeds::PATHWAY_POLICY, args.pathway_id.as_ref()],
        &program_id,
    );

    if pathway_policy_account_info.key != &expected_key {
        return Err(ChanceryError::InvalidPda.into());
    }

    let requires_allocation = pathway_policy_account_info.data_is_empty();
    if !requires_allocation && pathway_policy_account_info.data_len() != PATHWAY_POLICY_SIZE {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    let mut proposed = PathwayPolicy::zeroed();
    proposed.discriminator                         = PATHWAY_POLICY_DISCRIMINATOR;
    proposed.version                               = 1;
    proposed.bump                                  = bump;
    proposed.pathway_kind                          = args.pathway_kind;
    proposed._pad0                                 = [0u8; 4];
    proposed.pathway_id                            = args.pathway_id;
    proposed.asset_mint                            = args.asset_mint;
    proposed.issued_token_mint                     = args.issued_token_mint;
    proposed.allowed_program_mask                  = [0u64; 2];
    proposed.allowed_instruction_mask              = [0u64; 2];
    proposed.designated_executor                   = args.designated_executor;
    proposed.source_account_policy                 = 0;
    proposed.destination_account_policy            = 0;
    proposed.reserve_compartment_policy_id         = args.reserve_compartment_policy_id;
    proposed.limit_policy_id                       = args.limit_policy_id;
    proposed.evidence_policy_id                    = args.evidence_policy_id;
    proposed.fee_policy_id                         = args.fee_policy_id;
    proposed.insurance_policy_id                   = args.insurance_policy_id;
    proposed.status_flags                          = status_flag::INITIALIZED;
    proposed.required_issued_token_module_mask     = [0u64; 2];
    proposed.required_collateral_module_mask       = [0u64; 2];
    proposed.forbidden_issued_token_extension_mask = [0u64; 2];
    proposed.forbidden_collateral_extension_mask   = [0u64; 2];
    proposed.asset_mint_limit_policy_id            = args.asset_mint_limit_policy_id;
    proposed.asset_redeem_limit_policy_id          = args.asset_redeem_limit_policy_id;
    proposed.counterparty_limit_policy_id          = args.counterparty_limit_policy_id;
    proposed.executor_limit_policy_id              = args.executor_limit_policy_id;

    proposed.zero_reserved();
    proposed.assert_runtime_supported()?;

    let reference_accounts = PathwayPolicyReferenceAccounts {
        limit_policy:              &accounts[LIMIT_POLICY],
        evidence_policy:           &accounts[EVIDENCE_POLICY],
        fee_policy:                &accounts[FEE_POLICY],
        insurance_policy:          &accounts[INSURANCE_POLICY],
        asset_mint_limit_policy:   &accounts[ASSET_MINT_LIMIT_POLICY],
        asset_redeem_limit_policy: &accounts[ASSET_REDEEM_LIMIT_POLICY],
        counterparty_limit_policy: &accounts[COUNTERPARTY_LIMIT_POLICY],
        executor_limit_policy:     &accounts[EXECUTOR_LIMIT_POLICY],
    };
    validate_pathway_policy_references(
        pathway_policy_account_info.key,
        &proposed,
        &reference_accounts,
    )?;

    // Allocate only after every semantic and referenced-policy validation has
    // succeeded. Transaction rollback is atomic either way, but this ordering
    // avoids a system-program CPI on a configuration that is already known to
    // be invalid and keeps the registration pattern consistent with the other
    // pending-change constructors.
    if requires_allocation {
        create_pda_account(
            payer_account_info,
            pathway_policy_account_info,
            system_program_account_info,
            &program_id,
            &[seeds::PATHWAY_POLICY, args.pathway_id.as_ref(), &[bump]],
            PATHWAY_POLICY_SIZE,
        )?;
    }

    {
        let mut p = PathwayPolicy::load_uninitialized_mut(pathway_policy_account_info)?;
        *p = proposed;
    }

    // ── Evidence (emitted last, after all state writes) ─────────────────────────
    let clock = Clock::get()?;

    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;

    // Total-ever pathway-policy count (doc 14 §14.6.4).
    chancery_config_mut.total_pathway_policies_registered = chancery_config_mut
        .total_pathway_policies_registered
        .checked_add(1)
        .ok_or(ChanceryError::ArithmeticOverflow)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    emit_pathway_policy_registered(
        event_authority_account_info,
        event_authority_bump,
        PathwayPolicyRegistered {
            sequence_nonce,
            chancery:       *chancery_config_account_info.key,
            slot:           clock.slot,
            unix_timestamp: clock.unix_timestamp,
            risk_class:     0,
            pathway_policy: *pathway_policy_account_info.key,
            pathway_id:     args.pathway_id,
            registered_by:  *operations_authority_account_info.key,
        },
    )?;

    Ok(())
}
