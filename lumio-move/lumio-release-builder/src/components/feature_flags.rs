// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{components::get_signer_arg, utils::*};
use anyhow::Result;
use lumio_crypto::HashValue;
use lumio_types::on_chain_config::{FeatureFlag as LumioFeatureFlag, Features as LumioFeatures};
use move_model::{code_writer::CodeWriter, emit, emitln, model::Loc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Deserialize, PartialEq, Eq, Serialize, Debug)]
pub struct Features {
    #[serde(default)]
    pub enabled: Vec<FeatureFlag>,
    #[serde(default)]
    pub disabled: Vec<FeatureFlag>,
}

impl Features {
    pub fn empty() -> Self {
        Self {
            enabled: vec![],
            disabled: vec![],
        }
    }

    pub fn squash(&mut self, rhs: Self) {
        let mut enabled: HashSet<_> = self.enabled.iter().cloned().collect();
        let mut disabled: HashSet<_> = self.disabled.iter().cloned().collect();
        let to_enable: HashSet<_> = rhs.enabled.into_iter().collect();
        let to_disable: HashSet<_> = rhs.disabled.into_iter().collect();

        disabled = disabled.difference(&to_enable).cloned().collect();
        enabled.extend(to_enable);

        enabled = enabled.difference(&to_disable).cloned().collect();
        disabled.extend(to_disable);

        self.enabled = enabled.into_iter().collect();
        self.disabled = disabled.into_iter().collect();
    }

    pub fn is_empty(&self) -> bool {
        self.enabled.is_empty() && self.disabled.is_empty()
    }
}

#[derive(Clone, Debug, Deserialize, EnumIter, PartialEq, Eq, Serialize, Hash)]
#[allow(non_camel_case_types)]
#[serde(rename_all = "snake_case")]
pub enum FeatureFlag {
    CodeDependencyCheck,
    CollectAndDistributeGasFees,
    TreatFriendAsPrivate,
    Sha512AndRipeMd160Natives,
    LumioStdChainIdNatives,
    VMBinaryFormatV6,
    MultiEd25519PkValidateV2Natives,
    Blake2b256Native,
    ResourceGroups,
    MultisigAccounts,
    DelegationPools,
    CryptographyAlgebraNatives,
    Bls12381Structures,
    Ed25519PubkeyValidateReturnFalseWrongLength,
    StructConstructors,
    PeriodicalRewardRateReduction,
    PartialGovernanceVoting,
    SignatureCheckerV2,
    StorageSlotMetadata,
    ChargeInvariantViolation,
    DelegationPoolPartialGovernanceVoting,
    GasPayerEnabled,
    LumioUniqueIdentifiers,
    BulletproofsNatives,
    SignerNativeFormatFix,
    ModuleEvent,
    EmitFeeStatement,
    StorageDeletionRefund,
    AggregatorV2Api,
    SignatureCheckerV2ScriptFix,
    SaferResourceGroups,
    SaferMetadata,
    SingleSenderAuthenticator,
    SponsoredAutomaticAccountCreation,
    FeePayerAccountOptional,
    AggregatorV2DelayedFields,
    ConcurrentTokenV2,
    LimitMaxIdentifierLength,
    OperatorBeneficiaryChange,
    VMBinaryFormatV7,
    ResourceGroupsSplitInVmChangeSet,
    CommissionChangeDelegationPool,
    Bn254Structures,
    WebAuthnSignature,
    ReconfigureWithDkg,
    KeylessAccounts,
    KeylessButZklessAccounts,
    RemoveDetailedError,
    JwkConsensus,
    ConcurrentFungibleAssets,
    RefundableBytes,
    ObjectCodeDeployment,
    MaxObjectNestingCheck,
    KeylessAccountsWithPasskeys,
    MultisigV2Enhancement,
    DelegationPoolAllowlisting,
    ModuleEventMigration,
    RejectUnstableBytecode,
    TransactionContextExtension,
    CoinToFungibleAssetMigration,
    PrimaryAPTFungibleStoreAtUserAddress,
    ObjectNativeDerivedAddress,
    DispatchableFungibleAsset,
    NewAccountsDefaultToFaAptStore,
    OperationsDefaultToFaAptStore,
    AggregatorV2IsAtLeastApi,
    ConcurrentFungibleBalance,
    DefaultToConcurrentFungibleBalance,
    LimitVMTypeSize,
    AbortIfMultisigPayloadMismatch,
    DisallowUserNative,
    AllowSerializedScriptArgs,
    UseCompatibilityCheckerV2,
    EnableEnumTypes,
    EnableResourceAccessControl,
    RejectUnstableBytecodeForScript,
    FederatedKeyless,
    TransactionSimulationEnhancement,
    CollectionOwner,
    NativeMemoryOperations,
    EnableLoaderV2,
    DisallowInitModuleToPublishModules,
    EnableCallTreeAndInstructionVMCache,
    PermissionedSigner,
    AccountAbstraction,
    VMBinaryFormatV8,
    BulletproofsBatchNatives,
    DerivableAccountAbstraction,
    EnableFunctionValues,
    NewAccountsDefaultToFaStore,
    DefaultAccountResource,
    JwkConsensusPerKeyMode,
    TransactionPayloadV2,
    OrderlessTransactions,
    EnableLazyLoading,
    CalculateTransactionFeeForDistribution,
    DistributeTransactionFee,
    MonotonicallyIncreasingCounter,
    EnableCaptureOption,
}

