// src/binance/main.rs
use std::sync::Arc;

use async_graphql::{Schema, http::GraphiQLSource};
use async_graphql_poem::GraphQL;
use poem::{get, handler, listener::TcpListener, Route, Server, web::Html};

mod graphql;
mod binance_client;
use binance_client::BinanceClient;
use graphql::{query::Query, mutation::Mutation, subscription::{Subscription, BinanceWsClient}};

use binance_sdk::config::ConfigurationRestApi;

#[handler]
async fn graphiql() -> impl poem::IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let rest_config = ConfigurationRestApi::builder().build().unwrap();
    let rest_client = BinanceClient::new(rest_config);
    let ws_client = BinanceWsClient::new();

    let schema = Schema::build(Query::default(), Mutation::default(), Subscription::default())
        .data(rest_client)
        .data(ws_client)
        .limit_depth(10)
        .limit_complexity(200)
        .finish();

    let app = Route::new()
        .at("/", get(graphiql).post(GraphQL::new(schema)));

    println!("GraphiQL IDE: http://localhost:8000");
    Server::new(TcpListener::bind("127.0.0.1:8000"))
        .run(app)
        .await
        .unwrap();
}