use tokio::sync::watch;
use std::str::FromStr;

pub struct Order {
    price: i64,
    quantity: i64,
    side: Side,
    order_type: OrderType,
    time_in_force: TimeInForce,
}
#[derive(Default)]
pub struct MarketUpdate {
    exchange: ExchangeId,
    symbol: Symbol,
    level: i64,
    quantity: i64,
}
pub enum OrderResponse {
    Success(OrderUpdate),
    Failure,
}
#[derive(Clone, Copy, Default)]
#[derive(Eq, Hash, PartialEq)]
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

#[derive(Default)]
pub struct OrderUpdate {
    order_id: String,
    price: i64,
    quantity: i64,
    side: Side,
}

#[derive(Eq, Hash, PartialEq, Default)]
pub enum Symbol {
    BTCUSD,
    ETHUSD,
}

impl Symbol {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::BTCUSD => "BTCUSD",
            Self::ETHUSD => "ETHUSD",
        }
    }
}

impl FromStr for Symbol {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "BTCUSDT" => Ok(Symbol::BTCUSD),
            "ETHUSDT" => Ok(Symbol::ETHUSD),
            _ => Err(anyhow::anyhow!("Cannot parse unknown symbol: {}", s)),
        }
    }
}

pub enum ExchangeUpdate {
    OrderUpdate(OrderUpdate),
    MarketUpdate(MarketUpdate),
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
    Sell,
}

impl Default for Side {
    fn default() -> Self {
        Side::Buy
    }
}
pub struct SymbolRequest {
    pub strategy: watch::Receiver<OrderUpdate>,
    pub symbol: Symbol,
    pub exchange: ExchangeId,
}


