use crate::exchange::{Order, OrderApi, OrderUpdate};

pub struct OrderClient {

}

impl OrderClient {
    pub fn new() -> Self {
        Self {}
    }
}

impl OrderApi for OrderClient {
    fn place_order(&self, order: Order) -> anyhow::Result<OrderUpdate> {
        todo!()
    }
    fn cancel_order(&self, order: String) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn get_balance(&self) -> Result<f64, anyhow::Error> {
        todo!()
    }
}