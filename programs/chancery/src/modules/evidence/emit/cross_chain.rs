//! Cross-chain events: remote-domain policy lifecycle, signer-set
//! lifecycle, and outbound/inbound message emission.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::modules::evidence::event::{emit_event, ChanceryEvent};

// ─── Remote-domain policy ───────────────────────────────────────────────────

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct RemoteDomainPolicyRegistered {
    pub sequence_nonce:       u64,
    pub chancery:             Pubkey,
    pub slot:                 u64,
    pub unix_timestamp:       i64,

    pub risk_class:           u8,
    pub remote_domain_policy: Pubkey,
    pub remote_chain_kind:    u8,
    pub remote_domain_id:     u64,
    pub mode:                 u8,
    pub pause_bits:           u64,
    pub signer_set_id:        [u8; 32],
    pub registered_by:        Pubkey,
}
impl ChanceryEvent for RemoteDomainPolicyRegistered {
    const NAME: &'static str = "RemoteDomainPolicyRegistered";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x3D, 0xD0, 0xEB, 0x87, 0xDD, 0xB5, 0xDE, 0xFF]
    }
}

pub fn emit_remote_domain_policy_registered(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            RemoteDomainPolicyRegistered,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct RemoteDomainPolicyUpdated {
    pub sequence_nonce:       u64,
    pub chancery:             Pubkey,
    pub slot:                 u64,
    pub unix_timestamp:       i64,

    pub risk_class:           u8,
    pub change_id:            [u8; 32],
    pub remote_domain_policy: Pubkey,
    pub remote_chain_kind:    u8,
    pub remote_domain_id:     u64,
    pub signer_set_id:        [u8; 32],
    pub old_value_hash:       [u8; 32],
    pub new_value_hash:       [u8; 32],
    pub change_mask:          u64,
    pub proposed_by:          Pubkey,
    pub updated_by:           Pubkey,
}
impl ChanceryEvent for RemoteDomainPolicyUpdated {
    const NAME: &'static str = "RemoteDomainPolicyUpdated";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x46, 0x21, 0x00, 0xD5, 0x32, 0xBF, 0x53, 0x65]
    }
}

pub fn emit_remote_domain_policy_updated(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            RemoteDomainPolicyUpdated,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct RemoteDomainPauseRelaxed {
    pub sequence_nonce:       u64,
    pub chancery:             Pubkey,
    pub slot:                 u64,
    pub unix_timestamp:       i64,

    pub risk_class:           u8,
    pub change_id:            [u8; 32],
    pub remote_domain_policy: Pubkey,
    pub old_pause_bits:       u64,
    pub new_pause_bits:       u64,
    pub reason_code:          u32,
    pub old_value_hash:       [u8; 32],
    pub new_value_hash:       [u8; 32],
    pub proposed_by:          Pubkey,
    pub relaxed_by:           Pubkey,
}
impl ChanceryEvent for RemoteDomainPauseRelaxed {
    const NAME: &'static str = "RemoteDomainPauseRelaxed";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x58, 0xEA, 0x74, 0xD2, 0xC0, 0x0C, 0x75, 0x7A]
    }
}

pub fn emit_remote_domain_pause_relaxed(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            RemoteDomainPauseRelaxed,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

// ─── Cross-chain signer set ─────────────────────────────────────────────────

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct CrossChainSignerSetRegistered {
    pub sequence_nonce:             u64,
    pub chancery:                   Pubkey,
    pub slot:                       u64,
    pub unix_timestamp:             i64,

    pub risk_class:                 u8,
    pub change_id:                  [u8; 32],
    pub cross_chain_signer_set:     Pubkey,
    pub signer_set_id:              [u8; 32],
    pub signer_root:                [u8; 32],
    pub signer_count:               u8,
    pub threshold:                  u8,
    pub valid_after_unix_timestamp: i64,
    pub expires_at_unix_timestamp:  i64,
    pub old_value_hash:             [u8; 32],
    pub new_value_hash:             [u8; 32],
    pub proposed_by:                Pubkey,
    pub registered_by:              Pubkey,
}
impl ChanceryEvent for CrossChainSignerSetRegistered {
    const NAME: &'static str = "CrossChainSignerSetRegistered";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x7E, 0x20, 0x54, 0xC6, 0x4C, 0x64, 0xE8, 0x4E]
    }
}

