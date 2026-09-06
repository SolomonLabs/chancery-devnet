/// rotate_cross_chain_signer_set
///
/// Marks an existing `CrossChainSignerSet` as revoked by setting the
/// `cross_chain_signer_set_flag::REVOKED` bit on its `status_flags`.
/// Revocation is sticky - the set is preserved on-chain for audit, and
/// `assert_active` rejects revoked sets unconditionally regardless of
/// their validity window.
///
/// A planned replacement is a multi-ceremony operation, not a single atomic
/// transaction:
///   1. register the replacement set through its accepted `Dangerous` change;
///   2. move each remote-domain policy through an accepted policy change;
///   3. revoke this OLD set after every intended policy migration is final.
///
/// Emergency response may revoke first to stop inbound consumption
/// immediately. Policies that still reference the revoked set then fail
/// closed until governance completes replacement proposals.
///
/// Accounts:
///   0  chancery_config                writable  PDA  (sequence_nonce incremented for evidence)
///   1  event_authority                readable  PDA [b"event-authority"]
///   2  signer_set                     writable  PDA [b"cross-chain-signer-set", signer_set_id]
///   3  authority                      signer
///   4  authority_permission_record    readable  PDA [b"permission", authority, CROSS_CHAIN_SIGNER_SET, signer_set]
///
/// Authority: signer must hold `CAN_ROTATE_CROSS_CHAIN_SIGNER_SET` on a
/// `PermissionRecord` scoped to `(scope::CROSS_CHAIN_SIGNER_SET, signer_set)`.
/// Registration uses its separate `CAN_REGISTER_CROSS_CHAIN_SIGNER_SET` role and an accepted pending change.

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_sysvar::Sysvar;

use crate::{
    constants::{cross_chain_signer_set_flag, role, scope},
    error::ChanceryError,
    modules::{
        core::state::chancery_config::ChanceryConfig,
        cross_chain::{
            auth::assert_signer_holds_role,
            instructions::{
                assert_disjoint_cross_chain_account_index_groups,
                assert_distinct_cross_chain_account_indexes,
            },
            state::cross_chain_signer_set::CrossChainSignerSet,
        },
        evidence::emit::{emit_cross_chain_signer_set_rotated, CrossChainSignerSetRotated},
    },
};

const CHANCERY_CONFIG:               usize = 0;
const EVENT_AUTHORITY:               usize = 1;
const SIGNER_SET:                    usize = 2;
const AUTHORITY:                     usize = 3;
const AUTHORITY_PERMISSION_RECORD:   usize = 4;
const REQUIRED_ACCOUNT_COUNT:        usize = 5;

const PROTECTED_ACCOUNT_INDEXES:  &[usize] = &[
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    SIGNER_SET,
];
const IDENTITY_ACCOUNT_INDEXES:   &[usize] = &[AUTHORITY];
const PERMISSION_ACCOUNT_INDEXES: &[usize] = &[AUTHORITY_PERMISSION_RECORD];

#[derive(BorshDeserialize)]
pub struct RotateCrossChainSignerSetArgs {
    pub signer_set_id: [u8; 32],
    pub reason_code:   u16,
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    assert_distinct_cross_chain_account_indexes(accounts, PROTECTED_ACCOUNT_INDEXES)?;
    assert_disjoint_cross_chain_account_index_groups(
        accounts,
        PROTECTED_ACCOUNT_INDEXES,
        IDENTITY_ACCOUNT_INDEXES,
    )?;
    assert_disjoint_cross_chain_account_index_groups(
        accounts,
        PROTECTED_ACCOUNT_INDEXES,
        PERMISSION_ACCOUNT_INDEXES,
    )?;
    assert_disjoint_cross_chain_account_index_groups(
        accounts,
        IDENTITY_ACCOUNT_INDEXES,
        PERMISSION_ACCOUNT_INDEXES,
    )?;

    let chancery_config_account_info             = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info             = &accounts[EVENT_AUTHORITY];
    let signer_set_account_info                  = &accounts[SIGNER_SET];
    let authority_account_info                   = &accounts[AUTHORITY];
    let authority_permission_record_account_info = &accounts[AUTHORITY_PERMISSION_RECORD];

    if !signer_set_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    // assert_not_paused intentionally NOT checked: emergency revocation must
    // succeed even when the chancery is globally paused.
    let args       = RotateCrossChainSignerSetArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let program_id = crate::id();

    let signer_set_bump = CrossChainSignerSet::verify_pda(signer_set_account_info, &args.signer_set_id, &program_id)?;

    assert_signer_holds_role(
        authority_account_info,
        authority_permission_record_account_info,
        role::CAN_ROTATE_CROSS_CHAIN_SIGNER_SET,
        scope::CROSS_CHAIN_SIGNER_SET,
        signer_set_account_info.key,
        &program_id,
    )?;

    let mut s = CrossChainSignerSet::load_mut_for_verified_pda(
        signer_set_account_info,
        &args.signer_set_id,
        signer_set_bump,
    )?;

    // Reject double-revocation explicitly: assert_active fails on REVOKED.
    s.assert_active()?;

    s.status_flags |= cross_chain_signer_set_flag::REVOKED;

    // ── Emit evidence ─────────────────────────────────────────────────────────
    let clock                = Clock::get()?;
    let mut chancery_config_mut  = ChanceryConfig::load_verified_mut(chancery_config_account_info)?;
    let event_authority_bump = chancery_config_mut.event_authority_bump;
    let seq                  = chancery_config_mut.next_sequence_nonce()?;

    emit_cross_chain_signer_set_rotated(
        event_authority_account_info,
        event_authority_bump,
        CrossChainSignerSetRotated {
            sequence_nonce:         seq,
            chancery:               *chancery_config_account_info.key,
            slot:                   clock.slot,
            unix_timestamp:         clock.unix_timestamp,
            cross_chain_signer_set: *signer_set_account_info.key,
            signer_set_id:          args.signer_set_id,
            reason_code:            args.reason_code,
            rotated_by:             *authority_account_info.key,
        },
    )?;

    Ok(())
}
