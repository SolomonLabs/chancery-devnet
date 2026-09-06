//! `reclaim_expired_outbound` - permissionless post-retirement recovery of a
//! burned outbound emission (CROSS_CHAIN 0x0B, spec 15 §15.5 instruction B).
//!
//! When the daughter retires a Chancery-emitted outbound message because its
//! expiry lapsed or a later policy tightening made it permanently unexecutable,
//! it emits `InboundMessageExpired` (E2). The canonical cross-chain principal
//! committed at emission time is otherwise stranded. `emit_outbound_message`
//! burns a gross amount, deducts any non-refundable effective fee, and commits
//! only the net principal as canonical `amount`. This handler re-mints exactly
//! that canonical net principal to the ORIGINAL sender against quorum-attested
//! E2 evidence; fees already assessed at emission are not refunded.
//!
//! **No discretion anywhere.** The instruction is permissionless (the payer
//! signs only to fund the reclaim record's rent) and executable only under a
//! complete proof. The attester quorum is the sole trust root, and it gains
//! **no new power**: a quorum that can authorize reclaims could already
//! fabricate inbound mint messages outright, so reclaim authorization is
//! strictly weaker than the mint power the quorum already holds.
//!
//! Chancery deliberately keeps no per-emission records (spec 14): only
//! `next_outbound_nonce` and the last-emitted cursor exist. Consequently
//! (all normative, §15.5):
//!
//!   - *Emission authenticity is quorum-attested, not state-verified.* The
//!     reclaim digest R jointly attests "Chancery emitted this content at
//!     this nonce" and "the daughter expired it". On-chain checks are:
//!       1. `source_nonce < next_outbound_nonce` (plausibility floor);
//!       2. `epoch_free_content_hash` recomputed from the full supplied
//!          preimage (§15.4: canonical hash with `signer_set_id` zeroed)
//!          matches the caller-provided value bound into R;
//!       3. `message_kind` is an `OUTBOUND_*` burn kind (lock kinds have a
//!          different refund shape and are NOT re-mintable);
//!       4. a full attestation quorum over R against the CURRENT corridor
//!          signer set (active + temporally valid, id matching the policy).
//!   - *Double-reclaim prevention* is the permanent emission-identity
//!     `OutboundReclaimRecord` PDA ([b"outbound-reclaim", remote_chain_kind,
//!     remote_domain_id BE, source_nonce BE]): created here, existence ==
//!     this emission was already reclaimed under ANY content claim, NEVER
//!     closeable. Rent is paid by the reclaimer. Keying by (corridor, nonce)
//!     rather than by content hash enforces
//!     `reclaim_count(corridor, source_nonce) <= 1` on-chain (H-01): a
//!     quorum signing two different content objects for one historical
//!     nonce collides on the same record instead of minting twice. Residual
//!     trust: a malicious quorum can still attest ONE forged content per
//!     emitted nonce - bounded by `next_outbound_nonce`, and mirroring the
//!     daughter's per-nonce single-shot flip (spec 15 instruction D).
//!   - *Recovery*: re-mint exactly canonical `amount` (u128 → u64 checked),
//!     which is the net cross-chain principal after any emission-time fee and
//!     rebate. The target is the canonical associated token account derived from
//!     `sender`, the configured issued-token program, and the pathway's issued-token
//!     mint. `sender` is the original burner from the attested preimage, never a
//!     caller argument. The effective fee
//!     (`gross_amount - amount == fee_output_amount - rebate_amount`) remains
//!     charged and is never re-minted by this instruction.
//!
//! Executability gating - this half DOES move value, so unlike the expiry
//! halves it honors liftable gates (none can lock permanently):
//!   - the global and pathway-asset `pause_bit::MINT`;
//!   - the corridor's directional pause
//!     (OUTBOUND_MESSAGES | REMOTE_REDEEM | DAUGHTER_CONTRACT);
//!   - the canonical active `CROSS_CHAIN_REDEEM` pathway used by emission;
//!   - deliberately no `AssetConfig` lifecycle-mode gate: reclaim restores
//!     principal already burned by an earlier accepted emission, so applying
//!     `FROZEN` here would strand a valid refund. Explicit global and
//!     pathway-asset `MINT` pauses remain the incident controls for re-minting;
//!   - the standard mint-site plumbing shared with every other `cpi_mint_to`
//!     call site: canonical issued-token mint binding and the Token-2022
//!     extension gates (observation is refreshable, so honoring them cannot
//!     lock permanently either).
//!
//! Defense-in-depth (§15.6 retirement note): for `EXPIRY_LAPSED` only, the
//! source re-checks `local_now >= expires_at`. Policy-invalidated retirements
//! are quorum-attested destination facts and have no source-side clock gate.
//! The destination's retirement decision (attested in R) is authoritative for
//! corridor progression.
//!
//! Accounts (fixed):
//!   0  chancery_config             writable  PDA [b"chancery-config"]
//!   1  event_authority             readable  PDA [b"event-authority"]
//!   2  pause_state                 readable  PDA [b"pause-state"]
//!   3  asset_pause_state           readable  PDA [b"asset-pause", pathway.asset_mint]
//!   4  pathway_policy              readable  PDA [b"pathway-policy", pathway_id]
//!   5  remote_domain_policy        readable  PDA [b"remote-domain-policy", chain_kind, domain_id_be]
//!   6  cross_chain_signer_set      readable  PDA [b"cross-chain-signer-set", policy.signer_set_id]
//!   7  remote_nonce                readable  PDA [b"remote-nonce", chain_kind, domain_id_be, default]
//!   8  outbound_reclaim_record     writable  PDA [b"outbound-reclaim", chain_kind, domain_id_be, source_nonce_be]; created here
//!   9  sender_issued_token_account writable  canonical ATA for the attested sender + issued mint/program
//!  10  issued_token_mint           writable  mint-target
//!  11  mint_authority_pda          readable  PDA [b"mint-authority"]
//!  12  issued_token_program        readable
//!  13  issued_token_control        readable  PDA [b"issued-token-control"]
//!  14  payer                       signer, writable  rent for the reclaim record only
//!  15  system_program              readable

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;
use solana_sysvar::Sysvar;