pub fn emit_cross_chain_signer_set_registered(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            CrossChainSignerSetRegistered,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct CrossChainSignerSetRotated {
    pub sequence_nonce:         u64,
    pub chancery:               Pubkey,
    pub slot:                   u64,
    pub unix_timestamp:         i64,

    pub cross_chain_signer_set: Pubkey,
    pub signer_set_id:          [u8; 32],
    pub reason_code:            u16,
    pub rotated_by:             Pubkey,
}
impl ChanceryEvent for CrossChainSignerSetRotated {
    const NAME: &'static str = "CrossChainSignerSetRotated";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x20, 0x33, 0x5E, 0x02, 0xC4, 0x60, 0xE8, 0xD7]
    }
}

pub fn emit_cross_chain_signer_set_rotated(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            CrossChainSignerSetRotated,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

// ─── Outbound / inbound message emission ────────────────────────────────────

/// Emitted by `emit_outbound_message` after the issued tokens have been
/// burned and the outbound nonce bumped (spec 10 §10.9). Carries the FULL
/// canonical preimage so off-chain attestors can recompute `message_hash`
/// byte-for-byte without making additional account fetches, then sign the
/// hash for the daughter contract.
///
/// Field order matches `MessageHashPreimage` declaration order, with the
/// hash itself and the local-correlation fields appended at the end. Local
/// fields (`pathway_id`, `burned_by`) are NOT in the canonical hash - they
/// exist for indexing and audit only.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct CrossChainOutboundEmitted {
    // ── Header ──
    pub sequence_nonce: u64,
    pub chancery:       Pubkey,
    pub slot:           u64,
    pub unix_timestamp: i64,

    // ── Hash + canon-version dimension ──
    pub message_hash : [u8; 32],
    pub canon_version: u16,

    // ── Canon scalars ──
    pub message_kind:                  u8,
    pub source_chain_kind:             u8,
    pub destination_chain_kind:        u8,
    pub source_domain_id:              u64,
    pub destination_domain_id:         u64,
    pub source_nonce:                  u64,
    pub amount:                        u128,
    pub expires_at_unix_timestamp:     i64,

    // ── Canon 32-byte fields ──
    pub source_chancery_contract:      [u8; 32],
    pub destination_chancery_contract: [u8; 32],
    pub source_domain_separator:       [u8; 32],
    pub destination_domain_separator:  [u8; 32],
    pub source_asset:                  [u8; 32],
    pub destination_asset:             [u8; 32],
    pub source_issued_token:           [u8; 32],
    pub destination_issued_token:      [u8; 32],
    pub sender:                        [u8; 32],
    pub recipient:                     [u8; 32],
    pub signer_set_id:                 [u8; 32],

    // ── Local correlation (NOT in canon hash) ──
    pub pathway_id:                    [u8; 32],
    pub burned_by:                     Pubkey,

    // ── Fee-policy outbound accounting (NOT in canon hash) ──
    /// Gross amount the principal actually burned on Solana.
    /// `gross_amount == amount + fee_output_amount - rebate_amount`.
    pub gross_amount:                  u128,
    
    /// Pre-rebate fee assessed by the pathway fee policy. The effective fee is
    /// `fee_output_amount - rebate_amount`; it is either minted to the approved
    /// fee recipient or retained as local supply reduction. `0` without a fee.
    pub fee_output_amount:             u128,

    /// Rebate forgiven from the assessed fee.
    /// `gross_amount - (fee_output_amount - rebate_amount) == amount`.
    /// `0` when no fee policy was applied or rebate inactive.
    pub rebate_amount:                 u128,

    /// `pathway.fee_policy_id` when applied; all-zero when not.
    pub fee_policy_id:                 [u8; 32],
}
impl ChanceryEvent for CrossChainOutboundEmitted {
    const NAME: &'static str = "CrossChainOutboundEmitted";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xF3, 0xDB, 0x6C, 0x2D, 0xBF, 0xAC, 0xFE, 0x52]
    }
}

