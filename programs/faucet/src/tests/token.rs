use crate::token::{maximum_base_units, mint_decimals};
use solana_pubkey::Pubkey;

#[test]
fn cap_scales_with_decimals_and_rejects_overflow() {
    assert_eq!(maximum_base_units(0).unwrap(), 10_000);
    assert_eq!(maximum_base_units(6).unwrap(), 10_000_000_000);
    assert_eq!(maximum_base_units(9).unwrap(), 10_000_000_000_000);
    assert!(maximum_base_units(16).is_err());
    assert!(maximum_base_units(255).is_err());
}

#[test]
fn mint_requires_initialized_state_and_faucet_authority() {
    let authority = Pubkey::new_from_array([8; 32]);
    let mut data = [0u8; 82];
    assert!(mint_decimals(&data[..81], &authority).is_err());
    data[0] = 1;
    data[4..36].copy_from_slice(authority.as_ref());
    data[44] = 6;
    assert!(mint_decimals(&data, &authority).is_err());
    data[45] = 1;
    assert_eq!(mint_decimals(&data, &authority).unwrap(), 6);
    data[0] = 0;
    assert!(mint_decimals(&data, &authority).is_err());
    data[0] = 1;
    data[4] ^= 1;
    assert!(mint_decimals(&data, &authority).is_err());
}