use crate::{
    constants::{
        chain_kind, cross_chain_message_kind, inbound_message_retirement_reason,
        pathway_kind, programs, remote_domain_pause_bit, seeds, SOLANA_SELF_DOMAIN_ID,
    },
    error::ChanceryError,
    modules::{
        control::state::{
            asset_pause_state::AssetPauseState,
            pause_state::{pause_bit, PauseState},
        },
        core::{
            instructions::create_pda_account,
            state::chancery_config::ChanceryConfig,
        },
        cross_chain::{
            attestation::{
                verify_attestation_quorum, AttestationSignature,
                MAX_ATTESTED_INSTRUCTION_DATA_LEN,
            },
            instructions::assert_distinct_cross_chain_account_indexes,
            message_hash::{
                compute_epoch_free_content_hash, MessageHashPreimage, MESSAGE_HASH_CANON_VERSION,
            },
            reclaim_digest::{compute_reclaim_digest, ReclaimDigestPreimage},
            state::{
                cross_chain_signer_set::CrossChainSignerSet,
                outbound_reclaim_record::{
                    OutboundReclaimRecord, OUTBOUND_RECLAIM_RECORD_DISCRIMINATOR,
                    OUTBOUND_RECLAIM_RECORD_SIZE,
                },
                remote_domain_policy::RemoteDomainPolicy,
                remote_nonce::RemoteNonce,
            },
        },
        evidence::emit::{emit_cross_chain_outbound_reclaimed, CrossChainOutboundReclaimed},
        limits::state::usage_window::u128_to_words,
        pathway::state::pathway_policy::PathwayPolicy,
        settlement::instructions::{
            assert_canonical_issued_token_mint, assert_issued_token_extension_gates,
            assert_spl_token_account_binding, cpi_mint_to,
        },
    },
};

// ─── Account indices ──────────────────────────────────────────────────────────

