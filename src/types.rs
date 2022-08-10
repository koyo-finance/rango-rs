use std::collections::HashMap;

use ethers::types::Address;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionType {
    Evm,
    Transfer,
    Cosmos,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckApproval {
    pub is_approved: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FailureReportParameters {
    pub request_id: String,
    pub event_type: String,
    pub data: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Asset {
    pub blockchain: String,
    pub address: Option<String>,
    pub symbol: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Token {
    pub blockchain: String,
    pub address: Option<String>,
    pub symbol: String,
    pub decimals: u64,
    pub image: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Amount {
    pub amount: String,
    pub decimals: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetAndAmount {
    pub amount: Amount,
    pub asset: Asset,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletDetail {
    pub failed: bool,
    pub block_chain: String,
    pub address: Address,
    pub balances: Option<Vec<AssetAndAmount>>,
    pub explorer_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletDetails {
    pub wallets: Vec<WalletDetail>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainMeta {
    pub name: String,
    pub default_decimals: u64,
    pub fee_assets: Vec<Asset>,
    pub address_patterns: Vec<String>,
    #[serde(alias = "type")]
    pub transaction_types: Option<TransactionType>,
    pub chain_id: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SwapperMeta {
    pub id: String,
    pub title: String,
    pub logo: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Meta {
    pub blockchains: Vec<WalletDetail>,
    pub tokens: Vec<Token>,
    pub swappers: Vec<SwapperMeta>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResultType {
    Ok,
    HighImpact,
    InputLimitIssue,
    NoRoute,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SwapperType {
    Bridge,
    Dex,
    Composer,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotePath {
    from: Token,
    to: Token,
    swapper: SwapperMeta,
    swapper_type: SwapperType,
    expected_output: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExpenseType {
    FromSourceWallet,
    DecreaseFromOutput,
    FromDestinationWallet,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapFee {
    name: String,
    token: Token,
    expense_type: ExpenseType,
    amount: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RangeType {
    Inclusive,
    Exclusive,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmountRestriction {
    min: Option<String>,
    max: Option<String>,
    #[serde(alias = "type")]
    range_type: RangeType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteSimulationResult {
    output_amount: String,
    swapper: SwapperMeta,
    path: Option<Vec<QuotePath>>,
    fee: Vec<SwapFee>,
    amount_restriction: Option<AmountRestriction>,
    estimated_time_in_seconds: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapResponse {
    pub request_id: String,
    pub result_type: ResultType,
    pub route: Qption<QuoteSimulationResult>,
    pub error: Option<String>,
    //pub tx:  TODO: add type based on the type; implement correct deserializer
}
