/// Filters for [`ItemQueryPort::search_items`](super::ItemQueryPort::search_items).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ItemSearchQuery {
    /// Exact match on `items.name` (case-sensitive).
    pub name: Option<String>,
    /// Case-insensitive substring match on `items.name`.
    pub fuzzyname: Option<String>,
}
