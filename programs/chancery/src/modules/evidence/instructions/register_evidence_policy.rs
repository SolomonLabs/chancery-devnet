/// register_evidence_policy
///
/// Accounts:
///   0  chancery_config      writable  PDA
///   1  event_authority      readable  PDA [b"event-authority"]
///   2  evidence_policy   writable  PDA [b"evidence-policy", evidence_policy_id]
///   3  payer             signer
///   4  operations_authority     signer
///   5  system_program

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    constants::seeds,
    error::ChanceryError,
    modules::{
        core::{instructions::create_pda_account, state::chancery_config::ChanceryConfig},
        evidence::emit::{emit_evidence_policy_registered, EvidencePolicyRegistered},
        evidence::state::evidence_policy::{
            EvidencePolicy, EVIDENCE_POLICY_DISCRIMINATOR, EVIDENCE_POLICY_SIZE,
        },
    },
};

const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const EVIDENCE_POLICY:        usize = 2;
const PAYER:                  usize = 3;
const OPERATIONS_AUTHORITY:   usize = 4;
const SYSTEM_PROGRAM:         usize = 5;
const REQUIRED_ACCOUNT_COUNT: usize = 6;

#[derive(BorshDeserialize)]
pub struct RegisterEvidencePolicyArgs {
    pub evidence_policy_id:                 [u8; 32],
    pub required_field_mask:                [u64; 2],
    pub counterparty_reporting_schema_hash: [u8; 32],
    pub allow_freeform_counterparty_fields: bool,
    pub maximum_freeform_field_count:       u16,
    pub maximum_freeform_value_bytes:       u16,
    pub retention_flags:                    u64,
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info      = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];
    let evidence_policy_account_info      = &accounts[EVIDENCE_POLICY];
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

    if !evidence_policy_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;


    if operations_authority_account_info.key != &chancery_config.operations_authority {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    let args = RegisterEvidencePolicyArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    if args.evidence_policy_id == [0u8; 32] {
        return Err(ChanceryError::EvidencePolicyUnsupportedConfiguration.into());
    }

    let program_id           = crate::id();
    let (expected_key, bump) = Pubkey::find_program_address(
        &[seeds::EVIDENCE_POLICY, args.evidence_policy_id.as_ref()],
        &program_id,
    );

    if evidence_policy_account_info.key != &expected_key {
        return Err(ChanceryError::InvalidPda.into());
    }

    if evidence_policy_account_info.data_is_empty() {
        create_pda_account(
            payer_account_info, evidence_policy_account_info, system_program_account_info, &program_id,
            &[seeds::EVIDENCE_POLICY, args.evidence_policy_id.as_ref(), &[bump]],
            EVIDENCE_POLICY_SIZE,
        )?;
    } else if evidence_policy_account_info.data_len() != EVIDENCE_POLICY_SIZE {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    let mut p = EvidencePolicy::load_uninitialized_mut(evidence_policy_account_info)?;

    p.discriminator                      = EVIDENCE_POLICY_DISCRIMINATOR;
    p.version                            = 1;
    p.bump                               = bump;
    p.allow_freeform_counterparty_fields = args.allow_freeform_counterparty_fields as u8;
    p._pad0                              = [0u8; 4];
    p.evidence_policy_id                 = args.evidence_policy_id;
    p.required_field_mask                = args.required_field_mask;
    p.counterparty_reporting_schema_hash = args.counterparty_reporting_schema_hash;
    p.maximum_freeform_field_count       = args.maximum_freeform_field_count;
    p.maximum_freeform_value_bytes       = args.maximum_freeform_value_bytes;
    p._pad1                              = [0u8; 4];
    p.retention_flags                    = args.retention_flags;
    p._reserved                          = [0u8; 32];

    p.assert_runtime_supported()?;
    drop(p);

    // ── Evidence (emitted last, after all state writes) ─────────────────────────
    let clock = Clock::get()?;

    drop(chancery_config);

    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;

    emit_evidence_policy_registered(
        event_authority_account_info,
        event_authority_bump,
        EvidencePolicyRegistered {
            sequence_nonce,
            chancery:        *chancery_config_account_info.key,
            slot:            clock.slot,
            unix_timestamp:  clock.unix_timestamp,
            risk_class:      0,
            evidence_policy: *evidence_policy_account_info.key,
            registered_by:   *operations_authority_account_info.key,
        },
    )?;

    Ok(())
}
