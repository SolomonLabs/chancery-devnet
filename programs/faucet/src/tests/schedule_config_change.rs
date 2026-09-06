use super::*;

fn proposal(risk: u8) -> Vec<u8> {
    let mut data = vec![module::CONTROL, ix::control::PROPOSE_CONFIG_CHANGE];
    data.extend_from_slice(&1u16.to_le_bytes());
    data.push(risk);
    data.extend_from_slice(&[7u8; 96]);
    data.extend_from_slice(&0i64.to_le_bytes());
    data.extend_from_slice(&0i64.to_le_bytes());
    data.extend_from_slice(&42u64.to_le_bytes());
    data
}

#[test]
fn schedules_against_the_execution_clock_and_preserves_proposal_identity() {
    let original = proposal(2);
    let data = schedule(&original, 100, 1, 60).unwrap();
    let decoded = ProposeConfigChangeArgs::try_from_slice(&data[2..]).unwrap();
    assert_eq!(decoded.executable_after_unix_timestamp, 101);
    assert_eq!(decoded.expires_at_unix_timestamp, 161);
    assert_eq!(decoded.proposer_nonce, 42);
    assert_eq!(&data[..data.len() - 24], &original[..original.len() - 24]);
}

#[test]
fn preserves_all_timelocked_risk_floors() {
    for risk in 2..=5 {
        assert!(schedule(&proposal(risk), 100, 0, 60).is_err());
        assert!(schedule(&proposal(risk), 100, 1, 60).is_ok());
    }
}

#[test]
fn rejects_invalid_proposals_and_timing() {
    assert!(schedule(&proposal(0), 100, 1, 60).is_err());
    assert!(schedule(&proposal(6), 100, 1, 60).is_err());
    assert!(schedule(&proposal(2), 100, 1, 0).is_err());
    assert!(schedule(&proposal(2), i64::MAX, 1, 60).is_err());
    assert!(schedule(&proposal(2), 100, u64::MAX, 60).is_err());
    assert!(schedule(&[module::CORE, ix::core::REGISTER_ASSET], 100, 1, 60).is_err());
    assert!(schedule(&proposal(2)[..124], 100, 1, 60).is_err());
}
