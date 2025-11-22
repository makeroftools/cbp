use async_graphql::{Context, Object};
use binance_sdk::spot::rest_api::RestApi;
use binance_sdk::spot::rest_api::ExchangeInfoParams;
use std::sync::Arc;
use serde_json;
use tokio::sync::Mutex;

#[derive(Default)]
pub struct BinanceQuery;

#[Object]
impl BinanceQuery {
    async fn time(&self, ctx: &Context<'_>) -> async_graphql::Result<String> {
        let client = ctx.data::<Arc<Mutex<RestApi>>>()?.clone();
        // let resp = (&*client.lock().await).time().await?;
        let resp = (&client.lock().await).time().await?;
        let data = resp.data().await?;
        let stime = data.server_time.unwrap().to_string();
        Ok(stime)
    }
    
    async fn ping(&self, ctx: &Context<'_>) -> async_graphql::Result<String> {
        let client = ctx.data::<Arc<Mutex<RestApi>>>()?.clone();
        let resp = (&client.lock().await).ping().await?;
        let data = resp.data().await?;
        let ack = data.to_string();
        Ok(ack)
    }
    
    // async fn exchange_info(&self, ctx: &Context<'_>) -> async_graphql::Result<ExchangeInfoResponse> {
    //     let client = ctx.data::<Arc<Mutex<RestApi>>>()?.clone();
    //     let params = ExchangeInfoParams::default();
    //     let resp = (&client.lock().await).exchange_info(params).await?;
    //     let data = resp.data().await?;
    //     Ok(data)
    // }
    async fn exchange_info(&self, ctx: &Context<'_>) -> async_graphql::Result<String> {
        let client = ctx.data::<Arc<Mutex<RestApi>>>()?.clone();
        let resp = client.lock().await.exchange_info(ExchangeInfoParams::default()).await?;
        let data = resp.data().await?;
        Ok(serde_json::to_string_pretty(&data)?)
    }
}

