/// initialize_chancery
///
/// Creates the singleton ChanceryConfig and PauseState PDAs.
///
/// Accounts:
///   0  chancery_config       writable  PDA [b"chancery-config"]
///   1  event_authority       readable  PDA [b"event-authority"]
///   2  payer                 signer
///   3  governance_authority  signer
///   4  system_program
///   5  pause_state           writable  PDA [b"pause-state"]
///   6  program_account                 this program's own Program account
///   7  programdata_account             this program's ProgramData account
///   8  upgrade_authority     signer    must equal programdata.upgrade_authority
///
/// Init is bound to the BPF upgrade authority - without that, anyone
/// observing a deploy can race the first call and seize all five
/// authority slots.
///
/// One-shot: `load_uninitialized_mut` rejects re-init.

use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::{pubkey, Pubkey};
use solana_sha256_hasher::hashv as sha256_hashv;
use solana_sysvar::Sysvar;

use crate::{
    account_security::assert_external_signer,
    constants::{seeds, status_flag, token_program},
    error::ChanceryError,
    modules::{
        control::state::pause_state::{
            PauseState, PAUSE_STATE_DISCRIMINATOR, PAUSE_STATE_SIZE,
        },
        core::{
            instructions::create_pda_account,
            state::chancery_config::{
                ChanceryConfig, AUTHORITY_BUMP_CACHE_VERSION,
                CHANCERY_CONFIG_DISCRIMINATOR, CHANCERY_CONFIG_SIZE,
            },
        },
        evidence::emit::{emit_chancery_initialized, ChanceryInitialized},
    },
};

// BPF Loader Upgradeable program id (owner of Program + ProgramData accounts).
const BPF_LOADER_UPGRADEABLE: Pubkey =
    pubkey!("BPFLoaderUpgradeab1e11111111111111111111111");

// Byte-offset constants for parsing the BPF loader-upgradeable accounts.
// Kept in a module so the IDL generator (which scrapes top-level
// `const X: usize = N` lines as account-index declarations) ignores them.
mod programdata_layout {
    /// Program layout:     4-byte tag | 32-byte programdata pubkey
    /// ProgramData layout: 4-byte tag | 8-byte slot | 1-byte Option tag | 32-byte upgrade_authority | bytecode
    pub const PROGRAM_DATA_MIN_LEN:            usize = 36;
    pub const PROGRAMDATA_DATA_MIN_LEN:        usize = 45;
    pub const PROGRAMDATA_PUBKEY_OFFSET:       usize = 4;
    pub const UPGRADE_AUTHORITY_OPTION_OFFSET: usize = 12;
    pub const UPGRADE_AUTHORITY_PUBKEY_OFFSET: usize = 13;
}

// Domain-separation tag for the on-chain-derived `domain_separator`. Mirrored
// off-chain so signers can recompute the value from the program_id alone.
// Bumping this tag is a wire-format breaking change.
const DOMAIN_SEPARATOR_TAG: &[u8] = b"CHANCERY_DOMAIN_V1";

// ─── Account indices ──────────────────────────────────────────────────────────
const CHANCERY_CONFIG:        usize = 0;
const EVENT_AUTHORITY:        usize = 1;
const PAYER:                  usize = 2;
const GOVERNANCE_AUTHORITY:   usize = 3;
const SYSTEM_PROGRAM:         usize = 4;
const PAUSE_STATE:            usize = 5;
const PROGRAM_ACCOUNT:        usize = 6;
const PROGRAMDATA_ACCOUNT:    usize = 7;
const UPGRADE_AUTHORITY:      usize = 8;
const REQUIRED_ACCOUNT_COUNT: usize = 9;

// ─── Args ─────────────────────────────────────────────────────────────────────
#[derive(BorshDeserialize)]
pub struct InitializeChanceryArgs {
    /// Canonical Token-2022 issued-token mint. Must be non-default because
    /// initialization is one-shot and no later instruction can repair it.
    pub issued_token_mint:    Pubkey,

    /// Must be exactly `token_program::TOKEN_2022`.
    pub issued_token_program: Pubkey,

    /// Legacy SPL mint for migration; default() if not applicable.
    pub legacy_token_mint:    Pubkey,

    /// Must be exactly `token_program::SPL_TOKEN`, or `Pubkey::default()` when
    /// no legacy migration is configured. The migration handler intentionally
    /// supports only the classic SPL legacy mint and initialization is one-shot.
    pub legacy_token_program: Pubkey,
}

