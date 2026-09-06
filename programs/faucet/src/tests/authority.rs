use crate::authority::{authority_address, validate_role_mask, ROLES};
use solana_pubkey::Pubkey;

#[test]
fn role_authorities_are_distinct() {
    let program = Pubkey::new_from_array([7; 32]);
    let addresses: Vec<Pubkey> = ROLES.iter().map(|role| authority_address(*role, &program).unwrap().0).collect();
    for (index, address) in addresses.iter().enumerate() {
        assert!(!addresses[..index].contains(address));
    }
    assert!(authority_address(5, &program).is_err());
}

#[test]
fn only_nonempty_known_role_masks_are_accepted() {
    for mask in 0u8..=u8::MAX {
        assert_eq!(validate_role_mask(mask).is_ok(), mask > 0 && mask < 32);
    }
}
