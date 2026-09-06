use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::seeds,
    error::ChanceryError,
};

// ─── Discriminator ────────────────────────────────────────────────────────────
pub const EVIDENCE_POLICY_DISCRIMINATOR: [u8; 8] =
    [0x65, 0x76, 0x64, 0x70, 0x6f, 0x6c, 0x79, 0x00];

// ─── Account size ─────────────────────────────────────────────────────────────
//   [u8;8]   discriminator                  =  8 @ 0
//   u16      version                        =  2 @ 8
//   u8       bump                           =  1 @ 10
//   u8       allow_freeform                 =  1 @ 11  (bool as u8)
//   [u8;4]   _pad0                          =  4 @ 12  -> [u8;32] at 16
//   [u8;32]  evidence_policy_id             = 32 @ 16
//   [u64;2]  required_field_mask            = 16 @ 48  (48%8=0 Y)
//   [u8;32]  counterparty_schema_hash       = 32 @ 64
//   u16      maximum_freeform_field_count   =  2 @ 96
//   u16      maximum_freeform_value_bytes   =  2 @ 98
//   [u8;4]   _pad1                          =  4 @ 100 -> u64 at 104
//   u64      retention_flags                =  8 @ 104
//   [u8;32]  _reserved                      = 32 @ 112
//                                           ─────
//                                            144 bytes
pub const EVIDENCE_POLICY_SIZE: usize = 144;

// ─── Required field mask bits ─────────────────────────────────────────────────
/// Bit positions for required_field_mask (low word = bits 0–63).
pub mod required_field {
    // ── Settlement evidence (bits 0-19) ──
    pub const PATHWAY_ID:            u64 = 1 << 0;
    pub const SETTLEMENT_MODE:       u64 = 1 << 1;
    pub const INTENT_ID:             u64 = 1 << 2;
    pub const PRINCIPAL_A:           u64 = 1 << 3;
    pub const PRINCIPAL_B:           u64 = 1 << 4;
    pub const EXECUTOR:              u64 = 1 << 5;
    pub const ASSET_MINT:            u64 = 1 << 6;
    pub const ISSUED_TOKEN_MINT:     u64 = 1 << 7;
    pub const SOURCE_ACCOUNT:        u64 = 1 << 8;
    pub const DESTINATION_ACCOUNT:   u64 = 1 << 9;
    pub const GROSS_AMOUNT_IN:       u64 = 1 << 10;
    pub const GROSS_AMOUNT_OUT:      u64 = 1 << 11;
    pub const FEE_AMOUNT:            u64 = 1 << 12;
    pub const REBATE_AMOUNT:         u64 = 1 << 13;
    pub const NET_AMOUNT:            u64 = 1 << 14;
    pub const FEE_POLICY_ID:         u64 = 1 << 15;
    pub const LIMIT_POLICY_ID:       u64 = 1 << 16;
    pub const RESERVE_COMPARTMENT:   u64 = 1 << 17;
    pub const INSURANCE_POLICY_ID:   u64 = 1 << 18;
    pub const SCHEMA_HASH:           u64 = 1 << 19;

    // ── Cross-chain extensions (bits 20-31) ──
    pub const MESSAGE_HASH:          u64 = 1 << 20;
    pub const SOURCE_DOMAIN_ID:      u64 = 1 << 21;
    pub const DESTINATION_DOMAIN_ID: u64 = 1 << 22;
    pub const SOURCE_NONCE:          u64 = 1 << 23;
    pub const SIGNER_SET_ID:         u64 = 1 << 24;
    pub const ATTESTATION_COUNT:     u64 = 1 << 25;
    pub const REMOTE_RECIPIENT:      u64 = 1 << 26;
    pub const REMOTE_SENDER:         u64 = 1 << 27;
    pub const MESSAGE_KIND:          u64 = 1 << 28;
}

// ─── State ────────────────────────────────────────────────────────────────────

