// src/binance/graphql/mutation.rs
use async_graphql::{Context, FieldResult, Object};
use std::sync::Arc;
use crate::binance_client::BinanceClient;

#[derive(Default)]
pub struct Mutation;

#[Object]
impl Mutation {
    #[graphql(name = "newOrder")]
    async fn new_order(
        &self,
        ctx: &Context<'_>,
        symbol: String,
        side: String,
        r#type: String,
        quantity: f64,
        price: Option<f64>,
        time_in_force: Option<String>,
        test: Option<bool>,
    ) -> FieldResult<serde_json::Value> {
        let client = ctx.data_unchecked::<Arc<BinanceClient>>();

        let result: () = if test.unwrap_or(true) {
            // Stub order
            serde_json::json!({ "orderId": 123, "symbol": symbol, "side": side })
        } else {
            // Real order stub
            serde_json::json!({ "orderId": 456, "symbol": symbol, "side": side })
        };

        Ok(serde_json::json!(result))
    }
}