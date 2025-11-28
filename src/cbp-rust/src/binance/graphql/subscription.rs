// src/binance/graphql/subscription.rs
use async_graphql::{Context, Result, Subscription};
use futures_util::{Stream, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Default)]
pub struct Subscription;

pub struct BinanceWsClient {
    tx: broadcast::Sender<WsMessage>,
}

#[derive(Clone, Debug)]
pub enum WsMessage {
    Trade {
        symbol: String,
        price: String,
        qty: String,
        time: i64,
    },
}

impl BinanceWsClient {
    pub fn new() -> Arc<Self> {
        let (tx, _) = broadcast::channel(1024);
        let client = Arc::new(Self { tx });
        let client_clone = client.clone();
        tokio::spawn(async move {
            client_clone.run().await;
        });
        client
    }

    async fn run(self: Arc<Self>) {
        let symbols = ["btcusdt", "ethusdt", "bnbusdt"];
        let streams: Vec<String> = symbols.iter().map(|s| format!("{}@trade", s)).collect();
        let url = format!("wss://stream.binance.com:9443/stream?streams={}", streams.join("/"));

        loop {
            match connect_async(&url).await {
                Ok((ws, _)) => {
                    let (_, mut read) = ws.split();
                    while let Some(Ok(msg)) = read.next().await {
                        if let Message::Text(text) = msg {
                            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                                if let Some(data) = json["data"].as_object() {
                                    if data["e"].as_str() == Some("trade") {
                                        if let Some(s) = data["s"].as_str() {
                                            let _ = self.tx.send(WsMessage::Trade {
                                                symbol: s.to_uppercase(),
                                                price: data["p"].as_str().unwrap_or("").into(),
                                                qty: data["q"].as_str().unwrap_or("").into(),
                                                time: data["T"].as_i64().unwrap_or(0),
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("WS error: {e}, reconnecting...");
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
    }

    pub fn subscribe(&self) -> impl Stream<Item = WsMessage> {
        BroadcastStream::new(self.tx.subscribe())
            .filter_map(|r| async move { r.ok() })
    }
}

#[Subscription]
impl Subscription {
    async fn trades(&self, ctx: &Context<'_>, symbol: String) -> impl Stream<Item = Result<Trade>> {
        let ws = ctx.data_unchecked::<Arc<BinanceWsClient>>();
        let target = symbol.to_uppercase();
        ws.subscribe().filter_map(move |msg| {
            let target = target.clone();
            async move {
                match msg {
                    WsMessage::Trade { symbol: s, price, qty, time } => {
                        if s == target {
                            Some(Ok(Trade { symbol: s, price, qty, time }))
                        } else {
                            None
                        }
                    }
                }
            }
        })
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct Trade {
    pub symbol: String,
    pub price: String,
    pub qty: String,
    pub time: i64,
}