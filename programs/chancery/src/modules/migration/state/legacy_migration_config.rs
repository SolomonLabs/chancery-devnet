use bytemuck::{Pod, Zeroable};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{
    constants::seeds,
    error::ChanceryError,
};

// ─── Discriminator ────────────────────────────────────────────────────────────
pub const LEGACY_MIGRATION_CONFIG_DISCRIMINATOR: [u8; 8] =
    [0x6c, 0x67, 0x63, 0x79, 0x6d, 0x69, 0x67, 0x00];

// ─── Migration flags ──────────────────────────────────────────────────────────
pub mod migration_flag {
    /// Migration is currently enabled. One-way gate.
    pub const ENABLED: u64 = 1 << 0;
}

// ─── Account size ─────────────────────────────────────────────────────────────
//   [u8;8]   discriminator               =  8 @ 0
//   u16      version                     =  2 @ 8
//   u8       bump                        =  1 @ 10
//   [u8;5]   _pad0                       =  5 @ 11  -> u64 at 16
//   u64      migration_flags             =  8 @ 16
//   [u8;32]  legacy_program              = 32 @ 24
//   [u8;32]  legacy_mint                 = 32 @ 56
//   u64      migration_enabled_at_slot   =  8 @ 88  (88%8=0 Y)
//   u64      migrated_total              =  8 @ 96  (issued units minted)
//   u64      legacy_supply_snapshot      =  8 @ 104
//   u64      migrated_legacy_total       =  8 @ 112 (legacy units burned)
//   [u8;32]  _reserved                   = 32 @ 120
//                                         ─────
//                                         152 bytes
pub const LEGACY_MIGRATION_CONFIG_SIZE: usize = 152;

// ─── State ────────────────────────────────────────────────────────────────────

/// Singleton config for one-way legacy SPL -> Token-2022 migration.
/// Seed: [b"legacy-migration"]
///
/// Migration is one-way only: burn legacy token, mint equal amount of
/// Token-2022 issued token. No reserve movement. No reverse path.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LegacyMigrationConfig {
    pub discriminator:             [u8; 8],
    pub version:                   u16,
    pub bump:                      u8,
    pub _pad0:                     [u8; 5],

    /// Bitfield using `migration_flag::*` constants.
    pub migration_flags:           u64,

    /// The legacy program owning `legacy_mint`.
    pub legacy_program:            Pubkey,

    /// The legacy SPL token mint to be migrated from.
    pub legacy_mint:               Pubkey,

    /// Slot at which migration was enabled.
    pub migration_enabled_at_slot: u64,

    /// Cumulative total of legacy tokens burned and reissued (u64 is sufficient
    /// given token supply constraints), denominated in issued-token base units.
    pub migrated_total:            u64,

    /// Immutable cap captured from the initialized legacy mint's raw supply when
    /// migration is enabled. This bounds aggregate legacy units accepted by the
    /// migration without making any assertion about mint-authority state.
    pub legacy_supply_snapshot:    u64,

    /// Cumulative legacy-token base units burned under this migration config.
    pub migrated_legacy_total:     u64,

    pub _reserved:                 [u8; 32],
}

