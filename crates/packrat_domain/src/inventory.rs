use crate::{bucket::BucketId, location::LocationId, stock::{Stock, StockId}};

#[derive(Debug, PartialEq, Eq)]
pub enum InventoryId {
    Bucket(BucketId),
    Location(LocationId),
}

pub trait Inventory {
    fn find_item(&self, id: StockId) -> Option<impl Stock>;
}
