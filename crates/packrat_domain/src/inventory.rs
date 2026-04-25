use crate::stock::{Stock, StockId};

pub trait Inventory {
    fn find_item(&self, id: StockId) -> Option<&dyn Stock>;
}
