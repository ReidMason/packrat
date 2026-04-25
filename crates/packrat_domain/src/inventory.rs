use crate::item::{Item, ItemId};

pub trait Inventory {
    fn find_item(&self, id: ItemId) -> Option<Item>;
}
