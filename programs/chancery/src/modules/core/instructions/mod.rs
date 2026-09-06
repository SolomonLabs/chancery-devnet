use solana_account_info::AccountInfo;
use solana_cpi::{invoke, invoke_signed};
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_rent::Rent;
use solana_system_interface::{instruction as system_instruction, program as system_program};
use solana_sysvar::Sysvar;

use crate::{
    constants::ix::core as ix_id,
    error::ChanceryError,
};

pub mod accept_authority_transfer;
pub mod initialize_chancery;
pub mod propose_authority_transfer;
pub mod register_asset;
pub mod set_asset_mode;
pub mod set_asset_mode_with_pending_change;
pub mod update_asset_config;
pub mod update_asset_config_with_pending_change;

/// Route ix_id (data[0]) -> handler.
/// data[1..] = borsh args.
pub fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
    if data.is_empty() {
        return Err(ChanceryError::InstructionDataTooShort.into());
    }

    let ix   = data[0];
    let args = &data[1..];

    match ix {
        ix_id::ACCEPT_AUTHORITY_TRANSFER  => accept_authority_transfer::handle(accounts, args),
        ix_id::INITIALIZE_CHANCERY        => initialize_chancery::handle(accounts, args),
        ix_id::PROPOSE_AUTHORITY_TRANSFER => propose_authority_transfer::handle(accounts, args),
        ix_id::REGISTER_ASSET             => register_asset::handle(accounts, args),
        ix_id::SET_ASSET_MODE             => set_asset_mode::handle(accounts, args),
        ix_id::SET_ASSET_MODE_WITH_PENDING_CHANGE => {
            set_asset_mode_with_pending_change::handle(accounts, args)
        }
        ix_id::UPDATE_ASSET_CONFIG        => update_asset_config::handle(accounts, args),
        ix_id::UPDATE_ASSET_CONFIG_WITH_PENDING_CHANGE => {
            update_asset_config_with_pending_change::handle(accounts, args)
        }
        _                                 => Err(ChanceryError::UnknownInstruction.into()),
    }
}

// ─── Shared PDA creation helper ───────────────────────────────────────────────

/// Close a program-owned PDA and refund its rent to the recipient recorded in
/// its state at binding/creation time. Terminal cleanup can never redirect
/// rent: the caller must pass the exact stored recipient as the fixed account
/// for that instruction layout.
///
/// Close semantics: move every lamport to the recipient, then zero the data.
/// The runtime normally reaps the zero-lamport account at transaction end. A
/// later instruction in the same transaction can re-fund a just-closed shell;
/// if that shell survives, fresh-allocation-only initializers must continue to
/// reject it as allocated program-owned state. Chancery intentionally does not
/// retain a permanent tombstone or reusable-address reservation for each
/// terminal record, so exact historical-address availability is not guaranteed
/// (spec 14 §14.3.1).
pub fn close_pda_to_recipient_account<'a>(
    pda_account:       &'a AccountInfo<'a>,
    recipient:         &'a AccountInfo<'a>,
    stored_recipient:  &Pubkey,
) -> ProgramResult {
    if pda_account.owner != &crate::id() {
        return Err(ChanceryError::AccountOwnerMismatch.into());
    }

    if !pda_account.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    // The refund can only be conveyed, never redirected: the named account
    // must be exactly the recipient stored at creation time.
    if recipient.key != stored_recipient {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if !recipient.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let lamports = pda_account.lamports();

    {
        let mut recipient_lamports = recipient.try_borrow_mut_lamports()?;

        **recipient_lamports = recipient_lamports
            .checked_add(lamports)
            .ok_or(ChanceryError::ArithmeticOverflow)?;
    }

    {
        let mut pda_lamports = pda_account.try_borrow_mut_lamports()?;

        **pda_lamports = 0;
    }

    pda_account.try_borrow_mut_data()?.fill(0);

    Ok(())
}

/// Create a PDA via transfer + allocate + assign.
///
/// Avoids `create_account`'s `AccountAlreadyInUse` failure when the PDA has
/// already received lamports from an external transfer.
/// `seeds` must include the bump as the final element.
pub fn create_pda_account<'a>(
    payer:                       &AccountInfo<'a>,
    pda_account:                 &AccountInfo<'a>,
    system_program_account_info: &AccountInfo<'a>,
    program_id:                  &Pubkey,
    seeds:                       &[&[u8]],
    space:                       usize,
) -> ProgramResult {
    if !pda_account.data_is_empty() {
        return Err(crate::error::ChanceryError::AlreadyInitialized.into());
    }
    if pda_account.owner != &system_program::ID {
        return Err(crate::error::ChanceryError::AccountOwnerMismatch.into());
    }

    let rent     = Rent::get()?;
    let required = rent.minimum_balance(space);
    let current  = pda_account.lamports();

    if current < required {
        let topup = required.saturating_sub(current);
        invoke(
            &system_instruction::transfer(payer.key, pda_account.key, topup),
            &[
                payer.clone(),
                pda_account.clone(),
                system_program_account_info.clone(),
            ],
        )?;
    }

    invoke_signed(
        &system_instruction::allocate(pda_account.key, space as u64),
        &[pda_account.clone(), system_program_account_info.clone()],
        &[seeds],
    )?;

    invoke_signed(
        &system_instruction::assign(pda_account.key, program_id),
        &[pda_account.clone(), system_program_account_info.clone()],
        &[seeds],
    )?;

    Ok(())
}

