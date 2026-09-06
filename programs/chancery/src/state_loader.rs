use core::cell::{Ref, RefMut};

use bytemuck::Pod;
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;

use crate::error::ChanceryError;

pub const SUPPORTED_STATE_VERSION: u16 = 1;
const DISCRIMINATOR_LENGTH: usize = 8;
const VERSION_OFFSET: usize = DISCRIMINATOR_LENGTH;
const VERSION_END: usize = VERSION_OFFSET + core::mem::size_of::<u16>();

#[inline]
pub(crate) fn assert_stored_bump_matches(
    stored_bump:   u8,
    expected_bump: u8,
) -> Result<(), ProgramError> {
    if stored_bump != expected_bump {
        return Err(ChanceryError::StoredBumpMismatch.into());
    }

    Ok(())
}

fn validate_expected_layout<T: Pod>(expected_size: usize) -> Result<(), ProgramError> {
    if expected_size != core::mem::size_of::<T>() || expected_size < VERSION_END {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }
    Ok(())
}

pub fn load_state<'a, T: Pod>(
    account: &'a AccountInfo<'a>,
    expected_size: usize,
    expected_discriminator: [u8; DISCRIMINATOR_LENGTH],
    not_initialized_error: ChanceryError,
) -> Result<Ref<'a, T>, ProgramError> {
    validate_expected_layout::<T>(expected_size)?;

    if account.owner != &crate::id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    let data = account.try_borrow_data()?;

    validate_initialized_data(
        &data,
        expected_size,
        expected_discriminator,
        not_initialized_error,
    )?;

    Ref::filter_map(data, |bytes| bytemuck::try_from_bytes::<T>(bytes).ok())
        .map_err(|_| ChanceryError::AccountDataUnaligned.into())
}

pub fn load_state_mut<'a, T: Pod>(
    account: &'a AccountInfo<'a>,
    expected_size: usize,
    expected_discriminator: [u8; DISCRIMINATOR_LENGTH],
    not_initialized_error: ChanceryError,
) -> Result<RefMut<'a, T>, ProgramError> {
    validate_expected_layout::<T>(expected_size)?;

    if !account.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if account.owner != &crate::id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    let data = account.try_borrow_mut_data()?;

    validate_initialized_data(
        &data,
        expected_size,
        expected_discriminator,
        not_initialized_error,
    )?;

    RefMut::filter_map(data, |bytes| bytemuck::try_from_bytes_mut::<T>(bytes).ok())
        .map_err(|_| ChanceryError::AccountDataUnaligned.into())
}

pub fn load_uninitialized_state_mut<'a, T: Pod>(
    account: &'a AccountInfo<'a>,
    expected_size: usize,
    already_initialized_error: ChanceryError,
) -> Result<RefMut<'a, T>, ProgramError> {
    validate_expected_layout::<T>(expected_size)?;

    if !account.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if account.owner != &crate::id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    let data = account.try_borrow_mut_data()?;

    if data.len() != expected_size {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    if data[..DISCRIMINATOR_LENGTH] != [0u8; DISCRIMINATOR_LENGTH] {
        return Err(already_initialized_error.into());
    }

    RefMut::filter_map(data, |bytes| bytemuck::try_from_bytes_mut::<T>(bytes).ok())
        .map_err(|_| ChanceryError::AccountDataUnaligned.into())
}

