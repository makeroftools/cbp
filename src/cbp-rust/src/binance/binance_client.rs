// src/binance_client.rs
use binance_sdk::config::ConfigurationRestApi;
use std::sync::Arc;

#[derive(Clone)]
pub struct BinanceClient {
    pub config: ConfigurationRestApi,
}

impl BinanceClient {
    pub fn new(config: ConfigurationRestApi) -> Arc<Self> {
        Arc::new(Self { config })
    }

    pub fn config(&self) -> &ConfigurationRestApi {
        &self.config
    }
}