const CHANCERY_CONFIG:             usize = 0;
const EVENT_AUTHORITY:             usize = 1;
const PAUSE_STATE:                 usize = 2;
const ASSET_PAUSE_STATE:           usize = 3;
const PATHWAY_POLICY:              usize = 4;
const REMOTE_DOMAIN_POLICY:        usize = 5;
const SIGNER_SET:                  usize = 6;
const REMOTE_NONCE_ACCOUNT_INDEX:  usize = 7;
const OUTBOUND_RECLAIM_RECORD:     usize = 8;
const SENDER_ISSUED_TOKEN_ACCOUNT: usize = 9;
const ISSUED_TOKEN_MINT:           usize = 10;
const MINT_AUTHORITY_PDA:          usize = 11;
const ISSUED_TOKEN_PROGRAM:        usize = 12;
const ISSUED_TOKEN_CONTROL:        usize = 13;
const PAYER:                       usize = 14;
const SYSTEM_PROGRAM:              usize = 15;
const REQUIRED_ACCOUNT_COUNT:      usize = 16;

const DISTINCT_ACCOUNT_INDEXES: &[usize] = &[
    CHANCERY_CONFIG,
    EVENT_AUTHORITY,
    PAUSE_STATE,
    ASSET_PAUSE_STATE,
    PATHWAY_POLICY,
    REMOTE_DOMAIN_POLICY,
    SIGNER_SET,
    REMOTE_NONCE_ACCOUNT_INDEX,
    OUTBOUND_RECLAIM_RECORD,
    SENDER_ISSUED_TOKEN_ACCOUNT,
    ISSUED_TOKEN_MINT,
    MINT_AUTHORITY_PDA,
    ISSUED_TOKEN_PROGRAM,
    ISSUED_TOKEN_CONTROL,
    PAYER,
    SYSTEM_PROGRAM,
];

// ─── Args ─────────────────────────────────────────────────────────────────────
//
// The full canonical preimage of the reclaimed emission (fields not loadable
// from on-chain state) plus the expiry observation from the attested E2
// envelope. Every economic field is bound by the epoch-free content hash the
// quorum signed via R; nothing here is trusted on the caller's word.
#[derive(BorshDeserialize)]
pub struct ReclaimExpiredOutboundArgs {
    /// Local pathway selector; supplies the source asset / issued-token
    /// preimage fields exactly as `emit_outbound_message` did. The account
    /// must be the canonical, active `CROSS_CHAIN_REDEEM` pathway; this stops
    /// a caller from selecting another pathway with the same mint pair but
    /// weaker local controls.
    pub pathway_id:                       [u8; 32],

    /// Destination remote chain-family discriminant. `chain_kind::SOLANA` is
    /// valid for another SVM deployment; corridor identity is not inferred from
    /// this discriminant alone.
    pub remote_chain_kind:                u8,

    /// Destination remote domain id, BE-encoded into policy/nonce PDA seeds.
    pub remote_domain_id:                 u64,

    // ── Canon scalars ──
    /// Must be an `OUTBOUND_*` burn kind (0x01 or 0x02).
    pub message_kind:                     u8,

    /// The emission being recovered. Plausibility floor:
    /// must be `< RemoteNonce.next_outbound_nonce`.
    pub source_nonce:                     u64,

    /// Canonical net cross-chain principal from the attested preimage (E2
    /// width); narrowed to u64 with a checked conversion before the mint. Any
    /// effective fee assessed on the original gross burn is non-refundable.
    pub amount:                           u128,

    /// The emission's declared expiry. For `EXPIRY_LAPSED`, the source
    /// re-checks that this is nonzero and has elapsed. Policy-invalidated
    /// retirements may carry zero or future expiry as committed in the message.
    pub expires_at_unix_timestamp:        i64,

    // ── Canon 32-byte fields not loadable from on-chain state ──
    pub destination_asset:                [u8; 32],
    pub sender:                           [u8; 32],
    pub recipient:                        [u8; 32],

