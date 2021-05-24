use std::fmt;
use variant_count::VariantCount;

impl fmt::Display for CommandMetric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<usize> for CommandMetric {
    fn from(i: usize) -> Self {
        let conversion = num_traits::FromPrimitive::from_usize(i);
        if conversion.is_some() {
            conversion.unwrap()
        } else {
            panic!("Unable to convert from {}, unknown error code", i)
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, FromPrimitive, ToPrimitive, VariantCount)]
#[repr(usize)]
pub enum CommandMetric {
    // IssuerCommand
    IssuerCommandCreateSchema,
    IssuerCommandCreateAndStoreCredentialDefinition,
    IssuerCommandRotateCredentialDefinitionStart,
    IssuerCommandRotateCredentialDefinitionStartComplete,
    IssuerCommandRotateCredentialDefinitionApply,
    IssuerCommandCreateAndStoreRevocationRegistry,
    IssuerCommandCreateCredentialOffer,
    IssuerCommandCreateCredential,
    IssuerCommandRevokeCredential,
    IssuerCommandMergeRevocationRegistryDeltas,
    // ProverCommand
    ProverCommandCreateMasterSecret,
    ProverCommandCreateCredentialRequest,
    ProverCommandSetCredentialAttrTagPolicy,
    ProverCommandGetCredentialAttrTagPolicy,
    ProverCommandStoreCredential,
    ProverCommandGetCredentials,
    ProverCommandGetCredential,
    ProverCommandDeleteCredential,
    ProverCommandSearchCredentials,
    ProverCommandFetchCredentials,
    ProverCommandCloseCredentialsSearch,
    ProverCommandGetCredentialsForProofReq,
    ProverCommandSearchCredentialsForProofReq,
    ProverCommandFetchCredentialForProofReq,
    ProverCommandCloseCredentialsSearchForProofReq,
    ProverCommandCreateProof,
    ProverCommandCreateRevocationState,
    ProverCommandUpdateRevocationState,
    // VerifierCommand
    VerifierCommandVerifyProof,
    VerifierCommandGenerateNonce,
    // AnoncredsCommand
    AnoncredsCommandToUnqualified,
    // BlobStorage
    BlobStorageCommandOpenReader,
    BlobStorageCommandOpenWriter,
    // CryptoCommand
    CryptoCommandCreateKey,
    CryptoCommandSetKeyMetadata,
    CryptoCommandGetKeyMetadata,
    CryptoCommandCryptoSign,
    CryptoCommandCryptoVerify,
    CryptoCommandAuthenticatedEncrypt,
    CryptoCommandAuthenticatedDecrypt,
    CryptoCommandAnonymousEncrypt,
    CryptoCommandAnonymousDecrypt,
    CryptoCommandPackMessage,
    CryptoCommandUnpackMessage,
    LedgerCommandSignAndSubmitRequest,
    // LedgerCommand
    LedgerCommandSubmitRequest,
    LedgerCommandSubmitAck,
    LedgerCommandSubmitAction,
    LedgerCommandSignRequest,
    LedgerCommandMultiSignRequest,
    LedgerCommandBuildGetDdoRequest,
    LedgerCommandBuildNymRequest,
    LedgerCommandBuildAttribRequest,
    LedgerCommandBuildGetAttribRequest,
    LedgerCommandBuildGetNymRequest,
    LedgerCommandParseGetNymResponse,
    LedgerCommandBuildSchemaRequest,
    LedgerCommandBuildGetSchemaRequest,
    LedgerCommandParseGetSchemaResponse,
    LedgerCommandBuildCredDefRequest,
    LedgerCommandBuildGetCredDefRequest,
    LedgerCommandParseGetCredDefResponse,
    LedgerCommandBuildNodeRequest,
    LedgerCommandBuildGetValidatorInfoRequest,
    LedgerCommandBuildGetTxnRequest,
    LedgerCommandBuildPoolConfigRequest,
    LedgerCommandBuildPoolRestartRequest,
    LedgerCommandBuildPoolUpgradeRequest,
    LedgerCommandBuildRevocRegDefRequest,
    LedgerCommandBuildGetRevocRegDefRequest,
    LedgerCommandParseGetRevocRegDefResponse,
    LedgerCommandBuildRevocRegEntryRequest,
    LedgerCommandBuildGetRevocRegRequest,
    LedgerCommandParseGetRevocRegResponse,
    LedgerCommandBuildGetRevocRegDeltaRequest,
    LedgerCommandParseGetRevocRegDeltaResponse,
    LedgerCommandRegisterSPParser,
    LedgerCommandGetResponseMetadata,
    LedgerCommandBuildAuthRuleRequest,
    LedgerCommandBuildAuthRulesRequest,
    LedgerCommandBuildGetAuthRuleRequest,
    LedgerCommandGetSchema,
    LedgerCommandGetCredDef,
    LedgerCommandBuildTxnAuthorAgreementRequest,
    LedgerCommandBuildDisableAllTxnAuthorAgreementsRequest,
    LedgerCommandBuildGetTxnAuthorAgreementRequest,
    LedgerCommandBuildAcceptanceMechanismRequests,
    LedgerCommandBuildGetAcceptanceMechanismsRequest,
    LedgerCommandAppendTxnAuthorAgreementAcceptanceToRequest,
    LedgerCommandAppendRequestEndorser,
    // VerimLedger
    VerimLedgerCommandBuildMsgCreateNym,
    VerimLedgerCommandBuildMsgUpdateNym,
    VerimLedgerCommandBuildMsgDeleteNym,
    VerimLedgerCommandBuildQueryGetNym,
    VerimLedgerCommandParseMsgCreateNymResp,
    VerimLedgerCommandParseMsgUpdateNymResp,
    VerimLedgerCommandParseMsgDeleteNymResp,
    VerimLedgerCommandParseQueryGetNymResp,
    // CosmosPool
    CosmosPoolAdd,
    CosmosPoolGetConfig,
    CosmosPoolBuildTx,
    CosmosPoolBroadcastTxCommit,
    // PoolCommand
    PoolCommandCreate,
    PoolCommandDelete,
    PoolCommandOpen,
    PoolCommandOpenAck,
    PoolCommandList,
    PoolCommandClose,
    PoolCommandCloseAck,
    PoolCommandRefresh,
    PoolCommandRefreshAck,
    PoolCommandSetProtocolVersion,
    // DidCommand
    DidCommandCreateAndStoreMyDid,
    DidCommandReplaceKeysStart,
    DidCommandReplaceKeysApply,
    DidCommandStoreTheirDid,
    DidCommandGetMyDidWithMeta,
    DidCommandListMyDidsWithMeta,
    DidCommandKeyForDid,
    DidCommandKeyForLocalDid,
    DidCommandSetEndpointForDid,
    DidCommandGetEndpointForDid,
    DidCommandSetDidMetadata,
    DidCommandGetDidMetadata,
    DidCommandAbbreviateVerkey,
    DidCommandGetNymAck,
    DidCommandGetAttribAck,
    DidCommandQualifyDid,
    // CosmosKeys
    CosmosKeysAddRandom,
    CosmosKeysAddFromMnemonic,
    CosmosKeysKeyInfo,
    CosmosKeysSign,
    // WalletCommand
    WalletCommandRegisterWalletType,
    WalletCommandCreate,
    WalletCommandOpen,
    WalletCommandClose,
    WalletCommandDelete,
    WalletCommandExport,
    WalletCommandImport,
    WalletCommandGenerateKey,
    WalletCommandDeriveKey,
    // PairwiseCommand
    PairwiseCommandPairwiseExists,
    PairwiseCommandCreatePairwise,
    PairwiseCommandListPairwise,
    PairwiseCommandGetPairwise,
    PairwiseCommandSetPairwiseMetadata,
    // NonSecretsCommand
    NonSecretsCommandAddRecord,
    NonSecretsCommandUpdateRecordValue,
    NonSecretsCommandUpdateRecordTags,
    NonSecretsCommandAddRecordTags,
    NonSecretsCommandDeleteRecordTags,
    NonSecretsCommandDeleteRecord,
    NonSecretsCommandGetRecord,
    NonSecretsCommandOpenSearch,
    NonSecretsCommandFetchSearchNextRecords,
    NonSecretsCommandCloseSearch,
    // PaymentsCommand
    PaymentsCommandRegisterMethod,
    PaymentsCommandCreateAddress,
    PaymentsCommandCreateAddressAck,
    PaymentsCommandListAddresses,
    PaymentsCommandAddRequestFees,
    PaymentsCommandAddRequestFeesAck,
    PaymentsCommandParseResponseWithFees,
    PaymentsCommandParseResponseWithFeesAck,
    PaymentsCommandBuildGetPaymentSourcesRequest,
    PaymentsCommandBuildGetPaymentSourcesRequestAck,
    PaymentsCommandParseGetPaymentSourcesResponse,
    PaymentsCommandParseGetPaymentSourcesResponseAck,
    PaymentsCommandBuildPaymentReq,
    PaymentsCommandBuildPaymentReqAck,
    PaymentsCommandParsePaymentResponse,
    PaymentsCommandParsePaymentResponseAck,
    PaymentsCommandAppendTxnAuthorAgreementAcceptanceToExtra,
    PaymentsCommandBuildMintReq,
    PaymentsCommandBuildMintReqAck,
    PaymentsCommandBuildSetTxnFeesReq,
    PaymentsCommandBuildSetTxnFeesReqAck,
    PaymentsCommandBuildGetTxnFeesReq,
    PaymentsCommandBuildGetTxnFeesReqAck,
    PaymentsCommandParseGetTxnFeesResponse,
    PaymentsCommandParseGetTxnFeesResponseAck,
    PaymentsCommandBuildVerifyPaymentReq,
    PaymentsCommandBuildVerifyPaymentReqAck,
    PaymentsCommandParseVerifyPaymentResponse,
    PaymentsCommandParseVerifyPaymentResponseAck,
    PaymentsCommandGetRequestInfo,
    PaymentsCommandSignWithAddressReq,
    PaymentsCommandSignWithAddressAck,
    PaymentsCommandVerifyWithAddressReq,
    PaymentsCommandVerifyWithAddressAck,
    // CacheCommand
    CacheCommandGetSchema,
    CacheCommandGetCredDef,
    CacheCommandPurgeSchemaCache,
    CacheCommandPurgeCredDefCache,
    // MetricsCommand
    MetricsCommandCollectMetrics,
    // Exit
    Exit,
}
