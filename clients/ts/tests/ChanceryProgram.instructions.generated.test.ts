// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      test
// File:          ChanceryProgram.instructions.generated.test.ts
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

import { AcceptAuthorityTransferInstruction } from "../src/instructions/AcceptAuthorityTransferInstruction";
import { InitializeChanceryInstruction } from "../src/instructions/InitializeChanceryInstruction";
import { ProposeAuthorityTransferInstruction } from "../src/instructions/ProposeAuthorityTransferInstruction";
import { RegisterAssetInstruction } from "../src/instructions/RegisterAssetInstruction";
import { SetAssetModeInstruction } from "../src/instructions/SetAssetModeInstruction";
import { SetAssetModeWithPendingChangeInstruction } from "../src/instructions/SetAssetModeWithPendingChangeInstruction";
import { UpdateAssetConfigInstruction } from "../src/instructions/UpdateAssetConfigInstruction";
import { UpdateAssetConfigWithPendingChangeInstruction } from "../src/instructions/UpdateAssetConfigWithPendingChangeInstruction";
import { EmitInstruction } from "../src/instructions/EmitInstruction";
import { UpsertPermissionInstruction } from "../src/instructions/UpsertPermissionInstruction";
import { RevokePermissionInstruction } from "../src/instructions/RevokePermissionInstruction";
import { UpsertPermissionWithPendingChangeInstruction } from "../src/instructions/UpsertPermissionWithPendingChangeInstruction";
import { RegisterPathwayPolicyInstruction } from "../src/instructions/RegisterPathwayPolicyInstruction";
import { UpdatePathwayPolicyInstruction } from "../src/instructions/UpdatePathwayPolicyInstruction";
import { UpdatePathwayPolicyWithPendingChangeInstruction } from "../src/instructions/UpdatePathwayPolicyWithPendingChangeInstruction";
import { CreateSettlementIntentInstruction } from "../src/instructions/CreateSettlementIntentInstruction";
import { CancelSettlementIntentInstruction } from "../src/instructions/CancelSettlementIntentInstruction";
import { CloseExpiredSettlementIntentInstruction } from "../src/instructions/CloseExpiredSettlementIntentInstruction";
import { MintDelegatedInstruction } from "../src/instructions/MintDelegatedInstruction";
import { MintDirectInstruction } from "../src/instructions/MintDirectInstruction";
import { MintTrilateralInstruction } from "../src/instructions/MintTrilateralInstruction";
import { RedeemDelegatedInstruction } from "../src/instructions/RedeemDelegatedInstruction";
import { RedeemDirectInstruction } from "../src/instructions/RedeemDirectInstruction";
import { RedeemTrilateralInstruction } from "../src/instructions/RedeemTrilateralInstruction";
import { RegisterSettlementPolicyInstruction } from "../src/instructions/RegisterSettlementPolicyInstruction";
import { RegisterLimitPolicyInstruction } from "../src/instructions/RegisterLimitPolicyInstruction";
import { UpdateLimitPolicyInstruction } from "../src/instructions/UpdateLimitPolicyInstruction";
import { UpdateLimitPolicyWithPendingChangeInstruction } from "../src/instructions/UpdateLimitPolicyWithPendingChangeInstruction";
import { RegisterEvidencePolicyInstruction } from "../src/instructions/RegisterEvidencePolicyInstruction";
import { UpdateEvidencePolicyInstruction } from "../src/instructions/UpdateEvidencePolicyInstruction";
import { UpdateEvidencePolicyWithPendingChangeInstruction } from "../src/instructions/UpdateEvidencePolicyWithPendingChangeInstruction";
import { RegisterFeePolicyInstruction } from "../src/instructions/RegisterFeePolicyInstruction";
import { UpdateFeePolicyInstruction } from "../src/instructions/UpdateFeePolicyInstruction";
import { UpdateFeePolicyWithPendingChangeInstruction } from "../src/instructions/UpdateFeePolicyWithPendingChangeInstruction";
import { RegisterReserveDestinationWithPendingInstruction } from "../src/instructions/RegisterReserveDestinationWithPendingInstruction";
import { SetReserveDestinationStatusInstruction } from "../src/instructions/SetReserveDestinationStatusInstruction";
import { SetReserveDestinationStatusWithPendingInstruction } from "../src/instructions/SetReserveDestinationStatusWithPendingInstruction";
import { WithdrawReserveInstruction } from "../src/instructions/WithdrawReserveInstruction";
import { AcceptConfigChangeInstruction } from "../src/instructions/AcceptConfigChangeInstruction";
import { CancelConfigChangeInstruction } from "../src/instructions/CancelConfigChangeInstruction";
import { CloseExpiredConfigChangeInstruction } from "../src/instructions/CloseExpiredConfigChangeInstruction";
import { FreezeIssuedTokenAccountInstruction } from "../src/instructions/FreezeIssuedTokenAccountInstruction";
import { InitializeModuleActivationStateInstruction } from "../src/instructions/InitializeModuleActivationStateInstruction";
import { ProposeConfigChangeInstruction } from "../src/instructions/ProposeConfigChangeInstruction";
import { SetAssetPauseInstruction } from "../src/instructions/SetAssetPauseInstruction";
import { SetCounterpartyPauseInstruction } from "../src/instructions/SetCounterpartyPauseInstruction";
import { SetExecutorPauseInstruction } from "../src/instructions/SetExecutorPauseInstruction";
import { SetGlobalPauseInstruction } from "../src/instructions/SetGlobalPauseInstruction";
import { SetModuleStatusInstruction } from "../src/instructions/SetModuleStatusInstruction";
import { SetModuleStatusWithPendingChangeInstruction } from "../src/instructions/SetModuleStatusWithPendingChangeInstruction";
import { SetPathwayPauseInstruction } from "../src/instructions/SetPathwayPauseInstruction";
import { ThawIssuedTokenAccountInstruction } from "../src/instructions/ThawIssuedTokenAccountInstruction";
import { EnableLegacyMigrationInstruction } from "../src/instructions/EnableLegacyMigrationInstruction";
import { MigrateLegacyToToken2022Instruction } from "../src/instructions/MigrateLegacyToToken2022Instruction";
import { InitializeIssuedTokenControlInstruction } from "../src/instructions/InitializeIssuedTokenControlInstruction";
import { RefreshAssetExtensionObservationInstruction } from "../src/instructions/RefreshAssetExtensionObservationInstruction";
import { VerifyIssuedTokenDeploymentInstruction } from "../src/instructions/VerifyIssuedTokenDeploymentInstruction";
import { RefreshIssuedTokenExtensionObservationInstruction } from "../src/instructions/RefreshIssuedTokenExtensionObservationInstruction";
import { ConsumeInboundMessageInstruction } from "../src/instructions/ConsumeInboundMessageInstruction";
import { EmitOutboundMessageInstruction } from "../src/instructions/EmitOutboundMessageInstruction";
import { ExpireInboundMessageInstruction } from "../src/instructions/ExpireInboundMessageInstruction";
import { ReclaimExpiredOutboundInstruction } from "../src/instructions/ReclaimExpiredOutboundInstruction";
import { RelaxRemoteDomainPauseInstruction } from "../src/instructions/RelaxRemoteDomainPauseInstruction";
import { RegisterCrossChainSignerSetInstruction } from "../src/instructions/RegisterCrossChainSignerSetInstruction";
import { RegisterRemoteDomainPolicyInstruction } from "../src/instructions/RegisterRemoteDomainPolicyInstruction";
import { RestrictRemoteDomainPauseInstruction } from "../src/instructions/RestrictRemoteDomainPauseInstruction";
import { RotateCrossChainSignerSetInstruction } from "../src/instructions/RotateCrossChainSignerSetInstruction";
import { UpdateRemoteDomainPolicyInstruction } from "../src/instructions/UpdateRemoteDomainPolicyInstruction";
import { UpdateRemoteDomainPolicyWithPendingChangeInstruction } from "../src/instructions/UpdateRemoteDomainPolicyWithPendingChangeInstruction";
import { RelaxRemoteDomainPauseWithPendingChangeInstruction } from "../src/instructions/RelaxRemoteDomainPauseWithPendingChangeInstruction";

