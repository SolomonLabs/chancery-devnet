// @generated-idl
// ============================================================================
// AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY
// ============================================================================
//
// Generator:     solana-libgen v1.0.0
// Generated:     deterministic-codegen
// Category:      program
// File:          ChanceryProgram.ts
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
import type { IsEncodable, EncodableProps } from "@solomon-labs/solana-codec";
import { Transaction } from "@solomon-labs/solana-transactions";
import type { PublicKeyLike, SolanaAddressLike, SolanaBlockhash } from "@solomon-labs/types";

import { AssetPauseState } from "./accounts/AssetPauseState";
import { BasicFreezeRecord } from "./accounts/BasicFreezeRecord";
import { ModuleActivationState } from "./accounts/ModuleActivationState";
import { PauseState } from "./accounts/PauseState";
import { PendingConfigChange } from "./accounts/PendingConfigChange";
import { AssetConfig } from "./accounts/AssetConfig";
import { AuthorityTransfer } from "./accounts/AuthorityTransfer";
import { ChanceryConfig } from "./accounts/ChanceryConfig";
import { CrossChainSignerSet } from "./accounts/CrossChainSignerSet";
import { OutboundReclaimRecord } from "./accounts/OutboundReclaimRecord";
import { RemoteDomainPolicy } from "./accounts/RemoteDomainPolicy";
import { RemoteNonce } from "./accounts/RemoteNonce";
import { EvidencePolicy } from "./accounts/EvidencePolicy";
import { FeePolicy } from "./accounts/FeePolicy";
import { IssuedTokenControl } from "./accounts/IssuedTokenControl";
import { LimitPolicy } from "./accounts/LimitPolicy";
import { UsageWindow } from "./accounts/UsageWindow";
import { LegacyMigrationConfig } from "./accounts/LegacyMigrationConfig";
import { PathwayPolicy } from "./accounts/PathwayPolicy";
import { PermissionRecord } from "./accounts/PermissionRecord";
import { ReserveDestination } from "./accounts/ReserveDestination";
import { SettlementIntent } from "./accounts/SettlementIntent";
import { SettlementPolicy } from "./accounts/SettlementPolicy";
import { PROGRAM_ID } from "./constants";
import { AcceptAuthorityTransferInstruction } from "./instructions/AcceptAuthorityTransferInstruction";
import { InitializeChanceryInstruction } from "./instructions/InitializeChanceryInstruction";
import { ProposeAuthorityTransferInstruction } from "./instructions/ProposeAuthorityTransferInstruction";
import { RegisterAssetInstruction } from "./instructions/RegisterAssetInstruction";
import { SetAssetModeInstruction } from "./instructions/SetAssetModeInstruction";
import { SetAssetModeWithPendingChangeInstruction } from "./instructions/SetAssetModeWithPendingChangeInstruction";
import { UpdateAssetConfigInstruction } from "./instructions/UpdateAssetConfigInstruction";
import { UpdateAssetConfigWithPendingChangeInstruction } from "./instructions/UpdateAssetConfigWithPendingChangeInstruction";
import { EmitInstruction } from "./instructions/EmitInstruction";
import { UpsertPermissionInstruction } from "./instructions/UpsertPermissionInstruction";
import { RevokePermissionInstruction } from "./instructions/RevokePermissionInstruction";
import { UpsertPermissionWithPendingChangeInstruction } from "./instructions/UpsertPermissionWithPendingChangeInstruction";
import { RegisterPathwayPolicyInstruction } from "./instructions/RegisterPathwayPolicyInstruction";
import { UpdatePathwayPolicyInstruction } from "./instructions/UpdatePathwayPolicyInstruction";
import { UpdatePathwayPolicyWithPendingChangeInstruction } from "./instructions/UpdatePathwayPolicyWithPendingChangeInstruction";
import { CreateSettlementIntentInstruction } from "./instructions/CreateSettlementIntentInstruction";
import { CancelSettlementIntentInstruction } from "./instructions/CancelSettlementIntentInstruction";
import { CloseExpiredSettlementIntentInstruction } from "./instructions/CloseExpiredSettlementIntentInstruction";
import { MintDelegatedInstruction } from "./instructions/MintDelegatedInstruction";
import { MintDirectInstruction } from "./instructions/MintDirectInstruction";
import { MintTrilateralInstruction } from "./instructions/MintTrilateralInstruction";
import { RedeemDelegatedInstruction } from "./instructions/RedeemDelegatedInstruction";
import { RedeemDirectInstruction } from "./instructions/RedeemDirectInstruction";
import { RedeemTrilateralInstruction } from "./instructions/RedeemTrilateralInstruction";
import { RegisterSettlementPolicyInstruction } from "./instructions/RegisterSettlementPolicyInstruction";
import { RegisterLimitPolicyInstruction } from "./instructions/RegisterLimitPolicyInstruction";
import { UpdateLimitPolicyInstruction } from "./instructions/UpdateLimitPolicyInstruction";
import { UpdateLimitPolicyWithPendingChangeInstruction } from "./instructions/UpdateLimitPolicyWithPendingChangeInstruction";
import { RegisterEvidencePolicyInstruction } from "./instructions/RegisterEvidencePolicyInstruction";
import { UpdateEvidencePolicyInstruction } from "./instructions/UpdateEvidencePolicyInstruction";
import { UpdateEvidencePolicyWithPendingChangeInstruction } from "./instructions/UpdateEvidencePolicyWithPendingChangeInstruction";
import { RegisterFeePolicyInstruction } from "./instructions/RegisterFeePolicyInstruction";
import { UpdateFeePolicyInstruction } from "./instructions/UpdateFeePolicyInstruction";
import { UpdateFeePolicyWithPendingChangeInstruction } from "./instructions/UpdateFeePolicyWithPendingChangeInstruction";
import { RegisterReserveDestinationWithPendingInstruction } from "./instructions/RegisterReserveDestinationWithPendingInstruction";
import { SetReserveDestinationStatusInstruction } from "./instructions/SetReserveDestinationStatusInstruction";
import { SetReserveDestinationStatusWithPendingInstruction } from "./instructions/SetReserveDestinationStatusWithPendingInstruction";
import { WithdrawReserveInstruction } from "./instructions/WithdrawReserveInstruction";
import { AcceptConfigChangeInstruction } from "./instructions/AcceptConfigChangeInstruction";
import { CancelConfigChangeInstruction } from "./instructions/CancelConfigChangeInstruction";
import { CloseExpiredConfigChangeInstruction } from "./instructions/CloseExpiredConfigChangeInstruction";
import { FreezeIssuedTokenAccountInstruction } from "./instructions/FreezeIssuedTokenAccountInstruction";
import { InitializeModuleActivationStateInstruction } from "./instructions/InitializeModuleActivationStateInstruction";
import { ProposeConfigChangeInstruction } from "./instructions/ProposeConfigChangeInstruction";
import { SetAssetPauseInstruction } from "./instructions/SetAssetPauseInstruction";
import { SetCounterpartyPauseInstruction } from "./instructions/SetCounterpartyPauseInstruction";
import { SetExecutorPauseInstruction } from "./instructions/SetExecutorPauseInstruction";
import { SetGlobalPauseInstruction } from "./instructions/SetGlobalPauseInstruction";
import { SetModuleStatusInstruction } from "./instructions/SetModuleStatusInstruction";
import { SetModuleStatusWithPendingChangeInstruction } from "./instructions/SetModuleStatusWithPendingChangeInstruction";
import { SetPathwayPauseInstruction } from "./instructions/SetPathwayPauseInstruction";
import { ThawIssuedTokenAccountInstruction } from "./instructions/ThawIssuedTokenAccountInstruction";
import { EnableLegacyMigrationInstruction } from "./instructions/EnableLegacyMigrationInstruction";
import { MigrateLegacyToToken2022Instruction } from "./instructions/MigrateLegacyToToken2022Instruction";
import { InitializeIssuedTokenControlInstruction } from "./instructions/InitializeIssuedTokenControlInstruction";
import { RefreshAssetExtensionObservationInstruction } from "./instructions/RefreshAssetExtensionObservationInstruction";
import { VerifyIssuedTokenDeploymentInstruction } from "./instructions/VerifyIssuedTokenDeploymentInstruction";
import { RefreshIssuedTokenExtensionObservationInstruction } from "./instructions/RefreshIssuedTokenExtensionObservationInstruction";
import { ConsumeInboundMessageInstruction } from "./instructions/ConsumeInboundMessageInstruction";
import { EmitOutboundMessageInstruction } from "./instructions/EmitOutboundMessageInstruction";
import { ExpireInboundMessageInstruction } from "./instructions/ExpireInboundMessageInstruction";
import { ReclaimExpiredOutboundInstruction } from "./instructions/ReclaimExpiredOutboundInstruction";
import { RelaxRemoteDomainPauseInstruction } from "./instructions/RelaxRemoteDomainPauseInstruction";
import { RegisterCrossChainSignerSetInstruction } from "./instructions/RegisterCrossChainSignerSetInstruction";
import { RegisterRemoteDomainPolicyInstruction } from "./instructions/RegisterRemoteDomainPolicyInstruction";
import { RestrictRemoteDomainPauseInstruction } from "./instructions/RestrictRemoteDomainPauseInstruction";
import { RotateCrossChainSignerSetInstruction } from "./instructions/RotateCrossChainSignerSetInstruction";
import { UpdateRemoteDomainPolicyInstruction } from "./instructions/UpdateRemoteDomainPolicyInstruction";
import { UpdateRemoteDomainPolicyWithPendingChangeInstruction } from "./instructions/UpdateRemoteDomainPolicyWithPendingChangeInstruction";
import { RelaxRemoteDomainPauseWithPendingChangeInstruction } from "./instructions/RelaxRemoteDomainPauseWithPendingChangeInstruction";

