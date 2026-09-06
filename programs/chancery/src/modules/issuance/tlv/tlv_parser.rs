//! Minimal Token-2022 mint TLV parser.
//!
//! Parses the TLV (type-length-value) section of a Token-2022 mint account
//! and converts active extension tags into a Chancery `extension_bit`
//! bitmask.
//!
//! Wire layout of a Token-2022 mint account:
//!   [ 0..82 )    SPL base mint layout
//!   [ 82..165 )  padding (matches Token Account base length so multiplexing works)
//!   [ 165 )      account_type byte (1 for Mint, 2 for TokenAccount)
//!   [ 166..   )  TLV section: repeated [tag(u16 LE) | length(u16 LE) | value(length bytes)]
//!
//! Per minimal parser - only Token-2022 extensions in
//! registry, fail closed on unknown tag.

use crate::{
    constants::{extension_bit, MAX_TOKEN_DECIMALS},
    error::ChanceryError,
};

/// SPL base mint length.
pub const MINT_BASE_LEN: usize                  = 82;
/// Offset of the little-endian `supply` u64 in the SPL base mint layout.
pub const SPL_MINT_SUPPLY_OFFSET: usize         = 36;
/// Offset of the `decimals` byte in the SPL base mint layout (82 bytes).
/// Shared by SPL Token and Token-2022 - the extension region starts at 166.
pub const SPL_MINT_DECIMALS_OFFSET: usize       = 44;
/// Offset of the `is_initialized` byte in the SPL base mint layout.
pub const SPL_MINT_IS_INITIALIZED_OFFSET: usize = 45;
/// Offset of the account_type discriminator byte.
pub const ACCOUNT_TYPE_OFFSET: usize            = 165;
/// Start of the TLV section.
pub const TLV_OFFSET: usize                     = 166;
/// Token-2022 mint account_type discriminator.
pub const ACCOUNT_TYPE_MINT: u8                 = 1;

/// Canonical spl-token-2022 `ExtensionType` `#[repr(u16)]` discriminants.
/// Pinned to spl-token-2022 v6.x / `@solana/spl-token-2022` ExtensionType
/// (vendor/combined Token2022ExtensionType). Chancery bit positions remain
/// internal - see `constants::extension_bit`.
pub mod extension_tlv_tag {
    pub const TRANSFER_FEE_CONFIG:              u16 = 0x0001;
    pub const TRANSFER_FEE_AMOUNT:              u16 = 0x0002;
    pub const MINT_CLOSE_AUTHORITY:             u16 = 0x0003;
    pub const CONFIDENTIAL_TRANSFER_MINT:       u16 = 0x0004;
    pub const CONFIDENTIAL_TRANSFER_ACCOUNT:    u16 = 0x0005;
    pub const DEFAULT_ACCOUNT_STATE:            u16 = 0x0006;
    pub const IMMUTABLE_OWNER:                  u16 = 0x0007;
    pub const MEMO_TRANSFER:                    u16 = 0x0008;
    pub const NON_TRANSFERABLE:                 u16 = 0x0009;
    pub const INTEREST_BEARING_CONFIG:          u16 = 0x000A;
    pub const CPI_GUARD:                        u16 = 0x000B;
    pub const PERMANENT_DELEGATE:               u16 = 0x000C;
    pub const NON_TRANSFERABLE_ACCOUNT:         u16 = 0x000D;
    pub const TRANSFER_HOOK:                    u16 = 0x000E;
    pub const TRANSFER_HOOK_ACCOUNT:            u16 = 0x000F;
    pub const CONFIDENTIAL_TRANSFER_FEE_CONFIG: u16 = 0x0010;
    pub const CONFIDENTIAL_TRANSFER_FEE_AMOUNT: u16 = 0x0011;
    pub const METADATA_POINTER:                 u16 = 0x0012;
    pub const TOKEN_METADATA:                   u16 = 0x0013;
    pub const GROUP_POINTER:                    u16 = 0x0014;
    pub const TOKEN_GROUP:                      u16 = 0x0015;
    pub const GROUP_MEMBER_POINTER:             u16 = 0x0016;
    pub const TOKEN_GROUP_MEMBER:               u16 = 0x0017;
    pub const CONFIDENTIAL_MINT_BURN:           u16 = 0x0018;
    pub const SCALED_UI_AMOUNT:                 u16 = 0x0019;
    pub const PAUSABLE:                         u16 = 0x001A;
    pub const PAUSABLE_ACCOUNT:                 u16 = 0x001B;
    pub const PERMISSIONED_BURN:                u16 = 0x001C;
}

