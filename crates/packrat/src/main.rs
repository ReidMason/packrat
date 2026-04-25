//! Composition root: the only place `infrastructure` and the `packrat_application` layer are wired.

use packrat_application::get_item;
use packrat_domain::item::ItemId;
use packrat_infrastructure::StubItemQuery;

fn main() {
    let item_query = StubItemQuery;
    if let Some(item) = get_item(&item_query, ItemId::new(1)) {
        println!("#{:?}: {:?}", item.id, item.name);
    }
}