export class ChanceryProgram {
    static readonly programId = PROGRAM_ID;

    static async getAssetPauseState(address: SolanaAddressLike): Promise<AssetPauseState> {
        return AssetPauseState.get(address);
    }

    static async getBasicFreezeRecord(address: SolanaAddressLike): Promise<BasicFreezeRecord> {
        return BasicFreezeRecord.get(address);
    }

    static async getModuleActivationState(address: SolanaAddressLike): Promise<ModuleActivationState> {
        return ModuleActivationState.get(address);
    }

    static async getPauseState(address: SolanaAddressLike): Promise<PauseState> {
        return PauseState.get(address);
    }

    static async getPendingConfigChange(address: SolanaAddressLike): Promise<PendingConfigChange> {
        return PendingConfigChange.get(address);
    }

    static async getAssetConfig(address: SolanaAddressLike): Promise<AssetConfig> {
        return AssetConfig.get(address);
    }

    static async getAuthorityTransfer(address: SolanaAddressLike): Promise<AuthorityTransfer> {
        return AuthorityTransfer.get(address);
    }

    static async getChanceryConfig(address: SolanaAddressLike): Promise<ChanceryConfig> {
        return ChanceryConfig.get(address);
    }

    static async getCrossChainSignerSet(address: SolanaAddressLike): Promise<CrossChainSignerSet> {
        return CrossChainSignerSet.get(address);
    }

