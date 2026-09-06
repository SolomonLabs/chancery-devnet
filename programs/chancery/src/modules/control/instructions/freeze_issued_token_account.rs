//! freeze_issued_token_account.
//!
//! Emergency containment surface. Per
//!   - Hard-fail `reason_code == 0`
//!   - Authority: governance | ops | emergency | enforcement
//!     (any holder of CAN_FREEZE_TOKEN_ACCOUNT at narrow scope ALSO accepted)
//!   - status_flags=0 in events for MVP
//!   - Token CPI first → record write → emit (evidence-last)
//!
//! Record allocation does NOT use `system_instruction::create_account`.
//! `create_account` fails when the destination already holds lamports, and the
//! record PDA is derived from a publicly known token account, so any unrelated
//! party can transfer the 0-byte rent floor to that address and permanently
//! brick freeze for that holder. `transfer` + `allocate` + `assign` succeeds
//! from any starting balance; an attacker cannot `allocate` or `assign` the PDA
//! themselves because both require the account to sign and only this program
//! can `invoke_signed` those seeds. Donated lamports are swept back to the
//! payer so the record settles at exactly rent-exemption and the refund
//! recorded in `rent_refund_recipient` stays well-defined.
//!
//! Wire format:
//!   [ CONTROL(0x09) | FREEZE_ISSUED_TOKEN_ACCOUNT(0x05) | borsh(args) ]
//!
//! Accounts:
//!   0  chancery_config            writable PDA  (sequence_nonce for evidence)
//!   1  event_authority            readable PDA
//!   2  module_activation_state    readable PDA
//!   3  basic_freeze_record        writable PDA  (created on first freeze)
//!   4  issued_token_account       writable
//!   5  issued_token_mint          readable (writable per Token-2022 may need)
//!   6  freeze_authority_pda       readable PDA
//!   7  issued_token_program       readable
//!   8  authority                  signer
//!   9  payer                      signer  (rent for freeze record on first call)
//!  10  system_program
//!  11  permission_record          optional  (when authority is non-actor signer)

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_cpi::{invoke, invoke_signed};
use solana_program_entrypoint::ProgramResult;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;
use solana_rent::Rent;
use solana_system_interface::{instruction as system_instruction, program as system_program};
use solana_sysvar::Sysvar;

