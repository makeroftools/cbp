// src/binance/graphql/query.rs
use async_graphql::{Context, FieldResult, Object};
use std::sync::Arc;
use crate::binance_client::BinanceClient;

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    async fn ping(&self, ctx: &Context<'_>) -> FieldResult<bool> {
        let client = ctx.data_unchecked::<Arc<BinanceClient>>();
        Ok(client.spot.ping().await.is_ok())
    }

    async fn server_time(&self, ctx: &Context<'_>) -> FieldResult<i64> {
        let client = ctx.data_unchecked::<Arc<BinanceClient>>();
        let resp = client.spot.server_time().await?;
        Ok(resp.serverTime)
    }

    async fn exchange_info(&self, ctx: &Context<'_>) -> FieldResult<String> {
        let client = ctx.data_unchecked::<Arc<BinanceClient>>();
        let info: () = client.spot.exchange_info().await?;
        Ok("Exchange info stub".to_string())
    }

    async fn ticker_24hr(&self, ctx: &Context<'_>, symbol: String) -> FieldResult<serde_json::Value> {
        let client = ctx.data_unchecked::<Arc<BinanceClient>>();
        let ticker: () = client.spot.ticker_24hr(&symbol).await?;
        Ok(serde_json::json!({ "symbol": symbol, "price": "stub" }))
    }

    async fn account(&self, ctx: &Context<'_>) -> FieldResult<serde_json::Value> {
        let client = ctx.data_unchecked::<Arc<BinanceClient>>();
        let account: () = client.spot.account().await?;
        Ok(serde_json::json!({ "balances": [] }))
    }

    async fn open_orders(
        &self,
        ctx: &Context<'_>,
        symbol: Option<String>,
    ) -> FieldResult<Vec<serde_json::Value>> {
        let client = ctx.data_unchecked::<Arc<BinanceClient>>();
        let orders: () = if let Some(sym) = symbol {
            client.spot.open_orders(Some(&sym)).await?
        } else {
            client.spot.open_orders(None::<&str>).await?
        };
        Ok(vec![])
    }
}