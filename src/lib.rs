use api::RangoApi;

pub mod api;
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