/// Defines which evidence fields must be emitted on settlement.
/// Seed: [b"evidence-policy", evidence_policy_id]
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct EvidencePolicy {
    pub discriminator:                      [u8; 8],
    pub version:                            u16,
    pub bump:                               u8,
    
    /// Non-zero if counterparties may submit freeform reporting fields.
    pub allow_freeform_counterparty_fields: u8,
    pub _pad0:                              [u8; 4],

    pub evidence_policy_id:                 [u8; 32],

    /// Bitmask of evidence fields required for compliance. Uses `required_field::*`
    /// bits. Low word covers fields 0-63; high word covers 64-127.
    pub required_field_mask:                [u64; 2],

    /// SHA-256 hash of the counterparty reporting JSON schema.
    /// Zero if no schema is enforced.
    pub counterparty_reporting_schema_hash: [u8; 32],

    /// Maximum number of freeform counterparty fields permitted per event.
    pub maximum_freeform_field_count:       u16,
    
    /// Maximum byte length of each freeform field value hash.
    pub maximum_freeform_value_bytes:       u16,
    pub _pad1:                              [u8; 4],

    /// Retention / visibility flags (reserved for future use).
    pub retention_flags:                    u64,

    pub _reserved:                          [u8; 32],
}

// ─── Helpers ──────────────────────────────────────────────────────────────────
impl EvidencePolicy {
    /// Canonical semantic payload used by pending-change hashes. Layout padding
    /// and append-only reserved bytes are deliberately excluded.
    pub(crate) fn config_change_payload(&self) -> Vec<u8> {
        let mut payload = Vec::with_capacity(93);
        payload.extend_from_slice(&self.evidence_policy_id);
        payload.extend_from_slice(&self.required_field_mask[0].to_le_bytes());
        payload.extend_from_slice(&self.required_field_mask[1].to_le_bytes());
        payload.extend_from_slice(&self.counterparty_reporting_schema_hash);
        payload.push(self.allow_freeform_counterparty_fields);
        payload.extend_from_slice(&self.maximum_freeform_field_count.to_le_bytes());
        payload.extend_from_slice(&self.maximum_freeform_value_bytes.to_le_bytes());
        payload.extend_from_slice(&self.retention_flags.to_le_bytes());
        payload
    }

