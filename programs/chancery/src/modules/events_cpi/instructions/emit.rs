/// emit
///
/// Event sink with signer guard. Validates that account[0] is the
/// `event_authority` PDA AND is a signer. Only the chancery program itself
/// can produce that signature (via `invoke_signed` with the cached bump).
///
/// External transactions cannot forge it: signing for a PDA requires
/// `invoke_signed` from the program that derived it, which means external
/// callers can never satisfy the signer check on this account.
///
/// Side effect: within a finalized, successfully committed transaction, the
/// CPI's presence in `meta.innerInstructions` is the evidence record. Off-chain
/// decoders must first require present metadata with `meta.err` present and
/// exactly null, then validate the event_authority account and decode
/// `data[2..]` as `[ discriminator(8) | borsh_payload ]`. The discriminator
/// occupies `data[2..10]`; the Borsh payload begins at `data[10..]`.
///
/// Accounts:
///   0  event_authority  signer  PDA [b"event-authority"]
///
/// Data: `[ discriminator(8) | borsh_payload(...) ]` (opaque to handler).

use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::{constants::seeds, error::ChanceryError};

const EVENT_AUTHORITY:        usize = 0;
const REQUIRED_ACCOUNT_COUNT: usize = 1;

pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], _data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let event_authority_account_info = &accounts[EVENT_AUTHORITY];

    if !event_authority_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    let (expected_key, _bump) =
        Pubkey::find_program_address(&[seeds::EVENT_AUTHORITY], &crate::id());

    if event_authority_account_info.key != &expected_key {
        return Err(ChanceryError::InvalidPda.into());
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    #[allow(deprecated)]
    fn account_info<'a>(
        key:       &'a Pubkey,
        owner:     &'a Pubkey,
        lamports:  &'a mut u64,
        data:      &'a mut [u8],
        is_signer: bool,
    ) -> AccountInfo<'a> {
        AccountInfo {
            key,
            lamports: Rc::new(RefCell::new(lamports)),
            data: Rc::new(RefCell::new(data)),
            owner,
            _unused: 0,
            is_signer,
            is_writable: false,
            executable: false,
        }
    }

    #[test]
    fn missing_event_authority_is_rejected() {
        assert_eq!(
            handle(&[], &[]),
            Err(ChanceryError::MissingAccount.into()),
        );
    }

    #[test]
    fn canonical_top_level_call_without_pda_signature_is_rejected() {
        let (event_authority, _) =
            Pubkey::find_program_address(&[seeds::EVENT_AUTHORITY], &crate::id());
        let owner = crate::id();
        let mut lamports = 1;
        let mut data = [];
        let account = account_info(
            &event_authority,
            &owner,
            &mut lamports,
            &mut data,
            false,
        );

        assert_eq!(
            handle(&[account], &[]),
            Err(ChanceryError::AccountNotSigner.into()),
        );
    }

    #[test]
    fn signed_noncanonical_event_authority_is_rejected() {
        let key = Pubkey::new_unique();
        let owner = crate::id();
        let mut lamports = 1;
        let mut data = [];
        let account = account_info(&key, &owner, &mut lamports, &mut data, true);

        assert_eq!(
            handle(&[account], &[]),
            Err(ChanceryError::InvalidPda.into()),
        );
    }

    #[test]
    fn canonical_signed_self_cpi_sink_is_accepted() {
        let (event_authority, _) =
            Pubkey::find_program_address(&[seeds::EVENT_AUTHORITY], &crate::id());
        let owner = crate::id();
        let mut lamports = 1;
        let mut data = [];
        let account = account_info(
            &event_authority,
            &owner,
            &mut lamports,
            &mut data,
            true,
        );

        assert!(handle(&[account], &[]).is_ok());
    }
}
