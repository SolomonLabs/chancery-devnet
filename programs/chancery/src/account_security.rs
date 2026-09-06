use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{constants::seeds, error::ChanceryError};

/// Every PDA that the current binary can sign for with `invoke_signed`.
///
/// These identities are protocol capabilities, not users or operators. They
/// must never be persisted as permission subjects, governance authorities,
/// settlement executors, or principals: a later CPI addition must not turn a
/// previously inert permission record into an executable authority.
const PROTOCOL_SIGNER_SEEDS: [&[u8]; 12] = [
    seeds::MINT_AUTHORITY,
    seeds::FREEZE_AUTHORITY,
    seeds::RESERVE_AUTHORITY,
    seeds::CLOSE_MINT_AUTHORITY,
    seeds::TRANSFER_HOOK_AUTHORITY,
    seeds::PERMANENT_DELEGATE_AUTHORITY,
    seeds::METADATA_POINTER_AUTHORITY,
    seeds::METADATA_UPDATE_AUTHORITY,
    seeds::PAUSE_AUTHORITY,
    seeds::CONFIDENTIAL_TRANSFER_AUTHORITY,
    seeds::DEFAULT_ACCOUNT_STATE_AUTHORITY,
    seeds::EVENT_AUTHORITY,
];

const COMPACT_DISTINCT_ACCOUNT_CAPACITY: usize = 32;

pub fn is_protocol_signer_identity(identity: &Pubkey, program_id: &Pubkey) -> bool {
    // Program-derived addresses are always off-curve. Normal wallet and
    // keypair identities therefore need no protocol-PDA derivations at all.
    if identity.is_on_curve() {
        return false;
    }

    let mut index = 0usize;
    while index < PROTOCOL_SIGNER_SEEDS.len() {
        let (candidate, _) = Pubkey::find_program_address(
            &[PROTOCOL_SIGNER_SEEDS[index]],
            program_id,
        );
        if identity == &candidate {
            return true;
        }
        index += 1;
    }
    false
}

pub fn assert_external_identity(
    identity: &Pubkey,
    program_id: &Pubkey,
) -> Result<(), ProgramError> {
    if identity == &Pubkey::default() || is_protocol_signer_identity(identity, program_id) {
        return Err(ChanceryError::ProtocolSignerIdentityForbidden.into());
    }
    Ok(())
}

pub fn assert_optional_external_identity(
    identity: &Pubkey,
    program_id: &Pubkey,
) -> Result<(), ProgramError> {
    if identity == &Pubkey::default() {
        return Ok(());
    }
    assert_external_identity(identity, program_id)
}

pub fn assert_external_signer(
    account: &AccountInfo,
    program_id: &Pubkey,
) -> Result<(), ProgramError> {
    if !account.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }
    assert_external_identity(account.key, program_id)
}

#[inline]
pub fn assert_distinct_non_default_keys(keys: &[Pubkey]) -> Result<(), ProgramError> {
    if keys.len() > COMPACT_DISTINCT_ACCOUNT_CAPACITY {
        return assert_distinct_non_default_keys_quadratic(keys);
    }

    let default_key = Pubkey::default();
    let mut first_index = 0usize;
    while first_index < keys.len() {
        if keys[first_index] == default_key {
            return Err(ChanceryError::AccountKeyMismatch.into());
        }
        let mut second_index = first_index + 1;
        while second_index < keys.len() {
            if keys[first_index] == keys[second_index] {
                return Err(ChanceryError::AccountAliasNotAllowed.into());
            }
            second_index += 1;
        }
        first_index += 1;
    }
    Ok(())
}

#[cold]
fn assert_distinct_non_default_keys_quadratic(
    keys: &[Pubkey],
) -> Result<(), ProgramError> {
    let default_key = Pubkey::default();
    let mut first_index = 0usize;
    while first_index < keys.len() {
        if keys[first_index] == default_key {
            return Err(ChanceryError::AccountKeyMismatch.into());
        }
        let mut second_index = first_index + 1;
        while second_index < keys.len() {
            if keys[first_index] == keys[second_index] {
                return Err(ChanceryError::AccountAliasNotAllowed.into());
            }
            second_index += 1;
        }
        first_index += 1;
    }
    Ok(())
}

/// Reject aliases among accounts that are semantically independent. A
/// default-key account is an optional placeholder and is ignored; duplicate
/// placeholders therefore remain valid.
#[inline]
pub fn assert_distinct_present_accounts(
    accounts: &[&AccountInfo],
) -> Result<(), ProgramError> {
    if accounts.len() > COMPACT_DISTINCT_ACCOUNT_CAPACITY {
        return assert_distinct_present_accounts_quadratic(accounts);
    }

    let default_key = Pubkey::default();
    let mut present_positions = [0u8; COMPACT_DISTINCT_ACCOUNT_CAPACITY];
    let mut present_count = 0usize;
    let mut position = 0usize;
    while position < accounts.len() {
        if accounts[position].key != &default_key {
            present_positions[present_count] = position as u8;
            present_count += 1;
        }
        position += 1;
    }

    let mut first_index = 0usize;
    while first_index < present_count {
        let first_position = present_positions[first_index] as usize;
        let mut second_index = first_index + 1;
        while second_index < present_count {
            let second_position = present_positions[second_index] as usize;
            if accounts[first_position].key == accounts[second_position].key {
                return Err(ChanceryError::AccountAliasNotAllowed.into());
            }
            second_index += 1;
        }
        first_index += 1;
    }
    Ok(())
}

