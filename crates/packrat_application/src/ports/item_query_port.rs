use packrat_domain::{item::Item, models::Id};

pub trait ItemQueryPort: Send + Sync {
    fn get_item_by_id(&self, id: Id) -> Option<Item>;
}
