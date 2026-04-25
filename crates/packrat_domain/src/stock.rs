use crate::{bucket::BucketId, inventory::Inventory, item::ItemId};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum StockId {
    Item(ItemId),
    Bucket(BucketId),
}

pub trait Stock {
    fn id(&self) -> StockId;
    fn as_inventory(&self) -> Option<&dyn Inventory> {
        None
    }
}
