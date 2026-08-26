use crate::exchange::ExchangeType;

pub mod funding_arb;

pub trait Strategy {
    fn create(exchanges: Vec<ExchangeType>);
    fn run();
}