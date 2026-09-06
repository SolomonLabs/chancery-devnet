use solana_account_info::AccountInfo;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_program_entrypoint::ProgramResult;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::ix::settlement as ix_id,
    error::ChanceryError,
    modules::{
        evidence::state::evidence_policy::EvidencePolicy,
        pathway::state::pathway_policy::PathwayPolicy,
        settlement::state::settlement_policy::SettlementPolicy,
    },
};

const COMPACT_DISTINCT_INDEX_CAPACITY: usize = 32;

/// Reject duplicate accounts across independently validated settlement
/// dimensions. Optional default-key placeholders and optional indexes that are
/// not present are ignored. Token-program indexes are deliberately omitted by
/// callers because the same Token-2022 program may service both mints.
#[inline]
pub fn assert_distinct_settlement_account_indexes(
    accounts: &[AccountInfo],
    indexes:  &[usize],
) -> ProgramResult {
    if indexes.len() > COMPACT_DISTINCT_INDEX_CAPACITY {
        return assert_distinct_settlement_account_indexes_quadratic(accounts, indexes);
    }

    let account_count = accounts.len();
    let default_key = Pubkey::default();
    let mut present_positions = [0u8; COMPACT_DISTINCT_INDEX_CAPACITY];
    let mut present_count = 0usize;
    let mut position = 0usize;
    while position < indexes.len() {
        let account_index = indexes[position];
        if account_index < account_count && accounts[account_index].key != &default_key {
            present_positions[present_count] = position as u8;
            present_count += 1;
        }
        position += 1;
    }

    let mut first_position = 0usize;
    while first_position < present_count {
        let first_index = indexes[present_positions[first_position] as usize];
        let first_key = accounts[first_index].key;
        let mut second_position = first_position + 1;
        while second_position < present_count {
            let second_index = indexes[present_positions[second_position] as usize];
            if first_key == accounts[second_index].key {
                return Err(ChanceryError::AccountAliasNotAllowed.into());
            }
            second_position += 1;
        }
        first_position += 1;
    }
    Ok(())
}

#[cold]
fn assert_distinct_settlement_account_indexes_quadratic(
    accounts: &[AccountInfo],
    indexes:  &[usize],
) -> ProgramResult {
    let account_count = accounts.len();
    let default_key = Pubkey::default();
    let mut first_position = 0usize;
    while first_position < indexes.len() {
        let first_index = indexes[first_position];
        if first_index < account_count && accounts[first_index].key != &default_key {
            let mut second_position = first_position + 1;
            while second_position < indexes.len() {
                let second_index = indexes[second_position];
                if second_index < account_count
                    && accounts[second_index].key != &default_key
                    && accounts[first_index].key == accounts[second_index].key
                {
                    return Err(ChanceryError::AccountAliasNotAllowed.into());
                }
                second_position += 1;
            }
        }
        first_position += 1;
    }
    Ok(())
}

/// Reject aliases across two semantic account groups while allowing overlap
/// within either group. This is used for identities and permission records:
/// one subject may occupy multiple roles and one canonical permission PDA may
/// satisfy multiple role checks, but neither may alias economic, policy, mint,
/// token-program, or protocol-state accounts.
pub fn assert_disjoint_settlement_account_index_groups(
    accounts:       &[AccountInfo],
    first_indexes:  &[usize],
    second_indexes: &[usize],
) -> ProgramResult {
    let mut first_position = 0usize;
    while first_position < first_indexes.len() {
        let first_index = first_indexes[first_position];
        if first_index < accounts.len() && accounts[first_index].key != &Pubkey::default() {
            let mut second_position = 0usize;
            while second_position < second_indexes.len() {
                let second_index = second_indexes[second_position];
                if second_index < accounts.len()
                    && accounts[second_index].key != &Pubkey::default()
                    && accounts[first_index].key == accounts[second_index].key
                {
                    return Err(ChanceryError::AccountAliasNotAllowed.into());
                }
                second_position += 1;
            }
        }
        first_position += 1;
    }
    Ok(())
}

pub mod create_settlement_intent;
pub mod cancel_settlement_intent;
pub mod close_expired_settlement_intent;
pub mod mint_delegated;
pub mod mint_direct;
pub mod mint_trilateral;
pub mod redeem_delegated;
pub mod redeem_direct;
pub mod redeem_trilateral;
pub mod register_settlement_policy;

