use packrat_domain::item::{Item, ItemId};

pub trait ItemQueryPort: Send + Sync {
    fn get_item_by_id(&self, id: ItemId) -> Option<Item>;
}