    static async getOutboundReclaimRecord(address: SolanaAddressLike): Promise<OutboundReclaimRecord> {
        return OutboundReclaimRecord.get(address);
    }

    static async getRemoteDomainPolicy(address: SolanaAddressLike): Promise<RemoteDomainPolicy> {
        return RemoteDomainPolicy.get(address);
    }

    static async getRemoteNonce(address: SolanaAddressLike): Promise<RemoteNonce> {
        return RemoteNonce.get(address);
    }

    static async getEvidencePolicy(address: SolanaAddressLike): Promise<EvidencePolicy> {
        return EvidencePolicy.get(address);
    }

    static async getFeePolicy(address: SolanaAddressLike): Promise<FeePolicy> {
        return FeePolicy.get(address);
    }

    static async getIssuedTokenControl(address: SolanaAddressLike): Promise<IssuedTokenControl> {
        return IssuedTokenControl.get(address);
    }

    static async getLimitPolicy(address: SolanaAddressLike): Promise<LimitPolicy> {
        return LimitPolicy.get(address);
    }

    static async getUsageWindow(address: SolanaAddressLike): Promise<UsageWindow> {
        return UsageWindow.get(address);
    }

    static async getLegacyMigrationConfig(address: SolanaAddressLike): Promise<LegacyMigrationConfig> {
        return LegacyMigrationConfig.get(address);
    }

    static async getPathwayPolicy(address: SolanaAddressLike): Promise<PathwayPolicy> {
        return PathwayPolicy.get(address);
    }

