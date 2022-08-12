extern crate core;

use api::RangoApi;

pub mod api;
mod response;
mod transactions;
pub mod types;

#[derive(Clone)]
pub struct Client {
    pub api: RangoApi,
}

impl Client {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            api: RangoApi::with_default_url(api_key),
        }
    }
}
