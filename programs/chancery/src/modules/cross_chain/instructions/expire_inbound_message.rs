//! `expire_inbound_message` - permissionless authenticated head-message
//! retirement and nonce advancement (CROSS_CHAIN 0x0A).
//!
//! Resolves the corridor hard-lock recorded as SL-01 in
//! `audits/reviews/2026-07-17-state-lock-and-stale-reference-review.md`:
//! under strict in-order delivery, an authentic inbound message that becomes
//! unexecutable or terminally policy-invalid can block its corridor forever -
//! `consume_inbound_message` rejects it, every later nonce fails
//! `RemoteNonceMismatch`, and the `RemoteNonce` PDA survives corridor
//! re-registration.
//!
//! This instruction is `consume_inbound_message` minus every value-moving
//! effect. It proves three things, each unfakeable, and does nothing else:
//!
//!   1. **Authenticity** - the canonical message hash is recomputed from the
//!      current policy/pathway/config state plus args (identical preimage
//!      construction to consume, including the policy's current
//!      `signer_set_id`), and a full attestation quorum over that hash is
//!      verified against the current signer set's Merkle root.
//!   2. **Position** - strict `RemoteNonce` equality: only the message at
//!      the head of the corridor can be expired.
//!   3. **Retirement eligibility** - either the message is terminally invalid
//!      (elapsed expiry, unexecutable kind/amount, current policy/cap rejection,
//!      or source-asset binding failure), or the recipient's exact canonical
//!      issued-token ATA is frozen, missing, or uninitialized. Recipient-state
//!      retirement is an explicit cancellation/refund decision based on state
//!      observed atomically during retirement. Temporary usage-window saturation
//!      is never a retirement condition.
//!
//! Then it advances the nonce atomically, records the hash on the
//! reconciliation cursor (the cursor means "last nonce-advancing message
//! hash"; the evidence stream distinguishes consumed from expired), and
//! emits `CrossChainInboundExpired` carrying the full canonical preimage and
//! a retirement-reason discriminant - the canonical trigger for the source
//! contract's reclaim path (spec 11 §6.6).
//!
//! **Check-selection rule** (deliberate divergences from consume - each
//! dropped check is an *executability* condition that could itself be
//! reconfigured after emission to block the expiry and reintroduce the
//! lock):
//!   - No signer, no permission record: fully permissionless. The proof
//!     set leaves nothing discretionary - an executable message fails gate 3,
//!     an unattested one fails gate 1, and a forged one changes the hash.
//!   - No `message_kind` allowlist: any authentically attested kind at the
//!     head nonce blocks the corridor equally (consume would reject an
//!     unsupported kind forever), so any expired one may be retired. The
//!     hash binds the kind; the quorum vouches for it.
//!   - No global `pause_bit::MINT` gate: nothing mints.
//!   - Current expiry/cap policy and canonical remote-asset binding are read
//!     only to prove terminal rejection. The pathway must still be the active
//!     canonical `CROSS_CHAIN_MINT` pathway so a caller cannot substitute a
//!     same-asset pathway with a stricter cap. Recipient token binding applies
//!     only when proving a frozen, missing, or uninitialized canonical ATA.
//!     Chancery never creates that account. Issued-token extension gates and
//!     usage-window mutation do not apply: nothing moves.
//!   - Retained: the directional remote-domain pause minus `REMOTE_MINT`
//!     (relax paths exist, so honoring it cannot lock permanently, and it
//!     preserves the incident freeze), and the signer set's active +
//!     temporal validity (they define attestation validity - part of
//!     authenticity, not executability).
//!
//! Residual (recorded, not solvable here): after a signer-set rotation the
//! retired message needs re-attestation by the new set over the recomputed
//! hash - the same attester-liveness root consume already stands on.
//!
//! Accounts (in order):
//!   0  chancery_config                    writable  PDA [b"chancery-config"]
//!   1  event_authority                    readable  PDA [b"event-authority"]
//!   2  pathway_policy                     readable  PDA [b"pathway-policy", pathway_id]
//!   3  remote_domain_policy               readable  PDA [b"remote-domain-policy", chain_kind, domain_id_be]
//!   4  cross_chain_signer_set             readable  PDA [b"cross-chain-signer-set", signer_set_id]
//!   5  remote_nonce                       writable  PDA [b"remote-nonce", chain_kind, domain_id_be, default]
//!   6  limit_policy                       optional  PDA [b"limit-policy", pathway.limit_policy_id]; required iff configured
//!   7  recipient_issued_token_account      optional  exact canonical ATA; required only for recipient-state retirement

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;
use solana_system_interface::program as system_program;
use solana_sysvar::Sysvar;