    static async getPermissionRecord(address: SolanaAddressLike): Promise<PermissionRecord> {
        return PermissionRecord.get(address);
    }

    static async getReserveDestination(address: SolanaAddressLike): Promise<ReserveDestination> {
        return ReserveDestination.get(address);
    }

    static async getSettlementIntent(address: SolanaAddressLike): Promise<SettlementIntent> {
        return SettlementIntent.get(address);
    }

    static async getSettlementPolicy(address: SolanaAddressLike): Promise<SettlementPolicy> {
        return SettlementPolicy.get(address);
    }

    static async createAcceptAuthorityTransferTransaction(
        props: EncodableProps<AcceptAuthorityTransferInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new AcceptAuthorityTransferInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.newAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createInitializeChanceryTransaction(
        props: EncodableProps<InitializeChanceryInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new InitializeChanceryInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.payer;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createProposeAuthorityTransferTransaction(
        props: EncodableProps<ProposeAuthorityTransferInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new ProposeAuthorityTransferInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.payer;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createRegisterAssetTransaction(
        props: EncodableProps<RegisterAssetInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new RegisterAssetInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.payer;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createSetAssetModeTransaction(
        props: EncodableProps<SetAssetModeInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new SetAssetModeInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.authority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createSetAssetModeWithPendingChangeTransaction(
        props: EncodableProps<SetAssetModeWithPendingChangeInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new SetAssetModeWithPendingChangeInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.governanceAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createUpdateAssetConfigTransaction(
        props: EncodableProps<UpdateAssetConfigInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new UpdateAssetConfigInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.operationsAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createUpdateAssetConfigWithPendingChangeTransaction(
        props: EncodableProps<UpdateAssetConfigWithPendingChangeInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new UpdateAssetConfigWithPendingChangeInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.governanceAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createEmitTransaction(
        props: EncodableProps<EmitInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new EmitInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.eventAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createUpsertPermissionTransaction(
        props: EncodableProps<UpsertPermissionInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new UpsertPermissionInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.grantingAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createRevokePermissionTransaction(
        props: EncodableProps<RevokePermissionInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new RevokePermissionInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.revokingAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createUpsertPermissionWithPendingChangeTransaction(
        props: EncodableProps<UpsertPermissionWithPendingChangeInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new UpsertPermissionWithPendingChangeInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.payer;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createRegisterPathwayPolicyTransaction(
        props: EncodableProps<RegisterPathwayPolicyInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new RegisterPathwayPolicyInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.payer;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createUpdatePathwayPolicyTransaction(
        props: EncodableProps<UpdatePathwayPolicyInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new UpdatePathwayPolicyInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.operationsAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createUpdatePathwayPolicyWithPendingChangeTransaction(
        props: EncodableProps<UpdatePathwayPolicyWithPendingChangeInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new UpdatePathwayPolicyWithPendingChangeInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.governanceAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createCreateSettlementIntentTransaction(
        props: EncodableProps<CreateSettlementIntentInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new CreateSettlementIntentInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.payer;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createCancelSettlementIntentTransaction(
        props: EncodableProps<CancelSettlementIntentInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new CancelSettlementIntentInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.principalA;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createCloseExpiredSettlementIntentTransaction(
        props: EncodableProps<CloseExpiredSettlementIntentInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new CloseExpiredSettlementIntentInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        const feePayer: PublicKeyLike | undefined = options.feePayer;
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createMintDelegatedTransaction(
        props: EncodableProps<MintDelegatedInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new MintDelegatedInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.executor;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createMintDirectTransaction(
        props: EncodableProps<MintDirectInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new MintDirectInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.principal;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createMintTrilateralTransaction(
        props: EncodableProps<MintTrilateralInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new MintTrilateralInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.executor;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createRedeemDelegatedTransaction(
        props: EncodableProps<RedeemDelegatedInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new RedeemDelegatedInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.executor;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createRedeemDirectTransaction(
        props: EncodableProps<RedeemDirectInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new RedeemDirectInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.principal;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createRedeemTrilateralTransaction(
        props: EncodableProps<RedeemTrilateralInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new RedeemTrilateralInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.executor;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createRegisterSettlementPolicyTransaction(
        props: EncodableProps<RegisterSettlementPolicyInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new RegisterSettlementPolicyInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.payer;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createRegisterLimitPolicyTransaction(
        props: EncodableProps<RegisterLimitPolicyInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new RegisterLimitPolicyInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.payer;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createUpdateLimitPolicyTransaction(
        props: EncodableProps<UpdateLimitPolicyInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new UpdateLimitPolicyInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.operationsAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createUpdateLimitPolicyWithPendingChangeTransaction(
        props: EncodableProps<UpdateLimitPolicyWithPendingChangeInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new UpdateLimitPolicyWithPendingChangeInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.governanceAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createRegisterEvidencePolicyTransaction(
        props: EncodableProps<RegisterEvidencePolicyInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new RegisterEvidencePolicyInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.payer;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createUpdateEvidencePolicyTransaction(
        props: EncodableProps<UpdateEvidencePolicyInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new UpdateEvidencePolicyInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.operationsAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createUpdateEvidencePolicyWithPendingChangeTransaction(
        props: EncodableProps<UpdateEvidencePolicyWithPendingChangeInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new UpdateEvidencePolicyWithPendingChangeInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.governanceAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createRegisterFeePolicyTransaction(
        props: EncodableProps<RegisterFeePolicyInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new RegisterFeePolicyInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.payer;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createUpdateFeePolicyTransaction(
        props: EncodableProps<UpdateFeePolicyInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new UpdateFeePolicyInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.operationsAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createUpdateFeePolicyWithPendingChangeTransaction(
        props: EncodableProps<UpdateFeePolicyWithPendingChangeInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new UpdateFeePolicyWithPendingChangeInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.governanceAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createRegisterReserveDestinationWithPendingTransaction(
        props: EncodableProps<RegisterReserveDestinationWithPendingInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new RegisterReserveDestinationWithPendingInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.payer;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createSetReserveDestinationStatusTransaction(
        props: EncodableProps<SetReserveDestinationStatusInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new SetReserveDestinationStatusInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.authority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createSetReserveDestinationStatusWithPendingTransaction(
        props: EncodableProps<SetReserveDestinationStatusWithPendingInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new SetReserveDestinationStatusWithPendingInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.governanceAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createWithdrawReserveTransaction(
        props: EncodableProps<WithdrawReserveInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new WithdrawReserveInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.authority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createAcceptConfigChangeTransaction(
        props: EncodableProps<AcceptConfigChangeInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new AcceptConfigChangeInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.governanceAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createCancelConfigChangeTransaction(
        props: EncodableProps<CancelConfigChangeInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new CancelConfigChangeInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.governanceAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createCloseExpiredConfigChangeTransaction(
        props: EncodableProps<CloseExpiredConfigChangeInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new CloseExpiredConfigChangeInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        const feePayer: PublicKeyLike | undefined = options.feePayer;
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createFreezeIssuedTokenAccountTransaction(
        props: EncodableProps<FreezeIssuedTokenAccountInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new FreezeIssuedTokenAccountInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.authority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createInitializeModuleActivationStateTransaction(
        props: EncodableProps<InitializeModuleActivationStateInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new InitializeModuleActivationStateInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.payer;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createProposeConfigChangeTransaction(
        props: EncodableProps<ProposeConfigChangeInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new ProposeConfigChangeInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.payer;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createSetAssetPauseTransaction(
        props: EncodableProps<SetAssetPauseInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new SetAssetPauseInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.payer;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createSetCounterpartyPauseTransaction(
        props: EncodableProps<SetCounterpartyPauseInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new SetCounterpartyPauseInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.authority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createSetExecutorPauseTransaction(
        props: EncodableProps<SetExecutorPauseInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new SetExecutorPauseInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.authority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createSetGlobalPauseTransaction(
        props: EncodableProps<SetGlobalPauseInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new SetGlobalPauseInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.authority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createSetModuleStatusTransaction(
        props: EncodableProps<SetModuleStatusInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new SetModuleStatusInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.authority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createSetModuleStatusWithPendingChangeTransaction(
        props: EncodableProps<SetModuleStatusWithPendingChangeInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new SetModuleStatusWithPendingChangeInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.governanceAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createSetPathwayPauseTransaction(
        props: EncodableProps<SetPathwayPauseInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new SetPathwayPauseInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.authority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createThawIssuedTokenAccountTransaction(
        props: EncodableProps<ThawIssuedTokenAccountInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new ThawIssuedTokenAccountInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.authority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createEnableLegacyMigrationTransaction(
        props: EncodableProps<EnableLegacyMigrationInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new EnableLegacyMigrationInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.payer;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createMigrateLegacyToToken2022Transaction(
        props: EncodableProps<MigrateLegacyToToken2022Instruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new MigrateLegacyToToken2022Instruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.holder;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createInitializeIssuedTokenControlTransaction(
        props: EncodableProps<InitializeIssuedTokenControlInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new InitializeIssuedTokenControlInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.payer;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createRefreshAssetExtensionObservationTransaction(
        props: EncodableProps<RefreshAssetExtensionObservationInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new RefreshAssetExtensionObservationInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.operationsAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createVerifyIssuedTokenDeploymentTransaction(
        props: EncodableProps<VerifyIssuedTokenDeploymentInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new VerifyIssuedTokenDeploymentInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.payer;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createRefreshIssuedTokenExtensionObservationTransaction(
        props: EncodableProps<RefreshIssuedTokenExtensionObservationInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new RefreshIssuedTokenExtensionObservationInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.operationsAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createConsumeInboundMessageTransaction(
        props: EncodableProps<ConsumeInboundMessageInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new ConsumeInboundMessageInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.relayer;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createEmitOutboundMessageTransaction(
        props: EncodableProps<EmitOutboundMessageInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new EmitOutboundMessageInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.sender;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createExpireInboundMessageTransaction(
        props: EncodableProps<ExpireInboundMessageInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new ExpireInboundMessageInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        const feePayer: PublicKeyLike | undefined = options.feePayer;
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createReclaimExpiredOutboundTransaction(
        props: EncodableProps<ReclaimExpiredOutboundInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new ReclaimExpiredOutboundInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.payer;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createRelaxRemoteDomainPauseTransaction(
        props: EncodableProps<RelaxRemoteDomainPauseInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new RelaxRemoteDomainPauseInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.authority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createRegisterCrossChainSignerSetTransaction(
        props: EncodableProps<RegisterCrossChainSignerSetInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new RegisterCrossChainSignerSetInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.payer;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createRegisterRemoteDomainPolicyTransaction(
        props: EncodableProps<RegisterRemoteDomainPolicyInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new RegisterRemoteDomainPolicyInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.payer;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createRestrictRemoteDomainPauseTransaction(
        props: EncodableProps<RestrictRemoteDomainPauseInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new RestrictRemoteDomainPauseInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.authority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createRotateCrossChainSignerSetTransaction(
        props: EncodableProps<RotateCrossChainSignerSetInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new RotateCrossChainSignerSetInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.authority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createUpdateRemoteDomainPolicyTransaction(
        props: EncodableProps<UpdateRemoteDomainPolicyInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new UpdateRemoteDomainPolicyInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.authority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createUpdateRemoteDomainPolicyWithPendingChangeTransaction(
        props: EncodableProps<UpdateRemoteDomainPolicyWithPendingChangeInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new UpdateRemoteDomainPolicyWithPendingChangeInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.updatingAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

    static async createRelaxRemoteDomainPauseWithPendingChangeTransaction(
        props: EncodableProps<RelaxRemoteDomainPauseWithPendingChangeInstruction>,
        options: {
            feePayer?: PublicKeyLike;
            recentBlockhash?: SolanaBlockhash;
        } = {}
    ): Promise<Transaction> {
        const instruction = new RelaxRemoteDomainPauseWithPendingChangeInstruction(props);
        const instructions: IsEncodable[] = [instruction];
        let feePayer: PublicKeyLike | undefined = options.feePayer;
        if (!feePayer) {
            feePayer = props.relaxingAuthority;
        }
        return new Transaction({
            feePayer,
            recentBlockhash: options.recentBlockhash,
            instructions
        });
    }

}
