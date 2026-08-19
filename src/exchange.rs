use::anyhow::Result;
use::async_trait::async_trait;

#[async_trait]
pub trait OrderClient {
    fn id(&self) -> ExchangeId;
    fn place_order(order: Order) -> Result<OrderUpdate>;

}

#[async_trait]
pub trait MarketDataClient {
    fn start_listening(&self) -> Result<()>;
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

pub enum ExchangeId {
    Binance,
    Bitget,
    Okx,
}

impl ExchangeId {
    pub fn as_str(&self) -> &str {
        match self {
            ExchangeId::Binance => "binance",
            ExchangeId::Bitget => "bitget",
            ExchangeId::Okx => "okx",
        }
    }
}


pub mod binance;