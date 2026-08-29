use crate::exchange::{ExchangeUpdate, MarketDataProvider};
use crossfire::{MAsyncTx, mpsc};

pub struct MarketData {}

impl MarketData {
    pub fn new() -> Self {
        Self {}
    }
}

impl MarketDataProvider for MarketData {
    async fn start_listening(
        &self,
        tx: MAsyncTx<mpsc::Array<ExchangeUpdate>>,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