enum MintTlvTagClass {
    MintExtension(u64),
    /// Account-only ExtensionType on a mint TLV stream - skip, do not set mint bits.
    AccountExtension,
    Unknown,
}

/// Read the `decimals` field from an SPL Token / Token-2022 base mint account.
pub fn read_spl_base_mint_decimals(mint_data: &[u8]) -> Result<u8, ChanceryError> {
    if mint_data.len() < MINT_BASE_LEN {
        return Err(ChanceryError::AccountDataLengthMismatch);
    }

    if mint_data[SPL_MINT_IS_INITIALIZED_OFFSET] != 1 {
        return Err(ChanceryError::NotInitialized);
    }

    let decimals = mint_data[SPL_MINT_DECIMALS_OFFSET];

    if decimals > MAX_TOKEN_DECIMALS {
        return Err(ChanceryError::AssetDecimalsMismatch);
    }

    Ok(decimals)
}

/// Read the raw `supply` field from an initialized SPL Token / Token-2022 base mint.
pub fn read_spl_base_mint_supply(mint_data: &[u8]) -> Result<u64, ChanceryError> {
    if mint_data.len() < MINT_BASE_LEN {
        return Err(ChanceryError::AccountDataLengthMismatch);
    }

    if mint_data[SPL_MINT_IS_INITIALIZED_OFFSET] != 1 {
        return Err(ChanceryError::NotInitialized);
    }

    let mut supply_bytes = [0u8; 8];
    let supply_end = SPL_MINT_SUPPLY_OFFSET + supply_bytes.len();
    supply_bytes.copy_from_slice(&mint_data[SPL_MINT_SUPPLY_OFFSET..supply_end]);

    Ok(u64::from_le_bytes(supply_bytes))
}

/// Map a canonical ExtensionType tag to a Chancery mint bit, account skip, or unknown.
fn classify_mint_tlv_tag(tlv_tag: u16) -> MintTlvTagClass {
    use extension_tlv_tag as tag;

    match tlv_tag {
        tag::TRANSFER_FEE_CONFIG              => MintTlvTagClass::MintExtension(extension_bit::TRANSFER_FEE_CONFIG),
        tag::TRANSFER_FEE_AMOUNT              => MintTlvTagClass::AccountExtension,
        tag::MINT_CLOSE_AUTHORITY             => MintTlvTagClass::MintExtension(extension_bit::MINT_CLOSE_AUTHORITY),
        tag::CONFIDENTIAL_TRANSFER_MINT       => MintTlvTagClass::MintExtension(extension_bit::CONFIDENTIAL_TRANSFER),
        tag::CONFIDENTIAL_TRANSFER_ACCOUNT    => MintTlvTagClass::AccountExtension,
        tag::DEFAULT_ACCOUNT_STATE            => MintTlvTagClass::MintExtension(extension_bit::DEFAULT_ACCOUNT_STATE),
        tag::IMMUTABLE_OWNER                  => MintTlvTagClass::AccountExtension,
        tag::MEMO_TRANSFER                    => MintTlvTagClass::AccountExtension,
        tag::NON_TRANSFERABLE                 => MintTlvTagClass::MintExtension(extension_bit::NON_TRANSFERABLE),
        tag::INTEREST_BEARING_CONFIG          => MintTlvTagClass::MintExtension(extension_bit::INTEREST_BEARING_CONFIG),
        tag::CPI_GUARD                        => MintTlvTagClass::AccountExtension,
        tag::PERMANENT_DELEGATE               => MintTlvTagClass::MintExtension(extension_bit::PERMANENT_DELEGATE),
        tag::NON_TRANSFERABLE_ACCOUNT         => MintTlvTagClass::AccountExtension,
        // Value-aware classification is handled in parse_mint_extension_mask.
        tag::TRANSFER_HOOK                    => MintTlvTagClass::MintExtension(extension_bit::TRANSFER_HOOK),
        tag::TRANSFER_HOOK_ACCOUNT            => MintTlvTagClass::AccountExtension,
        tag::CONFIDENTIAL_TRANSFER_FEE_CONFIG => MintTlvTagClass::MintExtension(extension_bit::CONFIDENTIAL_TRANSFER_FEE_CONFIG),
        tag::CONFIDENTIAL_TRANSFER_FEE_AMOUNT => MintTlvTagClass::AccountExtension,
        tag::METADATA_POINTER                 => MintTlvTagClass::MintExtension(extension_bit::METADATA_POINTER),
        tag::TOKEN_METADATA                   => MintTlvTagClass::MintExtension(extension_bit::TOKEN_METADATA),
        tag::GROUP_POINTER                    => MintTlvTagClass::MintExtension(extension_bit::GROUP_POINTER),
        tag::TOKEN_GROUP                      => MintTlvTagClass::MintExtension(extension_bit::TOKEN_GROUP),
        tag::GROUP_MEMBER_POINTER             => MintTlvTagClass::MintExtension(extension_bit::GROUP_MEMBER_POINTER),
        tag::TOKEN_GROUP_MEMBER               => MintTlvTagClass::MintExtension(extension_bit::TOKEN_GROUP_MEMBER),
        tag::CONFIDENTIAL_MINT_BURN           => MintTlvTagClass::MintExtension(extension_bit::CONFIDENTIAL_MINT_BURN),
        tag::SCALED_UI_AMOUNT                 => MintTlvTagClass::MintExtension(extension_bit::SCALED_UI_AMOUNT),
        tag::PAUSABLE                         => MintTlvTagClass::MintExtension(extension_bit::PAUSABLE),
        tag::PAUSABLE_ACCOUNT                 => MintTlvTagClass::AccountExtension,
        tag::PERMISSIONED_BURN                => MintTlvTagClass::MintExtension(extension_bit::PERMISSIONED_BURN),
        _                                     => MintTlvTagClass::Unknown,
    }
}