pub fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
    if data.is_empty() {
        return Err(ChanceryError::InstructionDataTooShort.into());
    }

    match data[0] {
        ix_id::CREATE_SETTLEMENT_INTENT        => create_settlement_intent::handle(accounts, &data[1..]),
        ix_id::CANCEL_SETTLEMENT_INTENT        => cancel_settlement_intent::handle(accounts, &data[1..]),
        ix_id::CLOSE_EXPIRED_SETTLEMENT_INTENT => close_expired_settlement_intent::handle(accounts, &data[1..]),
        ix_id::MINT_DELEGATED                  => mint_delegated::handle(accounts, &data[1..]),
        ix_id::MINT_DIRECT                     => mint_direct::handle(accounts, &data[1..]),
        ix_id::MINT_TRILATERAL                 => mint_trilateral::handle(accounts, &data[1..]),
        ix_id::REDEEM_DELEGATED                => redeem_delegated::handle(accounts, &data[1..]),
        ix_id::REDEEM_DIRECT                   => redeem_direct::handle(accounts, &data[1..]),
        ix_id::REDEEM_TRILATERAL               => redeem_trilateral::handle(accounts, &data[1..]),
        ix_id::REGISTER_SETTLEMENT_POLICY      => register_settlement_policy::handle(accounts, &data[1..]),
        _                                      => Err(ChanceryError::UnknownInstruction.into()),
    }
}

/// Resolve and validate a non-zero settlement policy reference.
/// A zero policy ID means the intent is not policy-backed and requires no account.
/// Bind and validate a referenced settlement policy account. The caller gates
/// on `policy_id != 0` and on account presence (mirroring the fee/limit
/// optional-account pattern), keeping every account index visible in the
/// handler for source-derived IDL generation.
pub fn assert_settlement_policy_for_terms<'a>(
    settlement_policy_account_info: &'a AccountInfo<'a>,
    policy_id:                      &[u8; 32],
    settlement_mode:                u8,
    asset_mint:                     &Pubkey,
    principal_a:                    &Pubkey,
    principal_b:                    &Pubkey,
    executor:                       &Pubkey,
    asset_amount:                   u64,
    now:                            i64,
) -> ProgramResult {
    let expected_bump =
        SettlementPolicy::verify_pda(settlement_policy_account_info, policy_id, &crate::id())?;
    let settlement_policy = SettlementPolicy::load_for_verified_pda(
        settlement_policy_account_info,
        policy_id,
        expected_bump,
    )?;

    if settlement_policy.policy_id != *policy_id {
        return Err(ChanceryError::InvalidPda.into());
    }

    if settlement_policy.bump != expected_bump {
        return Err(ChanceryError::StoredBumpMismatch.into());
    }

    settlement_policy.assert_effective(now)?;
    settlement_policy.assert_settlement_mode_allowed(settlement_mode)?;
    settlement_policy.assert_asset(asset_mint)?;
    settlement_policy.assert_principals(principal_a, principal_b)?;
    settlement_policy.assert_executor(executor)?;
    settlement_policy.assert_notional(asset_amount)
}

/// Resolve the pathway evidence policy and reject requirements that the current
/// settlement event schema cannot prove. The caller gates on
/// `pathway.has_evidence_policy()` and on account presence, keeping every
/// account index visible in the handler for source-derived IDL generation.
pub fn assert_pathway_evidence_policy<'a>(
    evidence_policy_account_info: &'a AccountInfo<'a>,
    pathway: &PathwayPolicy,
) -> ProgramResult {
    let expected_bump = EvidencePolicy::verify_pda(
        evidence_policy_account_info,
        &pathway.evidence_policy_id,
        &crate::id(),
    )?;

    let evidence_policy = EvidencePolicy::load_for_verified_pda(
        evidence_policy_account_info,
        &pathway.evidence_policy_id,
        expected_bump,
    )?;

    evidence_policy.assert_settlement_event_supported()
}

// ─── Shared settlement helpers ────────────────────────────────────────────────
//
// All token CPIs use the *Checked variants (TransferChecked / MintToChecked /
// BurnChecked) so collateral mints carrying the Token-2022 TransferFeeConfig
// extension are handled correctly. Plain `Transfer` (disc 3) reverts with
// `MintHasFee` against any transfer-fee mint, which would silently brick
// every settlement path. Checked variants also guard against decimals drift
// between the asset_config record and the on-chain mint.

