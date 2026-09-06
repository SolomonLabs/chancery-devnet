// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      test
// File:          ChanceryProgram.integration.generated.test.ts
//
// Source IDL:    programs/chancery/idl/idl.json
// Output Dir:    clients/ts
//
// Introspection: disabled
// Confidence:    medium
//
// To regenerate: Run the IDL generator with the same options, or use
//                `idl-generator --use-last` to repeat the last run.
//
// ============================================================================
// @end-generated-idl
import { assert, describe, it } from "@solomon-labs/testing";

import { ChanceryProgram } from "../src/ChanceryProgram";
import * as accountExports from "../src/accounts";
import { PROGRAM_ID } from "../src/constants";
import * as eventExports from "../src/events";
import * as instructionExports from "../src/instructions";

describe("ChanceryProgram generated client integration", () => {
    it("exports every canonical IDL instruction, account, and event", () => {
        assert.equal(ChanceryProgram.programId.toBase58(), PROGRAM_ID.toBase58());
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "AcceptAuthorityTransferInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "InitializeChanceryInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "ProposeAuthorityTransferInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "RegisterAssetInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "SetAssetModeInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "SetAssetModeWithPendingChangeInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "UpdateAssetConfigInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "UpdateAssetConfigWithPendingChangeInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "EmitInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "UpsertPermissionInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "RevokePermissionInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "UpsertPermissionWithPendingChangeInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "RegisterPathwayPolicyInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "UpdatePathwayPolicyInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "UpdatePathwayPolicyWithPendingChangeInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "CreateSettlementIntentInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "CancelSettlementIntentInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "CloseExpiredSettlementIntentInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "MintDelegatedInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "MintDirectInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "MintTrilateralInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "RedeemDelegatedInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "RedeemDirectInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "RedeemTrilateralInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "RegisterSettlementPolicyInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "RegisterLimitPolicyInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "UpdateLimitPolicyInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "UpdateLimitPolicyWithPendingChangeInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "RegisterEvidencePolicyInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "UpdateEvidencePolicyInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "UpdateEvidencePolicyWithPendingChangeInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "RegisterFeePolicyInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "UpdateFeePolicyInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "UpdateFeePolicyWithPendingChangeInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "RegisterReserveDestinationWithPendingInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "SetReserveDestinationStatusInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "SetReserveDestinationStatusWithPendingInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "WithdrawReserveInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "AcceptConfigChangeInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "CancelConfigChangeInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "CloseExpiredConfigChangeInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "FreezeIssuedTokenAccountInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "InitializeModuleActivationStateInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "ProposeConfigChangeInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "SetAssetPauseInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "SetCounterpartyPauseInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "SetExecutorPauseInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "SetGlobalPauseInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "SetModuleStatusInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "SetModuleStatusWithPendingChangeInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "SetPathwayPauseInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "ThawIssuedTokenAccountInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "EnableLegacyMigrationInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "MigrateLegacyToToken2022Instruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "InitializeIssuedTokenControlInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "RefreshAssetExtensionObservationInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "VerifyIssuedTokenDeploymentInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "RefreshIssuedTokenExtensionObservationInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "ConsumeInboundMessageInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "EmitOutboundMessageInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "ExpireInboundMessageInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "ReclaimExpiredOutboundInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "RelaxRemoteDomainPauseInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "RegisterCrossChainSignerSetInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "RegisterRemoteDomainPolicyInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "RestrictRemoteDomainPauseInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "RotateCrossChainSignerSetInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "UpdateRemoteDomainPolicyInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "UpdateRemoteDomainPolicyWithPendingChangeInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(instructionExports, "RelaxRemoteDomainPauseWithPendingChangeInstruction"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "AssetPauseState"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "BasicFreezeRecord"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "ModuleActivationState"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "PauseState"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "PendingConfigChange"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "AssetConfig"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "AuthorityTransfer"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "ChanceryConfig"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "CrossChainSignerSet"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "OutboundReclaimRecord"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "RemoteDomainPolicy"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "RemoteNonce"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "EvidencePolicy"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "FeePolicy"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "IssuedTokenControl"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "LimitPolicy"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "UsageWindow"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "LegacyMigrationConfig"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "PathwayPolicy"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "PermissionRecord"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "ReserveDestination"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "SettlementIntent"));
        assert.ok(Object.prototype.hasOwnProperty.call(accountExports, "SettlementPolicy"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "AssetRegisteredLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "AssetConfigUpdatedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "AssetModeChangedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "AssetExtensionRefreshedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "AuthorityTransferProposedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "AuthorityTransferAcceptedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "AuthorityTransferCancelledLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "ChanceryInitializedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "ModuleActivationStateInitializedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "ConfigChangeProposedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "ConfigChangeAcceptedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "ConfigChangeCancelledLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "ConfigChangeExpiredLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "RemoteDomainPolicyRegisteredLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "RemoteDomainPolicyUpdatedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "RemoteDomainPauseRelaxedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "CrossChainSignerSetRegisteredLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "CrossChainSignerSetRotatedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "CrossChainOutboundEmittedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "CrossChainInboundConsumedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "CrossChainInboundExpiredLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "CrossChainOutboundReclaimedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "EvidencePolicyRegisteredLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "EvidencePolicyUpdatedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "FeePolicyRegisteredLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "FeePolicyUpdatedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "IssuedTokenControlInitializedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "IssuedTokenDeploymentVerifiedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "IssuedTokenControlChangeLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "BasicTokenFreezeLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "BasicTokenThawLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "IssuedTokenExtensionObservationRefreshedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "LimitPolicyRegisteredLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "UsageWindowInitializedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "UsageWindowRolledLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "LimitPolicyUpdatedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "LegacyMigrationEnabledLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "LegacyMigrationLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "ModuleStatusChangedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "PathwayPolicyRegisteredLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "PathwayPolicyUpdatedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "PathwayStatusChangedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "PauseStateChangeLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "PermissionUpsertedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "PermissionRevokedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "ReserveDestinationRegisteredLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "ReserveDestinationStatusChangedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "ReserveWithdrawalLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "SettlementPolicyRegisteredLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "SettlementIntentCreatedLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "SettlementIntentExpiredLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "SettlementIntentCancelledLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "SettlementMintLog"));
        assert.ok(Object.prototype.hasOwnProperty.call(eventExports, "SettlementRedeemLog"));
    });
});
