use async_trait::async_trait;
use packrat_domain::asset::{Asset, AssetId};

use super::asset_search_query::AssetSearchQuery;

#[async_trait]
pub trait AssetQueryPort: Send + Sync {
    async fn get_asset_by_id(&self, id: AssetId) -> Option<Asset>;

    /// All non-deleted assets, typically ordered for display (e.g. by name).
    async fn list_active_assets(&self) -> Vec<Asset>;

    /// Active assets matching all supplied filters (`name` exact, `fuzzyname` substring).
    async fn search_assets(&self, query: &AssetSearchQuery) -> Vec<Asset>;

    /// Active assets whose `parent_id` is `parent_id`.
    async fn list_child_assets(&self, parent_id: AssetId) -> Vec<Asset>;
}