pub fn emit_cross_chain_outbound_emitted(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            CrossChainOutboundEmitted,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

/// Emitted by `consume_inbound_message` after the attestation quorum has
/// been verified, the message hash has been consumed (replay protection),
/// the inbound nonce has been bumped, and issued tokens have been minted
/// to the recipient (spec 10 §10.8). Carries the FULL canonical preimage -
/// symmetric with `CrossChainOutboundEmitted` - plus local correlation
/// fields binding the on-chain mint to its remote-side counterpart.
///
/// `attestation_count` records how many distinct signers attested at
/// consumption time. `consumed_by` is the relayer who submitted the tx.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct CrossChainInboundConsumed {
    // ── Header ──
    pub sequence_nonce:                u64,
    pub chancery:                      Pubkey,
    pub slot:                          u64,
    pub unix_timestamp:                i64,

    // ── Hash + canon-version dimension ──
    pub message_hash :                 [u8; 32],
    pub canon_version:                 u16,

    // ── Canon scalars ──
    pub message_kind:                  u8,
    pub source_chain_kind:             u8,
    pub destination_chain_kind:        u8,
    pub source_domain_id:              u64,
    pub destination_domain_id:         u64,
    pub source_nonce:                  u64,
    pub amount:                        u128,
    pub expires_at_unix_timestamp:     i64,

    // ── Canon 32-byte fields ──
    pub source_chancery_contract:      [u8; 32],
    pub destination_chancery_contract: [u8; 32],
    pub source_domain_separator:       [u8; 32],
    pub destination_domain_separator:  [u8; 32],
    pub source_asset:                  [u8; 32],
    pub destination_asset:             [u8; 32],
    pub source_issued_token:           [u8; 32],
    pub destination_issued_token:      [u8; 32],
    pub sender:                        [u8; 32],
    pub recipient:                     [u8; 32],
    pub signer_set_id:                 [u8; 32],

    // ── Local correlation (NOT in canon hash) ──
    pub pathway_id:                    [u8; 32],
    pub recipient_token_account:       Pubkey,
    pub consumed_by:                   Pubkey,
    
    /// Number of distinct attestor signatures verified.
    pub attestation_count:             u8,
}
impl ChanceryEvent for CrossChainInboundConsumed {
    const NAME: &'static str = "CrossChainInboundConsumed";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x15, 0xC7, 0xC2, 0xCD, 0x1C, 0xC7, 0xBE, 0xEE]
    }
}

pub fn emit_cross_chain_inbound_consumed(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            CrossChainInboundConsumed,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

/// An authentic inbound message was permanently unexecutable at the head of
/// its corridor and was retired by permissionless authenticated advancement
/// (`expire_inbound_message`): the full attestation quorum was verified over
/// the recomputed canonical hash, strict nonce equality held, a terminal
/// expiry/policy condition held, the inbound nonce was bumped, and **nothing
/// was minted**. Carries the FULL canonical preimage - symmetric with
/// `CrossChainInboundConsumed` - and is the canonical trigger for the source
/// contract's reclaim path (spec 11 §6.6): the source refunds its lock/burn
/// against this record.
///
/// No actor field: the instruction is permissionless and the transaction
/// signature identifies the submitter off-chain. `attestation_count` records
/// how many distinct signers attested the terminally retired message at retirement time.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct CrossChainInboundExpired {
    // ── Header ──
    pub sequence_nonce:                u64,
    pub chancery:                      Pubkey,
    pub slot:                          u64,
    pub unix_timestamp:                i64,

    // ── Hash + canon-version dimension ──
    pub message_hash :                 [u8; 32],

    /// Spec 15 §15.4: the canonical message hash with `signer_set_id` fixed
    /// to 32 zero bytes - the rotation-stable identity of the emission. The
    /// source contract's reclaim verifier binds to THIS hash, not the
    /// epoch-dependent `message_hash`.
    pub epoch_free_content_hash:       [u8; 32],
    pub canon_version:                 u16,

    // ── Canon scalars ──
    pub message_kind:                  u8,
    pub source_chain_kind:             u8,
    pub destination_chain_kind:        u8,
    pub source_domain_id:              u64,
    pub destination_domain_id:         u64,
    pub source_nonce:                  u64,
    pub amount:                        u128,
    pub expires_at_unix_timestamp:     i64,

    // ── Canon 32-byte fields ──
    pub source_chancery_contract:      [u8; 32],
    pub destination_chancery_contract: [u8; 32],
    pub source_domain_separator:       [u8; 32],
    pub destination_domain_separator:  [u8; 32],
    pub source_asset:                  [u8; 32],
    pub destination_asset:             [u8; 32],
    pub source_issued_token:           [u8; 32],
    pub destination_issued_token:      [u8; 32],
    pub sender:                        [u8; 32],
    pub recipient:                     [u8; 32],

    /// Which epoch's quorum proved the expiry (§15.3 `attesting_signer_set_id`).
    pub signer_set_id:                 [u8; 32],

    // ── Local correlation (NOT in canon hash) ──
    pub pathway_id:                    [u8; 32],

    /// One of `inbound_message_retirement_reason::*`. Distinguishes elapsed
    /// time from a later policy tightening that made the message impossible to
    /// consume without weakening the corridor policy.
    pub retirement_reason:             u8,

    /// Number of distinct attestor signatures verified over the retired message.
    pub attestation_count:             u8,
}
impl ChanceryEvent for CrossChainInboundExpired {
    const NAME: &'static str = "CrossChainInboundExpired";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x53, 0x51, 0x66, 0x47, 0x2D, 0x4C, 0x3B, 0xAC]
    }
}

