use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;

use crate::{error::ChanceryError, modules::ChanceryModule};

pub mod instructions;
pub mod state;

/// Single scalar settlement-output floor checker used by direct and
/// intent-backed paths. The caller must supply the minimum from the authority
/// that owns it: instruction args for direct settlement, or the verified and
/// content-addressed intent for delegated/trilateral settlement.
pub fn assert_output_meets_minimum(
    actual_output:       u64,
    minimum_output:      u64,
    below_minimum_error: ChanceryError,
) -> ProgramResult {
    if actual_output < minimum_output {
        return Err(below_minimum_error.into());
    }

    Ok(())
}

#[cfg(test)]
pub mod tests;

pub struct Module;

impl ChanceryModule for Module {
    const MODULE_ID: u8 = crate::constants::module::SETTLEMENT;

    fn dispatch<'a>(accounts: &'a [AccountInfo<'a>], data: &[u8]) -> ProgramResult {
        crate::modules::control::auth::dispatch_gated(
            Self::MODULE_ID,
            accounts,
            data,
            instructions::dispatch,
        )
    }
}

#[cfg(test)]
mod minimum_output_tests {
    use super::*;
    use solana_program_error::ProgramError;

    #[test]
    fn shared_minimum_checker_accepts_equal_output() {
        assert!(assert_output_meets_minimum(
            20,
            20,
            ChanceryError::AmountBelowMinimum,
        )
        .is_ok());
    }

    #[test]
    fn shared_minimum_checker_uses_the_callers_error_vocabulary() {
        assert_eq!(
            assert_output_meets_minimum(
                9,
                10,
                ChanceryError::IntentAmountBelowMinimum,
            ),
            Err(ProgramError::from(ChanceryError::IntentAmountBelowMinimum)),
        );
    }

    #[test]
    fn zero_minimum_is_disabled_without_a_separate_branch() {
        assert!(assert_output_meets_minimum(
            0,
            0,
            ChanceryError::AmountBelowMinimum,
        )
        .is_ok());
    }
}
