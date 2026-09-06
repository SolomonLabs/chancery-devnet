//! Durable event emission via signed self-CPI to `events_cpi::EMIT`.
//!
//! Wire format on-chain (CPI instruction data):
//!   `[ EVENTS_CPI(1) | EMIT(1) | discriminator(8) | borsh_payload(...) ]`
//!
//! Signer: the `event_authority` PDA, derived as
//!   `find_program_address(&[seeds::EVENT_AUTHORITY], &chancery::id())`
//!
//! The `events_cpi::EMIT` handler validates that account[0] is this PDA AND
//! is a signer. Only the chancery program itself can produce that signature,
//! via `invoke_signed` with the cached bump. External transactions cannot
//! forge it.
//!
//! Bump source: cached on `ChanceryConfig.event_authority_bump`. Handlers
//! load `ChanceryConfig` for other reasons anyway (sequence_nonce, pause checks),
//! so reading the bump is free.

use borsh::BorshSerialize;
use solana_account_info::AccountInfo;
use solana_cpi::invoke_signed;
use solana_instruction::{AccountMeta, Instruction};
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;
use solana_sha256_hasher::hashv;

use crate::{
    constants::{ix::events_cpi as events_cpi_ix, module, seeds},
    error::ChanceryError,
};

// ─── PDA helper ───────────────────────────────────────────────────────────────

/// Derive the event authority PDA. Used at chancery initialization time
/// (to cache the bump on `ChanceryConfig`) and by off-chain clients that
/// need to verify event provenance.
pub fn event_authority_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[seeds::EVENT_AUTHORITY], program_id)
}

// ─── Event trait ──────────────────────────────────────────────────────────────

/// Implemented by every event payload. Built-in events override the default
/// discriminator with the precomputed first 8 bytes of `sha256("event:<EventName>")`.
pub trait ChanceryEvent: BorshSerialize {
    /// Stable on-chain name. Off-chain decoders key on this via the IDL.
    const NAME: &'static str;

    /// First 8 bytes of `sha256("event:" + Self::NAME)`.
    ///
    /// The default preserves source compatibility for downstream event types;
    /// Chancery's built-in event implementations embed this value directly.
    fn discriminator() -> [u8; 8] {
        let h = hashv(&[b"event:", Self::NAME.as_bytes()]);
        let bytes = h.to_bytes();
        [
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
        ]
    }
}

// ─── Emission helper ──────────────────────────────────────────────────────────

/// Serialise an event with its discriminator and emit via signed self-CPI to
/// `events_cpi::EMIT`. Returns CPI errors verbatim so handlers can `?`.
///
/// Parameters:
///   - `event_authority_account_info`: AccountInfo of the event authority PDA, passed
///     in by the caller (typically `&accounts[EVENT_AUTHORITY]`).
///   - `event_authority_bump`: cached bump from `ChanceryConfig`. Caller
///     reads it via `chancery_config.event_authority_bump`.
///   - `event`: the payload struct (any type implementing `ChanceryEvent`).
///
/// Call as the final step of every handler, after all other CPIs have
/// succeeded and after the sequence_nonce has been incremented (invariant #9).
/// A later outer transaction instruction can still fail and roll back this CPI;
/// off-chain decoders must therefore require successful transaction metadata.
pub fn emit_event<E: ChanceryEvent>(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    event:                        &E,
) -> Result<(), ProgramError> {
    let mut data: Vec<u8> = Vec::with_capacity(2 + 8 + 128);

    data.push(module::EVENTS_CPI);
    data.push(events_cpi_ix::EMIT);
    data.extend_from_slice(&E::discriminator());

    event
        .serialize(&mut data)
        .map_err(|_| ChanceryError::EventSerializationFailed)?;

    let ix = Instruction {
        program_id: crate::id(),
        accounts:   vec![AccountMeta::new_readonly(*event_authority_account_info.key, true)],
        data,
    };

    invoke_signed(
        &ix,
        &[event_authority_account_info.clone()],
        &[&[seeds::EVENT_AUTHORITY, &[event_authority_bump]]],
    )
}