use crate::{
    constants::{
        chain_kind, inbound_message_retirement_reason, pathway_kind, programs,
        remote_domain_pause_bit, scope, SOLANA_SELF_DOMAIN_ID,
    },
    error::ChanceryError,
    modules::{
        core::state::chancery_config::ChanceryConfig,
        cross_chain::{
            asset_binding::assert_inbound_remote_asset_binding,
            attestation::{
                verify_attestation_quorum, AttestationSignature,
                MAX_ATTESTED_INSTRUCTION_DATA_LEN,
            },
            instructions::assert_distinct_cross_chain_account_indexes,
            message_hash::{
                compute_epoch_free_content_hash, compute_message_hash, MessageHashPreimage,
                MESSAGE_HASH_CANON_VERSION,
            },
            state::{
                cross_chain_signer_set::CrossChainSignerSet,
                remote_domain_policy::{
                    expiry_violates_current_window, RemoteDomainPolicy,
                },
                remote_nonce::RemoteNonce,
            },
        },
        evidence::emit::{emit_cross_chain_inbound_expired, CrossChainInboundExpired},
        limits::state::limit_policy::LimitPolicy,
        pathway::state::pathway_policy::PathwayPolicy,
    },
};

// ─── Account indices ──────────────────────────────────────────────────────────

const CHANCERY_CONFIG:             usize = 0;
const EVENT_AUTHORITY:             usize = 1;
const PATHWAY_POLICY:              usize = 2;
const REMOTE_DOMAIN_POLICY:        usize = 3;
const SIGNER_SET:                  usize = 4;
const REMOTE_NONCE_ACCOUNT_INDEX:  usize = 5;
const LIMIT_POLICY:                usize = 6;
const RECIPIENT_ISSUED_TOKEN_ACCOUNT: usize = 7;
const REQUIRED_ACCOUNT_COUNT:      usize = 6;

const DISTINCT_ACCOUNT_INDEXES: &[usize] = &[
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    PATHWAY_POLICY,
    REMOTE_DOMAIN_POLICY,
    SIGNER_SET,
    REMOTE_NONCE_ACCOUNT_INDEX,
    LIMIT_POLICY,
    RECIPIENT_ISSUED_TOKEN_ACCOUNT,
];

/// SPL Token / Token-2022 base token-account layout. Extension TLV data, if
/// present, follows the 165-byte base and does not move the state byte.
const TOKEN_ACCOUNT_BASE_SIZE:    usize = 165;
const TOKEN_ACCOUNT_STATE_OFFSET: usize = 108;
const TOKEN_ACCOUNT_STATE_FROZEN: u8    = 2;