use crate::{
    constants::{module, seeds},
    error::ChanceryError,
    modules::{
        core::state::chancery_config::ChanceryConfig,
        control::{
            change_risk::ConfigChangeRiskClass,
            instructions::cpi::{cpi_freeze_account, freeze_authority_bump},
            state::{
                basic_freeze_record::{
                    BasicFreezeRecord, BASIC_FREEZE_RECORD_DISCRIMINATOR,
                    BASIC_FREEZE_RECORD_SIZE,
                },
                module_activation_state::ModuleActivationState,
            },
        },
        evidence::emit::{emit_basic_token_freeze, BasicTokenFreeze},
        permissions::auth::assert_can_freeze_token_account,
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
const PAYER:                  usize = 9;
const SYSTEM_PROGRAM:         usize = 10;
const PERMISSION_RECORD:      usize = 11;
const REQUIRED_ACCOUNT_COUNT: usize = 11;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FreezeRecordDisposition {
    /// Address holds no data. May still hold lamports donated by anyone.
    Allocate,
    /// Program-owned, correctly sized, fully zeroed - a closed record's shell.
    ReinitializeZeroedShell,
    /// Live record; re-freeze in place.
    Update,
}

/// Lamport movement required to bring the record to exactly rent-exemption.
/// At most one leg is non-zero.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FreezeRecordFundingPlan {
    top_up_lamports: u64,
    sweep_lamports:  u64,
}

fn plan_freeze_record_funding(
    current_lamports:  u64,
    required_lamports: u64,
) -> FreezeRecordFundingPlan {
    FreezeRecordFundingPlan {
        top_up_lamports: required_lamports.saturating_sub(current_lamports),
        sweep_lamports:  current_lamports.saturating_sub(required_lamports),
    }
}

fn classify_freeze_record_account(
    account:    &AccountInfo,
    program_id: &Pubkey,
) -> Result<FreezeRecordDisposition, ProgramError> {
    if account.data_is_empty() {
        // Ownership, not balance, is what makes an empty address allocatable.
        // A donated balance on a system-owned address is inert.
        if account.owner != &system_program::ID {
            return Err(ChanceryError::AccountOwnerMismatch.into());
        }

        return Ok(FreezeRecordDisposition::Allocate);
    }

    if account.data_len() != BASIC_FREEZE_RECORD_SIZE {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    if account.owner != program_id {
        return Err(ChanceryError::AccountOwnerMismatch.into());
    }

    let data = account.try_borrow_data()?;
    if data.iter().all(|byte| *byte == 0) {
        Ok(FreezeRecordDisposition::ReinitializeZeroedShell)
    } else {
        Ok(FreezeRecordDisposition::Update)
    }
}

/// SPL Token / Token-2022 base account `state` byte (offset 108):
/// 0 = Uninitialized, 1 = Initialized, 2 = Frozen. Extension TLV data, if any,
/// follows the 165-byte base and does not move this field.
const TOKEN_ACCOUNT_STATE_OFFSET: usize = 108;
const TOKEN_ACCOUNT_STATE_FROZEN: u8    = 2;

/// True iff the SPL Token / Token-2022 account is already Frozen. Binds the
/// account owner to the token program before the raw base-layout read (never
/// raw-read a token account before binding its owner). issue-16.
fn issued_token_account_is_frozen(
    token_account_info:         &AccountInfo,
    token_program_account_info: &AccountInfo,
) -> Result<bool, ProgramError> {
    if token_account_info.owner != token_program_account_info.key {
        return Err(ChanceryError::TokenProgramMismatch.into());
    }

    let data = token_account_info.try_borrow_data()?;
    if data.len() <= TOKEN_ACCOUNT_STATE_OFFSET {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    Ok(data[TOKEN_ACCOUNT_STATE_OFFSET] == TOKEN_ACCOUNT_STATE_FROZEN)
}

/// Leave `record` program-owned, `BASIC_FREEZE_RECORD_SIZE` bytes, zeroed, and
/// holding exactly rent-exemption - from any starting balance and from either
/// an unallocated address or an existing zeroed shell.
fn ensure_freeze_record_allocated<'a>(
    payer:                       &AccountInfo<'a>,
    record:                      &AccountInfo<'a>,
    system_program_account_info: &AccountInfo<'a>,
    program_id:                  &Pubkey,
    signer_seeds:                &[&[u8]],
) -> ProgramResult {
    if system_program_account_info.key != &system_program::ID {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    let required_lamports = Rent::get()?.minimum_balance(BASIC_FREEZE_RECORD_SIZE);
    let plan              = plan_freeze_record_funding(record.lamports(), required_lamports);

    // Top up first: `transfer` needs a system-owned source, not a system-owned
    // destination, so this is legal before or after assignment. Doing it first
    // keeps the account rent-exempt at every point it is program-owned.
    if plan.top_up_lamports > 0 {
        invoke(
            &system_instruction::transfer(payer.key, record.key, plan.top_up_lamports),
            &[
                payer.clone(),
                record.clone(),
                system_program_account_info.clone(),
            ],
        )?;
    }

    if record.data_is_empty() {
        // `allocate` requires zero data length and system ownership; lamports
        // are irrelevant to it. That is precisely what survives a donation.
        invoke_signed(
            &system_instruction::allocate(record.key, BASIC_FREEZE_RECORD_SIZE as u64),
            &[record.clone(), system_program_account_info.clone()],
            &[signer_seeds],
        )?;

        invoke_signed(
            &system_instruction::assign(record.key, program_id),
            &[record.clone(), system_program_account_info.clone()],
            &[signer_seeds],
        )?;
    }

    // Post-conditions, re-derived rather than assumed: the sweep below debits
    // the record, which is only legal while this program owns it.
    if record.owner != program_id {
        return Err(ChanceryError::AccountOwnerMismatch.into());
    }

    if record.data_len() != BASIC_FREEZE_RECORD_SIZE {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    if plan.sweep_lamports > 0 {
        **record.try_borrow_mut_lamports()? -= plan.sweep_lamports;
        **payer.try_borrow_mut_lamports()?  += plan.sweep_lamports;
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn initialize_freeze_record(
    record:                 &mut BasicFreezeRecord,
    record_bump:            u8,
    args:                   &FreezeIssuedTokenAccountArgs,
    issued_token_account:   &Pubkey,
    issued_token_mint:      &Pubkey,
    freeze_authority_pda:   &Pubkey,
    authority:              &Pubkey,
    payer:                  &Pubkey,
    current_slot:           u64,
) {
    record.discriminator             = BASIC_FREEZE_RECORD_DISCRIMINATOR;
    record.version                   = 1;
    record.bump                      = record_bump;
    record.status                    = crate::constants::basic_freeze_status::FROZEN;
    record.reason_code               = args.reason_code;
    record.issued_token_account      = *issued_token_account;
    record.issued_token_mint         = *issued_token_mint;
    record.freeze_authority_pda      = *freeze_authority_pda;
    record.frozen_by                 = *authority;
    record.thawed_by                 = Pubkey::default();
    record.frozen_at_slot            = current_slot;
    record.thawed_at_slot            = 0;
    record.last_event_sequence_nonce = 0;
    record.freeze_flags              = args.freeze_flags;
    record.thaw_flags                = 0;
    // Safe to record unconditionally: `ensure_freeze_record_allocated` swept
    // any balance the payer did not supply, so the rent held here is the
    // payer's own.
    record.rent_refund_recipient     = *payer;
    record._reserved                 = [0u8; 24];
}

#[derive(BorshDeserialize)]
pub struct FreezeIssuedTokenAccountArgs {
    pub reason_code : u32,
    pub freeze_flags: u64,
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
    let payer_account_info                = &accounts[PAYER];
    let system_account_info               = &accounts[SYSTEM_PROGRAM];

    if !authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !payer_account_info.is_signer {
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

    // Module activation gate (CONTROL must be ACTIVE).
    let activation = ModuleActivationState::load_verified(activation_account_info)?;

    activation.assert_active(module::CONTROL)?;

    let args = FreezeIssuedTokenAccountArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    // Hard-fail reason_code == 0 per
    if args.reason_code == 0 {
        return Err(ChanceryError::ArgsDeserializationFailed.into());
    }

    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(cfg_account_info)?;
    // Note: we deliberately do NOT call assert_not_paused - freeze is the
    // emergency containment surface and must work during global pause.

    // Validate mint and program bindings.
    if &chancery_config.issued_token_mint != issued_token_mint_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if &chancery_config.issued_token_program != issued_token_program_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if &chancery_config.freeze_authority_pda != freeze_authority_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    // Validate token account: mint must be issued_token_mint.
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

    // Authority: actor authority OR holder of CAN_FREEZE_TOKEN_ACCOUNT at
    // narrow scope.
    let is_actor =
           authority_account_info.key == &chancery_config.governance_authority
        || authority_account_info.key == &chancery_config.operations_authority
        || authority_account_info.key == &chancery_config.emergency_authority
        || authority_account_info.key == &chancery_config.enforcement_authority;

    if !is_actor {
        if accounts.len() <= PERMISSION_RECORD {
            return Err(ChanceryError::InsufficientRole.into());
        }

        let permission_record = assert_can_freeze_token_account(
            authority_account_info,
            &accounts[PERMISSION_RECORD],
            issued_token_account_info.key,
            &crate::id(),
        )?;

        drop(permission_record);
    }

    // Validate and classify the canonical record before the Token CPI. A thawed
    // record can survive transaction finalization as a re-funded, program-owned,
    // zeroed shell; that shell is a valid fresh-cycle target, not initialized state.
    let program_id = crate::id();
    let (expected_record_pda, rec_bump) =
        BasicFreezeRecord::pda(issued_token_account_info.key, &program_id);
    if freeze_rec_account_info.key != &expected_record_pda {
        return Err(ChanceryError::InvalidPda.into());
    }

    let record_disposition = classify_freeze_record_account(
        freeze_rec_account_info,
        &program_id,
    )?;

    // ── 1. Token CPI first (per evidence-last invariant). ────────────────
    // Freeze the account only if it is not already frozen. Token-2022 rejects a
    // redundant FreezeAccount, which would revert an in-place metadata Update (a
    // live BasicFreezeRecord) before the record could be rewritten. Skipping the
    // CPI when the account is already frozen makes the freeze idempotent and lets
    // an authorized actor update the freeze reason / flags on a standing freeze.
    // (issue-16)
    let bump = freeze_authority_bump(&program_id);

    if !issued_token_account_is_frozen(
        issued_token_account_info,
        issued_token_program_account_info,
    )? {
        cpi_freeze_account(
            issued_token_program_account_info,
            issued_token_account_info,
            issued_token_mint_account_info,
            freeze_authority_account_info,
            bump,
        )?;
    }

    // ── 2. Allocate / rehydrate / load freeze record + write status. ─────
    let clock = Clock::get()?;

    match record_disposition {
        // Both fresh-cycle cases converge: `ensure_freeze_record_allocated` is
        // idempotent over allocation and normalises the balance either way.
        FreezeRecordDisposition::Allocate | FreezeRecordDisposition::ReinitializeZeroedShell => {
            ensure_freeze_record_allocated(
                payer_account_info,
                freeze_rec_account_info,
                system_account_info,
                &program_id,
                &[
                    seeds::BASIC_FREEZE_RECORD,
                    issued_token_account_info.key.as_ref(),
                    &[rec_bump],
                ],
            )?;

            let mut record = BasicFreezeRecord::load_uninitialized_mut(freeze_rec_account_info)?;

            initialize_freeze_record(
                &mut record,
                rec_bump,
                &args,
                issued_token_account_info.key,
                issued_token_mint_account_info.key,
                freeze_authority_account_info.key,
                authority_account_info.key,
                payer_account_info.key,
                clock.slot,
            );
        }
        FreezeRecordDisposition::Update => {
            let mut record = BasicFreezeRecord::load_mut_for_verified_pda(
                freeze_rec_account_info,
                issued_token_account_info.key,
                rec_bump,
            )?;

            record.status         = crate::constants::basic_freeze_status::FROZEN;
            record.reason_code    = args.reason_code;
            record.frozen_by      = *authority_account_info.key;
            record.frozen_at_slot = clock.slot;
            record.freeze_flags   = args.freeze_flags;
        }
    }

    // ── 3. Emit BasicTokenFreeze event (evidence-last). ──────────────────
    drop(chancery_config);

    let (event_authority_bump, sequence_nonce) = {
        let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(cfg_account_info, chancery_config_verified_bump)?;
        let event_authority_bump    = chancery_config_mut.event_authority_bump;
        let sequence_nonce          = chancery_config_mut.next_sequence_nonce()?;
        (event_authority_bump, sequence_nonce)
    };

    {
        let mut r = BasicFreezeRecord::load_mut_for_verified_pda(
            freeze_rec_account_info,
            issued_token_account_info.key,
            rec_bump,
        )?;
        r.last_event_sequence_nonce = sequence_nonce;
    }

    emit_basic_token_freeze(
        event_authority_account_info,
        event_authority_bump,
        BasicTokenFreeze {
            sequence_nonce,
            chancery            : *cfg_account_info.key,
            slot                : clock.slot,
            unix_timestamp      : clock.unix_timestamp,
            risk_class          : ConfigChangeRiskClass::RestrictiveImmediate.as_u8(),
            basic_freeze_record : *freeze_rec_account_info.key,
            issued_token_account: *issued_token_account_info.key,
            issued_token_mint   : *issued_token_mint_account_info.key,
            frozen_by           : *authority_account_info.key,
            reason_code         : args.reason_code,
            status_flags        : 0,
            freeze_flags        : args.freeze_flags,
        },
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    /// Rent-exemption for the record under default cluster parameters:
    /// (128 + 272) * 3480 * 2.
    const RECORD_RENT_LAMPORTS: u64 = 2_784_000;

    /// Rent-exemption for a 0-byte account: 128 * 3480 * 2. The cheapest
    /// donation the runtime will let an attacker leave at an empty address.
    const EMPTY_ADDRESS_RENT_LAMPORTS: u64 = 890_880;

    #[allow(deprecated)]
    fn account_info<'a>(
        key:      &'a Pubkey,
        owner:    &'a Pubkey,
        lamports: &'a mut u64,
        data:     &'a mut [u8],
    ) -> AccountInfo<'a> {
        AccountInfo {
            key,
            lamports: Rc::new(RefCell::new(lamports)),
            data: Rc::new(RefCell::new(data)),
            owner,
            _unused: 0,
            is_signer: false,
            is_writable: true,
            executable: false,
        }
    }

    #[test]
    fn unallocated_address_is_allocatable() {
        let key = Pubkey::new_unique();
        let owner = system_program::ID;
        let mut lamports = 0;
        let mut data: [u8; 0] = [];
        let account = account_info(&key, &owner, &mut lamports, &mut data);

        assert_eq!(
            classify_freeze_record_account(&account, &crate::id()),
            Ok(FreezeRecordDisposition::Allocate),
        );
    }

    #[test]
    fn donated_lamports_do_not_change_the_disposition() {
        let key = Pubkey::new_unique();
        let owner = system_program::ID;
        let mut data: [u8; 0] = [];

        // The griefing balance, and a balance far above the record's own rent.
        for donation in [EMPTY_ADDRESS_RENT_LAMPORTS, RECORD_RENT_LAMPORTS * 10] {
            let mut lamports = donation;
            let account = account_info(&key, &owner, &mut lamports, &mut data);

            assert_eq!(
                classify_freeze_record_account(&account, &crate::id()),
                Ok(FreezeRecordDisposition::Allocate),
                "donation of {donation} must not brick allocation",
            );
        }
    }

    #[test]
    fn zeroed_program_owned_record_is_reinitializable() {
        let key = Pubkey::new_unique();
        let owner = crate::id();
        let mut lamports = RECORD_RENT_LAMPORTS;
        let mut data = [0u8; BASIC_FREEZE_RECORD_SIZE];
        let account = account_info(&key, &owner, &mut lamports, &mut data);

        assert_eq!(
            classify_freeze_record_account(&account, &owner),
            Ok(FreezeRecordDisposition::ReinitializeZeroedShell),
        );
    }

    #[test]
    fn nonzero_program_owned_record_uses_initialized_path() {
        let key = Pubkey::new_unique();
        let owner = crate::id();
        let mut lamports = RECORD_RENT_LAMPORTS;
        let mut data = [0u8; BASIC_FREEZE_RECORD_SIZE];
        data[0] = 1;
        let account = account_info(&key, &owner, &mut lamports, &mut data);

        assert_eq!(
            classify_freeze_record_account(&account, &owner),
            Ok(FreezeRecordDisposition::Update),
        );
    }

    #[test]
    fn allocated_record_with_wrong_owner_fails_closed() {
        let key = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let program_id = crate::id();
        let mut lamports = RECORD_RENT_LAMPORTS;
        let mut data = [0u8; BASIC_FREEZE_RECORD_SIZE];
        let account = account_info(&key, &owner, &mut lamports, &mut data);

        assert_eq!(
            classify_freeze_record_account(&account, &program_id),
            Err(ChanceryError::AccountOwnerMismatch.into()),
        );
    }

    #[test]
    fn empty_address_owned_by_a_third_party_fails_closed() {
        let key = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let mut lamports = EMPTY_ADDRESS_RENT_LAMPORTS;
        let mut data: [u8; 0] = [];
        let account = account_info(&key, &owner, &mut lamports, &mut data);

        assert_eq!(
            classify_freeze_record_account(&account, &crate::id()),
            Err(ChanceryError::AccountOwnerMismatch.into()),
        );
    }

    #[test]
    fn wrong_sized_allocation_fails_closed() {
        let key = Pubkey::new_unique();
        let owner = crate::id();
        let mut lamports = RECORD_RENT_LAMPORTS;
        let mut data = [0u8; BASIC_FREEZE_RECORD_SIZE - 1];
        let account = account_info(&key, &owner, &mut lamports, &mut data);

        assert_eq!(
            classify_freeze_record_account(&account, &owner),
            Err(ChanceryError::AccountDataLengthMismatch.into()),
        );
    }

    #[test]
    fn funding_plan_tops_up_from_empty() {
        assert_eq!(
            plan_freeze_record_funding(0, RECORD_RENT_LAMPORTS),
            FreezeRecordFundingPlan {
                top_up_lamports: RECORD_RENT_LAMPORTS,
                sweep_lamports:  0,
            },
        );
    }

    #[test]
    fn funding_plan_tops_up_the_remainder_after_a_donation() {
        assert_eq!(
            plan_freeze_record_funding(EMPTY_ADDRESS_RENT_LAMPORTS, RECORD_RENT_LAMPORTS),
            FreezeRecordFundingPlan {
                top_up_lamports: RECORD_RENT_LAMPORTS - EMPTY_ADDRESS_RENT_LAMPORTS,
                sweep_lamports:  0,
            },
        );
    }

    #[test]
    fn funding_plan_sweeps_an_overshooting_donation() {
        assert_eq!(
            plan_freeze_record_funding(RECORD_RENT_LAMPORTS + 7, RECORD_RENT_LAMPORTS),
            FreezeRecordFundingPlan {
                top_up_lamports: 0,
                sweep_lamports:  7,
            },
        );
    }

    #[test]
    fn funding_plan_is_a_noop_at_exact_rent() {
        assert_eq!(
            plan_freeze_record_funding(RECORD_RENT_LAMPORTS, RECORD_RENT_LAMPORTS),
            FreezeRecordFundingPlan {
                top_up_lamports: 0,
                sweep_lamports:  0,
            },
        );
    }

    #[test]
    fn funding_plan_legs_are_mutually_exclusive_and_settle_at_rent() {
        let balances = [
            0,
            1,
            EMPTY_ADDRESS_RENT_LAMPORTS,
            RECORD_RENT_LAMPORTS - 1,
            RECORD_RENT_LAMPORTS,
            RECORD_RENT_LAMPORTS + 1,
            u64::MAX,
        ];

        for current in balances {
            let plan = plan_freeze_record_funding(current, RECORD_RENT_LAMPORTS);

            assert!(
                plan.top_up_lamports == 0 || plan.sweep_lamports == 0,
                "both legs non-zero at {current}",
            );
            assert_eq!(
                current + plan.top_up_lamports - plan.sweep_lamports,
                RECORD_RENT_LAMPORTS,
                "balance does not settle at rent-exemption from {current}",
            );
        }
    }

    // issue-16: detecting an already-frozen account lets the handler skip the
    // redundant FreezeAccount CPI (which Token-2022 rejects) so an in-place
    // freeze-metadata Update can proceed instead of reverting.
    #[test]
    fn is_frozen_detects_state_byte_after_binding_owner() {
        let token_program      = Pubkey::new_unique();
        let mut tp_lamports    = 1u64;
        let mut tp_data: [u8; 0] = [];
        let token_program_info = account_info(
            &token_program, &system_program::ID, &mut tp_lamports, &mut tp_data,
        );

        let acct_key = Pubkey::new_unique();

        // Frozen (state byte == 2).
        {
            let mut lamports = 1u64;
            let mut data     = vec![0u8; 165];
            data[TOKEN_ACCOUNT_STATE_OFFSET] = TOKEN_ACCOUNT_STATE_FROZEN;
            let acct = account_info(&acct_key, &token_program, &mut lamports, &mut data);
            assert_eq!(
                issued_token_account_is_frozen(&acct, &token_program_info),
                Ok(true),
            );
        }

        // Initialized but not frozen (state byte == 1).
        {
            let mut lamports = 1u64;
            let mut data     = vec![0u8; 165];
            data[TOKEN_ACCOUNT_STATE_OFFSET] = 1;
            let acct = account_info(&acct_key, &token_program, &mut lamports, &mut data);
            assert_eq!(
                issued_token_account_is_frozen(&acct, &token_program_info),
                Ok(false),
            );
        }

        // Owner not bound to the token program → rejected before any state read.
        {
            let wrong        = Pubkey::new_unique();
            let mut lamports = 1u64;
            let mut data     = vec![0u8; 165];
            let acct = account_info(&acct_key, &wrong, &mut lamports, &mut data);
            assert_eq!(
                issued_token_account_is_frozen(&acct, &token_program_info),
                Err(ChanceryError::TokenProgramMismatch.into()),
            );
        }
    }
}