/// Read mint decimals (byte 44 of the SPL Token / Token-2022 mint layout:
/// mint_authority option(36) + supply(8) + decimals(1)). The owning token
/// program is checked before any raw-layout read.
#[inline]
fn read_mint_decimals(
    mint_account_info:          &AccountInfo,
    token_program_account_info: &AccountInfo,
) -> Result<u8, ProgramError> {
    if mint_account_info.owner != token_program_account_info.key {
        return Err(ChanceryError::TokenProgramMismatch.into());
    }

    let data = mint_account_info.try_borrow_data()?;

    if data.len() < 45 {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    Ok(data[44])
}

/// Read an SPL Token / Token-2022 token account's `amount` (u64 LE at byte
/// offset 64). The base token-account layout is identical across both programs,
/// so this works for classic and Token-2022 reserve accounts alike. The owning
/// token program is checked before any raw-layout read.
#[inline]
fn read_token_account_amount(
    token_account_info:         &AccountInfo,
    token_program_account_info: &AccountInfo,
) -> Result<u64, ProgramError> {
    if token_account_info.owner != token_program_account_info.key {
        return Err(ChanceryError::TokenProgramMismatch.into());
    }

    let data = token_account_info.try_borrow_data()?;

    if data.len() < 72 {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    let mut amount_bytes = [0u8; 8];
    amount_bytes.copy_from_slice(&data[64..72]);
    Ok(u64::from_le_bytes(amount_bytes))
}

/// Build `TransferChecked` (discriminant 12).
/// Wire: [12u8, amount(8 LE), decimals(1)].
/// Accounts: [source, mint(readonly), destination, authority(signer)].
pub fn token_transfer_checked_ix(
    token_program: &Pubkey,
    source:        &Pubkey,
    mint:          &Pubkey,
    destination:   &Pubkey,
    authority:     &Pubkey,
    amount:        u64,
    decimals:      u8,
) -> Instruction {
    let mut data = Vec::with_capacity(10);

    data.push(12u8);
    data.extend_from_slice(&amount.to_le_bytes());
    data.push(decimals);

    Instruction {
        program_id: *token_program,
        accounts: vec![
            AccountMeta::new(*source,              false),
            AccountMeta::new_readonly(*mint,       false),
            AccountMeta::new(*destination,         false),
            AccountMeta::new_readonly(*authority,  true),
        ],
        data,
    }
}

/// Build `MintToChecked` (discriminant 14). Same accounts as MintTo, trailing decimals byte.
pub fn token_mint_to_checked_ix(
    token_program:  &Pubkey,
    mint:           &Pubkey,
    destination:    &Pubkey,
    mint_authority: &Pubkey,
    amount:         u64,
    decimals:       u8,
) -> Instruction {
    let mut data = Vec::with_capacity(10);

    data.push(14u8);
    data.extend_from_slice(&amount.to_le_bytes());
    data.push(decimals);

    Instruction {
        program_id: *token_program,
        accounts: vec![
            AccountMeta::new(*mint,                    false),
            AccountMeta::new(*destination,             false),
            AccountMeta::new_readonly(*mint_authority, true),
        ],
        data,
    }
}

/// Build `BurnChecked` (discriminant 15). Same accounts as Burn, trailing decimals byte.
pub fn token_burn_checked_ix(
    token_program: &Pubkey,
    account:       &Pubkey,
    mint:          &Pubkey,
    authority:     &Pubkey,
    amount:        u64,
    decimals:      u8,
) -> Instruction {
    let mut data = Vec::with_capacity(10);

    data.push(15u8);
    data.extend_from_slice(&amount.to_le_bytes());
    data.push(decimals);

    Instruction {
        program_id: *token_program,
        accounts: vec![
            AccountMeta::new(*account,            false),
            AccountMeta::new(*mint,               false),
            AccountMeta::new_readonly(*authority, true),
        ],
        data,
    }
}

/// CPI: an owner or approved delegate transfers asset into the reserve token
/// account. The supplied authority signs directly; no Chancery PDA signs.
///
/// Returns the amount the reserve **actually received**, measured as the balance
/// delta across the transfer. For a Token-2022 collateral carrying an active
/// `TransferFeeConfig` (or a transfer hook that skims), the reserve is credited
/// with less than `amount`; callers must base issuance on the returned value,
/// not on `amount`, or the issued token would be under-collateralized (issue-73).
/// The measurement is program-agnostic - identical for classic SPL and Token-2022.
pub fn cpi_transfer_asset_in<'a>(
    token_program_account_info: &AccountInfo<'a>,
    source_account_info:        &AccountInfo<'a>,
    asset_mint_account_info:    &AccountInfo<'a>,
    reserve_account_info:       &AccountInfo<'a>,
    authority_account_info:     &AccountInfo<'a>,
    amount:                     u64,
) -> Result<u64, ProgramError> {
    let decimals = read_mint_decimals(asset_mint_account_info, token_program_account_info)?;

    let reserve_balance_before = read_token_account_amount(reserve_account_info, token_program_account_info)?;

    let ix = token_transfer_checked_ix(
        token_program_account_info.key,
        source_account_info.key,
        asset_mint_account_info.key,
        reserve_account_info.key,
        authority_account_info.key,
        amount,
        decimals,
    );

    invoke(
        &ix,
        &[
            source_account_info.clone(),
            asset_mint_account_info.clone(),
            reserve_account_info.clone(),
            authority_account_info.clone(),
            token_program_account_info.clone(),
        ],
    )?;

    let reserve_balance_after = read_token_account_amount(reserve_account_info, token_program_account_info)?;

    // Net credit to the reserve. A transfer can only increase the reserve unless
    // a hostile transfer hook drains it mid-CPI; in that case the underflow
    // fails the whole instruction (nothing is minted).
    reserve_balance_after
        .checked_sub(reserve_balance_before)
        .ok_or(ChanceryError::ArithmeticUnderflow.into())
}

/// CPI: mint issued tokens to the recipient. Signs with mint_authority PDA.
pub fn cpi_mint_to<'a>(
    token_program_account_info:  &AccountInfo<'a>,
    mint_account_info:           &AccountInfo<'a>,
    destination_account_info:    &AccountInfo<'a>,
    mint_authority_account_info: &AccountInfo<'a>,
    mint_authority_bump:         u8,
    amount:                      u64,
) -> ProgramResult {
    let decimals = read_mint_decimals(mint_account_info, token_program_account_info)?;

    cpi_mint_to_with_decimals(
        token_program_account_info,
        mint_account_info,
        destination_account_info,
        mint_authority_account_info,
        mint_authority_bump,
        amount,
        decimals,
    )
}