fn generate_features_blob(writer: &CodeWriter, data: &[u64]) {
    emitln!(writer, "vector[");
    writer.indent();
    for (i, b) in data.iter().enumerate() {
        if i % 20 == 0 {
            if i > 0 {
                emitln!(writer);
            }
        } else {
            emit!(writer, " ");
        }
        emit!(writer, "{},", b);
    }
    emitln!(writer);
    writer.unindent();
    emit!(writer, "]")
}

pub fn generate_feature_upgrade_proposal(
    features: &Features,
    is_testnet: bool,
    next_execution_hash: Option<HashValue>,
    is_multi_step: bool,
) -> Result<Vec<(String, String)>> {
    let signer_arg = get_signer_arg(is_testnet, &next_execution_hash);
    let mut result = vec![];

    let enabled = features
        .enabled
        .iter()
        .map(|f| LumioFeatureFlag::from(f.clone()) as u64)
        .collect::<Vec<_>>();
    let disabled = features
        .disabled
        .iter()
        .map(|f| LumioFeatureFlag::from(f.clone()) as u64)
        .collect::<Vec<_>>();

    assert!(enabled.len() < u16::MAX as usize);
    assert!(disabled.len() < u16::MAX as usize);

    let writer = CodeWriter::new(Loc::default());

    emitln!(writer, "// Modifying on-chain feature flags: ");
    emitln!(writer, "// Enabled Features: {:?}", features.enabled);
    emitln!(writer, "// Disabled Features: {:?}", features.disabled);
    emitln!(writer, "//");

    let proposal = generate_governance_proposal(
        &writer,
        is_testnet,
        next_execution_hash,
        is_multi_step,
        &["std::features"],
        |writer| {
            emit!(writer, "let enabled_blob: vector<u64> = ");
            generate_features_blob(writer, &enabled);
            emitln!(writer, ";\n");

            emit!(writer, "let disabled_blob: vector<u64> = ");
            generate_features_blob(writer, &disabled);
            emitln!(writer, ";\n");

            emitln!(
                writer,
                "features::change_feature_flags_for_next_epoch({}, enabled_blob, disabled_blob);",
                signer_arg
            );
            emitln!(writer, "lumio_governance::reconfigure({});", signer_arg);
        },
    );

    result.push(("features".to_string(), proposal));
    Ok(result)
}