    // ── Retirement observation (from the attested E2 envelope) ──
    pub retirement_reason:                u8,
    pub expired_at_unix_timestamp:        i64,
    pub expired_at_slot_or_block:         u64,

    // ── Provided hashes (must match recomputed values) ──
    pub provided_epoch_free_content_hash: [u8; 32],
    pub provided_reclaim_digest:          [u8; 32],

    // ── Attestation bundle over R ──
    pub signatures:                       Vec<AttestationSignature>,
}

// ─── Pure gates (unit-tested in cross_chain/tests) ────────────────────────────

/// §15.5: only `OUTBOUND_*` burn kinds are reclaimable by re-mint. The lock
/// kind (`OUTBOUND_LOCK_FOR_REMOTE_RELEASE`) moves value by locking, not
/// burning; re-minting it would fabricate supply on top of the still-locked
/// collateral. Its refund shape is a release, out of scope for this handler.
pub(crate) fn assert_reclaimable_outbound_kind(message_kind: u8) -> Result<(), ChanceryError> {
    match message_kind {
        cross_chain_message_kind::OUTBOUND_REDEEM_TO_REMOTE_MINT
        | cross_chain_message_kind::OUTBOUND_REDEEM_TO_REMOTE_RELEASE => Ok(()),
        _ => Err(ChanceryError::ReclaimMessageKindNotOutboundBurn),
    }
}

/// Validate the reason copied from the authenticated destination retirement
/// event. Only time-based retirement needs a local clock re-check; the policy
/// and execution-state reasons are authoritative quorum statements about the
/// destination at retirement time.
pub(crate) fn assert_reclaim_retirement_condition(
    retirement_reason:             u8,
    expires_at_unix_timestamp:     i64,
    now:                           i64,
) -> Result<(), ChanceryError> {
    match retirement_reason {
        inbound_message_retirement_reason::EXPIRY_LAPSED => {
            if expires_at_unix_timestamp == 0 || now < expires_at_unix_timestamp {
                return Err(ChanceryError::MessageNotExpired);
            }
            Ok(())
        }
        // Destination policy/execution invalidations are authoritative quorum
        // statements. The source cannot reconstruct destination policy or an
        // exact recipient account's state at retirement time; these reasons may
        // carry zero or a future timestamp.
        inbound_message_retirement_reason::EXPIRY_POLICY_TIGHTENED
        | inbound_message_retirement_reason::PER_MESSAGE_CAP_TIGHTENED
        | inbound_message_retirement_reason::PER_DAY_CAP_TIGHTENED
        | inbound_message_retirement_reason::UNSUPPORTED_MESSAGE_KIND
        | inbound_message_retirement_reason::ZERO_AMOUNT
        | inbound_message_retirement_reason::AMOUNT_OUT_OF_RANGE
        | inbound_message_retirement_reason::REMOTE_ASSET_BINDING_INVALID
        | inbound_message_retirement_reason::PATHWAY_PER_TRANSACTION_CAP_TIGHTENED
        | inbound_message_retirement_reason::PATHWAY_PERIOD_CAP_TIGHTENED
        | inbound_message_retirement_reason::RECIPIENT_ACCOUNT_FROZEN
        | inbound_message_retirement_reason::RECIPIENT_ACCOUNT_MISSING_OR_UNINITIALIZED => Ok(()),
        _ => Err(ChanceryError::InvalidInboundRetirementReason),
    }
}

/// §15.5: narrow the attested canonical net principal from u128 into
/// Solana's u64 token domain with a checked conversion.
pub(crate) fn narrow_net_reclaim_amount(amount: u128) -> Result<u64, ChanceryError> {
    u64::try_from(amount).map_err(|_| ChanceryError::ReclaimAmountExceedsTokenDomain)
}

/// §15.5 plausibility floor: Chancery cannot have emitted a nonce it has not
/// yet issued. Strictly-below, because `next_outbound_nonce` is the NEXT
/// nonce to be committed, not the last committed one.
pub(crate) fn assert_source_nonce_emitted(
    source_nonce:        u64,
    next_outbound_nonce: u64,
) -> Result<(), ChanceryError> {
    if source_nonce >= next_outbound_nonce {
        return Err(ChanceryError::ReclaimSourceNonceNotEmitted);
    }

    Ok(())
}

