use crate::exchange::{Exchange, ExchangeId};

pub mod market_data;
pub mod order_client;
use market_data::MarketData;
use order_client::OrderClient;

pub struct Binance {
    id: ExchangeId,
    pub order_client: OrderClient,
    pub market_data: MarketData,
}

impl Exchange for Binance {
    fn new() -> Self {
        Self {
            id: ExchangeId::Binance,
            order_client: OrderClient::new(),
            market_data: MarketData::new(),
        }
    }

    fn id(&self) -> ExchangeId {
        self.id
    }
}
