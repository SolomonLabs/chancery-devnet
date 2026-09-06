use solana_program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ChanceryError {
    // ── Wire / dispatch (0x01xx) ──────────────────────────────────────────────
    #[error("instruction data too short")]
    InstructionDataTooShort                     = 0x0100,

    #[error("unknown module id")]
    UnknownModule                               = 0x0101,

    #[error("unknown instruction id")]
    UnknownInstruction                          = 0x0102,

    #[error("module not enabled")]
    ModuleNotEnabled                            = 0x0103,

    #[error("failed to deserialize instruction args")]
    ArgsDeserializationFailed                   = 0x0104,

    // ── Account / PDA (0x02xx) ────────────────────────────────────────────────
    #[error("account is not writable")]
    AccountNotWritable                          = 0x0200,

    #[error("account is not a signer")]
    AccountNotSigner                            = 0x0201,

    #[error("invalid account owner")]
    AccountOwnerMismatch                        = 0x0202,

    #[error("account key mismatch")]
    AccountKeyMismatch                          = 0x0203,

    #[error("invalid pda derivation")]
    InvalidPda                                  = 0x0204,

    #[error("account already initialized")]
    AlreadyInitialized                          = 0x0205,

    #[error("account not initialized")]
    NotInitialized                              = 0x0206,

    #[error("account data length mismatch")]
    AccountDataLengthMismatch                   = 0x0207,

    #[error("missing required account")]
    MissingAccount                              = 0x0208,

    #[error("account data is not aligned")]
    AccountDataUnaligned                        = 0x0209,

    #[error("unsupported account state version")]
    UnsupportedStateVersion                     = 0x020A,

    #[error("stored account bump does not match canonical pda")]
    StoredBumpMismatch                          = 0x020B,

    #[error("semantically distinct accounts may not alias")]
    AccountAliasNotAllowed                      = 0x020C,

    #[error("account balance is below the rent-exempt minimum")]
    AccountNotRentExempt                        = 0x020D,

    // ── Authority / permission (0x03xx) ───────────────────────────────────────
    #[error("signer lacks required role bits")]
    InsufficientRole                            = 0x0300,

    #[error("permission record not found or expired")]
    PermissionNotFound                          = 0x0301,

    #[error("permission scope mismatch")]
    PermissionScopeMismatch                     = 0x0302,

    #[error("permission expired")]
    PermissionExpired                           = 0x0303,

    #[error("authority mismatch")]
    AuthorityMismatch                           = 0x0304,

    #[error("authority transfer not pending")]
    AuthorityTransferNotPending                 = 0x0305,

    #[error("authority transfer timelock not elapsed")]
    AuthorityTransferTimelockActive             = 0x0306,

    #[error("authority transfer timelock below minimum")]
    AuthorityTransferTimelockBelowMinimum       = 0x0307,

    #[error("permission record is paused")]
    PermissionPaused                            = 0x0308,

    #[error("authority transfer acceptance window elapsed")]
    AuthorityTransferExpired                    = 0x0309,

    #[error("authority transfer proposer superseded by governance rotation")]
    AuthorityTransferProposerSuperseded         = 0x030A,

    #[error("protocol signer pda cannot be used as an external identity")]
    ProtocolSignerIdentityForbidden             = 0x030B,

    #[error("permission role bits contain unknown, retired, or inactive bits")]
    PermissionUnknownRoleBits                   = 0x030C,

    #[error("permission role schema does not match this binary")]
    PermissionRoleSchemaMismatch                = 0x030D,

    #[error("authority role assignment overlaps another authority role")]
    AuthorityRoleOverlap                        = 0x030E,

    // ── Pause / control (0x04xx) ──────────────────────────────────────────────
    #[error("chancery is globally paused")]
    GloballyPaused                              = 0x0400,

    #[error("asset is paused")]
    AssetPaused                                 = 0x0401,

    #[error("pathway is paused")]
    PathwayPaused                               = 0x0402,

    #[error("executor is paused")]
    ExecutorPaused                              = 0x0403,

    #[error("counterparty is paused")]
    CounterpartyPaused                          = 0x0404,

    #[error("freeze record is not frozen")]
    FreezeRecordNotFrozen                       = 0x0405,

    #[error("pause state invalid parameters")]
    PauseStateInvalidParameters                 = 0x0406,

    // ── Asset / mode (0x05xx) ─────────────────────────────────────────────────
    #[error("asset not registered")]
    AssetNotRegistered                          = 0x0500,

    #[error("asset mode does not permit this operation")]
    AssetModeForbids                            = 0x0501,

    #[error("unsupported token extension")]
    UnsupportedTokenExtension                   = 0x0502,

    #[error("token program mismatch")]
    TokenProgramMismatch                        = 0x0503,

    #[error("asset decimals mismatch")]
    AssetDecimalsMismatch                       = 0x0504,

    #[error("asset economics invariant violated")]
    AssetEconomicsInvariantViolated             = 0x0505,

    #[error("issued token mint cannot occupy both pathway asset and issued-token legs")]
    IssuedTokenCannotBeCollateral                = 0x0506,

    // ── Pathway / policy (0x06xx) ────────────────────────────────────
    #[error("pathway policy not found")]
    PathwayPolicyNotFound                       = 0x0600,

    #[error("pathway policy inactive")]
    PathwayPolicyInactive                       = 0x0601,

    #[error("pathway kind mismatch")]
    PathwayKindMismatch                         = 0x0602,

    #[error("executor not permitted on pathway")]
    ExecutorNotPermitted                        = 0x0603,

    #[error("source account violates pathway constraint")]
    SourceAccountForbidden                      = 0x0604,

    #[error("destination account violates pathway constraint")]
    DestinationAccountForbidden                 = 0x0605,

    #[error("settlement policy not found")]
    SettlementPolicyNotFound                    = 0x0606,

    #[error("settlement policy expired")]
    SettlementPolicyExpired                     = 0x0607,

    #[error("settlement mode not allowed by policy")]
    SettlementModeNotAllowed                    = 0x0608,

    #[error("pathway disabled")]
    PathwayDisabled                             = 0x060A,

    #[error("pathway emergency-disabled")]
    PathwayEmergencyDisabled                    = 0x060B,

    #[error("pathway deprecated")]
    PathwayDeprecated                           = 0x060C,

    #[error("pathway not initialized")]
    PathwayNotInitialized                       = 0x060D,

    #[error("immutable field change attempted")]
    ImmutableFieldChange                        = 0x060E,

    #[error("pathway dependency policy missing or inactive")]
    PathwayDependencyMissing                    = 0x060F,

    #[error("settlement policy invalid parameters")]
    SettlementPolicyInvalidParameters           = 0x0610,

    // ── Intent (0x07xx) ───────────────────────────────────────────────────────
    #[error("intent not found")]
    IntentNotFound                              = 0x0700,

    #[error("intent already executed")]
    IntentAlreadyExecuted                       = 0x0701,

    #[error("intent expired")]
    IntentExpired                               = 0x0702,

    #[error("intent not yet valid")]
    IntentNotYetValid                           = 0x0703,

    #[error("intent hash mismatch")]
    IntentHashMismatch                          = 0x0704,

    #[error("intent amount below minimum")]
    IntentAmountBelowMinimum                    = 0x0705,

    #[error("intent not expired")]
    IntentNotExpired                            = 0x0706,

    #[error("settlement intent parameters are not executable")]
    IntentInvalidParameters                     = 0x0707,

    // ── Limits (0x08xx) ───────────────────────────────────────────────────────
    #[error("per-transaction limit breached")]
    PerTxLimitBreached                          = 0x0800,

    #[error("hourly limit breached")]
    HourlyLimitBreached                         = 0x0801,

    #[error("daily limit breached")]
    DailyLimitBreached                          = 0x0802,

    #[error("weekly limit breached")]
    WeeklyLimitBreached                         = 0x0803,

    #[error("monthly limit breached")]
    MonthlyLimitBreached                        = 0x0804,

    #[error("action count limit breached")]
    ActionCountLimitBreached                    = 0x0805,

    #[error("usage window kind invalid")]
    UsageWindowKindInvalid                      = 0x0806,

    #[error("usage window start misaligned")]
    UsageWindowStartMisaligned                  = 0x0807,

    #[error("limit policy invalid parameters")]
    LimitPolicyInvalidParameters                = 0x0808,

    #[error("limit policy scope mismatch")]
    LimitPolicyScopeMismatch                    = 0x0809,

    // NOTE: 0x080A-0x0810 are reserve-block extension codes. Reserve
    // module errors currently live at 0x0Bxx, but these codes are intentionally
    // pinned at 0x080A-0x0810 for wire stability. Limits-specific codes
    // resume at 0x0806+.
    #[error("reserve destination disabled")]
    ReserveDestinationDisabled                  = 0x080A,

    #[error("reserve destination deprecated")]
    ReserveDestinationDeprecated                = 0x080B,

    #[error("reserve destination token mint mismatch")]
    ReserveDestinationTokenMintMismatch         = 0x080C,

    #[error("reserve destination owner mismatch")]
    ReserveDestinationOwnerMismatch             = 0x080D,

    #[error("reserve destination purpose flag required")]
    ReserveDestinationPurposeRequired           = 0x080E,

    #[error("reserve destination purpose flag ambiguous (more than one)")]
    ReserveDestinationPurposeAmbiguous          = 0x080F,

    #[error("reserve destination unknown flag")]
    ReserveDestinationUnknownFlag               = 0x0810,

    #[error("usage window clock regression")]
    UsageWindowClockRegression                  = 0x0811,

    // ── Fees (0x09xx) ─────────────────────────────────────────────────────────
    #[error("fee policy not found")]
    FeePolicyNotFound                           = 0x0900,

    #[error("fee policy expired")]
    FeePolicyExpired                            = 0x0901,

    #[error("fee recipient account mismatch")]
    FeeRecipientMismatch                        = 0x0902,

    #[error("net output would be zero or negative after fee")]
    NetOutputZero                               = 0x0903,

    #[error("fee policy denomination ambiguous: exactly one of FEE_IN_ASSET / FEE_IN_ISSUED_TOKEN must be set")]
    FeePolicyDenominationAmbiguous              = 0x0904,

    #[error("fee policy denomination mismatch: flat fee field set in wrong denomination")]
    FeePolicyDenominationMismatch               = 0x0905,

    #[error("fee recipient token account mint mismatch")]
    FeeRecipientMintMismatch                    = 0x0906,

    #[error("fee recipient token account owner mismatch")]
    FeeRecipientOwnerMismatch                   = 0x0907,

    #[error("fee policy invalid parameters")]
    FeePolicyInvalidParameters                  = 0x0908,

    #[error("fee policy rebate exceeds assessed fee while zero-floor is disabled")]
    FeePolicyNegativeNetFeeNotAllowed           = 0x0909,

    // ── Amounts / math (0x0Axx) ───────────────────────────────────────────────
    #[error("arithmetic overflow")]
    ArithmeticOverflow                          = 0x0A00,

    #[error("arithmetic underflow")]
    ArithmeticUnderflow                         = 0x0A01,

    #[error("amount must be positive")]
    AmountMustBePositive                        = 0x0A05,

    #[error("division by zero")]
    DivisionByZero                              = 0x0A02,

    #[error("amount below minimum")]
    AmountBelowMinimum                          = 0x0A03,

    #[error("amount exceeds maximum")]
    AmountExceedsMaximum                        = 0x0A04,

    // ── Reserve (0x0Bxx) ──────────────────────────────────────────────────────
    #[error("reserve destination not approved")]
    ReserveDestinationNotApproved               = 0x0B00,

    #[error("reserve destination asset mismatch")]
    ReserveDestinationAssetMismatch             = 0x0B01,

    #[error("insufficient reserve balance")]
    InsufficientReserveBalance                  = 0x0B02,

    #[error("reserve withdrawal requires a destination-scoped per-transaction and daily limit policy")]
    ReserveWithdrawalLimitPolicyRequired        = 0x0B03,

    // ── Compartment (0x0Cxx) ──────────────────────────────────────────────────
    #[error("compartment is frozen")]
    CompartmentFrozen                           = 0x0C00,

    #[error("compartment kind does not permit promotion")]
    CompartmentKindForbidsPromotion             = 0x0C01,

    // ── Provenance (0x0Dxx) ───────────────────────────────────────────────────
    #[error("provenance case not open")]
    ProvenanceCaseNotOpen                       = 0x0D00,

    #[error("provenance case already approved")]
    ProvenanceCaseAlreadyApproved               = 0x0D01,

    // ── Enforcement (0x0Exx) ──────────────────────────────────────────────────
    #[error("enforcement case not approved")]
    EnforcementCaseNotApproved                  = 0x0E00,

    // ── Insurance (0x0Fxx) ────────────────────────────────────────────────────
    #[error("insurance policy not found")]
    InsurancePolicyNotFound                     = 0x0F00,

    #[error("insurance policy expired")]
    InsurancePolicyExpired                      = 0x0F01,

    // ── Migration (0x10xx) ────────────────────────────────────────────────────
    #[error("legacy migration not enabled")]
    LegacyMigrationNotEnabled                   = 0x1000,

    #[error("legacy mint mismatch")]
    LegacyMintMismatch                          = 0x1001,

    #[error("legacy migration allowance exceeded")]
    MigrationSupplyExceeded                     = 0x1002,

    // ── Issued token control (0x11xx) ─────────────────────────────────────────
    #[error("issued token control not initialized")]
    IssuedTokenControlNotInitialized            = 0x1100,

    #[error("issued token module not active")]
    IssuedTokenModuleNotActive                  = 0x1101,

    #[error("extension not reserved on mint")]
    ExtensionNotReserved                        = 0x1102,

    #[error("extension already active")]
    ExtensionAlreadyActive                      = 0x1103,

    #[error("forbidden extension detected")]
    ForbiddenExtension                          = 0x1104,

    #[error("event serialization failed")]
    EventSerializationFailed                    = 0x1105,

    #[error("evidence policy configuration is not executable by this program version")]
    EvidencePolicyUnsupportedConfiguration      = 0x1106,

    #[error("evidence policy requires a field not emitted by this settlement event")]
    EvidencePolicyRequiredFieldUnsupported      = 0x1107,

    // ── Cross-chain (0x12xx, +docs 14, 15) ────────────────────────────────────
    #[error("remote domain policy not found")]
    RemoteDomainPolicyNotFound                  = 0x1200,

    #[error("remote domain policy expired")]
    RemoteDomainPolicyExpired                   = 0x1201,

    #[error("remote domain inactive")]
    RemoteDomainInactive                        = 0x1202,

    #[error("remote domain paused for this operation")]
    RemoteDomainPaused                          = 0x1203,

    #[error("daughter contract mismatch")]
    DaughterContractMismatch                    = 0x1204,

    #[error("domain separator mismatch")]
    DomainSeparatorMismatch                     = 0x1205,

    #[error("source / destination domain mismatch")]
    DomainMismatch                              = 0x1206,

    #[error("chain kind not supported")]
    ChainKindNotSupported                       = 0x1207,

    #[error("message hash mismatch")]
    MessageHashMismatch                         = 0x1208,

    /// Retired pre-deployment with the per-message replay PDA: replays now
    /// fail `RemoteNonceMismatch` on strict nonce equality. Code pinned for
    /// wire stability.
    #[error("message already consumed")]
    MessageAlreadyConsumed                      = 0x1209,

    #[error("message expired")]
    MessageExpired                              = 0x120A,

    #[error("message kind not supported")]
    MessageKindNotSupported                     = 0x120B,

    #[error("invalid cross-chain message kind discriminant")]
    InvalidMessageKind                          = 0x122A,

    #[error("remote nonce gap")]
    RemoteNonceGap                              = 0x120C,

    #[error("remote nonce mismatch")]
    RemoteNonceMismatch                         = 0x120D,

    #[error("signer set not found")]
    SignerSetNotFound                           = 0x120E,

    #[error("signer set expired")]
    SignerSetExpired                            = 0x120F,

    #[error("signer set id mismatch")]
    SignerSetIdMismatch                         = 0x1210,

    #[error("signer set inactive")]
    SignerSetInactive                           = 0x1211,

    #[error("attestation threshold not met")]
    AttestationThresholdNotMet                  = 0x1212,

    #[error("attestation signer not in set")]
    AttestationSignerNotInSet                   = 0x1213,

    #[error("attestation signature invalid")]
    AttestationSignatureInvalid                 = 0x1214,

    #[error("attestation duplicate signer")]
    AttestationDuplicateSigner                  = 0x1215,

    #[error("recovered address mismatch")]
    RecoveredAddressMismatch                    = 0x1216,

    #[error("merkle proof invalid")]
    MerkleProofInvalid                          = 0x1217,

    #[error("signer root mismatch")]
    SignerRootMismatch                          = 0x1218,

    #[error("finality proof missing")]
    FinalityProofMissing                        = 0x1219,

    #[error("per-message limit breached")]
    PerMessageLimitBreached                     = 0x121A,

    #[error("per-day cross-chain limit breached")]
    CrossChainPerDayLimitBreached               = 0x121B,

    #[error("invalid signer set parameters")]
    InvalidSignerSetParameters                  = 0x121C,

    #[error("merkle proof depth exceeded signer set max")]
    MerkleProofDepthExceeded                    = 0x121D,

    #[error("remote asset binding mismatch")]
    RemoteAssetBindingMismatch                  = 0x121E,

    #[error("remote asset forbidden in mint mode")]
    RemoteAssetForbiddenInMintMode              = 0x121F,

    #[error("remote asset required in release mode")]
    RemoteAssetRequiredInReleaseMode            = 0x1220,

    #[error("remote issued token required")]
    RemoteIssuedTokenRequired                   = 0x1221,

    #[error("remote domain mode mismatch")]
    RemoteDomainModeMismatch                    = 0x1222,

    #[error("invalid remote domain mode")]
    InvalidRemoteDomainMode                     = 0x1223,

    // 0x1224 reserved (was extensions' duplicate `MessageAlreadyConsumed`;
    // canonical value lives at 0x1209). Do not reuse.

    #[error("signer set max proof depth must be non-zero")]
    SignerSetMaxProofDepthZero                  = 0x1225,

    #[error("message expiry exceeds remote domain policy window")]
    MessageExpiryWindowExceeded                 = 0x1226,
    #[error("attestation signature non-canonical")]
    AttestationSignatureNonCanonical            = 0x1227,

    /// `expire_inbound_message`: the message is not dead - expiry is zero
    /// (never expires) or still in the future. Inverse of `MessageExpired`.
    #[error("message not expired")]
    MessageNotExpired                           = 0x1228,

    // 0x1229 intentionally left unassigned (mirrors the 0x1224 reservation
    // convention). Do not reuse without a spec entry.

    #[error("reclaim digest mismatch")]
    ReclaimDigestMismatch                       = 0x122B,

    /// `reclaim_expired_outbound` plausibility floor (spec 15 §15.5): the
    /// claimed `source_nonce` is not strictly below the corridor's
    /// `next_outbound_nonce`, so Chancery cannot have emitted it.
    #[error("reclaim source nonce not below next outbound nonce")]
    ReclaimSourceNonceNotEmitted                = 0x122C,

    /// The permanent `OutboundReclaimRecord` for this epoch-free content hash
    /// already exists: the emission was reclaimed before. Single-shot.
    #[error("outbound emission already reclaimed")]
    OutboundAlreadyReclaimed                    = 0x122D,

    /// `reclaim_expired_outbound`: only `OUTBOUND_*` burn kinds are
    /// reclaimable by re-mint; lock kinds have a different refund shape.
    #[error("message kind is not a reclaimable outbound burn kind")]
    ReclaimMessageKindNotOutboundBurn           = 0x122E,

    /// The attested u128 amount does not fit Solana's u64 token domain.
    #[error("reclaim amount exceeds u64 token amount domain")]
    ReclaimAmountExceedsTokenDomain             = 0x122F,

    /// Epoch-free content hash recomputed from the supplied preimage does
    /// not match the caller-provided value (spec 15 §15.4).
    #[error("epoch-free content hash mismatch")]
    EpochFreeContentHashMismatch                = 0x1230,

    #[error("cross-chain attestation exceeds executable resource bounds")]
    AttestationResourceLimitExceeded            = 0x1231,

    /// A hash-critical remote endpoint anchor is all-zero. The remote issued
    /// token has its own mode-scoped `RemoteIssuedTokenRequired` error.
    #[error("remote-domain endpoint identity anchor is zero")]
    RemoteIdentityAnchorZero                    = 0x1232,

    #[error("invalid inbound message retirement reason")]
    InvalidInboundRetirementReason              = 0x1233,

    #[error("live cross-chain corridors require a non-expiring signer set")]
    LiveCorridorSignerSetMustNotExpire           = 0x1234,

    // ── Pending config change + risk class (0x13xx) ───────────────────
    #[error("config change requires accepted pending change with timelock")]
    ConfigChangeRequiresTimelock                = 0x1300,

    #[error("pending config change not yet executable")]
    ConfigChangeTimelockNotElapsed              = 0x1301,

    #[error("pending config change expired")]
    ConfigChangeExpired                         = 0x1302,

    #[error("pending config change hash mismatch")]
    ConfigChangeHashMismatch                    = 0x1303,

    #[error("pending config change wrong target")]
    ConfigChangeWrongTarget                     = 0x1304,

    #[error("pending config change not in accepted state")]
    ConfigChangeNotAccepted                     = 0x1305,

    #[error("pending config change already accepted")]
    ConfigChangeAlreadyAccepted                 = 0x1306,

    #[error("pending config change already cancelled")]
    ConfigChangeAlreadyCancelled                = 0x1307,

    #[error("pending config change kind mismatch")]
    ConfigChangeKindMismatch                    = 0x1308,

    #[error("pending config change risk class mismatch")]
    ConfigChangeRiskClassMismatch               = 0x1309,

    #[error("config change widening requires governance signature")]
    ConfigChangeWideningRequiresGovernance      = 0x130A,

    #[error("config change invalid risk class enum value")]
    ConfigChangeInvalidRiskClass                = 0x130B,

    #[error("cross-chain pathway id must equal sha256(kind || asset_mint || issued_token_mint)")]
    CrossChainPathwayIdNotCanonical             = 0x130C,

    #[error("config change not expired")]
    ConfigChangeNotExpired                      = 0x130D,

    #[error("pending config change proposer superseded by governance rotation")]
    ConfigChangeProposerSuperseded              = 0x130E,

    // ── Module activation (0x14xx) ────────────────────────────────────
    #[error("module is administrative-only; hot path disabled")]
    ModuleAdminOnly                             = 0x1400,

    #[error("module is emergency-disabled")]
    ModuleEmergencyDisabled                     = 0x1401,

    #[error("module is deprecated")]
    ModuleDeprecated                            = 0x1402,

    #[error("module cannot be disabled (core/evidence/control/events_cpi)")]
    ModuleUndisableable                         = 0x1403,

    #[error("module activation state not initialized")]
    ModuleActivationStateNotInitialized         = 0x1404,

    #[error("invalid module status enum value")]
    InvalidModuleStatus                         = 0x1405,

    #[error("module deprecation review must be proposed after the disable transition")]
    ModuleDeprecationReviewPrecedesDisable      = 0x1406,

    // ── Permission grant hardening (0x15xx) ───────────────────────────
    #[error("dangerous permission requires narrow scope")]
    DangerousPermissionRequiresNarrowScope      = 0x1500,

    #[error("dangerous permission requires expiry")]
    DangerousPermissionRequiresExpiry           = 0x1501,

    #[error("permission scope broadening forbidden for non-admin grantor")]
    PermissionScopeBroadeningFailed             = 0x1502,

    #[error("permission expiry exceeds grantor's expiry")]
    PermissionExpiryExtensionExceedsGrantor     = 0x1503,

    #[error("permission scope broadening forbidden")]
    PermissionScopeWideningForbidden            = 0x1504,

    #[error("permission unknown scope kind")]
    PermissionUnknownScope                      = 0x1505,

    #[error("authority-adjacent role bits are not grantable by non-admin grantors")]
    RestrictedRoleNotGrantableByNonAdmin        = 0x1506,

    #[error("permission flags contain unknown or reserved bits")]
    PermissionUnknownFlags                      = 0x1507,

    #[error("new permission record must grant at least one role")]
    PermissionEmptyGrant                        = 0x1508,

    // ── Issued token deployment (0x16xx) ──────────────────────────────
    #[error("issued token deployment not verified")]
    IssuedTokenDeploymentNotVerified            = 0x1600,

    #[error("issued token mint authority absent or invalid")]
    IssuedTokenMintAuthorityInvalid             = 0x1601,

    #[error("issued token freeze authority absent or invalid")]
    IssuedTokenFreezeAuthorityInvalid           = 0x1602,

    #[error("issued token observed extension outside reserved mask")]
    IssuedTokenObservedExtensionOutsideReserved = 0x1603,

    #[error("issued token forbidden default extension active")]
    IssuedTokenForbiddenDefaultExtensionActive  = 0x1604,

    #[error("issued token reserved extension authority is not the expected chancery pda")]
    IssuedTokenExtensionAuthorityInvalid        = 0x1605,

    // ── Extension observation (0x17xx) ────────────────────────────────
    /// Issue #40: `Token2022ExtensionsStale` (existing wire code `0x1700`).
    #[error("extension observation stale")]
    ExtensionObservationStale                   = 0x1700,

    #[error("token-2022 unknown TLV extension tag")]
    Token2022UnknownExtensionTag                = 0x1701,

    #[error("token-2022 TLV malformed (length overrun)")]
    Token2022TlvMalformed                       = 0x1702,

    #[error("forbidden extension active on collateral or issued mint")]
    ForbiddenExtensionActive                    = 0x1703,

    #[error("declared extension mask does not match on-chain mint TLV")]
    Token2022ObservedMaskMismatch               = 0x1704,

    #[error("extension observation slot is in the future")]
    ExtensionObservationFuture                  = 0x1705,
}

impl From<ChanceryError> for ProgramError {
    fn from(e: ChanceryError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
