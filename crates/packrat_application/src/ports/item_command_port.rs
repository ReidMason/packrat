use async_trait::async_trait;
use packrat_domain::bucket::BucketId;
use packrat_domain::item::{Item, ItemName};
use packrat_domain::location::LocationId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemPlacement {
    InLocation(LocationId),
    InBucket(BucketId),
}

#[async_trait]
pub trait ItemCommandPort: Send + Sync {
    async fn create_item(&self, name: ItemName, placement: ItemPlacement) -> Item;
}