// ─── Helpers ──────────────────────────────────────────────────────────────────
impl LegacyMigrationConfig {
    pub fn pda(program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[seeds::LEGACY_MIGRATION], program_id)
    }

    pub fn verify_pda(
        account:    &AccountInfo,
        program_id: &Pubkey,
    ) -> Result<u8, ProgramError> {
        let (expected, bump) = Self::pda(program_id);

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
            LEGACY_MIGRATION_CONFIG_SIZE,
            LEGACY_MIGRATION_CONFIG_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    pub(crate) fn load_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        expected_bump: u8,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;

        crate::state_loader::assert_stored_bump_matches(state.bump, expected_bump)?;

        Ok(state)
    }

    /// Load initialized state and prove that its identity fields resolve to
    /// this account's canonical PDA and stored bump.
    pub fn load_verified<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::Ref<'a, Self>, ProgramError> {
        let state = Self::load(account)?;
        let expected_bump = Self::verify_pda(account, &crate::id())?;

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
            LEGACY_MIGRATION_CONFIG_SIZE,
            LEGACY_MIGRATION_CONFIG_DISCRIMINATOR,
            ChanceryError::NotInitialized,
        )?;

        Ok(state)
    }

    pub(crate) fn load_mut_for_verified_pda<'a>(
        account: &'a AccountInfo<'a>,
        expected_bump: u8,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;

        crate::state_loader::assert_stored_bump_matches(state.bump, expected_bump)?;

        Ok(state)
    }

    /// Mutable counterpart to `load_verified`.
    pub fn load_verified_mut<'a>(
        account: &'a AccountInfo<'a>,
    ) -> Result<core::cell::RefMut<'a, Self>, ProgramError> {
        let state = Self::load_mut(account)?;
        let expected_bump = Self::verify_pda(account, &crate::id())?;

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
            LEGACY_MIGRATION_CONFIG_SIZE,
            ChanceryError::AlreadyInitialized,
        )
    }

    // ── Guards ────────────────────────────────────────────────────────────────

    pub fn assert_enabled(&self) -> Result<(), ProgramError> {
        if self.migration_flags & migration_flag::ENABLED == 0 {
            return Err(ChanceryError::LegacyMigrationNotEnabled.into());
        }

        Ok(())
    }

    pub fn assert_legacy_mint(&self, mint: &Pubkey) -> Result<(), ProgramError> {
        if &self.legacy_mint != mint {
            return Err(ChanceryError::LegacyMintMismatch.into());
        }

        Ok(())
    }

    /// Prove the snapshot conservation envelope before a migration burn:
    ///
    /// `current legacy supply + cumulative migrated legacy units <= snapshot`.
    ///
    /// A normal migration preserves the left-hand side because the burn reduces
    /// current supply by exactly the amount added to `migrated_legacy_total`.
    /// External burns may reduce it; post-snapshot issuance that would raise it
    /// above the activation snapshot fails closed. This does not assert mint-
    /// authority revocation or claim that individual token units predate the
    /// snapshot.
    pub fn assert_migration_supply_available(
        &self,
        current_legacy_supply: u64,
        legacy_amount:         u64,
    ) -> Result<(), ProgramError> {
        if legacy_amount > current_legacy_supply {
            return Err(ChanceryError::MigrationSupplyExceeded.into());
        }

        let accounted_legacy_supply = (current_legacy_supply as u128)
            .checked_add(self.migrated_legacy_total as u128)
            .ok_or(ChanceryError::ArithmeticOverflow)?;

        if accounted_legacy_supply > self.legacy_supply_snapshot as u128 {
            return Err(ChanceryError::MigrationSupplyExceeded.into());
        }

        let next_legacy_total = self
            .migrated_legacy_total
            .checked_add(legacy_amount)
            .ok_or(ChanceryError::ArithmeticOverflow)?;
        if next_legacy_total > self.legacy_supply_snapshot {
            return Err(ChanceryError::MigrationSupplyExceeded.into());
        }

        Ok(())
    }

    /// Accumulate both migration denominations after successful burn and mint
    /// CPIs. Both additions are proven before either field is mutated.
    pub fn accumulate_migrated(
        &mut self,
        legacy_amount: u64,
        issued_amount: u64,
    ) -> Result<(), ProgramError> {
        let next_legacy_total = self
            .migrated_legacy_total
            .checked_add(legacy_amount)
            .ok_or(ChanceryError::ArithmeticOverflow)?;
        let next_issued_total = self
            .migrated_total
            .checked_add(issued_amount)
            .ok_or(ChanceryError::ArithmeticOverflow)?;

        if next_legacy_total > self.legacy_supply_snapshot {
            return Err(ChanceryError::MigrationSupplyExceeded.into());
        }

        self.migrated_legacy_total = next_legacy_total;
        self.migrated_total = next_issued_total;

        Ok(())
    }
}