pub(crate) fn assert_initialization_args(args: &InitializeChanceryArgs) -> ProgramResult {
    if args.issued_token_mint == Pubkey::default() {
        return Err(ChanceryError::AccountKeyMismatch.into());
    }
    if args.issued_token_program != token_program::TOKEN_2022 {
        return Err(ChanceryError::TokenProgramMismatch.into());
    }

    let legacy_mint_configured = args.legacy_token_mint != Pubkey::default();
    let legacy_program_configured = args.legacy_token_program != Pubkey::default();
    if legacy_mint_configured != legacy_program_configured {
        return Err(ChanceryError::TokenProgramMismatch.into());
    }
    if legacy_program_configured && args.legacy_token_program != token_program::SPL_TOKEN {
        return Err(ChanceryError::TokenProgramMismatch.into());
    }
    if legacy_mint_configured && args.legacy_token_mint == args.issued_token_mint {
        return Err(ChanceryError::LegacyMintMismatch.into());
    }
    Ok(())
}

// ─── Handler ──────────────────────────────────────────────────────────────────
pub fn handle<'a>(accounts: &'a [AccountInfo<'a>], args_data: &[u8]) -> ProgramResult {
    if accounts.len() < REQUIRED_ACCOUNT_COUNT {
        return Err(ChanceryError::MissingAccount.into());
    }

    let chancery_config_account_info      = &accounts[CHANCERY_CONFIG];
    let event_authority_account_info      = &accounts[EVENT_AUTHORITY];
    let payer_account_info                = &accounts[PAYER];
    let governance_authority_account_info = &accounts[GOVERNANCE_AUTHORITY];
    let system_program_account_info       = &accounts[SYSTEM_PROGRAM];
    let pause_state_account_info          = &accounts[PAUSE_STATE];
    let program_account_info              = &accounts[PROGRAM_ACCOUNT];
    let programdata_account_info          = &accounts[PROGRAMDATA_ACCOUNT];
    let upgrade_authority_account_info    = &accounts[UPGRADE_AUTHORITY];

    if !pause_state_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    // ── Signers ───────────────────────────────────────────────────────────────
    if !payer_account_info.is_signer {
        return Err(ChanceryError::AccountNotSigner.into());
    }

    assert_external_signer(governance_authority_account_info, &crate::id())?;

    assert_external_signer(upgrade_authority_account_info, &crate::id())?;

    if !chancery_config_account_info.is_writable {
        return Err(ChanceryError::AccountNotWritable.into());
    }

    // ── Bind init to BPF upgrade authority ────────────────────────────────────
    if program_account_info.key != &crate::id()
        || program_account_info.owner != &BPF_LOADER_UPGRADEABLE
        || programdata_account_info.owner != &BPF_LOADER_UPGRADEABLE
    {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    // Program -> ProgramData linkage (offset 4..36 of program data).
    let expected_programdata_key = {
        let data = program_account_info.try_borrow_data()?;

        if data.len() < programdata_layout::PROGRAM_DATA_MIN_LEN {
            return Err(ChanceryError::AuthorityMismatch.into());
        }

        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(
            &data[programdata_layout::PROGRAMDATA_PUBKEY_OFFSET
                ..programdata_layout::PROGRAM_DATA_MIN_LEN],
        );
        Pubkey::new_from_array(bytes)
    };

    if programdata_account_info.key != &expected_programdata_key {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    // ProgramData -> upgrade_authority (Option<Pubkey> at offset 12).
    // tag = 0 means authority revoked; init must run before revocation.
    let expected_upgrade_authority = {
        let data = programdata_account_info.try_borrow_data()?;

        if data.len() < programdata_layout::PROGRAMDATA_DATA_MIN_LEN
            || data[programdata_layout::UPGRADE_AUTHORITY_OPTION_OFFSET] != 1
        {
            return Err(ChanceryError::AuthorityMismatch.into());
        }

        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(
            &data[programdata_layout::UPGRADE_AUTHORITY_PUBKEY_OFFSET
                ..programdata_layout::UPGRADE_AUTHORITY_PUBKEY_OFFSET + 32],
        );
        Pubkey::new_from_array(bytes)
    };

    if upgrade_authority_account_info.key != &expected_upgrade_authority {
        return Err(ChanceryError::AuthorityMismatch.into());
    }

    // ── Args ──────────────────────────────────────────────────────────────────
    let args = InitializeChanceryArgs::try_from_slice(args_data)
        .map_err(|_| ChanceryError::ArgsDeserializationFailed)?;

    assert_initialization_args(&args)?;

    // ── Create PDA account if not yet allocated ───────────────────────────────
    let program_id = crate::id();
    let (expected_key, bump) =
        Pubkey::find_program_address(&[seeds::CHANCERY_CONFIG], &program_id);

    if chancery_config_account_info.key != &expected_key {
        return Err(ChanceryError::InvalidPda.into());
    }

    if chancery_config_account_info.data_is_empty() {
        create_pda_account(
            payer_account_info,
            chancery_config_account_info,
            system_program_account_info,
            &program_id,
            &[seeds::CHANCERY_CONFIG, &[bump]],
            CHANCERY_CONFIG_SIZE,
        )?;
    } else if chancery_config_account_info.data_len() != CHANCERY_CONFIG_SIZE {
        return Err(ChanceryError::AccountDataLengthMismatch.into());
    }

    // ── Derive authority PDAs and event-authority bump on-chain ──────────────
    // mint/freeze authority pubkeys are canonical PDAs - deriving here means
    // they cannot be spoofed via args and the stored value is provably the
    // address every CPI helper signs with.
    let (mint_authority_pda, mint_authority_bump) =
        Pubkey::find_program_address(&[seeds::MINT_AUTHORITY], &program_id);
    let (freeze_authority_pda, _freeze_authority_bump) =
        Pubkey::find_program_address(&[seeds::FREEZE_AUTHORITY], &program_id);
    let (_, reserve_authority_bump) =
        Pubkey::find_program_address(&[seeds::RESERVE_AUTHORITY], &program_id);

    // Authority bumps are cached so hot-path CPIs avoid repeated PDA derivation.
    let (expected_event_authority, event_authority_bump) =
        Pubkey::find_program_address(&[seeds::EVENT_AUTHORITY], &program_id);

    if event_authority_account_info.key != &expected_event_authority {
        return Err(ChanceryError::InvalidPda.into());
    }

    // Derive the domain separator from program-controlled inputs. Off-chain
    // signers recompute the same value from program_id + tag; bumping the
    // tag is a wire-format break.
    let domain_separator = sha256_hashv(&[
        DOMAIN_SEPARATOR_TAG,
        program_id.as_ref(),
    ]).to_bytes();

    // ── Write state ───────────────────────────────────────────────────────────
    // load_uninitialized_mut errors if discriminator already set - one-shot guard.
    let mut chancery_config = ChanceryConfig::load_uninitialized_mut(chancery_config_account_info)?;

    chancery_config.discriminator                         = CHANCERY_CONFIG_DISCRIMINATOR;
    chancery_config.version                               = 1;
    chancery_config.bump                                  = bump;
    chancery_config._pad0                                 = 0;
    chancery_config._pad1                                 = [0u8; 4];
    chancery_config.status_flags                          = status_flag::INITIALIZED;
    // Contained bootstrap only: all authority slots start as governance so the
    // singleton can initialize atomically. Authority-transfer proposal and
    // acceptance both reject assigning a key already used by another role, so
    // the handoff can only converge toward distinct role holders.
    chancery_config.governance_authority                  = *governance_authority_account_info.key;
    chancery_config.operations_authority                  = *governance_authority_account_info.key;
    chancery_config.emergency_authority                   = *governance_authority_account_info.key;
    chancery_config.enforcement_authority                 = *governance_authority_account_info.key;
    chancery_config.insurance_admin_authority             = *governance_authority_account_info.key;
    chancery_config.issued_token_mint                     = args.issued_token_mint;
    chancery_config.issued_token_program                  = args.issued_token_program;
    chancery_config.legacy_token_mint                     = args.legacy_token_mint;
    chancery_config.legacy_token_program                  = args.legacy_token_program;
    chancery_config.mint_authority_pda                    = mint_authority_pda;
    chancery_config.freeze_authority_pda                  = freeze_authority_pda;
    chancery_config.event_sequence_nonce                  = 0;
    chancery_config.domain_separator                      = domain_separator;
    chancery_config.event_authority_bump                  = event_authority_bump;
    chancery_config.mint_authority_bump                   = mint_authority_bump;
    chancery_config.reserve_authority_bump                = reserve_authority_bump;
    chancery_config.authority_bump_cache_version          = AUTHORITY_BUMP_CACHE_VERSION;
    chancery_config._pad_authority_bumps                  = [0u8; 4];
    chancery_config.total_remote_domains_registered       = 0;
    chancery_config.total_signer_sets_registered          = 0;
    chancery_config.allocated_permission_record_slots     = 0;
    chancery_config.total_reserve_destinations_registered = 0;
    chancery_config.total_limit_policies_registered       = 0;
    chancery_config.total_pathway_policies_registered     = 0;
    chancery_config._reserved                             = [0u8; 24];

    // ── Eager PauseState init ─────────────────────────────────────────────────
    let (pause_state_key, pause_state_bump) =
        Pubkey::find_program_address(&[seeds::PAUSE_STATE], &program_id);

    if pause_state_account_info.key != &pause_state_key {
        return Err(ChanceryError::InvalidPda.into());
    }

    create_pda_account(
        payer_account_info,
        pause_state_account_info,
        system_program_account_info,
        &program_id,
        &[seeds::PAUSE_STATE, &[pause_state_bump]],
        PAUSE_STATE_SIZE,
    )?;

    let mut pause_state = PauseState::load_uninitialized_mut(pause_state_account_info)?;
    pause_state.discriminator     = PAUSE_STATE_DISCRIMINATOR;
    pause_state.version           = 1;
    pause_state.bump              = pause_state_bump;
    pause_state._pad0             = [0u8; 5];
    pause_state.global_pause_bits = 0;
    pause_state.reason_code       = 0;
    pause_state._pad1             = [0u8; 4];
    pause_state.activated_by      = Pubkey::default();
    pause_state.activated_at_slot = 0;
    pause_state.expires_at_slot   = 0;
    pause_state._reserved         = [0u8; 32];

    drop(chancery_config);

    // ── Evidence (emitted last, after all state writes) ─────────────────────────
    // Re-borrow the freshly written config to consume a sequence nonce. This is
    // the final mutation before emission; `next_sequence_nonce` advances the
    // counter from its just-initialized value of 0.
    let mut chancery_config_mut  = ChanceryConfig::load_verified_mut(chancery_config_account_info)?;
    let event_authority_bump = chancery_config_mut.event_authority_bump;
    let sequence_nonce       = chancery_config_mut.next_sequence_nonce()?;

    let clock = Clock::get()?;

    emit_chancery_initialized(
        event_authority_account_info,
        event_authority_bump,
        ChanceryInitialized {
            sequence_nonce,
            chancery:                  *chancery_config_account_info.key,
            slot:                      clock.slot,
            unix_timestamp:            clock.unix_timestamp,
            governance_authority:      *governance_authority_account_info.key,
            operations_authority:      *governance_authority_account_info.key,
            emergency_authority:       *governance_authority_account_info.key,
            enforcement_authority:     *governance_authority_account_info.key,
            insurance_admin_authority: *governance_authority_account_info.key,
            issued_token_mint:         args.issued_token_mint,
            issued_token_program:      args.issued_token_program,
            legacy_token_mint:         args.legacy_token_mint,
            legacy_token_program:      args.legacy_token_program,
            mint_authority_pda,
            freeze_authority_pda,
            domain_separator,
        },
    )?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    fn valid_args() -> InitializeChanceryArgs {
        InitializeChanceryArgs {
            issued_token_mint: Pubkey::new_unique(),
            issued_token_program: token_program::TOKEN_2022,
            legacy_token_mint: Pubkey::default(),
            legacy_token_program: Pubkey::default(),
        }
    }

    #[test]
    fn one_shot_initialization_rejects_unrepairable_token_identity() {
        let mut args = valid_args();
        args.issued_token_mint = Pubkey::default();
        assert_eq!(
            assert_initialization_args(&args),
            Err(ChanceryError::AccountKeyMismatch.into()),
        );

        let mut args = valid_args();
        args.issued_token_program = token_program::SPL_TOKEN;
        assert_eq!(
            assert_initialization_args(&args),
            Err(ChanceryError::TokenProgramMismatch.into()),
        );
    }

    #[test]
    fn legacy_identity_must_be_absent_or_fully_coherent() {
        let mut args = valid_args();
        args.legacy_token_mint = Pubkey::new_unique();
        assert_eq!(
            assert_initialization_args(&args),
            Err(ChanceryError::TokenProgramMismatch.into()),
        );

        let mut args = valid_args();
        args.legacy_token_program = token_program::SPL_TOKEN;
        assert_eq!(
            assert_initialization_args(&args),
            Err(ChanceryError::TokenProgramMismatch.into()),
        );

        let mut args = valid_args();
        args.legacy_token_mint = args.issued_token_mint;
        args.legacy_token_program = token_program::SPL_TOKEN;
        assert_eq!(
            assert_initialization_args(&args),
            Err(ChanceryError::LegacyMintMismatch.into()),
        );

        let mut args = valid_args();
        args.legacy_token_mint = Pubkey::new_unique();
        args.legacy_token_program = token_program::TOKEN_2022;
        assert_eq!(
            assert_initialization_args(&args),
            Err(ChanceryError::TokenProgramMismatch.into()),
        );
    }
}
