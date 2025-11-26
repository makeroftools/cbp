<<<<<<< HEAD
=======
use async_graphql::*;
use binance_sdk::spot::rest_api::RestApi;
use binance_sdk::spot::rest_api::ExchangeInfoParams;
use binance_sdk::spot::rest_api::ExchangeInfoResponse
use std::sync::Arc;
// use serde_json;
use tokio::sync::Mutex;
 


pub struct BinanceQuery;

#[Object]
impl BinanceQuery {
    async fn time(&self, ctx: &Context<'_>) -> async_graphql::Result<String> {
        let client = ctx.data::<Arc<Mutex<RestApi>>>()?.clone();
        // let resp = (&*client.lock().await).time().await?;
        // let resp = (&client.lock().await).time().await?;
        let resp = client.lock().await.time().await?;
        let data = resp.data().await?;
        let stime = data.server_time.unwrap().to_string();
        Ok(stime)
    }
    
    async fn ping(&self, ctx: &Context<'_>) -> async_graphql::Result<String> {
        let client = ctx.data::<Arc<Mutex<RestApi>>>()?.clone();
        // let resp = (&client.lock().await).ping().await?;
        // let resp = client.lock().await.ping().await?;
        let resp = client.lock().await.ping().await?;
        let data = resp.data().await?;
        let ack = data.to_string();
        Ok(ack)
    }

    async fn exchange_info(&self, ctx: &Context<'_>) -> async_graphql::Result<&ExchangeInfoResponse> {
        let client = ctx.data::<Arc<Mutex<RestApi>>>()?.clone();
        let resp = (&client.lock().await).exchange_info(ExchangeInfoParams::default()).await?;
        let data = resp.data().await?;
        // Ok(serde_json::to_string_pretty(&data)?)
        Ok(&data)
    }
}


// struct Mutation;

// #[Object]
// impl Mutation {
//     async fn signup(&self, username: String, password: String) -> Result<bool> {
//         // User signup
//     }

//     async fn login(&self, username: String, password: String) -> Result<String> {
//         // User login (generate token)
//     }
// }


// struct Subscription;

// #[Subscription]
// impl Subscription {
//     async fn integers(&self, #[graphql(default = 1)] step: i32) -> impl Stream<Item = i32> {
//         let mut value = 0;
//         tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
//             .map(move |_| {
//                 value += step;
//                 value
//             })
//     }
// }
>>>>>>> origin/main
