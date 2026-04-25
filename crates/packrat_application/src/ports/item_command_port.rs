use packrat_domain::{Item, ItemName};

pub trait ItemCommandPort: Send + Sync {
    fn create_item(&self, name: ItemName) -> Item;
}