/// Classify the narrowly scoped issue-25 recipient retirement condition.
///
/// The caller must always pass the exact canonical ATA address. A system-owned,
/// data-empty account at that address represents an absent ATA; a Token-owned
/// account with SPL state zero is uninitialized. Neither case is created or
/// repaired by Chancery. Initialized accounts remain consumable and therefore
/// cannot be retired through this path.
pub(crate) fn classify_canonical_recipient_ata_retirement(
    token_account_info:    &AccountInfo,
    issued_token_program:  &Pubkey,
    issued_token_mint:     &Pubkey,
    recipient_owner:       &Pubkey,
) -> Result<u8, ProgramError> {
    let (expected_token_account, _) = Pubkey::find_program_address(
        &[
            recipient_owner.as_ref(),
            issued_token_program.as_ref(),
            issued_token_mint.as_ref(),
        ],
        &programs::ASSOCIATED_TOKEN_PROGRAM_ID,
    );

    if token_account_info.key != &expected_token_account {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    // A non-existent account is exposed to the program as an empty
    // system-owned account. A prefunded but still-unallocated canonical ATA has
    // the same execution semantics and is also unavailable for MintTo.
    if token_account_info.owner == &system_program::ID
        && token_account_info.data_is_empty()
    {
        return Ok(
            inbound_message_retirement_reason::RECIPIENT_ACCOUNT_MISSING_OR_UNINITIALIZED,
        );
    }

    if token_account_info.owner != issued_token_program {
        return Err(ChanceryError::TokenProgramMismatch.into());
    }

    let data = token_account_info.try_borrow_data()?;
    if data.len() < TOKEN_ACCOUNT_BASE_SIZE {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    match data[TOKEN_ACCOUNT_STATE_OFFSET] {
        0 => Ok(
            inbound_message_retirement_reason::RECIPIENT_ACCOUNT_MISSING_OR_UNINITIALIZED,
        ),
        1 | TOKEN_ACCOUNT_STATE_FROZEN => {
            if data[0..32] != issued_token_mint.to_bytes()
                || data[32..64] != recipient_owner.to_bytes()
            {
                return Err(ChanceryError::AccountKeyMismatch.into());
            }

            if data[TOKEN_ACCOUNT_STATE_OFFSET] == TOKEN_ACCOUNT_STATE_FROZEN {
                Ok(inbound_message_retirement_reason::RECIPIENT_ACCOUNT_FROZEN)
            } else {
                Err(ChanceryError::MessageNotExpired.into())
            }
        }
        _ => Err(ChanceryError::MessageNotExpired.into()),
    }
}

// ─── Args ─────────────────────────────────────────────────────────────────────
//
// Field-for-field identical to `ConsumeInboundMessageArgs`: the canonical
// hash must be recomputable from exactly the same inputs.
#[derive(BorshDeserialize)]
pub struct ExpireInboundMessageArgs {
    /// Local pathway selector; supplies destination asset / issued-token
    /// preimage fields. Must resolve to the active canonical
    /// `CROSS_CHAIN_MINT` pathway before its limits may classify retirement.
    pub pathway_id:                [u8; 32],

    /// Source remote chain-family discriminant. `chain_kind::SOLANA` is valid
    /// for another SVM deployment; corridor identity is not inferred from this
    /// discriminant alone.
    pub remote_chain_kind:         u8,

    /// Source remote domain id, BE-encoded into policy/nonce PDA seeds.
    pub remote_domain_id:          u64,

    // ── Canon scalars ──
    /// Bound by the hash and vouched for by the quorum; not allowlisted here.
    pub message_kind:              u8,

    /// Must equal `RemoteNonce.next_inbound_nonce` (strict head-of-corridor).
    pub source_nonce:              u64,

    /// Full canonical amount. Retirement accepts u128 so an authenticated head
    /// above Solana's u64 token domain can still be hash-verified and skipped.
    pub amount:                    u128,

    /// Unix expiry. A lapsed nonzero value is one retirement condition. A
    /// zero or future-expiry head may also be retired when its authenticated
    /// kind or amount is intrinsically unexecutable, or when its expiry violates
    /// the current finite policy window.
    pub expires_at_unix_timestamp: i64,

    // ── Canon 32-byte fields not loadable from on-chain state ──
    pub source_asset:              [u8; 32],
    pub sender:                    [u8; 32],
    pub recipient:                 [u8; 32],

    // ── Provided hash (must match recomputed canonical hash) ──
    pub provided_message_hash:     [u8; 32],

    // ── Attestation bundle ──
    pub signatures:                Vec<AttestationSignature>,
}

/// Classify why an authenticated head message is terminally unexecutable.
///
/// A finite expiry window is a terminal validity bound at the time the strict
/// nonce head is processed. Otherwise an arbitrarily distant authentic expiry
/// can block every later nonce while waiting to enter the rolling window.
/// Daily usage saturation remains temporary and never qualifies.
pub(crate) fn classify_message_retirement(
    message_kind:                    u8,
    expires_at_unix_timestamp:       i64,
    amount:                          u128,
    now:                             i64,
    message_expiry_seconds:          u64,
    per_message_maximum:             u64,
    per_day_maximum:                 u64,
    remote_asset_binding_valid:      bool,
    pathway_per_transaction_maximum: u64,
    pathway_period_maximums:         [u64; 4],
) -> Result<u8, ChanceryError> {
    if expires_at_unix_timestamp != 0 && now >= expires_at_unix_timestamp {
        return Ok(inbound_message_retirement_reason::EXPIRY_LAPSED);
    }

    if message_kind
        != crate::constants::cross_chain_message_kind::INBOUND_MINT_FROM_REMOTE_BURN
    {
        return Ok(inbound_message_retirement_reason::UNSUPPORTED_MESSAGE_KIND);
    }

    if amount == 0 {
        return Ok(inbound_message_retirement_reason::ZERO_AMOUNT);
    }

    if amount > u64::MAX as u128 {
        return Ok(inbound_message_retirement_reason::AMOUNT_OUT_OF_RANGE);
    }

    if expiry_violates_current_window(
        expires_at_unix_timestamp,
        now,
        message_expiry_seconds,
    ) {
        return Ok(inbound_message_retirement_reason::EXPIRY_POLICY_TIGHTENED);
    }

    if per_message_maximum != 0 && amount > per_message_maximum as u128 {
        return Ok(inbound_message_retirement_reason::PER_MESSAGE_CAP_TIGHTENED);
    }

    if per_day_maximum != 0 && amount > per_day_maximum as u128 {
        return Ok(inbound_message_retirement_reason::PER_DAY_CAP_TIGHTENED);
    }

    if !remote_asset_binding_valid {
        return Ok(inbound_message_retirement_reason::REMOTE_ASSET_BINDING_INVALID);
    }

    if pathway_per_transaction_maximum != 0
        && amount > pathway_per_transaction_maximum as u128
    {
        return Ok(
            inbound_message_retirement_reason::PATHWAY_PER_TRANSACTION_CAP_TIGHTENED,
        );
    }

    // A finite pathway fixed-window volume cap (hourly/daily/seven-day/thirty-day)
    // smaller than the amount is a TERMINAL bound, not transient saturation:
    // consume checks `window_gross + amount > cap`, so `amount > cap` fails even
    // against an empty window and can never clear with time. Ordinary saturation
    // (amount <= cap but the window is already full) is deliberately NOT retired.
    for period_maximum in pathway_period_maximums {
        if period_maximum != 0 && amount > period_maximum as u128 {
            return Ok(
                inbound_message_retirement_reason::PATHWAY_PERIOD_CAP_TIGHTENED,
            );
        }
    }

    Err(ChanceryError::MessageNotExpired)
}

// ─── Handler ──────────────────────────────────────────────────────────────────
pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    if args_data.len() > MAX_ATTESTED_INSTRUCTION_DATA_LEN {
        return Err(ChanceryError::AttestationResourceLimitExceeded.into());
    }

    assert_distinct_cross_chain_account_indexes(accounts, DISTINCT_ACCOUNT_INDEXES)?;

    let chancery_config_account_info = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info = &accounts[EVENT_AUTHORITY];
    let pathway_policy_account_info  = &accounts[PATHWAY_POLICY];
    let remote_policy_account_info   = &accounts[REMOTE_DOMAIN_POLICY];
    let signer_set_account_info      = &accounts[SIGNER_SET];
    let remote_nonce_account_info    = &accounts[REMOTE_NONCE_ACCOUNT_INDEX];

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !remote_nonce_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let args = ExpireInboundMessageArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let clock      = Clock::get()?;
    let program_id = crate::id();

    // ── Chancery config: domain separator for the canon hash ──────────────────
    let (
        chancery_domain_separator,
        issued_token_program,
        chancery_config_verified_bump,
    ): ([u8; 32], Pubkey, u8) = {
        let (chancery_config, verified_bump) =
            ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;
        (
            chancery_config.domain_separator,
            chancery_config.issued_token_program,
            verified_bump,
        )
    };

    // ── Pathway: active canonical inbound-mint identity ───────────────────────
    let pathway_policy_bump = PathwayPolicy::verify_pda(pathway_policy_account_info, &args.pathway_id, &program_id)?;
    let pathway = PathwayPolicy::load_for_verified_pda(
        pathway_policy_account_info,
        &args.pathway_id,
        pathway_policy_bump,
    )?;
    // Registration canonically derives cross-chain ids from
    // (kind, asset_mint, issued_token_mint); those identity fields are immutable.
    pathway.assert_active()?;
    pathway.assert_kind(pathway_kind::CROSS_CHAIN_MINT)?;

    // A configured pathway limit is part of permanent executability. Both the
    // absolute per-transaction cap AND any absolute fixed-window volume cap are
    // terminal when the amount exceeds them: the amount can never fit even in an
    // empty window. Transient saturation (amount within the cap, window already
    // full) clears with time and is classified as non-terminal in
    // classify_message_retirement.
    let (pathway_per_transaction_maximum, pathway_period_maximums) = if pathway.has_limit_policy() {
        if accounts.len() <= LIMIT_POLICY
            || accounts[LIMIT_POLICY].key == &Pubkey::default()
        {
            return Err(ChanceryError::MissingAccount.into());
        }

        let limit_policy_bump = LimitPolicy::verify_pda(
            &accounts[LIMIT_POLICY],
            &pathway.limit_policy_id,
            &program_id,
        )?;
        let limit_policy = LimitPolicy::load_for_verified_pda(
            &accounts[LIMIT_POLICY],
            &pathway.limit_policy_id,
            limit_policy_bump,
        )?;

        if limit_policy.scope_kind != scope::PATHWAY
            || limit_policy.scope_key != *pathway_policy_account_info.key
        {
            return Err(ChanceryError::LimitPolicyScopeMismatch.into());
        }

        (
            limit_policy.per_transaction_maximum,
            [
                limit_policy.per_hour_maximum,
                limit_policy.per_day_maximum,
                limit_policy.per_seven_day_maximum,
                limit_policy.per_thirty_day_maximum,
            ],
        )
    } else {
        (0, [0u64; 4])
    };

    // ── Remote-domain policy: canonical PDA + directional pause ───────────────
    let remote_policy_bump = RemoteDomainPolicy::verify_pda(
        remote_policy_account_info,
        args.remote_chain_kind,
        args.remote_domain_id,
        &program_id,
    )?;

    let remote_policy = RemoteDomainPolicy::load_for_verified_pda(
        remote_policy_account_info,
        args.remote_chain_kind,
        args.remote_domain_id,
        remote_policy_bump,
    )?;

    // ── Retirement eligibility (gate 3) ──────────────────────────────────────
    // Policy-terminal reasons include expiry/cap tightening, canonical
    // remote-asset binding failures, and pathway absolute caps. A frozen exact
    // canonical recipient ATA is the sole execution-state cancellation reason.
    let remote_asset_binding_valid = assert_inbound_remote_asset_binding(
        args.message_kind,
        &args.source_asset,
        &remote_policy,
    )
    .is_ok();
    let policy_retirement_reason = classify_message_retirement(
        args.message_kind,
        args.expires_at_unix_timestamp,
        args.amount,
        clock.unix_timestamp,
        remote_policy.message_expiry_seconds,
        remote_policy.per_message_maximum,
        remote_policy.per_day_maximum,
        remote_asset_binding_valid,
        pathway_per_transaction_maximum,
        pathway_period_maximums,
    );

    let retirement_reason = match policy_retirement_reason {
        Ok(reason) => reason,
        Err(ChanceryError::MessageNotExpired) => {
            // issue-25: a valid head addressed to a frozen, missing, or
            // uninitialized canonical ATA cannot complete its MintTo and would
            // otherwise block every later nonce. Chancery never creates the ATA;
            // the source sender must ensure it exists before initiating transfer.
            if accounts.len() <= RECIPIENT_ISSUED_TOKEN_ACCOUNT
                || accounts[RECIPIENT_ISSUED_TOKEN_ACCOUNT].key == &Pubkey::default()
            {
                return Err(ChanceryError::MessageNotExpired.into());
            }

            let recipient_owner = Pubkey::new_from_array(args.recipient);
            classify_canonical_recipient_ata_retirement(
                &accounts[RECIPIENT_ISSUED_TOKEN_ACCOUNT],
                &issued_token_program,
                &pathway.issued_token_mint,
                &recipient_owner,
            )?
        }
        Err(error) => return Err(error.into()),
    };

    // REMOTE_MINT deliberately excluded: nothing mints here, and honoring it
    // would let a mint-side freeze block corridor hygiene. The retained bits
    // cannot lock permanently - relax paths exist for all of them.
    remote_policy.assert_not_paused_for(
        remote_domain_pause_bit::INBOUND_MESSAGES
            | remote_domain_pause_bit::ATTESTATION_ACCEPTANCE
            | remote_domain_pause_bit::DAUGHTER_CONTRACT,
    )?;

    // ── Signer set: PDA, active, temporally valid, id matches policy ──────────
    // These define attestation validity (authenticity), not executability.
    let signer_set_bump = CrossChainSignerSet::verify_pda(signer_set_account_info, &remote_policy.signer_set_id, &program_id)?;

    let signer_set = CrossChainSignerSet::load_for_verified_pda(
        signer_set_account_info,
        &remote_policy.signer_set_id,
        signer_set_bump,
    )?;

    signer_set.assert_active()?;
    signer_set.assert_temporally_valid(clock.unix_timestamp)?;
    remote_policy.assert_signer_set_id_matches(&signer_set.signer_set_id)?;

    // ── Position (gate 2): RemoteNonce PDA + strict head-of-corridor ──────────
    let nonce_scope_key = [0u8; 32];
    let remote_nonce_bump = RemoteNonce::verify_pda(
        remote_nonce_account_info,
        args.remote_chain_kind,
        args.remote_domain_id,
        &nonce_scope_key,
        &program_id,
    )?;

    {
        let n = RemoteNonce::load_for_verified_pda(
            remote_nonce_account_info,
            args.remote_domain_id,
            &nonce_scope_key,
            remote_nonce_bump,
        )?;
        n.assert_inbound_nonce(args.source_nonce)?;
    }

    // ── Authenticity (gate 1): recompute the canon hash, verify the quorum ───
    //    Identical preimage construction to consume, including the CURRENT
    //    policy signer_set_id - a retired message straddling a rotation
    //    requires re-attestation by the new set, exactly like consumption.
    let destination_chancery_bytes = chancery_config_account_info.key.to_bytes();
    let destination_asset_bytes    = pathway.asset_mint.to_bytes();
    let destination_issued_bytes   = pathway.issued_token_mint.to_bytes();

    let preimage = MessageHashPreimage {
        message_kind:                  args.message_kind,
        source_chain_kind:             remote_policy.remote_chain_kind,
        destination_chain_kind:        chain_kind::SOLANA,
        source_domain_id:              remote_policy.remote_domain_id,
        destination_domain_id:         SOLANA_SELF_DOMAIN_ID,
        source_nonce:                  args.source_nonce,
        amount:                        args.amount,
        expires_at_unix_timestamp:     args.expires_at_unix_timestamp,
        source_chancery_contract:      &remote_policy.remote_chancery_contract,
        destination_chancery_contract: &destination_chancery_bytes,
        source_domain_separator:       &remote_policy.remote_domain_separator,
        destination_domain_separator:  &chancery_domain_separator,
        source_asset:                  &args.source_asset,
        destination_asset:             &destination_asset_bytes,
        source_issued_token:           &remote_policy.remote_issued_token,
        destination_issued_token:      &destination_issued_bytes,
        sender:                        &args.sender,
        recipient:                     &args.recipient,
        signer_set_id:                 &remote_policy.signer_set_id,
    };

    let message_hash = compute_message_hash(&preimage);

    if message_hash != args.provided_message_hash {
        return Err(ChanceryError::MessageHashMismatch.into());
    }

    // §15.4: the rotation-stable identity of the retired message - the same
    // preimage with `signer_set_id` zeroed. The source contract's reclaim
    // verifier (spec 15 §15.5 instruction D) binds to this hash, so E1 must
    // carry it verbatim.
    let epoch_free_content_hash = compute_epoch_free_content_hash(&preimage);

    verify_attestation_quorum(
        &message_hash,
        &signer_set.signer_root,
        signer_set.threshold,
        signer_set.signer_count,
        remote_policy.minimum_attestation_threshold,
        &args.signatures,
    )?;

    // ── Advance: retire the terminally invalid nonce ─────────────────────────
    // The cursor records the last nonce-advancing message hash; the evidence
    // stream (Consumed vs Expired) distinguishes how it advanced.
    {
        let mut n = RemoteNonce::load_mut_for_verified_pda(
            remote_nonce_account_info,
            args.remote_domain_id,
            &nonce_scope_key,
            remote_nonce_bump,
        )?;

        n.bump_inbound()?;

        n.last_consumed_message_hash = message_hash;
    }

    // ── Bump seq + emit (invariant #9: evidence last) ─────────────────────────
    // No token CPIs, no window mutation - the nonce bump above is the only
    // state effect, and this event is the canonical post-retirement reclaim
    // trigger for the source contract (spec 11 §6.6).
    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let seq                     = chancery_config_mut.next_sequence_nonce()?;

    emit_cross_chain_inbound_expired(
        event_authority_account_info,
        event_authority_bump,
        CrossChainInboundExpired {
            sequence_nonce:                seq,
            chancery:                      *chancery_config_account_info.key,
            slot:                          clock.slot,
            unix_timestamp:                clock.unix_timestamp,
            message_hash,
            epoch_free_content_hash,
            canon_version:                 MESSAGE_HASH_CANON_VERSION,
            message_kind:                  args.message_kind,
            source_chain_kind:             remote_policy.remote_chain_kind,
            destination_chain_kind:        chain_kind::SOLANA,
            source_domain_id:              remote_policy.remote_domain_id,
            destination_domain_id:         SOLANA_SELF_DOMAIN_ID,
            source_nonce:                  args.source_nonce,
            amount:                        args.amount,
            expires_at_unix_timestamp:     args.expires_at_unix_timestamp,
            source_chancery_contract:      remote_policy.remote_chancery_contract,
            destination_chancery_contract: destination_chancery_bytes,
            source_domain_separator:       remote_policy.remote_domain_separator,
            destination_domain_separator:  chancery_domain_separator,
            source_asset:                  args.source_asset,
            destination_asset:             destination_asset_bytes,
            source_issued_token:           remote_policy.remote_issued_token,
            destination_issued_token:      destination_issued_bytes,
            sender:                        args.sender,
            recipient:                     args.recipient,
            signer_set_id:                 remote_policy.signer_set_id,
            pathway_id:                    args.pathway_id,
            retirement_reason,
            attestation_count:             args.signatures.len() as u8,
        },
    )?;

    Ok(())
}

#[cfg(test)]
mod issue_19_tests {
    use super::*;

    const KIND: u8 = crate::constants::cross_chain_message_kind::INBOUND_MINT_FROM_REMOTE_BURN;

    // issue-19: a message whose amount exceeds a finite pathway fixed-window cap
    // can never execute - consume checks `window_gross + amount > cap`, so
    // `amount > cap` fails even against an empty window. With an unbounded
    // per-transaction cap (0) such a message used to be neither consumable nor
    // retirable, wedging the strict inbound nonce head permanently. It must now
    // classify as terminally retirable.
    #[test]
    fn amount_over_a_pathway_period_cap_is_terminally_retirable() {
        let reason = classify_message_retirement(
            KIND,
            0,     // no expiry
            101,   // amount
            0,     // now
            0,     // message_expiry_seconds (no expiry window)
            0,     // remote per_message_maximum (unbounded)
            0,     // remote per_day_maximum (unbounded)
            true,  // remote_asset_binding_valid
            0,     // pathway_per_transaction_maximum (unbounded)
            [100, 0, 0, 0], // hourly cap 100, others unbounded
        )
        .expect("over-cap amount must classify as terminally retirable");
        assert_eq!(
            reason,
            inbound_message_retirement_reason::PATHWAY_PERIOD_CAP_TIGHTENED,
        );
    }

    // Each of the four period slots (hourly/daily/seven-day/thirty-day)
    // independently makes an over-cap amount terminal.
    #[test]
    fn every_period_slot_independently_triggers_retirement() {
        for slot in 0..4 {
            let mut caps = [0u64; 4];
            caps[slot] = 100;
            let reason =
                classify_message_retirement(KIND, 0, 101, 0, 0, 0, 0, true, 0, caps)
                    .expect("over-cap amount must classify as terminally retirable");
            assert_eq!(
                reason,
                inbound_message_retirement_reason::PATHWAY_PERIOD_CAP_TIGHTENED,
                "slot {slot}",
            );
        }
    }

    // An amount within every cap is transient saturation at worst - NOT retirable.
    // Force-expiring it would let ordinary rate-limiting wedge the nonce head.
    #[test]
    fn amount_within_all_pathway_caps_is_not_retirable() {
        let result = classify_message_retirement(
            KIND, 0, 50, 0, 0, 0, 0, true, 0, [100, 1_000, 5_000, 20_000],
        );
        assert!(matches!(result, Err(ChanceryError::MessageNotExpired)));
    }
}

#[cfg(test)]
mod issue_25_tests {
    use super::*;

    fn account_info<'a>(
        key:      &'a Pubkey,
        owner:    &'a Pubkey,
        lamports: &'a mut u64,
        data:     &'a mut [u8],
    ) -> AccountInfo<'a> {
        AccountInfo::new(key, false, false, lamports, data, owner, false)
    }

    fn token_account_data(mint: &Pubkey, owner: &Pubkey, state: u8) -> Vec<u8> {
        let mut data = vec![0u8; TOKEN_ACCOUNT_BASE_SIZE];
        data[0..32].copy_from_slice(mint.as_ref());
        data[32..64].copy_from_slice(owner.as_ref());
        data[TOKEN_ACCOUNT_STATE_OFFSET] = state;
        data
    }

    #[test]
    fn exact_frozen_canonical_ata_qualifies_for_retirement() {
        let token_program = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let recipient = Pubkey::new_unique();
        let (canonical_ata, _) = Pubkey::find_program_address(
            &[recipient.as_ref(), token_program.as_ref(), mint.as_ref()],
            &programs::ASSOCIATED_TOKEN_PROGRAM_ID,
        );
        let mut lamports = 1u64;
        let mut data = token_account_data(&mint, &recipient, TOKEN_ACCOUNT_STATE_FROZEN);
        let account = account_info(&canonical_ata, &token_program, &mut lamports, &mut data);

        assert_eq!(
            classify_canonical_recipient_ata_retirement(
                &account,
                &token_program,
                &mint,
                &recipient,
            ),
            Ok(inbound_message_retirement_reason::RECIPIENT_ACCOUNT_FROZEN),
        );
    }

    #[test]
    fn missing_or_uninitialized_canonical_ata_qualifies_for_retirement() {
        let token_program = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let recipient = Pubkey::new_unique();
        let (canonical_ata, _) = Pubkey::find_program_address(
            &[recipient.as_ref(), token_program.as_ref(), mint.as_ref()],
            &programs::ASSOCIATED_TOKEN_PROGRAM_ID,
        );

        let mut missing_lamports = 0u64;
        let mut missing_data = [];
        let missing_account = account_info(
            &canonical_ata,
            &system_program::ID,
            &mut missing_lamports,
            &mut missing_data,
        );
        assert_eq!(
            classify_canonical_recipient_ata_retirement(
                &missing_account,
                &token_program,
                &mint,
                &recipient,
            ),
            Ok(
                inbound_message_retirement_reason::RECIPIENT_ACCOUNT_MISSING_OR_UNINITIALIZED,
            ),
        );

        let mut uninitialized_lamports = 1u64;
        let mut uninitialized_data = token_account_data(&mint, &recipient, 0);
        let uninitialized_account = account_info(
            &canonical_ata,
            &token_program,
            &mut uninitialized_lamports,
            &mut uninitialized_data,
        );
        assert_eq!(
            classify_canonical_recipient_ata_retirement(
                &uninitialized_account,
                &token_program,
                &mint,
                &recipient,
            ),
            Ok(
                inbound_message_retirement_reason::RECIPIENT_ACCOUNT_MISSING_OR_UNINITIALIZED,
            ),
        );
    }

    #[test]
    fn initialized_canonical_ata_does_not_qualify_for_retirement() {
        let token_program = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let recipient = Pubkey::new_unique();
        let (canonical_ata, _) = Pubkey::find_program_address(
            &[recipient.as_ref(), token_program.as_ref(), mint.as_ref()],
            &programs::ASSOCIATED_TOKEN_PROGRAM_ID,
        );
        let mut lamports = 1u64;
        let mut data = token_account_data(&mint, &recipient, 1);
        let account = account_info(&canonical_ata, &token_program, &mut lamports, &mut data);

        assert_eq!(
            classify_canonical_recipient_ata_retirement(
                &account,
                &token_program,
                &mint,
                &recipient,
            ),
            Err(ChanceryError::MessageNotExpired.into()),
        );
    }

    #[test]
    fn frozen_retirement_rejects_noncanonical_or_misbound_accounts() {
        let token_program = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let recipient = Pubkey::new_unique();
        let (canonical_ata, _) = Pubkey::find_program_address(
            &[recipient.as_ref(), token_program.as_ref(), mint.as_ref()],
            &programs::ASSOCIATED_TOKEN_PROGRAM_ID,
        );

        let wrong_key = Pubkey::new_unique();
        let mut wrong_key_lamports = 1u64;
        let mut wrong_key_data =
            token_account_data(&mint, &recipient, TOKEN_ACCOUNT_STATE_FROZEN);
        let wrong_key_account = account_info(
            &wrong_key,
            &token_program,
            &mut wrong_key_lamports,
            &mut wrong_key_data,
        );
        assert_eq!(
            classify_canonical_recipient_ata_retirement(
                &wrong_key_account,
                &token_program,
                &mint,
                &recipient,
            ),
            Err(ChanceryError::AccountKeyMismatch.into()),
        );

        let wrong_program = Pubkey::new_unique();
        let mut wrong_program_lamports = 1u64;
        let mut wrong_program_data =
            token_account_data(&mint, &recipient, TOKEN_ACCOUNT_STATE_FROZEN);
        let wrong_program_account = account_info(
            &canonical_ata,
            &wrong_program,
            &mut wrong_program_lamports,
            &mut wrong_program_data,
        );
        assert_eq!(
            classify_canonical_recipient_ata_retirement(
                &wrong_program_account,
                &token_program,
                &mint,
                &recipient,
            ),
            Err(ChanceryError::TokenProgramMismatch.into()),
        );

        let wrong_mint = Pubkey::new_unique();
        let mut wrong_mint_lamports = 1u64;
        let mut wrong_mint_data =
            token_account_data(&wrong_mint, &recipient, TOKEN_ACCOUNT_STATE_FROZEN);
        let wrong_mint_account = account_info(
            &canonical_ata,
            &token_program,
            &mut wrong_mint_lamports,
            &mut wrong_mint_data,
        );
        assert_eq!(
            classify_canonical_recipient_ata_retirement(
                &wrong_mint_account,
                &token_program,
                &mint,
                &recipient,
            ),
            Err(ChanceryError::AccountKeyMismatch.into()),
        );
    }
}