impl From<FeatureFlag> for LumioFeatureFlag {
    fn from(f: FeatureFlag) -> Self {
        match f {
            FeatureFlag::CodeDependencyCheck => LumioFeatureFlag::CODE_DEPENDENCY_CHECK,
            FeatureFlag::CollectAndDistributeGasFees => {
                LumioFeatureFlag::_DEPRECATED_COLLECT_AND_DISTRIBUTE_GAS_FEES
            },
            FeatureFlag::TreatFriendAsPrivate => LumioFeatureFlag::TREAT_FRIEND_AS_PRIVATE,
            FeatureFlag::Sha512AndRipeMd160Natives => {
                LumioFeatureFlag::SHA_512_AND_RIPEMD_160_NATIVES
            },
            FeatureFlag::LumioStdChainIdNatives => LumioFeatureFlag::LUMIO_STD_CHAIN_ID_NATIVES,
            FeatureFlag::VMBinaryFormatV6 => LumioFeatureFlag::VM_BINARY_FORMAT_V6,
            FeatureFlag::VMBinaryFormatV7 => LumioFeatureFlag::VM_BINARY_FORMAT_V7,
            FeatureFlag::VMBinaryFormatV8 => LumioFeatureFlag::VM_BINARY_FORMAT_V8,
            FeatureFlag::MultiEd25519PkValidateV2Natives => {
                LumioFeatureFlag::MULTI_ED25519_PK_VALIDATE_V2_NATIVES
            },
            FeatureFlag::Blake2b256Native => LumioFeatureFlag::BLAKE2B_256_NATIVE,
            FeatureFlag::ResourceGroups => LumioFeatureFlag::RESOURCE_GROUPS,
            FeatureFlag::MultisigAccounts => LumioFeatureFlag::MULTISIG_ACCOUNTS,
            FeatureFlag::DelegationPools => LumioFeatureFlag::DELEGATION_POOLS,
            FeatureFlag::CryptographyAlgebraNatives => {
                LumioFeatureFlag::CRYPTOGRAPHY_ALGEBRA_NATIVES
            },
            FeatureFlag::Bls12381Structures => LumioFeatureFlag::BLS12_381_STRUCTURES,
            FeatureFlag::Ed25519PubkeyValidateReturnFalseWrongLength => {
                LumioFeatureFlag::ED25519_PUBKEY_VALIDATE_RETURN_FALSE_WRONG_LENGTH
            },
            FeatureFlag::StructConstructors => LumioFeatureFlag::STRUCT_CONSTRUCTORS,
            FeatureFlag::PeriodicalRewardRateReduction => {
                LumioFeatureFlag::PERIODICAL_REWARD_RATE_DECREASE
            },
            FeatureFlag::PartialGovernanceVoting => LumioFeatureFlag::PARTIAL_GOVERNANCE_VOTING,
            FeatureFlag::SignatureCheckerV2 => LumioFeatureFlag::SIGNATURE_CHECKER_V2,
            FeatureFlag::StorageSlotMetadata => LumioFeatureFlag::STORAGE_SLOT_METADATA,
            FeatureFlag::ChargeInvariantViolation => LumioFeatureFlag::CHARGE_INVARIANT_VIOLATION,
            FeatureFlag::DelegationPoolPartialGovernanceVoting => {
                LumioFeatureFlag::DELEGATION_POOL_PARTIAL_GOVERNANCE_VOTING
            },
            FeatureFlag::GasPayerEnabled => LumioFeatureFlag::GAS_PAYER_ENABLED,
            FeatureFlag::LumioUniqueIdentifiers => LumioFeatureFlag::LUMIO_UNIQUE_IDENTIFIERS,
            FeatureFlag::BulletproofsNatives => LumioFeatureFlag::BULLETPROOFS_NATIVES,
            FeatureFlag::SignerNativeFormatFix => LumioFeatureFlag::SIGNER_NATIVE_FORMAT_FIX,
            FeatureFlag::ModuleEvent => LumioFeatureFlag::MODULE_EVENT,
            FeatureFlag::EmitFeeStatement => LumioFeatureFlag::EMIT_FEE_STATEMENT,
            FeatureFlag::StorageDeletionRefund => LumioFeatureFlag::STORAGE_DELETION_REFUND,
            FeatureFlag::AggregatorV2Api => LumioFeatureFlag::AGGREGATOR_V2_API,
            FeatureFlag::SignatureCheckerV2ScriptFix => {
                LumioFeatureFlag::SIGNATURE_CHECKER_V2_SCRIPT_FIX
            },
            FeatureFlag::SaferResourceGroups => LumioFeatureFlag::SAFER_RESOURCE_GROUPS,
            FeatureFlag::SaferMetadata => LumioFeatureFlag::SAFER_METADATA,
            FeatureFlag::SingleSenderAuthenticator => LumioFeatureFlag::SINGLE_SENDER_AUTHENTICATOR,
            FeatureFlag::SponsoredAutomaticAccountCreation => {
                LumioFeatureFlag::SPONSORED_AUTOMATIC_ACCOUNT_V1_CREATION
            },
            FeatureFlag::FeePayerAccountOptional => LumioFeatureFlag::FEE_PAYER_ACCOUNT_OPTIONAL,
            FeatureFlag::AggregatorV2DelayedFields => {
                LumioFeatureFlag::AGGREGATOR_V2_DELAYED_FIELDS
            },
            FeatureFlag::ConcurrentTokenV2 => LumioFeatureFlag::CONCURRENT_TOKEN_V2,
            FeatureFlag::LimitMaxIdentifierLength => LumioFeatureFlag::LIMIT_MAX_IDENTIFIER_LENGTH,
            FeatureFlag::OperatorBeneficiaryChange => LumioFeatureFlag::OPERATOR_BENEFICIARY_CHANGE,
            FeatureFlag::ResourceGroupsSplitInVmChangeSet => {
                LumioFeatureFlag::RESOURCE_GROUPS_SPLIT_IN_VM_CHANGE_SET
            },
            FeatureFlag::CommissionChangeDelegationPool => {
                LumioFeatureFlag::COMMISSION_CHANGE_DELEGATION_POOL
            },
            FeatureFlag::Bn254Structures => LumioFeatureFlag::BN254_STRUCTURES,
            FeatureFlag::WebAuthnSignature => LumioFeatureFlag::WEBAUTHN_SIGNATURE,
            FeatureFlag::ReconfigureWithDkg => LumioFeatureFlag::_DEPRECATED_RECONFIGURE_WITH_DKG,
            FeatureFlag::KeylessAccounts => LumioFeatureFlag::KEYLESS_ACCOUNTS,
            FeatureFlag::KeylessButZklessAccounts => LumioFeatureFlag::KEYLESS_BUT_ZKLESS_ACCOUNTS,
            FeatureFlag::RemoveDetailedError => {
                LumioFeatureFlag::_DEPRECATED_REMOVE_DETAILED_ERROR_FROM_HASH
            },
            FeatureFlag::JwkConsensus => LumioFeatureFlag::JWK_CONSENSUS,
            FeatureFlag::ConcurrentFungibleAssets => LumioFeatureFlag::CONCURRENT_FUNGIBLE_ASSETS,
            FeatureFlag::RefundableBytes => LumioFeatureFlag::REFUNDABLE_BYTES,
            FeatureFlag::ObjectCodeDeployment => LumioFeatureFlag::OBJECT_CODE_DEPLOYMENT,
            FeatureFlag::MaxObjectNestingCheck => LumioFeatureFlag::MAX_OBJECT_NESTING_CHECK,
            FeatureFlag::KeylessAccountsWithPasskeys => {
                LumioFeatureFlag::KEYLESS_ACCOUNTS_WITH_PASSKEYS
            },
            FeatureFlag::MultisigV2Enhancement => LumioFeatureFlag::MULTISIG_V2_ENHANCEMENT,
            FeatureFlag::DelegationPoolAllowlisting => {
                LumioFeatureFlag::DELEGATION_POOL_ALLOWLISTING
            },
            FeatureFlag::ModuleEventMigration => LumioFeatureFlag::MODULE_EVENT_MIGRATION,
            FeatureFlag::RejectUnstableBytecode => LumioFeatureFlag::_REJECT_UNSTABLE_BYTECODE,
            FeatureFlag::TransactionContextExtension => {
                LumioFeatureFlag::TRANSACTION_CONTEXT_EXTENSION
            },
            FeatureFlag::CoinToFungibleAssetMigration => {
                LumioFeatureFlag::COIN_TO_FUNGIBLE_ASSET_MIGRATION
            },
            FeatureFlag::PrimaryAPTFungibleStoreAtUserAddress => {
                LumioFeatureFlag::PRIMARY_LUM_FUNGIBLE_STORE_AT_USER_ADDRESS
            },
            FeatureFlag::ObjectNativeDerivedAddress => {
                LumioFeatureFlag::OBJECT_NATIVE_DERIVED_ADDRESS
            },
            FeatureFlag::DispatchableFungibleAsset => LumioFeatureFlag::DISPATCHABLE_FUNGIBLE_ASSET,
            FeatureFlag::NewAccountsDefaultToFaAptStore => {
                LumioFeatureFlag::NEW_ACCOUNTS_DEFAULT_TO_FA_LUM_STORE
            },
            FeatureFlag::OperationsDefaultToFaAptStore => {
                LumioFeatureFlag::OPERATIONS_DEFAULT_TO_FA_LUM_STORE
            },
            FeatureFlag::AggregatorV2IsAtLeastApi => {
                LumioFeatureFlag::AGGREGATOR_V2_IS_AT_LEAST_API
            },
            FeatureFlag::ConcurrentFungibleBalance => LumioFeatureFlag::CONCURRENT_FUNGIBLE_BALANCE,
            FeatureFlag::DefaultToConcurrentFungibleBalance => {
                LumioFeatureFlag::DEFAULT_TO_CONCURRENT_FUNGIBLE_BALANCE
            },
            FeatureFlag::LimitVMTypeSize => LumioFeatureFlag::_LIMIT_VM_TYPE_SIZE,
            FeatureFlag::AbortIfMultisigPayloadMismatch => {
                LumioFeatureFlag::ABORT_IF_MULTISIG_PAYLOAD_MISMATCH
            },
            FeatureFlag::DisallowUserNative => LumioFeatureFlag::_DISALLOW_USER_NATIVES,
            FeatureFlag::AllowSerializedScriptArgs => {
                LumioFeatureFlag::ALLOW_SERIALIZED_SCRIPT_ARGS
            },
            FeatureFlag::UseCompatibilityCheckerV2 => {
                LumioFeatureFlag::_USE_COMPATIBILITY_CHECKER_V2
            },
            FeatureFlag::EnableEnumTypes => LumioFeatureFlag::ENABLE_ENUM_TYPES,
            FeatureFlag::EnableResourceAccessControl => {
                LumioFeatureFlag::ENABLE_RESOURCE_ACCESS_CONTROL
            },
            FeatureFlag::RejectUnstableBytecodeForScript => {
                LumioFeatureFlag::_REJECT_UNSTABLE_BYTECODE_FOR_SCRIPT
            },
            FeatureFlag::FederatedKeyless => LumioFeatureFlag::FEDERATED_KEYLESS,
            FeatureFlag::TransactionSimulationEnhancement => {
                LumioFeatureFlag::TRANSACTION_SIMULATION_ENHANCEMENT
            },
            FeatureFlag::CollectionOwner => LumioFeatureFlag::COLLECTION_OWNER,
            FeatureFlag::NativeMemoryOperations => LumioFeatureFlag::NATIVE_MEMORY_OPERATIONS,
            FeatureFlag::EnableLoaderV2 => LumioFeatureFlag::_ENABLE_LOADER_V2,
            FeatureFlag::DisallowInitModuleToPublishModules => {
                LumioFeatureFlag::_DISALLOW_INIT_MODULE_TO_PUBLISH_MODULES
            },
            FeatureFlag::EnableCallTreeAndInstructionVMCache => {
                LumioFeatureFlag::ENABLE_CALL_TREE_AND_INSTRUCTION_VM_CACHE
            },
            FeatureFlag::PermissionedSigner => LumioFeatureFlag::PERMISSIONED_SIGNER,
            FeatureFlag::AccountAbstraction => LumioFeatureFlag::ACCOUNT_ABSTRACTION,
            FeatureFlag::BulletproofsBatchNatives => LumioFeatureFlag::BULLETPROOFS_BATCH_NATIVES,
            FeatureFlag::DerivableAccountAbstraction => {
                LumioFeatureFlag::DERIVABLE_ACCOUNT_ABSTRACTION
            },
            FeatureFlag::EnableFunctionValues => LumioFeatureFlag::ENABLE_FUNCTION_VALUES,
            FeatureFlag::NewAccountsDefaultToFaStore => {
                LumioFeatureFlag::NEW_ACCOUNTS_DEFAULT_TO_FA_STORE
            },
            FeatureFlag::DefaultAccountResource => LumioFeatureFlag::DEFAULT_ACCOUNT_RESOURCE,
            FeatureFlag::JwkConsensusPerKeyMode => LumioFeatureFlag::JWK_CONSENSUS_PER_KEY_MODE,
            FeatureFlag::TransactionPayloadV2 => LumioFeatureFlag::TRANSACTION_PAYLOAD_V2,
            FeatureFlag::OrderlessTransactions => LumioFeatureFlag::ORDERLESS_TRANSACTIONS,
            FeatureFlag::EnableLazyLoading => LumioFeatureFlag::ENABLE_LAZY_LOADING,
            FeatureFlag::CalculateTransactionFeeForDistribution => {
                LumioFeatureFlag::CALCULATE_TRANSACTION_FEE_FOR_DISTRIBUTION
            },
            FeatureFlag::DistributeTransactionFee => LumioFeatureFlag::DISTRIBUTE_TRANSACTION_FEE,
            FeatureFlag::MonotonicallyIncreasingCounter => {
                LumioFeatureFlag::MONOTONICALLY_INCREASING_COUNTER
            },
            FeatureFlag::EnableCaptureOption => LumioFeatureFlag::ENABLE_CAPTURE_OPTION,
        }
    }
}