pub fn emit_cross_chain_inbound_expired(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            CrossChainInboundExpired,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

/// A burned outbound emission was recovered after daughter-side terminal retirement
/// (`reclaim_expired_outbound`, spec 15 §15.5 instruction B): the full
/// attestation quorum was verified over the reclaim digest R against the
/// current corridor signer set, the epoch-free content hash was recomputed
/// from the supplied preimage, the permanent single-shot
/// `OutboundReclaimRecord` was created, and exactly canonical `amount` issued
/// tokens were re-minted to the original `sender`. For fee-bearing emissions,
/// canonical `amount` is the net cross-chain principal; the effective fee
/// assessed against the original gross burn remains non-refundable. Carries
/// the FULL canonical preimage plus the retirement observation so the record is
/// auditable offline against the daughter's terminal-retirement `InboundMessageExpired` (E2) event.
///
/// No actor field: the instruction is permissionless (`payer` funds rent
/// only) and the transaction signature identifies the submitter off-chain.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct CrossChainOutboundReclaimed {
    // ── Header ──
    pub sequence_nonce:                u64,
    pub chancery:                      Pubkey,
    pub slot:                          u64,
    pub unix_timestamp:                i64,

    // ── Hash + canon-version dimension ──
    /// Spec 15 §15.4 rotation-stable content binding retained for audit.
    /// The reclaim-record PDA is keyed by corridor identity plus source nonce,
    /// not by this hash, so divergent content claims collide on one guard.
    pub epoch_free_content_hash:       [u8; 32],

    /// The reclaim digest R actually proven (spec 15 §15.5).
    pub reclaim_digest:                [u8; 32],
    pub canon_version:                 u16,

    // ── Canon scalars ──
    pub message_kind:                  u8,
    pub source_chain_kind:             u8,
    pub destination_chain_kind:        u8,
    pub source_domain_id:              u64,
    pub destination_domain_id:         u64,
    pub source_nonce:                  u64,
    pub amount:                        u128,
    pub expires_at_unix_timestamp:     i64,

    // ── Retirement observation (from the attested E2 envelope) ──
    pub retirement_reason:             u8,
    pub expired_at_unix_timestamp:     i64,
    pub expired_at_slot_or_block:      u64,

    // ── Canon 32-byte fields ──
    pub source_chancery_contract:      [u8; 32],
    pub destination_chancery_contract: [u8; 32],
    pub source_domain_separator:       [u8; 32],
    pub destination_domain_separator:  [u8; 32],
    pub source_asset:                  [u8; 32],
    pub destination_asset:             [u8; 32],
    pub source_issued_token:           [u8; 32],
    pub destination_issued_token:      [u8; 32],

    /// Net-principal recovery recipient identity from the attested preimage.
    pub sender:                        [u8; 32],

    /// Audit completeness: the never-credited destination party.
    pub recipient:                     [u8; 32],

    /// Which epoch's quorum proved the reclaim.
    pub signer_set_id:                 [u8; 32],

    // ── Local correlation (NOT in canon hash) ──
    pub pathway_id:                    [u8; 32],
    pub reclaim_record:                Pubkey,
    pub sender_token_account:          Pubkey,

    /// Number of distinct attestor signatures verified over R.
    pub attestation_count:             u8,
}
impl ChanceryEvent for CrossChainOutboundReclaimed {
    const NAME: &'static str = "CrossChainOutboundReclaimed";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xC8, 0x1D, 0xA5, 0x8B, 0x68, 0xA6, 0xC1, 0x32]
    }
}

pub fn emit_cross_chain_outbound_reclaimed(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            CrossChainOutboundReclaimed,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}
