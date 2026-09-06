// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      test
// File:          ChanceryProgram.accounts.generated.test.ts
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

import { AssetPauseState } from "../src/accounts/AssetPauseState";
import { BasicFreezeRecord } from "../src/accounts/BasicFreezeRecord";
import { ModuleActivationState } from "../src/accounts/ModuleActivationState";
import { PauseState } from "../src/accounts/PauseState";
import { PendingConfigChange } from "../src/accounts/PendingConfigChange";
import { AssetConfig } from "../src/accounts/AssetConfig";
import { AuthorityTransfer } from "../src/accounts/AuthorityTransfer";
import { ChanceryConfig } from "../src/accounts/ChanceryConfig";
import { CrossChainSignerSet } from "../src/accounts/CrossChainSignerSet";
import { OutboundReclaimRecord } from "../src/accounts/OutboundReclaimRecord";
import { RemoteDomainPolicy } from "../src/accounts/RemoteDomainPolicy";
import { RemoteNonce } from "../src/accounts/RemoteNonce";
import { EvidencePolicy } from "../src/accounts/EvidencePolicy";
import { FeePolicy } from "../src/accounts/FeePolicy";
import { IssuedTokenControl } from "../src/accounts/IssuedTokenControl";
import { LimitPolicy } from "../src/accounts/LimitPolicy";
import { UsageWindow } from "../src/accounts/UsageWindow";
import { LegacyMigrationConfig } from "../src/accounts/LegacyMigrationConfig";
import { PathwayPolicy } from "../src/accounts/PathwayPolicy";
import { PermissionRecord } from "../src/accounts/PermissionRecord";
import { ReserveDestination } from "../src/accounts/ReserveDestination";
import { SettlementIntent } from "../src/accounts/SettlementIntent";
import { SettlementPolicy } from "../src/accounts/SettlementPolicy";
import { ChanceryProgram } from "../src";

