use osmosis_std_derive::CosmwasmExt;
/// ===================== MsgJoinPool
/// This is really MsgJoinPoolNoSwap
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgJoinPool")]
pub struct MsgJoinPool {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub share_out_amount: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub token_in_maxs: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgJoinPoolResponse")]
pub struct MsgJoinPoolResponse {
    #[prost(string, tag = "1")]
    pub share_out_amount: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub token_in: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// ===================== MsgExitPool
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgExitPool")]
pub struct MsgExitPool {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub share_in_amount: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub token_out_mins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgExitPoolResponse")]
pub struct MsgExitPoolResponse {
    #[prost(message, repeated, tag = "1")]
    pub token_out: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// ===================== MsgSwapExactAmountIn
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.SwapAmountInRoute")]
pub struct SwapAmountInRoute {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub token_out_denom: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgSwapExactAmountIn")]
pub struct MsgSwapExactAmountIn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
    #[prost(message, optional, tag = "3")]
    pub token_in: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "4")]
    pub token_out_min_amount: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgSwapExactAmountInResponse")]
pub struct MsgSwapExactAmountInResponse {
    #[prost(string, tag = "1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// ===================== MsgSwapExactAmountOut
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.SwapAmountOutRoute")]
pub struct SwapAmountOutRoute {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub token_in_denom: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgSwapExactAmountOut")]
pub struct MsgSwapExactAmountOut {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
    #[prost(string, tag = "3")]
    pub token_in_max_amount: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub token_out: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgSwapExactAmountOutResponse")]
pub struct MsgSwapExactAmountOutResponse {
    #[prost(string, tag = "1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
/// ===================== MsgJoinSwapExternAmountIn
/// TODO: Rename to MsgJoinSwapExactAmountIn
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgJoinSwapExternAmountIn")]
pub struct MsgJoinSwapExternAmountIn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(message, optional, tag = "3")]
    pub token_in: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// repeated cosmos.base.v1beta1.Coin tokensIn = 5 [
    ///   (gogoproto.moretags) = "yaml:\"tokens_in\"",
    ///   (gogoproto.nullable) = false
    /// ];
    #[prost(string, tag = "4")]
    pub share_out_min_amount: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgJoinSwapExternAmountInResponse")]
pub struct MsgJoinSwapExternAmountInResponse {
    #[prost(string, tag = "1")]
    pub share_out_amount: ::prost::alloc::string::String,
}
/// ===================== MsgJoinSwapShareAmountOut
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgJoinSwapShareAmountOut")]
pub struct MsgJoinSwapShareAmountOut {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub token_in_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub share_out_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub token_in_max_amount: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgJoinSwapShareAmountOutResponse")]
pub struct MsgJoinSwapShareAmountOutResponse {
    #[prost(string, tag = "1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
/// ===================== MsgExitSwapShareAmountIn
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgExitSwapShareAmountIn")]
pub struct MsgExitSwapShareAmountIn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub token_out_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub share_in_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub token_out_min_amount: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgExitSwapShareAmountInResponse")]
pub struct MsgExitSwapShareAmountInResponse {
    #[prost(string, tag = "1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// ===================== MsgExitSwapExternAmountOut
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgExitSwapExternAmountOut")]
pub struct MsgExitSwapExternAmountOut {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(message, optional, tag = "3")]
    pub token_out: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "4")]
    pub share_in_max_amount: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgExitSwapExternAmountOutResponse")]
pub struct MsgExitSwapExternAmountOutResponse {
    #[prost(string, tag = "1")]
    pub share_in_amount: ::prost::alloc::string::String,
}
/// Params holds parameters for the incentives module
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.Params")]
pub struct Params {
    #[prost(message, repeated, tag = "1")]
    pub pool_creation_fee:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the gamm module's genesis state.
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.GenesisState")]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<crate::shim::Any>,
    /// will be renamed to next_pool_id in an upcoming version
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub next_pool_number: u64,
    #[prost(message, optional, tag = "3")]
    pub params: ::core::option::Option<Params>,
}
///=============================== Pool
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryPoolRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/Pool",
    response_type = QueryPoolResponse
)]
pub struct QueryPoolRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryPoolResponse")]
pub struct QueryPoolResponse {
    #[prost(message, optional, tag = "1")]
    pub pool: ::core::option::Option<crate::shim::Any>,
}
///=============================== Pools
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryPoolsRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/Pools",
    response_type = QueryPoolsResponse
)]
pub struct QueryPoolsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryPoolsResponse")]
pub struct QueryPoolsResponse {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<crate::shim::Any>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
///=============================== NumPools
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryNumPoolsRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/NumPools",
    response_type = QueryNumPoolsResponse
)]
pub struct QueryNumPoolsRequest {}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryNumPoolsResponse")]
pub struct QueryNumPoolsResponse {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub num_pools: u64,
}
///=============================== PoolType
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryPoolTypeRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/PoolType",
    response_type = QueryPoolTypeResponse
)]
pub struct QueryPoolTypeRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryPoolTypeResponse")]
pub struct QueryPoolTypeResponse {
    #[prost(string, tag = "1")]
    pub pool_type: ::prost::alloc::string::String,
}
///=============================== CalcJoinPoolShares
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryCalcJoinPoolSharesRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/CalcJoinPoolShares",
    response_type = QueryCalcJoinPoolSharesResponse
)]
pub struct QueryCalcJoinPoolSharesRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(message, repeated, tag = "2")]
    pub tokens_in: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryCalcJoinPoolSharesResponse")]
