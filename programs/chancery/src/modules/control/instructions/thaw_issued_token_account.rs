//! thaw_issued_token_account.
//!
//! Mirror of `freeze_issued_token_account`. Differences:
//!   - Global execution pause does not gate thaw; widening authority is
//!     enforced separately through the actor and scoped-permission checks
//!   - Authority list: governance | ops | enforcement (NOT emergency)
//!     OR holder of CAN_THAW_TOKEN_ACCOUNT at narrow scope
//!   - Requires existing BasicFreezeRecord; cannot thaw an unfrozen account
//!
//! Wire format:
//!   [ CONTROL(0x09) | THAW_ISSUED_TOKEN_ACCOUNT(0x06) | borsh(args) ]
//!
//! Accounts:
//!   0  chancery_config            writable PDA  (sequence_nonce for evidence)
//!   1  event_authority            readable PDA
//!   2  module_activation_state    readable PDA
//!   3  basic_freeze_record        writable PDA
//!   4  issued_token_account       writable
//!   5  issued_token_mint          readable
//!   6  freeze_authority_pda       readable PDA
//!   7  issued_token_program       readable
//!   8  authority                  signer
//!   9  permission_record          optional  (when authority is non-actor signer)

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_sysvar::Sysvar;

use crate::{
    constants::{basic_freeze_status, module},
    error::ChanceryError,
    modules::{
        core::{
            instructions::close_pda_to_recipient_account,
            state::chancery_config::ChanceryConfig,
        },
        control::{
            change_risk::ConfigChangeRiskClass,
            instructions::cpi::{cpi_thaw_account, freeze_authority_bump},
            state::{
                basic_freeze_record::BasicFreezeRecord,
                module_activation_state::ModuleActivationState,
            },
        },
        evidence::emit::{emit_basic_token_thaw, BasicTokenThaw},
        permissions::auth::assert_can_thaw_token_account,
    },
};

const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const ACTIVATION:             usize = 2;
const FREEZE_RECORD:          usize = 3;
const ISSUED_TOKEN_ACCOUNT:   usize = 4;
const ISSUED_TOKEN_MINT:      usize = 5;
const FREEZE_AUTHORITY_PDA:   usize = 6;
const ISSUED_TOKEN_PROGRAM:   usize = 7;
const AUTHORITY:              usize = 8;
const PERMISSION_RECORD:      usize = 9;
const REQUIRED_ACCOUNT_COUNT: usize = 9;

// Refund conveyance: the account whose key equals the record's stored
// rent_refund_recipient. Required whenever this instruction closes the
// record - the refund is conveyed, never redirected.
const RENT_REFUND_RECIPIENT:  usize = 10;

