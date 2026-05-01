use async_trait::async_trait;
use packrat_domain::entity::{Entity, EntityId};

use super::asset_search_query::AssetSearchQuery;

#[async_trait]
pub trait AssetQueryPort: Send + Sync {
    async fn get_asset_by_id(&self, id: EntityId) -> Option<Entity>;

    /// All non-deleted assets, typically ordered for display (e.g. by name).
    async fn list_active_assets(&self) -> Vec<Entity>;

    /// Active assets matching all supplied filters (`name` exact, `fuzzyname` substring).
    async fn search_assets(&self, query: &AssetSearchQuery) -> Vec<Entity>;

    /// Active assets whose `parent_id` is `parent_id`.
    async fn list_child_assets(&self, parent_id: EntityId) -> Vec<Entity>;
}
