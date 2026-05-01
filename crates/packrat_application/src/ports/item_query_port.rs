use async_trait::async_trait;
use packrat_domain::entity::{Entity, EntityId};

use super::item_search_query::ItemSearchQuery;

#[async_trait]
pub trait ItemQueryPort: Send + Sync {
    async fn get_item_by_id(&self, id: EntityId) -> Option<Entity>;

    /// All non-deleted items, typically ordered for display (e.g. by name).
    async fn list_active_items(&self) -> Vec<Entity>;

    /// Active items matching all supplied filters (`name` exact, `fuzzyname` substring).
    async fn search_items(&self, query: &ItemSearchQuery) -> Vec<Entity>;
}