/// Parse a Token-2022 mint account's TLV section into a Chancery mint
/// `extension_bit` mask (low u64 word). Account-only ExtensionType tags are
/// skipped; unknown tags fail closed.
///
/// Fails closed on:
///   - data shorter than required
///   - account_type byte is not Mint
///   - unknown TLV tag
///   - TLV length runs past end of account
pub fn parse_mint_extension_mask(data: &[u8]) -> Result<u64, ChanceryError> {
    if data.len() <= ACCOUNT_TYPE_OFFSET {
        // Pre-extension SPL Token mint (no extensions). Return empty mask.
        if data.len() == MINT_BASE_LEN {
            return Ok(0);
        }

        return Err(ChanceryError::Token2022TlvMalformed);
    }
    if data[ACCOUNT_TYPE_OFFSET] != ACCOUNT_TYPE_MINT {
        return Err(ChanceryError::Token2022TlvMalformed);
    }

    let mut mask: u64     = 0;
    let mut cursor: usize = TLV_OFFSET;
    let total: usize      = data.len();

    while cursor + 4 <= total {
        let tag_lo: u16 = data[cursor] as u16;
        let tag_hi: u16 = data[cursor + 1] as u16;
        let tag: u16    = tag_lo | (tag_hi << 8);
        let len_lo: u16 = data[cursor + 2] as u16;
        let len_hi: u16 = data[cursor + 3] as u16;
        let len: usize  = (len_lo | (len_hi << 8)) as usize;

        // Tag 0 is the "uninitialized" marker - reach end of TLV.
        if tag == 0 {
            break;
        }

        let value_start: usize = cursor + 4;
        let value_end: usize   = value_start.saturating_add(len);

        if value_end > total {
            return Err(ChanceryError::Token2022TlvMalformed);
        }

        if tag == extension_tlv_tag::TRANSFER_HOOK {
            // Token-2022 TransferHook stores two 32-byte OptionalNonZeroPubkeys:
            // authority then program id. A zero program id means the extension
            // is reserved but disabled and no hook CPI is executed.
            if len != 64 {
                return Err(ChanceryError::Token2022TlvMalformed);
            }
            let program_id = &data[value_start + 32..value_end];
            let bit = if program_id.iter().all(|byte| *byte == 0) {
                extension_bit::DORMANT_TRANSFER_HOOK
            } else {
                extension_bit::TRANSFER_HOOK
            };
            mask |= bit;
        } else {
            match classify_mint_tlv_tag(tag) {
                MintTlvTagClass::MintExtension(bit) => mask |= bit,
                MintTlvTagClass::AccountExtension => {}
                MintTlvTagClass::Unknown          => return Err(ChanceryError::Token2022UnknownExtensionTag),
            }
        }

        cursor = value_end;
    }

    Ok(mask)
}

