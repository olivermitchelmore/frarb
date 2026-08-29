use crossfire::{mpsc, MAsyncRx};
use crate::exchange::{ExchangeType, ExchangeUpdate};

pub mod funding_arb;

pub trait Strategy {
    fn new(exchanges: Vec<ExchangeType>, rx: MAsyncRx<mpsc::Array<ExchangeUpdate>>) -> Self;
    async fn run(&self) -> anyhow::Result<()>;
}