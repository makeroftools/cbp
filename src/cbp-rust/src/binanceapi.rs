use async_graphql;
use async_graphql::*;
use binance_sdk::spot::rest_api::RestApi;
use binance_sdk::spot::rest_api::ExchangeInfoParams;
// use binance_sdk::spot::rest_api::ExchangeInfoResponse;
use std::sync::Arc;
// use serde_json;
use tokio::sync::Mutex;
  
#[derive(SimpleObject)]
struct RateLimit {
    pub interval: String,
    pub limit: i64,
    pub interval_num: i64,
    pub rate_limit_type: String,
}

#[derive(SimpleObject)]
pub struct Filters {
    pub filter_type: String,
    pub max_price: String,
    pub min_price: String,
    pub tick_size: String,
    pub max_qty: String,
    pub min_qty: String,
    pub step_size: String,
    pub limit: i64,
    pub multiplier_up: String,
    pub multiplier_down: String,
    pub multiplier_decimal: String,
}


#[derive(SimpleObject)]
pub struct Symbols {
    pub filters: Vec<Filters>,
    pub order_type: Vec<String>,
    pub time_in_force: Vec<String>,
    pub liquidation_fee: String,
    pub market_take_bound: String,
    pub symbol: String,
    pub pair: String,
    pub contract_type: String,
    pub delivery_date: i64,
    pub onboard_date: i64,
    pub contract_status: String,
    pub contract_size: i64,
    pub quote_asset: String,
    pub base_asset: String,
    pub margin_asset: String,
    pub price_precision: i64,
    pub quantity_precision: i64,
    pub base_asset_precision: i64,
    pub quote_precision: i64,
    pub equal_qty_precision: i64,
    pub trigger_protect: String,
    pub maint_margin_percent: String,
    pub required_margin_percent: String,
    pub underlying_type: String,
    pub underlying_sub_type: Vec<String>,
}

#[derive(SimpleObject)]
struct ExchangeInfo {
    exchange_filters: Vec<String>,
    rate_limits: Vec<RateLimit>,
    server_time: i64,
    symbols: Vec<String>,
    timezone: String
}

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

    async fn exchange_info(&self, ctx: &Context<'_>) -> async_graphql::Result<ExchangeInfo> {
        let client = ctx.data::<Arc<Mutex<RestApi>>>()?.clone();
        let data = client.lock().await
            .exchange_info(ExchangeInfoParams::default())
            .await?
            .data()
            .await?;
        let ret = ExchangeInfo {
            exchange_filters: data.exchange_filters,
            rate_limits: data.rate_limits,
            server_time: data.server_time.unwrap(),
            symbols: data.symbols,
            timezone: data.timezone,
        };
        Ok(ret)
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
