// src/binance/main.rs
use async_graphql::{
    Schema, 
    http::GraphiQLSource,
    EmptyMutation,
    EmptySubscription,
};
use async_graphql_poem::GraphQL;

use poem::{get, handler, listener::TcpListener, Route, Server, web::Html};

// mod graphql;
use binance_client::BinanceClient;
use binance_sdk::config::ConfigurationRestApi;
use cbp_binance::binanceapi::BinanceQuery

#[handler]
async fn graphiql() -> impl poem::IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let config = ConfigurationRestApi::builder().build().unwrap();
    let client = BinanceClient::new(config);
    let ws_client = BinanceWsClient::new();

    let schema = Schema::build(BinanceQuery, EmptyMutation, EmptySubscription)
        .data(client)
        .data(ws_client)
        .finish();

    let app = Route::new()
        .at("/", get(graphiql).post(GraphQL::new(schema)));

    println!("GraphiQL: http://127.0.0.1:8000");
    Server::new(TcpListener::bind("127.0.0.1:8000"))
        .run(app)
        .await
        .unwrap();
}


