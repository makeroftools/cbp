use binance_sdk::spot::{
    rest_api::TickerParams,
};

use async_graphql::{
    Context,
    Object
};


pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn ping(
        &self, 
        ctx: &Context,
        #[graphql(desc = "This arg is optional.")]
        arg: Option<Arg>,
    ) -> Ticker {
        let params = TickerParams::default();
        let response = rest_client
            .ticker(params)
            .await
            .context("ticker request failed");
        let data = response.data().await?;
        return data
    }
}

