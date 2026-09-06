//! Issued-token control & basic freeze/thaw events.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

use crate::modules::evidence::event::{emit_event, ChanceryEvent};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct IssuedTokenControlInitialized {
    pub sequence_nonce:                      u64,
    pub chancery:                            Pubkey,
    pub slot:                                u64,
    pub unix_timestamp:                      i64,

    pub risk_class:                          u8,
    pub issued_token_control:                Pubkey,
    pub reserved_mint_extension_mask:        [u64; 2],
    pub reserved_account_extension_mask:     [u64; 2],
    pub max_extension_observation_age_slots: u64,
    pub initialized_by:                      Pubkey,
}
impl ChanceryEvent for IssuedTokenControlInitialized {
    const NAME: &'static str = "IssuedTokenControlInitialized";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x95, 0xB5, 0x83, 0x9B, 0x5E, 0x0C, 0x34, 0x8C]
    }
}

pub fn emit_issued_token_control_initialized(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            IssuedTokenControlInitialized,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct IssuedTokenDeploymentVerified {
    pub sequence_nonce:          u64,
    pub chancery:                Pubkey,
    pub slot:                    u64,
    pub unix_timestamp:          i64,

    pub risk_class:              u8,
    pub issued_token_control:    Pubkey,
    pub observed_extension_mask: [u64; 2],
    pub control_flags:           u64,
    pub verified_by:             Pubkey,
}
impl ChanceryEvent for IssuedTokenDeploymentVerified {
    const NAME: &'static str = "IssuedTokenDeploymentVerified";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x73, 0xAA, 0x13, 0x7D, 0x3D, 0x29, 0xE4, 0x25]
    }
}

pub fn emit_issued_token_deployment_verified(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            IssuedTokenDeploymentVerified,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

/// Emitted when issued-token control state changes (extension activation,
/// authority change, module activation/deactivation).
///
/// Reserved schema (finding 102004, reviewed and declined): no current binary
/// emits this event because no post-initialization control mutation exists
/// yet. The definition, IDL entry, and generated decoder are retained so a
/// future binary that activates reserved extensions can emit it without an
/// event-schema migration.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct IssuedTokenControlChange {
    pub sequence_nonce:              u64,
    pub chancery:                    Pubkey,
    pub slot:                        u64,
    pub unix_timestamp:              i64,

    /// Active mint extension bitmask at time of change.
    pub issued_token_extension_mask: [u64; 2],
    
    /// Observed collateral extension bitmask at time of change.
    pub collateral_extension_mask:   [u64; 2],
    
    /// Active issued-token module bitmask at time of change.
    pub issued_token_module_mask:    [u64; 2],
    
    /// Required collateral module bitmask at time of change.
    pub collateral_module_mask:      [u64; 2],
    
    /// Current hook program ID (zero if inactive).
    pub hook_program_id:             Pubkey,
    
    /// Current permanent delegate (zero if inactive).
    pub permanent_delegate:          Pubkey,
    
    /// Current metadata address (zero if inactive).
    pub metadata_address:            Pubkey,
    pub token_control_flags:         u64,
    pub issued_token_mint:           Pubkey,
    pub configured_by:               Pubkey,
    
    /// Discriminant indicating what kind of change occurred.
    pub change_kind:                 u64,
}
impl ChanceryEvent for IssuedTokenControlChange {
    const NAME: &'static str = "IssuedTokenControlChange";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xB2, 0xA9, 0xD6, 0x61, 0xC9, 0xDC, 0x36, 0x0D]
    }
}

pub fn emit_issued_token_control_change(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            IssuedTokenControlChange,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct BasicTokenFreeze {
    pub sequence_nonce:       u64,
    pub chancery:             Pubkey,
    pub slot:                 u64,
    pub unix_timestamp:       i64,

    /// Always RestrictiveImmediate.
    pub risk_class:           u8,
    pub basic_freeze_record:  Pubkey,
    pub issued_token_account: Pubkey,
    pub issued_token_mint:    Pubkey,
    pub frozen_by:            Pubkey,
    pub reason_code:          u32,
    
    /// Always 0 for MVP.
    pub status_flags:         u64,
    pub freeze_flags:         u64,
}
impl ChanceryEvent for BasicTokenFreeze {
    const NAME: &'static str = "BasicTokenFreeze";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xD7, 0x0D, 0x23, 0xAF, 0x69, 0x9C, 0x4E, 0xCA]
    }
}

pub fn emit_basic_token_freeze(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            BasicTokenFreeze,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct BasicTokenThaw {
    pub sequence_nonce:       u64,
    pub chancery:             Pubkey,
    pub slot:                 u64,
    pub unix_timestamp:       i64,

    /// Widening when widening; RestrictiveImmediate otherwise.
    pub risk_class:           u8,
    pub basic_freeze_record:  Pubkey,
    pub issued_token_account: Pubkey,
    pub issued_token_mint:    Pubkey,
    pub thawed_by:            Pubkey,
    pub reason_code:          u32,
    pub status_flags:         u64,
    pub thaw_flags:           u64,
}
impl ChanceryEvent for BasicTokenThaw {
    const NAME: &'static str = "BasicTokenThaw";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0x68, 0x47, 0xC1, 0x8C, 0x6C, 0xF7, 0x1E, 0x7D]
    }
}

pub fn emit_basic_token_thaw(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            BasicTokenThaw,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct IssuedTokenExtensionObservationRefreshed {
    pub sequence_nonce:             u64,
    pub chancery:                   Pubkey,
    pub slot:                       u64,
    pub unix_timestamp:             i64,

    pub risk_class:                 u8,
    pub issued_token_control:       Pubkey,
    pub observed_extension_mask:    [u64; 2],
    pub control_flags:              u64,
    pub verification_succeeded:     bool,
    pub regression_error_code:      u32,
    pub refreshed_by:               Pubkey,
}
impl ChanceryEvent for IssuedTokenExtensionObservationRefreshed {
    const NAME: &'static str = "IssuedTokenExtensionObservationRefreshed";
    #[inline]
    fn discriminator() -> [u8; 8] {
        [0xED, 0xF2, 0x2F, 0x26, 0x51, 0xCF, 0xF4, 0x93]
    }
}

pub fn emit_issued_token_extension_observation_refreshed(
    event_authority_account_info: &AccountInfo,
    event_authority_bump:         u8,
    p:                            IssuedTokenExtensionObservationRefreshed,
) -> ProgramResult {
    emit_event(event_authority_account_info, event_authority_bump, &p)
}
