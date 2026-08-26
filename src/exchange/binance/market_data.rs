use crate::exchange::MarketDataProvider;

pub struct MarketData {

}

impl MarketData {
    pub fn new() -> Self{
        Self {}
    }
}

impl MarketDataProvider for MarketData {
    fn start_listening(&self) -> anyhow::Result<()> {
        todo!()
    }
}