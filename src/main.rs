mod exchange;
mod strategy;

use crate::strategy::Strategy;
use crate::strategy::funding_arb::FundingArb;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // let strategy = FundingArb::new();
    // strategy.run().await.unwrap();
}