#[cold]
fn assert_distinct_present_accounts_quadratic(
    accounts: &[&AccountInfo],
) -> Result<(), ProgramError> {
    let default_key = Pubkey::default();
    let mut first_index = 0usize;
    while first_index < accounts.len() {
        if accounts[first_index].key != &default_key {
            let mut second_index = first_index + 1;
            while second_index < accounts.len() {
                if accounts[second_index].key != &default_key
                    && accounts[first_index].key == accounts[second_index].key
                {
                    return Err(ChanceryError::AccountAliasNotAllowed.into());
                }
                second_index += 1;
            }
        }
        first_index += 1;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn account_info_for_key<'a>(
        key: &'a Pubkey,
        lamports: &'a mut u64,
        data: &'a mut [u8],
        owner: &'a Pubkey,
    ) -> AccountInfo<'a> {
        AccountInfo::new(key, false, false, lamports, data, owner, false)
    }

    fn leaked_account_info_for_key(key: Pubkey) -> AccountInfo<'static> {
        let key = Box::leak(Box::new(key));
        let lamports = Box::leak(Box::new(0u64));
        let data = Box::leak(Vec::<u8>::new().into_boxed_slice());
        let owner = Box::leak(Box::new(Pubkey::new_unique()));

        account_info_for_key(key, lamports, data, owner)
    }

    fn leaked_account_infos_for_keys(keys: &[Pubkey]) -> Vec<AccountInfo<'static>> {
        let mut accounts = Vec::with_capacity(keys.len());
        let mut index = 0usize;
        while index < keys.len() {
            accounts.push(leaked_account_info_for_key(keys[index]));
            index += 1;
        }
        accounts
    }

    #[test]
    fn normal_on_curve_identity_skips_protocol_signer_matching() {
        let program_id = crate::id();
        let mut identity = Pubkey::new_unique();
        while !identity.is_on_curve() {
            identity = Pubkey::new_unique();
        }

        assert!(!is_protocol_signer_identity(&identity, &program_id));
        assert!(assert_external_identity(&identity, &program_id).is_ok());
    }

    #[test]
    fn every_protocol_signer_pda_is_rejected_as_external_identity() {
        let program_id = crate::id();
        let mut index = 0usize;
        while index < PROTOCOL_SIGNER_SEEDS.len() {
            let (identity, _) = Pubkey::find_program_address(
                &[PROTOCOL_SIGNER_SEEDS[index]],
                &program_id,
            );
            assert_eq!(
                assert_external_identity(&identity, &program_id),
                Err(ChanceryError::ProtocolSignerIdentityForbidden.into()),
            );
            index += 1;
        }
    }

    #[test]
    fn every_readable_protocol_pda_remains_non_authoritative() {
        let program_id = crate::id();
        let owner = Pubkey::new_unique();
        let mut index = 0usize;
        while index < PROTOCOL_SIGNER_SEEDS.len() {
            let (identity, _) = Pubkey::find_program_address(
                &[PROTOCOL_SIGNER_SEEDS[index]],
                &program_id,
            );
            let mut lamports = 1u64;
            let mut data = [];
            let account = account_info_for_key(&identity, &mut lamports, &mut data, &owner);
            assert_eq!(
                assert_external_signer(&account, &program_id),
                Err(ChanceryError::AccountNotSigner.into()),
            );
            index += 1;
        }
    }

    #[test]
    fn signer_bit_cannot_turn_any_protocol_pda_into_external_authority() {
        let program_id = crate::id();
        let owner = Pubkey::new_unique();
        let mut index = 0usize;
        while index < PROTOCOL_SIGNER_SEEDS.len() {
            let (identity, _) = Pubkey::find_program_address(
                &[PROTOCOL_SIGNER_SEEDS[index]],
                &program_id,
            );
            let mut lamports = 1u64;
            let mut data = [];
            let mut account = account_info_for_key(&identity, &mut lamports, &mut data, &owner);
            account.is_signer = true;
            assert_eq!(
                assert_external_signer(&account, &program_id),
                Err(ChanceryError::ProtocolSignerIdentityForbidden.into()),
            );
            index += 1;
        }
    }

    #[test]
    fn full_duplicate_key_matrix_is_rejected() {
        let keys = [
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
        ];
        assert!(assert_distinct_non_default_keys(&keys).is_ok());
        let mut first_index = 0usize;
        while first_index < keys.len() {
            let mut second_index = first_index + 1;
            while second_index < keys.len() {
                let mut aliased = keys;
                aliased[second_index] = aliased[first_index];
                assert_eq!(
                    assert_distinct_non_default_keys(&aliased),
                    Err(ChanceryError::AccountAliasNotAllowed.into()),
                    "pair {first_index}/{second_index}",
                );
                second_index += 1;
            }
            first_index += 1;
        }
    }

    #[test]
    fn compact_distinct_key_boundary_rejects_default_and_duplicate_keys() {
        let mut keys = [Pubkey::default(); COMPACT_DISTINCT_ACCOUNT_CAPACITY];
        let mut index = 0usize;
        while index < keys.len() {
            keys[index] = Pubkey::new_unique();
            index += 1;
        }

        assert!(assert_distinct_non_default_keys(&keys).is_ok());

        let last_index = keys.len() - 1;
        let last_key = keys[last_index];
        keys[last_index] = keys[0];
        assert_eq!(
            assert_distinct_non_default_keys(&keys),
            Err(ChanceryError::AccountAliasNotAllowed.into()),
        );

        keys[last_index] = Pubkey::default();
        assert_eq!(
            assert_distinct_non_default_keys(&keys),
            Err(ChanceryError::AccountKeyMismatch.into()),
        );

        keys[last_index] = last_key;
        assert!(assert_distinct_non_default_keys(&keys).is_ok());
    }

    #[test]
    fn quadratic_distinct_key_fallback_rejects_default_and_duplicate_keys() {
        let mut keys = [Pubkey::default(); COMPACT_DISTINCT_ACCOUNT_CAPACITY + 1];
        let mut index = 0usize;
        while index < keys.len() {
            keys[index] = Pubkey::new_unique();
            index += 1;
        }

        assert!(assert_distinct_non_default_keys(&keys).is_ok());

        let last_index = keys.len() - 1;
        keys[last_index] = keys[0];
        assert_eq!(
            assert_distinct_non_default_keys(&keys),
            Err(ChanceryError::AccountAliasNotAllowed.into()),
        );

        keys[last_index] = Pubkey::default();
        assert_eq!(
            assert_distinct_non_default_keys(&keys),
            Err(ChanceryError::AccountKeyMismatch.into()),
        );
    }

    #[test]
    fn present_account_distinctness_checks_both_capacity_paths() {
        let mut keys = [Pubkey::default(); COMPACT_DISTINCT_ACCOUNT_CAPACITY + 1];
        let mut index = 0usize;
        while index < keys.len() {
            keys[index] = Pubkey::new_unique();
            index += 1;
        }

        let compact_accounts = leaked_account_infos_for_keys(
            &keys[..COMPACT_DISTINCT_ACCOUNT_CAPACITY],
        );
        let compact_refs: Vec<&AccountInfo> = compact_accounts.iter().collect();
        assert!(assert_distinct_present_accounts(&compact_refs).is_ok());

        let mut compact_duplicate_keys = keys[..COMPACT_DISTINCT_ACCOUNT_CAPACITY].to_vec();
        compact_duplicate_keys[COMPACT_DISTINCT_ACCOUNT_CAPACITY - 1] =
            compact_duplicate_keys[0];
        let compact_duplicate_accounts =
            leaked_account_infos_for_keys(&compact_duplicate_keys);
        let compact_duplicate_refs: Vec<&AccountInfo> =
            compact_duplicate_accounts.iter().collect();
        assert_eq!(
            assert_distinct_present_accounts(&compact_duplicate_refs),
            Err(ChanceryError::AccountAliasNotAllowed.into()),
        );

        let fallback_accounts = leaked_account_infos_for_keys(&keys);
        let fallback_refs: Vec<&AccountInfo> = fallback_accounts.iter().collect();
        assert!(assert_distinct_present_accounts(&fallback_refs).is_ok());

        let mut fallback_duplicate_keys = keys;
        fallback_duplicate_keys[COMPACT_DISTINCT_ACCOUNT_CAPACITY] =
            fallback_duplicate_keys[0];
        let fallback_duplicate_accounts =
            leaked_account_infos_for_keys(&fallback_duplicate_keys);
        let fallback_duplicate_refs: Vec<&AccountInfo> =
            fallback_duplicate_accounts.iter().collect();
        assert_eq!(
            assert_distinct_present_accounts(&fallback_duplicate_refs),
            Err(ChanceryError::AccountAliasNotAllowed.into()),
        );
    }

    #[test]
    fn duplicate_optional_placeholders_are_allowed() {
        let default_key = Pubkey::default();
        let owner = Pubkey::new_unique();
        let mut first_lamports = 0u64;
        let mut second_lamports = 0u64;
        let mut first_data = [];
        let mut second_data = [];
        let first = account_info_for_key(
            &default_key,
            &mut first_lamports,
            &mut first_data,
            &owner,
        );
        let second = account_info_for_key(
            &default_key,
            &mut second_lamports,
            &mut second_data,
            &owner,
        );
        assert!(assert_distinct_present_accounts(&[&first, &second]).is_ok());
    }
}