fn validate_initialized_data(
    data: &[u8],
    expected_size: usize,
    expected_discriminator: [u8; DISCRIMINATOR_LENGTH],
    not_initialized_error: ChanceryError,
) -> Result<(), ProgramError> {
    if expected_size < VERSION_END || data.len() != expected_size {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    if data[..DISCRIMINATOR_LENGTH] != expected_discriminator {
        return Err(not_initialized_error.into());
    }

    let version = u16::from_le_bytes(
        data[VERSION_OFFSET..VERSION_END]
            .try_into()
            .map_err(|_| ChanceryError::AccountDataLengthMismatch)?,
    );

    if version != SUPPORTED_STATE_VERSION {
        return Err(ChanceryError::UnsupportedStateVersion.into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use bytemuck::{Pod, Zeroable};
    use solana_pubkey::Pubkey;

    use super::*;

    const TEST_DISCRIMINATOR: [u8; 8] = *b"loadtest";
    const TEST_SIZE: usize = 16;

    #[repr(C)]
    #[derive(Clone, Copy, Pod, Zeroable)]
    struct TestState {
        discriminator: [u8; 8],
        version:       u16,
        bump:          u8,
        reserved:      [u8; 5],
    }

    #[allow(deprecated)]
    fn account_info<'a>(
        key:      &'a Pubkey,
        owner:    &'a Pubkey,
        lamports: &'a mut u64,
        data:     &'a mut [u8],
        writable: bool,
    ) -> AccountInfo<'a> {
        AccountInfo {
            key,
            lamports: Rc::new(RefCell::new(lamports)),
            data: Rc::new(RefCell::new(data)),
            owner,
            _unused: 0,
            is_signer: false,
            is_writable: writable,
            executable: false,
        }
    }

    fn initialized_state() -> TestState {
        TestState {
            discriminator: TEST_DISCRIMINATOR,
            version: SUPPORTED_STATE_VERSION,
            bump: 1,
            reserved: [0u8; 5],
        }
    }

    #[test]
    fn shared_reference_retains_runtime_borrow_guard() {
        let key = Pubkey::new_unique();
        let owner = crate::id();
        let mut lamports = 1u64;
        let mut state = initialized_state();
        let account = account_info(
            &key,
            &owner,
            &mut lamports,
            bytemuck::bytes_of_mut(&mut state),
            true,
        );

        let shared = load_state::<TestState>(
            &account,
            TEST_SIZE,
            TEST_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )
        .expect("load shared state");

        assert_eq!(shared.bump, 1);
        assert!(matches!(
            load_state_mut::<TestState>(
                &account,
                TEST_SIZE,
                TEST_DISCRIMINATOR,
                ChanceryError::NotInitialized,
            ),
            Err(ProgramError::AccountBorrowFailed),
        ));

        drop(shared);
        assert!(load_state_mut::<TestState>(
            &account,
            TEST_SIZE,
            TEST_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )
        .is_ok());
    }

    #[test]
    fn rejects_unsupported_version() {
        let key = Pubkey::new_unique();
        let owner = crate::id();
        let mut lamports = 1u64;
        let mut state = initialized_state();
        state.version = SUPPORTED_STATE_VERSION + 1;
        let account = account_info(
            &key,
            &owner,
            &mut lamports,
            bytemuck::bytes_of_mut(&mut state),
            false,
        );

        assert!(matches!(
            load_state::<TestState>(
                &account,
                TEST_SIZE,
                TEST_DISCRIMINATOR,
                ChanceryError::NotInitialized,
            ),
            Err(ProgramError::Custom(code))
                if code == ChanceryError::UnsupportedStateVersion as u32,
        ));
    }

    #[test]
    fn rejects_wrong_length() {
        let key = Pubkey::new_unique();
        let owner = crate::id();
        let mut lamports = 1u64;
        let mut data = [0u8; TEST_SIZE - 1];
        data[..8].copy_from_slice(&TEST_DISCRIMINATOR);
        data[8..10].copy_from_slice(&SUPPORTED_STATE_VERSION.to_le_bytes());
        let account = account_info(&key, &owner, &mut lamports, &mut data, false);

        assert!(matches!(
            load_state::<TestState>(
                &account,
                TEST_SIZE,
                TEST_DISCRIMINATOR,
                ChanceryError::NotInitialized,
            ),
            Err(ProgramError::Custom(code))
                if code == ChanceryError::AccountDataLengthMismatch as u32,
        ));
    }

    #[test]
    fn rejects_impossibly_small_expected_layout_without_panicking() {
        let key = Pubkey::new_unique();
        let owner = crate::id();
        let mut lamports = 1u64;
        let mut data = [0u8; 1];
        let account = account_info(&key, &owner, &mut lamports, &mut data, true);

        assert!(matches!(
            load_state::<TestState>(
                &account,
                1,
                TEST_DISCRIMINATOR,
                ChanceryError::NotInitialized,
            ),
            Err(ProgramError::Custom(code))
                if code == ChanceryError::AccountDataLengthMismatch as u32,
        ));
        assert!(matches!(
            load_uninitialized_state_mut::<TestState>(
                &account,
                1,
                ChanceryError::AlreadyInitialized,
            ),
            Err(ProgramError::Custom(code))
                if code == ChanceryError::AccountDataLengthMismatch as u32,
        ));
    }

    #[test]
    fn rejects_wrong_discriminator() {
        let key = Pubkey::new_unique();
        let owner = crate::id();
        let mut lamports = 1u64;
        let mut state = initialized_state();
        state.discriminator = *b"wrongdis";
        let account = account_info(
            &key,
            &owner,
            &mut lamports,
            bytemuck::bytes_of_mut(&mut state),
            false,
        );

        assert!(matches!(
            load_state::<TestState>(
                &account,
                TEST_SIZE,
                TEST_DISCRIMINATOR,
                ChanceryError::NotInitialized,
            ),
            Err(ProgramError::Custom(code))
                if code == ChanceryError::NotInitialized as u32,
        ));
    }

    #[test]
    fn rejects_wrong_owner() {
        let key = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let mut lamports = 1u64;
        let mut state = initialized_state();
        let account = account_info(
            &key,
            &owner,
            &mut lamports,
            bytemuck::bytes_of_mut(&mut state),
            false,
        );

        assert!(matches!(
            load_state::<TestState>(
                &account,
                TEST_SIZE,
                TEST_DISCRIMINATOR,
                ChanceryError::NotInitialized,
            ),
            Err(ProgramError::IncorrectProgramId),
        ));
    }
}
