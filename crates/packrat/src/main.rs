//! Composition root: the only place `infrastructure` and the `packrat_application` layer are wired.

use packrat_application::execute;
use packrat_domain::item::{Item, ItemId, ItemName};
use packrat_infrastructure::StubItemQuery;

fn main() {
    let item_query = StubItemQuery;
    if let Some(item) = execute(&item_query, ItemId::new(1)) {
        println!("#{:?}: {:?}", item.id, item.name);
    }
    let local = Item::new(ItemId::new(1), ItemName::from("Test"));
    println!("#{:?}: {:?}", local.id, local.name);
}