#[derive(BorshDeserialize)]
pub struct ThawIssuedTokenAccountArgs {
    pub reason_code: u32,
    pub thaw_flags:  u64,
}

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let cfg_account_info                  = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];
    let activation_account_info           = &accounts[ACTIVATION];
    let freeze_rec_account_info           = &accounts[FREEZE_RECORD];
    let issued_token_account_info         = &accounts[ISSUED_TOKEN_ACCOUNT];
    let issued_token_mint_account_info    = &accounts[ISSUED_TOKEN_MINT];
    let freeze_authority_account_info     = &accounts[FREEZE_AUTHORITY_PDA];
    let issued_token_program_account_info = &accounts[ISSUED_TOKEN_PROGRAM];
    let authority_account_info            = &accounts[AUTHORITY];

    if !authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !cfg_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !freeze_rec_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !issued_token_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let activation = ModuleActivationState::load_verified(activation_account_info)?;

    activation.assert_active(module::CONTROL)?;

    let args = ThawIssuedTokenAccountArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    if args.reason_code == 0 {
        return Err(ChanceryError::ArgsDeserializationFailed.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(cfg_account_info)?;

    if &chancery_config.issued_token_mint != issued_token_mint_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if &chancery_config.issued_token_program != issued_token_program_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if &chancery_config.freeze_authority_pda != freeze_authority_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    // Token account mint validation (offsets 0..32).
    if issued_token_account_info.owner != issued_token_program_account_info.key {
        return Err(ChanceryError::TokenProgramMismatch.into());
    }

    {
        let data = issued_token_account_info.try_borrow_data()?;

        if data.len() < 64 {
            return Err(ChanceryError::AccountDataLengthMismatch.into());
        }

        let mut mint_bytes: [u8; 32] = [0u8; 32];

        mint_bytes.copy_from_slice(&data[0..32]);

        if mint_bytes != chancery_config.issued_token_mint.to_bytes() {
            return Err(ChanceryError::AccountKeyMismatch.into());
        }
    }

    // Authority list: governance | ops | enforcement (NOT emergency).
    let is_actor =
           authority_account_info.key == &chancery_config.governance_authority
        || authority_account_info.key == &chancery_config.operations_authority
        || authority_account_info.key == &chancery_config.enforcement_authority;

    // Role precedence matters during the contained bootstrap handoff, where
    // one key initially occupies every authority slot. A signer that is also a
    // governance/ops/enforcement actor retains that higher thaw capability;
    // only an emergency-only signer is denied. New authority transfers cannot
    // introduce cross-role overlap.
    if !is_actor && chancery_config.is_emergency_authority(authority_account_info.key) {
        return Err(ChanceryError::InsufficientRole.into());
    }

    if !is_actor {
        if accounts.len() <= PERMISSION_RECORD {
            return Err(ChanceryError::InsufficientRole.into());
        }

        let permission_record = assert_can_thaw_token_account(
            authority_account_info,
            &accounts[PERMISSION_RECORD],
            issued_token_account_info.key,
            &crate::id(),
        )?;

        drop(permission_record);
    }

    // Freeze record must be the canonical PDA for this token account,
    // already exist, and be FROZEN.
    let freeze_record_bump = BasicFreezeRecord::verify_pda(
        freeze_rec_account_info,
        issued_token_account_info.key,
        &crate::id(),
    )?;

    let r = BasicFreezeRecord::load_for_verified_pda(
        freeze_rec_account_info,
        issued_token_account_info.key,
        freeze_record_bump,
    )?;

    if r.status != basic_freeze_status::FROZEN {
        return Err(ChanceryError::FreezeRecordNotFrozen.into());
    }

    let record_rent_refund_recipient = r.rent_refund_recipient;

    drop(r);

    // ── 1. Token CPI first. ──────────────────────────────────────────────
    let program_id = crate::id();
    let bump       = freeze_authority_bump(&program_id);

    cpi_thaw_account(
        issued_token_program_account_info,
        issued_token_account_info,
        issued_token_mint_account_info,
        freeze_authority_account_info,
        bump,
    )?;

    // ── 2. Close the freeze record (terminal). ───────────────────────────
    // Thaw is the record's terminal transition: close and refund rent to the
    // payer stored at freeze time (which must appear as a writable account in
    // this transaction). The `BasicTokenThaw` event is the canonical record;
    // a re-freeze recreates the account fresh.
    let clock = Clock::get()?;

    if accounts.len() <= RENT_REFUND_RECIPIENT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let rent_refund_recipient_account_info = &accounts[RENT_REFUND_RECIPIENT];

    close_pda_to_recipient_account(freeze_rec_account_info, rent_refund_recipient_account_info, &record_rent_refund_recipient)?;

    // ── 3. Emit BasicTokenThaw event. ────────────────────────────────────
    let event_authority_bump;
    let sequence_nonce;

    drop(chancery_config);

    {
        let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(cfg_account_info, chancery_config_verified_bump)?;
        event_authority_bump       = chancery_config_mut.event_authority_bump;
        sequence_nonce             = chancery_config_mut.next_sequence_nonce()?;
    }

    emit_basic_token_thaw(
        event_authority_account_info,
        event_authority_bump,
        BasicTokenThaw {
            sequence_nonce,
            chancery:             *cfg_account_info.key,
            slot:                 clock.slot,
            unix_timestamp:       clock.unix_timestamp,
            risk_class:           ConfigChangeRiskClass::Widening.as_u8(),
            basic_freeze_record:  *freeze_rec_account_info.key,
            issued_token_account: *issued_token_account_info.key,
            issued_token_mint:    *issued_token_mint_account_info.key,
            thawed_by:            *authority_account_info.key,
            reason_code:          args.reason_code,
            status_flags:         0,
            thaw_flags:           args.thaw_flags,
        },
    )?;

    Ok(())
}
