use std::collections::HashMap;
use crate::exchange::{ExchangeId, MarketUpdate, Symbol, SymbolRequest};
use tokio::sync::watch;

type Topic = (ExchangeId, Symbol);
pub struct ChannelRegistry {
    channels: HashMap<Topic, watch::Sender<MarketUpdate>>
}

impl ChannelRegistry {
    pub fn new(symbols: Vec<SymbolRequest>) -> Self {
        let mut channels = HashMap::with_capacity(symbols.len());
        for symbol in symbols {
            let topic: Topic = (symbol.exchange, symbol.symbol);
            if channels.contains_key(&topic) {
                continue;
            }
            else {
                let (tx, rx) = watch::channel(MarketUpdate::default());
                channels.insert(topic, tx);
            }
        };
        Self {
            channels
        }
    }
}