/// CPI counterpart that reuses a decimals byte already validated against the
/// same mint and token-program accounts earlier in the handler.
pub(crate) fn cpi_mint_to_with_decimals<'a>(
    token_program_account_info:  &AccountInfo<'a>,
    mint_account_info:           &AccountInfo<'a>,
    destination_account_info:    &AccountInfo<'a>,
    mint_authority_account_info: &AccountInfo<'a>,
    mint_authority_bump:         u8,
    amount:                      u64,
    decimals:                    u8,
) -> ProgramResult {
    let ix = token_mint_to_checked_ix(
        token_program_account_info.key,
        mint_account_info.key,
        destination_account_info.key,
        mint_authority_account_info.key,
        amount,
        decimals,
    );

    invoke_signed(
        &ix,
        &[mint_account_info.clone(), destination_account_info.clone(), mint_authority_account_info.clone(), token_program_account_info.clone()],
        &[&[crate::constants::seeds::MINT_AUTHORITY, &[mint_authority_bump]]],
    )
}

/// CPI: burn issued tokens from sender. The supplied token-account owner or
/// approved delegate signs directly.
pub fn cpi_burn<'a>(
    token_program_account_info: &AccountInfo<'a>,
    account_account_info:       &AccountInfo<'a>,
    mint_account_info:          &AccountInfo<'a>,
    authority_account_info:     &AccountInfo<'a>,
    amount:                     u64,
) -> ProgramResult {
    let decimals = read_mint_decimals(mint_account_info, token_program_account_info)?;

    let ix = token_burn_checked_ix(
        token_program_account_info.key,
        account_account_info.key,
        mint_account_info.key,
        authority_account_info.key,
        amount,
        decimals,
    );

    invoke(
        &ix,
        &[account_account_info.clone(), mint_account_info.clone(), authority_account_info.clone(), token_program_account_info.clone()],
    )
}

