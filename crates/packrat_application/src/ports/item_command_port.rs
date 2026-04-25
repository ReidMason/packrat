use packrat_domain::item::{Item, ItemName};

pub trait ItemCommandPort: Send + Sync {
    fn create_item(&self, name: ItemName) -> Item;
}
