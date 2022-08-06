use std::collections::HashMap;

use ethers::types::Address;
use serde::{Deserialize, Serialize};

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