pub struct QueryCalcJoinPoolSharesResponse {
    #[prost(string, tag = "1")]
    pub share_out_amount: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub tokens_out: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
///=============================== CalcExitPoolCoinsFromShares
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryCalcExitPoolCoinsFromSharesRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/CalcExitPoolCoinsFromShares",
    response_type = QueryCalcExitPoolCoinsFromSharesResponse
)]
pub struct QueryCalcExitPoolCoinsFromSharesRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub share_in_amount: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryCalcExitPoolCoinsFromSharesResponse")]
pub struct QueryCalcExitPoolCoinsFromSharesResponse {
    #[prost(message, repeated, tag = "1")]
    pub tokens_out: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
///=============================== PoolParams
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryPoolParamsRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/PoolParams",
    response_type = QueryPoolParamsResponse
)]
pub struct QueryPoolParamsRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryPoolParamsResponse")]
pub struct QueryPoolParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<crate::shim::Any>,
}
///=============================== PoolLiquidity
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryTotalPoolLiquidityRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/TotalPoolLiquidity",
    response_type = QueryTotalPoolLiquidityResponse
)]
pub struct QueryTotalPoolLiquidityRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryTotalPoolLiquidityResponse")]
pub struct QueryTotalPoolLiquidityResponse {
    #[prost(message, repeated, tag = "1")]
    pub liquidity: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
///=============================== TotalShares
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryTotalSharesRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/TotalShares",
    response_type = QueryTotalSharesResponse
)]
pub struct QueryTotalSharesRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryTotalSharesResponse")]
pub struct QueryTotalSharesResponse {
    #[prost(message, optional, tag = "1")]
    pub total_shares: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
///=============================== CalcJoinPoolNoSwapShares
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryCalcJoinPoolNoSwapSharesRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/CalcJoinPoolNoSwapShares",
    response_type = QueryCalcJoinPoolNoSwapSharesResponse
)]
pub struct QueryCalcJoinPoolNoSwapSharesRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(message, repeated, tag = "2")]
    pub tokens_in: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryCalcJoinPoolNoSwapSharesResponse")]
