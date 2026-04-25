use crate::{item::Item, models::Id};

pub trait Inventory {
    fn find_item(&self, id: Id) -> Option<Item>;
}
