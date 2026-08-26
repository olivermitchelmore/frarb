pub mod binance;
pub mod bitget;

use::anyhow::Result;
use binance::Binance;
use bitget::Bitget;

pub trait OrderApi {
    fn place_order(&self, order: Order) -> Result<OrderUpdate>;
    fn cancel_order(&self, order_id: String) -> Result<()>;
    fn get_balance(&self) -> Result<f64, anyhow::Error>;
}

pub trait MarketDataProvider {
    fn start_listening(&self) -> Result<()>;
}

pub trait Exchange {
    fn new() -> Self;
    fn id(&self) -> ExchangeId;
}

pub struct Order {
    price: i64,
    quantity: i64,
    side: Side,
    order_type: OrderType,
    time_in_force: TimeInForce,
}

pub enum TimeInForce {
    Gtc,
    Ioc,
    Fok,
}

pub enum OrderType {
    Market,
    Limit,
}

pub enum Side {
    Buy,
    Sell
}

pub enum OrderResponse {
    Success(OrderUpdate),
    Failure,
}
pub struct OrderUpdate {
    order_id: String,
    price: i64,
    quantity: i64,
    side: Side,
}

#[derive(Clone, Copy)]
pub enum ExchangeId {
    Binance,
    Bitget,
    Okx,
}

impl ExchangeId {
    pub fn name(&self) -> &str {
        match self {
            ExchangeId::Binance => "binance",
            ExchangeId::Bitget => "bitget",
            ExchangeId::Okx => "okx",
        }
    }
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
