use async_trait::async_trait;
use packrat_domain::item::{Item, ItemName};

#[async_trait]
pub trait ItemCommandPort: Send + Sync {
    async fn create_item(&self, name: ItemName) -> Item;
}