/// CPI: transfer asset out of program-owned reserve. Signs with reserve_authority PDA.
pub fn cpi_transfer_asset_out<'a>(
    token_program_account_info:     &AccountInfo<'a>,
    reserve_account_info:           &AccountInfo<'a>,
    asset_mint_account_info:        &AccountInfo<'a>,
    destination_account_info:       &AccountInfo<'a>,
    reserve_authority_account_info: &AccountInfo<'a>,
    reserve_authority_bump:         u8,
    amount:                         u64,
) -> Result<u64, ProgramError> {
    let decimals = read_mint_decimals(asset_mint_account_info, token_program_account_info)?;

    cpi_transfer_asset_out_with_decimals(
        token_program_account_info,
        reserve_account_info,
        asset_mint_account_info,
        destination_account_info,
        reserve_authority_account_info,
        reserve_authority_bump,
        amount,
        decimals,
    )
}

/// Reserve transfer counterpart that reuses a decimals byte already validated
/// against the same mint and token-program accounts earlier in the handler.
pub(crate) fn cpi_transfer_asset_out_with_decimals<'a>(
    token_program_account_info:     &AccountInfo<'a>,
    reserve_account_info:           &AccountInfo<'a>,
    asset_mint_account_info:        &AccountInfo<'a>,
    destination_account_info:       &AccountInfo<'a>,
    reserve_authority_account_info: &AccountInfo<'a>,
    reserve_authority_bump:         u8,
    amount:                         u64,
    decimals:                       u8,
) -> Result<u64, ProgramError> {
    let destination_balance_before = read_token_account_amount(destination_account_info, token_program_account_info)?;

    let ix = token_transfer_checked_ix(
        token_program_account_info.key,
        reserve_account_info.key,
        asset_mint_account_info.key,
        destination_account_info.key,
        reserve_authority_account_info.key,
        amount,
        decimals,
    );

    invoke_signed(
        &ix,
        &[
            reserve_account_info.clone(),
            asset_mint_account_info.clone(),
            destination_account_info.clone(),
            reserve_authority_account_info.clone(),
            token_program_account_info.clone(),
        ],
        &[&[crate::constants::seeds::RESERVE_AUTHORITY, &[reserve_authority_bump]]],
    )?;

    let destination_balance_after = read_token_account_amount(destination_account_info, token_program_account_info)?;
    destination_balance_after
        .checked_sub(destination_balance_before)
        .ok_or(ChanceryError::ArithmeticUnderflow.into())
}

/// Resolve the mint-authority PDA address.
pub fn mint_authority_pda(program_id: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[crate::constants::seeds::MINT_AUTHORITY],
        program_id,
    )
    .0
}