describe("ChanceryProgram instructions auto generated tests", () => {
    describe("instructions", () => {
        it("has schema for AcceptAuthorityTransferInstruction", () => {
            const schema = AcceptAuthorityTransferInstruction.getSchema();
            assert(schema.roleKind);
        });

        it("has accounts schema for AcceptAuthorityTransferInstruction", () => {
            const accountsSchema = AcceptAuthorityTransferInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.authorityTransfer);
            assert(accountsSchema.newAuthority);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for InitializeChanceryInstruction", () => {
            const schema = InitializeChanceryInstruction.getSchema();
            assert(schema.issuedTokenMint);
            assert(schema.issuedTokenProgram);
            assert(schema.legacyTokenMint);
            assert(schema.legacyTokenProgram);
        });

        it("has accounts schema for InitializeChanceryInstruction", () => {
            const accountsSchema = InitializeChanceryInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.payer);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.pauseState);
            assert(accountsSchema.programAccount);
            assert(accountsSchema.programdataAccount);
            assert(accountsSchema.upgradeAuthority);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for ProposeAuthorityTransferInstruction", () => {
            const schema = ProposeAuthorityTransferInstruction.getSchema();
            assert(schema.roleKind);
            assert(schema.proposedAuthority);
            assert(schema.timelockSlots);
        });

        it("has accounts schema for ProposeAuthorityTransferInstruction", () => {
            const accountsSchema = ProposeAuthorityTransferInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.authorityTransfer);
            assert(accountsSchema.payer);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for RegisterAssetInstruction", () => {
            const schema = RegisterAssetInstruction.getSchema();
            assert(schema.mode);
            assert(schema.approvedExtensionMask);
            assert(schema.observedExtensionMaskHint);
            assert(schema.depositRateE9);
            assert(schema.redeemRateE9);
            assert(schema.minimumDepositAmount);
            assert(schema.minimumRedeemAmount);
            assert(schema.maximumSingleSettlementAmount);
            assert(schema.maxExtensionObservationAgeSlots);
            assert(schema.assetTokenProgram);
        });

        it("has accounts schema for RegisterAssetInstruction", () => {
            const accountsSchema = RegisterAssetInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.assetConfig);
            assert(accountsSchema.assetMint);
            assert(accountsSchema.payer);
            assert(accountsSchema.operationsAuthority);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for SetAssetModeInstruction", () => {
            const schema = SetAssetModeInstruction.getSchema();
            assert(schema.newMode);
        });

        it("has accounts schema for SetAssetModeInstruction", () => {
            const accountsSchema = SetAssetModeInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.assetConfig);
            assert(accountsSchema.authority);
            assert(accountsSchema.eventAuthorityDirect);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for SetAssetModeWithPendingChangeInstruction", () => {
            const schema = SetAssetModeWithPendingChangeInstruction.getSchema();
            assert(schema.newMode);
        });

        it("has accounts schema for SetAssetModeWithPendingChangeInstruction", () => {
            const accountsSchema = SetAssetModeWithPendingChangeInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pendingConfigChange);
            assert(accountsSchema.assetConfig);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for UpdateAssetConfigInstruction", () => {
            const schema = UpdateAssetConfigInstruction.getSchema();
            assert(schema.approvedExtensionMask);
            assert(schema.observedExtensionMask);
            assert(schema.depositRateE9);
            assert(schema.redeemRateE9);
            assert(schema.minimumDepositAmount);
            assert(schema.minimumRedeemAmount);
            assert(schema.maximumSingleSettlementAmount);
            assert(schema.forbiddenExtensionMask);
            assert(schema.requiredModuleMask);
        });

        it("has accounts schema for UpdateAssetConfigInstruction", () => {
            const accountsSchema = UpdateAssetConfigInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.assetConfig);
            assert(accountsSchema.operationsAuthority);
            assert(accountsSchema.eventAuthorityDirect);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for UpdateAssetConfigWithPendingChangeInstruction", () => {
            const schema = UpdateAssetConfigWithPendingChangeInstruction.getSchema();
            assert(schema.approvedExtensionMask);
            assert(schema.observedExtensionMask);
            assert(schema.depositRateE9);
            assert(schema.redeemRateE9);
            assert(schema.minimumDepositAmount);
            assert(schema.minimumRedeemAmount);
            assert(schema.maximumSingleSettlementAmount);
            assert(schema.forbiddenExtensionMask);
            assert(schema.requiredModuleMask);
        });

        it("has accounts schema for UpdateAssetConfigWithPendingChangeInstruction", () => {
            const accountsSchema = UpdateAssetConfigWithPendingChangeInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pendingConfigChange);
            assert(accountsSchema.assetConfig);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for EmitInstruction", () => {
            const schema = EmitInstruction.getSchema();
        });

        it("has accounts schema for EmitInstruction", () => {
            const accountsSchema = EmitInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for UpsertPermissionInstruction", () => {
            const schema = UpsertPermissionInstruction.getSchema();
            assert(schema.subject);
            assert(schema.scopeKind);
            assert(schema.scopeKey);
            assert(schema.roleBits);
            assert(schema.expiryUnixTimestamp);
        });

        it("has accounts schema for UpsertPermissionInstruction", () => {
            const accountsSchema = UpsertPermissionInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.permissionRecord);
            assert(accountsSchema.grantingAuthority);
            assert(accountsSchema.grantorPermission);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for RevokePermissionInstruction", () => {
            const schema = RevokePermissionInstruction.getSchema();
        });

        it("has accounts schema for RevokePermissionInstruction", () => {
            const accountsSchema = RevokePermissionInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.permissionRecord);
            assert(accountsSchema.revokingAuthority);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for UpsertPermissionWithPendingChangeInstruction", () => {
            const schema = UpsertPermissionWithPendingChangeInstruction.getSchema();
            assert(schema.subject);
            assert(schema.scopeKind);
            assert(schema.scopeKey);
            assert(schema.roleBits);
            assert(schema.expiryUnixTimestamp);
        });

        it("has accounts schema for UpsertPermissionWithPendingChangeInstruction", () => {
            const accountsSchema = UpsertPermissionWithPendingChangeInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pending);
            assert(accountsSchema.permissionRecord);
            assert(accountsSchema.payer);
            assert(accountsSchema.grantingAuthority);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.grantorPermission);
            assert(accountsSchema.counterpartyDailyUsageWindow);
            assert(accountsSchema.executorDailyUsageWindow);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for RegisterPathwayPolicyInstruction", () => {
            const schema = RegisterPathwayPolicyInstruction.getSchema();
            assert(schema.pathwayId);
            assert(schema.pathwayKind);
            assert(schema.assetMint);
            assert(schema.issuedTokenMint);
            assert(schema.designatedExecutor);
            assert(schema.reserveCompartmentPolicyId);
            assert(schema.limitPolicyId);
            assert(schema.evidencePolicyId);
            assert(schema.feePolicyId);
            assert(schema.insurancePolicyId);
            assert(schema.assetMintLimitPolicyId);
            assert(schema.assetRedeemLimitPolicyId);
            assert(schema.counterpartyLimitPolicyId);
            assert(schema.executorLimitPolicyId);
        });

        it("has accounts schema for RegisterPathwayPolicyInstruction", () => {
            const accountsSchema = RegisterPathwayPolicyInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pathwayPolicy);
            assert(accountsSchema.payer);
            assert(accountsSchema.operationsAuthority);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.limitPolicy);
            assert(accountsSchema.evidencePolicy);
            assert(accountsSchema.feePolicy);
            assert(accountsSchema.insurancePolicy);
            assert(accountsSchema.assetMintLimitPolicy);
            assert(accountsSchema.assetRedeemLimitPolicy);
            assert(accountsSchema.counterpartyLimitPolicy);
            assert(accountsSchema.executorLimitPolicy);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for UpdatePathwayPolicyInstruction", () => {
            const schema = UpdatePathwayPolicyInstruction.getSchema();
            assert(schema.pathwayId);
            assert(schema.designatedExecutor);
            assert(schema.limitPolicyId);
            assert(schema.evidencePolicyId);
            assert(schema.feePolicyId);
            assert(schema.insurancePolicyId);
            assert(schema.forbiddenCollateralExtensionMask);
            assert(schema.assetMintLimitPolicyId);
            assert(schema.assetRedeemLimitPolicyId);
            assert(schema.counterpartyLimitPolicyId);
            assert(schema.executorLimitPolicyId);
        });

        it("has accounts schema for UpdatePathwayPolicyInstruction", () => {
            const accountsSchema = UpdatePathwayPolicyInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.pathwayPolicy);
            assert(accountsSchema.operationsAuthority);
            assert(accountsSchema.eventAuthorityDirect);
            assert(accountsSchema.limitPolicy);
            assert(accountsSchema.evidencePolicy);
            assert(accountsSchema.feePolicy);
            assert(accountsSchema.insurancePolicy);
            assert(accountsSchema.assetMintLimitPolicy);
            assert(accountsSchema.assetRedeemLimitPolicy);
            assert(accountsSchema.counterpartyLimitPolicy);
            assert(accountsSchema.executorLimitPolicy);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for UpdatePathwayPolicyWithPendingChangeInstruction", () => {
            const schema = UpdatePathwayPolicyWithPendingChangeInstruction.getSchema();
            assert(schema.pathwayId);
            assert(schema.designatedExecutor);
            assert(schema.limitPolicyId);
            assert(schema.evidencePolicyId);
            assert(schema.feePolicyId);
            assert(schema.insurancePolicyId);
            assert(schema.forbiddenCollateralExtensionMask);
            assert(schema.assetMintLimitPolicyId);
            assert(schema.assetRedeemLimitPolicyId);
            assert(schema.counterpartyLimitPolicyId);
            assert(schema.executorLimitPolicyId);
        });

        it("has accounts schema for UpdatePathwayPolicyWithPendingChangeInstruction", () => {
            const accountsSchema = UpdatePathwayPolicyWithPendingChangeInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pendingConfigChange);
            assert(accountsSchema.pathwayPolicy);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.limitPolicy);
            assert(accountsSchema.evidencePolicy);
            assert(accountsSchema.feePolicy);
            assert(accountsSchema.insurancePolicy);
            assert(accountsSchema.assetMintLimitPolicy);
            assert(accountsSchema.assetRedeemLimitPolicy);
            assert(accountsSchema.counterpartyLimitPolicy);
            assert(accountsSchema.executorLimitPolicy);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for CreateSettlementIntentInstruction", () => {
            const schema = CreateSettlementIntentInstruction.getSchema();
            assert(schema.intentId);
            assert(schema.pathwayId);
            assert(schema.settlementMode);
            assert(schema.settlementAction);
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
        });

        it("has accounts schema for CreateSettlementIntentInstruction", () => {
            const accountsSchema = CreateSettlementIntentInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.settlementIntent);
            assert(accountsSchema.payer);
            assert(accountsSchema.creator);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.pathwayPolicy);
            assert(accountsSchema.principalAPermissionRecord);
            assert(accountsSchema.principalBPermissionRecord);
            assert(accountsSchema.executorPermissionRecord);
            assert(accountsSchema.settlementPolicy);
            assert(accountsSchema.feePolicy);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for CancelSettlementIntentInstruction", () => {
            const schema = CancelSettlementIntentInstruction.getSchema();
        });

        it("has accounts schema for CancelSettlementIntentInstruction", () => {
            const accountsSchema = CancelSettlementIntentInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.settlementIntent);
            assert(accountsSchema.principalA);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for CloseExpiredSettlementIntentInstruction", () => {
            const schema = CloseExpiredSettlementIntentInstruction.getSchema();
        });

        it("has accounts schema for CloseExpiredSettlementIntentInstruction", () => {
            const accountsSchema = CloseExpiredSettlementIntentInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.settlementIntent);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for MintDelegatedInstruction", () => {
            const schema = MintDelegatedInstruction.getSchema();
            assert(schema.intentId);
            assert(schema.pathwayId);
        });

        it("has accounts schema for MintDelegatedInstruction", () => {
            const accountsSchema = MintDelegatedInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pauseState);
            assert(accountsSchema.assetConfig);
            assert(accountsSchema.pathwayPolicy);
            assert(accountsSchema.intent);
            assert(accountsSchema.principalPermissionRecord);
            assert(accountsSchema.executorPermissionRecord);
            assert(accountsSchema.sourceAssetTokenAccount);
            assert(accountsSchema.reserveAssetTokenAccount);
            assert(accountsSchema.destinationIssuedTokenAccount);
            assert(accountsSchema.assetMint);
            assert(accountsSchema.issuedTokenMint);
            assert(accountsSchema.mintAuthorityPda);
            assert(accountsSchema.assetTokenProgram);
            assert(accountsSchema.issuedTokenProgram);
            assert(accountsSchema.executor);
            assert(accountsSchema.principal);
            assert(accountsSchema.assetPauseState);
            assert(accountsSchema.issuedTokenControl);
            assert(accountsSchema.feePolicy);
            assert(accountsSchema.feeRecipientTokenAccount);
            assert(accountsSchema.limitPolicy);
            assert(accountsSchema.hourlyUsageWindow);
            assert(accountsSchema.dailyUsageWindow);
            assert(accountsSchema.weeklyUsageWindow);
            assert(accountsSchema.monthlyUsageWindow);
            assert(accountsSchema.evidencePolicy);
            assert(accountsSchema.settlementPolicy);
            assert(accountsSchema.assetLimitPolicy);
            assert(accountsSchema.assetDailyUsageWindow);
            assert(accountsSchema.counterpartyLimitPolicy);
            assert(accountsSchema.counterpartyDailyUsageWindow);
            assert(accountsSchema.executorLimitPolicy);
            assert(accountsSchema.executorDailyUsageWindow);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for MintDirectInstruction", () => {
            const schema = MintDirectInstruction.getSchema();
            assert(schema.pathwayId);
            assert(schema.assetAmount);
            assert(schema.minimumIssuedTokenAmount);
        });

        it("has accounts schema for MintDirectInstruction", () => {
            const accountsSchema = MintDirectInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pauseState);
            assert(accountsSchema.assetConfig);
            assert(accountsSchema.pathwayPolicy);
            assert(accountsSchema.permissionRecord);
            assert(accountsSchema.sourceAssetTokenAccount);
            assert(accountsSchema.reserveAssetTokenAccount);
            assert(accountsSchema.destinationIssuedTokenAccount);
            assert(accountsSchema.assetMint);
            assert(accountsSchema.issuedTokenMint);
            assert(accountsSchema.mintAuthorityPda);
            assert(accountsSchema.assetTokenProgram);
            assert(accountsSchema.issuedTokenProgram);
            assert(accountsSchema.principal);
            assert(accountsSchema.assetPauseState);
            assert(accountsSchema.issuedTokenControl);
            assert(accountsSchema.feePolicy);
            assert(accountsSchema.feeRecipientTokenAccount);
            assert(accountsSchema.limitPolicy);
            assert(accountsSchema.hourlyUsageWindow);
            assert(accountsSchema.dailyUsageWindow);
            assert(accountsSchema.weeklyUsageWindow);
            assert(accountsSchema.monthlyUsageWindow);
            assert(accountsSchema.evidencePolicy);
            assert(accountsSchema.assetLimitPolicy);
            assert(accountsSchema.assetDailyUsageWindow);
            assert(accountsSchema.counterpartyLimitPolicy);
            assert(accountsSchema.counterpartyDailyUsageWindow);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for MintTrilateralInstruction", () => {
            const schema = MintTrilateralInstruction.getSchema();
            assert(schema.intentId);
            assert(schema.pathwayId);
        });

        it("has accounts schema for MintTrilateralInstruction", () => {
            const accountsSchema = MintTrilateralInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pauseState);
            assert(accountsSchema.assetConfig);
            assert(accountsSchema.pathwayPolicy);
            assert(accountsSchema.intent);
            assert(accountsSchema.principalAPermissionRecord);
            assert(accountsSchema.principalBPermissionRecord);
            assert(accountsSchema.executorPermissionRecord);
            assert(accountsSchema.sourceAssetTokenAccount);
            assert(accountsSchema.reserveAssetTokenAccount);
            assert(accountsSchema.destinationIssuedTokenAccount);
            assert(accountsSchema.assetMint);
            assert(accountsSchema.issuedTokenMint);
            assert(accountsSchema.mintAuthorityPda);
            assert(accountsSchema.assetTokenProgram);
            assert(accountsSchema.issuedTokenProgram);
            assert(accountsSchema.executor);
            assert(accountsSchema.principalA);
            assert(accountsSchema.principalB);
            assert(accountsSchema.assetPauseState);
            assert(accountsSchema.issuedTokenControl);
            assert(accountsSchema.feePolicy);
            assert(accountsSchema.feeRecipientTokenAccount);
            assert(accountsSchema.limitPolicy);
            assert(accountsSchema.hourlyUsageWindow);
            assert(accountsSchema.dailyUsageWindow);
            assert(accountsSchema.weeklyUsageWindow);
            assert(accountsSchema.monthlyUsageWindow);
            assert(accountsSchema.evidencePolicy);
            assert(accountsSchema.settlementPolicy);
            assert(accountsSchema.assetLimitPolicy);
            assert(accountsSchema.assetDailyUsageWindow);
            assert(accountsSchema.counterpartyLimitPolicy);
            assert(accountsSchema.counterpartyADailyUsageWindow);
            assert(accountsSchema.counterpartyBDailyUsageWindow);
            assert(accountsSchema.executorLimitPolicy);
            assert(accountsSchema.executorDailyUsageWindow);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for RedeemDelegatedInstruction", () => {
            const schema = RedeemDelegatedInstruction.getSchema();
            assert(schema.intentId);
            assert(schema.pathwayId);
        });

        it("has accounts schema for RedeemDelegatedInstruction", () => {
            const accountsSchema = RedeemDelegatedInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pauseState);
            assert(accountsSchema.assetConfig);
            assert(accountsSchema.pathwayPolicy);
            assert(accountsSchema.intent);
            assert(accountsSchema.principalPermissionRecord);
            assert(accountsSchema.executorPermissionRecord);
            assert(accountsSchema.sourceIssuedTokenAccount);
            assert(accountsSchema.reserveAssetTokenAccount);
            assert(accountsSchema.destinationAssetTokenAccount);
            assert(accountsSchema.assetMint);
            assert(accountsSchema.issuedTokenMint);
            assert(accountsSchema.reserveAuthorityPda);
            assert(accountsSchema.assetTokenProgram);
            assert(accountsSchema.issuedTokenProgram);
            assert(accountsSchema.executor);
            assert(accountsSchema.principal);
            assert(accountsSchema.assetPauseState);
            assert(accountsSchema.issuedTokenControl);
            assert(accountsSchema.feePolicy);
            assert(accountsSchema.feeRecipientTokenAccount);
            assert(accountsSchema.limitPolicy);
            assert(accountsSchema.hourlyUsageWindow);
            assert(accountsSchema.dailyUsageWindow);
            assert(accountsSchema.weeklyUsageWindow);
            assert(accountsSchema.monthlyUsageWindow);
            assert(accountsSchema.evidencePolicy);
            assert(accountsSchema.settlementPolicy);
            assert(accountsSchema.assetLimitPolicy);
            assert(accountsSchema.assetDailyUsageWindow);
            assert(accountsSchema.counterpartyLimitPolicy);
            assert(accountsSchema.counterpartyDailyUsageWindow);
            assert(accountsSchema.executorLimitPolicy);
            assert(accountsSchema.executorDailyUsageWindow);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for RedeemDirectInstruction", () => {
            const schema = RedeemDirectInstruction.getSchema();
            assert(schema.pathwayId);
            assert(schema.issuedTokenAmount);
            assert(schema.minimumAssetAmount);
        });

        it("has accounts schema for RedeemDirectInstruction", () => {
            const accountsSchema = RedeemDirectInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pauseState);
            assert(accountsSchema.assetConfig);
            assert(accountsSchema.pathwayPolicy);
            assert(accountsSchema.permissionRecord);
            assert(accountsSchema.sourceIssuedTokenAccount);
            assert(accountsSchema.reserveAssetTokenAccount);
            assert(accountsSchema.destinationAssetTokenAccount);
            assert(accountsSchema.assetMint);
            assert(accountsSchema.issuedTokenMint);
            assert(accountsSchema.reserveAuthorityPda);
            assert(accountsSchema.assetTokenProgram);
            assert(accountsSchema.issuedTokenProgram);
            assert(accountsSchema.principal);
            assert(accountsSchema.assetPauseState);
            assert(accountsSchema.issuedTokenControl);
            assert(accountsSchema.feePolicy);
            assert(accountsSchema.feeRecipientTokenAccount);
            assert(accountsSchema.limitPolicy);
            assert(accountsSchema.hourlyUsageWindow);
            assert(accountsSchema.dailyUsageWindow);
            assert(accountsSchema.weeklyUsageWindow);
            assert(accountsSchema.monthlyUsageWindow);
            assert(accountsSchema.evidencePolicy);
            assert(accountsSchema.assetLimitPolicy);
            assert(accountsSchema.assetDailyUsageWindow);
            assert(accountsSchema.counterpartyLimitPolicy);
            assert(accountsSchema.counterpartyDailyUsageWindow);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for RedeemTrilateralInstruction", () => {
            const schema = RedeemTrilateralInstruction.getSchema();
            assert(schema.intentId);
            assert(schema.pathwayId);
        });

        it("has accounts schema for RedeemTrilateralInstruction", () => {
            const accountsSchema = RedeemTrilateralInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pauseState);
            assert(accountsSchema.assetConfig);
            assert(accountsSchema.pathwayPolicy);
            assert(accountsSchema.intent);
            assert(accountsSchema.principalAPermissionRecord);
            assert(accountsSchema.principalBPermissionRecord);
            assert(accountsSchema.executorPermissionRecord);
            assert(accountsSchema.sourceIssuedTokenAccount);
            assert(accountsSchema.reserveAssetTokenAccount);
            assert(accountsSchema.destinationAssetTokenAccount);
            assert(accountsSchema.assetMint);
            assert(accountsSchema.issuedTokenMint);
            assert(accountsSchema.reserveAuthorityPda);
            assert(accountsSchema.assetTokenProgram);
            assert(accountsSchema.issuedTokenProgram);
            assert(accountsSchema.executor);
            assert(accountsSchema.principalA);
            assert(accountsSchema.principalB);
            assert(accountsSchema.assetPauseState);
            assert(accountsSchema.issuedTokenControl);
            assert(accountsSchema.feePolicy);
            assert(accountsSchema.feeRecipientTokenAccount);
            assert(accountsSchema.limitPolicy);
            assert(accountsSchema.hourlyUsageWindow);
            assert(accountsSchema.dailyUsageWindow);
            assert(accountsSchema.weeklyUsageWindow);
            assert(accountsSchema.monthlyUsageWindow);
            assert(accountsSchema.evidencePolicy);
            assert(accountsSchema.settlementPolicy);
            assert(accountsSchema.assetLimitPolicy);
            assert(accountsSchema.assetDailyUsageWindow);
            assert(accountsSchema.counterpartyLimitPolicy);
            assert(accountsSchema.counterpartyADailyUsageWindow);
            assert(accountsSchema.counterpartyBDailyUsageWindow);
            assert(accountsSchema.executorLimitPolicy);
            assert(accountsSchema.executorDailyUsageWindow);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for RegisterSettlementPolicyInstruction", () => {
            const schema = RegisterSettlementPolicyInstruction.getSchema();
            assert(schema.policyId);
            assert(schema.policyFlags);
            assert(schema.allowedSettlementModes);
            assert(schema.allowedAssetMint);
            assert(schema.allowedPrincipalA);
            assert(schema.allowedPrincipalB);
            assert(schema.designatedExecutor);
            assert(schema.maxNotional);
            assert(schema.minNotional);
            assert(schema.validAfterUnixTimestamp);
            assert(schema.expiresAtUnixTimestamp);
        });

        it("has accounts schema for RegisterSettlementPolicyInstruction", () => {
            const accountsSchema = RegisterSettlementPolicyInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.settlementPolicy);
            assert(accountsSchema.payer);
            assert(accountsSchema.operationsAuthority);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for RegisterLimitPolicyInstruction", () => {
            const schema = RegisterLimitPolicyInstruction.getSchema();
            assert(schema.limitPolicyId);
            assert(schema.scopeKind);
            assert(schema.scopeKey);
            assert(schema.perTransactionMaximum);
            assert(schema.perHourMaximum);
            assert(schema.perDayMaximum);
            assert(schema.perSevenDayMaximum);
            assert(schema.perThirtyDayMaximum);
            assert(schema.maximumActionsPerHour);
            assert(schema.maximumActionsPerDay);
        });

        it("has accounts schema for RegisterLimitPolicyInstruction", () => {
            const accountsSchema = RegisterLimitPolicyInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.limitPolicy);
            assert(accountsSchema.payer);
            assert(accountsSchema.operationsAuthority);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.hourlyUsageWindow);
            assert(accountsSchema.dailyUsageWindow);
            assert(accountsSchema.weeklyUsageWindow);
            assert(accountsSchema.monthlyUsageWindow);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for UpdateLimitPolicyInstruction", () => {
            const schema = UpdateLimitPolicyInstruction.getSchema();
            assert(schema.limitPolicyId);
            assert(schema.perTransactionMaximum);
            assert(schema.perHourMaximum);
            assert(schema.perDayMaximum);
            assert(schema.perSevenDayMaximum);
            assert(schema.perThirtyDayMaximum);
            assert(schema.maximumActionsPerHour);
            assert(schema.maximumActionsPerDay);
        });

        it("has accounts schema for UpdateLimitPolicyInstruction", () => {
            const accountsSchema = UpdateLimitPolicyInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.limitPolicy);
            assert(accountsSchema.operationsAuthority);
            assert(accountsSchema.eventAuthorityDirect);
            assert(accountsSchema.payer);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.hourlyUsageWindow);
            assert(accountsSchema.dailyUsageWindow);
            assert(accountsSchema.weeklyUsageWindow);
            assert(accountsSchema.monthlyUsageWindow);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for UpdateLimitPolicyWithPendingChangeInstruction", () => {
            const schema = UpdateLimitPolicyWithPendingChangeInstruction.getSchema();
            assert(schema.limitPolicyId);
            assert(schema.perTransactionMaximum);
            assert(schema.perHourMaximum);
            assert(schema.perDayMaximum);
            assert(schema.perSevenDayMaximum);
            assert(schema.perThirtyDayMaximum);
            assert(schema.maximumActionsPerHour);
            assert(schema.maximumActionsPerDay);
        });

        it("has accounts schema for UpdateLimitPolicyWithPendingChangeInstruction", () => {
            const accountsSchema = UpdateLimitPolicyWithPendingChangeInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pendingConfigChange);
            assert(accountsSchema.limitPolicy);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.payer);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.hourlyUsageWindow);
            assert(accountsSchema.dailyUsageWindow);
            assert(accountsSchema.weeklyUsageWindow);
            assert(accountsSchema.monthlyUsageWindow);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for RegisterEvidencePolicyInstruction", () => {
            const schema = RegisterEvidencePolicyInstruction.getSchema();
            assert(schema.evidencePolicyId);
            assert(schema.requiredFieldMask);
            assert(schema.counterpartyReportingSchemaHash);
            assert(schema.allowFreeformCounterpartyFields);
            assert(schema.maximumFreeformFieldCount);
            assert(schema.maximumFreeformValueBytes);
            assert(schema.retentionFlags);
        });

        it("has accounts schema for RegisterEvidencePolicyInstruction", () => {
            const accountsSchema = RegisterEvidencePolicyInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.evidencePolicy);
            assert(accountsSchema.payer);
            assert(accountsSchema.operationsAuthority);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for UpdateEvidencePolicyInstruction", () => {
            const schema = UpdateEvidencePolicyInstruction.getSchema();
            assert(schema.evidencePolicyId);
            assert(schema.requiredFieldMask);
            assert(schema.counterpartyReportingSchemaHash);
            assert(schema.allowFreeformCounterpartyFields);
            assert(schema.maximumFreeformFieldCount);
            assert(schema.maximumFreeformValueBytes);
            assert(schema.retentionFlags);
        });

        it("has accounts schema for UpdateEvidencePolicyInstruction", () => {
            const accountsSchema = UpdateEvidencePolicyInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.evidencePolicy);
            assert(accountsSchema.operationsAuthority);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for UpdateEvidencePolicyWithPendingChangeInstruction", () => {
            const schema = UpdateEvidencePolicyWithPendingChangeInstruction.getSchema();
            assert(schema.evidencePolicyId);
            assert(schema.requiredFieldMask);
            assert(schema.counterpartyReportingSchemaHash);
            assert(schema.allowFreeformCounterpartyFields);
            assert(schema.maximumFreeformFieldCount);
            assert(schema.maximumFreeformValueBytes);
            assert(schema.retentionFlags);
        });

        it("has accounts schema for UpdateEvidencePolicyWithPendingChangeInstruction", () => {
            const accountsSchema = UpdateEvidencePolicyWithPendingChangeInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pendingConfigChange);
            assert(accountsSchema.evidencePolicy);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for RegisterFeePolicyInstruction", () => {
            const schema = RegisterFeePolicyInstruction.getSchema();
            assert(schema.feePolicyId);
            assert(schema.feePolicyFlags);
            assert(schema.flatFeeInAsset);
            assert(schema.flatFeeInIssuedToken);
            assert(schema.percentFeeBps);
            assert(schema.feeCapAmount);
            assert(schema.minimumFeeAmount);
            assert(schema.rebateFlatAmount);
            assert(schema.rebateBps);
            assert(schema.rebateCapAmount);
            assert(schema.netFeeFloorZero);
            assert(schema.feeRecipientPolicy);
            assert(schema.roundingMode);
            assert(schema.feeRecipientKey);
            assert(schema.effectiveFromUnixTimestamp);
            assert(schema.effectiveUntilUnixTimestamp);
        });

        it("has accounts schema for RegisterFeePolicyInstruction", () => {
            const accountsSchema = RegisterFeePolicyInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.feePolicy);
            assert(accountsSchema.payer);
            assert(accountsSchema.operationsAuthority);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for UpdateFeePolicyInstruction", () => {
            const schema = UpdateFeePolicyInstruction.getSchema();
            assert(schema.feePolicyId);
            assert(schema.feePolicyFlags);
            assert(schema.flatFeeInAsset);
            assert(schema.flatFeeInIssuedToken);
            assert(schema.percentFeeBps);
            assert(schema.feeCapAmount);
            assert(schema.minimumFeeAmount);
            assert(schema.rebateFlatAmount);
            assert(schema.rebateBps);
            assert(schema.rebateCapAmount);
            assert(schema.netFeeFloorZero);
            assert(schema.feeRecipientKey);
            assert(schema.effectiveFromUnixTimestamp);
            assert(schema.effectiveUntilUnixTimestamp);
        });

        it("has accounts schema for UpdateFeePolicyInstruction", () => {
            const accountsSchema = UpdateFeePolicyInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.feePolicy);
            assert(accountsSchema.operationsAuthority);
            assert(accountsSchema.eventAuthorityDirect);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for UpdateFeePolicyWithPendingChangeInstruction", () => {
            const schema = UpdateFeePolicyWithPendingChangeInstruction.getSchema();
            assert(schema.feePolicyId);
            assert(schema.feePolicyFlags);
            assert(schema.flatFeeInAsset);
            assert(schema.flatFeeInIssuedToken);
            assert(schema.percentFeeBps);
            assert(schema.feeCapAmount);
            assert(schema.minimumFeeAmount);
            assert(schema.rebateFlatAmount);
            assert(schema.rebateBps);
            assert(schema.rebateCapAmount);
            assert(schema.netFeeFloorZero);
            assert(schema.feeRecipientKey);
            assert(schema.effectiveFromUnixTimestamp);
            assert(schema.effectiveUntilUnixTimestamp);
        });

        it("has accounts schema for UpdateFeePolicyWithPendingChangeInstruction", () => {
            const accountsSchema = UpdateFeePolicyWithPendingChangeInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pendingConfigChange);
            assert(accountsSchema.feePolicy);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for RegisterReserveDestinationWithPendingInstruction", () => {
            const schema = RegisterReserveDestinationWithPendingInstruction.getSchema();
            assert(schema.destinationOwner);
            assert(schema.destinationFlags);
            assert(schema.withdrawalLimitPolicyId);
        });

        it("has accounts schema for RegisterReserveDestinationWithPendingInstruction", () => {
            const accountsSchema = RegisterReserveDestinationWithPendingInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pending);
            assert(accountsSchema.reserveDestination);
            assert(accountsSchema.assetMint);
            assert(accountsSchema.destinationTokenAccount);
            assert(accountsSchema.payer);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.withdrawalLimitPolicy);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for SetReserveDestinationStatusInstruction", () => {
            const schema = SetReserveDestinationStatusInstruction.getSchema();
            assert(schema.newStatus);
        });

        it("has accounts schema for SetReserveDestinationStatusInstruction", () => {
            const accountsSchema = SetReserveDestinationStatusInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.reserveDestination);
            assert(accountsSchema.authority);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for SetReserveDestinationStatusWithPendingInstruction", () => {
            const schema = SetReserveDestinationStatusWithPendingInstruction.getSchema();
            assert(schema.newStatus);
        });

        it("has accounts schema for SetReserveDestinationStatusWithPendingInstruction", () => {
            const accountsSchema = SetReserveDestinationStatusWithPendingInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pending);
            assert(accountsSchema.reserveDestination);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for WithdrawReserveInstruction", () => {
            const schema = WithdrawReserveInstruction.getSchema();
            assert(schema.amount);
            assert(schema.minimumDestinationAmount);
        });

        it("has accounts schema for WithdrawReserveInstruction", () => {
            const accountsSchema = WithdrawReserveInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pauseState);
            assert(accountsSchema.assetConfig);
            assert(accountsSchema.reserveDestination);
            assert(accountsSchema.reserveAssetTokenAccount);
            assert(accountsSchema.destinationAssetTokenAccount);
            assert(accountsSchema.assetMint);
            assert(accountsSchema.reserveAuthorityPda);
            assert(accountsSchema.assetTokenProgram);
            assert(accountsSchema.authority);
            assert(accountsSchema.assetPauseState);
            assert(accountsSchema.withdrawalLimitPolicy);
            assert(accountsSchema.withdrawalDailyUsageWindow);
            assert(accountsSchema.permissionRecord);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for AcceptConfigChangeInstruction", () => {
            const schema = AcceptConfigChangeInstruction.getSchema();
        });

        it("has accounts schema for AcceptConfigChangeInstruction", () => {
            const accountsSchema = AcceptConfigChangeInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.activationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pending);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for CancelConfigChangeInstruction", () => {
            const schema = CancelConfigChangeInstruction.getSchema();
        });

        it("has accounts schema for CancelConfigChangeInstruction", () => {
            const accountsSchema = CancelConfigChangeInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.activationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pending);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for CloseExpiredConfigChangeInstruction", () => {
            const schema = CloseExpiredConfigChangeInstruction.getSchema();
        });

        it("has accounts schema for CloseExpiredConfigChangeInstruction", () => {
            const accountsSchema = CloseExpiredConfigChangeInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.activationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pending);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for FreezeIssuedTokenAccountInstruction", () => {
            const schema = FreezeIssuedTokenAccountInstruction.getSchema();
            assert(schema.reasonCode);
            assert(schema.freezeFlags);
        });

        it("has accounts schema for FreezeIssuedTokenAccountInstruction", () => {
            const accountsSchema = FreezeIssuedTokenAccountInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.activation);
            assert(accountsSchema.freezeRecord);
            assert(accountsSchema.issuedTokenAccount);
            assert(accountsSchema.issuedTokenMint);
            assert(accountsSchema.freezeAuthorityPda);
            assert(accountsSchema.issuedTokenProgram);
            assert(accountsSchema.authority);
            assert(accountsSchema.payer);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.permissionRecord);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for InitializeModuleActivationStateInstruction", () => {
            const schema = InitializeModuleActivationStateInstruction.getSchema();
        });

        it("has accounts schema for InitializeModuleActivationStateInstruction", () => {
            const accountsSchema = InitializeModuleActivationStateInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.activation);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.payer);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.system);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for ProposeConfigChangeInstruction", () => {
            const schema = ProposeConfigChangeInstruction.getSchema();
            assert(schema.changeKind);
            assert(schema.riskClass);
            assert(schema.targetAccount);
            assert(schema.oldValueHash);
            assert(schema.newValueHash);
            assert(schema.executableAfterUnixTimestamp);
            assert(schema.expiresAtUnixTimestamp);
            assert(schema.proposerNonce);
        });

        it("has accounts schema for ProposeConfigChangeInstruction", () => {
            const accountsSchema = ProposeConfigChangeInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.activationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pending);
            assert(accountsSchema.payer);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.system);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for SetAssetPauseInstruction", () => {
            const schema = SetAssetPauseInstruction.getSchema();
            assert(schema.pauseBits);
            assert(schema.isClear);
            assert(schema.reasonCode);
            assert(schema.expiresAtSlot);
        });

        it("has accounts schema for SetAssetPauseInstruction", () => {
            const accountsSchema = SetAssetPauseInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.assetPauseState);
            assert(accountsSchema.assetMint);
            assert(accountsSchema.payer);
            assert(accountsSchema.authority);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for SetCounterpartyPauseInstruction", () => {
            const schema = SetCounterpartyPauseInstruction.getSchema();
            assert(schema.subject);
            assert(schema.scopeKind);
            assert(schema.scopeKey);
            assert(schema.isPaused);
            assert(schema.reasonCode);
        });

        it("has accounts schema for SetCounterpartyPauseInstruction", () => {
            const accountsSchema = SetCounterpartyPauseInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.permissionRecord);
            assert(accountsSchema.authority);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for SetExecutorPauseInstruction", () => {
            const schema = SetExecutorPauseInstruction.getSchema();
            assert(schema.subject);
            assert(schema.scopeKind);
            assert(schema.scopeKey);
            assert(schema.isPaused);
            assert(schema.reasonCode);
        });

        it("has accounts schema for SetExecutorPauseInstruction", () => {
            const accountsSchema = SetExecutorPauseInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.permissionRecord);
            assert(accountsSchema.authority);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for SetGlobalPauseInstruction", () => {
            const schema = SetGlobalPauseInstruction.getSchema();
            assert(schema.pauseBits);
            assert(schema.isClear);
            assert(schema.reasonCode);
            assert(schema.expiresAtSlot);
        });

        it("has accounts schema for SetGlobalPauseInstruction", () => {
            const accountsSchema = SetGlobalPauseInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pauseState);
            assert(accountsSchema.authority);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for SetModuleStatusInstruction", () => {
            const schema = SetModuleStatusInstruction.getSchema();
            assert(schema.moduleId);
            assert(schema.newStatus);
        });

        it("has accounts schema for SetModuleStatusInstruction", () => {
            const accountsSchema = SetModuleStatusInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.activation);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.authority);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for SetModuleStatusWithPendingChangeInstruction", () => {
            const schema = SetModuleStatusWithPendingChangeInstruction.getSchema();
            assert(schema.moduleId);
            assert(schema.newStatus);
        });

        it("has accounts schema for SetModuleStatusWithPendingChangeInstruction", () => {
            const accountsSchema = SetModuleStatusWithPendingChangeInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.activation);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pending);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for SetPathwayPauseInstruction", () => {
            const schema = SetPathwayPauseInstruction.getSchema();
            assert(schema.pathwayId);
            assert(schema.isPaused);
            assert(schema.reasonCode);
        });

        it("has accounts schema for SetPathwayPauseInstruction", () => {
            const accountsSchema = SetPathwayPauseInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pathwayPolicy);
            assert(accountsSchema.authority);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for ThawIssuedTokenAccountInstruction", () => {
            const schema = ThawIssuedTokenAccountInstruction.getSchema();
            assert(schema.reasonCode);
            assert(schema.thawFlags);
        });

        it("has accounts schema for ThawIssuedTokenAccountInstruction", () => {
            const accountsSchema = ThawIssuedTokenAccountInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.activation);
            assert(accountsSchema.freezeRecord);
            assert(accountsSchema.issuedTokenAccount);
            assert(accountsSchema.issuedTokenMint);
            assert(accountsSchema.freezeAuthorityPda);
            assert(accountsSchema.issuedTokenProgram);
            assert(accountsSchema.authority);
            assert(accountsSchema.permissionRecord);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for EnableLegacyMigrationInstruction", () => {
            const schema = EnableLegacyMigrationInstruction.getSchema();
            assert(schema.legacyProgram);
            assert(schema.legacyMint);
        });

        it("has accounts schema for EnableLegacyMigrationInstruction", () => {
            const accountsSchema = EnableLegacyMigrationInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.migrationConfig);
            assert(accountsSchema.payer);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.legacyMintAccount);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for MigrateLegacyToToken2022Instruction", () => {
            const schema = MigrateLegacyToToken2022Instruction.getSchema();
            assert(schema.amount);
        });

        it("has accounts schema for MigrateLegacyToToken2022Instruction", () => {
            const accountsSchema = MigrateLegacyToToken2022Instruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pauseState);
            assert(accountsSchema.migrationConfig);
            assert(accountsSchema.sourceLegacyTokenAccount);
            assert(accountsSchema.destinationIssuedTokenAccount);
            assert(accountsSchema.legacyMint);
            assert(accountsSchema.issuedTokenMint);
            assert(accountsSchema.mintAuthorityPda);
            assert(accountsSchema.legacyTokenProgram);
            assert(accountsSchema.issuedTokenProgram);
            assert(accountsSchema.holder);
            assert(accountsSchema.permissionRecord);
            assert(accountsSchema.issuedTokenControl);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for InitializeIssuedTokenControlInstruction", () => {
            const schema = InitializeIssuedTokenControlInstruction.getSchema();
            assert(schema.issuedTokenMint);
            assert(schema.issuedTokenProgram);
            assert(schema.reservedMintExtensionMask);
            assert(schema.reservedAccountExtensionMask);
            assert(schema.maxExtensionObservationAgeSlots);
        });

        it("has accounts schema for InitializeIssuedTokenControlInstruction", () => {
            const accountsSchema = InitializeIssuedTokenControlInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.issuedTokenControl);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.payer);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for RefreshAssetExtensionObservationInstruction", () => {
            const schema = RefreshAssetExtensionObservationInstruction.getSchema();
        });

        it("has accounts schema for RefreshAssetExtensionObservationInstruction", () => {
            const accountsSchema = RefreshAssetExtensionObservationInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.assetConfig);
            assert(accountsSchema.assetMint);
            assert(accountsSchema.operationsAuthority);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for VerifyIssuedTokenDeploymentInstruction", () => {
            const schema = VerifyIssuedTokenDeploymentInstruction.getSchema();
        });

        it("has accounts schema for VerifyIssuedTokenDeploymentInstruction", () => {
            const accountsSchema = VerifyIssuedTokenDeploymentInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.assetConfig);
            assert(accountsSchema.issuedTokenControl);
            assert(accountsSchema.issuedTokenMint);
            assert(accountsSchema.payer);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for RefreshIssuedTokenExtensionObservationInstruction", () => {
            const schema = RefreshIssuedTokenExtensionObservationInstruction.getSchema();
        });

        it("has accounts schema for RefreshIssuedTokenExtensionObservationInstruction", () => {
            const accountsSchema = RefreshIssuedTokenExtensionObservationInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.assetConfig);
            assert(accountsSchema.issuedTokenControl);
            assert(accountsSchema.issuedTokenMint);
            assert(accountsSchema.operationsAuthority);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for ConsumeInboundMessageInstruction", () => {
            const schema = ConsumeInboundMessageInstruction.getSchema();
            assert(schema.pathwayId);
            assert(schema.remoteChainKind);
            assert(schema.remoteDomainId);
            assert(schema.messageKind);
            assert(schema.sourceNonce);
            assert(schema.amount);
            assert(schema.expiresAtUnixTimestamp);
            assert(schema.sourceAsset);
            assert(schema.sender);
            assert(schema.recipient);
            assert(schema.providedMessageHash);
            assert(schema.signatures);
        });

        it("has accounts schema for ConsumeInboundMessageInstruction", () => {
            const accountsSchema = ConsumeInboundMessageInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pauseState);
            assert(accountsSchema.assetPauseState);
            assert(accountsSchema.pathwayPolicy);
            assert(accountsSchema.remoteDomainPolicy);
            assert(accountsSchema.signerSet);
            assert(accountsSchema.remoteNonceAccountIndex);
            assert(accountsSchema.recipientIssuedTokenAccount);
            assert(accountsSchema.issuedTokenMint);
            assert(accountsSchema.mintAuthorityPda);
            assert(accountsSchema.issuedTokenProgram);
            assert(accountsSchema.relayer);
            assert(accountsSchema.relayerPermissionRecord);
            assert(accountsSchema.issuedTokenControl);
            assert(accountsSchema.assetConfig);
            assert(accountsSchema.limitPolicy);
            assert(accountsSchema.hourlyUsageWindow);
            assert(accountsSchema.dailyUsageWindow);
            assert(accountsSchema.remoteDomainDailyUsageWindow);
            assert(accountsSchema.weeklyUsageWindow);
            assert(accountsSchema.monthlyUsageWindow);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for EmitOutboundMessageInstruction", () => {
            const schema = EmitOutboundMessageInstruction.getSchema();
            assert(schema.pathwayId);
            assert(schema.remoteChainKind);
            assert(schema.remoteDomainId);
            assert(schema.amount);
            assert(schema.minimumRemoteAmount);
            assert(schema.recipient);
            assert(schema.destinationAsset);
            assert(schema.messageKind);
            assert(schema.expiresAtUnixTimestamp);
        });

        it("has accounts schema for EmitOutboundMessageInstruction", () => {
            const accountsSchema = EmitOutboundMessageInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pauseState);
            assert(accountsSchema.assetPauseState);
            assert(accountsSchema.pathwayPolicy);
            assert(accountsSchema.remoteDomainPolicy);
            assert(accountsSchema.signerSet);
            assert(accountsSchema.remoteNonceAccountIndex);
            assert(accountsSchema.sourceIssuedTokenAccount);
            assert(accountsSchema.issuedTokenMint);
            assert(accountsSchema.issuedTokenProgram);
            assert(accountsSchema.sender);
            assert(accountsSchema.issuedTokenControl);
            assert(accountsSchema.senderPermissionRecord);
            assert(accountsSchema.assetConfig);
            assert(accountsSchema.feePolicy);
            assert(accountsSchema.feeRecipientTokenAccount);
            assert(accountsSchema.mintAuthorityPda);
            assert(accountsSchema.limitPolicy);
            assert(accountsSchema.hourlyUsageWindow);
            assert(accountsSchema.dailyUsageWindow);
            assert(accountsSchema.remoteDomainDailyUsageWindow);
            assert(accountsSchema.weeklyUsageWindow);
            assert(accountsSchema.monthlyUsageWindow);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for ExpireInboundMessageInstruction", () => {
            const schema = ExpireInboundMessageInstruction.getSchema();
            assert(schema.pathwayId);
            assert(schema.remoteChainKind);
            assert(schema.remoteDomainId);
            assert(schema.messageKind);
            assert(schema.sourceNonce);
            assert(schema.amount);
            assert(schema.expiresAtUnixTimestamp);
            assert(schema.sourceAsset);
            assert(schema.sender);
            assert(schema.recipient);
            assert(schema.providedMessageHash);
            assert(schema.signatures);
        });

        it("has accounts schema for ExpireInboundMessageInstruction", () => {
            const accountsSchema = ExpireInboundMessageInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pathwayPolicy);
            assert(accountsSchema.remoteDomainPolicy);
            assert(accountsSchema.signerSet);
            assert(accountsSchema.remoteNonceAccountIndex);
            assert(accountsSchema.limitPolicy);
            assert(accountsSchema.recipientIssuedTokenAccount);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for ReclaimExpiredOutboundInstruction", () => {
            const schema = ReclaimExpiredOutboundInstruction.getSchema();
            assert(schema.pathwayId);
            assert(schema.remoteChainKind);
            assert(schema.remoteDomainId);
            assert(schema.messageKind);
            assert(schema.sourceNonce);
            assert(schema.amount);
            assert(schema.expiresAtUnixTimestamp);
            assert(schema.destinationAsset);
            assert(schema.sender);
            assert(schema.recipient);
            assert(schema.retirementReason);
            assert(schema.expiredAtUnixTimestamp);
            assert(schema.expiredAtSlotOrBlock);
            assert(schema.providedEpochFreeContentHash);
            assert(schema.providedReclaimDigest);
            assert(schema.signatures);
        });

        it("has accounts schema for ReclaimExpiredOutboundInstruction", () => {
            const accountsSchema = ReclaimExpiredOutboundInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pauseState);
            assert(accountsSchema.assetPauseState);
            assert(accountsSchema.pathwayPolicy);
            assert(accountsSchema.remoteDomainPolicy);
            assert(accountsSchema.signerSet);
            assert(accountsSchema.remoteNonceAccountIndex);
            assert(accountsSchema.outboundReclaimRecord);
            assert(accountsSchema.senderIssuedTokenAccount);
            assert(accountsSchema.issuedTokenMint);
            assert(accountsSchema.mintAuthorityPda);
            assert(accountsSchema.issuedTokenProgram);
            assert(accountsSchema.issuedTokenControl);
            assert(accountsSchema.payer);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for RelaxRemoteDomainPauseInstruction", () => {
            const schema = RelaxRemoteDomainPauseInstruction.getSchema();
            assert(schema.remoteChainKind);
            assert(schema.remoteDomainId);
            assert(schema.pauseBitsToClear);
            assert(schema.reasonCode);
        });

        it("has accounts schema for RelaxRemoteDomainPauseInstruction", () => {
            const accountsSchema = RelaxRemoteDomainPauseInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.remoteDomainPolicy);
            assert(accountsSchema.authority);
            assert(accountsSchema.authorityPermissionRecord);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for RegisterCrossChainSignerSetInstruction", () => {
            const schema = RegisterCrossChainSignerSetInstruction.getSchema();
            assert(schema.signerSetId);
            assert(schema.threshold);
            assert(schema.signerCount);
            assert(schema.signerRoot);
            assert(schema.validAfterUnixTimestamp);
            assert(schema.expiresAtUnixTimestamp);
        });

        it("has accounts schema for RegisterCrossChainSignerSetInstruction", () => {
            const accountsSchema = RegisterCrossChainSignerSetInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pending);
            assert(accountsSchema.signerSet);
            assert(accountsSchema.payer);
            assert(accountsSchema.registeringAuthority);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.authorityPermissionRecord);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for RegisterRemoteDomainPolicyInstruction", () => {
            const schema = RegisterRemoteDomainPolicyInstruction.getSchema();
            assert(schema.remoteChainKind);
            assert(schema.remoteDomainId);
            assert(schema.minimumAttestationThreshold);
            assert(schema.requiredFinalityDepth);
            assert(schema.messageExpirySeconds);
            assert(schema.perMessageMaximum);
            assert(schema.perDayMaximum);
            assert(schema.remoteDomainSeparator);
            assert(schema.remoteChanceryContract);
            assert(schema.remoteIssuedToken);
            assert(schema.signerSetId);
            assert(schema.mode);
            assert(schema.remoteAsset);
            assert(schema.localAssetMint);
        });

        it("has accounts schema for RegisterRemoteDomainPolicyInstruction", () => {
            const accountsSchema = RegisterRemoteDomainPolicyInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.remoteDomainPolicy);
            assert(accountsSchema.payer);
            assert(accountsSchema.authority);
            assert(accountsSchema.authorityPermissionRecord);
            assert(accountsSchema.signerSet);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.remoteNonceAccountIndex);
            assert(accountsSchema.remoteDailyUsageWindow);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for RestrictRemoteDomainPauseInstruction", () => {
            const schema = RestrictRemoteDomainPauseInstruction.getSchema();
            assert(schema.remoteChainKind);
            assert(schema.remoteDomainId);
            assert(schema.pauseBitsToSet);
            assert(schema.reasonCode);
        });

        it("has accounts schema for RestrictRemoteDomainPauseInstruction", () => {
            const accountsSchema = RestrictRemoteDomainPauseInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.remoteDomainPolicy);
            assert(accountsSchema.authority);
            assert(accountsSchema.authorityPermissionRecord);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for RotateCrossChainSignerSetInstruction", () => {
            const schema = RotateCrossChainSignerSetInstruction.getSchema();
            assert(schema.signerSetId);
            assert(schema.reasonCode);
        });

        it("has accounts schema for RotateCrossChainSignerSetInstruction", () => {
            const accountsSchema = RotateCrossChainSignerSetInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.signerSet);
            assert(accountsSchema.authority);
            assert(accountsSchema.authorityPermissionRecord);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for UpdateRemoteDomainPolicyInstruction", () => {
            const schema = UpdateRemoteDomainPolicyInstruction.getSchema();
            assert(schema.remoteChainKind);
            assert(schema.remoteDomainId);
            assert(schema.minimumAttestationThreshold);
            assert(schema.requiredFinalityDepth);
            assert(schema.messageExpirySeconds);
            assert(schema.perMessageMaximum);
            assert(schema.perDayMaximum);
            assert(schema.remoteDomainSeparator);
            assert(schema.remoteChanceryContract);
            assert(schema.remoteIssuedToken);
            assert(schema.signerSetId);
            assert(schema.mode);
            assert(schema.remoteAsset);
        });

        it("has accounts schema for UpdateRemoteDomainPolicyInstruction", () => {
            const accountsSchema = UpdateRemoteDomainPolicyInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.remoteDomainPolicy);
            assert(accountsSchema.authority);
            assert(accountsSchema.authorityPermissionRecord);
            assert(accountsSchema.signerSet);
            assert(accountsSchema.payer);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.remoteDailyUsageWindow);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for UpdateRemoteDomainPolicyWithPendingChangeInstruction", () => {
            const schema = UpdateRemoteDomainPolicyWithPendingChangeInstruction.getSchema();
            assert(schema.remoteChainKind);
            assert(schema.remoteDomainId);
            assert(schema.minimumAttestationThreshold);
            assert(schema.requiredFinalityDepth);
            assert(schema.messageExpirySeconds);
            assert(schema.perMessageMaximum);
            assert(schema.perDayMaximum);
            assert(schema.remoteDomainSeparator);
            assert(schema.remoteChanceryContract);
            assert(schema.remoteIssuedToken);
            assert(schema.signerSetId);
            assert(schema.mode);
            assert(schema.remoteAsset);
        });

        it("has accounts schema for UpdateRemoteDomainPolicyWithPendingChangeInstruction", () => {
            const accountsSchema = UpdateRemoteDomainPolicyWithPendingChangeInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pending);
            assert(accountsSchema.remoteDomainPolicy);
            assert(accountsSchema.signerSet);
            assert(accountsSchema.updatingAuthority);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.authorityPermissionRecord);
            assert(accountsSchema.payer);
            assert(accountsSchema.systemProgram);
            assert(accountsSchema.remoteDailyUsageWindow);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

        it("has schema for RelaxRemoteDomainPauseWithPendingChangeInstruction", () => {
            const schema = RelaxRemoteDomainPauseWithPendingChangeInstruction.getSchema();
            assert(schema.remoteChainKind);
            assert(schema.remoteDomainId);
            assert(schema.pauseBitsToClear);
            assert(schema.reasonCode);
        });

        it("has accounts schema for RelaxRemoteDomainPauseWithPendingChangeInstruction", () => {
            const accountsSchema = RelaxRemoteDomainPauseWithPendingChangeInstruction.getAccountsSchema();
            assert(accountsSchema);
            assert(accountsSchema.moduleActivationState);
            assert(accountsSchema.chanceryConfig);
            assert(accountsSchema.eventAuthority);
            assert(accountsSchema.pending);
            assert(accountsSchema.remoteDomainPolicy);
            assert(accountsSchema.signerSet);
            assert(accountsSchema.relaxingAuthority);
            assert(accountsSchema.governanceAuthority);
            assert(accountsSchema.authorityPermissionRecord);
            assert(accountsSchema.rentRefundRecipient);
            assert(accountsSchema.eventProgram);
        });

    });
});