pub struct QueryCalcJoinPoolNoSwapSharesResponse {
    #[prost(message, repeated, tag = "1")]
    pub tokens_out: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "2")]
    pub shares_out: ::prost::alloc::string::String,
}
/// QuerySpotPriceRequest defines the gRPC request structure for a SpotPrice
/// query.
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QuerySpotPriceRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/SpotPrice",
    response_type = QuerySpotPriceResponse
)]
#[deprecated]
pub struct QuerySpotPriceRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub base_asset_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_asset_denom: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryPoolsWithFilterRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/PoolsWithFilter",
    response_type = QueryPoolsWithFilterResponse
)]
pub struct QueryPoolsWithFilterRequest {
    /// String of the coins in single string seperated by comma. Ex)
    /// 10uatom,100uosmo
    #[prost(string, tag = "1")]
    pub min_liquidity: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pool_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryPoolsWithFilterResponse")]
pub struct QueryPoolsWithFilterResponse {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<crate::shim::Any>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QuerySpotPriceResponse defines the gRPC response structure for a SpotPrice
/// query.
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QuerySpotPriceResponse")]
#[deprecated]
pub struct QuerySpotPriceResponse {
    /// String of the Dec. Ex) 10.203uatom
    #[prost(string, tag = "1")]
    pub spot_price: ::prost::alloc::string::String,
}
///=============================== EstimateSwapExactAmountIn
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QuerySwapExactAmountInRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/EstimateSwapExactAmountIn",
    response_type = QuerySwapExactAmountInResponse
)]
pub struct QuerySwapExactAmountInRequest {
    /// TODO: CHANGE THIS TO RESERVED IN A PATCH RELEASE
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QuerySwapExactAmountInResponse")]
pub struct QuerySwapExactAmountInResponse {
    #[prost(string, tag = "1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
///=============================== EstimateSwapExactAmountOut
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QuerySwapExactAmountOutRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/EstimateSwapExactAmountOut",
    response_type = QuerySwapExactAmountOutResponse
)]
pub struct QuerySwapExactAmountOutRequest {
    /// TODO: CHANGE THIS TO RESERVED IN A PATCH RELEASE
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(message, repeated, tag = "3")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
    #[prost(string, tag = "4")]
    pub token_out: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QuerySwapExactAmountOutResponse")]
pub struct QuerySwapExactAmountOutResponse {
    #[prost(string, tag = "1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryTotalLiquidityRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/TotalLiquidity",
    response_type = QueryTotalLiquidityResponse
)]
pub struct QueryTotalLiquidityRequest {}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryTotalLiquidityResponse")]
pub struct QueryTotalLiquidityResponse {
    #[prost(message, repeated, tag = "1")]
    pub liquidity: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// Parameters for changing the weights in a balancer pool smoothly from
/// a start weight and end weight over a period of time.
/// Currently, the only smooth change supported is linear changing between
/// the two weights, but more types may be added in the future.
/// When these parameters are set, the weight w(t) for pool time `t` is the
/// following:
///   t <= start_time: w(t) = initial_pool_weights
///   start_time < t <= start_time + duration:
///     w(t) = initial_pool_weights + (t - start_time) *
///       (target_pool_weights - initial_pool_weights) / (duration)
///   t > start_time + duration: w(t) = target_pool_weights
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.SmoothWeightChangeParams")]
pub struct SmoothWeightChangeParams {
    /// The start time for beginning the weight change.
    /// If a parameter change / pool instantiation leaves this blank,
    /// it should be generated by the state_machine as the current time.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<crate::shim::Timestamp>,
    /// Duration for the weights to change over
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<crate::shim::Duration>,
    /// The initial pool weights. These are copied from the pool's settings
    /// at the time of weight change instantiation.
    /// The amount PoolAsset.token.amount field is ignored if present,
    /// future type refactorings should just have a type with the denom & weight
    /// here.
    #[prost(message, repeated, tag = "3")]
    pub initial_pool_weights: ::prost::alloc::vec::Vec<PoolAsset>,
    /// The target pool weights. The pool weights will change linearly with respect
    /// to time between start_time, and start_time + duration. The amount
    /// PoolAsset.token.amount field is ignored if present, future type
    /// refactorings should just have a type with the denom & weight here.
    ///
    /// Intermediate variable for the 'slope' of pool weights. This is equal to
    /// (target_pool_weights - initial_pool_weights) / (duration)
    /// TODO: Work out precision, and decide if this is good to add
    /// repeated PoolAsset poolWeightSlope = 5 [
    ///  (gogoproto.moretags) = "yaml:\"pool_weight_slope\"",
    ///  (gogoproto.nullable) = false
    /// ];
    #[prost(message, repeated, tag = "4")]
    pub target_pool_weights: ::prost::alloc::vec::Vec<PoolAsset>,
}
/// PoolParams defined the parameters that will be managed by the pool
/// governance in the future. This params are not managed by the chain
/// governance. Instead they will be managed by the token holders of the pool.
/// The pool's token holders are specified in future_pool_governor.
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.PoolParams")]
pub struct PoolParams {
    #[prost(string, tag = "1")]
    pub swap_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub exit_fee: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub smooth_weight_change_params: ::core::option::Option<SmoothWeightChangeParams>,
}
/// Pool asset is an internal struct that combines the amount of the
/// token in the pool, and its balancer weight.
/// This is an awkward packaging of data,
/// and should be revisited in a future state migration.
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.PoolAsset")]
pub struct PoolAsset {
    /// Coins we are talking about,
    /// the denomination must be unique amongst all PoolAssets for this pool.
    #[prost(message, optional, tag = "1")]
    pub token: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// Weight that is not normalized. This weight must be less than 2^50
    #[prost(string, tag = "2")]
    pub weight: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.Pool")]
pub struct Pool {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    #[prost(message, optional, tag = "3")]
    pub pool_params: ::core::option::Option<PoolParams>,
    /// This string specifies who will govern the pool in the future.
    /// Valid forms of this are:
    /// {token name},{duration}
    /// {duration}
    /// where {token name} if specified is the token which determines the
    /// governor, and if not specified is the LP token for this pool.duration is
    /// a time specified as 0w,1w,2w, etc. which specifies how long the token
    /// would need to be locked up to count in governance. 0w means no lockup.
    /// TODO: Further improve these docs
    #[prost(string, tag = "4")]
    pub future_pool_governor: ::prost::alloc::string::String,
    /// sum of all LP tokens sent out
    #[prost(message, optional, tag = "5")]
    pub total_shares: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// These are assumed to be sorted by denomiation.
    /// They contain the pool asset and the information about the weight
    #[prost(message, repeated, tag = "6")]
    pub pool_assets: ::prost::alloc::vec::Vec<PoolAsset>,
    /// sum of all non-normalized pool weights
    #[prost(string, tag = "7")]
    pub total_weight: ::prost::alloc::string::String,
}
pub struct GammQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> GammQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn pools(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryPoolsResponse, cosmwasm_std::StdError> {
        QueryPoolsRequest { pagination }.query(self.querier)
    }
    pub fn num_pools(&self) -> Result<QueryNumPoolsResponse, cosmwasm_std::StdError> {
        QueryNumPoolsRequest {}.query(self.querier)
    }
    pub fn total_liquidity(&self) -> Result<QueryTotalLiquidityResponse, cosmwasm_std::StdError> {
        QueryTotalLiquidityRequest {}.query(self.querier)
    }
    pub fn pools_with_filter(
        &self,
        min_liquidity: ::prost::alloc::string::String,
        pool_type: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryPoolsWithFilterResponse, cosmwasm_std::StdError> {
        QueryPoolsWithFilterRequest {
            min_liquidity,
            pool_type,
            pagination,
        }
        .query(self.querier)
    }
    pub fn pool(&self, pool_id: u64) -> Result<QueryPoolResponse, cosmwasm_std::StdError> {
        QueryPoolRequest { pool_id }.query(self.querier)
    }
    pub fn pool_type(&self, pool_id: u64) -> Result<QueryPoolTypeResponse, cosmwasm_std::StdError> {
        QueryPoolTypeRequest { pool_id }.query(self.querier)
    }
    pub fn calc_join_pool_no_swap_shares(
        &self,
        pool_id: u64,
        tokens_in: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    ) -> Result<QueryCalcJoinPoolNoSwapSharesResponse, cosmwasm_std::StdError> {
        QueryCalcJoinPoolNoSwapSharesRequest { pool_id, tokens_in }.query(self.querier)
    }
    pub fn calc_join_pool_shares(
        &self,
        pool_id: u64,
        tokens_in: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    ) -> Result<QueryCalcJoinPoolSharesResponse, cosmwasm_std::StdError> {
        QueryCalcJoinPoolSharesRequest { pool_id, tokens_in }.query(self.querier)
    }
    pub fn calc_exit_pool_coins_from_shares(
        &self,
        pool_id: u64,
        share_in_amount: ::prost::alloc::string::String,
    ) -> Result<QueryCalcExitPoolCoinsFromSharesResponse, cosmwasm_std::StdError> {
        QueryCalcExitPoolCoinsFromSharesRequest {
            pool_id,
            share_in_amount,
        }
        .query(self.querier)
    }
    pub fn pool_params(
        &self,
        pool_id: u64,
    ) -> Result<QueryPoolParamsResponse, cosmwasm_std::StdError> {
        QueryPoolParamsRequest { pool_id }.query(self.querier)
    }
    pub fn total_pool_liquidity(
        &self,
        pool_id: u64,
    ) -> Result<QueryTotalPoolLiquidityResponse, cosmwasm_std::StdError> {
        QueryTotalPoolLiquidityRequest { pool_id }.query(self.querier)
    }
    pub fn total_shares(
        &self,
        pool_id: u64,
    ) -> Result<QueryTotalSharesResponse, cosmwasm_std::StdError> {
        QueryTotalSharesRequest { pool_id }.query(self.querier)
    }
    #[deprecated]
    pub fn spot_price(
        &self,
        pool_id: u64,
        base_asset_denom: ::prost::alloc::string::String,
        quote_asset_denom: ::prost::alloc::string::String,
    ) -> Result<QuerySpotPriceResponse, cosmwasm_std::StdError> {
        QuerySpotPriceRequest {
            pool_id,
            base_asset_denom,
            quote_asset_denom,
        }
        .query(self.querier)
    }
    pub fn estimate_swap_exact_amount_in(
        &self,
        sender: ::prost::alloc::string::String,
        pool_id: u64,
        token_in: ::prost::alloc::string::String,
        routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
    ) -> Result<QuerySwapExactAmountInResponse, cosmwasm_std::StdError> {
        QuerySwapExactAmountInRequest {
            sender,
            pool_id,
            token_in,
            routes,
        }
        .query(self.querier)
    }
    pub fn estimate_swap_exact_amount_out(
        &self,
        sender: ::prost::alloc::string::String,
        pool_id: u64,
        routes: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
        token_out: ::prost::alloc::string::String,
    ) -> Result<QuerySwapExactAmountOutResponse, cosmwasm_std::StdError> {
        QuerySwapExactAmountOutRequest {
            sender,
            pool_id,
            routes,
            token_out,
        }
        .query(self.querier)
    }
}