/// Convert a requested legacy burn `amount` into the `(burned, minted)` pair for a
/// value-preserving migration across (possibly different) token decimals.
///
/// - **Equal decimals**: exact 1:1 - `burned == minted == amount`.
/// - **Reducing decimals** (`legacy_decimals > issued_decimals`): the **full**
///   `amount` is burned and the mint is floored to `floor(amount / 10^(legacy-issued))`.
///   The sub-precision remainder is **forfeited** (burned without credit) so no legacy
///   dust is ever left behind. E.g. 9→6 decimals: burn `amount`, mint `floor(amount/1000)`.
/// - **Increasing decimals** (`legacy_decimals < issued_decimals`): exact, every
///   legacy unit maps to `10^(issued-legacy)` issued units (no dust).
///
/// Returns `AmountBelowMinimum` when nothing is migratable (`minted == 0`, e.g. a
/// reducing-decimals `amount` smaller than one issued unit) - burning legacy for zero
/// issued tokens is rejected.
pub fn convert_migration_amount(
    amount:          u64,
    legacy_decimals: u8,
    issued_decimals: u8,
) -> Result<(u64, u64), ProgramError> {
    let (burned, minted) = if legacy_decimals == issued_decimals {
        (amount, amount)
    } else if legacy_decimals > issued_decimals {
        let factor = 10u64
            .checked_pow((legacy_decimals - issued_decimals) as u32)
            .ok_or(ChanceryError::ArithmeticOverflow)?;
        let minted = amount.checked_div(factor).ok_or(ChanceryError::DivisionByZero)?;
        // Burn the full amount and floor the mint: the sub-precision remainder is
        // forfeited (burned without credit) so no legacy dust is stranded.
        (amount, minted)
    } else {
        let factor = 10u64
            .checked_pow((issued_decimals - legacy_decimals) as u32)
            .ok_or(ChanceryError::ArithmeticOverflow)?;
        let minted = amount.checked_mul(factor).ok_or(ChanceryError::ArithmeticOverflow)?;
        (amount, minted)
    };

    if minted == 0 {
        return Err(ChanceryError::AmountBelowMinimum.into());
    }

    Ok((burned, minted))
}

// ─── Size assertion ───────────────────────────────────────────────────────────
const _: () = assert!(
    core::mem::size_of::<LegacyMigrationConfig>() == LEGACY_MIGRATION_CONFIG_SIZE,
    "LegacyMigrationConfig size mismatch - update LEGACY_MIGRATION_CONFIG_SIZE",
);

#[cfg(test)]
mod migration_supply_tests {
    use super::*;

    fn config(snapshot: u64, migrated_legacy_total: u64, migrated_total: u64) -> LegacyMigrationConfig {
        let mut config = LegacyMigrationConfig::zeroed();
        config.migration_flags = migration_flag::ENABLED;
        config.legacy_supply_snapshot = snapshot;
        config.migrated_legacy_total = migrated_legacy_total;
        config.migrated_total = migrated_total;
        config
    }

    #[test]
    fn supply_snapshot_is_an_inclusive_aggregate_cap() {
        let mut config = config(100, 90, 45);

        assert!(config.assert_migration_supply_available(10, 10).is_ok());
        assert_eq!(
            config.assert_migration_supply_available(10, 11).unwrap_err(),
            ProgramError::Custom(ChanceryError::MigrationSupplyExceeded as u32),
        );

        config.accumulate_migrated(10, 5).unwrap();
        assert_eq!(config.migrated_legacy_total, 100);
        assert_eq!(config.migrated_total, 50);
    }

    #[test]
    fn failed_accumulation_does_not_partially_mutate_totals() {
        let mut config = config(100, 90, u64::MAX);

        assert_eq!(
            config.accumulate_migrated(10, 1).unwrap_err(),
            ProgramError::Custom(ChanceryError::ArithmeticOverflow as u32),
        );
        assert_eq!(config.migrated_legacy_total, 90);
        assert_eq!(config.migrated_total, u64::MAX);
    }
}
