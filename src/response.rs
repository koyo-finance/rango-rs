use crate::transactions::{CosmosTransaction, EvmTransaction, Transfer};
use crate::types::{QuoteSimulationResult, ResultType};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TxType {
    Evm(EvmTransaction),
    Cosmos(CosmosTransaction),
    Transfer(Transfer),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapResponse {
    pub request_id: String,
    pub result_type: ResultType,
    pub route: Option<QuoteSimulationResult>,
    pub error: Option<String>,
    pub tx: TxType,
}

impl<'de> Deserialize<'de> for SwapResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json: serde_json::value::Value = serde_json::value::Value::deserialize(deserializer)?;

        let tx_value = json.get("tx").expect("tx present");
        let tx_type = tx_value.get("type").expect("type present");

        let tx = match tx_type.as_str().unwrap() {
            "EVM" => {
                let tx: EvmTransaction =
                    serde_json::from_str(tx_value.to_string().as_str()).unwrap();
                TxType::Evm(tx)
            }
            "COSMOS" => {
                let tx: CosmosTransaction =
                    serde_json::from_str(tx_value.to_string().as_str()).unwrap();
                TxType::Cosmos(tx)
            }
            "TRANSFER" => {
                let tx: Transfer = serde_json::from_str(tx_value.to_string().as_str()).unwrap();
                TxType::Transfer(tx)
            }
            x => {
                panic!("Unsupported tx type {}", x);
            }
        };

        //TODO: get the rest of the SwapResponse - try to find a way not to do all fields manually but
        //rather use deserialize in some way to partially deser the fields

        Ok(SwapResponse {
            request_id: "".to_string(),
            result_type: ResultType::Ok,
            route: None,
            error: None,
            tx,
        })
    }
}

#[cfg(test)]
use super::types::TransactionType;

#[test]
fn swap_response_deser_should_work_when_receives_evm_transaction() {
    let input = r#"
    { "request_id": "id",
      "resultType": "Ok",
      "tx": { "type": "EVM", "blockChain": "blockChain", "txTo" : "tx_to" }
    }
    "#;

    let data: Result<SwapResponse, _> = serde_json::from_str(input); //.unwrap();

    assert!(data.is_ok());

    assert_eq!(
        data.unwrap(),
        SwapResponse {
            request_id: "".to_string(),
            result_type: ResultType::Ok,
            route: None,
            error: None,
            tx: TxType::Evm(EvmTransaction {
                tx_type: TransactionType::Evm,
                block_chain: "blockChain".to_string(),
                from: None,
                approve_to: None,
                approve_data: None,
                tx_to: "tx_to".to_string(),
                tx_data: None,
                value: None,
                gas_limit: None,
                gas_price: None,
            },),
        }
    );
}
