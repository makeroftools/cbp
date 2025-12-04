use std::error::Error;

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_poem::*;
// use binance_sdk::spot::websocket_api::order_cancel_replace_response_result_cancel_response;
use poem::{listener::TcpListener, web::Html, *};


struct Query;

#[Object]
impl Query {
    async fn time<'ctx>(&self, ctx: &Context<'ctx>) -> i64 {
        let response = ctx.data::<SpotRestApi>.time().await.context("time request failed")?;
        let time = response.server_time;
        Ok(time)
    }
}



#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    // config
    let rest_conf = ConfigurationRestApi::builder()
        .api_key("")
        .api_secret("")
        .build()?;
    
    let client = SpotRestApi::production(rest_conf);


    // create the schema
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).data(client).finish();

    // start the http server
    let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));
    println!("GraphiQL: http://localhost:8000");
    Server::new(TcpListener::bind("0.0.0.0:8000")).run(app).await?;
    Ok(())
}