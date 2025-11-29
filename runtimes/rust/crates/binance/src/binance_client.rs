// src/binance/binance_client.rs
use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::spot::rest_api::RestApi as SpotRestApi;
use std::sync::Arc;

#[derive(Clone)]
pub struct BinanceClient {
    pub spot: Arc<SpotRestApi>,
}

impl BinanceClient {
    pub fn new(config: ConfigurationRestApi) -> Self {
        let spot = SpotRestApi::new(config);
        Self { spot: Arc::new(spot) }
    }
}