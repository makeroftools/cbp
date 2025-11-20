use binance_sdk::spot::{
    rest_api::TickerParams,
    SpotRestApi,
};
use binance_sdk::
use async_graphql::{
    Context,
    Object
};

pub struct Ticker;
pub struct Params;

#[derive(Default)]
pub struct BinanceQuery;

#[Object]
impl BinanceQuery {
    async fn time(
        &self, 
        ctx: &Context<'_>
    ) -> async_graphql::Result<String> {
        let client = ctx.data::<SpotRestApi>()?;
        let params = TickerParams::default();
        let response = client.time().await.context("time request failed");
        Ok(response)
    }
}

