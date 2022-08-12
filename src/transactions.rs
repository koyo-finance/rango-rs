use crate::types::{AssetWithTicker, TransactionType};
use serde::{Deserialize, Serialize};

//
// EVM
//

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvmTransaction {
    #[serde(alias = "type")]
    pub(crate) tx_type: TransactionType,
    pub(crate) block_chain: String,
    pub(crate) from: Option<String>,
    pub(crate) approve_to: Option<String>,
    pub(crate) approve_data: Option<String>,
    pub(crate) tx_to: String,
    pub(crate) tx_data: Option<String>,
    pub(crate) value: Option<String>,
    pub(crate) gas_limit: Option<String>,
    pub(crate) gas_price: Option<String>,
}

//
// Cosmos
//

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosTransaction {
    #[serde(alias = "type")]
    tx_type: TransactionType,
    block_chain: String,
    from_wallet_address: String,
    data: CosmosMessage,
    raw_transfer: CosmosRawTransferData,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosRawTransferData {
    amount: String,
    asset: AssetWithTicker,
    decimals: u32,
    memo: Option<String>,
    method: String,
    recipient: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SignType {
    Amino,
    Direct,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosMessage {
    sign_type: SignType,
    sequence: Option<String>,
    source: Option<u32>,
    account_number: Option<u32>,
    rpc_url: Option<String>,
    chain_id: Option<String>,
    //msgs: any[] TODO: investigate any ??
    //protoMsgs: any[]
    memo: Option<String>,
    fee: Option<CosmosStdFee>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosCoin {
    amount: String,
    denom: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosStdFee {
    amount: Vec<CosmosCoin>,
    gas: String,
}

//
// Transfer
//
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    #[serde(alias = "type")]
    tx_type: TransactionType,
    method: String,
    asset: AssetWithTicker,
    amount: String,
    decimals: u32,
    from_wallet_address: String,
    recipient_address: String,
    memo: Option<String>,
}
