use std::collections::HashMap;

use anyhow::{Context, Result};
use reqwest::{Client, ClientBuilder, IntoUrl, Url};

use crate::types::{CheckApproval, FailureReportParameters, Meta, WalletDetails};

#[derive(Clone, Debug)]
pub struct RangoApi {
    client: Client,
    api_key: Option<String>,
    base_url: Url,
}

impl RangoApi {
    pub const DEFAULT_URL: &'static str = "https://api.rango.exchange/";

    pub fn new(base_url: impl IntoUrl, api_key: Option<String>) -> Result<Self> {
        let builder = ClientBuilder::new();
        let client = builder.build().unwrap();

        Ok(Self {
            client,
            api_key,
            base_url: base_url.into_url().context("rango exchange api url")?,
        })
    }

    pub fn with_default_url(api_key: Option<String>) -> Self {
        Self::new(Self::DEFAULT_URL, api_key).unwrap()
    }

    pub fn form_authenticated_url(&self, path: &str) -> Url {
        let mut url = self
            .base_url
            .join(path)
            .expect("unexpectedly invalid URL segment");

        if let Some(api_key) = &self.api_key {
            url.query_pairs_mut().extend_pairs(&[("apiKey", api_key)]);
        }

        url
    }

    /// Returns the approval status of a specific request for a transaction.
    pub async fn get_approval_status(
        &self,
        request_id: &str,
        transaction_id: &str,
    ) -> Result<CheckApproval> {
        let is_approved_url = self.form_authenticated_url("/basic/is-approved");

        let resp = self
            .client
            .get(is_approved_url)
            .query(&[("requestId", request_id), ("txId", transaction_id)])
            .send()
            .await?
            .json::<CheckApproval>()
            .await?;

        Ok(resp)
    }

    pub async fn get_meta(&self) -> Result<Meta> {
        let meta_url = self.form_authenticated_url("/basic/meta");

        let resp = self
            .client
            .get(meta_url)
            .send()
            .await?
            .json::<Meta>()
            .await?;

        Ok(resp)
    }

    pub async fn report_failure(
        &self,
        request_id: &str,
        details: &HashMap<String, String>,
    ) -> Result<()> {
        let report_url = self.form_authenticated_url("/basic/report-tx");

        let b = FailureReportParameters {
            request_id: request_id.to_string(),
            event_type: "TX_FAIL".to_string(),
            data: details.clone(),
        };

        self.client.post(report_url).json(&b).send().await?;

        Ok(())
    }

    pub async fn get_balances(&self, blockchain: &str, address: &str) -> Result<WalletDetails> {
        let balances_url = self.form_authenticated_url("/basic/balance");

        let resp = self
            .client
            .get(balances_url)
            .query(&[("blockchain", blockchain), ("address", address)])
            .send()
            .await?
            .json::<WalletDetails>()
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn can_get_approval_status() {
        let api_key = env::var("TEST_RANGO_API_KEY").unwrap();
        let api = RangoApi::with_default_url(Some(api_key));

        let should_be_valid_req = api
            .get_approval_status(
                "967efbd7-797e-429b-a587-ac973d8c8bea",
                "0x5c6aed428e9c6b76bde1e776120ac9a976289173161f28d37b2c0150c7ff9319",
            )
            .await
            .unwrap();

        assert!(should_be_valid_req.is_approved);

        let should_be_invalid_req = api
            .get_approval_status(
                "7c9d2c35-f0da-475a-aa78-1109562de4f4",
                "0x9ab55b8ff6ce92381d2c509140a2ae36079a5331e22248cd6974ff008de3d00a",
            )
            .await
            .unwrap();

        assert!(!should_be_invalid_req.is_approved);
    }
}