describe("ChanceryProgram accounts auto generated tests", () => {
    it("has static programId", () => {
        assert(ChanceryProgram.programId);
    });

    describe("accounts", () => {
        it("has schema for AssetPauseState", () => {
            const schema = AssetPauseState.getSchema();
            assert(AssetPauseState.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.pad0);
            assert(schema.assetMint);
            assert(schema.assetPauseBits);
            assert(schema.reasonCode);
            assert(schema.pad1);
            assert(schema.activatedBy);
            assert(schema.activatedAtSlot);
            assert(schema.expiresAtSlot);
            assert(schema.reserved);
        });

        it("has schema for BasicFreezeRecord", () => {
            const schema = BasicFreezeRecord.getSchema();
            assert(BasicFreezeRecord.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.status);
            assert(schema.reasonCode);
            assert(schema.issuedTokenAccount);
            assert(schema.issuedTokenMint);
            assert(schema.freezeAuthorityPda);
            assert(schema.frozenBy);
            assert(schema.thawedBy);
            assert(schema.frozenAtSlot);
            assert(schema.thawedAtSlot);
            assert(schema.lastEventSequenceNonce);
            assert(schema.freezeFlags);
            assert(schema.thawFlags);
            assert(schema.rentRefundRecipient);
            assert(schema.reserved);
        });

        it("has schema for ModuleActivationState", () => {
            const schema = ModuleActivationState.getSchema();
            assert(ModuleActivationState.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.pad0);
            assert(schema.moduleStatuses);
            assert(schema.lastUpdatedBy);
            assert(schema.lastUpdatedAtSlot);
            assert(schema.lastEventSequenceNonce);
            assert(schema.reserved);
        });

        it("has schema for PauseState", () => {
            const schema = PauseState.getSchema();
            assert(PauseState.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.pad0);
            assert(schema.globalPauseBits);
            assert(schema.reasonCode);
            assert(schema.pad1);
            assert(schema.activatedBy);
            assert(schema.activatedAtSlot);
            assert(schema.expiresAtSlot);
            assert(schema.reserved);
        });

        it("has schema for PendingConfigChange", () => {
            const schema = PendingConfigChange.getSchema();
            assert(PendingConfigChange.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.status);
            assert(schema.riskClass);
            assert(schema.pad0);
            assert(schema.changeId);
            assert(schema.changeKind);
            assert(schema.pad1);
            assert(schema.targetAccount);
            assert(schema.oldValueHash);
            assert(schema.newValueHash);
            assert(schema.proposedBy);
            assert(schema.proposerNonce);
            assert(schema.executableAfterUnixTimestamp);
            assert(schema.expiresAtUnixTimestamp);
            assert(schema.proposedAtSlot);
            assert(schema.acceptedAtSlot);
            assert(schema.cancelledAtSlot);
            assert(schema.consumedAtSlot);
            assert(schema.lastEventSequenceNonce);
            assert(schema.rentRefundRecipient);
            assert(schema.reserved);
        });

        it("has schema for AssetConfig", () => {
            const schema = AssetConfig.getSchema();
            assert(AssetConfig.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.decimals);
            assert(schema.mode);
            assert(schema.pad0);
            assert(schema.assetFlags);
            assert(schema.assetMint);
            assert(schema.assetTokenProgram);
            assert(schema.primaryReserveCompartmentId);
            assert(schema.approvedExtensionMask);
            assert(schema.observedExtensionMask);
            assert(schema.depositRateE9);
            assert(schema.redeemRateE9);
            assert(schema.minimumDepositAmount);
            assert(schema.minimumRedeemAmount);
            assert(schema.maximumSingleSettlementAmount);
            assert(schema.statusFlags);
            assert(schema.forbiddenExtensionMask);
            assert(schema.requiredModuleMask);
            assert(schema.extensionObservedAtSlot);
            assert(schema.maxExtensionObservationAgeSlots);
            assert(schema.reserved);
        });

        it("has schema for AuthorityTransfer", () => {
            const schema = AuthorityTransfer.getSchema();
            assert(AuthorityTransfer.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.roleKind);
            assert(schema.pad0);
            assert(schema.oldAuthority);
            assert(schema.proposedAuthority);
            assert(schema.proposedAtSlot);
            assert(schema.executableAfterSlot);
            assert(schema.proposingGovernance);
            assert(schema.expiresAtSlot);
            assert(schema.reserved);
        });

        it("has schema for ChanceryConfig", () => {
            const schema = ChanceryConfig.getSchema();
            assert(ChanceryConfig.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.pad0);
            assert(schema.pad1);
            assert(schema.statusFlags);
            assert(schema.governanceAuthority);
            assert(schema.operationsAuthority);
            assert(schema.emergencyAuthority);
            assert(schema.enforcementAuthority);
            assert(schema.insuranceAdminAuthority);
            assert(schema.issuedTokenMint);
            assert(schema.issuedTokenProgram);
            assert(schema.legacyTokenMint);
            assert(schema.legacyTokenProgram);
            assert(schema.mintAuthorityPda);
            assert(schema.freezeAuthorityPda);
            assert(schema.eventSequenceNonce);
            assert(schema.domainSeparator);
            assert(schema.eventAuthorityBump);
            assert(schema.mintAuthorityBump);
            assert(schema.reserveAuthorityBump);
            assert(schema.authorityBumpCacheVersion);
            assert(schema.padAuthorityBumps);
            assert(schema.totalRemoteDomainsRegistered);
            assert(schema.totalSignerSetsRegistered);
            assert(schema.allocatedPermissionRecordSlots);
            assert(schema.totalReserveDestinationsRegistered);
            assert(schema.totalLimitPoliciesRegistered);
            assert(schema.totalPathwayPoliciesRegistered);
            assert(schema.reserved);
        });

        it("has schema for CrossChainSignerSet", () => {
            const schema = CrossChainSignerSet.getSchema();
            assert(CrossChainSignerSet.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.threshold);
            assert(schema.signerCount);
            assert(schema.pad0);
            assert(schema.validAfterUnixTimestamp);
            assert(schema.expiresAtUnixTimestamp);
            assert(schema.statusFlags);
            assert(schema.signerSetId);
            assert(schema.signerRoot);
            assert(schema.createdBy);
            assert(schema.reserved);
        });

        it("has schema for OutboundReclaimRecord", () => {
            const schema = OutboundReclaimRecord.getSchema();
            assert(OutboundReclaimRecord.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.messageKind);
            assert(schema.remoteChainKind);
            assert(schema.retirementReason);
            assert(schema.pad0);
            assert(schema.remoteDomainId);
            assert(schema.sourceNonce);
            assert(schema.amountWords);
            assert(schema.reclaimedAtSlot);
            assert(schema.reclaimedAtUnixTimestamp);
            assert(schema.epochFreeContentHash);
            assert(schema.reclaimDigest);
            assert(schema.sender);
            assert(schema.attestingSignerSetId);
            assert(schema.reserved);
        });

        it("has schema for RemoteDomainPolicy", () => {
            const schema = RemoteDomainPolicy.getSchema();
            assert(RemoteDomainPolicy.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.remoteChainKind);
            assert(schema.minimumAttestationThreshold);
            assert(schema.pad0);
            assert(schema.remoteDomainId);
            assert(schema.requiredFinalityDepth);
            assert(schema.messageExpirySeconds);
            assert(schema.perMessageMaximum);
            assert(schema.perDayMaximum);
            assert(schema.statusFlags);
            assert(schema.updatedAtSlot);
            assert(schema.remoteDomainSeparator);
            assert(schema.remoteChanceryContract);
            assert(schema.remoteIssuedToken);
            assert(schema.signerSetId);
            assert(schema.createdBy);
            assert(schema.mode);
            assert(schema.padMode);
            assert(schema.remoteAsset);
            assert(schema.localAssetMint);
            assert(schema.reserved);
        });

        it("has schema for RemoteNonce", () => {
            const schema = RemoteNonce.getSchema();
            assert(RemoteNonce.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.pad0);
            assert(schema.remoteDomainId);
            assert(schema.nextInboundNonce);
            assert(schema.nextOutboundNonce);
            assert(schema.scopeKey);
            assert(schema.lastConsumedMessageHash);
            assert(schema.lastEmittedMessageHash);
            assert(schema.reserved);
        });

        it("has schema for EvidencePolicy", () => {
            const schema = EvidencePolicy.getSchema();
            assert(EvidencePolicy.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.allowFreeformCounterpartyFields);
            assert(schema.pad0);
            assert(schema.evidencePolicyId);
            assert(schema.requiredFieldMask);
            assert(schema.counterpartyReportingSchemaHash);
            assert(schema.maximumFreeformFieldCount);
            assert(schema.maximumFreeformValueBytes);
            assert(schema.pad1);
            assert(schema.retentionFlags);
            assert(schema.reserved);
        });

        it("has schema for FeePolicy", () => {
            const schema = FeePolicy.getSchema();
            assert(FeePolicy.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.feeRecipientPolicy);
            assert(schema.roundingMode);
            assert(schema.netFeeFloorZero);
            assert(schema.pad0);
            assert(schema.feePolicyId);
            assert(schema.feePolicyFlags);
            assert(schema.flatFeeInAsset);
            assert(schema.flatFeeInIssuedToken);
            assert(schema.percentFeeBps);
            assert(schema.pad1);
            assert(schema.feeCapAmount);
            assert(schema.minimumFeeAmount);
            assert(schema.rebateFlatAmount);
            assert(schema.rebateBps);
            assert(schema.pad2);
            assert(schema.rebateCapAmount);
            assert(schema.feeRecipientKey);
            assert(schema.effectiveFromUnixTimestamp);
            assert(schema.effectiveUntilUnixTimestamp);
            assert(schema.reserved);
        });

        it("has schema for IssuedTokenControl", () => {
            const schema = IssuedTokenControl.getSchema();
            assert(IssuedTokenControl.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.pad0);
            assert(schema.issuedTokenMint);
            assert(schema.issuedTokenProgram);
            assert(schema.reservedMintExtensionMask);
            assert(schema.activeMintExtensionMask);
            assert(schema.reservedAccountExtensionMask);
            assert(schema.activeAccountExtensionMask);
            assert(schema.controlFlags);
            assert(schema.mintAuthorityPda);
            assert(schema.freezeAuthorityPda);
            assert(schema.closeMintAuthorityPda);
            assert(schema.transferHookAuthorityPda);
            assert(schema.permanentDelegateAuthorityPda);
            assert(schema.metadataPointerAuthorityPda);
            assert(schema.metadataUpdateAuthorityPda);
            assert(schema.pauseAuthorityPda);
            assert(schema.confidentialTransferAuthorityPda);
            assert(schema.defaultAccountStateAuthorityPda);
            assert(schema.hookProgramId);
            assert(schema.permanentDelegate);
            assert(schema.metadataAddress);
            assert(schema.lastConfiguredAtSlot);
            assert(schema.configuredBy);
            assert(schema.extensionObservedAtSlot);
            assert(schema.maxExtensionObservationAgeSlots);
            assert(schema.reserved);
        });

        it("has schema for LimitPolicy", () => {
            const schema = LimitPolicy.getSchema();
            assert(LimitPolicy.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.scopeKind);
            assert(schema.pad0);
            assert(schema.limitPolicyId);
            assert(schema.scopeKey);
            assert(schema.perTransactionMaximum);
            assert(schema.perHourMaximum);
            assert(schema.perDayMaximum);
            assert(schema.perSevenDayMaximum);
            assert(schema.perThirtyDayMaximum);
            assert(schema.maximumActionsPerHour);
            assert(schema.maximumActionsPerDay);
            assert(schema.reservedBreachFlags);
            assert(schema.statusFlags);
            assert(schema.reserved);
        });

        it("has schema for UsageWindow", () => {
            const schema = UsageWindow.getSchema();
            assert(UsageWindow.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.windowKind);
            assert(schema.pad0);
            assert(schema.scopeHash);
            assert(schema.windowStartUnixTimestamp);
            assert(schema.grossIn);
            assert(schema.grossOutputAmount);
            assert(schema.netFlow);
            assert(schema.actionCount);
            assert(schema.pad1);
            assert(schema.rentRefundRecipient);
            assert(schema.reserved);
        });

        it("has schema for LegacyMigrationConfig", () => {
            const schema = LegacyMigrationConfig.getSchema();
            assert(LegacyMigrationConfig.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.pad0);
            assert(schema.migrationFlags);
            assert(schema.legacyProgram);
            assert(schema.legacyMint);
            assert(schema.migrationEnabledAtSlot);
            assert(schema.migratedTotal);
            assert(schema.legacySupplySnapshot);
            assert(schema.migratedLegacyTotal);
            assert(schema.reserved);
        });

        it("has schema for PathwayPolicy", () => {
            const schema = PathwayPolicy.getSchema();
            assert(PathwayPolicy.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.pathwayKind);
            assert(schema.pad0);
            assert(schema.pathwayId);
            assert(schema.assetMint);
            assert(schema.issuedTokenMint);
            assert(schema.allowedProgramMask);
            assert(schema.allowedInstructionMask);
            assert(schema.designatedExecutor);
            assert(schema.sourceAccountPolicy);
            assert(schema.destinationAccountPolicy);
            assert(schema.reserveCompartmentPolicyId);
            assert(schema.limitPolicyId);
            assert(schema.evidencePolicyId);
            assert(schema.feePolicyId);
            assert(schema.insurancePolicyId);
            assert(schema.statusFlags);
            assert(schema.requiredIssuedTokenModuleMask);
            assert(schema.requiredCollateralModuleMask);
            assert(schema.forbiddenIssuedTokenExtensionMask);
            assert(schema.forbiddenCollateralExtensionMask);
            assert(schema.assetMintLimitPolicyId);
            assert(schema.assetRedeemLimitPolicyId);
            assert(schema.counterpartyLimitPolicyId);
            assert(schema.executorLimitPolicyId);
            assert(schema.reserved);
        });

        it("has schema for PermissionRecord", () => {
            const schema = PermissionRecord.getSchema();
            assert(PermissionRecord.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.scopeKind);
            assert(schema.pad0);
            assert(schema.subject);
            assert(schema.scopeKey);
            assert(schema.roleBits);
            assert(schema.permissionFlags);
            assert(schema.issuedAtUnixTimestamp);
            assert(schema.expiryUnixTimestamp);
            assert(schema.grantedBy);
            assert(schema.roleSchemaVersion);
            assert(schema.pad1);
            assert(schema.permissionGeneration);
            assert(schema.reserved);
        });

        it("has schema for ReserveDestination", () => {
            const schema = ReserveDestination.getSchema();
            assert(ReserveDestination.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.status);
            assert(schema.pad0);
            assert(schema.assetMint);
            assert(schema.destinationTokenAccount);
            assert(schema.destinationOwner);
            assert(schema.destinationFlags);
            assert(schema.approvedBy);
            assert(schema.withdrawalLimitPolicyId);
            assert(schema.reserved);
        });

        it("has schema for SettlementIntent", () => {
            const schema = SettlementIntent.getSchema();
            assert(SettlementIntent.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.status);
            assert(schema.settlementMode);
            assert(schema.settlementAction);
            assert(schema.pad0);
            assert(schema.intentId);
            assert(schema.principalA);
            assert(schema.principalB);
            assert(schema.executor);
            assert(schema.assetMint);
            assert(schema.issuedTokenMint);
            assert(schema.assetAmount);
            assert(schema.issuedTokenAmount);
            assert(schema.minimumAssetAmount);
            assert(schema.minimumIssuedTokenAmount);
            assert(schema.nonce);
            assert(schema.validAfterUnixTimestamp);
            assert(schema.expiresAtUnixTimestamp);
            assert(schema.policyId);
            assert(schema.intentHash);
            assert(schema.pathwayId);
            assert(schema.rentRefundRecipient);
            assert(schema.reserved);
        });

        it("has schema for SettlementPolicy", () => {
            const schema = SettlementPolicy.getSchema();
            assert(SettlementPolicy.discriminator);
            assert(schema.version);
            assert(schema.bump);
            assert(schema.pad0);
            assert(schema.policyFlags);
            assert(schema.allowedSettlementModes);
            assert(schema.pad1);
            assert(schema.policyId);
            assert(schema.allowedAssetMint);
            assert(schema.allowedPrincipalA);
            assert(schema.allowedPrincipalB);
            assert(schema.designatedExecutor);
            assert(schema.maxNotional);
            assert(schema.minNotional);
            assert(schema.validAfterUnixTimestamp);
            assert(schema.expiresAtUnixTimestamp);
            assert(schema.createdBy);
            assert(schema.reserved);
        });

    });
});