// We don't need this implementation. Just to make sure we have an exhaustive 1-1 mapping between the two structs.
impl From<LumioFeatureFlag> for FeatureFlag {
    fn from(f: LumioFeatureFlag) -> Self {
        match f {
            LumioFeatureFlag::CODE_DEPENDENCY_CHECK => FeatureFlag::CodeDependencyCheck,
            LumioFeatureFlag::_DEPRECATED_COLLECT_AND_DISTRIBUTE_GAS_FEES => {
                FeatureFlag::CollectAndDistributeGasFees
            },
            LumioFeatureFlag::TREAT_FRIEND_AS_PRIVATE => FeatureFlag::TreatFriendAsPrivate,
            LumioFeatureFlag::SHA_512_AND_RIPEMD_160_NATIVES => {
                FeatureFlag::Sha512AndRipeMd160Natives
            },
            LumioFeatureFlag::LUMIO_STD_CHAIN_ID_NATIVES => FeatureFlag::LumioStdChainIdNatives,
            LumioFeatureFlag::VM_BINARY_FORMAT_V6 => FeatureFlag::VMBinaryFormatV6,
            LumioFeatureFlag::VM_BINARY_FORMAT_V7 => FeatureFlag::VMBinaryFormatV7,
            LumioFeatureFlag::VM_BINARY_FORMAT_V8 => FeatureFlag::VMBinaryFormatV8,
            LumioFeatureFlag::MULTI_ED25519_PK_VALIDATE_V2_NATIVES => {
                FeatureFlag::MultiEd25519PkValidateV2Natives
            },
            LumioFeatureFlag::BLAKE2B_256_NATIVE => FeatureFlag::Blake2b256Native,
            LumioFeatureFlag::RESOURCE_GROUPS => FeatureFlag::ResourceGroups,
            LumioFeatureFlag::MULTISIG_ACCOUNTS => FeatureFlag::MultisigAccounts,
            LumioFeatureFlag::DELEGATION_POOLS => FeatureFlag::DelegationPools,
            LumioFeatureFlag::CRYPTOGRAPHY_ALGEBRA_NATIVES => {
                FeatureFlag::CryptographyAlgebraNatives
            },
            LumioFeatureFlag::BLS12_381_STRUCTURES => FeatureFlag::Bls12381Structures,
            LumioFeatureFlag::ED25519_PUBKEY_VALIDATE_RETURN_FALSE_WRONG_LENGTH => {
                FeatureFlag::Ed25519PubkeyValidateReturnFalseWrongLength
            },
            LumioFeatureFlag::STRUCT_CONSTRUCTORS => FeatureFlag::StructConstructors,
            LumioFeatureFlag::PERIODICAL_REWARD_RATE_DECREASE => {
                FeatureFlag::PeriodicalRewardRateReduction
            },
            LumioFeatureFlag::PARTIAL_GOVERNANCE_VOTING => FeatureFlag::PartialGovernanceVoting,
            LumioFeatureFlag::SIGNATURE_CHECKER_V2 => FeatureFlag::SignatureCheckerV2,
            LumioFeatureFlag::STORAGE_SLOT_METADATA => FeatureFlag::StorageSlotMetadata,
            LumioFeatureFlag::CHARGE_INVARIANT_VIOLATION => FeatureFlag::ChargeInvariantViolation,
            LumioFeatureFlag::DELEGATION_POOL_PARTIAL_GOVERNANCE_VOTING => {
                FeatureFlag::DelegationPoolPartialGovernanceVoting
            },
            LumioFeatureFlag::GAS_PAYER_ENABLED => FeatureFlag::GasPayerEnabled,
            LumioFeatureFlag::LUMIO_UNIQUE_IDENTIFIERS => FeatureFlag::LumioUniqueIdentifiers,
            LumioFeatureFlag::BULLETPROOFS_NATIVES => FeatureFlag::BulletproofsNatives,
            LumioFeatureFlag::SIGNER_NATIVE_FORMAT_FIX => FeatureFlag::SignerNativeFormatFix,
            LumioFeatureFlag::MODULE_EVENT => FeatureFlag::ModuleEvent,
            LumioFeatureFlag::EMIT_FEE_STATEMENT => FeatureFlag::EmitFeeStatement,
            LumioFeatureFlag::STORAGE_DELETION_REFUND => FeatureFlag::StorageDeletionRefund,
            LumioFeatureFlag::AGGREGATOR_V2_API => FeatureFlag::AggregatorV2Api,
            LumioFeatureFlag::SIGNATURE_CHECKER_V2_SCRIPT_FIX => {
                FeatureFlag::SignatureCheckerV2ScriptFix
            },
            LumioFeatureFlag::SAFER_RESOURCE_GROUPS => FeatureFlag::SaferResourceGroups,
            LumioFeatureFlag::SAFER_METADATA => FeatureFlag::SaferMetadata,
            LumioFeatureFlag::SINGLE_SENDER_AUTHENTICATOR => FeatureFlag::SingleSenderAuthenticator,
            LumioFeatureFlag::SPONSORED_AUTOMATIC_ACCOUNT_V1_CREATION => {
                FeatureFlag::SponsoredAutomaticAccountCreation
            },
            LumioFeatureFlag::FEE_PAYER_ACCOUNT_OPTIONAL => FeatureFlag::FeePayerAccountOptional,
            LumioFeatureFlag::AGGREGATOR_V2_DELAYED_FIELDS => {
                FeatureFlag::AggregatorV2DelayedFields
            },
            LumioFeatureFlag::CONCURRENT_TOKEN_V2 => FeatureFlag::ConcurrentTokenV2,
            LumioFeatureFlag::LIMIT_MAX_IDENTIFIER_LENGTH => FeatureFlag::LimitMaxIdentifierLength,
            LumioFeatureFlag::OPERATOR_BENEFICIARY_CHANGE => FeatureFlag::OperatorBeneficiaryChange,
            LumioFeatureFlag::RESOURCE_GROUPS_SPLIT_IN_VM_CHANGE_SET => {
                FeatureFlag::ResourceGroupsSplitInVmChangeSet
            },
            LumioFeatureFlag::COMMISSION_CHANGE_DELEGATION_POOL => {
                FeatureFlag::CommissionChangeDelegationPool
            },
            LumioFeatureFlag::BN254_STRUCTURES => FeatureFlag::Bn254Structures,
            LumioFeatureFlag::WEBAUTHN_SIGNATURE => FeatureFlag::WebAuthnSignature,
            LumioFeatureFlag::_DEPRECATED_RECONFIGURE_WITH_DKG => FeatureFlag::ReconfigureWithDkg,
            LumioFeatureFlag::KEYLESS_ACCOUNTS => FeatureFlag::KeylessAccounts,
            LumioFeatureFlag::KEYLESS_BUT_ZKLESS_ACCOUNTS => FeatureFlag::KeylessButZklessAccounts,
            LumioFeatureFlag::_DEPRECATED_REMOVE_DETAILED_ERROR_FROM_HASH => {
                FeatureFlag::RemoveDetailedError
            },
            LumioFeatureFlag::JWK_CONSENSUS => FeatureFlag::JwkConsensus,
            LumioFeatureFlag::CONCURRENT_FUNGIBLE_ASSETS => FeatureFlag::ConcurrentFungibleAssets,
            LumioFeatureFlag::REFUNDABLE_BYTES => FeatureFlag::RefundableBytes,
            LumioFeatureFlag::OBJECT_CODE_DEPLOYMENT => FeatureFlag::ObjectCodeDeployment,
            LumioFeatureFlag::MAX_OBJECT_NESTING_CHECK => FeatureFlag::MaxObjectNestingCheck,
            LumioFeatureFlag::KEYLESS_ACCOUNTS_WITH_PASSKEYS => {
                FeatureFlag::KeylessAccountsWithPasskeys
            },
            LumioFeatureFlag::MULTISIG_V2_ENHANCEMENT => FeatureFlag::MultisigV2Enhancement,
            LumioFeatureFlag::DELEGATION_POOL_ALLOWLISTING => {
                FeatureFlag::DelegationPoolAllowlisting
            },
            LumioFeatureFlag::MODULE_EVENT_MIGRATION => FeatureFlag::ModuleEventMigration,
            LumioFeatureFlag::_REJECT_UNSTABLE_BYTECODE => FeatureFlag::RejectUnstableBytecode,
            LumioFeatureFlag::TRANSACTION_CONTEXT_EXTENSION => {
                FeatureFlag::TransactionContextExtension
            },
            LumioFeatureFlag::COIN_TO_FUNGIBLE_ASSET_MIGRATION => {
                FeatureFlag::CoinToFungibleAssetMigration
            },
            LumioFeatureFlag::PRIMARY_LUM_FUNGIBLE_STORE_AT_USER_ADDRESS => {
                FeatureFlag::PrimaryAPTFungibleStoreAtUserAddress
            },
            LumioFeatureFlag::OBJECT_NATIVE_DERIVED_ADDRESS => {
                FeatureFlag::ObjectNativeDerivedAddress
            },
            LumioFeatureFlag::DISPATCHABLE_FUNGIBLE_ASSET => FeatureFlag::DispatchableFungibleAsset,
            LumioFeatureFlag::NEW_ACCOUNTS_DEFAULT_TO_FA_LUM_STORE => {
                FeatureFlag::NewAccountsDefaultToFaAptStore
            },
            LumioFeatureFlag::OPERATIONS_DEFAULT_TO_FA_LUM_STORE => {
                FeatureFlag::OperationsDefaultToFaAptStore
            },
            LumioFeatureFlag::AGGREGATOR_V2_IS_AT_LEAST_API => {
                FeatureFlag::AggregatorV2IsAtLeastApi
            },
            LumioFeatureFlag::CONCURRENT_FUNGIBLE_BALANCE => FeatureFlag::ConcurrentFungibleBalance,
            LumioFeatureFlag::DEFAULT_TO_CONCURRENT_FUNGIBLE_BALANCE => {
                FeatureFlag::DefaultToConcurrentFungibleBalance
            },
            LumioFeatureFlag::_LIMIT_VM_TYPE_SIZE => FeatureFlag::LimitVMTypeSize,
            LumioFeatureFlag::ABORT_IF_MULTISIG_PAYLOAD_MISMATCH => {
                FeatureFlag::AbortIfMultisigPayloadMismatch
            },
            LumioFeatureFlag::_DISALLOW_USER_NATIVES => FeatureFlag::DisallowUserNative,
            LumioFeatureFlag::ALLOW_SERIALIZED_SCRIPT_ARGS => {
                FeatureFlag::AllowSerializedScriptArgs
            },
            LumioFeatureFlag::_USE_COMPATIBILITY_CHECKER_V2 => {
                FeatureFlag::UseCompatibilityCheckerV2
            },
            LumioFeatureFlag::ENABLE_ENUM_TYPES => FeatureFlag::EnableEnumTypes,
            LumioFeatureFlag::ENABLE_RESOURCE_ACCESS_CONTROL => {
                FeatureFlag::EnableResourceAccessControl
            },
            LumioFeatureFlag::_REJECT_UNSTABLE_BYTECODE_FOR_SCRIPT => {
                FeatureFlag::RejectUnstableBytecodeForScript
            },
            LumioFeatureFlag::FEDERATED_KEYLESS => FeatureFlag::FederatedKeyless,
            LumioFeatureFlag::TRANSACTION_SIMULATION_ENHANCEMENT => {
                FeatureFlag::TransactionSimulationEnhancement
            },
            LumioFeatureFlag::COLLECTION_OWNER => FeatureFlag::CollectionOwner,
            LumioFeatureFlag::NATIVE_MEMORY_OPERATIONS => FeatureFlag::NativeMemoryOperations,
            LumioFeatureFlag::_ENABLE_LOADER_V2 => FeatureFlag::EnableLoaderV2,
            LumioFeatureFlag::_DISALLOW_INIT_MODULE_TO_PUBLISH_MODULES => {
                FeatureFlag::DisallowInitModuleToPublishModules
            },
            LumioFeatureFlag::ENABLE_CALL_TREE_AND_INSTRUCTION_VM_CACHE => {
                FeatureFlag::EnableCallTreeAndInstructionVMCache
            },
            LumioFeatureFlag::PERMISSIONED_SIGNER => FeatureFlag::PermissionedSigner,
            LumioFeatureFlag::ACCOUNT_ABSTRACTION => FeatureFlag::AccountAbstraction,
            LumioFeatureFlag::BULLETPROOFS_BATCH_NATIVES => FeatureFlag::BulletproofsBatchNatives,
            LumioFeatureFlag::DERIVABLE_ACCOUNT_ABSTRACTION => {
                FeatureFlag::DerivableAccountAbstraction
            },
            LumioFeatureFlag::ENABLE_FUNCTION_VALUES => FeatureFlag::EnableFunctionValues,
            LumioFeatureFlag::NEW_ACCOUNTS_DEFAULT_TO_FA_STORE => {
                FeatureFlag::NewAccountsDefaultToFaStore
            },
            LumioFeatureFlag::DEFAULT_ACCOUNT_RESOURCE => FeatureFlag::DefaultAccountResource,
            LumioFeatureFlag::JWK_CONSENSUS_PER_KEY_MODE => FeatureFlag::JwkConsensusPerKeyMode,
            LumioFeatureFlag::TRANSACTION_PAYLOAD_V2 => FeatureFlag::TransactionPayloadV2,
            LumioFeatureFlag::ORDERLESS_TRANSACTIONS => FeatureFlag::OrderlessTransactions,
            LumioFeatureFlag::ENABLE_LAZY_LOADING => FeatureFlag::EnableLazyLoading,
            LumioFeatureFlag::CALCULATE_TRANSACTION_FEE_FOR_DISTRIBUTION => {
                FeatureFlag::CalculateTransactionFeeForDistribution
            },
            LumioFeatureFlag::DISTRIBUTE_TRANSACTION_FEE => FeatureFlag::DistributeTransactionFee,
            LumioFeatureFlag::MONOTONICALLY_INCREASING_COUNTER => {
                FeatureFlag::MonotonicallyIncreasingCounter
            },
            LumioFeatureFlag::ENABLE_CAPTURE_OPTION => FeatureFlag::EnableCaptureOption,
        }
    }
}

impl Features {
    // Compare if the current feature set is different from features that has been enabled on chain.
    pub(crate) fn has_modified(&self, on_chain_features: &LumioFeatures) -> bool {
        self.enabled
            .iter()
            .any(|f| !on_chain_features.is_enabled(LumioFeatureFlag::from(f.clone())))
            || self
                .disabled
                .iter()
                .any(|f| on_chain_features.is_enabled(LumioFeatureFlag::from(f.clone())))
    }
}

impl From<&LumioFeatures> for Features {
    fn from(features: &LumioFeatures) -> Features {
        let mut enabled = vec![];
        let mut disabled = vec![];
        for feature in FeatureFlag::iter() {
            if features.is_enabled(LumioFeatureFlag::from(feature.clone())) {
                enabled.push(feature);
            } else {
                disabled.push(feature);
            }
        }
        Features { enabled, disabled }
    }
}