/// Parse a mint account into the two-word `[u64; 2]` mask stored on
/// `AssetConfig` / `IssuedTokenControl`. The low word comes from TLV parsing;
/// the high word is reserved until any mapped extension exceeds bit 63.
pub fn parse_mint_extension_mask_full(data: &[u8]) -> Result<[u64; 2], ChanceryError> {
    Ok([parse_mint_extension_mask(data)?, 0])
}

/// Locate a single TLV value by tag. Returns the slice of the value bytes
/// (not including tag/length header).
pub fn find_tlv_value<'a>(
    data:       &'a [u8],
    target_tag: u16,
) -> Result<Option<&'a [u8]>, ChanceryError> {
    if data.len() <= ACCOUNT_TYPE_OFFSET {
        return Ok(None);
    }

    if data[ACCOUNT_TYPE_OFFSET] != ACCOUNT_TYPE_MINT {
        return Err(ChanceryError::Token2022TlvMalformed);
    }

    let mut cursor: usize = TLV_OFFSET;
    let total: usize      = data.len();

    while cursor + 4 <= total {
        let tag_lo: u16 = data[cursor] as u16;
        let tag_hi: u16 = data[cursor + 1] as u16;
        let tag: u16    = tag_lo | (tag_hi << 8);
        let len_lo: u16 = data[cursor + 2] as u16;
        let len_hi: u16 = data[cursor + 3] as u16;
        let len: usize  = (len_lo | (len_hi << 8)) as usize;

        if tag == 0 {
            break;
        }
        let value_start : usize = cursor + 4;
        let value_end: usize    = value_start.saturating_add(len);

        if value_end > total {
            return Err(ChanceryError::Token2022TlvMalformed);
        }

        if tag == target_tag {
            return Ok(Some(&data[value_start..value_end]));
        }

        cursor = value_end;
    }

    Ok(None)
}

/// Expected Chancery PDA authorities for the reserved mint extensions, copied
/// out of `IssuedTokenControl` by the verifier. All are 32-byte pubkeys.
pub struct ExpectedExtensionAuthorities {
    pub permanent_delegate:              [u8; 32],
    pub transfer_hook_authority:         [u8; 32],
    pub transfer_hook_program:           [u8; 32],
    pub close_mint_authority:            [u8; 32],
    pub pause_authority:                 [u8; 32],
    pub metadata_pointer_authority:      [u8; 32],
    pub metadata_update_authority:       [u8; 32],
    pub confidential_transfer_authority: [u8; 32],
}

/// Token-2022 `AccountState::Initialized` - the normal (non-frozen) default.
const ACCOUNT_STATE_INITIALIZED: u8 = 1;

