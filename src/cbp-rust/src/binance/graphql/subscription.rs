// src/graphql/subscription.rs
use async_graphql::{
    Context, Object, Result, Schema, Subscription, ID,
};
use futures_util::{Stream, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

/// Root Subscription object
#[derive(Default)]
pub struct Subscription;

/// Shared WebSocket manager that holds all active streams
pub struct BinanceWsClient {
    tx: broadcast::Sender<WsMessage>,
}

#[derive(Clone, Debug)]
pub enum WsMessage {
    Trade { symbol: String, price: String, qty: String, time: i64 },
    BookTicker { symbol: String, best_bid: String, best_ask: String },
    Kline { symbol: String, interval: String, open: String, close: String, high: String, low: String, time: i64 },
    UserDataBalance { asset: String, free: String, locked: String },
    UserDataOrder { symbol: String, order_id: i64, status: String, executed_qty: String },
}

impl BinanceWsClient {
    pub fn new() -> Arc<Self> {
        let (tx, _) = broadcast::channel(1024);
        let client = Arc::new(Self { tx });

        // Spawn global listeners
        let client_clone = client.clone();
        tokio::spawn(async move {
            client_clone.start_combined_streams().await;
        });

        client
    }

    async fn start_combined_streams(self: Arc<Self>) {
        use tokio_tungstenite::connect_async;
        use futures_util::sink::SinkExt;
        use tokio_tungstenite::tungstenite::Message;

        let symbols = vec!["btcusdt", "ethusdt", "bnbusdt"]; // customize or make dynamic
        let streams: Vec<String> = symbols
            .iter()
            .map(|s| format!("{}@trade", s.to_lowercase()))
            .collect();

        let url = format!(
            "wss://stream.binance.com:9443/stream?streams={}",
            streams.join("/")
        );

        loop {
            match connect_async(&url).await {
                Ok((ws_stream, _)) => {
                    let (mut write, mut read) = ws_stream.split();

                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                if let Ok(event) = serde_json::from_str::<serde_json::Value>(&text) {
                                    if let Some(data) = event["data"].as_object() {
                                        if let (Some(e), Some(symbol)) = (data["e"].as_str(), data["s"].as_str()) {
                                            match e {
                                                "trade" => {
                                                    let _ = self.tx.send(WsMessage::Trade {
                                                        symbol: symbol.to_uppercase(),
                                                        price: data["p"].as_str().unwrap_or("0").to_string(),
                                                        qty: data["q"].as_str().unwrap_or("0").to_string(),
                                                        time: data["T"].as_i64().unwrap_or(0),
                                                    });
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                            }
                            Ok(Message::Close(_)) | Err(_) => break,
                            _ => {}
                        }
                    }
                }
                Err(e) => {
                    eprintln!("WebSocket error: {e}, reconnecting in 5s...");
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
    }

    pub fn subscribe_trades(&self) -> impl Stream<Item = WsMessage> {
        BroadcastStream::new(self.tx.subscribe()).filter_map(|res| async move {
            match res {
                Ok(WsMessage::Trade { .. }) => Some(res.ok()?),
                _ => None,
            }
        })
    }

    pub fn subscribe_all(&self) -> impl Stream<Item = WsMessage> {
        BroadcastStream::new(self.tx.subscribe()).filter_map(|res| async move { res.ok() })
    }
}

#[Subscription]
impl Subscription {
    /// Live trades for a symbol (e.g. BTCUSDT)
    async fn trades(&self, ctx: &Context<'_>, symbol: String) -> impl Stream<Item = Result<Trade>> {
        let client = ctx.data_unchecked::<Arc<BinanceWsClient>>();
        client.subscribe_trades().filter_map(move |msg| async move {
            if let WsMessage::Trade { symbol: s, price, qty, time } = msg {
                if s == symbol.to_uppercase() {
                    return Some(Ok(Trade {
                        symbol: s,
                        price,
                        qty,
                        time,
                    }));
                }
            }
            None
        })
    }

    /// Best bid/ask ticker (optional — add your own stream if needed)
    // async fn book_ticker(&self, ...) { ... }
}

#[derive(async_graphql::SimpleObject)]
pub struct Trade {
    pub symbol: String,
    pub price: String,
    pub qty: String,
    pub time: i64,
}