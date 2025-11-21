use async_graphql::{EmptyMutation, EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_poem::GraphQL;
use poem::{IntoResponse, Route, Server, get, handler, listener::TcpListener, web::Html};

mod binanceapi;
use binanceapi::BinanceQuery;
use binance_sdk::config::ConfigurationRestApi;
// use binance_sdk::config::ConfigurationWebsocketApi;
use binance_sdk::spot;


#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let rest_config = ConfigurationRestApi::builder().build().unwrap();
    let rest_client = spot::SpotRestApi::production(rest_config);
    
    // let ws_config = ConfigurationWebsocketApi::builder().build();
    // let ws_client = spot::SpotWsApi::production(ws_config);
    // let ws_connection = ws_client.connect().await?;
    
    let schema = Schema::build(BinanceQuery, EmptyMutation, EmptySubscription)
        .data(rest_client)
        .finish();
    
let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));

println!("GraphiQL IDE: http://localhost:8000");

Server::new(TcpListener::bind("127.0.0.1:8000"))
    .run(app)
    .await
    .unwrap();
}

