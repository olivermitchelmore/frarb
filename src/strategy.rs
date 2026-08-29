use crossfire::{mpsc, MAsyncRx};
use crate::exchange::{ExchangeType, ExchangeUpdate};

pub mod funding_arb;
mod types;

pub trait Strategy {
    fn new(exchanges: Vec<ExchangeType>, rx: MAsyncRx<mpsc::Array<ExchangeUpdate>>) -> Self;
    fn subscribe(&self);
    async fn run(&self) -> anyhow::Result<()>;
}