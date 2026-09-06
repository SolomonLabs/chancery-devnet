//! Pause state change events (global, asset, pathway, executor, counterparty).

use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::modules::evidence::event::{emit_event, ChanceryEvent};

/// Emitted on any pause state change.
///
/// Carries enough to reconstruct effective pause state from the event stream
/// alone, at any slot, without reading accounts:
///   `previous_pause_bits` -> `changed_pause_bits` -> `effective_pause_bits`,
/// plus the resulting `expires_at_slot` (0 = no auto-expiry) and the `slot` the
/// transition landed on.
///
/// `acting_authority` and `activated_by` are deliberately separate. An
/// emergency addition to an already-active pause preserves the original
/// episode's `activated_by`, so without `acting_authority` the signed payload
/// would misattribute the transition to whoever opened the episode.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PauseStateChange {
    pub sequence_nonce:       u64,
    pub chancery:             Pubkey,
    pub slot:                 u64,
    pub unix_timestamp:       i64,

    /// Stored bits before this transition.
    pub previous_pause_bits:  u64,

    /// Bits that actually changed in this transition (`previous ^ effective`).
    /// A repeated no-op restriction therefore records zero rather than claiming
    /// that already-active bits changed again.
    pub changed_pause_bits:   u64,

    /// Stored bits after this transition.
    pub effective_pause_bits: u64,

    /// False = set (add bits), true = clear (remove bits).
    pub is_clear:             bool,

    /// Resulting stored auto-expiry slot; 0 = no auto-expiry. Consumers need
    /// this to know when a stored non-zero bitfield stopped being effective.
    pub expires_at_slot:      u64,

    pub reason_code:          u32,

    /// The signer that executed THIS transition.
    pub acting_authority:     Pubkey,

    /// The authority stored as the opener of the current pause episode where
    /// the underlying state carries that metadata (global and asset scopes).
    /// Stateless scopes emit `Pubkey::default()`; their opener is reconstructed
    /// from the first 0 -> non-zero event and its `acting_authority`. Also
    /// defaults once a stored episode is fully cleared.
    pub activated_by:         Pubkey,

    /// One of `scope::*` constants.
    pub scope_kind:           u8,
    
    /// Key of the paused entity; `Pubkey::default()` for global.
    pub scope_key:            Pubkey,

    /// Permission-record subject for executor/counterparty pause surfaces.
    /// Other pause scopes emit `Pubkey::default()` because `scope_key` alone
    /// identifies their canonical state row.
    pub subject:              Pubkey,
}
impl ChanceryEvent for PauseStateChange {
    const NAME: &'static str = "PauseStateChange";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xA3, 0x72, 0x2D, 0x5A, 0x6E, 0xD9, 0x4A, 0xF0]
    }
}

pub fn emit_pause_state_change(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            PauseStateChange,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}
