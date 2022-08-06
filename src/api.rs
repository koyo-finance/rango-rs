use anyhow::{Context, Result};
use reqwest::{Client, ClientBuilder, IntoUrl, Url};

use crate::types::CheckApproval;

#[derive(Clone, Debug)]
pub struct RangoApi {
    client: Client,
    base_url: Url,
}

impl RangoApi {
    pub const DEFAULT_URL: &'static str = "https://api.rango.exchange/";

    pub fn new(base_url: impl IntoUrl, api_key: Option<String>) -> Result<Self> {
        let builder = ClientBuilder::new();
        let client = builder.build().unwrap();
        let mut base_url = base_url.into_url().context("rango exchange api url")?;

        if let Some(api_key) = api_key {
            base_url
                .query_pairs_mut()
                .extend_pairs(&[("apiKey", api_key)]);
        }

        Ok(Self { client, base_url })
    }

    pub fn with_default_url(api_key: Option<String>) -> Self {
        Self::new(Self::DEFAULT_URL, api_key).unwrap()
    }

    pub async fn get_approval_status(
        &self,
        request_id: &str,
        transaction_id: &str,
    ) -> Result<CheckApproval> {
        let is_approved_url = self
            .base_url
            .clone()
            .join("/basic/is-approved")
            .expect("unexpectedly invalid URL segment");

        let res = self
            .client
            .get(is_approved_url)
            .query(&[("requestId", request_id), ("txId", transaction_id)])
            .send()
            .await?;
        let text = res.text().await?;
        let resp: CheckApproval = serde_json::from_str(&text)?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn can_get_approval_status() {
        let api = RangoApi::with_default_url(None);

        let should_be_valid_req = api.get_approval_status("", "").await.unwrap();
        assert!(should_be_valid_req.is_approved);

        let should_not_be_valid_req = api.get_approval_status("", "").await.unwrap();
        assert!(!should_not_be_valid_req.is_approved);
    }
}
