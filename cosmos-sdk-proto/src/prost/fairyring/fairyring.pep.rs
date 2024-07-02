// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregatedKeyShare {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub creator: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptedTx {
    #[prost(uint64, tag = "1")]
    pub target_height: u64,
    #[prost(uint64, tag = "2")]
    pub index: u64,
    #[prost(string, tag = "3")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub creator: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub charged_gas: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "6")]
    pub processed_at_chain_height: u64,
    #[prost(bool, tag = "7")]
    pub expired: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptedTxArray {
    #[prost(message, repeated, tag = "1")]
    pub encrypted_tx: ::prost::alloc::vec::Vec<EncryptedTx>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneralEncryptedTx {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub index: u64,
    #[prost(string, tag = "3")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub creator: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub charged_gas: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneralEncryptedTxArray {
    #[prost(message, repeated, tag = "1")]
    pub encrypted_tx: ::prost::alloc::vec::Vec<GeneralEncryptedTx>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenEncTxExecutionQueue {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub tx_list: ::core::option::Option<GeneralEncryptedTxArray>,
    #[prost(string, tag = "6")]
    pub aggr_keyshare: ::prost::alloc::string::String,
}
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(message, repeated, tag = "1")]
    pub trusted_counter_parties: ::prost::alloc::vec::Vec<TrustedCounterParty>,
    #[prost(string, repeated, tag = "2")]
    pub trusted_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// string pep_channel_id = 3;
    #[prost(string, tag = "4")]
    pub keyshare_channel_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub min_gas_price: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(bool, tag = "6")]
    pub is_source_chain: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrustedCounterParty {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub channel_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PepNonce {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub nonce: u64,
}
/// GenesisState defines the pep module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(string, tag = "2")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub encrypted_tx_array: ::prost::alloc::vec::Vec<EncryptedTxArray>,
    #[prost(message, repeated, tag = "4")]
    pub pep_nonce_list: ::prost::alloc::vec::Vec<PepNonce>,
    /// this line is used by starport scaffolding # genesis/proto/state
    #[prost(message, repeated, tag = "6")]
    pub aggregated_key_share_list: ::prost::alloc::vec::Vec<AggregatedKeyShare>,
    #[prost(message, optional, tag = "7")]
    pub active_pub_key: ::core::option::Option<super::common::ActivePublicKey>,
    #[prost(message, optional, tag = "8")]
    pub queued_pub_key: ::core::option::Option<super::common::QueuedPublicKey>,
    #[prost(uint64, tag = "9")]
    pub request_count: u64,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryKeyshareRequest {
    #[prost(string, tag = "1")]
    pub req_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryKeyshareResponse {
    #[prost(message, optional, tag = "1")]
    pub keyshare: ::core::option::Option<GenEncTxExecutionQueue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllKeyshareRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllKeyshareResponse {
    #[prost(message, repeated, tag = "1")]
    pub keyshares: ::prost::alloc::vec::Vec<GenEncTxExecutionQueue>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetEncryptedTxRequest {
    #[prost(uint64, tag = "1")]
    pub target_height: u64,
    #[prost(uint64, tag = "2")]
    pub index: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetEncryptedTxResponse {
    #[prost(message, optional, tag = "1")]
    pub encrypted_tx: ::core::option::Option<EncryptedTx>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllEncryptedTxRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllEncryptedTxResponse {
    #[prost(message, repeated, tag = "1")]
    pub encrypted_tx_array: ::prost::alloc::vec::Vec<EncryptedTxArray>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllEncryptedTxFromHeightRequest {
    #[prost(uint64, tag = "1")]
    pub target_height: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllEncryptedTxFromHeightResponse {
    #[prost(message, optional, tag = "1")]
    pub encrypted_tx_array: ::core::option::Option<EncryptedTxArray>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLatestHeightRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLatestHeightResponse {
    #[prost(uint64, tag = "1")]
    pub height: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetPepNonceRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetPepNonceResponse {
    #[prost(message, optional, tag = "1")]
    pub pep_nonce: ::core::option::Option<PepNonce>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllPepNonceRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllPepNonceResponse {
    #[prost(message, repeated, tag = "1")]
    pub pep_nonce: ::prost::alloc::vec::Vec<PepNonce>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPubKeyRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPubKeyResponse {
    #[prost(message, optional, tag = "1")]
    pub active_pub_key: ::core::option::Option<super::common::ActivePublicKey>,
    #[prost(message, optional, tag = "2")]
    pub queued_pub_key: ::core::option::Option<super::common::QueuedPublicKey>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitEncryptedTx {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub target_block_height: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitGeneralEncryptedTx {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub req_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitEncryptedTxResponse {}
/// this line is used by starport scaffolding # proto/tx/message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateAggregatedKeyShare {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub height: u64,
    #[prost(string, tag = "3")]
    pub data: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateAggregatedKeyShareResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestGeneralKeyshare {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestGeneralKeyshareResponse {
    #[prost(string, tag = "1")]
    pub req_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGetGeneralKeyshare {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub req_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGetGeneralKeyshareResponse {}
include!("fairyring.pep.tonic.rs");
// @@protoc_insertion_point(module)