/// issue-72: the tag-only mask check cannot tell a Chancery-controlled reserved
/// extension from an attacker-controlled one (a `PermanentDelegate` / `TransferHook`
/// / etc. whose value names an external key). For every present authority-bearing
/// reserved extension, require its authority/delegate/program value to equal the
/// corresponding Chancery PDA. The authority is at offset 0 of each extension's
/// TLV value; `TransferHook` additionally carries its hook program at offset 32;
/// `DefaultAccountState` has no authority of its own (governed by the already-
/// verified freeze authority) so its state byte must be `Initialized`.
pub fn assert_mint_extension_authorities(
    data:          &[u8],
    observed_mask: [u64; 2],
    reserved_mask: [u64; 2],
    expected:      &ExpectedExtensionAuthorities,
) -> Result<(), ChanceryError> {
    use extension_tlv_tag as tag;

    const UNBOUND_AUTHORITY_BEARING_EXTENSION_MASK: u64 =
        extension_bit::GROUP_POINTER
        | extension_bit::TOKEN_GROUP
        | extension_bit::GROUP_MEMBER_POINTER
        | extension_bit::TOKEN_GROUP_MEMBER
        | extension_bit::CONFIDENTIAL_TRANSFER_FEE_CONFIG
        | extension_bit::PERMISSIONED_BURN;

    // Audit issue #72 follow-up: a reserved extension is safe only when its
    // authority-bearing fields are bound to a known Chancery PDA. The protocol
    // does not currently store canonical authorities for these extension
    // families, so their presence must fail closed even if governance reserved
    // their bits for possible future use.
    if observed_mask[0] & reserved_mask[0] & UNBOUND_AUTHORITY_BEARING_EXTENSION_MASK != 0 {
        return Err(ChanceryError::IssuedTokenExtensionAuthorityInvalid);
    }

    fn require(value: &[u8], expected: &[u8; 32]) -> Result<(), ChanceryError> {
        if value.len() < 32 || value[..32] != expected[..] {
            return Err(ChanceryError::IssuedTokenExtensionAuthorityInvalid);
        }
        Ok(())
    }

    if let Some(v) = find_tlv_value(data, tag::PERMANENT_DELEGATE)? {
        require(v, &expected.permanent_delegate)?;
    }
    if let Some(v) = find_tlv_value(data, tag::MINT_CLOSE_AUTHORITY)? {
        require(v, &expected.close_mint_authority)?;
    }
    if let Some(v) = find_tlv_value(data, tag::PAUSABLE)? {
        require(v, &expected.pause_authority)?;
        // Token-2022 Pausable stores the current paused flag immediately after
        // the 32-byte authority. Deployment readiness requires an unpaused mint;
        // this binary has no active on-chain recovery instruction for a mint
        // that is already paused.
        if v.len() < 33 || v[32] != 0 {
            return Err(ChanceryError::IssuedTokenExtensionAuthorityInvalid);
        }
    }
    if let Some(v) = find_tlv_value(data, tag::METADATA_POINTER)? {
        require(v, &expected.metadata_pointer_authority)?;
    }
    if let Some(v) = find_tlv_value(data, tag::TOKEN_METADATA)? {
        require(v, &expected.metadata_update_authority)?;
    }
    if let Some(v) = find_tlv_value(data, tag::CONFIDENTIAL_TRANSFER_MINT)? {
        require(v, &expected.confidential_transfer_authority)?;
    }
    if let Some(v) = find_tlv_value(data, tag::TRANSFER_HOOK)? {
        require(v, &expected.transfer_hook_authority)?;
        if v.len() < 64 || v[32..64] != expected.transfer_hook_program[..] {
            return Err(ChanceryError::IssuedTokenExtensionAuthorityInvalid);
        }
    }
    if let Some(v) = find_tlv_value(data, tag::DEFAULT_ACCOUNT_STATE)? {
        if v.is_empty() || v[0] != ACCOUNT_STATE_INITIALIZED {
            return Err(ChanceryError::IssuedTokenExtensionAuthorityInvalid);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ChanceryError;

    fn build_mint_with_tlvs(tlvs: &[(u16, &[u8])]) -> Vec<u8> {
        let mut v: Vec<u8> = vec![0u8; ACCOUNT_TYPE_OFFSET];

        v.push(ACCOUNT_TYPE_MINT);

        for &(tag, value) in tlvs {
            v.push((tag & 0xFF) as u8);
            v.push((tag >> 8) as u8);
            v.push((value.len() & 0xFF) as u8);
            v.push((value.len() >> 8) as u8);
            v.extend_from_slice(value);
        }
        v
    }

    // ── issue-72: reserved-extension authority verification ───────────────────
    fn assert_authorities(
        mint:     &[u8],
        reserved: u64,
        expected: &ExpectedExtensionAuthorities,
    ) -> Result<(), ChanceryError> {
        let observed = parse_mint_extension_mask_full(mint)?;
        assert_mint_extension_authorities(mint, observed, [reserved, 0], expected)
    }

    fn expected_all(pda: [u8; 32]) -> ExpectedExtensionAuthorities {
        ExpectedExtensionAuthorities {
            permanent_delegate:              pda,
            transfer_hook_authority:         pda,
            transfer_hook_program:           [0u8; 32],
            close_mint_authority:            pda,
            pause_authority:                 pda,
            metadata_pointer_authority:      pda,
            metadata_update_authority:       pda,
            confidential_transfer_authority: pda,
        }
    }

    #[test]
    fn issue72_permanent_delegate_non_pda_authority_rejected() {
        use super::extension_tlv_tag as tag;
        let pda      = [7u8; 32];
        let attacker = [9u8; 32];
        let mint     = build_mint_with_tlvs(&[(tag::PERMANENT_DELEGATE, &attacker)]);
        assert!(matches!(
            assert_authorities(&mint, u64::MAX, &expected_all(pda)),
            Err(ChanceryError::IssuedTokenExtensionAuthorityInvalid)
        ));
    }

    #[test]
    fn issue72_permanent_delegate_pda_authority_accepted() {
        use super::extension_tlv_tag as tag;
        let pda  = [7u8; 32];
        let mint = build_mint_with_tlvs(&[(tag::PERMANENT_DELEGATE, &pda)]);
        assert!(assert_authorities(&mint, u64::MAX, &expected_all(pda)).is_ok());
    }

    #[test]
    fn issue72_transfer_hook_wrong_program_rejected() {
        use super::extension_tlv_tag as tag;
        let pda = [7u8; 32];
        let mut value = [0u8; 64];
        value[..32].copy_from_slice(&pda); // authority = pda (ok)
        value[32..].copy_from_slice(&[9u8; 32]); // program = attacker (bad)
        let mint = build_mint_with_tlvs(&[(tag::TRANSFER_HOOK, &value)]);
        assert!(matches!(
            assert_authorities(&mint, u64::MAX, &expected_all(pda)),
            Err(ChanceryError::IssuedTokenExtensionAuthorityInvalid)
        ));
    }

    #[test]
    fn issue72_default_account_state_frozen_rejected() {
        use super::extension_tlv_tag as tag;
        let pda  = [7u8; 32];
        let mint = build_mint_with_tlvs(&[(tag::DEFAULT_ACCOUNT_STATE, &[2u8])]); // Frozen
        assert!(matches!(
            assert_authorities(&mint, u64::MAX, &expected_all(pda)),
            Err(ChanceryError::IssuedTokenExtensionAuthorityInvalid)
        ));
    }

    #[test]
    fn issue72_no_reserved_extensions_is_ok() {
        let pda  = [7u8; 32];
        let mint = build_mint_with_tlvs(&[]);
        assert!(assert_authorities(&mint, u64::MAX, &expected_all(pda)).is_ok());
    }

    #[test]
    fn issue72_unbound_reserved_authority_extensions_fail_closed() {
        use super::extension_tlv_tag as tag;

        let cases = [
            (tag::GROUP_POINTER, extension_bit::GROUP_POINTER),
            (tag::TOKEN_GROUP, extension_bit::TOKEN_GROUP),
            (tag::GROUP_MEMBER_POINTER, extension_bit::GROUP_MEMBER_POINTER),
            (tag::TOKEN_GROUP_MEMBER, extension_bit::TOKEN_GROUP_MEMBER),
            (
                tag::CONFIDENTIAL_TRANSFER_FEE_CONFIG,
                extension_bit::CONFIDENTIAL_TRANSFER_FEE_CONFIG,
            ),
            (tag::PERMISSIONED_BURN, extension_bit::PERMISSIONED_BURN),
        ];

        for (tag_value, extension_bit_value) in cases {
            let value = [7u8; 64];
            let mint = build_mint_with_tlvs(&[(tag_value, &value)]);
            assert!(matches!(
                assert_authorities(&mint, extension_bit_value, &expected_all([7u8; 32])),
                Err(ChanceryError::IssuedTokenExtensionAuthorityInvalid)
            ));
        }
    }

    #[test]
    fn issue72_unbound_extension_not_reserved_does_not_trigger_authority_gate() {
        use super::extension_tlv_tag as tag;

        let value = [7u8; 64];
        let mint = build_mint_with_tlvs(&[(tag::GROUP_POINTER, &value)]);
        assert!(assert_authorities(&mint, 0, &expected_all([7u8; 32])).is_ok());
    }

    #[test]
    fn pre_extension_spl_mint_has_empty_mask() {
        let v: Vec<u8> = vec![0u8; MINT_BASE_LEN];
        let mask: u64  = parse_mint_extension_mask(&v).unwrap();

        assert_eq!(mask, 0);
    }

    #[test]
    fn empty_tlv_section_has_empty_mask() {
        let v         = build_mint_with_tlvs(&[]);
        let mask: u64 = parse_mint_extension_mask(&v).unwrap();

        assert_eq!(mask, 0);
    }

    #[test]
    fn single_tlv_yields_correct_bit() {
        use super::extension_tlv_tag as tag;

        let v         = build_mint_with_tlvs(&[(tag::TRANSFER_FEE_CONFIG, &[0u8; 16])]);
        let mask: u64 = parse_mint_extension_mask(&v).unwrap();

        assert_eq!(mask, extension_bit::TRANSFER_FEE_CONFIG);
    }

    #[test]
    fn multiple_tlvs_or_together() {
        use super::extension_tlv_tag as tag;

        let mut active_hook = [0u8; 64];
        active_hook[32..].copy_from_slice(&[9u8; 32]);
        let v = build_mint_with_tlvs(&[
            (tag::TRANSFER_HOOK,      &active_hook),
            (tag::PERMANENT_DELEGATE, &[0u8; 32]),
        ]);

        let mask: u64 = parse_mint_extension_mask(&v).unwrap();

        assert_eq!(mask, extension_bit::TRANSFER_HOOK | extension_bit::PERMANENT_DELEGATE);
    }

    #[test]
    fn non_transferable_tag_maps_to_non_transferable_bit() {
        use super::extension_tlv_tag as tag;

        let v         = build_mint_with_tlvs(&[(tag::NON_TRANSFERABLE, &[0u8; 1])]);
        let mask: u64 = parse_mint_extension_mask(&v).unwrap();

        assert_eq!(mask, extension_bit::NON_TRANSFERABLE);
    }

    #[test]
    fn account_level_tlv_tag_is_skipped_on_mint() {
        use super::extension_tlv_tag as tag;

        let v         = build_mint_with_tlvs(&[(tag::IMMUTABLE_OWNER, &[0u8; 1])]);
        let mask: u64 = parse_mint_extension_mask(&v).unwrap();

        assert_eq!(mask, 0);
    }

    #[test]
    fn unknown_tlv_tag_fails_closed() {
        let v = build_mint_with_tlvs(&[(0xDEAD, &[0u8; 4])]);

        assert!(parse_mint_extension_mask(&v).is_err());
    }

    #[test]
    fn malformed_length_fails() {
        let mut v: Vec<u8> = vec![0u8; ACCOUNT_TYPE_OFFSET];

        v.push(ACCOUNT_TYPE_MINT);
        
        // tag=0x0001, length=999 (overruns)
        v.extend_from_slice(&[0x01, 0x00, 0xE7, 0x03]);

        assert!(parse_mint_extension_mask(&v).is_err());
    }

    #[test]
    fn wrong_account_type_fails() {
        let mut v: Vec<u8> = vec![0u8; ACCOUNT_TYPE_OFFSET];

        v.push(2);  // TokenAccount, not Mint

        assert!(parse_mint_extension_mask(&v).is_err());
    }

    #[test]
    fn find_tlv_value_returns_slice_when_present() {
        use super::extension_tlv_tag as tag;

        let v                    = build_mint_with_tlvs(&[(tag::MINT_CLOSE_AUTHORITY, &[0xAB, 0xCD, 0xEF])]);
        let slice: Option<&[u8]> = find_tlv_value(&v, tag::MINT_CLOSE_AUTHORITY).unwrap();

        assert_eq!(slice, Some(&[0xAB, 0xCD, 0xEF][..]));
    }

    #[test]
    fn find_tlv_value_returns_none_when_absent() {
        use super::extension_tlv_tag as tag;

        let v                    = build_mint_with_tlvs(&[(tag::TRANSFER_FEE_CONFIG, &[0u8; 4])]);
        let slice: Option<&[u8]> = find_tlv_value(&v, tag::MINT_CLOSE_AUTHORITY).unwrap();

        assert_eq!(slice, None);
    }

    #[test]
    fn read_spl_base_mint_decimals_reads_byte_at_offset_44() {
        let mut v = vec![0u8; MINT_BASE_LEN];

        v[SPL_MINT_DECIMALS_OFFSET]        = 9;
        v[SPL_MINT_IS_INITIALIZED_OFFSET] = 1;

        assert_eq!(read_spl_base_mint_decimals(&v).unwrap(), 9);
    }

    #[test]
    fn read_spl_base_mint_decimals_accepts_the_supported_maximum() {
        let mut v = vec![0u8; MINT_BASE_LEN];

        v[SPL_MINT_DECIMALS_OFFSET]        = MAX_TOKEN_DECIMALS;
        v[SPL_MINT_IS_INITIALIZED_OFFSET] = 1;

        assert_eq!(read_spl_base_mint_decimals(&v).unwrap(), MAX_TOKEN_DECIMALS);
    }

    #[test]
    fn read_spl_base_mint_decimals_rejects_values_above_the_supported_maximum() {
        let mut v = vec![0u8; MINT_BASE_LEN];

        v[SPL_MINT_DECIMALS_OFFSET]        = MAX_TOKEN_DECIMALS + 1;
        v[SPL_MINT_IS_INITIALIZED_OFFSET] = 1;

        assert_eq!(
            read_spl_base_mint_decimals(&v).unwrap_err(),
            ChanceryError::AssetDecimalsMismatch,
        );
    }

    #[test]
    fn read_spl_base_mint_decimals_rejects_uninitialized_mint() {
        let v = vec![0u8; MINT_BASE_LEN];

        assert_eq!(
            read_spl_base_mint_decimals(&v).unwrap_err(),
            ChanceryError::NotInitialized,
        );
    }

    #[test]
    fn read_spl_base_mint_decimals_rejects_short_data() {
        let v = vec![0u8; MINT_BASE_LEN - 1];

        assert_eq!(
            read_spl_base_mint_decimals(&v).unwrap_err(),
            ChanceryError::AccountDataLengthMismatch,
        );
    }

    #[test]
    fn read_spl_base_mint_supply_reads_little_endian_u64() {
        let mut v = vec![0u8; MINT_BASE_LEN];
        let expected = 0x0102_0304_0506_0708u64;

        v[SPL_MINT_SUPPLY_OFFSET..SPL_MINT_SUPPLY_OFFSET + 8]
            .copy_from_slice(&expected.to_le_bytes());
        v[SPL_MINT_IS_INITIALIZED_OFFSET] = 1;

        assert_eq!(read_spl_base_mint_supply(&v).unwrap(), expected);
    }

    #[test]
    fn read_spl_base_mint_supply_rejects_uninitialized_or_short_data() {
        let uninitialized = vec![0u8; MINT_BASE_LEN];
        let short = vec![0u8; MINT_BASE_LEN - 1];

        assert_eq!(
            read_spl_base_mint_supply(&uninitialized).unwrap_err(),
            ChanceryError::NotInitialized,
        );
        assert_eq!(
            read_spl_base_mint_supply(&short).unwrap_err(),
            ChanceryError::AccountDataLengthMismatch,
        );
    }

    #[test]
    fn dormant_transfer_hook_uses_distinct_allowed_bit() {
        use super::extension_tlv_tag as tag;

        let value = [0u8; 64];
        let mint = build_mint_with_tlvs(&[(tag::TRANSFER_HOOK, &value)]);
        assert_eq!(
            parse_mint_extension_mask(&mint).unwrap(),
            extension_bit::DORMANT_TRANSFER_HOOK,
        );
    }

    #[test]
    fn active_transfer_hook_uses_protocol_forbidden_bit() {
        use super::extension_tlv_tag as tag;

        let mut value = [0u8; 64];
        value[32..].copy_from_slice(&[7u8; 32]);
        let mint = build_mint_with_tlvs(&[(tag::TRANSFER_HOOK, &value)]);
        assert_eq!(
            parse_mint_extension_mask(&mint).unwrap(),
            extension_bit::TRANSFER_HOOK,
        );
    }

    #[test]
    fn malformed_transfer_hook_length_fails_closed() {
        use super::extension_tlv_tag as tag;

        let mint = build_mint_with_tlvs(&[(tag::TRANSFER_HOOK, &[0u8; 63])]);
        assert_eq!(
            parse_mint_extension_mask(&mint).unwrap_err(),
            ChanceryError::Token2022TlvMalformed,
        );
    }

    #[test]
    fn full_mask_wraps_low_word_and_zeroes_high_word() {
        use super::extension_tlv_tag as tag;

        let v    = build_mint_with_tlvs(&[(tag::TRANSFER_HOOK, &[0u8; 64])]);
        let full = parse_mint_extension_mask_full(&v).unwrap();

        assert_eq!(full[0], extension_bit::DORMANT_TRANSFER_HOOK);
        assert_eq!(full[1], 0);
    }
}
