pub mod binance;
pub mod bitget;
pub mod types;
mod market_data;

pub use types::*;

use anyhow::Result;
use binance::Binance;
use bitget::Bitget;
use crossfire::{MAsyncTx, mpsc};

pub trait OrderApi {
    fn place_order(&self, order: Order) -> Result<OrderUpdate>;
    fn cancel_order(&self, order_id: String) -> Result<()>;
    fn get_balance(&self) -> Result<f64, anyhow::Error>;
}

pub trait MarketDataProvider {
    async fn start_listening(&self, tx: MAsyncTx<mpsc::Array<ExchangeUpdate>>) -> Result<()>;
}

pub trait Exchange {
    fn new() -> Self;
    fn id(&self) -> ExchangeId;
}

pub enum ExchangeType {
    Binance(Binance),
    Bitget(Bitget),
}

impl OrderApi for ExchangeType {
    fn place_order(&self, order: Order) -> Result<OrderUpdate> {
        match self {
            ExchangeType::Binance(e) => e.order_client.place_order(order),
            ExchangeType::Bitget(e) => e.order_client.place_order(order),
        }
    }

    fn cancel_order(&self, order_id: String) -> Result<()> {
        match self {
            ExchangeType::Binance(e) => e.order_client.cancel_order(order_id),
            ExchangeType::Bitget(e) => e.order_client.cancel_order(order_id),
        }
    }

    fn get_balance(&self) -> Result<f64> {
        match self {
            ExchangeType::Binance(e) => e.order_client.get_balance(),
            ExchangeType::Bitget(e) => e.order_client.get_balance(),
        }
    }
}
impl MarketDataProvider for ExchangeType {
    async fn start_listening(&self, tx: MAsyncTx<mpsc::Array<ExchangeUpdate>>) -> Result<()> {
        match self {
            ExchangeType::Binance(e) => e.market_data.start_listening(tx).await?,
            ExchangeType::Bitget(e) => e.market_data.start_listening(tx).await?,
        };
        Ok(())
    }
}