/// Pin the permissionless refund target to the deterministic associated token
/// account for the attested sender, issued-token program, and issued-token mint.
/// Owner/mint validation alone is insufficient because a competing submitter
/// could otherwise select another sender-owned account and consume the permanent
/// one-shot reclaim record before the intended canonical refund is submitted.
pub(crate) fn assert_canonical_sender_issued_token_account(
    sender_token_account: &Pubkey,
    sender:               &Pubkey,
    issued_token_program: &Pubkey,
    issued_token_mint:    &Pubkey,
) -> Result<(), ChanceryError> {
    let (expected_sender_token_account, _) = Pubkey::find_program_address(
        &[
            sender.as_ref(),
            issued_token_program.as_ref(),
            issued_token_mint.as_ref(),
        ],
        &programs::ASSOCIATED_TOKEN_PROGRAM_ID,
    );

    if sender_token_account != &expected_sender_token_account {
        return Err(ChanceryError::AccountKeyMismatch);
    }

    Ok(())
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

    let chancery_config_account_info     = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info     = &accounts[EVENT_AUTHORITY];
    let pause_state_account_info         = &accounts[PAUSE_STATE];
    let asset_pause_state_account_info   = &accounts[ASSET_PAUSE_STATE];
    let pathway_policy_account_info      = &accounts[PATHWAY_POLICY];
    let remote_policy_account_info       = &accounts[REMOTE_DOMAIN_POLICY];
    let signer_set_account_info          = &accounts[SIGNER_SET];
    let remote_nonce_account_info        = &accounts[REMOTE_NONCE_ACCOUNT_INDEX];
    let reclaim_record_account_info      = &accounts[OUTBOUND_RECLAIM_RECORD];
    let sender_token_account_info        = &accounts[SENDER_ISSUED_TOKEN_ACCOUNT];
    let issued_token_mint_account_info   = &accounts[ISSUED_TOKEN_MINT];
    let mint_authority_pda_account_info  = &accounts[MINT_AUTHORITY_PDA];
    let issued_token_program_account_info = &accounts[ISSUED_TOKEN_PROGRAM];
    let issued_token_control_account_info = &accounts[ISSUED_TOKEN_CONTROL];
    let payer_account_info               = &accounts[PAYER];
    let system_program_account_info      = &accounts[SYSTEM_PROGRAM];

    if !payer_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !reclaim_record_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !sender_token_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    if !issued_token_mint_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    let args = ReclaimExpiredOutboundArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    let clock      = Clock::get()?;
    let program_id = crate::id();

    // ── Cheap pure gates first ────────────────────────────────────────────────
    assert_reclaimable_outbound_kind(args.message_kind)?;
    assert_reclaim_retirement_condition(
        args.retirement_reason,
        args.expires_at_unix_timestamp,
        clock.unix_timestamp,
    )?;
    let net_reclaim_amount = narrow_net_reclaim_amount(args.amount)?;

    // ── Chancery config + global pause (executability: this half mints) ──────
    let (chancery_config, chancery_config_verified_bump) = ChanceryConfig::load_verified_with_bump(chancery_config_account_info)?;

    if &chancery_config.issued_token_program != issued_token_program_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }
    if &chancery_config.mint_authority_pda != mint_authority_pda_account_info.key {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }

    let chancery_domain_separator = chancery_config.domain_separator;

    let pause_state_bump = PauseState::verify_pda(pause_state_account_info, &program_id)?;
    let pause_state = PauseState::load_for_verified_pda(pause_state_account_info, pause_state_bump)?;
    pause_state.assert_not_paused_for(pause_bit::MINT, clock.slot)?;

    // ── Pathway: canonical active cross-chain redeem policy ──────────────────
    let pathway_policy_bump = PathwayPolicy::verify_pda(pathway_policy_account_info, &args.pathway_id, &program_id)?;
    let pathway = PathwayPolicy::load_for_verified_pda(
        pathway_policy_account_info,
        &args.pathway_id,
        pathway_policy_bump,
    )?;
    pathway.assert_active()?;
    pathway.assert_kind(pathway_kind::CROSS_CHAIN_REDEEM)?;

    let asset_pause_state_bump = AssetPauseState::verify_pda(
        asset_pause_state_account_info,
        &pathway.asset_mint,
        &program_id,
    )?;
    if !asset_pause_state_account_info.data_is_empty() {
        let asset_pause_state = AssetPauseState::load_for_verified_pda(
            asset_pause_state_account_info,
            &pathway.asset_mint,
            asset_pause_state_bump,
        )?;
        asset_pause_state.assert_not_paused_for(pause_bit::MINT, clock.slot)?;
    }

    // Intentionally do not load AssetConfig or apply its lifecycle mode here.
    // This instruction restores net principal burned by a previously accepted
    // outbound emission; treating FROZEN as a reclaim veto would strand user
    // funds. The explicit global and asset MINT pauses above remain available
    // when incident response must temporarily stop all supply restoration.

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

    // The corridor's directional pause bits for the outbound direction.
    // Liftable (relax paths exist), so honoring them cannot lock permanently,
    // and they preserve any incident freeze on the corridor.
    remote_policy.assert_not_paused_for(
        remote_domain_pause_bit::OUTBOUND_MESSAGES
            | remote_domain_pause_bit::REMOTE_REDEEM
            | remote_domain_pause_bit::ATTESTATION_ACCEPTANCE
            | remote_domain_pause_bit::DAUGHTER_CONTRACT,
    )?;

    // ── Signer set: PDA, active, temporally valid, id matches policy ──────────
    // Reclaim proofs use the source's CURRENT committed root (§15.6): the
    // epoch may differ from the emission epoch and from the daughter's expiry
    // epoch. The epoch-free content hash is the invariant thread.
    let signer_set_bump = CrossChainSignerSet::verify_pda(signer_set_account_info, &remote_policy.signer_set_id, &program_id)?;

    let signer_set = CrossChainSignerSet::load_for_verified_pda(
        signer_set_account_info,
        &remote_policy.signer_set_id,
        signer_set_bump,
    )?;

    signer_set.assert_active()?;
    signer_set.assert_temporally_valid(clock.unix_timestamp)?;
    remote_policy.assert_signer_set_id_matches(&signer_set.signer_set_id)?;

    // ── Plausibility floor: RemoteNonce PDA + source_nonce < next_outbound ────
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
        assert_source_nonce_emitted(args.source_nonce, n.next_outbound_nonce)?;
    }

    // ── Epoch-free content hash: recompute from the full supplied preimage ───
    //    Outbound orientation, exactly as `emit_outbound_message` constructed
    //    it: source = this chancery, destination = the corridor's remote side.
    let source_chancery_bytes = chancery_config_account_info.key.to_bytes();
    let source_asset_bytes    = pathway.asset_mint.to_bytes();
    let source_issued_bytes   = pathway.issued_token_mint.to_bytes();

    let preimage = MessageHashPreimage {
        message_kind:                  args.message_kind,
        source_chain_kind:             chain_kind::SOLANA,
        destination_chain_kind:        remote_policy.remote_chain_kind,
        source_domain_id:              SOLANA_SELF_DOMAIN_ID,
        destination_domain_id:         remote_policy.remote_domain_id,
        source_nonce:                  args.source_nonce,
        amount:                        args.amount,
        expires_at_unix_timestamp:     args.expires_at_unix_timestamp,
        source_chancery_contract:      &source_chancery_bytes,
        destination_chancery_contract: &remote_policy.remote_chancery_contract,
        source_domain_separator:       &chancery_domain_separator,
        destination_domain_separator:  &remote_policy.remote_domain_separator,
        source_asset:                  &source_asset_bytes,
        destination_asset:             &args.destination_asset,
        source_issued_token:           &source_issued_bytes,
        destination_issued_token:      &remote_policy.remote_issued_token,
        sender:                        &args.sender,
        recipient:                     &args.recipient,
        // Ignored by the epoch-free computation; zeroed there (§15.4).
        signer_set_id:                 &remote_policy.signer_set_id,
    };

    let epoch_free_content_hash = compute_epoch_free_content_hash(&preimage);

    if epoch_free_content_hash != args.provided_epoch_free_content_hash {
        return Err(ChanceryError::EpochFreeContentHashMismatch.into());
    }

    // ── Reclaim digest R + quorum (§15.5) ─────────────────────────────────────
    let reclaim_preimage = ReclaimDigestPreimage {
        source_chain_kind:             chain_kind::SOLANA,
        destination_chain_kind:        remote_policy.remote_chain_kind,
        source_domain_id:              SOLANA_SELF_DOMAIN_ID,
        destination_domain_id:         remote_policy.remote_domain_id,
        source_chancery_contract:      &source_chancery_bytes,
        destination_chancery_contract: &remote_policy.remote_chancery_contract,
        source_nonce:                  args.source_nonce,
        epoch_free_content_hash:       &epoch_free_content_hash,
        retirement_reason:             args.retirement_reason,
        expired_at_unix_timestamp:     args.expired_at_unix_timestamp,
        expired_at_slot_or_block:      args.expired_at_slot_or_block,
    };

    let reclaim_digest = compute_reclaim_digest(&reclaim_preimage);

    if reclaim_digest != args.provided_reclaim_digest {
        return Err(ChanceryError::ReclaimDigestMismatch.into());
    }

    verify_attestation_quorum(
        &reclaim_digest,
        &signer_set.signer_root,
        signer_set.threshold,
        signer_set.signer_count,
        remote_policy.minimum_attestation_threshold,
        &args.signatures,
    )?;

    // ── Refund plumbing: canonical mint + sender token binding ────────────────
    // Standard mint-site invariants shared with every `cpi_mint_to` call
    // site. `sender` comes from the attested preimage - never from a caller
    // argument - and the refund account is pinned to that sender's canonical
    // ATA before the permanent one-shot record can be created.
    assert_canonical_issued_token_mint(&chancery_config, &pathway, issued_token_mint_account_info)?;

    assert_issued_token_extension_gates(
        issued_token_control_account_info,
        issued_token_mint_account_info,
        &pathway,
        clock.slot,
        &program_id,
    )?;

    let sender = Pubkey::new_from_array(args.sender);

    assert_spl_token_account_binding(
        sender_token_account_info,
        issued_token_program_account_info,
        &pathway.issued_token_mint,
        &sender,
    )?;

    assert_canonical_sender_issued_token_account(
        sender_token_account_info.key,
        &sender,
        issued_token_program_account_info.key,
        &pathway.issued_token_mint,
    )?;

    let mint_authority_b = chancery_config.resolved_mint_authority_bump(&program_id);
    drop(chancery_config);

    // ── Single-shot boundary: create the permanent reclaim record ─────────────
    // Keyed by EMISSION IDENTITY (corridor + nonce), NOT by content hash:
    // every content claim for this nonce maps to the same PDA, so a quorum
    // signing divergent content objects for one historical emission collides
    // here instead of minting twice (H-01;
    // reclaim_count(corridor, source_nonce) <= 1). The attested content is
    // recorded below for offline audit against the emission event. The
    // record is NEVER closeable - closure would re-enable the reclaim.
    let record_bump = OutboundReclaimRecord::verify_pda(
        reclaim_record_account_info,
        args.remote_chain_kind,
        args.remote_domain_id,
        args.source_nonce,
        &program_id,
    )?;

    if !reclaim_record_account_info.data_is_empty() {
        return Err(ChanceryError::OutboundAlreadyReclaimed.into());
    }

    let remote_domain_id_be = args.remote_domain_id.to_be_bytes();
    let source_nonce_be     = args.source_nonce.to_be_bytes();

    create_pda_account(
        payer_account_info,
        reclaim_record_account_info,
        system_program_account_info,
        &program_id,
        &[
            seeds::OUTBOUND_RECLAIM_RECORD,
            &[args.remote_chain_kind],
            &remote_domain_id_be,
            &source_nonce_be,
            &[record_bump],
        ],
        OUTBOUND_RECLAIM_RECORD_SIZE,
    )?;

    {
        let mut record = OutboundReclaimRecord::load_uninitialized_mut(reclaim_record_account_info)?;

        record.discriminator               = OUTBOUND_RECLAIM_RECORD_DISCRIMINATOR;
        record.version                     = 1;
        record.bump                        = record_bump;
        record.message_kind                = args.message_kind;
        record.remote_chain_kind           = args.remote_chain_kind;
        record.retirement_reason           = args.retirement_reason;
        record._pad0                       = [0u8; 2];
        record.remote_domain_id            = args.remote_domain_id;
        record.source_nonce                = args.source_nonce;
        record.amount_words                = u128_to_words(args.amount);
        record.reclaimed_at_slot           = clock.slot;
        record.reclaimed_at_unix_timestamp = clock.unix_timestamp;
        record.epoch_free_content_hash     = epoch_free_content_hash;
        record.reclaim_digest              = reclaim_digest;
        record.sender                      = args.sender;
        record.attesting_signer_set_id     = remote_policy.signer_set_id;
        record._reserved                   = [0u8; 32];
    }

    // ── Recovery: re-mint canonical net principal; emission fee stays charged ─
    cpi_mint_to(
        issued_token_program_account_info,
        issued_token_mint_account_info,
        sender_token_account_info,
        mint_authority_pda_account_info,
        mint_authority_b,
        net_reclaim_amount,
    )?;

    // ── Bump seq + emit (invariant #9: evidence last) ─────────────────────────
    let mut chancery_config_mut = ChanceryConfig::load_mut_for_verified_pda(chancery_config_account_info, chancery_config_verified_bump)?;
    let event_authority_bump    = chancery_config_mut.event_authority_bump;
    let seq                     = chancery_config_mut.next_sequence_nonce()?;

    emit_cross_chain_outbound_reclaimed(
        event_authority_account_info,
        event_authority_bump,
        CrossChainOutboundReclaimed {
            sequence_nonce:                seq,
            chancery:                      *chancery_config_account_info.key,
            slot:                          clock.slot,
            unix_timestamp:                clock.unix_timestamp,
            epoch_free_content_hash,
            reclaim_digest,
            canon_version:                 MESSAGE_HASH_CANON_VERSION,
            message_kind:                  args.message_kind,
            source_chain_kind:             chain_kind::SOLANA,
            destination_chain_kind:        remote_policy.remote_chain_kind,
            source_domain_id:              SOLANA_SELF_DOMAIN_ID,
            destination_domain_id:         remote_policy.remote_domain_id,
            source_nonce:                  args.source_nonce,
            amount:                        args.amount,
            expires_at_unix_timestamp:     args.expires_at_unix_timestamp,
            retirement_reason:             args.retirement_reason,
            expired_at_unix_timestamp:     args.expired_at_unix_timestamp,
            expired_at_slot_or_block:      args.expired_at_slot_or_block,
            source_chancery_contract:      source_chancery_bytes,
            destination_chancery_contract: remote_policy.remote_chancery_contract,
            source_domain_separator:       chancery_domain_separator,
            destination_domain_separator:  remote_policy.remote_domain_separator,
            source_asset:                  source_asset_bytes,
            destination_asset:             args.destination_asset,
            source_issued_token:           source_issued_bytes,
            destination_issued_token:      remote_policy.remote_issued_token,
            sender:                        args.sender,
            recipient:                     args.recipient,
            signer_set_id:                 remote_policy.signer_set_id,
            pathway_id:                    args.pathway_id,
            reclaim_record:                *reclaim_record_account_info.key,
            sender_token_account:          *sender_token_account_info.key,
            attestation_count:             args.signatures.len() as u8,
        },
    )?;

    Ok(())
}
