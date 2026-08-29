use crate::exchange::{ExchangeType, ExchangeUpdate, MarketDataProvider};
use crate::strategy::Strategy;
use crossfire::{mpsc, MAsyncRx};

pub struct FundingArb {
    exchanges: Vec<ExchangeType>,
    rx: MAsyncRx<mpsc::Array<ExchangeUpdate>>,
}

impl Strategy for FundingArb {
    fn new(exchanges: Vec<ExchangeType>, rx: MAsyncRx<mpsc::Array<ExchangeUpdate>>) -> Self {
        Self {
            exchanges,
            rx
        }
    }

    fn subscribe(&self) {
        
    }

    async fn run(&self) -> anyhow::Result<()> {
        let (tx, _rx) = mpsc::bounded_async::<ExchangeUpdate>(100);

        for exchange in self.exchanges.iter() {
            exchange.start_listening(tx.clone()).await?;
        }

        Ok(())
    }
}
