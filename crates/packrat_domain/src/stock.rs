use crate::{bucket::BucketId, item::ItemId};

#[derive(Debug, PartialEq, Eq)]
pub enum StockId {
    Item(ItemId),
    Bucket(BucketId),
}

pub trait Stock {}
