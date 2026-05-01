/// Filters for [`AssetQueryPort::search_assets`](super::AssetQueryPort::search_assets).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AssetSearchQuery {
    /// Exact match on `assets.name` (case-sensitive).
    pub name: Option<String>,
    /// Case-insensitive substring match on `assets.name`.
    pub fuzzyname: Option<String>,
}
