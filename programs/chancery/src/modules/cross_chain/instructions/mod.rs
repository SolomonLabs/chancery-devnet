use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;

use crate::{
    constants::ix::cross_chain as ix,
    error::ChanceryError,
};

pub mod consume_inbound_message;
pub mod expire_inbound_message;
pub mod emit_outbound_message;
pub mod reclaim_expired_outbound;
pub mod relax_remote_domain_pause;
pub mod relax_remote_domain_pause_with_pending_change;
pub mod register_remote_domain_policy;
pub mod register_cross_chain_signer_set;
pub mod restrict_remote_domain_pause;
pub mod rotate_cross_chain_signer_set;
pub mod update_remote_domain_policy;
pub mod update_remote_domain_policy_with_pending_change;

/// Cross-chain instructions use the same account-separation primitive as
/// settlement. The underlying helper compacts present optional accounts for
/// the common <= 32-index case and retains the cold quadratic fallback for
/// larger account surfaces.
#[inline]
pub fn assert_distinct_cross_chain_account_indexes(
    accounts: &[AccountInfo],
    indexes:  &[usize],
) -> ProgramResult {
    crate::modules::settlement::instructions::assert_distinct_settlement_account_indexes(
        accounts,
        indexes,
    )
}

/// Reject aliases across semantic account groups while allowing overlap
/// within a group. This preserves valid cases where one external identity is
/// simultaneously payer, operator, governance authority, or refund recipient.
#[inline]
pub fn assert_disjoint_cross_chain_account_index_groups(
    accounts:       &[AccountInfo],
    first_indexes:  &[usize],
    second_indexes: &[usize],
) -> ProgramResult {
    crate::modules::settlement::instructions::assert_disjoint_settlement_account_index_groups(
        accounts,
        first_indexes,
        second_indexes,
    )
}

pub fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
    if data.is_empty() {
        return Err(ChanceryError::InstructionDataTooShort.into());
    }

    let ix_id = data[0];
    let args  = &data[1..];

    match ix_id {
        ix::CONSUME_INBOUND_MESSAGE         => consume_inbound_message::handle(accounts, args),
        ix::EMIT_OUTBOUND_MESSAGE           => emit_outbound_message::handle(accounts, args),
        ix::EXPIRE_INBOUND_MESSAGE          => expire_inbound_message::handle(accounts, args),
        ix::RECLAIM_EXPIRED_OUTBOUND        => reclaim_expired_outbound::handle(accounts, args),
        ix::RELAX_REMOTE_DOMAIN_PAUSE       => relax_remote_domain_pause::handle(accounts, args),
        ix::REGISTER_CROSS_CHAIN_SIGNER_SET => register_cross_chain_signer_set::handle(accounts, args),
        ix::REGISTER_REMOTE_DOMAIN_POLICY   => register_remote_domain_policy::handle(accounts, args),
        ix::RESTRICT_REMOTE_DOMAIN_PAUSE    => restrict_remote_domain_pause::handle(accounts, args),
        ix::ROTATE_CROSS_CHAIN_SIGNER_SET   => rotate_cross_chain_signer_set::handle(accounts, args),
        ix::UPDATE_REMOTE_DOMAIN_POLICY     => update_remote_domain_policy::handle(accounts, args),
        ix::UPDATE_REMOTE_DOMAIN_POLICY_WITH_PENDING_CHANGE =>
            update_remote_domain_policy_with_pending_change::handle(accounts, args),
        ix::RELAX_REMOTE_DOMAIN_PAUSE_WITH_PENDING_CHANGE =>
            relax_remote_domain_pause_with_pending_change::handle(accounts, args),
        _                                   => Err(ChanceryError::UnknownInstruction.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_pubkey::Pubkey;

    fn leaked_account_info(key: Pubkey) -> AccountInfo<'static> {
        let key = Box::leak(Box::new(key));
        let lamports = Box::leak(Box::new(0u64));
        let data = Box::leak(Vec::<u8>::new().into_boxed_slice());
        let owner = Box::leak(Box::new(Pubkey::new_unique()));

        AccountInfo::new(key, false, false, lamports, data, owner, false)
    }

    fn distinct_accounts(count: usize) -> Vec<AccountInfo<'static>> {
        let mut accounts = Vec::with_capacity(count);
        let mut index = 0usize;
        while index < count {
            accounts.push(leaked_account_info(Pubkey::new_unique()));
            index += 1;
        }
        accounts
    }

    #[test]
    fn cross_chain_account_alias_matrix_rejects_duplicate_accounts() {
        let accounts = distinct_accounts(4);
        let indexes = [0usize, 1, 2, 3];
        assert!(assert_distinct_cross_chain_account_indexes(&accounts, &indexes).is_ok());

        let mut aliased = accounts.clone();
        let aliased_key = aliased[0].key;
        aliased[3].key = aliased_key;
        assert_eq!(
            assert_distinct_cross_chain_account_indexes(&aliased, &indexes),
            Err(ChanceryError::AccountAliasNotAllowed.into()),
        );
    }

    #[test]
    fn cross_chain_account_alias_matrix_retains_quadratic_fallback() {
        let accounts = distinct_accounts(34);
        let indexes: Vec<usize> = (0usize..34usize).collect();
        assert!(assert_distinct_cross_chain_account_indexes(&accounts, &indexes).is_ok());

        let mut aliased = accounts.clone();
        let aliased_key = aliased[0].key;
        aliased[33].key = aliased_key;
        assert_eq!(
            assert_distinct_cross_chain_account_indexes(&aliased, &indexes),
            Err(ChanceryError::AccountAliasNotAllowed.into()),
        );
    }

    #[test]
    fn cross_chain_semantic_groups_allow_internal_identity_overlap_only() {
        let accounts = distinct_accounts(4);
        let protected = [0usize, 1];
        let identities = [2usize, 3];

        let mut internal_identity_alias = accounts.clone();
        let identity_key = internal_identity_alias[2].key;
        internal_identity_alias[3].key = identity_key;
        assert!(assert_disjoint_cross_chain_account_index_groups(
            &internal_identity_alias,
            &protected,
            &identities,
        )
        .is_ok());

        let mut cross_group_alias = accounts.clone();
        let protected_key = cross_group_alias[0].key;
        cross_group_alias[2].key = protected_key;
        assert_eq!(
            assert_disjoint_cross_chain_account_index_groups(
                &cross_group_alias,
                &protected,
                &identities,
            ),
            Err(ChanceryError::AccountAliasNotAllowed.into()),
        );
    }
}