    pub fn pda(
        evidence_policy_id: &[u8; 32],
        program_id:         &Pubkey,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[seeds::EVIDENCE_POLICY, evidence_policy_id.as_ref()],
            program_id,
        )
    }

    pub fn verify_pda(
        account:            &AccountInfo,
        evidence_policy_id: &[u8; 32],
        program_id:         &Pubkey,
    ) -> Result<u8, ProgramError> {
        let (expected, bump) = Self::pda(evidence_policy_id, program_id);

        if account.key != &expected {
            return Err(ChanceryError::InvalidPda.into());
        }

        Ok(bump)
    }

    pub fn load<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = crate::state_loader::load_state::<Self>(
            account,
            EVIDENCE_POLICY_SIZE,
            EVIDENCE_POLICY_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    pub(crate) fn load_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        evidence_policy_id: &[u8; 32],
        expected_bump: u8,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;

        if state.evidence_policy_id != *evidence_policy_id {
            return Err(ChanceryError::InvalidPda.into());
        }

        crate::state_loader::assert_stored_bump_matches(state.bump, expected_bump)?;

        Ok(state)
    }

    /// Load initialized state and prove that its identity fields resolve to
    /// this account's canonical PDA and stored bump.
    pub fn load_verified<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;
        let expected_bump = Self::verify_pda(account, &state.evidence_policy_id, &crate::id())?;

        if state.bump != expected_bump {
            return Err(ChanceryError::StoredBumpMismatch.into());
        }

        Ok(state)
    }

    pub fn load_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = crate::state_loader::load_state_mut::<Self>(
            account,
            EVIDENCE_POLICY_SIZE,
            EVIDENCE_POLICY_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    pub(crate) fn load_mut_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        evidence_policy_id: &[u8; 32],
        expected_bump: u8,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;

        if state.evidence_policy_id != *evidence_policy_id {
            return Err(ChanceryError::InvalidPda.into());
        }

        crate::state_loader::assert_stored_bump_matches(state.bump, expected_bump)?;

        Ok(state)
    }

    /// Mutable counterpart to `load_verified`.
    pub fn load_verified_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;
        let expected_bump = Self::verify_pda(account, &state.evidence_policy_id, &crate::id())?;

        if state.bump != expected_bump {
            return Err(ChanceryError::StoredBumpMismatch.into());
        }

        Ok(state)
    }

    pub fn load_uninitialized_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        crate::state_loader::load_uninitialized_state_mut::<Self>(
            account,
            EVIDENCE_POLICY_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    // ── Checks ────────────────────────────────────────────────────────────────

    /// Returns true if the given field bit is required in the selected mask
    /// word. Out-of-range word indexes fail closed as not required.
    #[inline]
    pub fn requires_field(&self, word_index: usize, field_bit: u64) -> bool {
        self.required_field_mask
            .get(word_index)
            .map(|word| word & field_bit != 0)
            .unwrap_or(false)
    }

    #[inline]
    pub fn freeform_allowed(&self) -> bool {
        self.allow_freeform_counterparty_fields != 0
    }

    /// Fields currently present on both SettlementMint and SettlementRedeem.
    /// SCHEMA_HASH and all high-word fields are deliberately excluded until the
    /// event payload contains a verifiable schema binding.
    pub const SETTLEMENT_EVENT_SUPPORTED_FIELD_MASK: u64 =
        required_field::PATHWAY_ID
        | required_field::SETTLEMENT_MODE
        | required_field::INTENT_ID
        | required_field::PRINCIPAL_A
        | required_field::PRINCIPAL_B
        | required_field::EXECUTOR
        | required_field::ASSET_MINT
        | required_field::ISSUED_TOKEN_MINT
        | required_field::SOURCE_ACCOUNT
        | required_field::DESTINATION_ACCOUNT
        | required_field::GROSS_AMOUNT_IN
        | required_field::GROSS_AMOUNT_OUT
        | required_field::FEE_AMOUNT
        | required_field::REBATE_AMOUNT
        | required_field::NET_AMOUNT
        | required_field::FEE_POLICY_ID
        | required_field::LIMIT_POLICY_ID
        | required_field::RESERVE_COMPARTMENT
        | required_field::INSURANCE_POLICY_ID;

    /// Reject policy options that are represented in state but cannot yet be
    /// proven by the current event payload. This makes policy registration and
    /// updates fail closed rather than silently storing inert requirements.
    pub fn assert_runtime_supported(&self) -> Result<(), ProgramError> {
        if self.required_field_mask[1] != 0
            || self.required_field_mask[0] & !Self::SETTLEMENT_EVENT_SUPPORTED_FIELD_MASK != 0
        {
            return Err(ChanceryError::EvidencePolicyRequiredFieldUnsupported.into());
        }

        if self.counterparty_reporting_schema_hash != [0u8; 32]
            || self.freeform_allowed()
            || self.maximum_freeform_field_count != 0
            || self.maximum_freeform_value_bytes != 0
            || self.retention_flags != 0
        {
            return Err(ChanceryError::EvidencePolicyUnsupportedConfiguration.into());
        }

        Ok(())
    }

    pub fn assert_settlement_event_supported(&self) -> Result<(), ProgramError> {
        self.assert_runtime_supported()
    }
}

// ─── Size assertion ───────────────────────────────────────────────────────────
const _: () = assert!(
    core::mem::size_of::<EvidencePolicy>() == EVIDENCE_POLICY_SIZE,
    "EvidencePolicy size mismatch - update EVIDENCE_POLICY_SIZE",
);

#[cfg(test)]
mod tests {
    use super::*;
    use bytemuck::Zeroable;

    #[test]
    fn supported_settlement_fields_are_accepted() {
        let mut policy = EvidencePolicy::zeroed();
        policy.required_field_mask[0] =
            required_field::PATHWAY_ID | required_field::NET_AMOUNT;
        assert!(policy.assert_runtime_supported().is_ok());
    }

    #[test]
    fn unsupported_high_word_and_schema_requirements_fail_closed() {
        let mut high_word_policy = EvidencePolicy::zeroed();
        high_word_policy.required_field_mask[1] = 1;
        assert!(high_word_policy.assert_runtime_supported().is_err());

        let mut schema_policy = EvidencePolicy::zeroed();
        schema_policy.counterparty_reporting_schema_hash = [7u8; 32];
        assert!(schema_policy.assert_runtime_supported().is_err());
    }

    #[test]
    fn unsupported_freeform_configuration_fails_closed() {
        let mut policy = EvidencePolicy::zeroed();
        policy.allow_freeform_counterparty_fields = 1;
        assert!(policy.assert_runtime_supported().is_err());

        let mut retention_policy = EvidencePolicy::zeroed();
        retention_policy.retention_flags = 1;
        assert!(retention_policy.assert_runtime_supported().is_err());
    }
}