/// SPL Token / Token-2022 account: mint @ 0..32, owner @ 32..64.
///
/// This helper proves the account is owned by the exact token program selected by
/// the pathway/config before reading the base account fields. Token-2022 extension
/// data may follow the base layout and is intentionally ignored here.
pub fn assert_spl_token_account_binding(
    token_account_info:         &AccountInfo,
    token_program_account_info: &AccountInfo,
    expected_mint:              &Pubkey,
    expected_owner:             &Pubkey,
) -> ProgramResult {
    if token_account_info.owner != token_program_account_info.key {
        return Err(ChanceryError::TokenProgramMismatch.into());
    }

    let data = token_account_info.try_borrow_data()?;

    if data.len() < 64 {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    let mut mint_bytes:  [u8; 32] = [0u8; 32];
    let mut owner_bytes: [u8; 32] = [0u8; 32];

    mint_bytes.copy_from_slice(&data[0..32]);
    owner_bytes.copy_from_slice(&data[32..64]);

    if mint_bytes != expected_mint.to_bytes() {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if owner_bytes != expected_owner.to_bytes() {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    Ok(())
}

/// Binds the supplied issued-token mint account and pathway to the singleton
/// `ChanceryConfig.issued_token_mint` (audit #49 / #50).
pub fn assert_canonical_issued_token_mint<'a>(
    chancery_config:                &crate::modules::core::state::chancery_config::ChanceryConfig,
    pathway:                        &crate::modules::pathway::state::pathway_policy::PathwayPolicy,
    issued_token_mint_account_info: &'a AccountInfo<'a>,
) -> ProgramResult {
    if issued_token_mint_account_info.key != &chancery_config.issued_token_mint {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    if &pathway.issued_token_mint != &chancery_config.issued_token_mint {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    Ok(())
}

/// Deployment-verification core shared by the settlement extension gates and
/// the migration mint path: IssuedTokenControl PDA + owner verification,
/// canonical issued-mint binding, `READY_FOR_SETTLEMENT` (set only by
/// `verify_issued_token_deployment`), and extension-observation freshness.
///
/// Returns the control's `active_mint_extension_mask` so pathway-scoped
/// callers can apply their extension policy on top; pathway-less callers
/// (migration) ignore it.
pub fn assert_issued_token_deployment_ready<'a>(
    issued_token_control_account_info: &'a AccountInfo<'a>,
    issued_token_mint_account_info:    &'a AccountInfo<'a>,
    current_slot:                      u64,
    program_id:                        &Pubkey,
) -> Result<[u64; 2], ProgramError> {
    use crate::modules::issuance::state::issued_token_control::IssuedTokenControl;

    let expected_bump =
        IssuedTokenControl::verify_pda(issued_token_control_account_info, program_id)?;
    let ctrl = IssuedTokenControl::load_for_verified_pda(
        issued_token_control_account_info,
        expected_bump,
    )?;

    if &ctrl.issued_token_mint != issued_token_mint_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    ctrl.assert_ready_for_settlement()?;
    ctrl.assert_extensions_fresh(current_slot)?;

    Ok(ctrl.active_mint_extension_mask)
}

/// Singleton `IssuedTokenControl` PDA verification, `READY_FOR_SETTLEMENT`
/// deployment gate, extension freshness, and pathway mask checks.
/// Collateral extension freshness is checked separately on `AssetConfig`.
pub fn assert_issued_token_extension_gates<'a>(
    issued_token_control_account_info: &'a AccountInfo<'a>,
    issued_token_mint_account_info:    &'a AccountInfo<'a>,
    pathway:                           &crate::modules::pathway::state::pathway_policy::PathwayPolicy,
    current_slot:                      u64,
    program_id:                        &Pubkey,
) -> ProgramResult {
    let active_mint_extension_mask = assert_issued_token_deployment_ready(
        issued_token_control_account_info,
        issued_token_mint_account_info,
        current_slot,
        program_id,
    )?;

    pathway.assert_issued_token_extensions_allowed(active_mint_extension_mask)?;

    Ok(())
}

// ─── Tests ────────────────────────────────────────────────────────────────────
// Regression coverage for issue-42: legacy `Transfer` (disc 3) reverts with
// `MintHasFee` against any Token-2022 mint carrying TransferFeeConfig. We
// migrated all token CPIs to the *Checked variants (12/14/15), which take a
// trailing decimals byte and (for transfer) the mint as an extra account.
// These tests pin the wire shape so a future refactor can't silently revert
// us to the unchecked discriminants.
#[cfg(test)]
mod tests {
    use super::*;

    fn key(b: u8) -> Pubkey {
        Pubkey::new_from_array([b; 32])
    }

    fn four_distinct_accounts<'a>(
        keys: &'a [Pubkey; 4],
        lamports: &'a mut u64,
        data: &'a mut [u8],
        owner: &'a Pubkey,
    ) -> [AccountInfo<'a>; 4] {
        let first = AccountInfo::new(&keys[0], false, true, lamports, data, owner, false);
        let mut second = first.clone();
        let mut third = first.clone();
        let mut fourth = first.clone();
        second.key = &keys[1];
        third.key = &keys[2];
        fourth.key = &keys[3];
        [first, second, third, fourth]
    }

    #[test]
    fn settlement_account_alias_matrix_rejects_every_pair() {
        let keys = [key(1), key(2), key(3), key(4)];
        let owner = key(9);
        let mut lamports = 1u64;
        let mut data = [];
        let accounts = four_distinct_accounts(&keys, &mut lamports, &mut data, &owner);
        let indexes = [0usize, 1, 2, 3];
        assert!(assert_distinct_settlement_account_indexes(&accounts, &indexes).is_ok());

        let mut first_index = 0usize;
        while first_index < accounts.len() {
            let mut second_index = first_index + 1;
            while second_index < accounts.len() {
                let mut aliased = accounts.clone();
                aliased[second_index].key = aliased[first_index].key;
                assert_eq!(
                    assert_distinct_settlement_account_indexes(&aliased, &indexes),
                    Err(ChanceryError::AccountAliasNotAllowed.into()),
                    "account pair {first_index}/{second_index}",
                );
                second_index += 1;
            }
            first_index += 1;
        }
    }

    #[test]
    fn settlement_account_alias_check_ignores_default_and_missing_indexes() {
        let keys = [key(1), key(2), Pubkey::default(), key(4)];
        let owner = key(9);
        let mut lamports = 1u64;
        let mut data = [];
        let accounts = four_distinct_accounts(&keys, &mut lamports, &mut data, &owner);
        let indexes = [0usize, 2, 99, 1];

        assert!(assert_distinct_settlement_account_indexes(&accounts, &indexes).is_ok());

        let mut aliased = accounts.clone();
        aliased[1].key = aliased[0].key;
        assert_eq!(
            assert_distinct_settlement_account_indexes(&aliased, &indexes),
            Err(ChanceryError::AccountAliasNotAllowed.into()),
        );
    }

    #[test]
    fn semantic_groups_allow_internal_role_overlap_but_reject_cross_group_aliases() {
        let keys = [key(1), key(2), key(3), key(4)];
        let owner = key(9);
        let mut lamports = 1u64;
        let mut data = [];
        let accounts = four_distinct_accounts(&keys, &mut lamports, &mut data, &owner);
        let first_group = [0usize, 1];
        let second_group = [2usize, 3];

        assert!(assert_disjoint_settlement_account_index_groups(
            &accounts,
            &first_group,
            &second_group,
        ).is_ok());

        let mut internal_alias = accounts.clone();
        internal_alias[1].key = internal_alias[0].key;
        assert!(assert_disjoint_settlement_account_index_groups(
            &internal_alias,
            &first_group,
            &second_group,
        ).is_ok());

        internal_alias = accounts.clone();
        internal_alias[3].key = internal_alias[2].key;
        assert!(assert_disjoint_settlement_account_index_groups(
            &internal_alias,
            &first_group,
            &second_group,
        ).is_ok());

        let mut cross_group_alias = accounts.clone();
        cross_group_alias[2].key = cross_group_alias[0].key;
        assert_eq!(
            assert_disjoint_settlement_account_index_groups(
                &cross_group_alias,
                &first_group,
                &second_group,
            ),
            Err(ChanceryError::AccountAliasNotAllowed.into()),
        );
    }

    // ── TransferChecked (disc 12) ─────────────────────────────────────────────

    #[test]
    fn token_transfer_checked_ix_encodes_discriminator_amount_and_decimals() {
        let ix = token_transfer_checked_ix(
            &key(0xA0),  // token program
            &key(0xA1),  // source
            &key(0xA2),  // mint
            &key(0xA3),  // destination
            &key(0xA4),  // authority
            1_234_567_890u64,
            6,
        );

        assert_eq!(ix.program_id, key(0xA0));
        assert_eq!(ix.data.len(), 10);
        assert_eq!(ix.data[0], 12);
        assert_eq!(&ix.data[1..9], &1_234_567_890u64.to_le_bytes());
        assert_eq!(ix.data[9], 6);
    }

    #[test]
    fn token_transfer_checked_ix_includes_mint_as_readonly_account() {
        let ix = token_transfer_checked_ix(
            &key(0xA0), &key(0xA1), &key(0xA2), &key(0xA3), &key(0xA4),
            100, 9,
        );

        // [source(w), mint(ro), destination(w), authority(ro, signer)]
        assert_eq!(ix.accounts.len(), 4);

        assert_eq!(ix.accounts[0].pubkey, key(0xA1));
        assert!( ix.accounts[0].is_writable);
        assert!(!ix.accounts[0].is_signer);

        // The mint slot was added by this fix - without it, Token-2022 has no
        // way to consult the TransferFeeConfig extension and reverts.
        assert_eq!(ix.accounts[1].pubkey, key(0xA2));
        assert!(!ix.accounts[1].is_writable);
        assert!(!ix.accounts[1].is_signer);

        assert_eq!(ix.accounts[2].pubkey, key(0xA3));
        assert!( ix.accounts[2].is_writable);
        assert!(!ix.accounts[2].is_signer);

        assert_eq!(ix.accounts[3].pubkey, key(0xA4));
        assert!(!ix.accounts[3].is_writable);
        assert!( ix.accounts[3].is_signer);
    }

    // ── MintToChecked (disc 14) ───────────────────────────────────────────────

    #[test]
    fn token_mint_to_checked_ix_encodes_discriminator_amount_and_decimals() {
        let ix = token_mint_to_checked_ix(
            &key(0xB0), &key(0xB1), &key(0xB2), &key(0xB3),
            42u64,
            9,
        );

        assert_eq!(ix.program_id, key(0xB0));
        assert_eq!(ix.data.len(), 10);
        assert_eq!(ix.data[0], 14);
        assert_eq!(&ix.data[1..9], &42u64.to_le_bytes());
        assert_eq!(ix.data[9], 9);
    }

    #[test]
    fn token_mint_to_checked_ix_account_shape() {
        let ix = token_mint_to_checked_ix(
            &key(0xB0), &key(0xB1), &key(0xB2), &key(0xB3),
            1, 0,
        );

        // [mint(w), destination(w), mint_authority(ro, signer)]
        assert_eq!(ix.accounts.len(), 3);

        assert_eq!(ix.accounts[0].pubkey, key(0xB1));
        assert!( ix.accounts[0].is_writable);

        assert_eq!(ix.accounts[1].pubkey, key(0xB2));
        assert!( ix.accounts[1].is_writable);

        assert_eq!(ix.accounts[2].pubkey, key(0xB3));
        assert!(!ix.accounts[2].is_writable);
        assert!( ix.accounts[2].is_signer);
    }

    // ── BurnChecked (disc 15) ─────────────────────────────────────────────────

    #[test]
    fn token_burn_checked_ix_encodes_discriminator_amount_and_decimals() {
        let ix = token_burn_checked_ix(
            &key(0xC0), &key(0xC1), &key(0xC2), &key(0xC3),
            u64::MAX,
            18,
        );

        assert_eq!(ix.program_id, key(0xC0));
        assert_eq!(ix.data.len(), 10);
        assert_eq!(ix.data[0], 15);
        assert_eq!(&ix.data[1..9], &u64::MAX.to_le_bytes());
        assert_eq!(ix.data[9], 18);
    }

    #[test]
    fn token_burn_checked_ix_account_shape() {
        let ix = token_burn_checked_ix(
            &key(0xC0), &key(0xC1), &key(0xC2), &key(0xC3),
            1, 0,
        );

        // [token_account(w), mint(w), authority(ro, signer)]
        assert_eq!(ix.accounts.len(), 3);

        assert_eq!(ix.accounts[0].pubkey, key(0xC1));
        assert!( ix.accounts[0].is_writable);

        assert_eq!(ix.accounts[1].pubkey, key(0xC2));
        assert!( ix.accounts[1].is_writable);

        assert_eq!(ix.accounts[2].pubkey, key(0xC3));
        assert!(!ix.accounts[2].is_writable);
        assert!( ix.accounts[2].is_signer);
    }

    // ── Discriminator distinctness (guards against future copy-paste bugs) ───

    #[test]
    fn checked_discriminators_are_distinct_and_not_the_legacy_ones() {
        // Legacy unchecked variants (3/7/8) must not reappear.
        let transfer = token_transfer_checked_ix(
            &key(1), &key(2), &key(3), &key(4), &key(5), 0, 0,
        );
        let mint_to = token_mint_to_checked_ix(&key(1), &key(2), &key(3), &key(4), 0, 0);
        let burn    = token_burn_checked_ix(&key(1), &key(2), &key(3), &key(4), 0, 0);

        assert_eq!(transfer.data[0], 12);
        assert_eq!(mint_to.data[0],  14);
        assert_eq!(burn.data[0],     15);

        assert_ne!(transfer.data[0], 3);
        assert_ne!(mint_to.data[0],  7);
        assert_ne!(burn.data[0],     8);
    }
}
